//! Device memory suballocation through `gpu-allocator` 0.27, exposed at the
//! [`AllocationBackend`] boundary.
//!
//! Modes: `Suballocated` (default) and `Dedicated` (one `VkDeviceMemory` per
//! resource, for comparison). The process default can be overridden with
//! `RHAPSODY_ALLOCATION=dedicated`.
use super::{
    allocator::{
        AllocationBackend, AllocationRequest, AllocatorReport, MemoryBlock, MemoryLocation,
    },
    device::Device,
    error::{Error, Result},
};
use ash::vk;
use gpu_allocator::vulkan::{Allocation, AllocationCreateDesc, AllocationScheme, Allocator};
use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocationMode {
    /// One dedicated `VkDeviceMemory` per resource.
    Dedicated,
    /// 经 gpu-allocator 在大块内子分配。
    Suballocated,
}

const UNSET: u8 = 0;
const DEDICATED: u8 = 1;
const SUBALLOCATED: u8 = 2;
static DEFAULT_MODE: AtomicU8 = AtomicU8::new(UNSET);

/// 之后创建的设备使用的默认分配模式。已创建的设备不受影响。
pub fn set_default_allocation_mode(mode: AllocationMode) {
    let value = match mode {
        AllocationMode::Dedicated => DEDICATED,
        AllocationMode::Suballocated => SUBALLOCATED,
    };
    DEFAULT_MODE.store(value, Ordering::SeqCst);
}
pub fn default_allocation_mode() -> AllocationMode {
    match DEFAULT_MODE.load(Ordering::SeqCst) {
        DEDICATED => AllocationMode::Dedicated,
        SUBALLOCATED => AllocationMode::Suballocated,
        _ => match std::env::var("RHAPSODY_ALLOCATION").as_deref() {
            Ok("dedicated") => AllocationMode::Dedicated,
            _ => AllocationMode::Suballocated,
        },
    }
}

// gpu-allocator 0.27 retains non-device-OOM Vulkan allocation failures as an
// exact diagnostic string. Preserve those categories at this pinned boundary.
fn allocation_error(error: gpu_allocator::AllocationError) -> Error {
    use gpu_allocator::AllocationError;
    match error {
        AllocationError::OutOfMemory => Error::OutOfDeviceMemory,
        AllocationError::Internal(ref text)
            if text == "Unexpected error in vkAllocateMemory: ERROR_DEVICE_LOST" =>
        {
            Error::DeviceLost
        }
        AllocationError::Internal(ref text)
            if text == "Unexpected error in vkAllocateMemory: ERROR_OUT_OF_HOST_MEMORY" =>
        {
            Error::OutOfHostMemory
        }
        AllocationError::NoCompatibleMemoryTypeFound => {
            Error::unsupported("没有兼容的 Vulkan 内存类型")
        }
        other => Error::invalid_usage_with("gpu-allocator 操作失败", other),
    }
}

/// gpu-allocator 0.27 ignores `AllocatorCreateDesc::allocation_sizes` and
/// always uses its default general block sizes.
const HOST_BLOCK_BYTES: u64 = 64 * 1024 * 1024;
const DEVICE_BLOCK_BYTES: u64 = 256 * 1024 * 1024;

