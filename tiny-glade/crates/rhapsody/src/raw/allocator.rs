//! Device memory blocks (dedicated or suballocated) and device/host memory
//! accounting.
use super::{
    device::Device,
    error::{Error, Result},
};
use crate::device::BudgetCategory;
use crate::sync::{CompletedValues, MAX_QUEUES, QueueId, SyncPoint};
use ash::vk;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{
        Arc,
        atomic::{AtomicU8, AtomicU64, AtomicUsize, Ordering},
    },
};
pub fn choose_memory_type(
    memory: &vk::PhysicalDeviceMemoryProperties,
    bits: u32,
    required: vk::MemoryPropertyFlags,
    preferred: vk::MemoryPropertyFlags,
) -> Option<u32> {
    let candidates = (0..memory.memory_type_count).filter(|&i| {
        bits & (1 << i) != 0
            && memory.memory_types[i as usize]
                .property_flags
                .contains(required)
    });
    candidates
        .clone()
        .find(|&i| {
            memory.memory_types[i as usize]
                .property_flags
                .contains(preferred)
        })
        .or_else(|| candidates.into_iter().next())
}
pub struct MemoryBlock {
    pub device: Arc<Device>,
    pub handle: vk::DeviceMemory,
    /// 在 `handle` 所指内存对象内的偏移；专用分配恒为 0，子分配时绑定资源必须带上它。
    pub offset: u64,
    pub size: u64,
    pub properties: vk::MemoryPropertyFlags,
    pub(crate) accounting_id: u64,
    pub(crate) shared: Option<Arc<super::transient_backing::SharedBacking>>,
    /// 子分配时持有 gpu-allocator 的分配；专用分配为 None。
    #[cfg(feature = "suballocation")]
    pub(crate) sub: Option<super::suballoc::SubAllocation>,
}
impl MemoryBlock {
    /// Memory for an `OPTIMAL`-tiling image. Use [`Self::allocate_image`] for
    /// other tilings.
    pub fn allocate(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        Self::allocate_image(
            device,
            requirements,
            required,
            preferred,
            vk::ImageTiling::OPTIMAL,
        )
    }
    /// Image memory. `LINEAR` images are linear resources for
    /// `bufferImageGranularity` placement next to buffers.
    pub fn allocate_image(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        tiling: vk::ImageTiling,
    ) -> Result<Self> {
        let flags = vk::MemoryAllocateFlags::empty();
        #[cfg(feature = "suballocation")]
        if device.suballocation_enabled() {
            let linear = tiling == vk::ImageTiling::LINEAR;
            return Self::allocate_sub(device, requirements, required, preferred, flags, linear);
        }
        let _ = tiling;
        Self::allocate_dedicated(device, requirements, required, preferred, flags)
    }

    /// Buffer (linear resource) memory; suballocated when the device enables it.
    pub fn allocate_with_flags(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
    ) -> Result<Self> {
        #[cfg(feature = "suballocation")]
        if device.suballocation_enabled() {
            return Self::allocate_sub(device, requirements, required, preferred, flags, true);
        }
        Self::allocate_dedicated(device, requirements, required, preferred, flags)
    }

    /// One dedicated `VkDeviceMemory` object per call, bypassing the suballocator.
    pub fn allocate_dedicated(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
    ) -> Result<Self> {
        Self::allocate_for_resource(
            device,
            requirements,
            required,
            preferred,
            flags,
            vk::Buffer::null(),
            vk::Image::null(),
        )
    }

    /// Required dedicated allocations name the one resource they may bind.
    /// Under memory pressure registered caches release idle entries first;
    /// an out-of-memory result evicts again and retries once.
    pub(crate) fn allocate_for_resource(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
        buffer: vk::Buffer,
        image: vk::Image,
    ) -> Result<Self> {
        let memory_type = choose_memory_type(
            &device.core.memory_properties,
            requirements.memory_type_bits,
            required,
            preferred,
        );
        relieve_pressure(&device, memory_type, requirements.size);
        with_eviction_retry(&device, requirements.size, || {
            Self::allocate_for_resource_once(
                device.clone(),
                requirements,
                required,
                preferred,
                flags,
                buffer,
                image,
            )
        })
    }
    fn allocate_for_resource_once(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
        buffer: vk::Buffer,
        image: vk::Image,
    ) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let index = choose_memory_type(
            &device.core.memory_properties,
            requirements.memory_type_bits,
            required,
            preferred,
        )
        .ok_or(Error::unsupported("没有匹配的内存类型"))?;
        let properties = device.core.memory_properties.memory_types[index as usize].property_flags;
        let mut allocation_flags = vk::MemoryAllocateFlagsInfo::default().flags(flags);
        let mut info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(index);
        if !flags.is_empty() {
            info = info.push_next(&mut allocation_flags);
        }
        let mut dedicated = vk::MemoryDedicatedAllocateInfo::default()
            .buffer(buffer)
            .image(image);
        if buffer != vk::Buffer::null() || image != vk::Image::null() {
            info = info.push_next(&mut dedicated);
        }
        if requirements.size == 0
            || (!flags.is_empty()
                && flags.contains(vk::MemoryAllocateFlags::DEVICE_ADDRESS)
                && !device.core.capabilities.buffer_device_address)
        {
            return Err(Error::invalid_usage("内存大小或设备地址分配能力无效"));
        }
        let heap = device.core.memory_properties.memory_types[index as usize].heap_index;
        let stats = &device.memory.stats;
        stats.acquire_native_object()?;
        let handle = match super::error::observe_vulkan(&device, unsafe {
            device.core.device.allocate_memory(&info, None)
        }) {
            Ok(handle) => handle,
            Err(error) => {
                stats.release_native_object();
                return Err(error);
            }
        };
        let accounting_id = match stats.register_native(requirements.size, index, heap, properties)
        {
            Ok(id) => id,
            Err(error) => {
                unsafe {
                    device.core.device.free_memory(handle, None);
                }
                stats.release_native_object();
                return Err(error);
            }
        };
        Ok(Self {
            device,
            handle,
            offset: 0,
            size: requirements.size,
            properties,
            accounting_id,
            shared: None,
            #[cfg(feature = "suballocation")]
            sub: None,
        })
    }
}
impl MemoryBlock {
    /// Detach from the device Arc and hand the memory to retirement, which
    /// frees it once the GPU no longer uses it.
    pub(crate) fn into_deferred(self) -> super::graveyard::DeferredMemory {
        let mut memory = std::mem::ManuallyDrop::new(self);
        // SAFETY: `memory` is never dropped; its Arc is read out exactly once
        // and released, while the caller still holds another device Arc.
        let device = unsafe { std::ptr::read(&memory.device) };
        if let Some(backing) = memory.shared.take() {
            backing.retire_member();
            return super::graveyard::DeferredMemory::Shared(backing);
        }
        device.memory.stats.mark_retiring(memory.accounting_id);
        #[cfg(feature = "suballocation")]
        if let Some(allocation) = memory.sub.take() {
            return super::graveyard::DeferredMemory::Sub { allocation };
        }
        super::graveyard::DeferredMemory::Dedicated {
            handle: memory.handle,
            accounting_id: memory.accounting_id,
        }
    }
    pub fn accounting_id(&self) -> u64 {
        self.accounting_id
    }
    pub(crate) fn from_shared(
        device: Arc<Device>,
        backing: Arc<super::transient_backing::SharedBacking>,
    ) -> Self {
        backing.add_member();
        Self {
            device,
            handle: backing.handle,
            offset: backing.offset,
            size: backing.size,
            properties: backing.properties,
            accounting_id: backing.accounting_id,
            shared: Some(backing),
            #[cfg(feature = "suballocation")]
            sub: None,
        }
    }
    pub(crate) fn shared_use(&self) -> Option<Arc<crate::resource::SyncCell>> {
        self.shared.as_ref().map(|backing| backing.last_use.clone())
    }
    pub(crate) fn is_shared(&self) -> bool {
        self.shared.is_some()
    }
    pub fn is_suballocated(&self) -> bool {
        #[cfg(feature = "suballocation")]
        return self.sub.is_some()
            || self
                .shared
                .as_ref()
                .is_some_and(|backing| backing.is_suballocated());
        #[cfg(not(feature = "suballocation"))]
        false
    }
    /// Persistently mapped pointer to this allocation's start for host-visible
    /// suballocations. `None` means the caller maps dedicated memory itself.
    pub fn host_ptr(&self) -> Option<*mut u8> {
        #[cfg(feature = "suballocation")]
        return self
            .sub
            .as_ref()
            .and_then(|a| a.mapped_ptr())
            .map(|p| p.as_ptr().cast::<u8>())
            .or_else(|| self.shared.as_ref().and_then(|backing| backing.host_ptr()));
        #[cfg(not(feature = "suballocation"))]
        None
    }
    /// Flush/invalidate range `(offset, size)`: the whole object for dedicated
    /// memory; the allocation's own atom-aligned range for suballocations.
    pub fn host_range(&self) -> (u64, u64) {
        #[cfg(feature = "suballocation")]
        if self
            .shared
            .as_ref()
            .is_some_and(|backing| backing.is_suballocated())
        {
            return (self.offset, self.size);
        }
        #[cfg(feature = "suballocation")]
        return match &self.sub {
            Some(allocation) => (allocation.offset(), allocation.size()),
            None => (0, vk::WHOLE_SIZE),
        };
        #[cfg(not(feature = "suballocation"))]
        (0, vk::WHOLE_SIZE)
    }
    pub(crate) fn host_subrange(
        &self,
        buffer_size: u64,
        offset: u64,
        size: u64,
        atom: u64,
    ) -> Result<HostMemoryRange> {
        let dedicated = !self.is_shared();
        #[cfg(feature = "suballocation")]
        let dedicated = dedicated && self.sub.is_none();
        mapped_host_subrange(
            self.offset,
            self.size,
            buffer_size,
            offset,
            size,
            atom,
            dedicated,
        )
    }
}

