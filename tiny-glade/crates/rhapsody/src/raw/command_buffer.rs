//! Vulkan command pools and buffers with precise resource ownership and retirement.
use super::device::Device;
use ash::vk;
use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicU64, AtomicUsize, Ordering},
};

/// 仅在 RHAPSODY_SUBMIT_TRACE=1 时启用的提交分段观测，开关每进程读取一次。
/// 禁用时不取时间、不分配；启用时所有样本也放在固定栈数组中。
pub(crate) struct SubmitTrace {
    active: Option<SubmitTraceActive>,
}
struct SubmitTraceActive {
    scope: &'static str,
    start: std::time::Instant,
    last: std::time::Instant,
    phases: [(&'static str, u64); 12],
    phase_count: usize,
    counts: [(&'static str, usize); 6],
    count_count: usize,
}
impl SubmitTrace {
    pub(crate) fn configured() -> bool {
        static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *ENABLED.get_or_init(|| {
            std::env::var_os("RHAPSODY_SUBMIT_TRACE").is_some_and(|value| value == "1")
        })
    }
    pub(crate) fn new(scope: &'static str) -> Self {
        Self {
            active: Self::configured().then(|| {
                let now = std::time::Instant::now();
                SubmitTraceActive {
                    scope,
                    start: now,
                    last: now,
                    phases: [("", 0); 12],
                    phase_count: 0,
                    counts: [("", 0); 6],
                    count_count: 0,
                }
            }),
        }
    }
    pub(crate) fn enabled(&self) -> bool {
        self.active.is_some()
    }
    pub(crate) fn mark(&mut self, phase: &'static str) {
        let Some(active) = &mut self.active else {
            return;
        };
        let now = std::time::Instant::now();
        if let Some(slot) = active.phases.get_mut(active.phase_count) {
            *slot = (
                phase,
                now.duration_since(active.last)
                    .as_nanos()
                    .min(u64::MAX as u128) as u64,
            );
            active.phase_count += 1;
        }
        active.last = now;
    }
    pub(crate) fn count(&mut self, name: &'static str, value: usize) {
        let Some(active) = &mut self.active else {
            return;
        };
        if let Some(slot) = active.counts.get_mut(active.count_count) {
            *slot = (name, value);
            active.count_count += 1;
        }
    }
}
impl Drop for SubmitTrace {
    fn drop(&mut self) {
        let Some(active) = &self.active else {
            return;
        };
        use std::io::Write as _;
        // 诊断输出失败不得覆盖 Vulkan 错误，也不能使原清理路径 panic。
        let _ = writeln!(
            std::io::stderr().lock(),
            "RHAPSODY_SUBMIT_TRACE scope={} total_ns={} phases_ns={:?} counts={:?}",
            active.scope,
            active.start.elapsed().as_nanos(),
            &active.phases[..active.phase_count],
            &active.counts[..active.count_count]
        );
    }
}

/// 实际命令池与命令缓冲持有关系；默认采用可单独重置的主命令缓冲。
pub struct VulkanCommandPool {
    // Rust drops fields in declaration order. Destroy the raw pool before the
    // last device owner; copies in Retirement are drained by Device::drop.
    allocation: Arc<CommandPoolAllocation>,
    pub(crate) device: Arc<Device>,
    pub(crate) handle: vk::CommandPool,
    pub family: u32,
    last_use: Arc<crate::resource::SyncCell>,
    live_commands: AtomicUsize,
    resets: AtomicU64,
    max_resources: usize,
    cpu_validation: Mutex<super::temp_list::TempList<OwnershipValidationState>>,
    cpu_state_patches: Mutex<super::temp_list::TempList<crate::resource::range_state::StatePatch>>,
    _memory: super::allocator::CpuMemoryReservation,
}

impl VulkanCommandPool {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::CommandPool {
        self.handle
    }
}

/// Pool storage retained by deferred command frees. It deliberately does not own
/// Arc<Device>; the retirement queue is drained before vkDestroyDevice.
///
/// Vulkan requires the `VkCommandPool` to be externally synchronized for every
/// pool-level call and for recording into any of its buffers. Recording
/// buffers are `&mut`-exclusive and the owning thread performs begin/end and
/// allocate/reset, all serialized by `lock`. The one call that can arrive from
/// an arbitrary thread is the deferred free issued by device retirement; while
/// any buffer of the pool is recording, such frees are queued and executed
/// when the last recording ends, so they never race the recording thread.
pub struct CommandPoolAllocation {
    device: ash::Device,
    handle: vk::CommandPool,
    lock: Mutex<PoolSync>,
    _memory: super::allocator::CpuMemoryReservation,
}
#[derive(Default)]
struct PoolSync {
    recording: usize,
    deferred_free: Vec<vk::CommandBuffer>,
}
impl CommandPoolAllocation {
    pub(crate) fn free(&self, handle: vk::CommandBuffer) {
        let mut sync = self.lock.lock();
        if sync.recording != 0 {
            if sync.deferred_free.try_reserve(1).is_ok() {
                sync.deferred_free.push(handle);
            }
            // On host allocation failure the handle is reclaimed with the pool.
            return;
        }
        unsafe {
            self.device.free_command_buffers(self.handle, &[handle]);
        }
    }
    fn end_recording(&self, sync: &mut PoolSync) {
        sync.recording -= 1;
        if sync.recording == 0 && !sync.deferred_free.is_empty() {
            unsafe {
                self.device
                    .free_command_buffers(self.handle, &sync.deferred_free);
            }
            sync.deferred_free.clear();
        }
    }
}
impl Drop for CommandPoolAllocation {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_command_pool(self.handle, None);
        }
    }
}
impl VulkanCommandPool {
    pub fn new(device: Arc<Device>) -> crate::Result<Arc<Self>> {
        let family = device.queues.primary_family;
        Self::new_for_family(device, family)
    }
    pub fn new_for_family(device: Arc<Device>, family: u32) -> crate::Result<Arc<Self>> {
        Self::new_for_family_with_cpu_limit(device, family, 65_536)
    }
    pub fn new_for_family_with_cpu_limit(
        device: Arc<Device>,
        family: u32,
        max_validation_items: usize,
    ) -> crate::Result<Arc<Self>> {
        Self::build(device, family, max_validation_items, None)
    }
    /// A recording wrapper over an existing native pool (kept by the device's
    /// setup cache without owning the device). The pool is destroyed when the
    /// last `allocation` owner drops, never by this wrapper.
    pub(crate) fn attach(
        device: Arc<Device>,
        family: u32,
        allocation: Arc<CommandPoolAllocation>,
    ) -> crate::Result<Arc<Self>> {
        Self::build(device, family, 65_536, Some(allocation))
    }
    /// The shared native pool, for caches that outlive this wrapper.
    pub(crate) fn allocation(&self) -> Arc<CommandPoolAllocation> {
        self.allocation.clone()
    }
    fn build(
        device: Arc<Device>,
        family: u32,
        max_validation_items: usize,
        existing: Option<Arc<CommandPoolAllocation>>,
    ) -> crate::Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let cpu_validation = Mutex::new(super::temp_list::TempList::new(
            max_validation_items,
            device.memory.stats.clone(),
        )?);
        let cpu_state_patches = Mutex::new(super::temp_list::TempList::new(
            max_validation_items,
            device.memory.stats.clone(),
        )?);
        if family != device.queues.primary_family
            && !device.queues.additional.iter().any(|q| q.family == family)
        {
            return Err(crate::Error::invalid_usage("命令状态或队列族无效"));
        }
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let memory =
            super::allocator::reserve_shared_cpu_metadata::<Self>(&device.memory.stats, 0)?;
        let allocation = match existing {
            Some(allocation) => allocation,
            None => {
                let info = vk::CommandPoolCreateInfo::default()
                    .queue_family_index(family)
                    .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
                let allocation_memory = super::allocator::reserve_shared_cpu_metadata::<
                    CommandPoolAllocation,
                >(&device.memory.stats, 0)?;
                let handle = super::error::observe_vulkan(&device, unsafe {
                    device.core.device.create_command_pool(&info, None)
                })?;
                Arc::new(CommandPoolAllocation {
                    device: device.core.device.clone(),
                    handle,
                    lock: Mutex::new(PoolSync::default()),
                    _memory: allocation_memory,
                })
            }
        };
        Ok(Arc::new(Self {
            max_resources: max_validation_items,
            cpu_validation,
            cpu_state_patches,
            last_use: crate::resource::SyncCell::for_device(device.generation())
                .into_reserved_arc(sync_storage),
            device,
            handle: allocation.handle,
            family,
            allocation,
            live_commands: AtomicUsize::new(0),
            resets: AtomicU64::new(0),
            _memory: memory,
        }))
    }
    /// Wrap a native command buffer previously detached from this pool's
    /// allocation ([`VulkanCommandBuffer::detach`]). The buffer must have
    /// been allocated from this pool and not freed.
    pub(crate) fn adopt(
        self: &Arc<Self>,
        handle: vk::CommandBuffer,
    ) -> crate::Result<VulkanCommandBuffer> {
        self.wrap(handle)
    }
    pub fn allocate(self: &Arc<Self>) -> crate::Result<VulkanCommandBuffer> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let _guard = self.allocation.lock.lock();
        let info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.handle)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        let handle = super::error::observe_vulkan(&self.device, unsafe {
            self.device.core.device.allocate_command_buffers(&info)
        })?[0];
        drop(_guard);
        match self.wrap(handle) {
            Ok(command) => Ok(command),
            Err(error) => {
                self.allocation.free(handle);
                Err(error)
            }
        }
    }
    fn wrap(self: &Arc<Self>, handle: vk::CommandBuffer) -> crate::Result<VulkanCommandBuffer> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let stats = &self.device.memory.stats;
        let range_uses = super::temp_list::TempList::new(self.max_resources, stats.clone())?;
        let state_patches = super::temp_list::TempList::new(self.max_resources, stats.clone())?;
        let owners = super::temp_list::TempList::new(self.max_resources, stats.clone())?;
        let mut uses = super::temp_list::TempList::new(self.max_resources, stats.clone())?;
        uses.add(self.last_use.clone())?;
        let upload_reads = super::temp_list::TempList::new(self.max_resources, stats.clone())?;
        self.live_commands.fetch_add(1, Ordering::Relaxed);
        Ok(VulkanCommandBuffer {
            handle,
            pool: self.clone(),
            recording: false,
            executable: false,
            detached: false,
            owners,
            use_set: {
                let mut set = crate::rg::token_map::TokenMap::default();
                set.insert(Arc::as_ptr(&self.last_use) as usize as u64, ());
                set
            },
            owner_set: Default::default(),
            uses,
            upload_reads,
            range_uses,
            state_patches,
            range_failure: None,
            last_use: crate::sync::SyncSet::EMPTY,
        })
    }
    /// 回收一条命令缓冲：等待设备完成、reset 并清空其拥有者，供下一帧复用。
    pub fn recycle(&self, command: &mut VulkanCommandBuffer) -> crate::Result<()> {
        command.reset()
    }

    /// Reset the entire pool once all of its live CPU commands are exclusively
    /// borrowed and every submission using this pool has actually completed.
    /// Failure preserves command states, owners and last-use receipts.
    pub fn reset_completed(&self, commands: &mut [&mut VulkanCommandBuffer]) -> crate::Result<()> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let _reservation = self.last_use.lock_submission();
        let mut guard = self.allocation.lock.lock();
        if commands.len() != self.live_commands.load(Ordering::Relaxed)
            || commands
                .iter()
                .any(|command| !std::ptr::eq(self, command.pool.as_ref()))
            || commands.iter().enumerate().any(|(index, command)| {
                commands[..index]
                    .iter()
                    .any(|previous| previous.handle == command.handle)
            })
        {
            return Err(crate::Error::invalid_usage(
                "whole-pool reset requires all live commands from this pool",
            ));
        }
        if !self
            .device
            .queues
            .timelines
            .is_reached(&self.last_use.snapshot())?
        {
            return Err(vk::Result::NOT_READY.into());
        }
        super::error::observe_vulkan(&self.device, unsafe {
            self.device
                .core
                .device
                .reset_command_pool(self.handle, vk::CommandPoolResetFlags::empty())
        })?;
        self.resets.fetch_add(1, Ordering::Relaxed);
        for command in commands.iter_mut() {
            if command.recording {
                self.allocation.end_recording(&mut guard);
                command.recording = false;
            }
        }
        drop(guard);
        drop(_reservation);
        for command in commands {
            command.clear_after_reset();
        }
        Ok(())
    }

    pub fn cpu_scratch_capacity_bytes(&self) -> usize {
        self.cpu_validation.lock().capacity_bytes() + self.cpu_state_patches.lock().capacity_bytes()
    }
    pub fn cpu_scratch_growth_count(&self) -> u64 {
        self.cpu_validation.lock().growth_count() + self.cpu_state_patches.lock().growth_count()
    }
    /// CPU-only validation storage can be trimmed while GPU commands are in
    /// flight; all temporary resource-state references have already been dropped.
    pub fn trim_cpu_scratch(&self) {
        self.cpu_validation.lock().trim_idle();
        self.cpu_state_patches.lock().trim_idle();
    }

    pub fn reset_count(&self) -> u64 {
        self.resets.load(Ordering::Relaxed)
    }
    /// Command buffers whose free waits for the pool's current recording to end.
    pub fn pending_free_count(&self) -> usize {
        self.allocation.lock.lock().deferred_free.len()
    }
    pub fn live_command_count(&self) -> usize {
        self.live_commands.load(Ordering::Relaxed)
    }
}

