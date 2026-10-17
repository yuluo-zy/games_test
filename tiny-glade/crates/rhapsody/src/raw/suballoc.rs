//! 显存子分配：把公开 `gpu-allocator` 接到 [`AllocationBackend`] 边界上，不自写次分配算法。
//!
//! 原作 `rhapsody` 依赖 gpu-allocator 的本地分叉（Cargo.lock 记 0.26.0，另依赖 arc-swap/egui/puffin），
//! 本身就做块内子分配。PDB 中 `with_global_allocator` 返回 `AllocatorReport`，该类型只存在于 0.27 系
//! （`generate_report`），公开 0.26.0 又依赖 ash <=0.37，所以算法按公开 0.27.0 复用；
//! 分叉相对 0.27 的附加项（egui 可视化、puffin、arc-swap 的用法）未恢复，不影响分配行为。
//!
//! 模式：`Suballocated`（默认，与原版一致）与 `Dedicated`（逐资源专用内存，仅作对比回退）。
//! 默认值由进程级开关决定；未设置时看环境变量 `RHAPSODY_ALLOCATION=dedicated`。
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
    atomic::{AtomicU8, Ordering},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocationMode {
    /// 每个资源一块专用 `VkDeviceMemory`（原有行为）。
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
        // 原版就是子分配（gpu-allocator）；环境变量只用于回退到逐资源专用分配做对比。
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

/// Shared by resource retirement and graph backing owners. Dispatch tables and
/// statistics have no Arc<Device> back edge. An empty memory-type shard can be
/// dropped independently because gpu-allocator retains its last general block.
pub struct SubAllocator {
    shards: Mutex<Shards>,
    instance: ash::Instance,
    device: ash::Device,
    physical: vk::PhysicalDevice,
    buffer_device_address: bool,
    stats: Arc<super::allocator::AllocatorStats>,
    memory: vk::PhysicalDeviceMemoryProperties,
    physical_peak: std::sync::atomic::AtomicU64,
    pub non_coherent_atom_size: u64,
}
struct Shards {
    allocators: [Option<Allocator>; 32],
    /// Internal aligned chunk padding is native cache, not an exposed range.
    range_bytes: [u64; 32],
    stopped: bool,
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
            shards: Mutex::new(Shards {
                allocators: std::array::from_fn(|_| None),
                range_bytes: [0; 32],
                stopped: false,
            }),
            instance: instance.clone(),
            device: device.clone(),
            physical,
            buffer_device_address,
            stats,
            memory: unsafe { instance.get_physical_device_memory_properties(physical) },
            physical_peak: Default::default(),
            non_coherent_atom_size: properties.limits.non_coherent_atom_size.max(1),
        })
    }
    pub(crate) fn should_use(&self, memory_type: u32, size: u64) -> bool {
        let limit = self.stats.budget_limit();
        if limit == 0 {
            return true;
        }
        let shards = self.shards.lock();
        if let Some(allocator) = &shards.allocators[memory_type as usize] {
            let report = allocator.generate_report();
            if report.total_reserved_bytes - report.total_allocated_bytes >= size {
                return true;
            }
        }
        // gpu-allocator 0.27 ignores AllocatorCreateDesc.allocation_sizes in
        // Allocator::new and installs its defaults (64/256 MiB) unconditionally.
        // Do not claim a smaller configured block works. Existing free ranges
        // remain useful; otherwise use the established dedicated fallback when
        // a native block would leave no budget for host metadata/other types.
        let default_block = if self.memory.memory_types[memory_type as usize]
            .property_flags
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            64 * 1024 * 1024
        } else {
            256 * 1024 * 1024
        };
        let native = default_block.max(size);
        self.stats.budget_available() >= native.saturating_add((limit / 16).max(64 * 1024))
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
        let mut shards = self.shards.lock();
        if shards.stopped {
            return Err(Error::invalid_usage("suballocator has shut down"));
        }
        let index = memory_type as usize;
        let heap = self.memory.memory_types[index].heap_index;
        if shards.allocators[index].is_none() {
            shards.allocators[index] = Some(self.make_allocator()?);
        }
        let allocator = shards.allocators[index]
            .as_mut()
            .expect("initialized shard");
        let mut requirements = desc.requirements;
        requirements.memory_type_bits = 1u32 << memory_type;
        let properties = self.memory.memory_types[index].property_flags;
        if properties.contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
            && !properties.contains(vk::MemoryPropertyFlags::HOST_COHERENT)
        {
            requirements.alignment = requirements.alignment.max(self.non_coherent_atom_size);
            requirements.size = round_up(requirements.size, self.non_coherent_atom_size);
        }
        let allocation = allocator
            .allocate(&AllocationCreateDesc {
                requirements,
                ..*desc
            })
            .map_err(allocation_error)?;
        let report = allocator.generate_report();
        let next_ranges = shards.range_bytes[index]
            .checked_add(allocation.size())
            .expect("native allocated ranges fit u64");
        let spare = report.total_reserved_bytes - next_ranges;
        let observed: u64 = shards
            .allocators
            .iter()
            .flatten()
            .map(|allocator| allocator.generate_report().total_reserved_bytes)
            .sum();
        self.physical_peak.fetch_max(observed, Ordering::Relaxed);
        let accounting_id = match self.stats.register_suballocation(
            allocation.size(),
            memory_type,
            heap,
            allocation.memory_properties(),
            spare,
        ) {
            Ok(id) => id,
            Err(error) => {
                // The dependency has no native-allocation preflight callback.
                // Admit its observed native delta, or return the real range and
                // destroy an empty shard before exposing a resource to callers.
                let allocator = shards.allocators[index].as_mut().expect("allocated shard");
                allocator
                    .free(allocation)
                    .expect("fresh allocation can be rolled back");
                let report = allocator.generate_report();
                if report.allocations.is_empty() {
                    shards.allocators[index].take();
                    self.stats.trim_suballocator_spare(memory_type, heap);
                }
                return Err(error);
            }
        };
        shards.range_bytes[index] = next_ranges;
        Ok(SubAllocation {
            allocation: Some(allocation),
            owner: self.clone(),
            memory_type,
            accounting_id,
        })
    }
    fn free(&self, allocation: Allocation, memory_type: u32, accounting_id: u64) {
        let mut shards = self.shards.lock();
        let index = memory_type as usize;
        let size = allocation.size();
        let allocator = shards.allocators[index]
            .as_mut()
            .expect("allocation owner is alive");
        allocator
            .free(allocation)
            .expect("suballocation is returned once");
        let report = allocator.generate_report();
        shards.range_bytes[index] -= size;
        let spare = if report.allocations.is_empty()
            && self.stats.budget_action() == crate::device::BudgetAction::EvictCache
        {
            shards.allocators[index].take();
            0
        } else {
            report.total_reserved_bytes - shards.range_bytes[index]
        };
        self.stats
            .release_suballocation(accounting_id, spare)
            .expect("range and native spare accounting are released together");
    }
    /// Only wholly empty memory-type shards can release native blocks. Holes in
    /// a shard with live ranges remain charged as GPU cache bytes.
    pub fn trim_unused(&self) -> u64 {
        let mut shards = self.shards.lock();
        let mut released = 0;
        for (index, slot) in shards.allocators.iter_mut().enumerate() {
            if let Some(allocator) = slot {
                let report = allocator.generate_report();
                if report.allocations.is_empty() {
                    released += report.total_reserved_bytes;
                    slot.take();
                    self.stats.trim_suballocator_spare(
                        index as u32,
                        self.memory.memory_types[index].heap_index,
                    );
                }
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
    pub(crate) fn memory_usage(&self) -> (u64, u64, usize, usize) {
        let shards = self.shards.lock();
        let mut usage =
            shards
                .allocators
                .iter()
                .flatten()
                .fold((0, 0, 0, 0), |total, allocator| {
                    let report = allocator.generate_report();
                    (
                        0,
                        total.1 + report.total_reserved_bytes,
                        total.2 + report.allocations.len(),
                        total.3 + report.blocks.len(),
                    )
                });
        usage.0 = shards.range_bytes.iter().sum();
        usage
    }
    pub(crate) fn shutdown(&self) {
        self.trim_unused();
        let mut shards = self.shards.lock();
        debug_assert!(
            shards
                .allocators
                .iter()
                .flatten()
                .all(|allocator| allocator.generate_report().allocations.is_empty())
        );
        for slot in &mut shards.allocators {
            slot.take();
        }
        shards.stopped = true;
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
    /// 经子分配器分配；`linear` 标明线性资源（缓冲/线性图像）或最优平铺图像，
    /// 由 gpu-allocator 按 `bufferImageGranularity` 处理两类资源的相邻关系。
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
            .ok_or(Error::invalid_usage("设备未启用子分配"))?;
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
        if !sub.should_use(memory_type, adjusted.size) {
            return Self::allocate_dedicated(device, requirements, required, preferred, flags);
        }
        let allocation = sub
            .allocate(
                &AllocationCreateDesc {
                    name: "",
                    requirements: adjusted,
                    location,
                    linear,
                    allocation_scheme: AllocationScheme::GpuAllocatorManaged,
                },
                memory_type,
            )
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

/// 把 gpu-allocator 接到原 `AllocationBackend` 边界；与 `DirectMemoryBackend`（专用分配）同构，
/// 可直接放进游戏私有峰值包装 `GpuAllocator<B>`。
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