/// Whether an allocation of `bytes` from `memory_type` should first ask
/// registered caches to release idle entries.
pub(crate) fn needs_eviction(
    action: crate::device::BudgetAction,
    heap_available: Option<u64>,
    bytes: u64,
) -> bool {
    action == crate::device::BudgetAction::EvictCache
        || heap_available.is_some_and(|available| available < bytes)
}

/// Under budget pressure, or when `bytes` would exceed the heap budget, ask
/// registered caches to release idle entries before allocating.
pub(crate) fn relieve_pressure(device: &Device, memory_type: Option<u32>, bytes: u64) {
    let stats = &device.memory.stats;
    let heap = memory_type
        .map(|index| device.core.memory_properties.memory_types[index as usize].heap_index);
    if needs_eviction(
        stats.budget_action(),
        heap.map(|heap| stats.heap_available(heap)),
        bytes,
    ) {
        device.evict_idle_caches(bytes);
    }
}

/// Whether an allocation error can be relieved by releasing cached memory.
pub(crate) fn is_memory_exhaustion(error: &Error) -> bool {
    matches!(error, Error::OutOfDeviceMemory | Error::Exhausted(_))
}

/// Run `attempt`; on memory exhaustion evict idle cache entries and, when
/// anything was released, retry exactly once.
pub(crate) fn with_eviction_retry<T>(
    device: &Device,
    bytes: u64,
    mut attempt: impl FnMut() -> Result<T>,
) -> Result<T> {
    match attempt() {
        Err(error) if is_memory_exhaustion(&error) => {
            if device.evict_idle_caches(bytes) != 0 {
                attempt()
            } else {
                Err(error)
            }
        }
        result => result,
    }
}

pub(crate) struct HostMemoryRange {
    pub memory_offset: u64,
    pub memory_size: u64,
    pub buffer_offset: u64,
    pub buffer_size: u64,
}
fn mapped_host_subrange(
    base: u64,
    allocation_size: u64,
    buffer_size: u64,
    offset: u64,
    size: u64,
    atom: u64,
    dedicated: bool,
) -> Result<HostMemoryRange> {
    let invalid = || Error::invalid_usage("mapped buffer interval exceeds its allocation");
    let end = offset
        .checked_add(size)
        .filter(|end| size != 0 && *end <= buffer_size && buffer_size <= allocation_size)
        .ok_or_else(invalid)?;
    let physical_start = base.checked_add(offset).ok_or_else(invalid)?;
    let physical_end = base.checked_add(end).ok_or_else(invalid)?;
    let allocation_end = base.checked_add(allocation_size).ok_or_else(invalid)?;
    let atom = atom.max(1);
    let start = physical_start - physical_start % atom;
    let end = physical_end
        .checked_add((atom - physical_end % atom) % atom)
        .ok_or_else(invalid)?
        .min(allocation_end);
    if start < base || (!dedicated && !end.is_multiple_of(atom)) {
        return Err(Error::unsupported(
            "mapped allocation does not isolate non-coherent atoms",
        ));
    }
    Ok(HostMemoryRange {
        memory_offset: start,
        memory_size: if !((end - start).is_multiple_of(atom)) {
            vk::WHOLE_SIZE
        } else {
            end - start
        },
        buffer_offset: start - base,
        buffer_size: (end - base).min(buffer_size) - (start - base),
    })
}

impl Drop for MemoryBlock {
    fn drop(&mut self) {
        if let Some(backing) = self.shared.take() {
            backing.retire_member();
            drop(backing);
            return;
        }
        #[cfg(feature = "suballocation")]
        match self.sub.take() {
            Some(allocation) => {
                drop(allocation);
                return;
            }
            None => unsafe {
                self.device.core.device.free_memory(self.handle, None);
            },
        }
        #[cfg(not(feature = "suballocation"))]
        unsafe {
            self.device.core.device.free_memory(self.handle, None)
        };
        if let Err(error) = self.device.memory.stats.release(self.accounting_id) {
            log::error!("memory accounting release failed: {error}");
        }
    }
}

