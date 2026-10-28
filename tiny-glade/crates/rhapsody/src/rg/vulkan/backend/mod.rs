//! Render-graph execution backend over one Vulkan queue family.
use ash::vk;
use crate::command::command_buffer::VulkanCommandBuffer;
use crate::command::command_buffer::VulkanCommandPool;
use crate::command::recording;
use crate::device::Device;
use crate::device::semaphore::TimelineSemaphore;
use crate::resource::managed::resources::ManagedResources;
use std::collections::BTreeMap;
use std::sync::Arc;
use super::access_table::AccessTable;
use super::capture::GpuSubmissionCapture;
use super::presentation;
use super::queue_runtime;
use super::resource_spec::GpuResource;
use super::resource_spec::ResourceSpec;
mod chunk_recording;
mod stats;
mod configure;
mod imports;
mod validate;
mod commands;
mod execution;
use chunk_recording::{CommandBarriers, OwnedBatchSync};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct FramePoolKey {
    frame_slot: u32,
    worker: usize,
    family: u32,
}

pub(super) struct FrameCommandPool {
    pool: Arc<VulkanCommandPool>,
    epoch: u64,
}

/// The alias set bound to the frame being populated.
pub(super) struct ActiveAlias {
    plan: Arc<crate::rg::transient_memory::PhysicalMemoryPlan>,
    pub(super) edges: Arc<[(u32, u32)]>,
}

pub struct VulkanGraphBackend {
    pub(crate) device: Arc<Device>,
    pub timeline: Arc<TimelineSemaphore>,
    pub resources: crate::collections::token_map::TokenMap<GpuResource>,
    pub accesses: Arc<AccessTable>,
    pub submissions: std::collections::VecDeque<GpuSubmissionCapture>,
    /// Set by the session right after it validated the declaration, so the
    /// trait's `prepare_declaration` does not validate the same graph twice.
    pub(super) receipt_storage: Option<crate::rg::receipt_storage::ReceiptStorage>,
    pub(super) active_receipt_slot: Option<crate::rg::receipt_storage::ReceiptLease>,
    pub(super) specifications: BTreeMap<crate::resource::ResourceName, ResourceSpec>,
    transient_aliasing: bool,
    prepared_transients: crate::rg::transient_resources::PreparedTransients,
    /// Evictable idle pool and alias sets, registered with the device.
    pub(super) caches: Arc<crate::rg::graph_caches::GraphCaches>,
    /// Members of the alias set lent to the frame being populated.
    lent_aliases: Option<crate::rg::transient_resources::LentAliases>,
    pub(super) active_alias: Option<ActiveAlias>,
    /// Pending acquires recorded into a not yet submitted command; re-queued
    /// on the device if that command is abandoned.
    drained_acquires: Vec<(u64, crate::command::DrainedAcquires)>,
    pub(super) transient_alias_tokens: BTreeMap<u64, crate::rg::transient_resources::AliasOrigin>,
    transient_counters: Arc<crate::device::transient_backing::TransientBackingCounters>,
    cpu_arenas: crate::command::CpuArenaPool,
    owner_pool: crate::collections::temp_list_pool::TempListPool<crate::command::binding::RetainedResource>,
    frame_storage: crate::rg::frame_storage::FrameStoragePool,
    import_validation: parking_lot::Mutex<crate::collections::temp_list::TempList<(u8, u64)>>,
    frame_pools: BTreeMap<FramePoolKey, FrameCommandPool>,
    recording_epoch: u64,
    active_frame_slot: u32,
    /// Set once the session uses explicit display frames; doubles the command
    /// slots (see `COMMAND_SLOTS_PER_DISPLAY_FRAME`). Sessions that only call
    /// `begin_frame` keep one slot per in-flight frame.
    pub(super) display_frames: bool,
    pub(super) commands: BTreeMap<u64, VulkanCommandBuffer>,
    pub(super) queue: recording::Queue,
    pub(super) family: u32,
    recording_family: u32,
    recording_queue: crate::sync::QueueId,
    gpu_timing: Option<crate::rg::profiling::GpuTimingRing>,
    pub(super) queue_runtime: queue_runtime::QueueRuntime,
    pub(super) queue_timelines: Vec<Arc<TimelineSemaphore>>,
    pub(super) import_families: crate::collections::token_map::TokenMap<u32>,
    pub(super) history_validity: crate::collections::token_map::TokenMap<bool>,
    pub(in crate::rg::vulkan) synchronization: BTreeMap<usize, OwnedBatchSync>,
    pub(super) graph_waits: Vec<TimelineWait>,
    pub(super) graph_signals: Vec<TimelineSignal>,
    /// The acquired swapchain image while its presentation graph executes.
    pub(super) presentation: Option<presentation::BackendPresentation>,
    pub(super) recycled_commands: Vec<u64>,
    /// A backend selects one actual queue family; each frame and recording worker
    /// receives a separate pool within that family.
    command_slot: BTreeMap<u64, FramePoolKey>,
    pub stores: ManagedResources,
    barrier_dispatch: crate::command::barrier::BarrierDispatch,
    barriers: CommandBarriers,
    /// Tokens already retained by `retained_command` in this recording; a
    /// resource touched by many transitions is retained (and its use cell
    /// scanned for) once. Sorted, capacity kept across frames.
    retained_command: u64,
    retained_tokens: Vec<u64>,
}