/// Shared by resource retirement and graph backing owners. Dispatch tables and
/// statistics have no Arc<Device> back edge. Each memory type has its own
/// allocator and lock; its block/range counters are maintained incrementally
/// (mirroring gpu-allocator's block lifetime rules), so no allocate/free path
/// walks the allocator's live allocations.
pub struct SubAllocator {
    shards: [Mutex<Shard>; 32],
    stopped: AtomicBool,
    instance: ash::Instance,
    device: ash::Device,
    physical: vk::PhysicalDevice,
    buffer_device_address: bool,
    stats: Arc<super::allocator::AllocatorStats>,
    memory: vk::PhysicalDeviceMemoryProperties,
    reserved_total: AtomicU64,
    physical_peak: AtomicU64,
    pub non_coherent_atom_size: u64,
}
#[derive(Default)]
struct Shard {
    allocator: Option<Allocator>,
    /// Bytes of live ranges handed to callers.
    range_bytes: u64,
    /// Bytes of native blocks owned by `allocator`.
    reserved: u64,
    allocations: usize,
    blocks: Vec<NativeBlock>,
    active_general_blocks: usize,
}
struct NativeBlock {
    memory: vk::DeviceMemory,
    size: u64,
    live: usize,
    general: bool,
}
impl Shard {
    fn spare(&self) -> u64 {
        self.reserved.saturating_sub(self.range_bytes)
    }
}
/// One actual range and its unique allocator owner. Deferred destruction can
/// return it without retaining the Device that owns the retirement queue.
pub struct SubAllocation {
    allocation: Option<Allocation>,
    owner: Arc<SubAllocator>,
    memory_type: u32,
    pub(crate) accounting_id: u64,
}
impl std::ops::Deref for SubAllocation {
    type Target = Allocation;
    fn deref(&self) -> &Allocation {
        // Only `Drop` takes the allocation; no deref can follow it.
        self.allocation.as_ref().expect("live suballocation")
    }
}
impl Drop for SubAllocation {
    fn drop(&mut self) {
        if let Some(allocation) = self.allocation.take() {
            self.owner
                .free(allocation, self.memory_type, self.accounting_id);
        }
    }
}
impl SubAllocator {
    pub fn new(
        instance: &ash::Instance,
        device: &ash::Device,
        physical: vk::PhysicalDevice,
        buffer_device_address: bool,
        stats: Arc<super::allocator::AllocatorStats>,
    ) -> Result<Self> {
        let properties = unsafe { instance.get_physical_device_properties(physical) };
        Ok(Self {
            shards: std::array::from_fn(|_| Mutex::new(Shard::default())),
            stopped: AtomicBool::new(false),
            instance: instance.clone(),
            device: device.clone(),
            physical,
            buffer_device_address,
            stats,
            memory: unsafe { instance.get_physical_device_memory_properties(physical) },
            reserved_total: AtomicU64::new(0),
            physical_peak: AtomicU64::new(0),
            non_coherent_atom_size: properties.limits.non_coherent_atom_size.max(1),
        })
    }
    fn general_block_bytes(&self, memory_type: usize) -> u64 {
        if self.memory.memory_types[memory_type]
            .property_flags
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            HOST_BLOCK_BYTES
        } else {
            DEVICE_BLOCK_BYTES
        }
    }
    /// Whether `size` bytes should be suballocated. Free space in existing
    /// blocks is always used. A new native block is admitted when the device
    /// and heap budgets can hold it, after first trimming empty blocks of
    /// other memory types. Only then does the caller fall back to a
    /// dedicated allocation of exactly `size` bytes.
    pub(crate) fn should_use(&self, memory_type: u32, size: u64) -> bool {
        let index = memory_type as usize;
        if self.shards[index].lock().spare() >= size {
            return true;
        }
        let block = self.general_block_bytes(index);
        if size > block {
            // A personal block of exactly `size` costs the same as dedicated.
            return true;
        }
        let heap = self.memory.memory_types[index].heap_index;
        if self.stats.heap_available(heap) >= block {
            return true;
        }
        self.trim_unused() != 0 && self.stats.heap_available(heap) >= block
    }
    fn make_allocator(&self) -> Result<Allocator> {
        Allocator::new(&gpu_allocator::vulkan::AllocatorCreateDesc {
            instance: self.instance.clone(),
            device: self.device.clone(),
            physical_device: self.physical,
            debug_settings: Default::default(),
            buffer_device_address: self.buffer_device_address,
            allocation_sizes: Default::default(),
        })
        .map_err(allocation_error)
    }
    pub(crate) fn allocate(
        self: &Arc<Self>,
        desc: &AllocationCreateDesc<'_>,
        memory_type: u32,
    ) -> Result<SubAllocation> {
        let index = memory_type as usize;
        let slot = self
            .shards
            .get(index)
            .ok_or_else(|| Error::invalid_usage("memory type index out of range"))?;
        let mut shard = slot.lock();
        if self.stopped.load(Ordering::Acquire) {
            return Err(Error::invalid_usage("suballocator has shut down"));
        }
        let heap = self.memory.memory_types[index].heap_index;
        let mut requirements = desc.requirements;
        requirements.memory_type_bits = 1u32 << memory_type;
        let properties = self.memory.memory_types[index].property_flags;
        if properties.contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
            && !properties.contains(vk::MemoryPropertyFlags::HOST_COHERENT)
        {
            requirements.alignment = requirements.alignment.max(self.non_coherent_atom_size);
            requirements.size = round_up(requirements.size, self.non_coherent_atom_size);
        }
        shard
            .blocks
            .try_reserve(1)
            .map_err(|_| Error::OutOfHostMemory)?;
        // Reserve a VkDeviceMemory slot in case a block is created. Without
        // one, only existing free space may be used.
        let have_slot = self.stats.acquire_native_object().is_ok();
        if !have_slot && shard.spare() < requirements.size {
            return Err(Error::exhausted("maxMemoryAllocationCount reached"));
        }
        if shard.allocator.is_none() {
            match self.make_allocator() {
                Ok(allocator) => shard.allocator = Some(allocator),
                Err(error) => {
                    if have_slot {
                        self.stats.release_native_object();
                    }
                    return Err(error);
                }
            }
        }
        let result = match shard.allocator.as_mut() {
            Some(allocator) => allocator
                .allocate(&AllocationCreateDesc {
                    requirements,
                    ..*desc
                })
                .map_err(allocation_error),
            None => Err(Error::invalid_usage("suballocator shard is unavailable")),
        };
        let allocation = match result {
            Ok(allocation) => allocation,
            Err(error) => {
                if have_slot {
                    self.stats.release_native_object();
                }
                return Err(error);
            }
        };
        let memory = unsafe { allocation.memory() };
        let size = allocation.size();
        let created = match shard.blocks.iter_mut().find(|block| block.memory == memory) {
            Some(block) => {
                block.live += 1;
                false
            }
            None => {
                let general = size <= self.general_block_bytes(index);
                let block_size = if general {
                    self.general_block_bytes(index)
                } else {
                    size
                };
                shard.blocks.push(NativeBlock {
                    memory,
                    size: block_size,
                    live: 1,
                    general,
                });
                if general {
                    shard.active_general_blocks += 1;
                }
                shard.reserved += block_size;
                let total = self.reserved_total.fetch_add(block_size, Ordering::AcqRel) + block_size;
                self.physical_peak.fetch_max(total, Ordering::Relaxed);
                true
            }
        };
        shard.range_bytes += size;
        shard.allocations += 1;
        match (created, have_slot) {
            (false, true) => self.stats.release_native_object(),
            (true, false) => {
                // A fragmented block forced growth past the native object limit.
                let released = self.release_locked(&mut shard, allocation);
                if released {
                    self.stats.trim_suballocator_spare(memory_type, heap);
                }
                return Err(Error::exhausted("maxMemoryAllocationCount reached"));
            }
            _ => {}
        }
        let accounting_id = match self.stats.register_suballocation(
            size,
            memory_type,
            heap,
            allocation.memory_properties(),
            shard.spare(),
        ) {
            Ok(id) => id,
            Err(error) => {
                // gpu-allocator has no native-allocation preflight. Return the
                // range and an emptied shard before exposing it to callers.
                if self.release_locked(&mut shard, allocation) {
                    self.stats.trim_suballocator_spare(memory_type, heap);
                }
                return Err(error);
            }
        };
        Ok(SubAllocation {
            allocation: Some(allocation),
            owner: self.clone(),
            memory_type,
            accounting_id,
        })
    }
    /// Return an unregistered range. Drops the whole allocator when it became
    /// empty; returns whether that happened.
    fn release_locked(&self, shard: &mut Shard, allocation: Allocation) -> bool {
        self.free_locked(shard, allocation);
        if shard.allocations == 0 {
            self.drop_allocator(shard);
            return true;
        }
        false
    }
    /// Free one range and mirror gpu-allocator's block destruction: personal
    /// blocks die with their range, empty general blocks die unless they are
    /// the last general block.
    fn free_locked(&self, shard: &mut Shard, allocation: Allocation) {
        let memory = unsafe { allocation.memory() };
        let size = allocation.size();
        match shard.allocator.as_mut() {
            Some(allocator) => {
                if let Err(error) = allocator.free(allocation) {
                    log::error!("gpu-allocator free failed: {error}");
                }
            }
            None => log::error!("suballocation freed after its allocator shut down"),
        }
        shard.range_bytes = shard.range_bytes.saturating_sub(size);
        shard.allocations = shard.allocations.saturating_sub(1);
        let Some(position) = shard.blocks.iter().position(|block| block.memory == memory) else {
            return;
        };
        let block = &mut shard.blocks[position];
        block.live = block.live.saturating_sub(1);
        if block.live != 0 {
            return;
        }
        let destroyed = if block.general {
            if shard.active_general_blocks > 1 {
                shard.active_general_blocks -= 1;
                true
            } else {
                false
            }
        } else {
            true
        };
        if destroyed {
            let block = shard.blocks.swap_remove(position);
            shard.reserved = shard.reserved.saturating_sub(block.size);
            self.reserved_total.fetch_sub(block.size, Ordering::AcqRel);
            self.stats.release_native_object();
        }
    }
    /// Destroy an empty shard's allocator and all of its native blocks.
    fn drop_allocator(&self, shard: &mut Shard) -> u64 {
        if shard.allocator.take().is_none() {
            return 0;
        }
        for _ in shard.blocks.drain(..) {
            self.stats.release_native_object();
        }
        let released = std::mem::take(&mut shard.reserved);
        self.reserved_total.fetch_sub(released, Ordering::AcqRel);
        shard.active_general_blocks = 0;
        released
    }
    fn free(&self, allocation: Allocation, memory_type: u32, accounting_id: u64) {
        let index = memory_type as usize;
        let Some(slot) = self.shards.get(index) else {
            return;
        };
        let mut shard = slot.lock();
        self.free_locked(&mut shard, allocation);
        let spare = if shard.allocations == 0
            && self.stats.budget_action() == crate::device::BudgetAction::EvictCache
        {
            self.drop_allocator(&mut shard);
            0
        } else {
            shard.spare()
        };
        if let Err(error) = self.stats.release_suballocation(accounting_id, spare) {
            log::error!("suballocation accounting release failed: {error}");
        }
    }
    /// Only wholly empty memory-type shards can release native blocks. Holes in
    /// a shard with live ranges remain charged as GPU cache bytes.
    pub fn trim_unused(&self) -> u64 {
        let mut released = 0;
        for index in 0..self.memory.memory_type_count as usize {
            let mut shard = self.shards[index].lock();
            if shard.allocator.is_some() && shard.allocations == 0 {
                released += self.drop_allocator(&mut shard);
                self.stats.trim_suballocator_spare(
                    index as u32,
                    self.memory.memory_types[index].heap_index,
                );
            }
        }
        released
    }
    pub fn block_count(&self) -> usize {
        self.memory_usage().3
    }
    pub fn physical_peak_bytes(&self) -> u64 {
        self.physical_peak.load(Ordering::Relaxed)
    }
    /// `(range bytes, reserved native bytes, live ranges, native blocks)`.
    pub(crate) fn memory_usage(&self) -> (u64, u64, usize, usize) {
        let mut usage = (0, 0, 0, 0);
        for shard in &self.shards[..self.memory.memory_type_count as usize] {
            let shard = shard.lock();
            usage.0 += shard.range_bytes;
            usage.1 += shard.reserved;
            usage.2 += shard.allocations;
            usage.3 += shard.blocks.len();
        }
        usage
    }
    pub(crate) fn shutdown(&self) {
        self.stopped.store(true, Ordering::Release);
        for (index, shard) in self.shards.iter().enumerate() {
            let mut shard = shard.lock();
            if shard.allocations != 0 {
                log::error!(
                    "suballocator shutdown with {} live ranges in memory type {index}",
                    shard.allocations
                );
            }
            self.drop_allocator(&mut shard);
        }
    }
}

