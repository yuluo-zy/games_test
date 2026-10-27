//! Device memory blocks (dedicated or suballocated) and device/host memory
//! accounting.
use crate::device::Device;
use crate::accounting::{AllocationTotals, AllocatorReport};
use crate::sync::CompletedValues;
use crate::{Error, Result};
use ash::vk;
use std::sync::Arc;
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
    pub(crate) shared: Option<Arc<crate::device::transient_backing::SharedBacking>>,
    /// 子分配时持有 gpu-allocator 的分配；专用分配为 None。
    #[cfg(feature = "suballocation")]
    pub(crate) sub: Option<crate::device::suballoc::SubAllocation>,
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
            &device,
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
        device: &Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
        buffer: vk::Buffer,
        image: vk::Image,
    ) -> Result<Self> {
        let mut reclaim = AllocationReclaim::new(device);
        Self::allocate_for_resource_reclaim(
            device,
            requirements,
            required,
            preferred,
            flags,
            buffer,
            image,
            &mut reclaim,
        )
    }
    pub(crate) fn allocate_for_resource_reclaim(
        device: &Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
        buffer: vk::Buffer,
        image: vk::Image,
        reclaim: &mut AllocationReclaim,
    ) -> Result<Self> {
        let memory_type = choose_memory_type(
            &device.core.memory_properties,
            requirements.memory_type_bits,
            required,
            preferred,
        );
        relieve_pressure(device, memory_type, requirements.size, reclaim);
        with_eviction_retry(device, requirements.size, reclaim, || {
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
        let handle = match crate::device::observe_vulkan(&device, unsafe {
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
    pub(crate) fn into_deferred(self) -> crate::device::graveyard::DeferredMemory {
        let mut memory = std::mem::ManuallyDrop::new(self);
        // SAFETY: `memory` is never dropped; its Arc is read out exactly once
        // and released, while the caller still holds another device Arc.
        let device = unsafe { std::ptr::read(&memory.device) };
        if let Some(backing) = memory.shared.take() {
            backing.retire_member();
            return crate::device::graveyard::DeferredMemory::Shared(backing);
        }
        device.memory.stats.mark_retiring(memory.accounting_id);
        #[cfg(feature = "suballocation")]
        if let Some(allocation) = memory.sub.take() {
            return crate::device::graveyard::DeferredMemory::Sub { allocation };
        }
        crate::device::graveyard::DeferredMemory::Dedicated {
            handle: memory.handle,
            accounting_id: memory.accounting_id,
        }
    }
    pub fn accounting_id(&self) -> u64 {
        self.accounting_id
    }
    pub(crate) fn from_shared(
        device: Arc<Device>,
        backing: Arc<crate::device::transient_backing::SharedBacking>,
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
    pub(crate) fn shared_use(&self) -> Option<Arc<crate::tracking::SyncCell>> {
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
    action: crate::accounting::BudgetAction,
    heap_available: Option<u64>,
    bytes: u64,
) -> bool {
    action == crate::accounting::BudgetAction::EvictCache
        || heap_available.is_some_and(|available| available < bytes)
}

/// Under budget pressure, or when `bytes` would exceed the heap budget, ask
/// registered caches to release idle entries before allocating.
/// One allocation, including proactive eviction, retry and suballocation
/// fallback, shares a completion snapshot and a finite retirement allowance.
pub(crate) struct AllocationReclaim {
    pub(crate) completed: Option<Option<CompletedValues>>,
    pub(crate) remaining: crate::device::ReclamationBudget,
}
impl AllocationReclaim {
    pub(crate) fn new(device: &Device) -> Self {
        Self {
            completed: None,
            remaining: device.pressure_reclamation_budget(),
        }
    }
}
pub(crate) fn relieve_pressure(
    device: &Device,
    memory_type: Option<u32>,
    bytes: u64,
    reclaim: &mut AllocationReclaim,
) {
    let stats = &device.memory.stats;
    let heap = memory_type
        .map(|index| device.core.memory_properties.memory_types[index as usize].heap_index);
    if needs_eviction(
        stats.budget_action(),
        heap.map(|heap| stats.heap_available(heap)),
        bytes,
    ) {
        device.evict_idle_caches_reclaim(bytes, reclaim);
    }
}

/// Whether an allocation error can be relieved by releasing cached memory.
pub(crate) fn is_memory_exhaustion(error: &Error) -> bool {
    matches!(error, Error::OutOfDeviceMemory | Error::Exhausted(_))
}

/// After memory exhaustion, reclaim completed idle storage and retry exactly
/// once. Reported cache bytes do not prove whether native retirement recovered.
pub(crate) fn with_eviction_retry<T>(
    device: &Device,
    bytes: u64,
    reclaim: &mut AllocationReclaim,
    mut attempt: impl FnMut() -> Result<T>,
) -> Result<T> {
    match attempt() {
        Err(error) if is_memory_exhaustion(&error) => {
            device.evict_idle_caches_reclaim(bytes, reclaim);
            attempt()
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
mod host_subrange_tests;