/// 命令缓冲录制时的图形状态缓存；`record_default` 提供初始默认值。
/// 候选只保留最少的深度/剔除/混合默认，具体默认策略由管线声明决定。
#[derive(Clone, Copy, Debug)]
pub struct CbGraphicsState {
    pub depth_test: bool,
    pub depth_write: bool,
    pub cull_mode: vk::CullModeFlags,
}
impl CbGraphicsState {
    pub fn record_default() -> Self {
        Self {
            depth_test: true,
            depth_write: true,
            cull_mode: vk::CullModeFlags::BACK,
        }
    }
}

pub struct VulkanCommandBuffer {
    pub(crate) handle: vk::CommandBuffer,
    pub pool: Arc<VulkanCommandPool>,
    recording: bool,
    executable: bool,
    /// The native buffer was handed to a cache ([`Self::detach`]); dropping
    /// this wrapper must not retire or free it.
    detached: bool,
    owners: super::temp_list::TempList<crate::raw::binding::ResourceOwner>,
    uses: super::temp_list::TempList<Arc<crate::resource::SyncCell>>,
    /// Pointer sets mirroring `uses` / the objects in `owners`: membership was a
    /// linear scan over every use for each retained resource (O(draws x uses)).
    use_set: crate::rg::token_map::TokenMap<()>,
    owner_set: crate::rg::token_map::TokenMap<()>,
    range_uses: super::temp_list::TempList<RecordedOpaqueUse>,
    state_patches: super::temp_list::TempList<RecordedStatePatch>,
    range_failure: Option<MetadataFailure>,
    upload_reads: super::temp_list::TempList<UploadRead>,
    last_use: crate::sync::SyncSet,
}