fn location_from_flags(
    required: vk::MemoryPropertyFlags,
    preferred: vk::MemoryPropertyFlags,
) -> gpu_allocator::MemoryLocation {
    use gpu_allocator::MemoryLocation as L;
    if required.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL) {
        L::GpuOnly
    } else if required.contains(vk::MemoryPropertyFlags::HOST_VISIBLE) {
        if preferred.contains(vk::MemoryPropertyFlags::HOST_CACHED) {
            L::GpuToCpu
        } else {
            L::CpuToGpu
        }
    } else {
        L::Unknown
    }
}

fn round_up(value: u64, to: u64) -> u64 {
    value.div_ceil(to) * to
}

impl MemoryBlock {
    /// Suballocate. `linear` marks buffers and linear-tiling images so
    /// gpu-allocator honours `bufferImageGranularity` between neighbours.
    pub(crate) fn allocate_sub(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
        linear: bool,
    ) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let sub = device
            .suballocator
            .as_ref()
            .ok_or_else(|| Error::invalid_usage("设备未启用子分配"))?;
        if requirements.size == 0
            || (flags.contains(vk::MemoryAllocateFlags::DEVICE_ADDRESS)
                && !device.capabilities.buffer_device_address)
        {
            return Err(Error::invalid_usage("内存大小或设备地址分配能力无效"));
        }
        let location = location_from_flags(required, preferred);
        let mut adjusted = requirements;
        if location != gpu_allocator::MemoryLocation::GpuOnly {
            // 宿主可见分配自身范围对齐到非一致性粒度，这样刷新/失效恰好覆盖本分配而不波及邻居。
            adjusted.alignment = adjusted.alignment.max(sub.non_coherent_atom_size);
            adjusted.size = round_up(adjusted.size, sub.non_coherent_atom_size);
        }
        let memory_type = super::allocator::choose_memory_type(
            &device.memory,
            adjusted.memory_type_bits,
            required,
            preferred,
        )
        .ok_or_else(|| Error::unsupported("no compatible suballocation memory type"))?;
        super::allocator::relieve_pressure(&device, Some(memory_type), adjusted.size);
        // should_use already trimmed empty native blocks; release idle cache
        // entries before falling back to a dedicated allocation.
        if !sub.should_use(memory_type, adjusted.size)
            && (device.evict_idle_caches(adjusted.size) == 0
                || !sub.should_use(memory_type, adjusted.size))
        {
            return Self::allocate_dedicated(device, requirements, required, preferred, flags);
        }
        let allocation = super::allocator::with_eviction_retry(&device, adjusted.size, || {
            sub.allocate(
                &AllocationCreateDesc {
                    name: "",
                    requirements: adjusted,
                    location,
                    linear,
                    allocation_scheme: AllocationScheme::GpuAllocatorManaged,
                },
                memory_type,
            )
        })
            .inspect_err(|error| {
                if matches!(error, Error::DeviceLost) {
                    device.timelines.mark_device_lost();
                }
            })?;
        let properties = allocation.memory_properties();
        let allocation_size = allocation.size();
        let accounting_id = allocation.accounting_id;
        Ok(Self {
            handle: unsafe { allocation.memory() },
            offset: allocation.offset(),
            size: allocation_size,
            properties,
            accounting_id,
            shared: None,
            sub: Some(allocation),
            device,
        })
    }
}