/// Requested residency: selects required/preferred memory properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryLocation {
    GpuOnly,
    CpuToGpu,
    GpuToCpu,
    Unknown,
}
impl MemoryLocation {
    pub fn properties(self) -> (vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) {
        match self {
            Self::GpuOnly => (
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            ),
            Self::CpuToGpu => (
                vk::MemoryPropertyFlags::HOST_VISIBLE,
                vk::MemoryPropertyFlags::HOST_COHERENT,
            ),
            Self::GpuToCpu => (
                vk::MemoryPropertyFlags::HOST_VISIBLE,
                vk::MemoryPropertyFlags::HOST_CACHED | vk::MemoryPropertyFlags::HOST_COHERENT,
            ),
            Self::Unknown => (
                vk::MemoryPropertyFlags::empty(),
                vk::MemoryPropertyFlags::empty(),
            ),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationTotals {
    pub total_allocated_bytes: u64,
    pub device_local_bytes: u64,
    pub system_bytes: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationRecord {
    pub id: u64,
    pub size: u64,
    pub memory_type: u32,
    pub heap: u32,
    pub properties: u32,
    pub name: String,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocatorReport {
    pub totals: AllocationTotals,
    pub allocations: Vec<AllocationRecord>,
}
/// Device memory accounting.
///
/// VkDeviceMemory ranges and unused native suballocator capacity are charged
/// to the device budget, optionally further limited per heap (explicit limits
/// or limits derived from `VK_EXT_memory_budget`). Renderer-owned host
/// metadata is charged to a separate host budget, so exhausting host metadata
/// reports `OutOfHostMemory` and never consumes device memory headroom.
///
/// GPU uses are staged in per-queue buffers without touching the global state
/// lock; completion moves only the in-flight entries whose timeline value has
/// been reached (per-queue queues ordered by value).
#[derive(Default)]
pub struct AllocatorStats {
    update_after_bind_descriptors: AtomicU64,
    /// Live VkDeviceMemory objects (dedicated allocations and native blocks).
    native_objects: AtomicU64,
    /// `maxMemoryAllocationCount`; 0 means unlimited.
    max_native_objects: AtomicU64,
    /// `(value, allocation id)` uses recorded since the last state drain.
    pending_uses: [Mutex<Vec<(u64, u64)>>; MAX_QUEUES],
    pending_count: AtomicUsize,
    /// Completion values already applied to the in-flight queues.
    observed: [AtomicU64; MAX_QUEUES],
    /// Combined device/heap/host budget action, published after each change.
    action: AtomicU8,
    host_action: AtomicU8,
    host: Mutex<HostState>,
    state: Mutex<StatisticsState>,
}
/// Known host allocations are charged to the host budget. They never become
/// VkDeviceMemory records or physical-GPU allocation totals. The ticket follows
/// the actual owner; cache removal cannot credit bytes held by an active lease.
pub(crate) struct CpuMemoryReservation {
    stats: std::sync::Arc<AllocatorStats>,
    bytes: u64,
    category: crate::device::BudgetCategory,
}
/// Known payload plus the two reference counters of a shared allocation. The
/// reservation stays in the payload, including when its Arc escapes a cache.
pub(crate) fn shared_cpu_bytes<T>() -> Result<u64> {
    let (layout, _) = std::alloc::Layout::new::<[usize; 2]>()
        .extend(std::alloc::Layout::new::<T>())
        .map_err(|_| Error::OutOfHostMemory)?;
    u64::try_from(layout.pad_to_align().size()).map_err(|_| Error::OutOfHostMemory)
}
/// Admit renderer-owned Arc payload and separately owned CPU buffers before a
/// native object is created. Keep this ticket in the actual allocation owner.
pub(crate) fn reserve_shared_cpu_metadata<T>(
    stats: &std::sync::Arc<AllocatorStats>,
    extra_bytes: u64,
) -> Result<CpuMemoryReservation> {
    let bytes = shared_cpu_bytes::<T>()?
        .checked_add(extra_bytes)
        .ok_or(Error::OutOfHostMemory)?;
    stats.reserve_cpu_bytes(bytes, crate::device::BudgetCategory::Live)
}
pub(crate) fn cpu_vec_bytes<T>(capacity: usize) -> Result<u64> {
    let layout = std::alloc::Layout::array::<T>(capacity).map_err(|_| Error::OutOfHostMemory)?;
    u64::try_from(layout.size()).map_err(|_| Error::OutOfHostMemory)
}
fn add_cpu(usage: &mut crate::device::BudgetUsage, category: BudgetCategory, bytes: u64) {
    match category {
        BudgetCategory::Live => usage.live += bytes,
        BudgetCategory::Cache => usage.cache += bytes,
        // Construction only admits live or cached host categories.
        BudgetCategory::InFlight | BudgetCategory::Retiring => {}
    }
}
fn sub_cpu(usage: &mut crate::device::BudgetUsage, category: BudgetCategory, bytes: u64) {
    match category {
        BudgetCategory::Live => usage.live -= bytes,
        BudgetCategory::Cache => usage.cache -= bytes,
        BudgetCategory::InFlight | BudgetCategory::Retiring => {}
    }
}
impl CpuMemoryReservation {
    pub(crate) fn resize(&mut self, bytes: u64) -> Result<()> {
        let category = self.category;
        let previous = self.bytes;
        self.stats.with_host(|host| {
            if bytes > previous {
                host.budget
                    .reserve(category, bytes - previous)
                    .map_err(|_| Error::OutOfHostMemory)?;
            } else {
                host.budget.release(category, previous - bytes);
            }
            sub_cpu(&mut host.usage, category, previous);
            add_cpu(&mut host.usage, category, bytes);
            host.peak = host.peak.max(host.usage.total());
            Ok::<(), Error>(())
        })?;
        self.bytes = bytes;
        Ok(())
    }
    pub(crate) fn reclassify(&mut self, category: BudgetCategory) {
        if category == self.category
            || !matches!(category, BudgetCategory::Live | BudgetCategory::Cache)
        {
            return;
        }
        let (from, bytes) = (self.category, self.bytes);
        self.stats.with_host(|host| {
            host.budget.reclassify(from, category, bytes);
            sub_cpu(&mut host.usage, from, bytes);
            add_cpu(&mut host.usage, category, bytes);
        });
        self.category = category;
    }
}
impl Drop for CpuMemoryReservation {
    fn drop(&mut self) {
        let (category, bytes) = (self.category, self.bytes);
        self.stats.with_host(|host| {
            host.budget.release(category, bytes);
            sub_cpu(&mut host.usage, category, bytes);
        });
    }
}
pub(crate) struct DescriptorReservation {
    stats: std::sync::Arc<AllocatorStats>,
    count: u64,
}
impl Drop for DescriptorReservation {
    fn drop(&mut self) {
        self.stats
            .update_after_bind_descriptors
            .fetch_sub(self.count, Ordering::AcqRel);
    }
}
#[derive(Default)]
struct HostState {
    budget: crate::device::MemoryBudgetController,
    usage: crate::device::BudgetUsage,
    peak: u64,
}
/// Vulkan's `VK_MAX_MEMORY_HEAPS`.
const MAX_HEAPS: usize = vk::MAX_MEMORY_HEAPS;
#[derive(Default)]
struct StatisticsState {
    budget: crate::device::MemoryBudgetController,
    phases: BTreeMap<u64, AllocationPhase>,
    next_id: u64,
    records: BTreeMap<u64, AllocationRecord>,
    totals: AllocationTotals,
    peak: AllocationTotals,
    peak_report: Option<AllocatorReport>,
    /// The peak was raised but its snapshot is not materialized yet. After a
    /// peak only registrations (which raise it again) or releases can follow,
    /// so materializing before the first release reproduces the exact state.
    peak_dirty: bool,
    /// Explicit per-heap limits set by the application.
    heap_budgets: [Option<u64>; MAX_HEAPS],
    /// Limits derived from `VK_EXT_memory_budget` (`None` = not reported).
    heap_limits: [Option<u64>; MAX_HEAPS],
    /// Native bytes charged per heap (ranges plus unused suballocator capacity).
    heap_used: [u64; MAX_HEAPS],
    heap_pressure: bool,
    /// Native allocator capacity not occupied by a live resource range. It is
    /// known GPU cache memory, distinct from host cache reservations.
    suballocator_spare: [u64; 32],
    /// Per-queue `(timeline value, id)` of in-flight uses, ordered by value.
    in_flight: [VecDeque<(u64, u64)>; MAX_QUEUES],
    observed: CompletedValues,
    /// Swap buffer for draining staged uses without reallocating.
    scratch: Vec<(u64, u64)>,
}
struct AllocationPhase {
    category: BudgetCategory,
    last_use: crate::sync::SyncSet,
    /// The record owns one VkDeviceMemory object (dedicated allocation).
    native: bool,
}
impl StatisticsState {
    fn report(&self) -> AllocatorReport {
        AllocatorReport {
            totals: self.totals,
            allocations: self.records.values().cloned().collect(),
        }
    }
    fn materialize_peak(&mut self) {
        if self.peak_dirty {
            self.peak_report = Some(self.report());
            self.peak_dirty = false;
        }
    }
    fn heap_limit(&self, heap: usize) -> Option<u64> {
        match (self.heap_budgets[heap], self.heap_limits[heap]) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }
    fn refresh_heap_pressure(&mut self) {
        // Same high-water fraction as the device controller (80%).
        self.heap_pressure = (0..MAX_HEAPS).any(|heap| {
            self.heap_limit(heap).is_some_and(|limit| {
                u128::from(self.heap_used[heap]) * 5 >= u128::from(limit) * 4 && limit != 0
            })
        });
    }
    fn apply_use(&mut self, queue: QueueId, value: u64, id: u64) {
        let observed = self.observed.get(queue);
        let Some(phase) = self.phases.get_mut(&id) else {
            return;
        };
        let previous = phase.last_use.get(queue);
        phase.last_use.insert(SyncPoint { queue, value });
        if value <= observed {
            return;
        }
        if value > previous {
            let fifo = &mut self.in_flight[queue.index()];
            // Submissions on one queue publish almost in value order.
            let at = fifo.partition_point(|&(queued, _)| queued <= value);
            fifo.insert(at, (value, id));
        }
        transition_phase(self, id, BudgetCategory::InFlight);
    }
    fn action(&self) -> crate::device::BudgetAction {
        use crate::device::BudgetAction;
        let device = self.budget.tick();
        if device == BudgetAction::EvictCache {
            return device;
        }
        if self.heap_pressure {
            return if self.budget.usage().cache > 0 {
                BudgetAction::EvictCache
            } else {
                BudgetAction::Backpressure
            };
        }
        device
    }
}
fn encode_action(action: crate::device::BudgetAction) -> u8 {
    match action {
        crate::device::BudgetAction::None => 0,
        crate::device::BudgetAction::EvictCache => 1,
        crate::device::BudgetAction::Backpressure => 2,
    }
}
impl AllocatorStats {
    fn with_state<R>(&self, run: impl FnOnce(&mut StatisticsState) -> R) -> R {
        let mut state = self.state.lock();
        self.drain_pending(&mut state);
        let result = run(&mut state);
        self.action
            .store(encode_action(state.action()), Ordering::Release);
        result
    }
    fn with_host<R>(&self, run: impl FnOnce(&mut HostState) -> R) -> R {
        let mut host = self.host.lock();
        let result = run(&mut host);
        self.host_action
            .store(encode_action(host.budget.tick()), Ordering::Release);
        result
    }
    fn drain_pending(&self, state: &mut StatisticsState) {
        if self.pending_count.load(Ordering::Acquire) == 0 {
            return;
        }
        for (index, pending) in self.pending_uses.iter().enumerate() {
            let Some(queue) = QueueId::new(index) else {
                continue;
            };
            let mut scratch = std::mem::take(&mut state.scratch);
            {
                let mut pending = pending.lock();
                if pending.is_empty() {
                    state.scratch = scratch;
                    continue;
                }
                std::mem::swap(&mut *pending, &mut scratch);
            }
            self.pending_count
                .fetch_sub(scratch.len(), Ordering::AcqRel);
            for &(value, id) in &scratch {
                state.apply_use(queue, value, id);
            }
            scratch.clear();
            state.scratch = scratch;
        }
    }
    pub(crate) fn reserve_cpu_bytes(
        self: &std::sync::Arc<Self>,
        bytes: u64,
        category: BudgetCategory,
    ) -> Result<CpuMemoryReservation> {
        if !matches!(category, BudgetCategory::Live | BudgetCategory::Cache) {
            return Err(Error::invalid_usage(
                "CPU allocation must be live or cached",
            ));
        }
        self.with_host(|host| {
            host.budget
                .reserve(category, bytes)
                .map_err(|_| Error::OutOfHostMemory)?;
            add_cpu(&mut host.usage, category, bytes);
            host.peak = host.peak.max(host.usage.total());
            Ok::<(), Error>(())
        })?;
        Ok(CpuMemoryReservation {
            stats: self.clone(),
            bytes,
            category,
        })
    }
    pub fn cpu_memory_usage(&self) -> crate::device::BudgetUsage {
        self.host.lock().usage
    }
    pub fn cpu_peak_bytes(&self) -> u64 {
        self.host.lock().peak
    }
    /// Host metadata limit; `0` means unbounded. Fails without change when the
    /// current host usage already exceeds `bytes`.
    pub fn set_host_budget(&self, bytes: u64) -> Result<()> {
        self.with_host(|host| {
            host.budget
                .set_limit(bytes)
                .map_err(|_| Error::OutOfHostMemory)
        })
    }
    pub fn host_budget_limit(&self) -> u64 {
        self.host.lock().budget.limit()
    }
    pub(crate) fn reserve_update_after_bind_descriptors(
        self: &std::sync::Arc<Self>,
        count: u64,
        limit: u64,
    ) -> Result<DescriptorReservation> {
        self.update_after_bind_descriptors
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(count).filter(|next| *next <= limit)
            })
            .map_err(|_| Error::OutOfDeviceMemory)?;
        Ok(DescriptorReservation {
            stats: self.clone(),
            count,
        })
    }
    /// Device memory limit across all heaps; `0` means unbounded.
    pub fn set_device_budget(&self, bytes: u64) -> Result<()> {
        self.with_state(|state| {
            state
                .budget
                .set_limit(bytes)
                .map_err(|_| Error::OutOfDeviceMemory)
        })
    }
    /// Set both the device memory limit and the host metadata limit to
    /// `bytes` (each is enforced independently). Nothing changes on failure.
    pub fn set_total_budget(&self, bytes: u64) -> Result<()> {
        let mut state = self.state.lock();
        let mut host = self.host.lock();
        let mut device_budget = state.budget;
        device_budget
            .set_limit(bytes)
            .map_err(|_| Error::OutOfDeviceMemory)?;
        let mut host_budget = host.budget;
        host_budget
            .set_limit(bytes)
            .map_err(|_| Error::OutOfHostMemory)?;
        state.budget = device_budget;
        host.budget = host_budget;
        self.action
            .store(encode_action(state.action()), Ordering::Release);
        self.host_action
            .store(encode_action(host.budget.tick()), Ordering::Release);
        Ok(())
    }
    /// Device memory usage by phase (host metadata excluded).
    pub fn device_memory_usage(&self) -> crate::device::BudgetUsage {
        self.with_state(|state| state.budget.usage())
    }
    /// Combined report: device memory by phase plus host metadata
    /// ([`Self::cpu_memory_usage`]) in its live/cache categories. Admission
    /// still applies the device and host limits independently.
    pub fn budget_usage(&self) -> crate::device::BudgetUsage {
        let mut usage = self.device_memory_usage();
        let host = self.cpu_memory_usage();
        usage.live = usage.live.saturating_add(host.live);
        usage.cache = usage.cache.saturating_add(host.cache);
        usage
    }
    /// Bytes still admissible on `heap` under the device and heap limits.
    pub(crate) fn heap_available(&self, heap: u32) -> u64 {
        let state = self.state.lock();
        let device = if state.budget.limit() == 0 {
            u64::MAX
        } else {
            state.budget.available()
        };
        let heap = heap as usize;
        let heap = match (heap < MAX_HEAPS).then(|| state.heap_limit(heap)).flatten() {
            Some(limit) => limit.saturating_sub(state.heap_used[heap]),
            None => u64::MAX,
        };
        device.min(heap)
    }
    pub fn suballocator_spare_bytes(&self) -> u64 {
        self.state.lock().suballocator_spare.iter().sum()
    }
    /// Lock-free pressure recommendation combining the device budget, heap
    /// budgets and the host metadata budget.
    pub fn budget_action(&self) -> crate::device::BudgetAction {
        use crate::device::BudgetAction;
        let decode = |value| match value {
            1 => BudgetAction::EvictCache,
            2 => BudgetAction::Backpressure,
            _ => BudgetAction::None,
        };
        let device = decode(self.action.load(Ordering::Acquire));
        let host = decode(self.host_action.load(Ordering::Acquire));
        if device == BudgetAction::EvictCache || host == BudgetAction::EvictCache {
            BudgetAction::EvictCache
        } else if device == BudgetAction::Backpressure || host == BudgetAction::Backpressure {
            BudgetAction::Backpressure
        } else {
            BudgetAction::None
        }
    }
    /// Stage a GPU use without the global statistics lock.
    pub(crate) fn record_gpu_use(&self, id: u64, point: SyncPoint) {
        {
            let mut pending = self.pending_uses[point.queue.index()].lock();
            if pending.try_reserve(1).is_ok() {
                pending.push((point.value, id));
                self.pending_count.fetch_add(1, Ordering::AcqRel);
                return;
            }
        }
        // No staging capacity: apply directly under the state lock.
        self.with_state(|state| state.apply_use(point.queue, point.value, id));
    }
    pub(crate) fn mark_retiring(&self, id: u64) {
        self.with_state(|state| transition_phase(state, id, BudgetCategory::Retiring));
    }
    pub(crate) fn mark_cached(&self, id: u64, completed: &CompletedValues) {
        self.with_state(|state| {
            if state
                .phases
                .get(&id)
                .is_some_and(|phase| phase.last_use.is_reached(completed))
            {
                transition_phase(state, id, BudgetCategory::Cache);
            }
        });
    }
    /// Reusing completed cached storage changes its category, not its real GPU
    /// history. A retiring allocation stays terminal in transition_phase.
    pub(crate) fn mark_live(&self, id: u64, completed: &CompletedValues) {
        self.with_state(|state| {
            if state
                .phases
                .get(&id)
                .is_some_and(|phase| phase.last_use.is_reached(completed))
            {
                transition_phase(state, id, BudgetCategory::Live);
            }
        });
    }
    /// Move in-flight allocations whose uses completed back to `Live`. Work is
    /// proportional to the reached entries; an unchanged snapshot returns
    /// without taking the statistics lock.
    pub(crate) fn observe_completed(&self, completed: &CompletedValues) {
        let advanced = self.observed.iter().enumerate().any(|(index, seen)| {
            QueueId::new(index)
                .is_some_and(|queue| completed.get(queue) > seen.load(Ordering::Acquire))
        });
        if !advanced {
            return;
        }
        self.with_state(|state| {
            for index in 0..MAX_QUEUES {
                let Some(queue) = QueueId::new(index) else {
                    continue;
                };
                let done = completed.get(queue);
                if done <= state.observed.get(queue) {
                    continue;
                }
                state.observed.raise(queue, done);
                while let Some(&(value, id)) = state.in_flight[index].front() {
                    if value > done {
                        break;
                    }
                    state.in_flight[index].pop_front();
                    let StatisticsState {
                        budget,
                        phases,
                        records,
                        observed,
                        ..
                    } = &mut *state;
                    if let Some(phase) = phases.get_mut(&id)
                        && phase.category == BudgetCategory::InFlight
                        && phase.last_use.is_reached(observed)
                        && let Some(record) = records.get(&id)
                    {
                        budget.reclassify(
                            BudgetCategory::InFlight,
                            BudgetCategory::Live,
                            record.size,
                        );
                        phase.category = BudgetCategory::Live;
                    }
                }
                self.observed[index].fetch_max(done, Ordering::AcqRel);
            }
        });
    }
    pub fn report(&self) -> AllocatorReport {
        self.state.lock().report()
    }
    /// Current totals without copying every allocation record.
    pub fn totals(&self) -> AllocationTotals {
        self.state.lock().totals
    }
    pub fn peak_allocator_report(&self) -> Option<AllocatorReport> {
        let mut state = self.state.lock();
        state.materialize_peak();
        state.peak_report.clone()
    }
    /// Cumulative owned allocation ranges. A suballocator can place many ranges
    /// in one VkDeviceMemory; native block counts are reported separately.
    pub fn total_registered(&self) -> u64 {
        self.state.lock().next_id
    }
    pub fn peak_totals(&self) -> AllocationTotals {
        self.state.lock().peak
    }
    /// Explicit per-heap limit. Fails without change when the heap already
    /// holds more than `bytes`.
    pub fn set_heap_budget(&self, heap: u32, bytes: u64) -> Result<()> {
        let index = heap_index(heap)?;
        self.with_state(|state| {
            if state.heap_used[index] > bytes {
                return Err(Error::invalid_usage(
                    "heap budget is below its current usage",
                ));
            }
            state.heap_budgets[index] = Some(bytes);
            state.refresh_heap_pressure();
            Ok(())
        })
    }
    /// Apply a `VK_EXT_memory_budget` snapshot. Each heap's limit becomes the
    /// driver budget minus memory this tracker did not allocate. A limit below
    /// current usage is accepted: new allocations fail and pressure is raised.
    pub(crate) fn apply_driver_heap_budgets(&self, heaps: &[(u64, u64)]) {
        self.with_state(|state| {
            for (index, &(budget, usage)) in heaps.iter().enumerate().take(MAX_HEAPS) {
                let foreign = usage.saturating_sub(state.heap_used[index]);
                state.heap_limits[index] = Some(budget.saturating_sub(foreign));
            }
            state.refresh_heap_pressure();
        });
    }
    pub fn heap_usage(&self, heap: u32) -> Result<(u64, Option<u64>)> {
        let index = heap_index(heap)?;
        let state = self.state.lock();
        Ok((state.heap_used[index], state.heap_limit(index)))
    }
    pub(crate) fn set_max_native_objects(&self, count: u64) {
        self.max_native_objects.store(count, Ordering::Release);
    }
    pub fn native_object_count(&self) -> u64 {
        self.native_objects.load(Ordering::Acquire)
    }
    /// Reserve one VkDeviceMemory object under `maxMemoryAllocationCount`.
    pub(crate) fn acquire_native_object(&self) -> Result<()> {
        let max = self.max_native_objects.load(Ordering::Acquire);
        self.native_objects
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                (max == 0 || count < max).then_some(count + 1)
            })
            .map(|_| ())
            .map_err(|_| Error::exhausted("maxMemoryAllocationCount reached"))
    }
    pub(crate) fn release_native_object(&self) {
        let _ = self
            .native_objects
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                count.checked_sub(1)
            });
    }
    pub fn register(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: vk::MemoryPropertyFlags,
        name: String,
    ) -> Result<u64> {
        self.register_with_spare(size, memory_type, heap, properties, name, None, false)
    }
    /// Register a dedicated VkDeviceMemory object whose native slot was taken
    /// by [`Self::acquire_native_object`]; releasing the record frees it.
    pub(crate) fn register_native(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: vk::MemoryPropertyFlags,
    ) -> Result<u64> {
        self.register_with_spare(
            size,
            memory_type,
            heap,
            properties,
            String::new(),
            None,
            true,
        )
    }
    #[cfg(any(feature = "suballocation", test))]
    pub(crate) fn register_suballocation(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: vk::MemoryPropertyFlags,
        spare: u64,
    ) -> Result<u64> {
        self.register_with_spare(
            size,
            memory_type,
            heap,
            properties,
            String::new(),
            Some(spare),
            false,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn register_with_spare(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: vk::MemoryPropertyFlags,
        name: String,
        spare: Option<u64>,
        native: bool,
    ) -> Result<u64> {
        if size == 0 {
            return Err(Error::invalid_usage(
                "cannot register a zero-sized allocation",
            ));
        }
        let heap_slot = heap_index(heap)?;
        let type_slot = usize::try_from(memory_type)
            .ok()
            .filter(|index| *index < 32)
            .ok_or_else(|| Error::invalid_usage("memory type index out of range"))?;
        let overflow = || Error::invalid_usage("memory statistics overflow");
        self.with_state(|state| {
            let total = state
                .totals
                .total_allocated_bytes
                .checked_add(size)
                .ok_or_else(overflow)?;
            let old_spare = spare.map_or(0, |_| state.suballocator_spare[type_slot]);
            let new_spare = spare.unwrap_or(0);
            let used = state.heap_used[heap_slot]
                .checked_sub(old_spare)
                .and_then(|used| used.checked_add(new_spare))
                .and_then(|used| used.checked_add(size))
                .ok_or_else(overflow)?;
            if state
                .heap_limit(heap_slot)
                .is_some_and(|limit| used > limit)
            {
                return Err(Error::OutOfDeviceMemory);
            }
            let id = state.next_id.checked_add(1).ok_or_else(overflow)?;
            let mut totals = state.totals;
            totals.total_allocated_bytes = total;
            if properties.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL) {
                totals.device_local_bytes = totals
                    .device_local_bytes
                    .checked_add(size)
                    .ok_or_else(overflow)?;
            } else {
                totals.system_bytes = totals.system_bytes.checked_add(size).ok_or_else(overflow)?;
            }
            let mut budget = state.budget;
            budget.release(BudgetCategory::Cache, old_spare);
            budget
                .reserve(BudgetCategory::Cache, new_spare)
                .map_err(|_| Error::OutOfDeviceMemory)?;
            budget
                .reserve(BudgetCategory::Live, size)
                .map_err(|_| Error::OutOfDeviceMemory)?;
            state.budget = budget;
            if spare.is_some() {
                state.suballocator_spare[type_slot] = new_spare;
            }
            state.next_id = id;
            state.phases.insert(
                id,
                AllocationPhase {
                    category: BudgetCategory::Live,
                    last_use: crate::sync::SyncSet::EMPTY,
                    native,
                },
            );
            state.records.insert(
                id,
                AllocationRecord {
                    id,
                    size,
                    memory_type,
                    heap,
                    properties: properties.as_raw(),
                    name,
                },
            );
            state.totals = totals;
            state.heap_used[heap_slot] = used;
            state.refresh_heap_pressure();
            state.peak.device_local_bytes =
                state.peak.device_local_bytes.max(totals.device_local_bytes);
            state.peak.system_bytes = state.peak.system_bytes.max(totals.system_bytes);
            if total > state.peak.total_allocated_bytes {
                state.peak.total_allocated_bytes = total;
                state.peak_dirty = true;
            }
            Ok(id)
        })
    }
    pub fn release(&self, id: u64) -> Result<()> {
        self.release_with_spare(id, None)
    }
    #[cfg(any(feature = "suballocation", test))]
    pub(crate) fn release_suballocation(&self, id: u64, spare: u64) -> Result<()> {
        self.release_with_spare(id, Some(spare))
    }
    fn release_with_spare(&self, id: u64, spare: Option<u64>) -> Result<()> {
        let native = self.with_state(|state| {
            if !state.records.contains_key(&id) {
                return Err(Error::invalid_usage(
                    "unknown or already released memory id",
                ));
            }
            // Releasing changes state: materialize a just-reached peak first.
            state.materialize_peak();
            let record = state
                .records
                .remove(&id)
                .ok_or_else(|| Error::invalid_usage("unknown or already released memory id"))?;
            let mut native = false;
            if let Some(phase) = state.phases.remove(&id) {
                state.budget.release(phase.category, record.size);
                native = phase.native;
            }
            if let Some(spare) = spare {
                replace_spare(state, record.memory_type, record.heap, spare);
            }
            if let Some(used) = state.heap_used.get_mut(record.heap as usize) {
                *used = used.saturating_sub(record.size);
            }
            state.refresh_heap_pressure();
            state.totals.total_allocated_bytes -= record.size;
            if record.properties & vk::MemoryPropertyFlags::DEVICE_LOCAL.as_raw() != 0 {
                state.totals.device_local_bytes -= record.size;
            } else {
                state.totals.system_bytes -= record.size;
            }
            Ok(native)
        })?;
        if native {
            self.release_native_object();
        }
        Ok(())
    }
    /// Only unused native blocks may be trimmed. No live range changes here.
    #[cfg(any(feature = "suballocation", test))]
    pub(crate) fn trim_suballocator_spare(&self, memory_type: u32, heap: u32) {
        self.with_state(|state| {
            replace_spare(state, memory_type, heap, 0);
            state.refresh_heap_pressure();
        });
    }
    pub(crate) fn rename(&self, id: u64, name: String) {
        let mut state = self.state.lock();
        if let Some(record) = state.records.get_mut(&id) {
            record.name = name.clone();
        }
        // Keep a materialized peak snapshot consistent with the new name.
        if let Some(report) = &mut state.peak_report
            && let Ok(index) = report
                .allocations
                .binary_search_by_key(&id, |record| record.id)
        {
            report.allocations[index].name = name;
        }
    }
}