impl VulkanCommandBuffer {
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::CommandBuffer {
        self.handle
    }
}
#[derive(Clone, Copy)]
enum RetainMode {
    Opaque,
    Recorded,
    Object,
}
#[derive(Clone, Copy)]
enum MetadataFailure {
    DeviceLost,
    HostMemory,
    DeviceMemory,
    Timeout,
    Unsupported,
    InvalidUsage,
    SurfaceLost,
    OutOfDate,
    Exhausted,
    Vulkan(vk::Result),
}
impl MetadataFailure {
    fn from_error(error: &crate::Error) -> Self {
        match error {
            crate::Error::DeviceLost => Self::DeviceLost,
            crate::Error::OutOfHostMemory => Self::HostMemory,
            crate::Error::OutOfDeviceMemory => Self::DeviceMemory,
            crate::Error::Timeout => Self::Timeout,
            crate::Error::Exhausted(_) => Self::Exhausted,
            crate::Error::Unsupported(_) => Self::Unsupported,
            crate::Error::InvalidUsage { .. } => Self::InvalidUsage,
            crate::Error::SurfaceLost => Self::SurfaceLost,
            crate::Error::SurfaceOutOfDate => Self::OutOfDate,
            crate::Error::Vulkan(error) => Self::Vulkan(*error),
        }
    }
    fn error(self) -> crate::Error {
        match self {
            Self::DeviceLost => crate::Error::DeviceLost,
            Self::HostMemory => crate::Error::OutOfHostMemory,
            Self::DeviceMemory => crate::Error::OutOfDeviceMemory,
            Self::Timeout => crate::Error::Timeout,
            Self::Exhausted => {
                crate::Error::exhausted("command resource admission previously failed")
            }
            Self::Unsupported => {
                crate::Error::unsupported("command resource admission previously failed")
            }
            Self::InvalidUsage => {
                crate::Error::invalid_usage("command resource admission previously failed")
            }
            Self::SurfaceLost => crate::Error::SurfaceLost,
            Self::OutOfDate => crate::Error::SurfaceOutOfDate,
            Self::Vulkan(error) => crate::Error::Vulkan(error),
        }
    }
}
struct UploadRead {
    buffer: Arc<super::buffer::Buffer>,
    offset: u64,
    size: u64,
    scope: crate::resource::range_state::AccessScope,
}
struct RecordedOpaqueUse {
    cell: Arc<crate::resource::SyncCell>,
    range: OpaqueUseRange,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OpaqueUseRange {
    Whole,
    Bytes { offset: u64, size: u64 },
}
impl OpaqueUseRange {
    fn checked(offset: u64, size: u64, limit: u64) -> crate::Result<Self> {
        if size == 0 || offset.checked_add(size).is_none_or(|end| end > limit) {
            return Err(crate::Error::invalid_usage(
                "GPU buffer use range is out of bounds",
            ));
        }
        Ok(Self::Bytes { offset, size })
    }
}
struct RecordedStatePatch {
    cell: Arc<crate::resource::SyncCell>,
    patch: crate::resource::range_state::StatePatch,
}
type OwnershipValidationState = Arc<crate::resource::SyncCell>;

/// All capacity and preconditions are prepared while the caller holds the
/// resource gates. Failure cancels every staged cell; success publishes once
/// per physical object with the same real timeline point as command retirement.
pub(crate) struct PreparedResourceStates {
    cells: super::submit::SubmitScratch<[Arc<crate::resource::SyncCell>; 32]>,
    queue: crate::sync::QueueId,
    dependencies: crate::sync::StagedDependencies,
    published: bool,
}
impl PreparedResourceStates {
    /// Prerequisites without stage information (conservative consumers).
    pub(crate) fn dependencies(&self) -> crate::sync::SyncSet {
        self.dependencies.points()
    }
    /// Prerequisites with the destination stages of the declared accesses
    /// that need them; opaque uses carry `ALL_COMMANDS`.
    pub(crate) fn staged_dependencies(&self) -> crate::sync::StagedDependencies {
        self.dependencies
    }
    fn prepare_cell(
        &mut self,
        index: usize,
        patches: &[crate::resource::range_state::StatePatch],
        family: u32,
        stats: &Arc<super::allocator::AllocatorStats>,
    ) -> crate::Result<()> {
        let cell = &self.cells[index];
        cell.prepare_range_state(patches, self.queue, family, stats)?;
        self.dependencies
            .merge(&cell.prepared_range_staged_dependencies());
        Ok(())
    }
    pub(crate) fn publish(mut self, point: crate::sync::SyncPoint) {
        assert_eq!(
            point.queue, self.queue,
            "native state and queue completion must match"
        );
        for cell in self.cells.iter() {
            cell.publish_range_state(point);
        }
        self.published = true;
    }
}
impl Drop for PreparedResourceStates {
    fn drop(&mut self) {
        if !self.published {
            for cell in self.cells.iter() {
                cell.cancel_range_state();
            }
        }
    }
}
/// Reusable index scratch for `prepare_submission_states`; keeps capacity per
/// thread so steady-state submission does not allocate.
#[derive(Default)]
struct BucketScratch {
    slot_of: std::collections::HashMap<usize, usize>,
    /// (cell slot, kind: 0 explicit patch / 1 opaque use / 2 private upload read,
    /// command index, entry index)
    recorded: Vec<(usize, u8, usize, usize)>,
}
thread_local! {
    static BUCKET_SCRATCH: std::cell::Cell<Option<BucketScratch>> =
        const { std::cell::Cell::new(None) };
}
impl BucketScratch {
    fn take() -> BucketGuard {
        let mut scratch = BUCKET_SCRATCH.with(|slot| slot.take()).unwrap_or_default();
        scratch.slot_of.clear();
        scratch.recorded.clear();
        BucketGuard(Some(scratch))
    }
}
struct BucketGuard(Option<BucketScratch>);
impl std::ops::Deref for BucketGuard {
    type Target = BucketScratch;
    fn deref(&self) -> &BucketScratch {
        self.0.as_ref().unwrap()
    }
}
impl std::ops::DerefMut for BucketGuard {
    fn deref_mut(&mut self) -> &mut BucketScratch {
        self.0.as_mut().unwrap()
    }
}
impl Drop for BucketGuard {
    fn drop(&mut self) {
        if let Some(scratch) = self.0.take() {
            BUCKET_SCRATCH.with(|slot| slot.set(Some(scratch)));
        }
    }
}
/// # Safety
/// The submission gates for all `uses` must remain held until this guard is
/// published or dropped. `commands` are in actual native execution order.
pub(crate) unsafe fn prepare_submission_states(
    commands: &[&VulkanCommandBuffer],
    uses: &[Arc<crate::resource::SyncCell>],
    queue: crate::sync::QueueId,
    family: u32,
    stats: &Arc<super::allocator::AllocatorStats>,
) -> crate::Result<PreparedResourceStates> {
    use crate::resource::range_state::{PatchAction, StatePatch};
    let mut trace = SubmitTrace::new("native_state_prepare");
    trace.count("commands", commands.len());
    trace.count("uses", uses.len());
    if trace.enabled() {
        trace.count(
            "state_patches",
            commands
                .iter()
                .map(|command| command.state_patches.as_slice().len())
                .sum(),
        );
        trace.count(
            "opaque_ranges",
            commands
                .iter()
                .map(|command| command.range_uses.as_slice().len())
                .sum(),
        );
        trace.count(
            "private_upload_reads",
            commands
                .iter()
                .map(|command| command.upload_reads.as_slice().len())
                .sum(),
        );
    }
    let physical = uses.iter().filter(|cell| cell.ownership_range().is_some());
    let cells = super::submit::SubmitScratch::try_collect(
        physical.clone().count(),
        stats,
        physical.map(|cell| Ok(cell.clone())),
    )?;
    let mut prepared = PreparedResourceStates {
        cells,
        queue,
        dependencies: crate::sync::StagedDependencies::EMPTY,
        published: false,
    };
    let Some(first) = commands.first() else {
        if !prepared.cells.is_empty() {
            return Err(crate::Error::invalid_usage(
                "native memory state requires recorded commands",
            ));
        }
        return Ok(prepared);
    };
    let mut scratch = first.pool.cpu_state_patches.lock();
    // One pass buckets every recorded patch by physical cell (command order,
    // explicit before opaque inside a command), instead of re-filtering all
    // patches once per cell.
    let mut bucket = BucketScratch::take();
    let BucketScratch {
        slot_of,
        recorded,
    } = &mut *bucket;
    for (index, cell) in prepared.cells.iter().enumerate() {
        assert!(
            slot_of.insert(Arc::as_ptr(cell) as usize, index).is_none(),
            "submission uses must name each physical cell once"
        );
    }
    for (command_index, command) in commands.iter().enumerate() {
        for (entry_index, entry) in command.state_patches.as_slice().iter().enumerate() {
            if let Some(&slot) = slot_of.get(&(Arc::as_ptr(&entry.cell) as usize)) {
                recorded.push((slot, 0, command_index, entry_index));
            }
        }
        for (entry_index, entry) in command.range_uses.as_slice().iter().enumerate() {
            if let Some(&slot) = slot_of.get(&(Arc::as_ptr(&entry.cell) as usize)) {
                recorded.push((slot, 1, command_index, entry_index));
            }
        }
        for (entry_index, read) in command.upload_reads.as_slice().iter().enumerate() {
            if let Some(&slot) = slot_of.get(&(Arc::as_ptr(&read.buffer.last_use) as usize)) {
                recorded.push((slot, 2, command_index, entry_index));
            }
        }
    }
    recorded.sort_by_key(|item| item.0);
    for index in 0..prepared.cells.len() {
        let cell = prepared.cells[index].clone();
        let domain = cell.ownership_range().expect("filtered native domain");
        let start = recorded.partition_point(|item| item.0 < index);
        let end = recorded.partition_point(|item| item.0 <= index);
        let own = &recorded[start..end];
        let count = own.iter().filter(|item| item.1 < 2).count();
        let has_upload = own.iter().any(|item| item.1 == 2);
        let fallback = count == 0 && !has_upload;
        scratch.with_entries(|patches|->crate::Result<()> {
            patches.reserve(count.max(usize::from(fallback)))?;
            for &(_, kind, command_index, entry_index) in own.iter().filter(|item| item.1 < 2) {
                if kind == 0 {
                    patches.add(commands[command_index].state_patches.as_slice()[entry_index].patch)?;
                } else {
                    let entry = &commands[command_index].range_uses.as_slice()[entry_index];
                    {
                        let range=match entry.range {OpaqueUseRange::Whole=>domain,OpaqueUseRange::Bytes {offset,size}=>crate::resource::OwnershipRange::Buffer {offset,size}};
                        patches.add(StatePatch {range,expected_revision:u64::MAX,action:PatchAction::Unknown {queue,family}})?;
                    }
                }
            }
            if fallback {patches.add(StatePatch {range:domain,expected_revision:u64::MAX,action:PatchAction::Unknown {queue,family}})?;}
            let declared=patches.as_slice().len();
            // A bindless parameter page records one private read per bind (hundreds
            // on one buffer). Preparing each read incrementally rescans every cell
            // the earlier reads split off: O(reads^2). When the reads are pairwise
            // disjoint they cannot affect one another, so generate every patch
            // from the state after the declared accesses and apply them in one
            // sweep; the end state is identical to the one-by-one path below.
            let batched_reads: smallvec::SmallVec<[&UploadRead; 16]> = own
                .iter()
                .filter(|item| item.1 == 2)
                .map(|&(_, _, command_index, entry_index)| {
                    &commands[command_index].upload_reads.as_slice()[entry_index]
                })
                .collect();
            if batched_reads.len() >= 4
                && let Some(merged) = merge_upload_reads(&batched_reads)
            {
                for &(offset, end, _) in &merged {
                    let range = crate::resource::OwnershipRange::Buffer { offset, size: end - offset };
                    if patches.as_slice()[..declared].iter().any(|patch| crate::resource::range_state::intersection(patch.range, range).is_some()) {
                        return Err(crate::Error::unsupported("private upload read overlaps a declared or opaque native access"));
                    }
                }
                cell.prepare_range_state(patches.as_slice(), queue, family, stats)?;
                cell.with_prepared_range_state(|table| {
                    for &(offset, end, scope) in &merged {
                        let wanted = crate::resource::OwnershipRange::Buffer { offset, size: end - offset };
                        for current in table.prepared_cells() {
                            let Some(range) = crate::resource::range_state::intersection(current.range, wanted) else { continue; };
                            if !current.state.writer.points.is_empty() || current.state.writer.scopes.iter().any(|scope| !scope.is_empty()) {
                                return Err(crate::Error::unsupported("private upload read has a GPU producer; a declared native access is required"));
                            }
                            patches.add(StatePatch { range, expected_revision: current.state.revision, action: PatchAction::Access { scope, layout: vk::ImageLayout::UNDEFINED, queue, family } })?;
                        }
                    }
                    Ok(())
                })?;
                cell.append_prepared_range_state_disjoint(&patches.as_slice()[declared..], queue, family)?;
                prepared.dependencies.merge(&cell.prepared_range_staged_dependencies());
                return Ok(());
            }
            let mut prepared_prefix = None;
            for read in own.iter().filter(|item|item.1==2).map(|&(_,_,command_index,entry_index)|&commands[command_index].upload_reads.as_slice()[entry_index]) {
                let range=crate::resource::OwnershipRange::Buffer {offset:read.offset,size:read.size};
                if patches.as_slice()[..declared].iter().any(|patch|crate::resource::range_state::intersection(patch.range,range).is_some()) {
                    return Err(crate::Error::unsupported("private upload read overlaps a declared or opaque native access"));
                }
                // 先检查当前 read 重叠，再准备前一 read 的新增补丁，保持首错误顺序。
                match prepared_prefix {
                    Some(end) => cell.append_prepared_range_state(&patches.as_slice()[end..], queue, family)?,
                    None => cell.prepare_range_state(patches.as_slice(), queue, family, stats)?,
                }
                prepared_prefix = Some(patches.as_slice().len());
                cell.with_prepared_range_state(|table| {
                    for current in table.prepared_cells() {
                        let Some(range)=crate::resource::range_state::intersection(current.range,range) else {continue;};
                        if !current.state.writer.points.is_empty() || current.state.writer.scopes.iter().any(|scope|!scope.is_empty()) {
                            return Err(crate::Error::unsupported("private upload read has a GPU producer; a declared native access is required"));
                        }
                        patches.add(StatePatch {range,expected_revision:current.state.revision,action:PatchAction::Access {scope:read.scope,layout:vk::ImageLayout::UNDEFINED,queue,family}})?;
                    }
                    Ok(())
                })?;
            }
            if let Some(end) = prepared_prefix {
                cell.append_prepared_range_state(&patches.as_slice()[end..], queue, family)?;
                prepared.dependencies.merge(&cell.prepared_range_staged_dependencies());
                Ok(())
            } else {
                prepared.prepare_cell(index,patches.as_slice(),family,stats)
            }
        })?;
    }
    trace.mark("prepare_all_cells");
    trace.count("physical_cells", prepared.cells.len());
    Ok(prepared)
}

/// Unions private upload reads into disjoint byte spans `(start, end, scope)`.
/// Duplicates, overlaps and touching spans with the same scope become one span;
/// the bytes covered and the scope applied to each byte are exactly those of the
/// original reads. `None` when a span is empty or overflows, or when reads with
/// different scopes overlap (the one-by-one path then decides the outcome).
fn merge_upload_reads(reads: &[&UploadRead]) -> Option<MergedReads> {
    let mut spans = MergedReads::new();
    for read in reads {
        spans.push((read.offset, read.offset.checked_add(read.size)?, read.scope));
        if read.size == 0 {
            return None;
        }
    }
    merge_read_spans(spans)
}
type MergedReads =
    smallvec::SmallVec<[(u64, u64, crate::resource::range_state::AccessScope); 16]>;
fn merge_read_spans(mut spans: MergedReads) -> Option<MergedReads> {
    spans.sort_unstable_by_key(|span| (span.0, span.1));
    let mut merged = MergedReads::new();
    for span in spans {
        match merged.last_mut() {
            Some(last) if span.0 < last.1 => {
                if span.2 != last.2 {
                    return None;
                }
                last.1 = last.1.max(span.1);
            }
            Some(last) if span.0 == last.1 && span.2 == last.2 => last.1 = span.1,
            _ => merged.push(span),
        }
    }
    Some(merged)
}

pub(crate) fn validate_submission_commands(commands: &[&VulkanCommandBuffer]) -> crate::Result<()> {
    let Some(first) = commands.first() else {
        return Ok(());
    };
    // Each command's `uses` is already sorted and unique. The previous
    // implementation also de-duplicated across commands into a scratch list
    // that nothing read, which was O(U^2) pointer comparisons plus an Arc clone
    // per cell, twice per submission.
    for command in commands {
        if let Some(error) = command.range_failure {
            return Err(error.error());
        }
        if !command.executable {
            return Err(crate::Error::invalid_usage("提交命令尚未结束录制"));
        }
        if first.pool.family != command.pool.family
            || !Arc::ptr_eq(&first.pool.device, &command.pool.device)
        {
            return Err(crate::Error::invalid_usage(
                "submitted commands use different devices or queue families",
            ));
        }
        for cell in command.uses.as_slice() {
            cell.validate_device(command.pool.device.generation())?;
            cell.validate_sharing_family(command.pool.family)?;
        }
    }
    Ok(())
}

fn record_command_uses(uses: &[Arc<crate::resource::SyncCell>], point: crate::sync::SyncPoint) {
    for cell in uses {
        cell.insert_overall(point);
    }
}
impl VulkanCommandBuffer {
    fn clear_after_reset(&mut self) {
        self.recording = false;
        self.executable = false;
        self.owners.clear();
        self.uses.clear();
        self.owner_set.clear();
        self.use_set.clear();
        self.range_uses.clear();
        self.state_patches.clear();
        self.upload_reads.clear();
        self.range_failure = None;
        self.uses
            .add(self.pool.last_use.clone())
            .expect("pool cell capacity was admitted at allocation");
        self.use_set
            .insert(Arc::as_ptr(&self.pool.last_use) as usize as u64, ());
        self.last_use = crate::sync::SyncSet::EMPTY;
    }
    pub fn is_recording(&self) -> bool {
        self.recording
    }
    pub(crate) fn has_submissions(&self) -> bool {
        !self.last_use.is_empty()
    }
    /// 安全公开入口先确认设备完成，再复用原命令缓冲及释放其帧内拥有者。
    pub fn reset(&mut self) -> crate::Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        // A rejected metadata operation has issued no queue work. Vulkan permits
        // resetting this recording buffer; preserve normal begin/end discipline
        // while allowing its admitted storage to recover after pressure clears.
        if self.recording && self.range_failure.is_none() {
            return Err(crate::Error::invalid_usage("命令状态或队列族无效"));
        }
        self.pool
            .device
            .queues
            .timelines
            .wait(&self.last_use, u64::MAX)?;
        {
            let mut sync = self.pool.allocation.lock.lock();
            if self.recording {
                self.pool.allocation.end_recording(&mut sync);
                self.recording = false;
            }
            super::error::observe_vulkan(&self.pool.device, unsafe {
                self.pool
                    .device
                    .core
                    .device
                    .reset_command_buffer(self.handle, vk::CommandBufferResetFlags::empty())
            })?;
        }
        self.clear_after_reset();
        Ok(())
    }
    /// 与 `reset` 相同但不做整设备 idle，用于帧循环复用。
    ///
    /// # Safety
    /// 调用者必须已等待覆盖此命令缓冲全部提交的时间线/fence（与 `release_completed_owners` 同一契约）。
    pub unsafe fn reset_after_completion(&mut self) -> crate::Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        {
            let mut sync = self.pool.allocation.lock.lock();
            if self.recording {
                self.pool.allocation.end_recording(&mut sync);
                self.recording = false;
            }
            super::error::observe_vulkan(&self.pool.device, unsafe {
                self.pool
                    .device
                    .core
                    .device
                    .reset_command_buffer(self.handle, vk::CommandBufferResetFlags::empty())
            })?;
        }
        self.clear_after_reset();
        Ok(())
    }
    pub fn is_executable(&self) -> bool {
        self.executable
    }
    pub fn begin(&mut self) -> crate::Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if self.recording || self.executable {
            return Err(crate::Error::invalid_usage("命令状态或队列族无效"));
        }
        let info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        let mut sync = self.pool.allocation.lock.lock();
        super::error::observe_vulkan(&self.pool.device, unsafe {
            self.pool
                .device
                .core
                .device
                .begin_command_buffer(self.handle, &info)
        })?;
        sync.recording += 1;
        self.recording = true;
        Ok(())
    }
    pub fn end(&mut self) -> crate::Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if !self.recording {
            return Err(crate::Error::invalid_usage("命令状态或队列族无效"));
        }
        if let Some(error) = self.range_failure {
            return Err(error.error());
        }
        {
            let mut sync = self.pool.allocation.lock.lock();
            let ended = super::error::observe_vulkan(&self.pool.device, unsafe {
                self.pool.device.core.device.end_command_buffer(self.handle)
            });
            // A failed end leaves the buffer invalid; either way it no longer
            // records, so pending frees may proceed.
            self.pool.allocation.end_recording(&mut sync);
            if let Err(error) = ended {
                self.recording = false;
                return Err(error);
            }
        }
        self.normalize_uses();
        self.recording = false;
        self.executable = true;
        Ok(())
    }
    /// # Safety
    /// command 必须在合法录制状态（GRAPHICS 绑定点）。
    pub unsafe fn set_stencil_reference(&self, value: u8) {
        unsafe {
            self.pool.device.core.device.cmd_set_stencil_reference(
                self.handle,
                vk::StencilFaceFlags::FRONT_AND_BACK,
                u32::from(value),
            )
        }
    }
    /// Retain an explicitly opaque raw reference. Every native memory domain
    /// contributes a conservative Unknown patch to the same submission protocol.
    pub fn retain_opaque<T: crate::resource::GpuUse + 'static>(
        &mut self,
        owner: Arc<T>,
    ) -> crate::Result<()> {
        self.retain_owner(owner, RetainMode::Opaque)
    }
    /// Retain a GPU object that exposes no buffer/image memory domain.
    pub fn retain_object<T: crate::resource::GpuUse + 'static>(
        &mut self,
        owner: Arc<T>,
    ) -> crate::Result<()> {
        self.retain_owner(owner, RetainMode::Object)
    }
    /// # Safety
    /// Actual memory accesses must be covered by the validated native range
    /// program. This is private to producers which have established that proof.
    pub(crate) unsafe fn retain_recorded<T: crate::resource::GpuUse + 'static>(
        &mut self,
        owner: Arc<T>,
    ) -> crate::Result<()> {
        self.retain_owner(owner, RetainMode::Recorded)
    }
    /// # Safety
    /// The synchronous caller keeps every borrowed resource alive until the
    /// command's real completion; all memory accesses have native state patches.
    pub(crate) unsafe fn retain_borrowed_recorded(
        &mut self,
        owner: &dyn crate::resource::GpuUse,
    ) -> crate::Result<()> {
        self.retain_borrowed(owner, RetainMode::Recorded)
    }
    fn retain_owner(
        &mut self,
        owner: crate::raw::binding::ResourceOwner,
        mode: RetainMode,
    ) -> crate::Result<()> {
        self.ensure_retention_allowed()?;
        let result = (|| {
            self.add_owner(&owner)?;
            self.visit_resource(owner.as_ref(), mode)
        })();
        self.remember_metadata_error(&result);
        result
    }
    fn ensure_retention_allowed(&self) -> crate::Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if let Some(error) = self.range_failure {
            return Err(error.error());
        }
        if self.has_submissions() {
            return Err(crate::Error::invalid_usage(
                "cannot modify a submitted command's resources",
            ));
        }
        Ok(())
    }
    fn retain_borrowed(
        &mut self,
        owner: &dyn crate::resource::GpuUse,
        mode: RetainMode,
    ) -> crate::Result<()> {
        self.ensure_retention_allowed()?;
        let result = self.visit_resource(owner, mode);
        self.remember_metadata_error(&result);
        result
    }
    fn visit_resource(
        &mut self,
        owner: &dyn crate::resource::GpuUse,
        mode: RetainMode,
    ) -> crate::Result<()> {
        owner.visit_uses(&mut |cell| {
            cell.validate_device(self.pool.device.generation())?;
            if matches!(mode, RetainMode::Object) && cell.ownership_range().is_some() {
                return Err(crate::Error::invalid_usage(
                    "retain_object cannot retain a native memory resource",
                ));
            }
            if matches!(mode, RetainMode::Opaque) {
                self.note_opaque_use(cell.clone(), OpaqueUseRange::Whole)?;
            }
            self.add_use_cell(cell)
        })?;
        Ok(())
    }
    /// Keep `owner` alive until completion. A resource retained again by the
    /// same command (a heap or pipeline bound by every draw) is stored once.
    fn add_owner(&mut self, owner: &crate::raw::binding::ResourceOwner) -> crate::Result<()> {
        let key = Arc::as_ptr(owner) as *const () as usize as u64;
        if self.owner_set.contains_key(&key) {
            return Ok(());
        }
        self.owners.add(owner.clone())?;
        self.owner_set.insert(key, ());
        Ok(())
    }
    fn add_use_cell(&mut self, cell: &Arc<crate::resource::SyncCell>) -> crate::Result<()> {
        let key = Arc::as_ptr(cell) as usize as u64;
        if self.use_set.contains_key(&key) {
            return Ok(());
        }
        self.uses.add(cell.clone())?;
        self.use_set.insert(key, ());
        if self.executable {
            // `end` sorted the uses and `use_set` excludes duplicates, so placing
            // the new cell keeps them sorted (a full sort per patch was quadratic).
            self.uses.sift_last_by_key(|cell| Arc::as_ptr(cell) as usize);
        }
        Ok(())
    }
    fn remember_metadata_error(&mut self, result: &crate::Result<()>) {
        if let Err(error) = result {
            if error.is_device_lost() {
                self.pool.device.queues.timelines.mark_device_lost();
            }
            self.range_failure
                .get_or_insert(MetadataFailure::from_error(error));
        }
    }
    /// Virtual state for consecutive setup/copy helpers in this same command.
    /// Committed physical state is unchanged and no future point is fabricated.
    pub(crate) fn with_recorded_state<R>(
        &self,
        cell: &Arc<crate::resource::SyncCell>,
        initial: crate::resource::range_state::NativeState,
        queue: crate::sync::QueueId,
        run: impl FnOnce(&crate::resource::range_state::RangeStateTable) -> crate::Result<R>,
    ) -> crate::Result<R> {
        use crate::resource::range_state::PatchAction;
        let entries = self
            .state_patches
            .as_slice()
            .iter()
            .filter(|entry| Arc::ptr_eq(&entry.cell, cell));
        // An acquire recorded earlier in this command is a valid predecessor
        // (the preview applies it like submission does); a release is not.
        if entries
            .clone()
            .any(|entry| matches!(entry.patch.action, PatchAction::Release(_)))
        {
            return Err(crate::Error::unsupported(
                "copy-helper preview cannot expose an unsubmitted ownership transfer",
            ));
        }
        let patches = super::submit::SubmitScratch::<[crate::resource::range_state::StatePatch; 8]>::try_collect(
            entries.clone().count(), &self.pool.device.memory.stats, entries.map(|entry| Ok(entry.patch)),
        )?;
        cell.preview_range_state(
            initial,
            &self.pool.device.memory.stats,
            &patches,
            queue,
            self.pool.family,
            run,
        )
    }
    pub(crate) fn recorded_state_queue(
        &self,
        cell: &Arc<crate::resource::SyncCell>,
    ) -> Option<crate::sync::QueueId> {
        self.state_patches
            .as_slice()
            .iter()
            .rev()
            .find_map(|entry| {
                if !Arc::ptr_eq(&entry.cell, cell) {
                    return None;
                }
                match entry.patch.action {
                    crate::resource::range_state::PatchAction::Access { queue, .. }
                    | crate::resource::range_state::PatchAction::Unknown { queue, .. } => {
                        Some(queue)
                    }
                    _ => None,
                }
            })
    }
    pub(crate) fn recorded_access(
        &self,
        cell: &Arc<crate::resource::SyncCell>,
        range: crate::resource::OwnershipRange,
    ) -> Option<(crate::resource::range_state::AccessScope, vk::ImageLayout)> {
        let entry = self.state_patches.as_slice().iter().rev().find(|entry| {
            Arc::ptr_eq(&entry.cell, cell)
                && crate::resource::range_state::contains(entry.patch.range, range)
        })?;
        match entry.patch.action {
            crate::resource::range_state::PatchAction::Access { scope, layout, .. } => {
                Some((scope, layout))
            }
            _ => None,
        }
    }
    pub(crate) fn has_opaque_overlap(
        &self,
        cell: &Arc<crate::resource::SyncCell>,
        range: crate::resource::OwnershipRange,
    ) -> bool {
        self.range_uses.as_slice().iter().any(|entry| {
            Arc::ptr_eq(&entry.cell, cell)
                && match entry.range {
                    OpaqueUseRange::Whole => true,
                    OpaqueUseRange::Bytes { offset, size } => {
                        crate::resource::range_state::intersection(
                            crate::resource::OwnershipRange::Buffer { offset, size },
                            range,
                        )
                        .is_some()
                    }
                }
        })
    }
    pub(crate) fn recorded_access_covers(
        &self,
        cell: &Arc<crate::resource::SyncCell>,
        range: crate::resource::OwnershipRange,
        mut predicate: impl FnMut(crate::resource::range_state::AccessScope, vk::ImageLayout) -> bool,
    ) -> crate::Result<bool> {
        if self.has_opaque_overlap(cell, range) {
            return Ok(false);
        }
        let Some(queue) = self.recorded_state_queue(cell) else {
            return Ok(false);
        };
        self.with_recorded_state(
            cell,
            crate::resource::range_state::NativeState::default(),
            queue,
            |table| {
                if !crate::resource::range_state::contains(table.domain(), range) {
                    return Ok(false);
                }
                let mut touched = false;
                for current in table.prepared_cells() {
                    if let Some(overlap) =
                        crate::resource::range_state::intersection(current.range, range)
                    {
                        touched = true;
                        let Some((scope, layout)) = self.recorded_access(cell, overlap) else {
                            return Ok(false);
                        };
                        if !predicate(scope, layout) {
                            return Ok(false);
                        }
                    }
                }
                Ok(touched)
            },
        )
    }
    pub(crate) fn record_state_patch(
        &mut self,
        cell: Arc<crate::resource::SyncCell>,
        patch: crate::resource::range_state::StatePatch,
    ) -> crate::Result<()> {
        self.ensure_retention_allowed()?;
        let result = (|| {
            cell.validate_device(self.pool.device.generation())?;
            crate::resource::range_state::validate_range(patch.range)?;
            if !cell
                .ownership_range()
                .is_some_and(|domain| crate::resource::range_state::contains(domain, patch.range))
            {
                return Err(crate::Error::invalid_usage(
                    "state patch exceeds native resource domain",
                ));
            }
            self.state_patches.add(RecordedStatePatch {
                cell: cell.clone(),
                patch,
            })?;
            self.add_use_cell(&cell)
        })();
        self.remember_metadata_error(&result);
        result
    }
    fn note_opaque_use(
        &mut self,
        cell: Arc<crate::resource::SyncCell>,
        range: OpaqueUseRange,
    ) -> crate::Result<()> {
        if cell.ownership_range().is_none()
            || self.range_uses.as_slice().iter().any(|previous| {
                Arc::ptr_eq(&previous.cell, &cell)
                    && (previous.range == OpaqueUseRange::Whole || previous.range == range)
            })
        {
            return Ok(());
        }
        self.range_uses.add(RecordedOpaqueUse { cell, range })?;
        Ok(())
    }
    /// Retain a buffer while describing exactly which bytes the native commands
    /// may access. Generic references to the same buffer still take precedence.
    /// # Safety
    /// The range must cover every read/write performed through this reference;
    /// matching Vulkan barriers and resource ownership are still required.
    pub unsafe fn retain_opaque_buffer_range(
        &mut self,
        buffer: Arc<super::buffer::Buffer>,
        offset: u64,
        size: u64,
    ) -> crate::Result<()> {
        self.ensure_retention_allowed()?;
        let result = (|| {
            buffer
                .last_use
                .validate_device(self.pool.device.generation())?;
            let range = OpaqueUseRange::checked(offset, size, buffer.size)?;
            self.note_opaque_use(buffer.last_use.clone(), range)?;
            let owner: crate::raw::binding::ResourceOwner = buffer.clone();
            self.add_owner(&owner)?;
            self.visit_resource(buffer.as_ref(), RetainMode::Recorded)
        })();
        self.remember_metadata_error(&result);
        result
    }
    /// Only immutable, flushed formal constants-ring ranges use this role.
    /// No GPU producer may be hidden behind this submission-time host read.
    pub(crate) fn retain_upload_read(
        &mut self,
        buffer: Arc<super::buffer::Buffer>,
        offset: u64,
        size: u64,
        scope: crate::resource::range_state::AccessScope,
    ) -> crate::Result<()> {
        self.ensure_retention_allowed()?;
        let result = (|| {
            if !buffer.is_dynamic_constants_page()
                || scope.write
                || scope.stages.is_empty()
                || scope.access.is_empty()
                || !(vk::AccessFlags::SHADER_READ | vk::AccessFlags::UNIFORM_READ)
                    .contains(scope.access)
            {
                return Err(crate::Error::invalid_usage(
                    "upload read requires a formal immutable shader-read block",
                ));
            }
            OpaqueUseRange::checked(offset, size, buffer.size)?;
            buffer
                .last_use
                .validate_device(self.pool.device.generation())?;
            self.visit_resource(buffer.as_ref(), RetainMode::Recorded)?;
            self.upload_reads.add(UploadRead {
                buffer,
                offset,
                size,
                scope,
            })?;
            Ok(())
        })();
        self.remember_metadata_error(&result);
        result
    }
    pub(crate) fn retain_resources(
        &mut self,
        owners: impl IntoIterator<Item = super::binding::RetainedResource>,
    ) -> crate::Result<()> {
        for owner in owners {
            match owner.0 {
                super::binding::Retention::Object(owner) => {
                    self.retain_owner(owner, RetainMode::Object)?
                }
                super::binding::Retention::Recorded(owner) => {
                    self.retain_owner(owner, RetainMode::Recorded)?
                }
                super::binding::Retention::UploadRead {
                    buffer,
                    offset,
                    size,
                    scope,
                } => self.retain_upload_read(buffer, offset, size, scope)?,
            }
        }
        Ok(())
    }
    pub(crate) fn normalize_uses(&mut self) {
        self.uses
            .sort_dedup_by_key(|cell| Arc::as_ptr(cell) as usize);
    }
    /// # Safety
    /// 调用者必须已等待覆盖此 CB 的提交凭据，不能仅依据 CPU 录制结束。
    pub unsafe fn release_completed_owners(&mut self) {
        self.owners.clear();
        self.uses.clear();
        self.owner_set.clear();
        self.use_set.clear();
        self.range_uses.clear();
        self.state_patches.clear();
        self.upload_reads.clear();
        self.range_failure = None;
        self.uses
            .add(self.pool.last_use.clone())
            .expect("pool cell capacity was admitted at allocation");
        self.use_set
            .insert(Arc::as_ptr(&self.pool.last_use) as usize as u64, ());
        self.last_use = crate::sync::SyncSet::EMPTY;
    }
    pub(crate) fn validate_submission(&self) -> crate::Result<()> {
        validate_submission_commands(&[self])
    }
    pub(crate) fn use_cells(&self) -> &[Arc<crate::resource::SyncCell>] {
        self.uses.as_slice()
    }
    pub(crate) fn record_submission(&mut self, point: crate::sync::SyncPoint) {
        self.last_use.insert(point);
        record_command_uses(self.uses.as_slice(), point);
    }
    pub fn submit(self) -> crate::Result<PendingCommand> {
        let queue = super::device::DeviceQueue {
            handle: self.pool.device.queues.primary,
            family: self.pool.device.queues.primary_family,
            index: 0,
        };
        self.submit_to(queue, &[], &[])
    }
    /// 显式生产时间线 wait/signal 数组，并将依赖持有到提交点登记。
    ///
    /// 提交与 PendingCommand 等待共用设备时间线。
    pub fn submit_to(
        mut self,
        queue: super::device::DeviceQueue,
        waits: &[(
            Arc<super::semaphore::TimelineSemaphore>,
            u64,
            vk::PipelineStageFlags,
        )],
        signals: &[(Arc<super::semaphore::TimelineSemaphore>, u64)],
    ) -> crate::Result<PendingCommand> {
        if !self.executable {
            return Err(crate::Error::invalid_usage("提交命令、队列或同步对象无效"));
        }
        self.validate_submission()?;
        let device = &self.pool.device;
        let known = (queue.handle == device.queues.primary
            && queue.family == device.queues.primary_family
            && queue.index == 0)
            || device.queues.additional.iter().any(|q| {
                q.handle == queue.handle && q.family == queue.family && q.index == queue.index
            });
        if !known
            || queue.family != self.pool.family
            || waits
                .iter()
                .any(|(s, _, stage)| !Arc::ptr_eq(&s.device, device) || stage.is_empty())
            || signals.iter().any(|(s, _)| !Arc::ptr_eq(&s.device, device))
        {
            return Err(crate::Error::invalid_usage("提交命令、队列或同步对象无效"));
        }
        let semaphore_count = waits
            .len()
            .checked_add(signals.len())
            .ok_or_else(|| crate::Error::invalid_usage("submission owner count overflows"))?;
        // The pending receipt owns the actual semaphore Arcs and their spill
        // credit together; dropping the caller's owners cannot release them.
        let semaphores = super::submit::SubmitScratch::<
            [Arc<super::semaphore::TimelineSemaphore>; 8],
        >::try_collect(
            semaphore_count,
            &device.memory.stats,
            waits
                .iter()
                .map(|(s, _, _)| Ok(s.clone()))
                .chain(signals.iter().map(|(s, _)| Ok(s.clone()))),
        )?;
        let wait_capacity = waits
            .len()
            .checked_add(device.queues.timelines.queue_count().saturating_sub(1))
            .ok_or_else(|| crate::Error::invalid_usage("submission wait count overflows"))?;
        let nested_bytes =
            super::submit::PreparedSubmit::heap_bytes_for_counts(wait_capacity, signals.len(), 1)?;
        // Declare the ticket first: all real nested arrays drop before their
        // credit on every path. Reserve foreign waits before taking any gates.
        let mut nested_memory = if nested_bytes == 0 {
            None
        } else {
            Some(
                device
                    .memory
                    .stats
                    .reserve_cpu_bytes(nested_bytes as u64, crate::device::BudgetCategory::Live)?,
            )
        };
        let mut prepared = [super::submit::PreparedSubmit::default()];
        {
            fn reserve<T>(
                values: &mut super::submit::SubmitArray<T>,
                count: usize,
            ) -> crate::Result<()> {
                values
                    .try_reserve_exact(count)
                    .map_err(|_| crate::Error::OutOfHostMemory)
            }
            let batch = &mut prepared[0];
            reserve(&mut batch.wait_semaphores, wait_capacity)?;
            reserve(&mut batch.wait_values, wait_capacity)?;
            reserve(&mut batch.wait_stages, wait_capacity)?;
            reserve(&mut batch.signal_semaphores, signals.len())?;
            reserve(&mut batch.signal_values, signals.len())?;
            for (semaphore, value, stage) in waits {
                batch.wait_semaphores.push(semaphore.handle);
                batch.wait_values.push(*value);
                batch.wait_stages.push(*stage);
            }
            for (semaphore, value) in signals {
                batch.signal_semaphores.push(semaphore.handle);
                batch.signal_values.push(*value);
            }
            batch.commands.push(self.handle);
        }
        if let Some(memory) = &mut nested_memory {
            memory.resize(prepared[0].heap_capacity_bytes() as u64)?;
        }
        let uses =
            super::submit::SubmitScratch::<[Arc<crate::resource::SyncCell>; 32]>::try_collect(
                self.uses.as_slice().len(),
                &device.memory.stats,
                self.uses.as_slice().iter().cloned().map(Ok),
            )?;
        let _reservations = crate::resource::tracked::lock_submission_uses(&uses)?;
        self.validate_submission()?;
        let native_states = unsafe {
            prepare_submission_states(
                &[&self],
                &uses,
                device.queue_id(queue.handle)?,
                queue.family,
                &device.memory.stats,
            )?
        };
        device.add_timeline_waits(queue.handle, &mut prepared, &native_states.dependencies())?;
        let result = device.submit_prepared(queue.handle, &prepared, vk::Fence::null());
        let point = result?;
        native_states.publish(point);
        self.record_submission(point);
        for semaphore in semaphores.iter() {
            semaphore.last_use.insert_overall(point);
        }
        Ok(PendingCommand {
            command: self,
            point,
            _semaphores: semaphores,
        })
    }
}
impl VulkanCommandBuffer {
    /// Hand the native buffer to a cache: this wrapper (and the resources it
    /// retained, which retire through their own use points) is dropped, the
    /// native command buffer is neither freed nor retired and stays owned by
    /// the pool's allocation. Returns the handle with the completion set of
    /// every submission of this command; re-wrap it with
    /// [`VulkanCommandPool::adopt`] after that set completed.
    pub(crate) fn detach(mut self) -> (vk::CommandBuffer, crate::sync::SyncSet) {
        self.detached = true;
        (self.handle, self.last_use)
    }
}
impl Drop for VulkanCommandBuffer {
    fn drop(&mut self) {
        {
            let mut sync = self.pool.allocation.lock.lock();
            if self.recording {
                self.pool.allocation.end_recording(&mut sync);
            }
            self.pool.live_commands.fetch_sub(1, Ordering::Relaxed);
        }
        // Retained objects live until the command's completion, not until this
        // wrapper is dropped.
        // Objects that track completion through their own use cells retire
        // themselves; only the others move into the graveyard.
        if !self.last_use.is_empty()
            && self
                .owners
                .as_slice()
                .iter()
                .any(|owner| owner.held_until_completion())
        {
            self.owners.retain(|owner| owner.held_until_completion());
            let owners = std::mem::replace(
                &mut self.owners,
                super::temp_list::TempList::new(1, self.pool.device.memory.stats.clone())
                    .expect("nonzero limit"),
            );
            self.pool
                .device
                .retire(self.last_use, super::graveyard::Doomed::Owners(owners));
        }
        if self.detached {
            return;
        }
        self.pool.device.retire(
            self.last_use,
            super::graveyard::Doomed::CommandBuffer(self.handle, self.pool.allocation.clone()),
        );
    }
}

