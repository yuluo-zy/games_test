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
    device::VulkanDevice,
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

/// 设备持有的子分配器。`Allocator` 内部只克隆了 ash 句柄表，不持有 `Arc<VulkanDevice>`，没有引用环。
pub struct SubAllocator {
    allocator: Mutex<Option<Allocator>>,
    /// 非一致性内存的刷新/失效范围必须按该粒度对齐，所以宿主可见分配的对齐与大小都向它取整。
    pub non_coherent_atom_size: u64,
}
impl SubAllocator {
    pub fn new(
        instance: &ash::Instance,
        device: &ash::Device,
        physical: vk::PhysicalDevice,
        buffer_device_address: bool,
    ) -> Result<Self> {
        let allocator = Allocator::new(&gpu_allocator::vulkan::AllocatorCreateDesc {
            instance: instance.clone(),
            device: device.clone(),
            physical_device: physical,
            debug_settings: Default::default(),
            buffer_device_address,
            allocation_sizes: Default::default(),
        })
        .map_err(|error| {
            log::error!("gpu-allocator 初始化失败: {error}");
            Error::unsupported("gpu-allocator 初始化失败")
        })?;
        log::info!("启用显存子分配（gpu-allocator）");
        let properties = unsafe { instance.get_physical_device_properties(physical) };
        Ok(Self {
            allocator: Mutex::new(Some(allocator)),
            non_coherent_atom_size: properties.limits.non_coherent_atom_size.max(1),
        })
    }
    fn allocate(&self, desc: &AllocationCreateDesc<'_>) -> Result<Allocation> {
        let mut guard = self.allocator.lock();
        let allocator = guard
            .as_mut()
            .ok_or(Error::invalid_usage("子分配器已随设备销毁"))?;
        allocator.allocate(desc).map_err(|error| {
            log::warn!(
                "子分配失败: size={} alignment={} location={:?}: {error}",
                desc.requirements.size,
                desc.requirements.alignment,
                desc.location
            );
            Error::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY)
        })
    }
    fn free(&self, allocation: Allocation) {
        let mut guard = self.allocator.lock();
        if let Some(allocator) = guard.as_mut() {
            // 释放失败只可能是重复释放；登记表另有一一对应的断言，这里不再额外 panic。
            let _ = allocator.free(allocation);
        }
    }
    /// 真实 `VkDeviceMemory` 块数（即 vkAllocateMemory 的存活次数）。
    pub fn block_count(&self) -> usize {
        self.allocator
            .lock()
            .as_ref()
            .map_or(0, |a| a.generate_report().blocks.len())
    }
    /// 设备销毁前释放全部内存块；必须先于 `vkDestroyDevice`。
    pub(crate) fn shutdown(&self) {
        self.allocator.lock().take();
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

/// 登记表需要类型/堆序号：gpu-allocator 不暴露，按属性在设备内存类型里反查。
fn locate_memory_type(
    memory: &vk::PhysicalDeviceMemoryProperties,
    bits: u32,
    properties: vk::MemoryPropertyFlags,
) -> (u32, u32) {
    let find = |exact: bool| {
        (0..memory.memory_type_count).find(|&i| {
            let flags = memory.memory_types[i as usize].property_flags;
            bits & (1 << i) != 0
                && if exact {
                    flags == properties
                } else {
                    flags.contains(properties)
                }
        })
    };
    let index = find(true).or_else(|| find(false)).unwrap_or(0);
    (index, memory.memory_types[index as usize].heap_index)
}

fn round_up(value: u64, to: u64) -> u64 {
    value.div_ceil(to) * to
}

impl MemoryBlock {
    /// 经子分配器分配；`linear` 标明线性资源（缓冲/线性图像）或最优平铺图像，
    /// 由 gpu-allocator 按 `bufferImageGranularity` 处理两类资源的相邻关系。
    pub(crate) fn allocate_sub(
        device: Arc<VulkanDevice>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
        linear: bool,
    ) -> Result<Self> {
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
        let allocation = sub.allocate(&AllocationCreateDesc {
            name: "",
            requirements: adjusted,
            location,
            linear,
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })?;
        let properties = allocation.memory_properties();
        let (memory_type, heap) =
            locate_memory_type(&device.memory, requirements.memory_type_bits, properties);
        let accounting_id = match device.allocator_stats.register(
            requirements.size,
            memory_type,
            heap,
            properties,
            String::new(),
        ) {
            Ok(id) => id,
            Err(error) => {
                sub.free(allocation);
                return Err(error);
            }
        };
        Ok(Self {
            handle: unsafe { allocation.memory() },
            offset: allocation.offset(),
            size: requirements.size,
            properties,
            accounting_id,
            sub: Some(allocation),
            device,
        })
    }
}

impl VulkanDevice {
    pub fn suballocation_enabled(&self) -> bool {
        self.suballocator.is_some()
    }
    pub(crate) fn free_suballocation(&self, allocation: Allocation) {
        if let Some(sub) = &self.suballocator {
            sub.free(allocation);
        }
    }
    /// 真实 `VkDeviceMemory` 块数：子分配模式下是大块数，专用模式下等于存活分配数。
    pub fn memory_block_count(&self) -> usize {
        match &self.suballocator {
            Some(sub) => sub.block_count(),
            None => self.allocator_stats.report().allocations.len(),
        }
    }
}

/// 把 gpu-allocator 接到原 `AllocationBackend` 边界；与 `DirectMemoryBackend`（专用分配）同构，
/// 可直接放进游戏私有峰值包装 `GpuAllocator<B>`。
pub struct SubAllocationBackend {
    pub device: Arc<VulkanDevice>,
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