fn heap_index(heap: u32) -> Result<usize> {
    usize::try_from(heap)
        .ok()
        .filter(|index| *index < MAX_HEAPS)
        .ok_or_else(|| Error::invalid_usage("memory heap index out of range"))
}

fn replace_spare(state: &mut StatisticsState, memory_type: u32, heap: u32, spare: u64) {
    let Some(slot) = state.suballocator_spare.get_mut(memory_type as usize) else {
        return;
    };
    let previous = std::mem::replace(slot, spare);
    state.budget.release(BudgetCategory::Cache, previous);
    // Freeing or trimming native memory never increases the admitted total,
    // so this re-admission cannot exceed a limit that admitted `previous`.
    // A limit lowered in between is tolerated: the charge is a fact.
    if state.budget.reserve(BudgetCategory::Cache, spare).is_err() {
        state.budget.record_cache(spare);
    }
    if let Some(used) = state.heap_used.get_mut(heap as usize) {
        *used = used.saturating_sub(previous).saturating_add(spare);
    }
}
fn transition_phase(state: &mut StatisticsState, id: u64, category: BudgetCategory) {
    let Some(phase) = state.phases.get_mut(&id) else {
        return;
    };
    if phase.category == category {
        return;
    }
    // Retiring is terminal: accounting can never make an unowned allocation
    // eligible for a cache just because another completion snapshot arrived.
    if phase.category == BudgetCategory::Retiring {
        return;
    }
    let Some(record) = state.records.get(&id) else {
        return;
    };
    state
        .budget
        .reclassify(phase.category, category, record.size);
    phase.category = category;
}

