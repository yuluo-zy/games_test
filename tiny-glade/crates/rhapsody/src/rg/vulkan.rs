//! 渲染图的 Vulkan 后端与会话；显式访问表给出每个访问码的阶段、访问掩码和布局。
use super::{
    executor::{
        ExecutionBackend, ExecutionChain, GraphBatchSync, GraphExecution,
        GraphTimelineWait,
    },
    graph,
    resource::ResourceOrigin,
};
use crate::command::recording;
use crate::raw::{
    buffer,
    command_buffer::{VulkanCommandBuffer, VulkanCommandPool},
    device::Device,
    managed::resources::ManagedResources,
    semaphore::TimelineSemaphore,
    submit, texture,
};
use ash::vk::{self, Handle};
use std::{collections::BTreeMap, sync::Arc};

#[path = "vulkan_presentation.rs"]
mod presentation;
#[path = "vulkan_queue.rs"]
mod queue_runtime;
pub use crate::raw::swapchain::PresentPolicy;
pub use presentation::{PRESENT_ACCESS, PresentTarget, SwapchainContents};

/// 后端导入 token 与跨帧 temporal token 共用的物理资源标识空间。
///
/// 两边曾各自从 1 起分配，会撞号：`bind_import_named` 会因“token 已属于其他物理资源”拒绝导入。
/// 自动分配只使用低 62 位；带高位标记的宿主编号属于独立域。
/// 不把自动计数器推进到宿主域：宿主下一次分配的编号可能恰好覆盖
/// 已退役但仍在资源池／计划缓存中的内部资源。显式宿主编号只接受高位域。
/// 0 与 `u64::MAX` 保留为无效。
const GENERATED_TOKEN_LIMIT: u64 = 1 << 62;
static NEXT_TOKEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
fn try_allocate_token() -> Option<u64> {
    use std::sync::atomic::Ordering;
    NEXT_TOKEN
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |t| {
            (t < GENERATED_TOKEN_LIMIT).then(|| t + 1)
        })
        .ok()
}

pub use crate::raw::access_table::AccessSpec;
/// Fixed u8 access-code space. Cloning the policy never allocates.
#[derive(Clone)]
pub struct AccessTable(pub AccessEntries);
#[derive(Clone)]
pub struct AccessEntries([Option<AccessSpec>; 256]);
impl Default for AccessEntries {
    fn default() -> Self {
        Self([None; 256])
    }
}
impl AccessEntries {
    pub fn get(&self, code: &u8) -> Option<&AccessSpec> {
        self.0[*code as usize].as_ref()
    }
    pub fn get_mut(&mut self, code: &u8) -> Option<&mut AccessSpec> {
        self.0[*code as usize].as_mut()
    }
    pub fn contains_key(&self, code: &u8) -> bool {
        self.get(code).is_some()
    }
    pub fn insert(&mut self, code: u8, spec: AccessSpec) -> Option<AccessSpec> {
        self.0[code as usize].replace(spec)
    }
    pub fn remove(&mut self, code: &u8) -> Option<AccessSpec> {
        self.0[*code as usize].take()
    }
}
impl std::ops::Index<&u8> for AccessEntries {
    type Output = AccessSpec;
    fn index(&self, code: &u8) -> &AccessSpec {
        self.get(code).expect("validated access code")
    }
}
impl Default for AccessTable {
    fn default() -> Self {
        let mut entries = AccessEntries::default();
        entries.insert(
            0,
            AccessSpec {
                scope: crate::resource::range_state::AccessScope {
                    stages: vk::PipelineStageFlags::TOP_OF_PIPE,
                    access: vk::AccessFlags::empty(),
                    write: false,
                },
                layout: vk::ImageLayout::UNDEFINED,
            },
        );
        Self(entries)
    }
}
impl AccessTable {
    /// Resolve broad or custom native stage policies against VkDevice's actual
    /// enabled features. Access masks and access codes stay intact.
    fn normalize_enabled_stages(&mut self, enabled: crate::raw::device::DeviceCapabilities) {
        let mut disabled = vk::PipelineStageFlags::empty();
        if !enabled.geometry_shader {
            disabled |= vk::PipelineStageFlags::GEOMETRY_SHADER;
        }
        if !enabled.tessellation_shader {
            disabled |= vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
                | vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER;
        }
        for entry in self.0.0.iter_mut().flatten() {
            let scope = &mut entry.scope;
            let original = scope.stages;
            scope.stages &= !disabled;
            // Empty stages must not carry memory access. A custom policy aimed
            // only at a disabled stage still has conservative ordering semantics.
            if scope.stages.is_empty() && (!original.is_empty() || !scope.access.is_empty()) {
                scope.stages = vk::PipelineStageFlags::ALL_COMMANDS;
            }
        }
    }
    /// The complete built-in 64-code policy (see `raw::access_table`).
    pub fn original() -> Self {
        let mut entries = AccessEntries::default();
        for (code, spec) in crate::raw::access_table::ACCESS_TABLE.iter().enumerate() {
            entries.insert(code as u8, *spec);
        }
        Self(entries)
    }
}
impl graph::AccessDependency for AccessTable {
    fn access_mask(&mut self, code: u8) -> vk::AccessFlags {
        self.0[&code].scope.access
    }
    fn is_write(&mut self, code: u8) -> bool {
        self.0[&code].scope.write
    }
    fn native_semantics(
        &mut self,
        code: u8,
    ) -> (crate::resource::range_state::AccessScope, vk::ImageLayout) {
        let spec = self.0[&code];
        let mut scope = spec.scope;
        // The shared shader heap lowers uniform blocks to storage-buffer reads.
        // Keep both read classes in the native scope so either pipeline ABI is
        // valid under this same policy. Descriptor usage remains independently
        // validated when the resource is bound.
        if scope.access.contains(vk::AccessFlags::UNIFORM_READ) {
            scope.access |= vk::AccessFlags::SHADER_READ;
        }
        (scope, spec.layout)
    }
}
#[derive(Clone, Copy)]
pub enum ResourceSpec {
    Buffer(buffer::BufferDesc),
    Image(texture::TextureDesc),
    ConcurrentBuffer(buffer::BufferDesc, crate::resource::QueueFamilies),
    ConcurrentImage(texture::TextureDesc, crate::resource::QueueFamilies),
}
impl ResourceSpec {
    pub fn sharing(self) -> crate::resource::ResourceSharing {
        match self {
            Self::Buffer(_) | Self::Image(_) => crate::resource::ResourceSharing::Exclusive,
            Self::ConcurrentBuffer(_, families) | Self::ConcurrentImage(_, families) => {
                crate::resource::ResourceSharing::Concurrent(families)
            }
        }
    }
    pub fn with_sharing(self, sharing: crate::resource::ResourceSharing) -> Self {
        match (self, sharing) {
            (
                Self::Buffer(desc) | Self::ConcurrentBuffer(desc, _),
                crate::resource::ResourceSharing::Exclusive,
            ) => Self::Buffer(desc),
            (
                Self::Image(desc) | Self::ConcurrentImage(desc, _),
                crate::resource::ResourceSharing::Exclusive,
            ) => Self::Image(desc),
            (
                Self::Buffer(desc) | Self::ConcurrentBuffer(desc, _),
                crate::resource::ResourceSharing::Concurrent(families),
            ) => Self::ConcurrentBuffer(desc, families),
            (
                Self::Image(desc) | Self::ConcurrentImage(desc, _),
                crate::resource::ResourceSharing::Concurrent(families),
            ) => Self::ConcurrentImage(desc, families),
        }
    }
    pub(crate) fn create_resource(
        self,
        device: Arc<Device>,
        usage: u32,
    ) -> crate::Result<GpuResource> {
        match self {
            Self::Buffer(mut desc) | Self::ConcurrentBuffer(mut desc, _) => {
                desc.usage |= vk::BufferUsageFlags::from_raw(usage);
                let object = match self.sharing() {
                    crate::resource::ResourceSharing::Exclusive => {
                        buffer::Buffer::new(device, &desc)
                    }
                    crate::resource::ResourceSharing::Concurrent(families) => {
                        buffer::Buffer::new_concurrent(device, &desc, families.as_slice())
                    }
                }?;
                Ok(GpuResource::Buffer(Arc::new(object)))
            }
            Self::Image(mut desc) | Self::ConcurrentImage(mut desc, _) => {
                desc.usage |= vk::ImageUsageFlags::from_raw(usage);
                let object = match self.sharing() {
                    crate::resource::ResourceSharing::Exclusive => {
                        texture::Texture::new(device, &desc)
                    }
                    crate::resource::ResourceSharing::Concurrent(families) => {
                        texture::Texture::new_concurrent(device, &desc, families.as_slice())
                    }
                }?;
                Ok(GpuResource::Image(Arc::new(object)))
            }
        }
    }

    /// 按描述符字节布局编码（缓冲：size@0 usage@8 alignment@0xc memory_location@0xe）。
    pub fn descriptor(self) -> graph::Descriptor {
        match self {
            Self::Buffer(d) | Self::ConcurrentBuffer(d, _) => {
                graph::Descriptor::Buffer(graph::BufferDesc {
                    word_at_00: d.size,
                    word_at_08: d.usage.as_raw(),
                    // `BufferDesc`：alignment u16 @0xc、memory_location u8 @0xe。
                    bytes_at_0c_through_0e: [
                        d.alignment.to_le_bytes()[0],
                        d.alignment.to_le_bytes()[1],
                        d.memory_location.as_raw(),
                    ],
                })
            }
            Self::Image(d) | Self::ConcurrentImage(d, _) => {
                graph::Descriptor::Image(graph::TextureDesc {
                    // `TextureDesc` 的字节布局：usage@0 flags@4 format@8 width@0xc height@0x10 depth@0x14
                    // array_elements@0x18；mip_levels@0x1c sample_count@0x1d kind@0x1e。
                    words_at_00_through_18: [
                        d.usage.as_raw(),
                        d.flags.as_raw(),
                        d.format.as_raw() as u32,
                        d.width,
                        d.height,
                        d.depth,
                        d.array_elements,
                    ],
                    bytes_at_1c_through_1e: [d.mip_levels, d.sample_count, d.kind as u8],
                })
            }
        }
    }
}

#[derive(Clone)]
pub enum GpuResource {
    Buffer(Arc<buffer::Buffer>),
    Image(Arc<texture::Texture>),
}
impl GpuResource {
    pub fn generation(&self) -> crate::device::DeviceGeneration {
        match self {
            Self::Buffer(buffer) => buffer.device.generation(),
            Self::Image(image) => image.device.generation(),
        }
    }

    pub fn descriptor(&self) -> graph::Descriptor {
        match self {
            Self::Buffer(buffer) => ResourceSpec::Buffer(buffer.desc).descriptor(),
            Self::Image(image) => ResourceSpec::Image(image.desc).descriptor(),
        }
    }

    fn check_device(&self, device: &Device) -> crate::Result<()> {
        crate::device::Generationed::new((), self.generation()).into_value(device.generation())
    }
}