impl Device {
    pub fn suballocation_enabled(&self) -> bool {
        self.suballocator.is_some()
    }
}

/// gpu-allocator suballocation behind [`AllocationBackend`]; usable with the
/// peak-tracking [`super::allocator::GpuAllocator`] wrapper.
pub struct SubAllocationBackend {
    pub device: Arc<Device>,
}
impl AllocationBackend for SubAllocationBackend {
    type Allocation = MemoryBlock;
    fn allocate(&mut self, request: &AllocationRequest) -> Result<MemoryBlock> {
        let (required, preferred) = request.location.properties();
        let linear = !matches!(request.location, MemoryLocation::Unknown);
        let block = MemoryBlock::allocate_sub(
            self.device.clone(),
            request.requirements,
            required,
            preferred,
            request.flags,
            linear,
        )?;
        self.device
            .allocator_stats
            .rename(block.accounting_id(), request.name.clone());
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
        self.device.allocator_stats.report()
    }
}

#[cfg(test)]
mod allocation_error_tests {
    use super::*;
    #[test]
    fn pinned_allocator_errors_preserve_fatal_categories() {
        assert!(matches!(
            allocation_error(gpu_allocator::AllocationError::OutOfMemory),
            Error::OutOfDeviceMemory
        ));
        for (code, expected) in [
            ("ERROR_DEVICE_LOST", crate::ErrorKind::DeviceLost),
            ("ERROR_OUT_OF_HOST_MEMORY", crate::ErrorKind::OutOfMemory),
        ] {
            let error = gpu_allocator::AllocationError::Internal(format!(
                "Unexpected error in vkAllocateMemory: {code}"
            ));
            assert_eq!(allocation_error(error).kind(), expected);
        }
    }
}