pub struct AllocationRequest {
    pub name: String,
    pub requirements: vk::MemoryRequirements,
    pub location: MemoryLocation,
    pub flags: vk::MemoryAllocateFlags,
}
/// Allocation backend boundary (dedicated or gpu-allocator suballocation).
pub trait AllocationBackend {
    type Allocation;
    fn allocate(&mut self, request: &AllocationRequest) -> Result<Self::Allocation>;
    fn free(&mut self, allocation: Self::Allocation) -> Result<()>;
    fn report(&self) -> AllocatorReport;
}
/// Peak-tracking wrapper: samples the backend report after each allocation.
pub struct GpuAllocator<B: AllocationBackend> {
    backend: B,
    track_peaks: bool,
    peaks: AllocationTotals,
    peak_report: Option<AllocatorReport>,
}
impl<B: AllocationBackend> GpuAllocator<B> {
    pub fn new(backend: B, track_peaks: bool) -> Self {
        Self {
            backend,
            track_peaks,
            peaks: AllocationTotals::default(),
            peak_report: None,
        }
    }
    pub fn inner(&self) -> &B {
        &self.backend
    }
    pub fn inner_mut(&mut self) -> &mut B {
        &mut self.backend
    }
    pub fn allocate(&mut self, request: &AllocationRequest) -> Result<B::Allocation> {
        let result = self.backend.allocate(request);
        if self.track_peaks {
            self.observe_report(self.backend.report());
        }
        result
    }
    /// Keep the full report at the strictly highest total; device/system
    /// peaks are tracked independently.
    pub fn observe_report(&mut self, report: AllocatorReport) {
        self.peaks.device_local_bytes = self
            .peaks
            .device_local_bytes
            .max(report.totals.device_local_bytes);
        self.peaks.system_bytes = self.peaks.system_bytes.max(report.totals.system_bytes);
        if report.totals.total_allocated_bytes > self.peaks.total_allocated_bytes {
            self.peaks.total_allocated_bytes = report.totals.total_allocated_bytes;
            self.peak_report = Some(report);
        }
    }
    pub fn free(&mut self, allocation: B::Allocation) -> Result<()> {
        self.backend.free(allocation)
    }
    pub fn peak_total_allocated_bytes(&self) -> u64 {
        self.peaks.total_allocated_bytes
    }
    pub fn peak_device_local_bytes(&self) -> u64 {
        self.peaks.device_local_bytes
    }
    pub fn peak_system_bytes(&self) -> u64 {
        self.peaks.system_bytes
    }
    pub fn peak_allocator_report(&self) -> Option<&AllocatorReport> {
        self.peak_report.as_ref()
    }
}
pub struct DirectMemoryBackend {
    pub device: Arc<Device>,
}
impl AllocationBackend for DirectMemoryBackend {
    type Allocation = MemoryBlock;
    fn allocate(&mut self, request: &AllocationRequest) -> Result<MemoryBlock> {
        let (required, preferred) = request.location.properties();
        let block = MemoryBlock::allocate_dedicated(
            self.device.clone(),
            request.requirements,
            required,
            preferred,
            request.flags,
        )?;
        self.device
            .memory
            .stats
            .rename(block.accounting_id, request.name.clone());
        Ok(block)
    }
    fn free(&mut self, allocation: MemoryBlock) -> Result<()> {
        if !Arc::ptr_eq(&self.device, &allocation.device) {
            return Err(Error::invalid_usage("不能向其他设备释放内存"));
        }
        drop(allocation);
        Ok(())
    }
    fn report(&self) -> AllocatorReport {
        self.device.memory.stats.report()
    }
}