/// 导入覆盖完整资源：尺寸、格式、mip 等必须一致，实际 usage 可以包含额外用途。
fn validate_import_descriptor(
    declared: graph::Descriptor,
    actual: graph::Descriptor,
) -> crate::Result<()> {
    if declared.usage() & !actual.usage() != 0 || declared.with_usage(actual.usage()) != actual {
        return Err(crate::Error::invalid_usage("导入声明与实际资源描述不匹配"));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FramePoolKey {
    frame_slot: u32,
    worker: usize,
    family: u32,
}
struct FrameCommandPool {
    pool: Arc<VulkanCommandPool>,
    epoch: u64,
}

/// The alias set bound to the frame being populated.
struct ActiveAlias {
    plan: Arc<super::transient_memory::PhysicalMemoryPlan>,
    edges: Arc<[(u32, u32)]>,
}

/// Pending barrier scopes of the commands being recorded. At most a handful of
/// commands are open at once, so a short vector beats a `BTreeMap` whose
/// ~600-byte values forced a node allocation each time a command got its first
/// barrier.
#[derive(Default)]
struct CommandBarriers(Vec<(u64, crate::raw::barrier::SynchronizationScope)>);
impl CommandBarriers {
    fn entry(&mut self, command: u64) -> &mut crate::raw::barrier::SynchronizationScope {
        let at = match self.0.iter().position(|(handle, _)| *handle == command) {
            Some(at) => at,
            None => {
                self.0.push((command, Default::default()));
                self.0.len() - 1
            }
        };
        &mut self.0[at].1
    }
    fn remove(&mut self, command: u64) -> Option<crate::raw::barrier::SynchronizationScope> {
        let at = self.0.iter().position(|(handle, _)| *handle == command)?;
        Some(self.0.swap_remove(at).1)
    }
}
pub struct VulkanGraphBackend {
    pub(crate) device: Arc<Device>,
    pub timeline: Arc<TimelineSemaphore>,
    pub resources: crate::collections::token_map::TokenMap<GpuResource>,
    pub accesses: AccessTable,
    pub submissions: std::collections::VecDeque<GpuSubmissionCapture>,
    /// Set by the session right after it validated the declaration, so the
    /// trait's `prepare_declaration` does not validate the same graph twice.
    receipt_storage: Option<super::receipt_storage::ReceiptStorage>,
    active_receipt_slot: Option<super::receipt_storage::ReceiptLease>,
    specifications: BTreeMap<crate::resource::ResourceName, ResourceSpec>,
    transient_aliasing: bool,
    prepared_transients: super::transient_resources::PreparedTransients,
    /// Evictable idle pool and alias sets, registered with the device.
    caches: Arc<super::graph_caches::GraphCaches>,
    /// Members of the alias set lent to the frame being populated.
    lent_aliases: Option<super::transient_resources::LentAliases>,
    active_alias: Option<ActiveAlias>,
    /// Pending acquires recorded into a not yet submitted command; re-queued
    /// on the device if that command is abandoned.
    drained_acquires: Vec<(u64, crate::device::DrainedAcquires)>,
    transient_alias_tokens: BTreeMap<u64, super::transient_resources::AliasOrigin>,
    transient_counters: Arc<crate::raw::transient_backing::TransientBackingCounters>,
    cpu_arenas: crate::command::CpuArenaPool,
    owner_pool: crate::raw::temp_list_pool::TempListPool<crate::raw::binding::RetainedResource>,
    frame_storage: super::frame_storage::FrameStoragePool,
    import_validation: parking_lot::Mutex<crate::raw::temp_list::TempList<(u8, u64)>>,
    frame_pools: BTreeMap<FramePoolKey, FrameCommandPool>,
    recording_epoch: u64,
    active_frame_slot: u32,
    /// Set once the session uses explicit display frames; doubles the command
    /// slots (see `COMMAND_SLOTS_PER_DISPLAY_FRAME`). Sessions that only call
    /// `begin_frame` keep one slot per in-flight frame.
    display_frames: bool,
    commands: BTreeMap<u64, VulkanCommandBuffer>,
    queue: recording::Queue,
    family: u32,
    recording_family: u32,
    recording_queue: crate::sync::QueueId,
    gpu_timing: Option<super::profiling::GpuTimingRing>,
    queue_runtime: queue_runtime::QueueRuntime,
    queue_timelines: Vec<Arc<TimelineSemaphore>>,
    import_families: crate::collections::token_map::TokenMap<u32>,
    history_validity: crate::collections::token_map::TokenMap<bool>,
    synchronization: BTreeMap<usize, OwnedBatchSync>,
    graph_waits: Vec<TimelineWait>,
    graph_signals: Vec<TimelineSignal>,
    /// The acquired swapchain image while its presentation graph executes.
    presentation: Option<presentation::BackendPresentation>,
    recycled_commands: Vec<u64>,
    /// A backend selects one actual queue family; each frame and recording worker
    /// receives a separate pool within that family.
    command_slot: BTreeMap<u64, FramePoolKey>,
    pub stores: ManagedResources,
    barrier_dispatch: crate::raw::barrier::BarrierDispatch,
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

/// `submissions` 捕获的上限：只保留最近的提交，避免不经 `begin_frame` 的长时间运行无界增长。
pub const SUBMISSION_CAPTURE_LIMIT: usize = 64;
pub struct GpuSubmissionCapture {
    pub queue: vk::Queue,
    pub batches: smallvec::SmallVec<[submit::PreparedSubmit; 1]>,
    pub result: i32,
    // Owned arrays are destroyed before their budget reservation is released.
    _memory: Option<crate::raw::allocator::CpuMemoryReservation>,
}
impl GpuSubmissionCapture {
    fn refresh_budget(
        &mut self,
        stats: &Arc<crate::raw::allocator::AllocatorStats>,
    ) -> crate::Result<()> {
        let bytes = self
            .batches
            .iter()
            .map(submit::PreparedSubmit::heap_capacity_bytes)
            .sum::<usize>()
            + if self.batches.spilled() {
                self.batches.capacity() * std::mem::size_of::<submit::PreparedSubmit>()
            } else {
                0
            };
        if let Some(memory) = &mut self._memory {
            memory.resize(bytes as u64)?;
        } else if bytes != 0 {
            self._memory =
                Some(stats.reserve_cpu_bytes(bytes as u64, crate::device::BudgetCategory::Cache)?);
        }
        Ok(())
    }
    fn try_new(
        queue: vk::Queue,
        prepared: &[submit::PreparedSubmit],
        extra_waits: usize,
        stats: &Arc<crate::raw::allocator::AllocatorStats>,
    ) -> crate::Result<Self> {
        let outer_bytes = if prepared.len() > 1 {
            std::alloc::Layout::array::<submit::PreparedSubmit>(prepared.len())
                .map_err(|_| crate::Error::invalid_usage("submission capture size overflows"))?
                .size()
        } else {
            0
        };
        let bytes = prepared
            .iter()
            .enumerate()
            .try_fold(outer_bytes, |sum, (index, batch)| {
                sum.checked_add(batch.copied_heap_bytes(if index == 0 {
                    extra_waits
                } else {
                    0
                })?)
                .ok_or_else(|| crate::Error::invalid_usage("submission capture storage overflows"))
            })?;
        let mut memory = if bytes == 0 {
            None
        } else {
            Some(stats.reserve_cpu_bytes(bytes as u64, crate::device::BudgetCategory::Cache)?)
        };
        let mut batches = smallvec::SmallVec::<[submit::PreparedSubmit; 1]>::new();
        batches
            .try_reserve_exact(prepared.len())
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        for (index, batch) in prepared.iter().enumerate() {
            batches.push(batch.try_clone_with_wait_capacity(if index == 0 {
                extra_waits
            } else {
                0
            })?);
        }
        if let Some(memory) = &mut memory {
            let actual = batches
                .iter()
                .map(submit::PreparedSubmit::heap_capacity_bytes)
                .sum::<usize>()
                + if batches.spilled() {
                    batches.capacity() * std::mem::size_of::<submit::PreparedSubmit>()
                } else {
                    0
                };
            memory.resize(actual as u64)?;
        }
        Ok(Self {
            queue,
            batches,
            result: 0,
            _memory: memory,
        })
    }
}
pub type TimelineWait = (Arc<TimelineSemaphore>, u64, vk::PipelineStageFlags);
pub type TimelineSignal = (Arc<TimelineSemaphore>, u64);
#[derive(Default)]
struct OwnedBatchSync {
    waits: Vec<TimelineWait>,
    signals: Vec<TimelineSignal>,
}
/// Emit exactly the native range chosen by the bound synchronization program.
fn add_transition(
    scope: &mut crate::raw::barrier::SynchronizationScope,
    resources: &crate::collections::token_map::TokenMap<GpuResource>,
    token: u64,
    transition: &super::sync_plan::BoundTransition,
    present: Option<presentation::PresentPatch>,
) -> crate::Result<()> {
    use crate::raw::barrier::{BarrierTarget, RangeBarrier};
    presentation::PresentPatch::apply(present, scope, token, transition);
    // Alias (shared backing) dependencies are ordinary transitions here: the
    // planner models them as a synthetic previous access of the new member.
    scope.source_stages |= transition.source.stages;
    scope.destination_stages |= transition.destination.stages;
    let target = match resources.get(&token) {
        Some(GpuResource::Buffer(buffer)) => BarrierTarget::Buffer(buffer.handle),
        Some(GpuResource::Image(image)) => BarrierTarget::Image(image.handle),
        None => {
            return Err(crate::Error::invalid_usage(
                "native barrier resource is missing",
            ));
        }
    };
    scope.push(
        target,
        &RangeBarrier {
            range: transition.range,
            source_access: transition.source.access,
            destination_access: transition.destination.access,
            old_layout: transition.old_layout,
            new_layout: transition.new_layout,
            source_family: transition.source_family,
            destination_family: transition.destination_family,
        },
    )
}
/// 并行录制时各块共享的只读视图：只读访问表与资源表，直接把每个 pass 的屏障写入块自己的命令缓冲。
struct VulkanChunkView<'a> {
    device: &'a ash::Device,
    present_patch: Option<presentation::PresentPatch>,
    barrier_device: &'a Device,
    barrier_dispatch: crate::raw::barrier::BarrierDispatch,
    resources: &'a crate::collections::token_map::TokenMap<GpuResource>,
    cpu_arenas: &'a crate::command::CpuArenaPool,
    owner_pool: &'a crate::raw::temp_list_pool::TempListPool<crate::raw::binding::RetainedResource>,
}
impl super::executor::ChunkRecorder for VulkanChunkView<'_> {
    unsafe fn write_pass_timestamp(
        &self,
        lease: &mut super::executor::ChunkLease<'_>,
        pair: super::profiling::GpuPassTimestamp,
        end: bool,
    ) {
        pair.write(
            self.device,
            vk::CommandBuffer::from_raw(lease.command()),
            end,
        );
    }
    fn owner_storage_pool(
        &self,
    ) -> Option<&crate::raw::temp_list_pool::TempListPool<crate::raw::binding::RetainedResource>>
    {
        Some(self.owner_pool)
    }

    fn cpu_arena_pool(&self) -> Option<&crate::command::CpuArenaPool> {
        Some(self.cpu_arenas)
    }
    fn record_transitions(
        &self,
        lease: &mut super::executor::ChunkLease<'_>,
        transitions: &[super::executor::ChunkTransition],
        owners: &mut super::owner_storage::ResourceOwners,
    ) -> crate::Result<()> {
        if transitions.is_empty() {
            return Ok(());
        }
        let mut scope = crate::raw::barrier::SynchronizationScope::default();
        for t in transitions {
            // These references are covered by the exact native program and
            // its preflight StatePatches, rather than opaque whole-resource uses.
            match self.resources.get(&t.token) {
                Some(GpuResource::Buffer(buffer)) => owners.add_recorded(buffer.clone())?,
                Some(GpuResource::Image(image)) => owners.add_recorded(image.clone())?,
                None => {
                    return Err(crate::Error::invalid_usage(
                        "parallel barrier resource is missing",
                    ));
                }
            }
            add_transition(
                &mut scope,
                self.resources,
                t.token,
                &t.transition,
                self.present_patch,
            )?;
        }
        // 每个 pass 一次屏障命令。
        unsafe {
            self.barrier_dispatch.issue(
                self.barrier_device,
                vk::CommandBuffer::from_raw(lease.command()),
                &scope,
            )
        }
    }
}
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
        let stores = device.caches.resources.clone();
        let cpu_arenas = crate::command::CpuArenaPool::new(
            crate::command::CpuArenaConfig::default(),
            device.memory.stats.clone(),
        )?;
        let owner_pool = crate::raw::temp_list_pool::TempListPool::new(
            Default::default(),
            device.memory.stats.clone(),
        )?;
        let frame_storage = super::frame_storage::FrameStoragePool::new(
            Default::default(),
            device.memory.stats.clone(),
        )?;
        let import_validation = parking_lot::Mutex::new(crate::raw::temp_list::TempList::new(
            65_536,
            device.memory.stats.clone(),
        )?);
        let projection_pool = super::pass_bindings::ManagedProjectionPool::new(
            device.generation(),
            device.memory.stats.clone(),
            Default::default(),
        )?;
        let queue_runtime = queue_runtime::QueueRuntime::new(&device)?;
        let barrier_dispatch = crate::raw::barrier::BarrierDispatch::new(&device);
        let caches = Arc::new(super::graph_caches::GraphCaches::new(projection_pool));
        // Weak registration: the backend owns the caches, so they unregister
        // when it drops and the device never keeps them (or itself) alive.
        let evictable: Arc<dyn crate::device::Evictable> = caches.clone();
        device.register_evictable(
            Arc::downgrade(&evictable),
            crate::device::EvictionPriority::Transient,
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
            accesses,
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
    pub fn managed_projection_stats(&self) -> super::pass_bindings::ManagedProjectionStats {
        self.caches.projection().pool.stats()
    }
    pub fn set_managed_projection_limits(
        &mut self,
        config: super::pass_bindings::ManagedProjectionConfig,
    ) -> crate::Result<()> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let replacement = super::pass_bindings::ManagedProjectionPool::new(
            self.device.generation(),
            self.device.memory.stats.clone(),
            config,
        )?;
        let mut projection = self.caches.projection();
        projection.snapshot.take();
        projection.pool = replacement;
        Ok(())
    }
    fn clear_pass_resources(&mut self) {
        let mut projection = self.caches.projection();
        projection.snapshot.take();
        projection.pool.clear_idle();
    }
    /// Physical aliasing of transient resources (on by default). Aliasing only
    /// uses orderings already in the graph; it never adds DAG edges.
    pub fn set_transient_aliasing(&mut self, enabled: bool) {
        self.transient_aliasing = enabled;
        if !enabled {
            self.return_unconsumed_aliases();
            self.active_alias = None;
            self.caches.alias().trim();
        }
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
    /// Optional production command-span timestamps. Results are per physical
    /// queue; their sum is not multi-queue frame latency or pure GPU active time.
    pub fn configure_gpu_timing(
        &mut self,
        config: Option<super::profiling::GpuTimingConfig>,
    ) -> crate::Result<()> {
        if match (&self.gpu_timing, config) {
            (None, None) => true,
            (Some(ring), Some(config)) => ring.matches_config(config),
            _ => false,
        } {
            return if self.device.is_device_lost() { Err(crate::Error::DeviceLost) } else { Ok(()) };
        }
        self.poll_gpu_timings()?;
        if self
            .gpu_timing
            .as_ref()
            .is_some_and(|ring| ring.stats().pending != 0)
        {
            return Err(crate::Error::invalid_usage(
                "GPU timing still owns recorded or in-flight queries",
            ));
        }
        let replacement = config
            .map(|config| super::profiling::GpuTimingRing::new(self.device.clone(), config))
            .transpose()?;
        self.gpu_timing = replacement;
        Ok(())
    }
    pub fn poll_gpu_timings(&mut self) -> crate::Result<Option<super::profiling::GpuTimingStats>> {
        self.gpu_timing
            .as_mut()
            .map(|ring| {
                ring.poll()?;
                Ok(ring.stats())
            })
            .transpose()
    }
    pub fn gpu_timings(&self) -> impl Iterator<Item = &super::profiling::GpuCommandTiming> {
        self.gpu_timing.iter().flat_map(|ring| ring.samples())
    }
    pub fn drain_gpu_timings(
        &mut self,
    ) -> impl Iterator<Item = super::profiling::GpuCommandTiming> + '_ {
        self.gpu_timing
            .iter_mut()
            .flat_map(|ring| ring.drain_samples())
    }
    pub fn transient_memory_plan(&self) -> &super::transient_memory::PhysicalMemoryPlan {
        self.active_alias
            .as_ref()
            .map_or(&self.prepared_transients.plan, |active| &*active.plan)
    }
    /// Alias-set members handed back while their GPU work may still run;
    /// they rejoin their set (and the set becomes reusable) once idle.
    pub fn transient_alias_retiring(&self) -> usize {
        self.caches.alias().retiring_count()
    }
    pub fn transient_backing_statistics(
        &self,
    ) -> crate::raw::transient_backing::TransientBackingStatistics {
        self.transient_counters.snapshot()
    }
    pub fn transient_backing_counters(
        &self,
    ) -> Arc<crate::raw::transient_backing::TransientBackingCounters> {
        self.transient_counters.clone()
    }
    /// Command pool slots: one per in-flight graph. Once display frames are in
    /// use each of the `frames_in_flight` display frames may hold two graphs.
    fn command_slot_count(&self) -> usize {
        self.device.frames_in_flight() as usize
            * if self.display_frames {
                COMMAND_SLOTS_PER_DISPLAY_FRAME as usize
            } else {
                1
            }
    }

    /// A new graph records into an available frame slot. CPU leases, rather than
    /// guessed frame counters, prevent resetting a pool still retained by a frame.
    fn begin_command_frame(&mut self) -> crate::Result<()> {
        self.poll_gpu_timings()?;
        if !matches!(
            self.device.memory.stats.budget_action(),
            crate::device::BudgetAction::None
        ) {
            self.cpu_arenas.trim_idle(0);
            self.owner_pool.trim_idle(0);
            self.frame_storage.trim_idle(0);
            self.import_validation.get_mut().trim_idle();
            self.prepared_transients.trim_unaliased();
            self.caches.alias().trim();
        }
        // One slot per graph, not per display frame: a display frame holds the
        // main and the presentation graph, and `frames_in_flight` display frames
        // may be in flight at once.
        let slots = self.command_slot_count() as u32;
        let selected = (1..=slots)
            .map(|offset| (self.active_frame_slot + offset) % slots)
            .find(|slot| {
                !self.command_slot.iter().any(|(handle, key)| {
                    key.frame_slot == *slot && !self.recycled_commands.contains(handle)
                })
            })
            .ok_or_else(|| crate::Error::from(vk::Result::NOT_READY))?;
        self.recording_epoch = self
            .recording_epoch
            .checked_add(1)
            .ok_or_else(|| crate::Error::unsupported("command frame epoch exhausted"))?;
        self.active_frame_slot = selected;
        Ok(())
    }

    pub fn cpu_arena_pool(&self) -> &crate::command::CpuArenaPool {
        &self.cpu_arenas
    }
    pub fn cpu_arena_statistics(&self) -> crate::command::CpuArenaStats {
        self.cpu_arenas.stats()
    }
    pub fn configure_cpu_arenas(
        &mut self,
        config: crate::command::CpuArenaConfig,
    ) -> crate::Result<()> {
        let replacement =
            crate::command::CpuArenaPool::new(config, self.device.memory.stats.clone())?;
        self.cpu_arenas = replacement;
        Ok(())
    }

    pub fn owner_storage_statistics(&self) -> crate::raw::temp_list_pool::TempListPoolStats {
        self.owner_pool.statistics()
    }
    pub fn configure_owner_storage(
        &mut self,
        config: crate::raw::temp_list_pool::TempListPoolConfig,
    ) -> crate::Result<()> {
        self.owner_pool = crate::raw::temp_list_pool::TempListPool::new(
            config,
            self.device.memory.stats.clone(),
        )?;
        Ok(())
    }
    pub fn frame_storage_statistics(&self) -> super::frame_storage::FrameStorageStats {
        self.frame_storage.stats()
    }
    pub fn configure_frame_storage(
        &mut self,
        config: super::frame_storage::FrameStorageConfig,
    ) -> crate::Result<()> {
        self.frame_storage =
            super::frame_storage::FrameStoragePool::new(config, self.device.memory.stats.clone())?;
        Ok(())
    }

    pub fn command_pool_count(&self) -> usize {
        self.frame_pools.len()
    }
    pub fn command_pool_reset_count(&self) -> u64 {
        self.frame_pools
            .values()
            .map(|entry| entry.pool.reset_count())
            .sum()
    }

    fn command_for_worker(&mut self, worker: usize) -> crate::Result<u64> {
        let key = FramePoolKey {
            frame_slot: self.active_frame_slot,
            worker,
            family: self.recording_family,
        };
        if !self.frame_pools.contains_key(&key) {
            let pool =
                VulkanCommandPool::new_for_family(self.device.clone(), self.recording_family)?;
            self.frame_pools.insert(
                key,
                FrameCommandPool {
                    pool,
                    epoch: self.recording_epoch,
                },
            );
        }
        let entry = self
            .frame_pools
            .get(&key)
            .expect("inserted frame command pool");
        let pool = entry.pool.clone();
        if entry.epoch != self.recording_epoch {
            if self
                .command_slot
                .iter()
                .any(|(handle, slot)| *slot == key && !self.recycled_commands.contains(handle))
            {
                return Err(vk::Result::NOT_READY.into());
            }
            let mut commands = self
                .commands
                .values_mut()
                .filter(|command| Arc::ptr_eq(&command.pool, &pool))
                .collect::<smallvec::SmallVec<[&mut VulkanCommandBuffer; 4]>>();
            pool.reset_completed(&mut commands)?;
            self.frame_pools
                .get_mut(&key)
                .expect("existing frame command pool")
                .epoch = self.recording_epoch;
        }
        let position = self.recycled_commands.iter().rposition(|handle| {
            self.command_slot.get(handle) == Some(&key)
                && self
                    .commands
                    .get(handle)
                    .is_some_and(|command| !command.is_recording() && !command.is_executable())
                && self
                    .gpu_timing
                    .as_ref()
                    .is_none_or(|ring| !ring.is_tracking_handle(*handle))
        });
        if let Some(position) = position {
            return Ok(self.recycled_commands.remove(position));
        }
        let command = pool.allocate()?;
        let handle = command.handle.as_raw();
        self.commands.insert(handle, command);
        self.command_slot.insert(handle, key);
        Ok(handle)
    }
    pub fn selected_family(&self) -> u32 {
        self.family
    }
    /// 惰性创建顶层时间戳查询池（两条查询：begin/end）。
    fn check_synchronization(
        &self,
        waits: &[TimelineWait],
        signals: &[TimelineSignal],
    ) -> crate::Result<()> {
        if waits
            .iter()
            .any(|(s, _, stage)| !Arc::ptr_eq(&s.device, &self.device) || stage.is_empty())
            || signals
                .iter()
                .any(|(s, _)| !Arc::ptr_eq(&s.device, &self.device))
        {
            return Err(crate::Error::invalid_usage(
                "外部时间线必须来自同一设备且等待阶段不能为空",
            ));
        }
        if waits
            .iter()
            .map(|(s, _, _)| s.handle)
            .chain(signals.iter().map(|(s, _)| s.handle))
            .any(|handle| self.queue_timelines.iter().any(|own| own.handle == handle))
        {
            return Err(crate::Error::invalid_usage(
                "队列内部时间线由提交生产者管理，不能重复作为外部同步输入",
            ));
        }
        Ok(())
    }
    /// 图依赖只等待首批次，图完成信号只写入最后批次，semaphore Arc 保留到回执持有者释放。
    pub fn set_graph_synchronization(
        &mut self,
        waits: Vec<TimelineWait>,
        signals: Vec<TimelineSignal>,
    ) -> crate::Result<()> {
        self.check_synchronization(&waits, &signals)?;
        self.graph_waits = waits;
        self.graph_signals = signals;
        Ok(())
    }
    pub fn declare(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        spec: ResourceSpec,
    ) -> Result<super::resource::ResourceHandle, graph::BuildError> {
        let descriptor = spec.descriptor();
        if self.specifications.contains_key(&crate::resource::ResourceName::new(name)) {
            return Err(graph::BuildError::DuplicateResourceName(name.into()));
        }
        let handle = graph.create_raw_resource(
            name,
            ResourceOrigin {
                descriptor,
                source: graph::OriginSource::Create,
            },
        )?;
        graph.declaration.resources[handle.raw() as usize].sharing = spec.sharing();
        self.specifications.insert(name.into(), spec);
        Ok(handle)
    }

    /// Change a not-yet-instantiated declaration and its allocation specification
    /// together. Imported objects retain the sharing mode chosen at creation.
    pub fn set_declared_sharing(
        &mut self,
        built: &mut graph::BuiltGraph,
        name: &str,
        sharing: crate::resource::ResourceSharing,
    ) -> crate::Result<()> {
        sharing.validate_device(&self.device)?;
        let declaration = built
            .declaration
            .resources
            .iter_mut()
            .find(|resource| resource.name == name)
            .ok_or_else(|| {
                crate::Error::invalid_usage("sharing override names an unknown resource")
            })?;
        if !matches!(declaration.source, graph::OriginSource::Create) {
            return Err(crate::Error::invalid_usage(
                "sharing cannot be changed after physical resource creation",
            ));
        }
        let specification = self.specifications.get_mut(&crate::resource::ResourceName::new(name)).ok_or_else(|| {
            crate::Error::invalid_usage("sharing override has no allocation specification")
        })?;
        *specification = specification.with_sharing(sharing);
        declaration.sharing = sharing;
        Ok(())
    }
    pub fn resource_clone(&self, token: u64) -> Option<GpuResource> {
        self.resources.get(&token).cloned()
    }

    fn collect_completed_managed(&mut self) -> crate::Result<()> {
        self.device.try_collect_garbage().map(|_| ())
    }

    /// 跨帧沿用 token 和真实 Arc，不以 token 猜测 Vulkan 句柄或 generation。
    pub fn bind_import_named(
        &mut self,
        name: &str,
        token: u64,
        resource: GpuResource,
    ) -> crate::Result<()> {
        self.bind_import_named_owned_by(name, token, resource, self.device.queues.primary_family)
    }
    /// 已有独占资源须声明真实归属；不为跨队列族资源伪造 release/acquire。
    pub fn bind_import_named_owned_by(
        &mut self,
        _name: &str,
        token: u64,
        resource: GpuResource,
        owner_family: u32,
    ) -> crate::Result<()> {
        if !(GENERATED_TOKEN_LIMIT..u64::MAX).contains(&token) {
            return Err(crate::Error::invalid_usage("显式导入 token 必须使用宿主高位域"));
        }
        self.bind_existing_token(token, resource, owner_family)
    }
    // Only allocator-issued internal owners can rebind the generated domain.
    // External imports enter through the disjoint host domain above.
    fn bind_existing_token(
        &mut self,
        token: u64,
        resource: GpuResource,
        owner_family: u32,
    ) -> crate::Result<()> {
        resource.check_device(&self.device)?;
        if token == 0 || token == u64::MAX {
            return Err(crate::Error::invalid_usage("跨帧资源 token 无效"));
        }
        if let Some(old) = self.resources.get(&token) {
            let same = match (old, &resource) {
                (GpuResource::Buffer(a), GpuResource::Buffer(b)) => Arc::ptr_eq(a, b),
                (GpuResource::Image(a), GpuResource::Image(b)) => Arc::ptr_eq(a, b),
                _ => false,
            };
            if !same {
                return Err(crate::Error::invalid_usage("token 已经属于其他物理资源"));
            }
        }
        self.resources.insert(token, resource);
        self.import_families.insert(token, owner_family);
        Ok(())
    }
    pub fn bind_import(&mut self, resource: GpuResource) -> crate::Result<u64> {
        self.bind_import_owned_by(resource, self.device.queues.primary_family)
    }
    pub fn bind_import_owned_by(
        &mut self,
        resource: GpuResource,
        owner_family: u32,
    ) -> crate::Result<u64> {
        resource.check_device(&self.device)?;
        let token =
            try_allocate_token().ok_or_else(|| crate::Error::unsupported("资源 token 空间耗尽"))?;
        self.resources.insert(token, resource);
        self.import_families.insert(token, owner_family);
        Ok(token)
    }
    fn validate_graph(&self, declaration: &graph::GraphDeclaration) -> crate::Result<()> {
        self.import_validation.lock().with_reserved(
            declaration.resources.len(),
            |physical_imports| -> crate::Result<()> {
                for resource in &declaration.resources {
                    if let graph::OriginSource::ImportOwned { token, .. }
                    | graph::OriginSource::ImportUnowned { token, .. } = resource.source
                    {
                        let actual = self.resources.get(&token).ok_or_else(|| {
                            crate::Error::invalid_usage("导入 token 没有绑定实际资源")
                        })?;
                        actual.check_device(&self.device)?;
                        let identity = match actual {
                            GpuResource::Buffer(buffer) => (0_u8, buffer.handle.as_raw()),
                            GpuResource::Image(image) => (1_u8, image.handle.as_raw()),
                        };
                        physical_imports.push(identity);
                        validate_import_descriptor(resource.descriptor, actual.descriptor())?;
                        if actual.sharing_mode() != resource.sharing {
                            return Err(crate::Error::invalid_usage(
                                "import sharing mode does not match its physical resource",
                            ));
                        }
                        if !self.import_families.contains_key(&token) {
                            return Err(crate::Error::invalid_usage(
                                "import has no declared queue family",
                            ));
                        }
                    }
                }
                physical_imports.sort_unstable();
                if physical_imports.windows(2).any(|pair| pair[0] == pair[1]) {
                    return Err(crate::Error::invalid_usage(
                        "同一物理资源被多个逻辑资源重复导入；缺少显式别名计划",
                    ));
                }
                Ok(())
            },
        )??;
        for code in declaration
            .passes
            .iter()
            .flat_map(|p| &p.accesses)
            .map(|a| a.access_code)
            .chain(declaration.exports.iter().map(|e| e.access_code))
            .chain(declaration.resources.iter().filter_map(|r| match r.source {
                graph::OriginSource::Create => None,
                graph::OriginSource::ImportOwned { initial_access, .. }
                | graph::OriginSource::ImportUnowned { initial_access, .. } => Some(initial_access),
            }))
        {
            if !self.accesses.0.contains_key(&code) {
                return Err(crate::Error::invalid_usage(format!(
                    "缺少访问码 {code} 的显式 Vulkan 映射"
                )));
            }
        }
        Ok(())
    }
    fn validate_batch_sync(&self, total: usize) -> crate::Result<()> {
        if self.synchronization.keys().any(|&index| index >= total)
            || (total == 0 && (!self.graph_waits.is_empty() || !self.graph_signals.is_empty()))
        {
            return Err(crate::Error::invalid_usage("同步计划指向不存在的图批次"));
        }
        let mut last_signals = BTreeMap::new();
        for index in 0..total {
            let plan = self.batch_synchronization(index, total);
            for signal in &plan.signals {
                let previous = if let Some(value) = last_signals.get(&signal.semaphore.as_raw()) {
                    *value
                } else {
                    unsafe {
                        self.device
                            .core
                            .device
                            .get_semaphore_counter_value(signal.semaphore)
                    }
                    .map_err(crate::Error::from)?
                };
                let required = plan
                    .waits
                    .iter()
                    .filter(|w| w.semaphore == signal.semaphore)
                    .map(|w| w.value)
                    .max()
                    .unwrap_or(0)
                    .max(previous);
                if signal.value <= required {
                    return Err(crate::Error::invalid_usage(
                        "图时间线信号必须严格大于该批等待值和前次信号值",
                    ));
                }
                last_signals.insert(signal.semaphore.as_raw(), signal.value);
            }
        }
        Ok(())
    }
    fn new_execution_chain(&self) -> ExecutionChain {
        let family_count = self
            .device
            .queues
            .additional
            .iter()
            .map(|q| q.family)
            .chain(std::iter::once(self.device.queues.primary_family))
            .max()
            .unwrap_or(0)
            + 1;
        let chain = ExecutionChain::new(
            self.device.queues.primary,
            self.device
                .queues
                .additional
                .iter()
                .map(|q| q.handle)
                .collect(),
            family_count,
        );
        chain
    }
}