/// A submission receipt. Dropping it transfers command and resource destruction
/// to the device timeline; it never blocks on a fence or device idle.
pub struct PendingCommand {
    command: VulkanCommandBuffer,
    pub point: crate::sync::SyncPoint,
    _semaphores: super::submit::SubmitScratch<[Arc<super::semaphore::TimelineSemaphore>; 8]>,
}
impl PendingCommand {
    pub fn wait(&self, timeout: u64) -> crate::Result<()> {
        self.command
            .pool
            .device
            .queues
            .timelines
            .wait(&crate::sync::SyncSet::from_point(self.point), timeout)
    }
    pub fn is_complete(&self) -> crate::Result<bool> {
        self.command
            .pool
            .device
            .queues
            .timelines
            .is_reached(&crate::sync::SyncSet::from_point(self.point))
    }
    /// The submitted command, for recycling after `point` completes.
    pub(crate) fn into_command(self) -> VulkanCommandBuffer {
        self.command
    }
}

#[cfg(test)]
mod native_state_submission_tests {
    use super::*;
    use crate::resource::{
        OwnershipRange, SyncCell,
        range_state::{AccessScope, NativeState, PatchAction, RangeOwner, StatePatch},
    };
    use crate::sync::{QueueId, SyncPoint, SyncSet};