#[cfg(test)]
mod budget_tests {
    use super::*;
    #[test]
    fn pending_descriptor_pool_owners_keep_the_global_quota_charged() {
        let stats = std::sync::Arc::new(AllocatorStats::default());
        let first = stats.reserve_update_after_bind_descriptors(6, 10).unwrap();
        assert!(stats.reserve_update_after_bind_descriptors(5, 10).is_err());
        let second = stats.reserve_update_after_bind_descriptors(4, 10).unwrap();
        drop(first);
        let third = stats.reserve_update_after_bind_descriptors(6, 10).unwrap();
        assert!(stats.reserve_update_after_bind_descriptors(1, 10).is_err());
        drop((second, third));
        assert!(stats.reserve_update_after_bind_descriptors(10, 10).is_ok());
    }
    use crate::{
        device::BudgetCategory,
        sync::{CompletedValues, QueueId, SyncPoint},
    };
    #[test]
    fn allocation_budget_tracks_real_lifetime_and_rejects_overcommit_atomically() {
        let stats = AllocatorStats::default();
        stats.set_total_budget(100).unwrap();
        let id = stats
            .register(
                60,
                0,
                0,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                "resource".into(),
            )
            .unwrap();
        assert_eq!(stats.budget_usage().live, 60);
        let point = SyncPoint {
            queue: QueueId::new(0).unwrap(),
            value: 3,
        };
        stats.record_gpu_use(id, point);
        assert_eq!(stats.budget_usage().in_flight, 60);
        assert!(matches!(
            stats.register(
                50,
                0,
                0,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                "over budget".into()
            ),
            Err(Error::OutOfDeviceMemory)
        ));
        assert_eq!(stats.total_registered(), 1);
        assert_eq!(stats.report().allocations.len(), 1);
        let mut completed = CompletedValues::default();
        completed.raise(point.queue, 2);
        stats.observe_completed(&completed);
        stats.mark_cached(id, &completed);
        stats.mark_live(id, &completed);
        assert_eq!(stats.budget_usage().in_flight, 60);
        completed.raise(point.queue, 3);
        stats.observe_completed(&completed);
        assert_eq!(stats.budget_usage().live, 60);
        stats.mark_cached(id, &completed);
        assert_eq!(stats.budget_usage().cache, 60);
        stats.mark_live(id, &completed);
        assert_eq!(stats.budget_usage().live, 60);
        assert_eq!(
            stats.state.lock().phases[&id].last_use.points().next(),
            Some(point)
        );
        stats.mark_cached(id, &completed);
        stats.mark_retiring(id);
        stats.observe_completed(&completed);
        stats.mark_cached(id, &completed);
        stats.mark_live(id, &completed);
        assert_eq!(
            stats.state.lock().phases[&id].category,
            BudgetCategory::Retiring
        );
        assert_eq!(stats.budget_usage().retiring, 60);
        stats.release(id).unwrap();
        assert_eq!(stats.budget_usage().total(), 0);
    }
    #[test]
    fn completion_moves_only_reached_in_flight_uses_across_queues() {
        let stats = AllocatorStats::default();
        let register = || {
            stats
                .register(
                    10,
                    0,
                    0,
                    vk::MemoryPropertyFlags::DEVICE_LOCAL,
                    String::new(),
                )
                .unwrap()
        };
        let (a, b, c) = (register(), register(), register());
        let (q0, q1) = (QueueId::new(0).unwrap(), QueueId::new(1).unwrap());
        // Out-of-order publication on one queue and a two-queue dependency.
        stats.record_gpu_use(
            a,
            SyncPoint {
                queue: q0,
                value: 5,
            },
        );
        stats.record_gpu_use(
            b,
            SyncPoint {
                queue: q0,
                value: 2,
            },
        );
        stats.record_gpu_use(
            c,
            SyncPoint {
                queue: q0,
                value: 1,
            },
        );
        stats.record_gpu_use(
            c,
            SyncPoint {
                queue: q1,
                value: 7,
            },
        );
        assert_eq!(stats.budget_usage().in_flight, 30);
        let mut completed = CompletedValues::default();
        completed.raise(q0, 2);
        stats.observe_completed(&completed);
        assert_eq!(
            stats.budget_usage().in_flight,
            20,
            "b reached; c waits for q1"
        );
        assert_eq!(stats.state.lock().in_flight[0].len(), 1);
        stats.observe_completed(&completed);
        completed.raise(q1, 7);
        stats.observe_completed(&completed);
        assert_eq!(stats.budget_usage().in_flight, 10);
        // A use already covered by the observed snapshot stays live.
        stats.record_gpu_use(
            b,
            SyncPoint {
                queue: q0,
                value: 2,
            },
        );
        assert_eq!(stats.budget_usage().in_flight, 10);
        completed.raise(q0, 5);
        stats.observe_completed(&completed);
        assert_eq!(stats.budget_usage().live, 30);
        for id in [a, b, c] {
            stats.release(id).unwrap();
        }
    }
}