impl graph::ResourceBackend for VulkanGraphBackend {
    fn frame_storage_pool(&self) -> Option<&super::frame_storage::FrameStoragePool> {
        Some(&self.frame_storage)
    }

    fn history_validity(&self, token: u64) -> Option<bool> {
        self.history_validity.get(&token).copied()
    }

    fn device_generation(&self) -> Option<crate::device::DeviceGeneration> {
        Some(self.device.generation())
    }
    fn prepare_transient_resources(
        &mut self,
        plan: &super::plan::CompiledPlan,
        graph: &graph::GraphDeclaration,
    ) -> crate::Result<()> {
        self.prepare_queue_plan(plan, graph)?;
        self.validate_batch_sync(plan.batches.len())?;
        // Drop any unused candidates from an earlier failed population before
        // allocating this frame. Already-published tokens belong to that frame.
        if self.transient_aliasing {
            self.return_unconsumed_aliases();
            self.active_alias = None;
            let (lent, evicted) = self.caches.alias().lend(
                &self.device,
                &self.specifications,
                plan,
                graph,
                self.transient_counters.clone(),
            )?;
            // The cache guard ended at the statement boundary. Native owners
            // removed by LRU eviction are retired without holding its mutex.
            drop(evicted);
            self.active_alias = lent.as_ref().map(|lent| ActiveAlias {
                plan: lent.plan.clone(),
                edges: lent.edges.clone(),
            });
            self.lent_aliases = lent;
        } else {
            self.prepared_transients
                .prepare_unaliased(&plan.dependency_edges, &self.device.memory.stats)?;
        }
        Ok(())
    }
    fn has_prepared_resource(&self, name: &str) -> bool {
        self.lent_aliases
            .as_ref()
            .is_some_and(|lent| lent.resources.contains_key(&crate::resource::ResourceName::new(name)))
    }
    fn is_transient_alias(&self, token: u64) -> bool {
        self.transient_alias_tokens.contains_key(&token)
    }
    fn claim_pooled(&mut self, token: u64, sharing: crate::resource::ResourceSharing) -> bool {
        if let Some(resource) = self.resources.get(&token) {
            return resource.sharing_mode() == sharing;
        }
        // Parked idle resource: take it back for this frame. A token the
        // evictor already released is not found and is refused.
        match self.caches.claim(token, sharing) {
            Some((resource, family)) => {
                self.resources.insert(token, resource);
                if let Some(family) = family {
                    self.import_families.insert(token, family);
                }
                true
            }
            None => false,
        }
    }
    fn park_pooled(&mut self, token: u64) {
        if self.transient_alias_tokens.contains_key(&token) {
            return;
        }
        if let Some(resource) = self.resources.remove(&token) {
            let family = self.import_families.remove(&token);
            self.caches.park(token, resource, family);
        }
    }
    fn create_without_initial_data(
        &mut self,
        descriptor: graph::Descriptor,
        name: &str,
    ) -> Result<u64, graph::BackendFailure> {
        let prepared = self
            .lent_aliases
            .as_mut()
            .and_then(|lent| lent.resources.remove(&crate::resource::ResourceName::new(name)));
        let origin = prepared.as_ref().map(|(_, origin)| *origin);
        let resource = if let Some((resource, _)) = prepared {
            resource
        } else {
            self.specifications
                .get(&crate::resource::ResourceName::new(name))
                .copied()
                .ok_or_else(|| crate::Error::invalid_usage(format!("缺少资源规格 {name}")))?
                .create_resource(self.device.clone(), descriptor.usage())?
        };
        let token = match self.bind_import_owned_by(resource.clone(), self.family) {
            Ok(token) => token,
            Err(error) => {
                if let Some(origin) = origin {
                    self.caches
                        .alias()
                        .give_back(origin, resource, &self.device.completed());
                }
                return Err(error);
            }
        };
        if let Some(origin) = origin {
            self.transient_alias_tokens.insert(token, origin);
        }
        Ok(token)
    }
    fn set_debug_name(&mut self, _token: u64, _descriptor: graph::Descriptor, _name: &str) {}
    fn release_created_resource(&mut self, token: u64) {
        // 只在 GPU 不会再用到该资源时调用（见 graph::reclaim_failed_resources）；命令拥有者仍持有的 Arc 不受影响。
        let resource = self.resources.remove(&token);
        if let (Some(origin), Some(resource)) =
            (self.transient_alias_tokens.remove(&token), resource)
        {
            // Completion is re-checked: a member of a partially submitted
            // frame rejoins its set only after its GPU work finished.
            self.caches
                .alias()
                .give_back(origin, resource, &self.device.completed());
        }
        self.import_families.remove(&token);
    }
}
impl ExecutionBackend for VulkanGraphBackend {
    fn reserve_pass_timestamp(
        &mut self,
        command: u64,
        pass: usize,
        name: &crate::resource::ResourceName,
    ) -> Option<super::profiling::GpuPassTimestamp> {
        self.gpu_timing.as_mut()?.reserve_pass(command, pass, name)
    }
    unsafe fn write_pass_timestamp(
        &self,
        command: u64,
        pair: super::profiling::GpuPassTimestamp,
        end: bool,
    ) {
        pair.write(
            &self.device.core.device,
            vk::CommandBuffer::from_raw(command),
            end,
        );
    }
    fn owner_storage_pool(
        &self,
    ) -> Option<&crate::raw::temp_list_pool::TempListPool<crate::raw::binding::RetainedResource>>
    {
        Some(&self.owner_pool)
    }

    fn requires_graph_completion(&self, batch: usize, total: usize) -> bool {
        batch + 1 == total && !self.graph_signals.is_empty()
    }
    fn select_queue_domain(
        &mut self,
        domain: super::queue_schedule::QueueDomain,
    ) -> crate::Result<()> {
        let family = if domain.queue_index == 0 {
            self.device.queues.primary_family
        } else {
            self.device
                .queues
                .additional
                .get(domain.queue_index as usize - 1)
                .ok_or_else(|| crate::Error::invalid_usage("scheduled queue does not exist"))?
                .family
        };
        if domain.family != family {
            return Err(crate::Error::invalid_usage(
                "scheduled queue family differs from actual queue",
            ));
        }
        self.recording_family = family;
        self.recording_queue =
            crate::sync::QueueId::new(domain.queue_index as usize).ok_or_else(|| {
                crate::Error::invalid_usage("scheduled queue index exceeds device limits")
            })?;
        Ok(())
    }
    fn next_queue_prelude(
        &mut self,
        compiled: &graph::FrameInstance,
        batch: usize,
    ) -> crate::Result<Option<super::executor::QueuePrelude>> {
        let prelude = self.next_prelude(compiled, batch)?;
        if let Some(prelude) = &prelude {
            self.select_presentation_batch(prelude.tag as usize);
        }
        Ok(prelude)
    }
    fn record_queue_prelude(
        &mut self,
        command: u64,
        prelude: super::executor::QueuePrelude,
    ) -> crate::Result<()> {
        self.record_prelude(command, prelude)
    }
    fn select_main_batch(
        &mut self,
        _compiled: &graph::FrameInstance,
        batch: usize,
    ) -> crate::Result<()> {
        self.queue_runtime.select_main_batch(batch)?;
        self.select_presentation_batch(batch);
        Ok(())
    }
    fn timeline_wait(&self, point: crate::sync::SyncPoint) -> crate::Result<GraphTimelineWait> {
        self.queue_timeline_wait(point)
    }
    fn cpu_arena_pool(&self) -> Option<&crate::command::CpuArenaPool> {
        Some(&self.cpu_arenas)
    }
    fn plan_context(&self) -> super::plan::PlanContext {
        super::plan::PlanContext {
            queue_domains: Some(
                crate::device::QueueClass::ALL.map(|class| self.domain_for_class(class)),
            ),
            device_generation: self.device.generation().value(),
            queue_family: self.family,
            queue_index: match self.queue {
                recording::Queue::Primary => 0,
                recording::Queue::Auxiliary(i) => i + 1,
            },
            policy_signature: u64::from(self.transient_aliasing),
            // Only the actually enabled feature lets depth and stencil be
            // tracked (and transitioned) independently; otherwise the planner
            // merges partial D/S ranges and uses combined layouts.
            separate_depth_stencil_layouts: self
                .device
                .caps()
                .is_enabled(crate::device::Feature::SeparateDepthStencilLayouts),
        }
    }
    fn prepare_declaration(
        &mut self,
        declaration: &graph::GraphDeclaration,
    ) -> crate::Result<()> {
        let result = self.validate_graph(declaration)
            .and_then(|()| self.begin_command_frame());
        if result.as_ref().is_err_and(|error| error.is_device_lost()) {
            self.device.queues.timelines.mark_device_lost();
        }
        result
    }
    fn synchronization_plan(&self) -> Arc<super::sync_plan::SynchronizationPlanner> {
        self.queue_runtime.planner()
    }
    fn prepare_batch_submission(&mut self, commands: &[u64], batch: usize) -> crate::Result<()> {
        self.attach_range_patches(commands, batch)
    }
    fn prepare_compiled_resources(&mut self, compiled: &graph::FrameInstance) -> crate::Result<()> {
        self.bind_queue_resources(compiled)?;
        self.resolve_presentation(compiled)?;
        if let Some(slot) = &mut self.active_receipt_slot {
            slot.reserve_points(self.queue_runtime.planner().bound().batches().len())?;
        }
        // Commands own GPU resources; external Arc/Weak owners pin the snapshot.
        let mut projection = self.caches.projection();
        projection.snapshot.take();
        projection.snapshot = Some(projection.pool.checkout(
            &self.device,
            &compiled.registry,
            &self.stores,
            Arc::as_ptr(&compiled.plan) as usize,
            |token| self.resources.get(&token).cloned(),
        )?);
        Ok(())
    }

