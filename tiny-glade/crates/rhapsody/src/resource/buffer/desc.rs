//! Buffer descriptions and their constructors.
use ash::vk;
use super::memory::MemoryLocation;

/// 16-byte layout: `{u64 size, u32 usage, u16 alignment, u8 memory_location}`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BufferDesc {
    pub size: u64,
    pub usage: vk::BufferUsageFlags,
    /// 请求的对齐（字节）；0 表示无显式要求。
    pub alignment: u16,
    pub memory_location: MemoryLocation,
}

impl BufferDesc {
    pub fn new(size: u64, usage: vk::BufferUsageFlags, memory_location: MemoryLocation) -> Self {
        Self {
            size,
            usage,
            alignment: 0,
            memory_location,
        }
    }
    pub fn new_gpu_only(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self::new(size, usage, MemoryLocation::GpuOnly)
    }
    pub fn new_cpu_to_gpu(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self::new(size, usage, MemoryLocation::CpuToGpu)
    }
    /// HOST_VISIBLE memory without a HOST_COHERENT requirement.
    pub fn new_cpu_to_gpu_low_prio(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self::new(size, usage, MemoryLocation::CpuToGpuLowPrio)
    }
    pub fn new_gpu_to_cpu(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self::new(size, usage, MemoryLocation::GpuToCpu)
    }
    /// 显式对齐请求；由调用方在满足 Vulkan memory-requirement 的前提下设置。
    pub fn with_alignment(mut self, alignment: u16) -> Self {
        self.alignment = alignment;
        self
    }
    pub fn alignment(&self) -> u16 {
        self.alignment
    }
}