#[cfg(test)]
mod native_spare_tests {
    use super::*;
    use crate::device::BudgetUsage;
    #[test]
    fn native_spare_moves_to_ranges_and_back_without_double_charging() {
        let stats = AllocatorStats::default();
        stats.set_total_budget(100).unwrap();
        let a = stats
            .register_suballocation(20, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 80)
            .unwrap();
        assert_eq!(
            stats.budget_usage(),
            BudgetUsage {
                live: 20,
                cache: 80,
                ..Default::default()
            }
        );
        let b = stats
            .register_suballocation(30, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 50)
            .unwrap();
        assert_eq!(
            stats.budget_usage(),
            BudgetUsage {
                live: 50,
                cache: 50,
                ..Default::default()
            }
        );
        stats.mark_retiring(a);
        stats.release_suballocation(a, 70).unwrap();
        assert_eq!(
            stats.budget_usage(),
            BudgetUsage {
                live: 30,
                cache: 70,
                ..Default::default()
            }
        );
        stats.release_suballocation(b, 100).unwrap();
        assert_eq!(stats.report().totals.total_allocated_bytes, 0);
        assert_eq!(stats.budget_usage().cache, 100);
        assert!(stats.set_heap_budget(0, 99).is_err());
        stats.trim_suballocator_spare(0, 0);
        assert_eq!(stats.budget_usage().total(), 0);
        stats.set_heap_budget(0, 0).unwrap();
    }
    #[test]
    fn failed_native_growth_preserves_the_entire_previous_ledger() {
        let stats = AllocatorStats::default();
        stats.set_total_budget(100).unwrap();
        let a = stats
            .register_suballocation(20, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 80)
            .unwrap();
        let before = stats.report();
        let usage = stats.budget_usage();
        assert!(matches!(
            stats.register_suballocation(40, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 70),
            Err(Error::OutOfDeviceMemory)
        ));
        assert_eq!(stats.report(), before);
        assert_eq!(stats.budget_usage(), usage);
        assert_eq!(stats.suballocator_spare_bytes(), 80);
        assert_eq!(stats.total_registered(), 1);
        stats.release_suballocation(a, 0).unwrap();
        assert_eq!(stats.budget_usage().total(), 0);
    }
    #[test]
    fn host_reservations_use_a_separate_budget_from_device_memory() {
        let stats = Arc::new(AllocatorStats::default());
        stats.set_total_budget(100).unwrap();
        stats.set_host_budget(30).unwrap();
        let host = stats
            .reserve_cpu_bytes(20, crate::device::BudgetCategory::Cache)
            .unwrap();
        // Host metadata does not consume device headroom...
        let a = stats
            .register_suballocation(20, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 80)
            .unwrap();
        assert_eq!(stats.device_memory_usage().total(), 100);
        assert_eq!(stats.budget_usage().total(), 120);
        assert_eq!(stats.cpu_memory_usage().total(), 20);
        // ...and host exhaustion is a host error, never device OOM.
        assert!(matches!(
            stats.reserve_cpu_bytes(11, crate::device::BudgetCategory::Live),
            Err(Error::OutOfHostMemory)
        ));
        assert!(matches!(
            stats.register(
                1,
                0,
                0,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                String::new()
            ),
            Err(Error::OutOfDeviceMemory)
        ));
        stats.release_suballocation(a, 100).unwrap();
        assert_eq!(stats.device_memory_usage().cache, 100);
        drop(host);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        stats.trim_suballocator_spare(0, 0);
        assert_eq!(stats.budget_usage().total(), 0);
    }
    #[test]
    fn driver_heap_budgets_exclude_foreign_usage_and_raise_pressure() {
        let stats = AllocatorStats::default();
        let a = stats
            .register(
                60,
                0,
                1,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                String::new(),
            )
            .unwrap();
        // Heap 1: budget 200, process usage 100 of which 60 are ours.
        stats.apply_driver_heap_budgets(&[(1000, 0), (200, 100)]);
        assert_eq!(stats.heap_usage(1).unwrap(), (60, Some(160)));
        assert_eq!(stats.budget_action(), crate::device::BudgetAction::None);
        assert!(matches!(
            stats.register(
                101,
                0,
                1,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                String::new()
            ),
            Err(Error::OutOfDeviceMemory)
        ));
        let b = stats
            .register(
                80,
                0,
                1,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                String::new(),
            )
            .unwrap();
        assert_ne!(stats.budget_action(), crate::device::BudgetAction::None);
        stats.release(b).unwrap();
        stats.release(a).unwrap();
        assert!(stats.heap_usage(16).is_err());
    }
    #[test]
    fn native_object_count_never_exceeds_the_device_limit() {
        let stats = AllocatorStats::default();
        stats.set_max_native_objects(2);
        stats.acquire_native_object().unwrap();
        let id = stats
            .register_native(8, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL)
            .unwrap();
        stats.acquire_native_object().unwrap();
        assert_eq!(
            stats.acquire_native_object().unwrap_err().kind(),
            crate::ErrorKind::Exhausted
        );
        stats.release_native_object();
        stats.release(id).unwrap();
        assert_eq!(stats.native_object_count(), 0);
    }
}