    fn managed_pass_resources(&self) -> Option<Arc<super::pass_bindings::ManagedPassResources>> {
        self.caches.projection().snapshot.clone()
    }
    fn retain_command_owners(
        &mut self,
        handle: u64,
        owners: &mut super::owner_storage::ResourceOwners,
    ) -> crate::Result<()> {
        self.commands.get_mut(&handle)
            .ok_or_else(|| crate::Error::invalid_usage("retained command is missing"))?
            .retain_resources(owners.drain())
    }

    fn graph_queue(&self) -> (recording::Queue, u32) {
        (self.queue, self.family)
    }
    fn batch_synchronization(&self, index: usize, total: usize) -> GraphBatchSync {
        let mut synchronization = GraphBatchSync::default();
        let explicit = self.synchronization.get(&index);
        let waits = explicit.into_iter().flat_map(|s| s.waits.iter()).chain(
            self.graph_waits
                .iter()
                .filter(|_| index == 0 || self.queue_runtime.multi_queue),
        );
        let signals = explicit
            .into_iter()
            .flat_map(|s| s.signals.iter())
            .chain(self.graph_signals.iter().filter(|_| index + 1 == total));
        synchronization.waits = waits
            .map(|(semaphore, value, stage)| GraphTimelineWait {
                semaphore: semaphore.handle,
                value: *value,
                stage: *stage,
            })
            .collect();
        synchronization.signals = signals
            .map(|(semaphore, value)| crate::sync::timeline::SubmitSignal {
                semaphore: semaphore.handle,
                value: *value,
            })
            .collect();
        synchronization
    }
    fn command_handle(&mut self) -> crate::Result<u64> {
        self.command_for_worker(0)
    }
    fn chunk_command_handle(&mut self, chunk: usize) -> crate::Result<u64> {
        // Chunk 0 reuses the initial command from pool 0. Remaining chunks
        // lease distinct pools; the owner never records while they are lent.
        self.command_for_worker(chunk + 1)
    }
    fn drain_pending_acquires(&mut self, handle: u64) -> crate::Result<u32> {
        let Some(command) = self.commands.get_mut(&handle) else {
            return Err(crate::Error::invalid_usage("batch command is missing"));
        };
        let (resources, tokens) = (&self.resources, self.queue_runtime.frame_tokens());
        // Resources of this frame are acquired by the planner itself.
        let drained = self
            .device
            .record_pending_acquires_except(command, |cell| {
                tokens.iter().flatten().any(|token| {
                    resources
                        .get(token)
                        .is_some_and(|resource| Arc::ptr_eq(&resource.owner_cell(), cell))
                })
            })?;
        if drained.is_empty() {
            return Ok(0);
        }
        let count = drained.len() as u32;
        self.drained_acquires
            .try_reserve(1)
            .map_err(|_| crate::Error::OutOfHostMemory)
            .inspect_err(|_| self.device.requeue_pending_acquires(drained.clone()))?;
        self.drained_acquires.push((handle, drained));
        Ok(count)
    }
    fn abandon_command(&mut self, handle: u64) {
        if let Some(position) = self
            .drained_acquires
            .iter()
            .position(|(command, _)| *command == handle)
        {
            let (_, drained) = self.drained_acquires.swap_remove(position);
            self.device.requeue_pending_acquires(drained);
        }
        if let Some(ring) = &mut self.gpu_timing {
            ring.abandon(handle);
        }
        self.barriers.remove(handle);
        let Some(command) = self.commands.get_mut(&handle) else {
            return;
        };
        // Unsubmitted recording state is discarded by the next whole-pool
        // reset. Keeping its owners until then also preserves reset failure state.
        if !command.has_submissions() && !self.recycled_commands.contains(&handle) {
            self.recycled_commands.push(handle);
        }
    }
    fn chunk_recording(
        &mut self,
        commands: &[u64],
    ) -> crate::Result<super::executor::ChunkRecording<'_>> {
        // Each lease exclusively borrows a distinct frame pool. Chunk zero
        // owns the initial command; its owner performs no recording while the
        // worker scope runs. Other chunks own independent pools.
        let present_patch = self.present_patch();
        let mut leases = smallvec::SmallVec::<[super::executor::ChunkLease<'_>; 8]>::new();
        let mut entries = smallvec::SmallVec::<[(&FramePoolKey, &mut FrameCommandPool); 8]>::new();
        let frame_slot = self.active_frame_slot;
        let family = self.recording_family;
        for (key, entry) in self.frame_pools.iter_mut() {
            if key.frame_slot == frame_slot && key.family == family {
                entries.push((key, entry));
            }
        }
        for &command in commands {
            let key = self
                .command_slot
                .get(&command)
                .ok_or_else(|| crate::Error::invalid_usage("parallel command has no frame pool"))?;
            let position = entries
                .iter()
                .position(|(candidate, _)| *candidate == key)
                .ok_or_else(|| {
                    crate::Error::invalid_usage(
                        "parallel commands must each own a distinct command pool",
                    )
                })?;
            let (_, entry) = entries.swap_remove(position);
            if !self
                .commands
                .get(&command)
                .is_some_and(|recorded| Arc::ptr_eq(&recorded.pool, &entry.pool))
            {
                return Err(crate::Error::invalid_usage(
                    "parallel command was not allocated from its leased pool",
                ));
            }
            // SAFETY: `command` belongs to this exclusively borrowed pool
            // entry, which no other lease or recording command shares.
            leases.push(unsafe { super::executor::ChunkLease::new(entry, command) });
        }
        let view: Box<dyn super::executor::ChunkRecorder + '_> = Box::new(VulkanChunkView {
            present_patch,
            barrier_device: &self.device,
            barrier_dispatch: self.barrier_dispatch,
            cpu_arenas: &self.cpu_arenas,
            owner_pool: &self.owner_pool,
            device: &self.device.core.device,
            resources: &self.resources,
        });
        Ok((view, leases))
    }
    fn begin_command(&mut self, handle: u64) -> crate::Result<()> {
        self.retained_tokens.clear();
        self.retained_command = handle;
        self.commands.get_mut(&handle).expect("owned command").begin()?;
        if let Some(ring) = &mut self.gpu_timing {
            ring.begin(
                self.commands.get_mut(&handle).expect("owned command"),
                self.recording_queue,
                self.recording_epoch,
            )?;
        }
        Ok(())
    }

    fn end_command(&mut self, handle: vk::CommandBuffer) -> crate::Result<()> {
        self.retained_tokens.clear();
        self.flush_barriers(handle.as_raw())?;
        if let Some(ring) = &mut self.gpu_timing {
            ring.end(handle);
        }
        self.commands.get_mut(&handle.as_raw())
            .ok_or_else(|| crate::Error::invalid_usage("ending command is missing"))?
            .end()
    }

    fn flush_barriers(&mut self, handle: u64) -> crate::Result<()> {
        let Some(scope) = self.barriers.remove(handle) else {
            return Ok(());
        };
        if !self.commands.contains_key(&handle) {
            return Err(crate::Error::invalid_usage("barrier command is missing"));
        }
        if !scope.is_empty() {
            unsafe {
                self.barrier_dispatch.issue(
                    &self.device,
                    vk::CommandBuffer::from_raw(handle),
                    &scope,
                )
            }?;
        }
        Ok(())
    }

    fn transition(
        &mut self,
        command: u64,
        token: u64,
        transition: &super::sync_plan::BoundTransition,
    ) -> crate::Result<()> {
        let Some(recording) = self.commands.get_mut(&command) else {
            return Err(crate::Error::invalid_usage("transition command is missing"));
        };
        if self.retained_command != command {
            self.retained_tokens.clear();
            self.retained_command = command;
        }
        let first_in_command = match self.retained_tokens.binary_search(&token) {
            Ok(_) => false,
            Err(at) => {
                self.retained_tokens.insert(at, token);
                true
            }
        };
        // SAFETY: this operand is exactly covered by the graph's bound native
        // transition and by the batch state patch attached before submission.
        let retained = if first_in_command {
            unsafe {
                match self.resources.get(&token) {
                    Some(GpuResource::Buffer(buffer)) => recording.retain_recorded(buffer.clone()),
                    Some(GpuResource::Image(image)) => recording.retain_recorded(image.clone()),
                    None => return Err(crate::Error::invalid_usage("transition resource is missing")),
                }
            }
        } else {
            Ok(())
        };
        if let Err(error) = retained {
            // Not retained: the next transition for this token must retry.
            if let Ok(at) = self.retained_tokens.binary_search(&token) {
                self.retained_tokens.remove(at);
            }
            return Err(error);
        }
        let present = self.present_patch();
        add_transition(
            self.barriers.entry(command),
            &self.resources,
            token,
            transition,
            present,
        )
    }
    fn submit(
        &mut self,
        queue: vk::Queue,
        prepared: &[submit::PreparedSubmit],
    ) -> crate::Result<crate::sync::SyncPoint> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let mut trace = crate::raw::command_buffer::SubmitTrace::new("rg_submit");
        trace.count("batches", prepared.len());
        let mut capture = GpuSubmissionCapture::try_new(
            queue,
            prepared,
            self.device.queues.timelines.queue_count().saturating_sub(1),
            &self.device.memory.stats,
        )?;
        let (present_role, fence) = self.amend_presentation_submit(&mut capture.batches)?;
        if present_role.waits_acquire || present_role.signals_rendered {
            capture.refresh_budget(&self.device.memory.stats)?;
        }
        if self.submissions.len() < SUBMISSION_CAPTURE_LIMIT {
            self.submissions
                .try_reserve(1)
                .map_err(|_| crate::Error::OutOfHostMemory)?;
        }
        let command_count = prepared.iter().try_fold(0usize, |count, batch| {
            count
                .checked_add(batch.commands.len())
                .ok_or_else(|| crate::Error::invalid_usage("submission command count overflows"))
        })?;
        let command_refs = submit::SubmitScratch::<[&VulkanCommandBuffer; 8]>::try_collect(
            command_count,
            &self.device.memory.stats,
            prepared
                .iter()
                .flat_map(|batch| &batch.commands)
                .map(|command| {
                    self.commands.get(&command.as_raw()).ok_or_else(|| {
                        crate::Error::invalid_usage("submitted command is absent from backend")
                    })
                }),
        )?;
        trace.mark("capture_and_command_refs");
        crate::raw::command_buffer::validate_submission_commands(&command_refs)?;
        trace.mark("validate1");
        let semaphore_refs = self
            .graph_waits
            .iter()
            .map(|(s, _, _)| s)
            .chain(self.graph_signals.iter().map(|(s, _)| s))
            .chain(self.synchronization.values().flat_map(|sync| {
                sync.waits
                    .iter()
                    .map(|(s, _, _)| s)
                    .chain(sync.signals.iter().map(|(s, _)| s))
            }))
            .filter(|s| {
                prepared.iter().any(|batch| {
                    batch.wait_semaphores.contains(&s.handle)
                        || batch.signal_semaphores.contains(&s.handle)
                })
            });
        let semaphores = submit::SubmitScratch::<[Arc<TimelineSemaphore>; 8]>::try_collect(
            semaphore_refs.clone().count(),
            &self.device.memory.stats,
            semaphore_refs.map(|semaphore| Ok(semaphore.clone())),
        )?;
        trace.mark("semaphore_refs");
        let use_sources =
            submit::SubmitScratch::<[crate::resource::tracked::OrderedSubmissionUses<'_>; 8]>::try_collect(
                command_refs.len(),
                &self.device.memory.stats,
                command_refs.iter().map(|command| command.submission_uses()),
            )?;
        let uses = crate::resource::tracked::collect_submission_uses(
            &use_sources,
            &self.device.memory.stats,
        )?;
        drop(use_sources);
        trace.mark("uses_merge");
        trace.count("commands", command_refs.len());
        trace.count("unique_uses", uses.len());
        let _reservations =
            crate::resource::tracked::lock_submission_uses(uses.ordered(), &self.device.memory.stats)?;
        trace.mark("state_locks");
        let family = self.recording_family;
        let prepared_states = unsafe {
            crate::raw::command_buffer::prepare_submission_states(
                &command_refs,
                &uses,
                self.device.queue_id(queue)?,
                family,
                &self.device.memory.stats,
            )?
        };
        trace.mark("prepare_states");
        drop(command_refs);
        // Declared (planner-bound) accesses carry their first-use stages;
        // opaque uses inside the commands keep ALL_COMMANDS.
        let mut dependencies = prepared_states.staged_dependencies();
        dependencies.merge(&self.queue_submission_dependencies()?);
        self.device
            .add_staged_timeline_waits(queue, &mut capture.batches, &dependencies)?;
        trace.mark("dependencies");
        let result = self.device.submit_prepared(queue, &capture.batches, fence);
        trace.mark("device_submit_prepared");
        if let Ok(point) = result {
            // Infallible after the irreversible queue call: the owner only
            // records the receipt it must now protect.
            if let Err(error) = self.presentation_submitted(present_role, point) {
                log::error!(target: "rhapsody::rg::presentation", "presentation bookkeeping failed after submit: {error}");
            }
            prepared_states.publish(point);
            self.queue_submitted(point);
            // Coalesced acquires in these commands are now committed.
            self.drained_acquires.retain(|(command, _)| {
                !prepared
                    .iter()
                    .any(|batch| batch.commands.iter().any(|c| c.as_raw() == *command))
            });
            for semaphore in semaphores.iter() {
                semaphore.last_use.insert_overall(point);
            }
            for batch in prepared {
                for command in &batch.commands {
                    // All entries were validated before the irreversible queue call.
                    if let Some(ring) = &mut self.gpu_timing {
                        ring.submitted(command.as_raw(), point);
                    }
                    self.commands
                        .get_mut(&command.as_raw())
                        .expect("validated command")
                        .record_submission(point);
                }
            }
        }
        capture.result = match &result {
            Ok(_) => 0,
            Err(crate::Error::DeviceLost) => vk::Result::ERROR_DEVICE_LOST.as_raw(),
            Err(_) => vk::Result::ERROR_UNKNOWN.as_raw(),
        };
        if self.submissions.len() >= SUBMISSION_CAPTURE_LIMIT {
            self.submissions.pop_front();
        }
        self.submissions.push_back(capture);
        result
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
#[derive(Clone, Debug)]
pub struct SessionFrameReceipt {
    pub frame: u64,
    storage: Arc<super::receipt_storage::ReceiptSlot>,
    /// 本帧编译/录制/提交耗时与并行分块统计。
    pub stats: super::executor::ExecutionStats,
}
impl std::ops::Deref for SessionFrameReceipt {
    type Target = super::receipt_storage::ReceiptData;
    fn deref(&self) -> &Self::Target {
        &self.storage.data
    }
}
/// Real resource allocation and callback ownership before command recording.
/// Execute or cancel this value through the session that compiled it.
#[must_use = "execute or cancel this compiled frame through its originating session"]
pub struct PreparedSessionFrame {
    graph_id: super::resource::GraphId,
    generation: crate::device::DeviceGeneration,
    prepared: super::executor::PreparedGraph,
    callbacks: Vec<graph::RenderCallback>,
    retire: Vec<graph::RetireCallback>,
    receipt_slot: super::receipt_storage::ReceiptLease,
}
impl PreparedSessionFrame {
    pub fn snapshot(&self) -> &graph::Snapshot {
        &self.prepared.compiled.snapshot
    }
}

/// Graphs recorded per display frame (main + presentation); sizes the command
/// pool slots so that `frames_in_flight` whole display frames fit.
const COMMAND_SLOTS_PER_DISPLAY_FRAME: u32 = 2;

/// One displayed frame. Several graphs (main plus presentation) executed
/// inside one display frame share a single in-flight throttle slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayFrameId(pub u64);

struct PendingSessionFrame {
    failed: bool,
    frame: u64,
    /// Display frame this graph belongs to; the throttle counts these groups.
    display: u64,
    execution: GraphExecution,
    retire: Vec<graph::RetireCallback>,
    started_at: std::time::Instant,
}

/// Quiescent transfer of the existing named-resource cache. Contains actual
/// GPU owners, parity, confirmed access/validity and queue ownership; no frame
/// callbacks, Rc graph/session, command pools or alternate resource backend.
/// Constructed only after this session confirms real GPU completion.
pub struct CompletedNamedResources {
    named: NamedResources,
    families: BTreeMap<u64, u32>,
}

pub struct VulkanGraphSession {
    pub backend: VulkanGraphBackend,
    pub chain: ExecutionChain,
    pub(crate) result_cache: super::result_cache::ResultCache,
    pool: graph::Pool,
    /// 在途提交队列（帧节流）。
    pending: super::frame_throttle::FrameThrottle<PendingSessionFrame>,
    next_frame: u64,
    /// Next display-frame id (ids start at 1).
    next_display: u64,
    /// Explicit display frame opened by `begin_display_frame`, if any.
    active_display: Option<u64>,
    /// Display frame of the graph currently being declared.
    current_display: u64,
    frame_imports: Option<super::frame_imports::FrameImports>,
    frame_import_config: super::frame_imports::FrameImportConfig,
    device_lost: bool,
    last_execution_partial: bool,
    active_prepared: Option<super::resource::GraphId>,
    /// 上一帧从 begin_frame 到该帧完成的墙钟时长；仅统计用途。
    last_frame_duration_ns: u64,
    /// 本帧提交账本（`frame::FrameLedger`）。
    ledger: super::frame::FrameLedger,
    named: Option<NamedResources>,
    /// The acquired swapchain image imported into a presentation graph.
    presentation: Option<presentation::SessionPresentation>,
    /// What happens to an acquired image whose presentation graph fails.
    present_policy: PresentPolicy,
}