impl VulkanGraphBackend {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

pub type TimelineWait = (Arc<TimelineSemaphore>, u64, vk::PipelineStageFlags);

pub type TimelineSignal = (Arc<TimelineSemaphore>, u64);

impl VulkanGraphBackend {
    pub fn new(device: Arc<Device>, accesses: AccessTable) -> crate::Result<Self> {
        Self::new_for_queue(device, accesses, recording::Queue::Primary)
    }
    /// 使用设备创建配置的队列类别；别名类别共享 VkQueue、提交锁与时间线。
    pub fn new_for_class(
        device: Arc<Device>,
        accesses: AccessTable,
        class: crate::device::QueueClass,
    ) -> crate::Result<Self> {
        let id = device.queues.plan.queue(class);
        let queue = if id.index() == 0 {
            recording::Queue::Primary
        } else {
            recording::Queue::Auxiliary(id.index() as u32 - 1)
        };
        Self::new_for_queue(device, accesses, queue)
    }
    pub fn new_for_queue(
        device: Arc<Device>,
        accesses: AccessTable,
        queue: recording::Queue,
    ) -> crate::Result<Self> {
        let mut accesses = accesses;
        accesses.normalize_enabled_stages(device.core.capabilities);
        let (slot, family) = match queue {
            recording::Queue::Primary => (0, device.queues.primary_family),
            recording::Queue::Auxiliary(index) => {
                let selected = device
                    .queues
                    .additional
                    .get(index as usize)
                    .ok_or_else(|| crate::Error::invalid_usage("请求的辅助队列不存在"))?;
                (index as usize + 1, selected.family)
            }
        };
        let queue_timelines = (0..=device.queues.additional.len())
            .map(|index| {
                TimelineSemaphore::for_queue(
                    device.clone(),
                    crate::sync::QueueId::new(index).expect("已验证队列数"),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let stores = device.resources()?.clone();
        let cpu_arenas = crate::command::CpuArenaPool::new(
            crate::command::CpuArenaConfig::default(),
            device.memory.stats.clone(),
        )?;
        let owner_pool = crate::collections::temp_list_pool::TempListPool::new(
            Default::default(),
            device.memory.stats.clone(),
        )?;
        let frame_storage = crate::rg::frame_storage::FrameStoragePool::new(
            Default::default(),
            device.memory.stats.clone(),
        )?;
        let import_validation = parking_lot::Mutex::new(crate::collections::temp_list::TempList::new(
            65_536,
            device.memory.stats.clone(),
        )?);
        let projection_pool = crate::rg::pass_bindings::ManagedProjectionPool::new(
            device.generation(),
            device.memory.stats.clone(),
            Default::default(),
        )?;
        let queue_runtime = queue_runtime::QueueRuntime::new(&device)?;
        let barrier_dispatch = crate::command::barrier::BarrierDispatch::new(&device);
        let caches = Arc::new(crate::rg::graph_caches::GraphCaches::new(projection_pool));
        // Weak registration: the backend owns the caches, so they unregister
        // when it drops and the device never keeps them (or itself) alive.
        let evictable: Arc<dyn crate::accounting::Evictable> = caches.clone();
        device.register_evictable(
            Arc::downgrade(&evictable),
            crate::accounting::EvictionPriority::Transient,
        );
        Ok(Self {
            cpu_arenas,
            owner_pool,
            frame_storage,
            import_validation,
            recording_family: family,
            recording_queue: crate::sync::QueueId::new(slot).expect("validated queue slot"),
            gpu_timing: None,
            queue_runtime,
            timeline: queue_timelines[slot].clone(),
            frame_pools: BTreeMap::new(),
            recording_epoch: 0,
            active_frame_slot: 0,
            display_frames: false,
            device,
            accesses: Arc::new(accesses),
            submissions: Default::default(),
            receipt_storage: None,
            active_receipt_slot: None,
            resources: Default::default(),
            specifications: BTreeMap::new(),
            transient_aliasing: true,
            prepared_transients: Default::default(),
            caches,
            lent_aliases: None,
            active_alias: None,
            drained_acquires: Vec::new(),
            transient_alias_tokens: Default::default(),
            transient_counters: Default::default(),
            commands: BTreeMap::new(),
            queue,
            family,
            queue_timelines,
            import_families: Default::default(),
            history_validity: Default::default(),
            synchronization: BTreeMap::new(),
            graph_waits: vec![],
            graph_signals: vec![],
            presentation: None,
            recycled_commands: vec![],
            command_slot: BTreeMap::new(),
            stores,
            barrier_dispatch,
            barriers: CommandBarriers::default(),
            retained_command: 0,
            retained_tokens: Vec::new(),
        })
    }
    /// Release a pooled token's resource, wherever it is (owner table or the
    /// evictable idle store).
    pub(crate) fn discard_pooled(&mut self, token: u64) {
        self.resources.remove(&token);
        self.import_families.remove(&token);
        self.caches.discard(token);
    }
    pub(super) fn clear_pass_resources(&self) {
        let mut projection = self.caches.projection();
        projection.snapshot.take();
        projection.pool.clear_idle();
    }
    /// Members lent to a frame that never bound them (population failed or
    /// stopped early) were not used by the GPU; they return to their set.
    fn return_unconsumed_aliases(&mut self) {
        if let Some(lent) = self.lent_aliases.take() {
            let completed = self.device.completed();
            let mut aliases = self.caches.alias();
            for (resource, origin) in lent.resources.into_values() {
                aliases.give_back(origin, resource, &completed);
            }
        }
    }

    pub fn selected_family(&self) -> u32 {
        self.family
    }

}

impl Drop for VulkanGraphBackend {
    fn drop(&mut self) {
        if self.device.wait_idle().is_ok() {
            self.commands.clear();
            self.clear_pass_resources();
        }
    }
}