#[cfg(test)]
mod host_subrange_tests {
    use super::*;
    #[test]
    fn noncoherent_ranges_use_the_physical_base_and_isolate_neighboring_atoms() {
        let range = mapped_host_subrange(1024, 512, 500, 65, 1, 64, false).unwrap();
        assert_eq!((range.memory_offset, range.memory_size), (1088, 64));
        assert_eq!((range.buffer_offset, range.buffer_size), (64, 64));
        let range = mapped_host_subrange(1024, 512, 500, 65, 1, 1, false).unwrap();
        assert_eq!((range.memory_offset, range.memory_size), (1089, 1));
        assert_eq!((range.buffer_offset, range.buffer_size), (65, 1));
        assert!(matches!(
            mapped_host_subrange(1056, 512, 500, 0, 1, 64, false),
            Err(Error::Unsupported(_))
        ));
    }
    #[test]
    fn only_dedicated_memory_can_use_a_non_aligned_end_of_object() {
        let range = mapped_host_subrange(0, 1000, 1000, 999, 1, 256, true).unwrap();
        assert_eq!(
            (range.memory_offset, range.memory_size),
            (768, vk::WHOLE_SIZE)
        );
        assert_eq!((range.buffer_offset, range.buffer_size), (768, 232));
        assert!(mapped_host_subrange(0, 1000, 1000, 999, 1, 256, false).is_err());
        assert!(mapped_host_subrange(0, 1000, 1000, 999, 2, 256, true).is_err());
        assert!(mapped_host_subrange(u64::MAX - 1, 4, 4, 0, 1, 1, true).is_err());
    }

    #[test]
    fn eviction_runs_under_pressure_or_when_the_heap_cannot_hold_the_request() {
        use crate::device::BudgetAction;
        assert!(!needs_eviction(BudgetAction::None, None, 1 << 40));
        assert!(!needs_eviction(BudgetAction::None, Some(100), 100));
        assert!(needs_eviction(BudgetAction::None, Some(99), 100));
        assert!(needs_eviction(BudgetAction::EvictCache, Some(u64::MAX), 1));
        assert!(!needs_eviction(
            BudgetAction::Backpressure,
            Some(u64::MAX),
            1
        ));
        assert!(is_memory_exhaustion(&Error::OutOfDeviceMemory));
        assert!(is_memory_exhaustion(&Error::exhausted("native objects")));
        assert!(!is_memory_exhaustion(&Error::OutOfHostMemory));
        assert!(!is_memory_exhaustion(&Error::DeviceLost));
    }
}