impl VulkanGraphSession {
    pub fn new(backend: VulkanGraphBackend) -> Self {
        let chain = backend.new_execution_chain();
        let named = Some(NamedResources::new(backend.device.clone()));
        Self {
            backend,
            chain,
            result_cache: Default::default(),
            pool: graph::Pool::default(),
            pending: Default::default(),
            next_frame: 1,
            next_display: 1,
            active_display: None,
            current_display: 0,
            frame_imports: None,
            frame_import_config: Default::default(),
            device_lost: false,
            last_execution_partial: false,
            active_prepared: None,
            last_frame_duration_ns: 0,
            ledger: Default::default(),
            named,
            presentation: None,
            present_policy: PresentPolicy::default(),
        }
    }
    /// Additive current-architecture handoff used by background App warmup.
    /// Ordinary sessions retain their existing API and resource/cache owners.
    pub fn take_completed_named_resources(
        &mut self,
        timeout_ns: u64,
    ) -> crate::Result<CompletedNamedResources> {
        if self.active_prepared.is_some()
            || self.last_execution_partial
            || !self.ledger.has_ended()
            || self.ledger.submissions() == 0
        {
            return Err(crate::Error::invalid_usage(
                "named handoff requires a completely submitted frame",
            ));
        }
        self.wait_submits_done(timeout_ns)?;
        if !self.pending.is_empty() || self.frame_import_count() != 0 || self.is_poisoned() {
            return Err(crate::Error::invalid_usage(
                "named handoff has pending or lost GPU work",
            ));
        }
        let named = self.named.as_ref().ok_or_else(|| {
            crate::Error::invalid_usage("named handoff while a graph is executing")
        })?;
        if named.state.requested_validity().next().is_some() {
            return Err(crate::Error::invalid_usage(
                "named handoff has an uncommitted frame declaration",
            ));
        }
        let families = named
            .resources
            .resources
            .keys()
            .map(|token| {
                (
                    *token,
                    self.backend
                        .import_families
                        .get(token)
                        .copied()
                        .unwrap_or(self.backend.device.queues.primary_family),
                )
            })
            .collect();
        let mut named = self
            .named
            .replace(NamedResources::new(self.backend.device.clone()))
            .expect("validated named owner");
        named.graph_id = None;
        Ok(CompletedNamedResources { named, families })
    }

    /// Restore the same actual named owners into the App's original idle
    /// session. It never merges two histories or overwrites live GPU work.
    pub fn validate_completed_named_resources_adoption(
        &self,
        completed: &CompletedNamedResources,
    ) -> crate::Result<()> {
        if self.active_prepared.is_some()
            || !self.pending.is_empty()
            || self.frame_import_count() != 0
            || self.is_poisoned()
        {
            return Err(crate::Error::invalid_usage(
                "named adoption requires an idle healthy session",
            ));
        }
        let current = self.named.as_ref().ok_or_else(|| {
            crate::Error::invalid_usage("named adoption while a graph is executing")
        })?;
        if (current.graph_id.is_some() && !self.ledger.has_ended()) || current.resources.len() != 0
        {
            return Err(crate::Error::invalid_usage(
                "named adoption cannot replace an existing graph/history owner",
            ));
        }
        if !Arc::ptr_eq(&completed.named.resources.device, &self.backend.device) {
            return Err(crate::Error::invalid_usage(
                "named handoff belongs to another device owner",
            ));
        }
        for resource in completed.named.resources.resources.values() {
            resource.check_device(&self.backend.device)?;
        }
        Ok(())
    }

    /// Consume a completed owner's cache only after all adoption checks succeed.
    /// Call the borrowed validation first when retaining the transfer on failure
    /// is required by an application-owned one-shot channel.
    pub fn adopt_completed_named_resources(
        &mut self,
        completed: CompletedNamedResources,
    ) -> crate::Result<()> {
        self.validate_completed_named_resources_adoption(&completed)?;
        for (token, family) in completed.families {
            self.backend.import_families.insert(token, family);
        }
        self.named = Some(completed.named);
        Ok(())
    }