    fn queue() -> QueueId {
        QueueId::new(0).unwrap()
    }
    fn range(offset: u64, size: u64) -> OwnershipRange {
        OwnershipRange::Buffer { offset, size }
    }
    fn cell(stats: &Arc<super::super::allocator::AllocatorStats>) -> Arc<SyncCell> {
        let cell = Arc::new(
            SyncCell::for_device(crate::device::DeviceGeneration::next())
                .with_ownership_range(range(0, 64)),
        );
        cell.with_range_state(NativeState::default(), stats, |_| Ok(()))
            .unwrap();
        cell
    }
    fn access(range: OwnershipRange, revision: u64, family: u32) -> StatePatch {
        StatePatch {
            range,
            expected_revision: revision,
            action: PatchAction::Access {
                scope: AccessScope {
                    stages: vk::PipelineStageFlags::TRANSFER,
                    access: vk::AccessFlags::TRANSFER_WRITE,
                    write: true,
                },
                layout: vk::ImageLayout::UNDEFINED,
                queue: queue(),
                family,
            },
        }
    }
    fn guard(
        cells: &[Arc<SyncCell>],
        stats: &Arc<super::super::allocator::AllocatorStats>,
    ) -> PreparedResourceStates {
        PreparedResourceStates {
            cells: super::super::submit::SubmitScratch::try_collect(
                cells.len(),
                stats,
                cells.iter().cloned().map(Ok),
            )
            .unwrap(),
            queue: queue(),
            dependencies: crate::sync::StagedDependencies::EMPTY,
            published: false,
        }
    }
    #[test]
    fn failed_native_submission_cancels_staged_state_and_retry_publishes_once() {
        let stats = Arc::new(super::super::allocator::AllocatorStats::default());
        let cell = cell(&stats);
        let cells = [cell.clone()];
        {
            let _gate = cell.lock_submission();
            let mut prepared = guard(&cells, &stats);
            prepared
                .prepare_cell(0, &[access(range(0, 32), 0, 2)], 2, &stats)
                .unwrap();
            // Queue failure or a pre-submit error drops this actual production guard.
        }
        cell.with_range_state(NativeState::default(), &stats, |table| {
            assert_eq!(table.cells().len(), 1);
            assert_eq!(table.cells()[0].state.revision, 0);
            assert_eq!(table.cells()[0].state.owner, RangeOwner::Unclaimed);
            Ok(())
        })
        .unwrap();
        assert!(cell.snapshot().is_empty());
        let point = SyncPoint {
            queue: queue(),
            value: 7,
        };
        {
            let _gate = cell.lock_submission();
            let mut prepared = guard(&cells, &stats);
            prepared
                .prepare_cell(0, &[access(range(0, 32), 0, 2)], 2, &stats)
                .unwrap();
            prepared.publish(point);
            record_command_uses(&cells, point);
        }
        cell.with_range_state(NativeState::default(), &stats, |table| {
            let written = table
                .cells()
                .iter()
                .find(|cell| cell.range == range(0, 32))
                .unwrap();
            assert_eq!(written.state.revision, 1);
            assert_eq!(written.state.owner, RangeOwner::Owned(2));
            assert_eq!(written.state.writer.points, SyncSet::from_point(point));
            assert!(
                table
                    .cells()
                    .iter()
                    .find(|cell| cell.range == range(32, 32))
                    .unwrap()
                    .state
                    .writer
                    .points
                    .is_empty()
            );
            Ok(())
        })
        .unwrap();
        assert_eq!(cell.snapshot(), SyncSet::from_point(point));
        assert!(cell.validate_queue_range(3, range(0, 32)).is_err());
        assert!(cell.validate_queue_range(3, range(32, 32)).is_ok());
        let compute = QueueId::new(1).unwrap();
        {
            let _gate = cell.lock_submission();
            let mut prepared = guard(&cells, &stats);
            prepared.queue = compute;
            let mut patch = access(range(32, 32), 0, 3);
            if let PatchAction::Access { queue, .. } = &mut patch.action {
                *queue = compute;
            }
            prepared.prepare_cell(0, &[patch], 3, &stats).unwrap();
            let point = SyncPoint {
                queue: compute,
                value: 5,
            };
            prepared.publish(point);
            record_command_uses(&cells, point);
        }
        assert!(cell.validate_queue_range(2, range(0, 32)).is_ok());
        assert!(cell.validate_queue_range(3, range(32, 32)).is_ok());
        assert!(cell.validate_queue_range(2, range(32, 32)).is_err());
        assert!(cell.validate_queue_range(2, range(0, 64)).is_err());
    }
    #[test]
    fn public_opaque_registration_waits_for_prepared_native_publication() {
        use std::{sync::mpsc, time::Duration};
        let stats = Arc::new(super::super::allocator::AllocatorStats::default());
        let cell = cell(&stats);
        let (prepared_tx, prepared_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let native_cell = cell.clone();
        let native = std::thread::spawn(move || {
            let cells = [native_cell.clone()];
            let _gate = native_cell.lock_submission();
            let mut prepared = guard(&cells, &stats);
            prepared
                .prepare_cell(0, &[access(range(0, 64), 0, 2)], 2, &stats)
                .unwrap();
            prepared_tx.send(()).unwrap();
            release_rx.recv_timeout(Duration::from_secs(5)).unwrap();
            let point = SyncPoint {
                queue: queue(),
                value: 3,
            };
            prepared.publish(point);
            record_command_uses(&cells, point);
        });
        prepared_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        let opaque_cell = cell.clone();
        let (started_tx, started_rx) = mpsc::channel();
        let (published_tx, published_rx) = mpsc::channel();
        let opaque = std::thread::spawn(move || {
            started_tx.send(()).unwrap();
            opaque_cell.insert(SyncPoint {
                queue: QueueId::new(1).unwrap(),
                value: 9,
            });
            published_tx.send(()).unwrap();
        });
        started_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        let blocked = published_rx
            .recv_timeout(Duration::from_millis(20))
            .is_err();
        // Release before asserting, so even a failed concurrency invariant
        // cannot strand a worker holding the production resource gate.
        release_tx.send(()).unwrap();
        native.join().unwrap();
        opaque.join().unwrap();
        assert!(
            blocked,
            "opaque registration must not cancel a prepared submit"
        );
        assert_eq!(cell.snapshot().get(queue()), 3);
        assert_eq!(cell.snapshot().get(QueueId::new(1).unwrap()), 9);
        cell.with_existing_range_state(|table| {
            let dependencies = table.cells()[0].state.dependencies(true);
            assert_eq!(dependencies.get(queue()), 3);
            assert_eq!(dependencies.get(QueueId::new(1).unwrap()), 9);
            Ok(())
        })
        .unwrap();
    }
    #[test]
    fn recorded_preview_failure_cancels_every_unpublished_split() {
        let stats = Arc::new(super::super::allocator::AllocatorStats::default());
        let cell = cell(&stats);
        let error = cell
            .preview_range_state::<()>(
                NativeState::default(),
                &stats,
                &[access(range(0, 32), 0, 2), access(range(32, 32), 8, 2)],
                queue(),
                2,
                |_| panic!("a failed preview must not expose its partial program"),
            )
            .unwrap_err();
        assert!(matches!(error, crate::Error::Timeout));
        cell.with_range_state(NativeState::default(), &stats, |table| {
            assert_eq!(table.cells().len(), 1);
            assert_eq!(table.cells()[0].state.revision, 0);
            assert_eq!(table.prepared_cells().count(), 0);
            Ok(())
        })
        .unwrap();
        assert!(cell.snapshot().is_empty());
    }
    #[test]
    fn stale_second_resource_aborts_all_prepared_resources_without_advancing_completion() {
        let stats = Arc::new(super::super::allocator::AllocatorStats::default());
        let first = cell(&stats);
        let second = cell(&stats);
        let cells = [first.clone(), second.clone()];
        {
            let _a = first.lock_submission();
            let _b = second.lock_submission();
            let mut prepared = guard(&cells, &stats);
            prepared
                .prepare_cell(0, &[access(range(0, 64), 0, 2)], 2, &stats)
                .unwrap();
            assert!(matches!(
                prepared.prepare_cell(1, &[access(range(0, 64), 9, 2)], 2, &stats),
                Err(crate::Error::Timeout)
            ));
        }
        for cell in &cells {
            cell.with_range_state(NativeState::default(), &stats, |table| {
                assert_eq!(table.cells()[0].state.revision, 0);
                assert!(table.cells()[0].state.writer.points.is_empty());
                assert_eq!(table.prepared_cells().count(), 0);
                Ok(())
            })
            .unwrap();
            assert!(cell.snapshot().is_empty());
        }
    }
    #[test]
    fn incremental_append_failure_cancels_all_cells_and_retry_publishes_once() {
        let stats = Arc::new(super::super::allocator::AllocatorStats::default());
        let first = cell(&stats);
        let second = cell(&stats);
        let cells = [first.clone(), second.clone()];
        {
            let _a = first.lock_submission();
            let _b = second.lock_submission();
            let mut prepared = guard(&cells, &stats);
            prepared.prepare_cell(0, &[], 2, &stats).unwrap();
            first
                .append_prepared_range_state(&[access(range(0, 32), 0, 2)], queue(), 2)
                .unwrap();
            prepared.prepare_cell(1, &[], 2, &stats).unwrap();
            assert!(matches!(
                second.append_prepared_range_state(&[access(range(0, 32), 9, 2)], queue(), 2),
                Err(crate::Error::Timeout)
            ));
        }
        for cell in &cells {
            cell.with_range_state(NativeState::default(), &stats, |table| {
                assert_eq!(table.cells()[0].state.revision, 0);
                assert_eq!(table.prepared_cells().count(), 0);
                Ok(())
            })
            .unwrap();
            assert!(cell.snapshot().is_empty());
        }
        {
            let _gate = first.lock_submission();
            let mut prepared = guard(&[first.clone()], &stats);
            prepared.prepare_cell(0, &[], 2, &stats).unwrap();
            first
                .append_prepared_range_state(&[access(range(0, 32), 0, 2)], queue(), 2)
                .unwrap();
            first
                .append_prepared_range_state(&[access(range(0, 32), 1, 2)], queue(), 2)
                .unwrap();
            prepared
                .dependencies
                .merge(&first.prepared_range_staged_dependencies());
            let point = SyncPoint {
                queue: queue(),
                value: 12,
            };
            prepared.publish(point);
            record_command_uses(&[first.clone()], point);
        }
        first
            .with_range_state(NativeState::default(), &stats, |table| {
                let changed = table
                    .cells()
                    .iter()
                    .find(|cell| cell.range == range(0, 32))
                    .unwrap();
                assert_eq!(changed.state.revision, 2);
                assert_eq!(changed.state.writer.points.get(queue()), 12);
                assert_eq!(table.prepared_cells().count(), 0);
                Ok(())
            })
            .unwrap();
    }
}

#[cfg(test)]
mod merge_read_tests {
    use super::*;
    use crate::resource::range_state::AccessScope;
    fn scope(write: bool) -> AccessScope {
        AccessScope {
            stages: vk::PipelineStageFlags::VERTEX_SHADER,
            access: vk::AccessFlags::SHADER_READ,
            write,
        }
    }
    fn merge(spans: &[(u64, u64, AccessScope)]) -> Option<Vec<(u64, u64)>> {
        merge_read_spans(spans.iter().copied().collect())
            .map(|merged| merged.iter().map(|span| (span.0, span.1)).collect())
    }
    #[test]
    fn duplicates_overlaps_and_touching_spans_union() {
        let s = scope(false);
        assert_eq!(
            merge(&[(256, 512, s), (0, 256, s), (256, 512, s), (400, 700, s), (900, 950, s)]),
            Some(vec![(0, 700), (900, 950)])
        );
        assert_eq!(merge(&[]), Some(vec![]));
    }
    #[test]
    fn different_scopes_may_touch_but_not_overlap() {
        let (a, b) = (scope(false), scope(true));
        assert_eq!(merge(&[(0, 10, a), (10, 20, b)]), Some(vec![(0, 10), (10, 20)]));
        assert_eq!(merge(&[(0, 10, a), (5, 20, b)]), None);
    }
}