    pub fn configure_receipt_storage(
        &mut self,
        config: super::receipt_storage::ReceiptStorageConfig,
    ) -> crate::Result<()> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "a compiled frame owns an admitted receipt slot",
            ));
        }
        self.backend.receipt_storage = Some(super::receipt_storage::ReceiptStorage::new(
            config,
            self.backend.device.memory.stats.clone(),
        )?);
        Ok(())
    }
    pub fn configure_frame_imports(
        &mut self,
        config: super::frame_imports::FrameImportConfig,
    ) -> crate::Result<()> {
        self.collect_frame_imports();
        if self.frame_import_count() != 0 {
            return Err(crate::Error::invalid_usage(
                "frame imports are still owned by live graphs",
            ));
        }
        self.frame_imports = Some(super::frame_imports::FrameImports::new(
            config,
            self.backend.device.memory.stats.clone(),
        )?);
        self.frame_import_config = config;
        Ok(())
    }
    /// Import an actual buffer for this graph without exposing a host token.
    /// Non-exported declaration bindings are released on cancellation or after recording;
    /// submitted commands independently own the physical resource until ready.
    pub fn import_frame_buffer(
        &mut self,
        graph: &mut graph::RenderGraph,
        buffer: Arc<buffer::Buffer>,
        initial_access: u8,
    ) -> crate::Result<super::resource::ResourceHandle> {
        self.import_frame_resource(graph, GpuResource::Buffer(buffer), initial_access, false)
    }
    /// Frame-scoped texture import with the same cancellation and retirement
    /// ownership as `import_frame_buffer`. Existing physical declarations are reused.
    pub fn import_frame_texture(
        &mut self,
        graph: &mut graph::RenderGraph,
        texture: Arc<texture::Texture>,
        initial_access: u8,
    ) -> crate::Result<super::resource::ResourceHandle> {
        self.import_frame_resource(graph, GpuResource::Image(texture), initial_access, false)
    }
    /// B compile-present transfers its main output into the second graph with
    /// access 0x22, while the acquired swapchain image remains borrowed.
    pub fn import_frame_texture_owned(
        &mut self,
        graph: &mut graph::RenderGraph,
        texture: Arc<texture::Texture>,
        initial_access: u8,
    ) -> crate::Result<super::resource::ResourceHandle> {
        self.import_frame_resource(graph, GpuResource::Image(texture), initial_access, true)
    }
    /// `Some(result)` when declaration `index` already is this physical resource.
    fn existing_import(
        &self,
        graph: &graph::RenderGraph,
        index: usize,
        resource: &GpuResource,
        owned: bool,
        sharing: crate::resource::ResourceSharing,
        initial_access: u8,
    ) -> Option<crate::Result<super::resource::ResourceHandle>> {
        let declaration = graph.declaration.resources.get(index)?;
        let (token, previous_access) = match declaration.source {
            graph::OriginSource::ImportOwned { token, initial_access }
            | graph::OriginSource::ImportUnowned { token, initial_access } => (token, initial_access),
            graph::OriginSource::Create => return None,
        };
        let matches = match (self.backend.resources.get(&token), resource) {
            (Some(GpuResource::Buffer(existing)), GpuResource::Buffer(buffer)) => {
                Arc::ptr_eq(existing, buffer)
            }
            (Some(GpuResource::Image(existing)), GpuResource::Image(image)) => {
                Arc::ptr_eq(existing, image)
            }
            _ => false,
        };
        if !matches {
            return None;
        }
        Some((|| {
            if previous_access != initial_access {
                return Err(crate::Error::invalid_usage(format!(
                    "frame import initial access mismatch: resource={} index={index} existing={previous_access:#04x} requested={initial_access:#04x}", declaration.name
                )));
            }
            if owned != matches!(declaration.source, graph::OriginSource::ImportOwned { .. }) {
                return Err(crate::Error::invalid_usage(
                    "frame import ownership differs from its existing declaration",
                ));
            }
            validate_import_descriptor(declaration.descriptor, resource.descriptor())?;
            if declaration.sharing != sharing {
                return Err(crate::Error::invalid_usage("frame import sharing mismatch"));
            }
            graph.resource_handle(index as u32).map_err(Into::into)
        })())
    }
    fn import_frame_resource(
        &mut self,
        graph: &mut graph::RenderGraph,
        resource: GpuResource,
        initial_access: u8,
        owned: bool,
    ) -> crate::Result<super::resource::ResourceHandle> {
        use crate::resource::identity::PhysicalIdentity;
        let (identity, sharing, name) = match &resource {
            GpuResource::Buffer(buffer) => (
                PhysicalIdentity::Buffer(Arc::as_ptr(buffer) as usize),
                buffer.sharing(),
                "frame-buffer",
            ),
            GpuResource::Image(image) => (
                PhysicalIdentity::Image(Arc::as_ptr(image) as usize),
                image.sharing(),
                "frame-image",
            ),
        };
        let descriptor = resource.descriptor();
        resource.check_device(&self.backend.device)?;
        if usize::from(initial_access) >= crate::raw::access_table::ACCESS_CODE_COUNT {
            return Err(crate::Error::invalid_usage("invalid frame import access"));
        }
        if self.is_poisoned() {
            return Err(crate::Error::DeviceLost);
        }
        // Fast path: this graph already imported this physical resource through
        // a frame import (one hash lookup instead of scanning every declaration).
        if let Some(existing) = self
            .frame_imports
            .as_ref()
            .and_then(|imports| imports.find(graph.graph_id(), identity))
            && let Some(found) =
                self.existing_import(graph, existing.raw() as usize, &resource, owned, sharing, initial_access)
        {
            return found;
        }
        // Other typed hosts may already have imported the same physical resource.
        // A second logical declaration would split its dependency history.
        for &index in &graph.foreign_imports {
            if let Some(found) =
                self.existing_import(graph, index as usize, &resource, owned, sharing, initial_access)
            {
                return found;
            }
        }
        if self.frame_imports.is_none() {
            self.frame_imports = Some(super::frame_imports::FrameImports::new(
                self.frame_import_config,
                self.backend.device.memory.stats.clone(),
            )?);
        }
        if self
            .frame_imports
            .as_ref()
            .expect("frame imports initialized")
            .find(graph.graph_id(), identity)
            .is_some()
        {
            return Err(crate::Error::invalid_usage(
                "frame import was invalidated before graph execution",
            ));
        }
        // Collection walks every live import, so run it only when the registry
        // is full (a per-import collect made a frame's imports quadratic).
        let reserved = self
            .frame_imports
            .as_mut()
            .expect("frame imports initialized")
            .reserve();
        let lease = match reserved {
            Ok(lease) => lease,
            Err(crate::Error::Exhausted(_)) => {
                // Only unrecorded graphs consume declaration capacity. Reclaim
                // abandoned graphs without polling or waiting for GPU work.
                self.collect_frame_imports();
                self.frame_imports
                    .as_mut()
                    .expect("frame imports initialized")
                    .reserve()?
            }
            Err(error) => return Err(error),
        };
        let imports = self
            .frame_imports
            .as_mut()
            .expect("frame imports initialized");
        graph.reserve_retirement_callback()?;
        let marker = lease.marker();
        let token = self.backend.bind_import(resource)?;
        let imported = if owned {
            graph.import_owned_with_sharing(
                format!("{name}-{}", graph.declaration.resources.len()),
                descriptor,
                token,
                initial_access,
                sharing,
            )
        } else {
            graph.import_unowned_with_sharing(
                format!("{name}-{}", graph.declaration.resources.len()),
                descriptor,
                token,
                initial_access,
                sharing,
            )
        };
        let handle = match imported {
            Ok(handle) => {
                // Found through `frame_imports` from now on, not by the foreign scan.
                graph.foreign_imports.pop();
                handle
            }
            Err(error) => {
                self.backend.resources.remove(&token);
                self.backend.import_families.remove(&token);
                return Err(error.into());
            }
        };
        if let Err(error) = imports.insert(identity, token, handle, lease) {
            self.backend.resources.remove(&token);
            self.backend.import_families.remove(&token);
            return Err(error);
        }
        graph.on_retire(move || drop(marker));
        Ok(handle)
    }
    pub fn frame_import_count(&self) -> usize {
        self.frame_imports
            .as_ref()
            .map_or(0, |imports| imports.len())
    }
    fn collect_frame_imports(&mut self) {
        self.collect_recorded_imports(None);
    }
    fn collect_recorded_imports(&mut self, recorded: Option<(super::resource::GraphId, &[u16])>) {
        let Some(imports) = &mut self.frame_imports else {
            return;
        };
        let backend = &mut self.backend;
        let mut removed_any = false;
        imports.collect(recorded, |token| {
            removed_any = true;
            backend.resources.remove(&token);
            backend.import_families.remove(&token);
            backend.history_validity.remove(&token);
        });
        if removed_any {
            // Declaration tokens expired; the bounded idle CPU projections
            // retain only physical owners and cannot revive old graph handles.
            let mut projection = backend.caches.projection();
            projection.snapshot.take();
            projection.pool.park_idle();
        }
    }
    pub fn receipt_storage_statistics(&mut self) -> super::receipt_storage::ReceiptStorageStats {
        self.backend.receipt_storage
            .as_mut()
            .map_or(Default::default(), |pool| pool.statistics())
    }
    pub fn last_synchronization(&self) -> &super::sync_plan::BoundSynchronization {
        self.backend.bound_synchronization()
    }
    pub fn is_poisoned(&self) -> bool {
        self.device_lost || self.backend.device.queues.timelines.is_device_lost()
    }
    /// 最近一次完成帧的墙钟时长。
    pub fn last_known_frame_duration_ns(&self) -> u64 {
        self.last_frame_duration_ns
    }
    /// 本帧提交账本（`begin_frame` 清零，`execute_frame` 记提交，`end_frame` 校验）。
    pub fn frame_ledger(&self) -> &super::frame::FrameLedger {
        &self.ledger
    }
    /// 等待在途提交全部完成。
    /// wait_frame 对每个在途帧执行有界等待，超时以 Err 返回。
    pub fn wait_submits_done(&mut self, timeout_ns: u64) -> crate::Result<()> {
        while let Some(pending) = self.pending.front() {
            let frame = pending.frame;
            self.wait_frame(frame, timeout_ns)?;
        }
        if self.result_cache.has_work() {
            let completed = self.backend.device.poll()?;
            self.collect_cached_results(&completed)?;
        }
        self.collect_frame_imports();
        self.backend.device.try_collect_all_completed()?;
        Ok(())
    }
    /// 配置限制在飞显示帧数量；只等待最旧帧，不等待其他会话或无关队列。
    fn throttle_in_flight(&mut self) -> Result<(), crate::Error> {
        let limit = self.backend.device.frames_in_flight() as usize;
        while self.pending.in_flight_groups(|pending| pending.display) >= limit {
            let frame = self.pending.front().unwrap().frame;
            self.wait_frame(frame, 10_000_000_000)?;
        }
        Ok(())
    }
    fn allocate_display(&mut self) -> u64 {
        let display = self.next_display;
        self.next_display += 1;
        display
    }
    /// Opens a display frame and throttles once. Graphs declared with
    /// [`Self::begin_frame`] before [`Self::end_display_frame`] (for example the
    /// main graph and the presentation graph) share this one in-flight slot, so
    /// CPU work for frame N+1 can overlap GPU work for frame N.
    ///
    /// A display frame left open by an aborted frame is replaced, not an error.
    pub fn begin_display_frame(&mut self) -> Result<DisplayFrameId, crate::Error> {
        self.active_display = None;
        self.backend.display_frames = true;
        self.throttle_in_flight()?;
        if self.is_poisoned() {
            return Err(crate::Error::DeviceLost);
        }
        let display = self.allocate_display();
        self.active_display = Some(display);
        Ok(DisplayFrameId(display))
    }
    /// Closes the explicit display frame. Idempotent.
    pub fn end_display_frame(&mut self) {
        self.active_display = None;
    }
    pub fn has_active_display_frame(&self) -> bool {
        self.active_display.is_some()
    }
    pub fn begin_frame(&mut self) -> Result<graph::RenderGraph, crate::Error> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "execute or cancel the compiled graph before beginning another frame",
            ));
        }
        self.collect_frame_imports();
        self.reap_presentation();
        self.result_cache.abandon_pending();
        if self.result_cache.has_work() {
            let completed = self.backend.device.poll()?;
            if self.backend.device.memory.stats.budget_action()
                == crate::device::BudgetAction::EvictCache
            {
                self.result_cache.evict_completed(&completed);
            }
            self.collect_cached_results(&completed)?;
        }
        self.forget_evicted_pool_entries();
        self.evict_idle_resources_under_pressure()?;
        // 显示帧内的图共用一个槽：已有显式显示帧时节流已在其开始处完成；
        // 否则本图自成一个显示帧（与旧行为一致：每图占一个槽）。
        let display = match self.active_display {
            Some(display) => {
                // The display-frame throttle counts groups, so a display frame
                // with more graphs than expected (a retry, a partial failure kept
                // in `pending`) could otherwise exhaust the command slots. Bound
                // the graphs themselves as well.
                let slots = self.backend.command_slot_count();
                while self.pending.in_flight() >= slots {
                    let frame = self.pending.front().unwrap().frame;
                    self.wait_frame(frame, 10_000_000_000)?;
                }
                display
            }
            None => {
                self.throttle_in_flight()?;
                self.allocate_display()
            }
        };
        if self.is_poisoned() {
            return Err(crate::Error::DeviceLost);
        }
        self.current_display = display;
        if let Some(named) = &mut self.named {
            named.state.begin_frame();
        }
        self.collect_named_retired();
        self.backend.history_validity.clear();
        self.ledger.begin();
        self.backend.specifications.clear();
        self.backend.synchronization.clear();
        self.backend.graph_waits.clear();
        self.backend.graph_signals.clear();
        self.backend.submissions.clear();
        let graph = graph::RenderGraph::new();
        if let Some(named) = &mut self.named {
            named.graph_id = Some(graph.graph_id());
        }
        Ok(graph)
    }
    /// Cancel a graph that has never been submitted. Declaration leases and
    /// callbacks must be retired even when a game-side builder returns an error.
    /// Already submitted frames are owned by the pending queue and are untouched.
    pub fn cancel_frame(&mut self, built: graph::BuiltGraph) -> crate::Result<()> {
        let belongs_to_frame = self
            .named
            .as_ref()
            .is_some_and(|named| named.graph_id == Some(built.graph_id()));
        self.cancel_presentation_of(built.graph_id());
        let (_, callbacks, retire, _cancelled_rejection) = built.into_parts();
        drop(callbacks);
        let retired = run_retire_callbacks(retire);
        self.collect_frame_imports();
        if !belongs_to_frame {
            return Err(crate::Error::invalid_usage(
                "cancelled graph does not belong to the active frame",
            ));
        }
        if let Some(named) = &mut self.named {
            named.state.abandon_frame();
            named.graph_id = None;
        }
        self.result_cache.abandon_pending();
        self.backend.specifications.clear();
        self.backend.history_validity.clear();
        self.backend.synchronization.clear();
        self.backend.graph_waits.clear();
        self.backend.graph_signals.clear();
        retired
    }

    fn collect_named_retired(&mut self) {
        let Some(named) = &mut self.named else {
            return;
        };
        if named.resources.retired.is_empty() {
            return;
        }
        let mut removed_any = false;
        named.resources.retired.retain(|token| {
            if self.pending.iter().any(|frame| {
                frame
                    .execution
                    .compiled
                    .snapshot
                    .physical_mapping
                    .iter()
                    .any(|mapping| mapping.token == Some(*token))
            }) {
                return true;
            }
            removed_any = true;
            self.backend.resources.remove(token);
            self.backend.import_families.remove(token);
            self.backend.history_validity.remove(token);
            false
        });
        // Invalidating the pass-resource projection once is enough; doing it
        // per token also forced a full rebuild for each retired resource.
        if removed_any {
            self.backend.clear_pass_resources();
        }
    }
    /// Drop pool entries whose parked resource the device's eviction already
    /// released (see `rg::graph_caches`).
    fn forget_evicted_pool_entries(&mut self) {
        let evicted = self.backend.caches.take_evicted();
        if !evicted.is_empty() {
            // Tokens come from one process-wide counter shared by textures
            // and buffers.
            self.pool.forget_handles(&evicted, &evicted);
        }
    }
    /// Idle pooled resources and retired alias sets that memory pressure may
    /// release.
    pub fn cache_stats(&self) -> super::graph_caches::GraphCacheStats {
        self.backend.caches.stats()
    }
    fn evict_idle_resources_under_pressure(&mut self) -> crate::Result<()> {
        let _ = self.backend.device.poll()?;
        if self.backend.device.memory.stats.budget_action()
            == crate::device::BudgetAction::EvictCache
        {
            if let Some(storage) = &mut self.backend.receipt_storage {
                storage.trim_idle(0);
            }
            let mut projection = self.backend.caches.projection();
            projection.snapshot.take();
            projection.pool.trim_idle();
            drop(projection);
            let (textures, buffers) = self.pool.drain();
            for token in textures
                .into_iter()
                .map(|r| r.handle)
                .chain(buffers.into_iter().map(|r| r.handle))
            {
                self.backend.discard_pooled(token);
            }
            self.backend.device.try_collect_garbage()?;
        }
        Ok(())
    }
    /// Compile/allocate without issuing Vulkan commands.
    pub fn compile_frame(
        &mut self,
        mut built: graph::BuiltGraph,
    ) -> crate::Result<PreparedSessionFrame> {
        if self.active_prepared.is_some() {
            let (_, _, retire, _rejected_input) = built.into_parts();
            let _ = run_retire_callbacks(retire);
            return Err(crate::Error::invalid_usage(
                "a compiled frame already owns this session",
            ));
        }
        let Some(mut named) = self.named.take() else {
            let (_, _, retire, _rejected_input) = built.into_parts();
            let _ = run_retire_callbacks(retire);
            return Err(crate::Error::invalid_usage(
                "named frame is already compiling",
            ));
        };
        let graph_id = built.graph_id();
        let result = (|| {
            // Known producer failures precede named imports, graph validation,
            // result-cache preparation and all native/CPU frame allocation.
            if let Some(error) = built.take_rejection() {
                let error = if self.is_poisoned() { crate::Error::DeviceLost } else { error };
                let (_, callbacks, retire, rejection) = built.into_parts();
                debug_assert!(rejection.is_none());
                if error.is_device_lost() {
                    self.backend.device.queues.timelines.mark_device_lost();
                }
                drop(callbacks);
                let _ = run_retire_callbacks(retire);
                return Err(error);
            }
            if named.state.requested().is_empty() {
                return self.compile_built(built, |_| Ok(()));
            }
            if named.graph_id != Some(graph_id) {
                let (_, _, retire, _rejected_input) = built.into_parts();
                let _ = run_retire_callbacks(retire);
                return Err(crate::Error::invalid_usage(
                    "named resources belong to another frame graph",
                ));
            }
            let mut graph = built.into_graph();
            let preparation = (|| -> crate::Result<()> {
                for (name, resource) in named.state.requested() {
                    named
                        .resources
                        .import_into(&mut self.backend, &name, resource.token)?;
                }
                named.state.export_temporal(&mut graph)?;
                Ok(())
            })();
            if let Err(error) = preparation {
                let (_, _, retire, _rejected_input) = graph.build().into_parts();
                let _ = run_retire_callbacks(retire);
                return Err(error);
            }
            self.compile_built(graph.build(), |compiled| {
                named
                    .state
                    .validate_compiled_exports(compiled)
                    .map_err(crate::Error::from)
            })
        })();
        if result.is_ok() {
            self.active_prepared = Some(graph_id);
        } else {
            named.state.abandon_frame();
            if named.graph_id == Some(graph_id) {
                named.graph_id = None;
            }
            self.result_cache.abort(graph_id);
            self.cancel_presentation_of(graph_id);
        }
        self.named = Some(named);
        self.collect_frame_imports();
        result
    }

    /// Compile and execute through the session frame protocol.
    pub fn execute_frame(
        &mut self,
        built: graph::BuiltGraph,
    ) -> crate::Result<SessionFrameReceipt> {
        let prepared = self.compile_frame(built)?;
        self.execute_prepared(prepared)
    }

    pub fn cancel_prepared(&mut self, mut frame: PreparedSessionFrame) -> crate::Result<()> {
        if self.active_prepared != Some(frame.graph_id)
            || frame.generation != self.backend.device.generation()
        {
            return Err(crate::Error::invalid_usage(
                "compiled frame belongs to another session",
            ));
        }
        self.active_prepared = None;
        self.result_cache.abort(frame.graph_id);
        self.cancel_presentation_of(frame.graph_id);
        graph::reclaim_failed_resources(
            &mut frame.prepared.compiled.tables.registry,
            &frame.prepared.compiled.declarations.resources,
            &frame.prepared.compiled.tables.snapshot.pool_hits,
            &mut self.pool,
            &mut self.backend,
        );
        drop(frame.callbacks);
        let result = run_retire_callbacks(frame.retire);
        if let Some(named) = &mut self.named {
            named.state.abandon_frame();
            named.graph_id = None;
        }
        self.collect_frame_imports();
        result
    }

    /// A persistent named slot; contents are valid only after complete publication.
    pub fn named_texture(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: texture::TextureDesc,
    ) -> crate::Result<super::temporal::NamedTexture> {
        self.named_texture_with_sharing(
            graph,
            name,
            desc,
            crate::resource::ResourceSharing::Exclusive,
        )
    }
    pub fn named_texture_with_sharing(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: texture::TextureDesc,
        sharing: crate::resource::ResourceSharing,
    ) -> crate::Result<super::temporal::NamedTexture> {
        let lookup = self.request_named(
            graph,
            &format!("slot/{}:{name}", name.len()),
            ResourceSpec::Image(desc).with_sharing(sharing),
            false,
        )?;
        Ok(super::temporal::NamedTexture {
            texture: graph.texture_id(lookup.handle)?,
            history_valid: lookup.reused,
        })
    }
    pub fn named_buffer(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: buffer::BufferDesc,
    ) -> crate::Result<super::temporal::NamedBuffer> {
        self.named_buffer_with_sharing(
            graph,
            name,
            desc,
            crate::resource::ResourceSharing::Exclusive,
        )
    }
    /// Like [`Self::named_buffer`], but a newly created (or resized) buffer is
    /// zero-filled, so reading it before the first write is defined.
    /// `history_valid` still reports whether earlier frames wrote it.
    pub fn named_buffer_zeroed(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: buffer::BufferDesc,
    ) -> crate::Result<super::temporal::NamedBuffer> {
        self.named_buffer_with_options(
            graph,
            name,
            desc,
            crate::resource::ResourceSharing::Exclusive,
            true,
        )
    }
    pub fn named_buffer_with_sharing(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: buffer::BufferDesc,
        sharing: crate::resource::ResourceSharing,
    ) -> crate::Result<super::temporal::NamedBuffer> {
        self.named_buffer_with_options(graph, name, desc, sharing, false)
    }
    /// A session-owned buffer that persists across frames. New contents are
    /// undefined (a read before the first write is rejected) unless
    /// `zero_initialized` requests a zero-filled allocation.
    pub fn named_buffer_with_options(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: buffer::BufferDesc,
        sharing: crate::resource::ResourceSharing,
        zero_initialized: bool,
    ) -> crate::Result<super::temporal::NamedBuffer> {
        let lookup = self.request_named(
            graph,
            &format!("slot/{}:{name}", name.len()),
            ResourceSpec::Buffer(desc).with_sharing(sharing),
            zero_initialized,
        )?;
        Ok(super::temporal::NamedBuffer {
            buffer: graph.buffer_id(lookup.handle)?,
            history_valid: lookup.reused,
        })
    }
    /// Request the session-owned current/previous texture pair. No physical token escapes.
    pub fn temporal_texture(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: texture::TextureDesc,
    ) -> crate::Result<super::temporal::TemporalTexturePair> {
        self.temporal_texture_with_sharing(
            graph,
            name,
            desc,
            crate::resource::ResourceSharing::Exclusive,
        )
    }
    pub fn temporal_texture_with_sharing(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: texture::TextureDesc,
        sharing: crate::resource::ResourceSharing,
    ) -> crate::Result<super::temporal::TemporalTexturePair> {
        let named = self
            .named
            .as_mut()
            .ok_or_else(|| crate::Error::invalid_usage("named frame is already executing"))?;
        named.validate_graph(graph)?;
        let key = format!("pair/{}:{name}", name.len());
        named.ensure_capacity(&format!("{key}:0"), 2)?;
        let names = [format!("{key}:0"), format!("{key}:1")];
        let previous = names.each_ref().map(|name| {
            named
                .resources
                .register(name, ResourceSpec::Image(desc).with_sharing(sharing))
        });
        let pair = match named.state.get_or_create_owned_pair(
            graph,
            &key,
            ResourceSpec::Image(desc).descriptor(),
            &mut named.resources,
        ) {
            Ok(pair) => pair,
            Err(error) => {
                for (name, previous) in names.iter().zip(previous) {
                    named.resources.restore_specification(name, previous);
                }
                return Err(error.into());
            }
        };
        for (token, valid) in named.state.requested_validity() {
            self.backend.history_validity.insert(token, valid);
        }
        Ok(super::temporal::TemporalTexturePair::from_lookups(
            graph, pair,
        )?)
    }
    fn request_named(
        &mut self,
        graph: &mut graph::RenderGraph,
        key: &str,
        spec: ResourceSpec,
        zero_initialized: bool,
    ) -> crate::Result<super::temporal::TemporalLookup> {
        let named = self
            .named
            .as_mut()
            .ok_or_else(|| crate::Error::invalid_usage("named frame is already executing"))?;
        named.validate_graph(graph)?;
        named.ensure_capacity(key, 1)?;
        let previous = named.resources.register(key, spec);
        named.resources.set_zero_initialized(key, zero_initialized);
        let lookup = match named.state.get_or_create_owned(
            graph,
            key,
            spec.descriptor(),
            &mut named.resources,
        ) {
            Ok(lookup) => lookup,
            Err(error) => {
                named.resources.restore_specification(key, previous);
                return Err(error.into());
            }
        };
        self.backend
            .history_validity
            .insert(lookup.resource.token, lookup.reused);
        Ok(lookup)
    }
    /// Invalidate history without waiting for unrelated GPU work or destroying allocations.
    pub fn reset_temporal(&mut self, name: &str) -> crate::Result<()> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "a compiled frame owns temporal references",
            ));
        }
        let named = self
            .named
            .as_mut()
            .ok_or_else(|| crate::Error::invalid_usage("named frame is already executing"))?;
        named.state.reset(&format!("slot/{}:{name}", name.len()))?;
        named.state.reset(&format!("pair/{}:{name}", name.len()))?;
        Ok(())
    }

    /// Original OOM recovery clears only reusable idle allocations. Pending
    /// frames retain their own physical owners until actual GPU completion.
    pub fn clear_idle_resource_pool(&mut self) -> crate::Result<()> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "a compiled frame still owns resource-pool allocations",
            ));
        }
        let (textures, buffers) = self.pool.drain();
        for token in textures
            .into_iter()
            .map(|resource| resource.handle)
            .chain(buffers.into_iter().map(|resource| resource.handle))
        {
            self.backend.discard_pooled(token);
        }
        Ok(())
    }

    /// B render 0x1404e8... replaces the two temporal tables on OOM recovery.
    /// clear() retires their ownership; collect_named_retired keeps in-flight
    /// frames alive instead of treating table removal as GPU completion.
    pub fn reset_all_temporals(&mut self) -> crate::Result<()> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "a compiled frame still owns temporal references",
            ));
        }
        let named = self
            .named
            .as_mut()
            .ok_or_else(|| crate::Error::invalid_usage("named frame is already executing"))?;
        named.state.clear();
        // 将状态表的退役令牌移交给物理 owner；在飞帧仍由后端按 pending 保活。
        named.state.release_retired(&mut named.resources);
        self.collect_named_retired();
        Ok(())
    }
    pub fn named_resource_count(&self) -> usize {
        self.named.as_ref().map_or(0, |named| named.resources.len())
    }
    /// Inspect a named owner through its graph-scoped typed ID, never a physical identifier.
    pub fn named_texture_resource(
        &self,
        graph: &graph::RenderGraph,
        id: super::resource::TextureId,
    ) -> crate::Result<Arc<texture::Texture>> {
        graph.check_handle(id.raw())?;
        let (graph::OriginSource::ImportOwned { token, .. }
        | graph::OriginSource::ImportUnowned { token, .. }) =
            graph.declaration.resources[id.raw().raw() as usize].source
        else {
            return Err(crate::Error::invalid_usage("texture is not a named import"));
        };
        match self
            .named
            .as_ref()
            .and_then(|named| named.resources.resource(token))
        {
            Some(GpuResource::Image(texture)) => Ok(texture),
            _ => Err(crate::Error::invalid_usage(
                "texture is not owned by this session",
            )),
        }
    }
    fn compile_built(
        &mut self,
        built: graph::BuiltGraph,
        validate: impl FnOnce(&graph::FrameInstance) -> crate::Result<()>,
    ) -> crate::Result<PreparedSessionFrame> {
        let graph_id = built.graph_id();
        let (declaration, callbacks, retire, rejection) = built.into_parts();
        let result = (|| -> crate::Result<(super::executor::PreparedGraph, super::receipt_storage::ReceiptLease)> {
            if self.is_poisoned() {
                return Err(crate::Error::DeviceLost);
            }
            if let Some(error) = rejection {
                return Err(error);
            }
            self.result_cache.prepare(
                graph_id,
                &declaration,
                self.backend.device.generation().value(),
            )?;
            self.pending.reserve_frame()?;
            if self.backend.receipt_storage.is_none() {
                self.backend.receipt_storage = Some(super::receipt_storage::ReceiptStorage::new(
                    Default::default(),
                    self.backend.device.memory.stats.clone(),
                )?);
            }
            let mut access = self.backend.accesses.clone();
            let prepared = self
                .chain
                .compile_graph(
                    declaration,
                    &mut self.pool,
                    &mut access,
                    &mut self.backend,
                    callbacks.len(),
                );
            let mut prepared = prepared.map_err(crate::Error::from)?;
            let reservation = (|| {
                validate(&prepared.compiled)?;
                let capacity = prepared.compiled.snapshot.batches.len();
                self.backend.receipt_storage
                    .as_mut()
                    .expect("receipt storage initialized")
                    .prepare(&prepared.compiled.snapshot, capacity)
            })();
            match reservation {
                Ok(slot) => Ok((prepared, slot)),
                Err(error) => {
                    graph::reclaim_failed_resources(
                        &mut prepared.compiled.tables.registry,
                        &prepared.compiled.declarations.resources,
                        &prepared.compiled.tables.snapshot.pool_hits,
                        &mut self.pool,
                        &mut self.backend,
                    );
                    Err(error)
                }
            }
        })();
        match result {
            Ok((prepared, receipt_slot)) => Ok(PreparedSessionFrame {
                graph_id,
                generation: self.backend.device.generation(),
                prepared,
                callbacks,
                retire,
                receipt_slot,
            }),
            Err(error) => {
                self.result_cache.abort(graph_id);
                if error.is_device_lost() {
                    self.backend.device.queues.timelines.mark_device_lost();
                }
                let _ = run_retire_callbacks(retire);
                Err(error)
            }
        }
    }

    pub fn execute_prepared(
        &mut self,
        frame: PreparedSessionFrame,
    ) -> crate::Result<SessionFrameReceipt> {
        if self.active_prepared != Some(frame.graph_id)
            || frame.generation != self.backend.device.generation()
        {
            return Err(crate::Error::invalid_usage(
                "compiled frame belongs to another session",
            ));
        }
        if self.is_poisoned() {
            self.cancel_prepared(frame)?;
            return Err(crate::Error::DeviceLost);
        }
        if let Err(error) = self.begin_presentation_execution(frame.graph_id) {
            self.cancel_prepared(frame)?;
            return Err(error);
        }
        self.active_prepared = None;
        let PreparedSessionFrame {
            graph_id,
            prepared,
            mut callbacks,
            retire,
            receipt_slot,
            ..
        } = frame;
        self.last_execution_partial = false;
        self.backend.submissions.clear();
        if self.ledger.has_ended() {
            self.ledger.begin();
        }
        let mut access = self.backend.accesses.clone();
        self.backend.active_receipt_slot = Some(receipt_slot);
        let executed = self.chain.execute_prepared_validated(
            prepared,
            &mut self.pool,
            &mut access,
            &mut self.backend,
            &mut callbacks,
            |_| Ok(()),
        );
        let receipt_slot = self.backend.active_receipt_slot.take().expect("active frame receipt lease");
        self.finish_presentation_execution();
        let result = (|| match executed {
            Ok(execution) => {
                let frame = self.next_frame;
                self.next_frame += 1;
                let storage = receipt_slot.publish(execution.transaction.receipts());
                let receipt = SessionFrameReceipt {
                    frame,
                    storage,
                    stats: execution.stats,
                };
                self.end_frame(PendingSessionFrame {
                    failed: false,
                    frame,
                    display: self.current_display,
                    execution,
                    retire,
                    started_at: std::time::Instant::now(),
                });
                Ok(receipt)
            }
            Err(error) => {
                let (error, deferred) = match error {
                    super::executor::Error::PartialSubmission {
                        original,
                        execution,
                    } => (crate::Error::from(*original), Some(*execution)),
                    error => (crate::Error::from(error), None),
                };
                self.device_lost =
                    error.is_device_lost() || self.backend.device.queues.timelines.is_device_lost();
                if self.device_lost {
                    self.backend.device.queues.timelines.mark_device_lost();
                }
                if let Some(execution) = deferred {
                    self.last_execution_partial = true;
                    let frame = self.next_frame;
                    self.next_frame += 1;
                    self.end_frame(PendingSessionFrame {
                        failed: true,
                        frame,
                        display: self.current_display,
                        execution,
                        retire,
                        started_at: std::time::Instant::now(),
                    });
                    return Err(error);
                }
                self.backend.clear_pass_resources();
                self.recycle_completed_commands();
                if let Err(cleanup) = self.backend.collect_completed_managed() {
                    log::warn!("失败帧清理失败: {cleanup}");
                }
                let _ = run_retire_callbacks(retire);
                Err(error)
            }
        })();
        match &result {
            Ok(receipt) if !receipt.receipts.is_empty() => {
                self.result_cache.commit(graph_id);
                if let Some(named) = &mut self.named {
                    named.state.confirm_exports();
                }
            }
            _ => {
                self.result_cache.abort(graph_id);
                if let Some(named) = &mut self.named {
                    if self.last_execution_partial {
                        named.state.invalidate_submitted_frame();
                    } else {
                        named.state.abandon_frame();
                    }
                }
            }
        }
        // The declaration was consumed regardless of submit success. Pending
        // GPU work and committed history have their own independent owners.
        if let Some(named) = &mut self.named {
            named.graph_id = None;
        }
        // Recording has transferred every used physical resource to command
        // owners, including a partially submitted prefix. The declaration's
        // token lookup can be reclaimed, except exports which post-submit
        // consumers (screenshots, presentation) still resolve by receipt.
        let exports = result.as_ref().map_or(&[][..], |receipt| receipt.snapshot.exports.as_slice());
        self.collect_recorded_imports(Some((graph_id, exports)));
        result
    }

    fn recycle_completed_commands(&mut self) {
        while let Some(command) = self.chain.recycle() {
            if let Some(raw) = self.backend.commands.get_mut(&command.handle()) {
                unsafe { raw.release_completed_owners() };
            }
            self.backend.recycled_commands.push(command.handle());
        }
    }
    fn end_frame(&mut self, pending: PendingSessionFrame) {
        // A frame must contain an actual submitted receipt before completion is published.
        // 这里的 execute_frame 只在有回执时才到这一步，空图（无 pass）可能没有回执，所以只告警不 panic。
        if !pending.execution.transaction.receipts().is_empty() {
            self.ledger.note_submission();
        }
        if let Err(error) = self.ledger.end() {
            log::warn!("end_frame 账本异常: {error:?}");
        }
        self.pending.end_frame(pending);
    }
    /// Reclaim only frames whose actual queue timelines are already complete.
    pub fn collect_completed_frames(&mut self) -> crate::Result<usize> {
        let completed = self.backend.device.poll()?;
        let mut count = 0;
        while let Some(pending) = self.pending.front() {
            if !pending.execution.completion().is_reached(&completed) {
                break;
            }
            let frame = pending.frame;
            self.wait_frame(frame, 0)?;
            count += 1;
        }
        self.collect_cached_results(&completed)?;
        self.collect_frame_imports();
        Ok(count)
    }
    pub fn pending_frame_count(&self) -> usize {
        self.pending.in_flight()
    }
    /// 有界等候队头；即使完成也不弹出，退役只在 begin_frame/wait_frame 发生。
    pub fn wait_gpu_queue_trim(&self, timeout: u64) -> Result<bool, crate::Error> {
        let Some(pending) = self.pending.front() else {
            return Ok(true);
        };
        match self
            .backend
            .device
            .queues
            .timelines
            .wait(&pending.execution.completion(), timeout)
        {
            Ok(()) => Ok(true),
            Err(crate::Error::Timeout) => Ok(false),
            Err(error) => Err(error),
        }
    }

    pub fn wait_frame(&mut self, frame: u64, timeout: u64) -> crate::Result<()> {
        let pending = self
            .pending
            .front()
            .ok_or_else(|| crate::Error::invalid_usage("没有待完成帧"))?;
        if pending.frame != frame {
            return Err(crate::Error::invalid_usage("帧回执不属于当前待完成帧"));
        }
        // 等待失败（含超时）时帧仍在队头，调用方可重试。
        self.backend
            .device
            .queues
            .timelines
            .wait(&pending.execution.completion(), timeout)?;
        let pending = self.pending.pop_front().unwrap();
        self.finish_frame(pending)
    }
    fn finish_frame(&mut self, pending: PendingSessionFrame) -> crate::Result<()> {
        let mut first_error = None;
        keep_first_error(
            &mut first_error,
            self.backend.poll_gpu_timings().map(|_| ()),
        );
        for point in pending.execution.transaction.receipts() {
            self.chain.observe_completion(*point);
        }
        // 真正完成后才把 Command 交回后端；下次 reset/重录时仍使用递增的 per-queue counter。
        self.recycle_completed_commands();
        keep_first_error(&mut first_error, self.backend.collect_completed_managed());
        let mut compiled = pending.execution.compiled;
        let completed = match self.backend.device.poll() {
            Ok(completed) => Some(completed),
            Err(error) => { keep_first_error(&mut first_error, Err(error)); None }
        };
        if pending.failed || completed.is_none() {
            graph::reclaim_failed_resources(
                &mut compiled.tables.registry,
                &compiled.declarations.resources,
                &compiled.tables.snapshot.pool_hits,
                &mut self.pool,
                &mut self.backend,
            );
        } else {
            let completed = completed.expect("completion queried before cache publication");
            // A sorted vector instead of a tree: one allocation, no node churn.
            let mut created = compiled
                .declarations
                .resources
                .iter()
                .enumerate()
                .filter(|(_, r)| matches!(r.source, graph::OriginSource::Create))
                .filter_map(|(i, _)| compiled.snapshot.physical_mapping[i].token)
                .collect::<Vec<u64>>();
            created.sort_unstable();
            created.dedup();
            // 只有本图创建的非导出资源加入 Pool；外部导入不被误当作可任意覆盖的暂存资源。
            let registry = &mut compiled.tables.registry;
            let mut exported: Vec<_> = registry.exported_logical_handles.iter()
                .filter_map(|logical| registry.resources.get(*logical as usize).map(|mapping| mapping.backing_index as usize)).collect();
            exported.sort_unstable(); exported.dedup();
            for (physical, resource) in registry.backing_resources.drain(..).enumerate() {
                if exported.binary_search(&physical).is_ok() { continue; }
                match resource {
                    graph::RegistryResource::OwnedBuffer(buffer)
                        if created.binary_search(&buffer.handle).is_ok() =>
                    {
                        if graph::ResourceBackend::is_transient_alias(&self.backend, buffer.handle)
                        {
                            graph::ResourceBackend::release_created_resource(
                                &mut self.backend,
                                buffer.handle,
                            );
                            continue;
                        }
                        if let Some(GpuResource::Buffer(actual)) =
                            self.backend.resources.get(&buffer.handle)
                        {
                            self.backend
                                .device
                                .memory
                                .stats
                                .mark_cached(actual.allocation_id(), &completed);
                        }
                        let token = buffer.handle;
                        self.pool.insert_buffer(buffer);
                        graph::ResourceBackend::park_pooled(&mut self.backend, token);
                    }
                    graph::RegistryResource::OwnedTexture(texture)
                        if created.binary_search(&texture.handle).is_ok() =>
                    {
                        if graph::ResourceBackend::is_transient_alias(&self.backend, texture.handle)
                        {
                            graph::ResourceBackend::release_created_resource(
                                &mut self.backend,
                                texture.handle,
                            );
                            continue;
                        }
                        if let Some(GpuResource::Image(actual)) =
                            self.backend.resources.get(&texture.handle)
                        {
                            self.backend
                                .device
                                .memory
                                .stats
                                .mark_cached(actual.allocation_id(), &completed);
                        }
                        let token = texture.handle;
                        self.pool.insert_texture(texture);
                        graph::ResourceBackend::park_pooled(&mut self.backend, token);
                    }
                    _ => {}
                }
            }
        }
        keep_first_error(&mut first_error, run_retire_callbacks(pending.retire));
        self.collect_frame_imports();
        self.last_frame_duration_ns = pending.started_at.elapsed().as_nanos() as u64;
        first_error.map_or(Ok(()), Err)
    }
    /// 显式淘汰缓存，必须在当前帧完成后调用；保持 counter/timeline，避免 resize 残留无限增长。
    pub fn collect_after_completion(
        &mut self,
        retained_tokens: &[u64],
    ) -> Result<usize, crate::Error> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "a compiled frame still owns its physical resources",
            ));
        }
        if !self.pending.is_empty() {
            return Err(crate::Error::invalid_usage(
                "待完成帧仍拥有资源，不能清理缓存",
            ));
        }
        self.backend.device.wait_idle()?;
        self.result_cache.abandon_pending();
        self.result_cache.clear();
        self.collect_cached_results(&crate::sync::CompletedValues::all())?;
        // 缩放/收尾级等待：idle 后推进并回收提交引擎。
        self.backend.clear_pass_resources();
        for command in self.backend.commands.values_mut() {
            unsafe { command.release_completed_owners() };
        }
        let removed = self
            .backend
            .resources
            .keys()
            .copied()
            .filter(|token| {
                !retained_tokens.contains(token)
                    && !self
                        .named
                        .as_ref()
                        .is_some_and(|named| named.state.owns_token(*token))
            })
            .collect::<Vec<_>>();
        self.pool.clear();
        let parked = self.backend.caches.clear_idle();
        for token in &removed {
            self.backend.resources.remove(token);
            self.backend.import_families.remove(token);
        }
        self.backend.device.try_collect_all_completed()?;
        Ok(removed.len() + parked)
    }
    /// 释放单个导入资源（`bind_import*` 登记的 token），不需要整设备空闲，也不清别的缓存。
    ///
    /// 只拒绝“仍被在飞帧引用”的 token：那种资源 GPU 可能还在用，必须等对应帧 `wait_frame` 之后。
    /// 返回是否真的移除了东西（未登记的 token 返回 `Ok(false)`）。
    /// 以前只能靠 `collect_after_completion`（要求在飞为空 + 设备级 `wait_idle`）才会放掉导入资源，
    /// 每帧新建并导入的资源因此会一直堆到排空点。
    pub fn release_import(&mut self, token: u64) -> Result<bool, crate::Error> {
        if self
            .named
            .as_ref()
            .is_some_and(|named| named.state.owns_token(token))
        {
            return Err(crate::Error::invalid_usage(
                "named imports remain owned by their session",
            ));
        }
        if self.result_cache.owns_token(token) {
            return Err(crate::Error::invalid_usage(
                "result-cache imports remain owned until cache eviction",
            ));
        }
        if self.result_import_in_flight(token) {
            return Err(crate::Error::invalid_usage(format!(
                "token {token} 仍被在飞帧引用，不能释放"
            )));
        }
        self.backend.clear_pass_resources();
        let removed = self.backend.resources.remove(&token).is_some();
        self.backend.import_families.remove(&token);
        self.backend.collect_completed_managed()?;
        Ok(removed)
    }
    pub(crate) fn result_import_in_flight(&self, token: u64) -> bool {
        self.pending.iter().any(|frame| {
            frame
                .execution
                .compiled
                .snapshot
                .physical_mapping
                .iter()
                .any(|mapping| mapping.token == Some(token))
        })
    }
    pub fn clear_after_completion(&mut self) -> Result<usize, crate::Error> {
        self.collect_after_completion(&[])
    }
    /// Physical resources the session owns, including idle pooled ones
    /// parked in the evictable cache.
    pub fn resource_count(&self) -> usize {
        self.backend.resources.len() + self.backend.caches.parked_count()
    }
    pub fn command_count(&self) -> usize {
        self.backend.commands.len()
    }
}
pub(crate) fn run_retire_callbacks(callbacks: Vec<graph::RetireCallback>) -> crate::Result<()> {
    let mut first_error = None;
    for callback in callbacks {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(callback)).is_err() {
            keep_first_error(
                &mut first_error,
                Err(crate::Error::invalid_usage("退役回调 panic")),
            );
        }
    }
    first_error.map_or(Ok(()), Err)
}
fn keep_first_error(first: &mut Option<crate::Error>, result: crate::Result<()>) {
    if let Err(error) = result {
        log::warn!("帧完成清理步骤失败: {error}");
        first.get_or_insert(error);
    }
}
impl Drop for VulkanGraphSession {
    fn drop(&mut self) {
        while let Some(frame) = self.pending.front().map(|p| p.frame) {
            if let Err(error) = self.wait_frame(frame, u64::MAX) {
                // Device loss is terminal. An ordinary query failure needs an
                // actual idle proof before invoking callbacks or recycling pools.
                let safe = error.is_device_lost()
                    || self.backend.device.queues.timelines.is_device_lost()
                    || self.backend.device.wait_idle().is_ok();
                if safe {
                    while let Some(pending) = self.pending.pop_front() {
                        let _ = self.finish_frame(pending);
                    }
                } else {
                    log::error!("无法确认关闭时 GPU 已停止，保留在飞帧及退役回调: {error}");
                    while let Some(pending) = self.pending.pop_front() {
                        std::mem::forget(pending);
                    }
                }
                break;
            }
        }
        let _ = self.clear_after_completion();
    }
}

struct NamedResources {
    state: super::temporal::TemporalRenderGraphState,
    resources: TemporalGpuResources,
    graph_id: Option<super::resource::GraphId>,
}
impl NamedResources {
    fn new(device: Arc<Device>) -> Self {
        Self {
            state: Default::default(),
            resources: TemporalGpuResources::new(device),
            graph_id: None,
        }
    }
    fn validate_graph(&self, graph: &graph::RenderGraph) -> crate::Result<()> {
        if self.graph_id != Some(graph.graph_id()) {
            return Err(crate::Error::invalid_usage(
                "named resource request requires the session's current frame graph",
            ));
        }
        Ok(())
    }
    fn ensure_capacity(&self, key: &str, additional: usize) -> crate::Result<()> {
        if !self.resources.specifications.contains_key(&crate::resource::ResourceName::new(key))
            && self
                .resources
                .specifications
                .len()
                .saturating_add(additional)
                > 1024
        {
            return Err(crate::Error::unsupported(
                "session named-resource capacity exceeded",
            ));
        }
        Ok(())
    }
}

/// 帧缓存持有实际资源；每帧后端导入 Arc 后，resize 可安全释放旧缓存引用。
struct TemporalGpuResources {
    pub(crate) device: Arc<Device>,
    specifications: BTreeMap<crate::resource::ResourceName, ResourceSpec>,
    /// Keys whose buffer is zero-filled when it is (re)created.
    zero_initialized: std::collections::BTreeSet<String>,
    /// Tokens whose complete allocation was uploaded at creation.
    defined: std::collections::BTreeSet<u64>,
    resources: BTreeMap<u64, GpuResource>,
    retired: Vec<u64>,
}
impl TemporalGpuResources {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            device,
            specifications: BTreeMap::new(),
            zero_initialized: Default::default(),
            defined: Default::default(),
            resources: BTreeMap::new(),
            retired: Vec::new(),
        }
    }
    fn register(&mut self, name: &str, spec: ResourceSpec) -> Option<ResourceSpec> {
        self.specifications.insert(name.into(), spec)
    }
    fn set_zero_initialized(&mut self, name: &str, zeroed: bool) {
        if zeroed {
            if !self.zero_initialized.contains(name) {
                self.zero_initialized.insert(name.into());
            }
        } else {
            self.zero_initialized.remove(name);
        }
    }
    fn restore_specification(&mut self, name: &str, previous: Option<ResourceSpec>) {
        if let Some(previous) = previous {
            self.specifications.insert(name.into(), previous);
        } else {
            self.specifications.remove(&crate::resource::ResourceName::new(name));
        }
    }
    pub fn resource(&self, token: u64) -> Option<GpuResource> {
        self.resources.get(&token).cloned()
    }
    pub fn import_into(
        &self,
        backend: &mut VulkanGraphBackend,
        _name: &str,
        token: u64,
    ) -> crate::Result<()> {
        let resource = self
            .resource(token)
            .ok_or_else(|| crate::Error::invalid_usage("跨帧物理资源缺失"))?;
        let family = backend
            .import_families
            .get(&token)
            .copied()
            .unwrap_or(backend.device.queues.primary_family);
        backend.bind_existing_token(token, resource, family)
    }
    pub fn len(&self) -> usize {
        self.resources.len()
    }
}
impl super::temporal::TemporalResourceOwner for TemporalGpuResources {
    fn create_temporal(
        &mut self,
        name: &str,
        descriptor: graph::Descriptor,
    ) -> Result<u64, graph::BuildError> {
        let spec = self.specifications.get(&crate::resource::ResourceName::new(name)).copied().ok_or_else(|| {
            graph::BuildError::ResourceCreationFailed(format!("没有{name}的物理资源规格"))
        })?;
        if spec.descriptor() != descriptor {
            return Err(graph::BuildError::ResourceCreationFailed(
                "跨帧声明与物理规格不一致".into(),
            ));
        }
        // Contents of a new named resource are undefined (history_valid is
        // false and a read before the first write is rejected) unless the
        // caller asked for a zero-filled buffer, which costs one upload.
        let zeroed = self.zero_initialized.contains(name);
        let resource = match spec {
            ResourceSpec::Buffer(desc) | ResourceSpec::ConcurrentBuffer(desc, _) if zeroed => {
                let size = usize::try_from(desc.size)
                    .map_err(|_| graph::BuildError::from(crate::Error::OutOfHostMemory))?;
                let mut initial = Vec::new();
                initial
                    .try_reserve_exact(size)
                    .map_err(|_| graph::BuildError::from(crate::Error::OutOfHostMemory))?;
                initial.resize(size, 0u8);
                let object = match spec.sharing() {
                    crate::resource::ResourceSharing::Exclusive => self
                        .device
                        .create_buffer(&desc, name, Some(initial.as_slice()))
                        .map_err(graph::BuildError::from)?,
                    crate::resource::ResourceSharing::Concurrent(families) => {
                        let mut physical_desc = desc;
                        physical_desc.usage |= vk::BufferUsageFlags::TRANSFER_DST;
                        let raw = buffer::Buffer::new_concurrent(
                            self.device.clone(),
                            &physical_desc,
                            families.as_slice(),
                        )
                        .map_err(graph::BuildError::from)?;
                        let managed = self
                            .device
                            .caches
                            .resources
                            .buffers
                            .allocate(raw)
                            .map_err(graph::BuildError::from)?;
                        self.device.set_buffer_debug_name(&managed, name);
                        if size != 0 {
                            self.device
                                .update_buffer_bytes(
                                    &managed,
                                    &initial,
                                    std::slice::from_ref(&(0..size)),
                                    0,
                                )
                                .map_err(graph::BuildError::from)?;
                        }
                        managed
                    }
                };
                GpuResource::Buffer(object.shared())
            }
            _ => spec
                .create_resource(self.device.clone(), 0)
                .map_err(graph::BuildError::from)?,
        };
        let token = try_allocate_token()
            .ok_or_else(|| graph::BuildError::ResourceCreationFailed("跨帧token耗尽".into()))?;
        self.resources.insert(token, resource);
        if zeroed
            && matches!(
                spec,
                ResourceSpec::Buffer(_) | ResourceSpec::ConcurrentBuffer(..)
            )
        {
            self.defined.insert(token);
        }
        Ok(token)
    }
    fn initial_contents_defined(&self, token: u64) -> bool {
        self.defined.contains(&token)
    }
    fn temporal_sharing(&self, name: &str) -> crate::resource::ResourceSharing {
        self.specifications
            .get(&crate::resource::ResourceName::new(name))
            .map(|spec| spec.sharing())
            .unwrap_or_default()
    }
    fn release_temporal(&mut self, token: u64) {
        self.resources.remove(&token);
        self.defined.remove(&token);
        self.retired.push(token);
    }
}

#[cfg(test)]
mod submission_capture_tests {
    use super::*;

    #[test]
    fn submission_capture_is_inline_or_fallibly_budgeted_for_its_real_lifetime() {
        let stats = Arc::new(crate::raw::allocator::AllocatorStats::default());
        stats.set_total_budget(1).unwrap();
        let batch = submit::PreparedSubmit {
            commands: [vk::CommandBuffer::from_raw(1)].into_iter().collect(),
            ..Default::default()
        };
        let capture = GpuSubmissionCapture::try_new(
            vk::Queue::from_raw(1),
            std::slice::from_ref(&batch),
            2,
            &stats,
        )
        .unwrap();
        assert!(!capture.batches.spilled());
        assert_eq!(capture.batches[0].heap_capacity_bytes(), 0);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        drop(capture);
        let batches = [batch.clone(), batch];
        assert!(matches!(
            GpuSubmissionCapture::try_new(vk::Queue::from_raw(1), &batches, 2, &stats),
            Err(crate::Error::OutOfHostMemory)
        ));
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        stats.set_total_budget(1 << 20).unwrap();
        let capture =
            GpuSubmissionCapture::try_new(vk::Queue::from_raw(1), &batches, 2, &stats).unwrap();
        assert!(capture.batches.spilled());
        assert_eq!(capture.batches.len(), 2);
        assert!(
            stats.cpu_memory_usage().cache
                >= (2 * std::mem::size_of::<submit::PreparedSubmit>()) as u64
        );
        drop(capture);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        assert_eq!(batches[0].commands[0], vk::CommandBuffer::from_raw(1));
    }
}

#[cfg(test)]
mod import_contract_tests {
    use super::*;

    fn rejected(declared: graph::Descriptor, actual: graph::Descriptor) {
        assert_eq!(
            validate_import_descriptor(declared, actual)
                .unwrap_err()
                .kind(),
            crate::ErrorKind::InvalidUsage
        );
    }

    #[test]
    fn import_accepts_extra_physical_usage_but_not_missing_usage_or_wrong_kind() {
        let declared = ResourceSpec::Buffer(buffer::BufferDesc::new_gpu_only(
            64,
            vk::BufferUsageFlags::TRANSFER_SRC,
        ))
        .descriptor();
        let actual = declared.with_usage(
            (vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::STORAGE_BUFFER).as_raw(),
        );
        assert!(validate_import_descriptor(declared, actual).is_ok());
        rejected(actual, declared);
        let image = ResourceSpec::Image(texture::TextureDesc::new_2d(
            4,
            4,
            vk::Format::R8G8B8A8_UNORM,
        ))
        .descriptor();
        rejected(image, declared);
        rejected(declared, image);
    }

    #[test]
    fn import_rejects_buffer_extent_alignment_and_memory_location_mismatches() {
        let desc = buffer::BufferDesc::new_gpu_only(64, vk::BufferUsageFlags::STORAGE_BUFFER);
        let actual = ResourceSpec::Buffer(desc).descriptor();
        let mut variants = [desc; 3];
        variants[0].size = 128;
        variants[1].alignment = 256;
        variants[2].memory_location = buffer::MemoryLocation::CpuToGpu;
        for changed in variants {
            rejected(ResourceSpec::Buffer(changed).descriptor(), actual);
        }
    }

    #[test]
    fn import_rejects_image_shape_format_and_subresource_mismatches() {
        let desc = texture::TextureDesc::new_2d(16, 16, vk::Format::R8G8B8A8_UNORM);
        let actual = ResourceSpec::Image(desc).descriptor();
        let mut variants = [desc; 9];
        variants[0].width = 32;
        variants[1].height = 32;
        variants[2].depth = 2;
        variants[3].array_elements = 2;
        variants[4].mip_levels = 2;
        variants[5].sample_count = 4;
        variants[6].format = vk::Format::B8G8R8A8_UNORM;
        variants[7].kind = texture::TextureKind::Cube;
        variants[8].flags = vk::ImageCreateFlags::MUTABLE_FORMAT;
        for changed in variants {
            rejected(ResourceSpec::Image(changed).descriptor(), actual);
        }
    }
}

#[cfg(test)]
mod retire_callback_tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn a_panicking_retire_callback_does_not_discard_later_cleanup() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let callbacks: Vec<graph::RetireCallback> = (0..3)
            .map(|index| {
                let calls = calls.clone();
                Box::new(move || {
                    calls.lock().unwrap().push(index);
                    if index == 1 {
                        panic!("injected cleanup failure");
                    }
                }) as graph::RetireCallback
            })
            .collect();
        assert_eq!(
            run_retire_callbacks(callbacks).unwrap_err().kind(),
            crate::ErrorKind::InvalidUsage
        );
        assert_eq!(*calls.lock().unwrap(), [0, 1, 2]);
    }
}

#[cfg(test)]
mod enabled_access_stage_tests {
    use super::*;
    use crate::raw::device::DeviceCapabilities;
    use graph::AccessDependency;

    #[test]
    fn original_graphics_scope_omits_optional_stages_that_were_not_enabled() {
        let mut table = AccessTable::original();
        let original = table.0[&0x1d];
        let optional = vk::PipelineStageFlags::GEOMETRY_SHADER
            | vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
            | vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER;
        assert!(original.scope.stages.contains(optional));
        table.normalize_enabled_stages(DeviceCapabilities::default());
        let (native, layout) = table.native_semantics(0x1d);
        assert_eq!(native.stages, original.scope.stages & !optional);
        assert!(!native.stages.is_empty());
        assert_eq!(native.access, original.scope.access);
        assert_eq!(layout, original.layout);
        assert_eq!(table.access_mask(0x1d), original.scope.access);
        assert_eq!(
            crate::raw::access_table::ACCESS_TABLE[0x1d].scope.stages,
            original.scope.stages
        );
    }

    #[test]
    fn custom_optional_only_scopes_fall_back_and_empty_no_access_stays_empty() {
        let mut table = AccessTable::default();
        for (code, stages, access) in [
            (
                200,
                vk::PipelineStageFlags::GEOMETRY_SHADER,
                vk::AccessFlags::SHADER_READ,
            ),
            (
                201,
                vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
                    | vk::PipelineStageFlags::FRAGMENT_SHADER,
                vk::AccessFlags::SHADER_READ,
            ),
            (
                202,
                vk::PipelineStageFlags::empty(),
                vk::AccessFlags::SHADER_WRITE,
            ),
            (
                203,
                vk::PipelineStageFlags::empty(),
                vk::AccessFlags::empty(),
            ),
        ] {
            table.0.insert(
                code,
                AccessSpec {
                    scope: crate::resource::range_state::AccessScope {
                        stages,
                        access,
                        write: access.contains(vk::AccessFlags::SHADER_WRITE),
                    },
                    layout: vk::ImageLayout::GENERAL,
                },
            );
        }
        table.normalize_enabled_stages(DeviceCapabilities::default());
        assert_eq!(
            table.native_semantics(200).0.stages,
            vk::PipelineStageFlags::ALL_COMMANDS
        );
        assert_eq!(
            table.native_semantics(201).0.stages,
            vk::PipelineStageFlags::FRAGMENT_SHADER
        );
        assert_eq!(
            table.native_semantics(202).0.stages,
            vk::PipelineStageFlags::ALL_COMMANDS
        );
        assert_eq!(
            table.native_semantics(203).0.stages,
            vk::PipelineStageFlags::empty()
        );
    }

    #[test]
    fn explicitly_enabled_geometry_and_tessellation_scopes_are_preserved() {
        for (geometry_shader, tessellation_shader) in [(true, true), (true, false), (false, true)] {
            let mut table = AccessTable::original();
            let original = table.0[&0x1d].scope.stages;
            table.normalize_enabled_stages(DeviceCapabilities {
                geometry_shader,
                tessellation_shader,
                ..Default::default()
            });
            let native = table.native_semantics(0x1d).0.stages;
            assert_eq!(
                native.contains(vk::PipelineStageFlags::GEOMETRY_SHADER),
                geometry_shader
            );
            assert_eq!(
                native.contains(vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER),
                tessellation_shader
            );
            assert_eq!(
                native.contains(vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER),
                tessellation_shader
            );
            if geometry_shader && tessellation_shader {
                assert_eq!(native, original);
            }
        }
    }
}
/// A session and its compiled frames may move to a dedicated render thread.
const _: fn() = || {
    fn send<T: Send>() {}
    send::<VulkanGraphSession>();
    send::<PreparedSessionFrame>();
};

#[cfg(test)]
mod declaration_rejection_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn session() -> Option<VulkanGraphSession> {
        let device = match Device::new_headless(&crate::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "strict graph rejection GPU initialization failed: {error}");
                eprintln!("SKIP graph rejection GPU initialization: {error}");
                return None;
            }
        };
        Some(VulkanGraphSession::new(VulkanGraphBackend::new(device, AccessTable::original()).unwrap()))
    }

    #[test]
    fn rejection_precedes_uninitialized_read_and_compile_allocation_then_next_frame_recovers() {
        let Some(mut session) = session() else { return };
        let device = session.backend.device().clone();
        let desc = buffer::BufferDesc::new_gpu_only(64,
            vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST);
        let imported = Arc::new(buffer::Buffer::new(device.clone(), &desc).unwrap());
        let imported_weak = Arc::downgrade(&imported);
        let mut graph = session.begin_frame().unwrap();
        session.import_frame_buffer(&mut graph, imported, 0x34).unwrap();
        let named = session.named_buffer(&mut graph, "rejection-history", desc).unwrap();
        assert!(!named.history_valid);
        // Named storage is created during declaration and may stay cached;
        // rejecting compilation must not create any additional named owners.
        let named_before_compile = session.named_resource_count();
        assert_eq!(named_before_compile, 1);
        let executed = Arc::new(AtomicUsize::new(0));
        let callback_owner = Arc::new(());
        let callback_weak = Arc::downgrade(&callback_owner);
        let calls = executed.clone();
        let mut reader = graph.add_pass("uninitialized downstream consumer").with_side_effect();
        reader.add_resource_access(named.buffer.raw(), 0x30, 0).unwrap();
        reader.render_with_result_context(move |_| {
            let _retain = &callback_owner;
            calls.fetch_add(1, Ordering::Relaxed);
            Ok(())
        });
        let retired = Arc::new(AtomicUsize::new(0));
        let calls = retired.clone();
        graph.on_retire(move || { calls.fetch_add(1, Ordering::Relaxed); });
        graph.on_retire(|| panic!("injected retirement failure must not replace OOM"));
        graph.reject(crate::Error::OutOfHostMemory);
        graph.reject(crate::Error::invalid_usage("later producer failure"));
        let submitted = device.retirement_stats().submitted;
        assert!(matches!(session.execute_frame(graph.build()), Err(crate::Error::OutOfHostMemory)));
        assert_eq!(executed.load(Ordering::Relaxed), 0);
        assert_eq!(retired.load(Ordering::Relaxed), 1);
        assert!(callback_weak.upgrade().is_none());
        assert!(imported_weak.upgrade().is_none());
        assert_eq!(session.frame_import_count(), 0);
        assert_eq!(session.pending_frame_count(), 0);
        assert_eq!(session.named_resource_count(), named_before_compile);
        assert!(session.active_prepared.is_none());
        assert!(session.backend.receipt_storage.is_none());
        assert_eq!(device.retirement_stats().submitted, submitted);
        assert!(!session.is_poisoned());
        let mut next = session.begin_frame().unwrap();
        let named = session.named_buffer(&mut next, "rejection-history", desc).unwrap();
        assert!(!named.history_valid);
        session.cancel_frame(next.build()).unwrap();
        let next = session.begin_frame().unwrap();
        let prepared = session.compile_frame(next.build()).unwrap();
        session.cancel_prepared(prepared).unwrap();
    }

    #[test]
    fn cancelling_rejected_graph_retires_once_and_does_not_poison_next_frame() {
        let Some(mut session) = session() else { return };
        let retired = Arc::new(AtomicUsize::new(0));
        let mut graph = session.begin_frame().unwrap();
        let calls = retired.clone();
        graph.on_retire(move || { calls.fetch_add(1, Ordering::Relaxed); });
        graph.reject(crate::Error::OutOfHostMemory);
        session.cancel_frame(graph.build()).unwrap();
        assert_eq!(retired.load(Ordering::Relaxed), 1);
        let graph = session.begin_frame().unwrap();
        let prepared = session.compile_frame(graph.build()).unwrap();
        session.cancel_prepared(prepared).unwrap();
        assert_eq!(retired.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn rejected_device_lost_is_terminal_and_retirement_failure_cannot_replace_it() {
        let Some(mut session) = session() else { return };
        let mut graph = session.begin_frame().unwrap();
        graph.on_retire(|| panic!("cleanup cannot mask device loss"));
        graph.reject(crate::Error::DeviceLost);
        assert!(matches!(session.compile_frame(graph.build()), Err(crate::Error::DeviceLost)));
        assert!(session.is_poisoned());
        assert_eq!(session.pending_frame_count(), 0);
        assert!(session.active_prepared.is_none());
    }
}
