//! Buffer 创建、内存绑定、映射读写候选；`BufferDesc` 对齐原 16 字节描述符布局。
use super::{
    allocator::MemoryBlock,
    device::VulkanDevice,
    error::{Error, Result},
    graveyard::Doomed,
};
use ash::vk;
use std::sync::Arc;
/// 原作 `BufferDesc.memory_location` 枚举（取值 1..=4，0 非法）。
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryLocation {
    GpuOnly = 1,
    CpuToGpu = 2,
    /// HOST_VISIBLE 但非低延迟映射，不要求 HOST_COHERENT。
    CpuToGpuLowPrio = 3,
    GpuToCpu = 4,
}
impl MemoryLocation {
    pub const fn as_raw(self) -> u8 {
        self as u8
    }
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            1 => Some(Self::GpuOnly),
            2 => Some(Self::CpuToGpu),
            3 => Some(Self::CpuToGpuLowPrio),
            4 => Some(Self::GpuToCpu),
            _ => None,
        }
    }
    /// 返回 (必需, 偏好) 内存属性；映射集中在此处，创建路径不再自行拼装。
    pub fn memory_properties(self) -> (vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) {
        use vk::MemoryPropertyFlags as F;
        match self {
            Self::GpuOnly => (F::DEVICE_LOCAL, F::DEVICE_LOCAL),
            Self::CpuToGpu => (F::HOST_VISIBLE, F::HOST_COHERENT),
            Self::CpuToGpuLowPrio => (F::HOST_VISIBLE, F::empty()),
            Self::GpuToCpu => (F::HOST_VISIBLE, F::HOST_CACHED | F::HOST_COHERENT),
        }
    }
}
/// 原 16 字节 ABI：`{u64 size, u32 usage, u16 alignment, u8 memory_location}`。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BufferDesc {
    pub size: u64,
    pub usage: vk::BufferUsageFlags,
    /// 请求的对齐（字节）；0 表示无显式要求。
    pub alignment: u16,
    pub memory_location: MemoryLocation,
}
const _: () = assert!(std::mem::size_of::<BufferDesc>() == 16);
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
    /// 原 memory_location=3 语义：HOST_VISIBLE 但非低延迟映射（不要求 HOST_COHERENT）。
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
pub struct Buffer {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::Buffer,
    pub size: u64,
    pub usage: vk::BufferUsageFlags,
    allocation: std::mem::ManuallyDrop<MemoryBlock>,
    mapped_ptr: Option<usize>,
}
impl Buffer {
    pub fn new(device: Arc<VulkanDevice>, desc: &BufferDesc) -> Result<Self> {
        if desc.size == 0 {
            return Err(Error::Invalid("缓冲区大小不能为零"));
        }
        if !device.capabilities.buffer_device_address
            && desc
                .usage
                .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            return Err(Error::Unsupported("此路径未启用 bufferDeviceAddress"));
        }
        let info = vk::BufferCreateInfo::default()
            .size(desc.size)
            .usage(desc.usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let handle = unsafe { device.device.create_buffer(&info, None) }?;
        let requirements = unsafe { device.device.get_buffer_memory_requirements(handle) };
        let flags = if desc
            .usage
            .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            vk::MemoryAllocateFlags::DEVICE_ADDRESS
        } else {
            vk::MemoryAllocateFlags::empty()
        };
        let (required_memory, preferred_memory) = desc.memory_location.memory_properties();
        let allocation = match MemoryBlock::allocate_with_flags(
            Arc::clone(&device),
            requirements,
            required_memory,
            preferred_memory,
            flags,
        ) {
            Ok(value) => value,
            Err(error) => {
                unsafe {
                    device.device.destroy_buffer(handle, None);
                }
                return Err(error);
            }
        };
        if let Err(error) = unsafe {
            device
                .device
                .bind_buffer_memory(handle, allocation.handle, allocation.offset)
        } {
            unsafe {
                device.device.destroy_buffer(handle, None);
            }
            return Err(error.into());
        }
        Ok(Self {
            device,
            handle,
            size: desc.size,
            usage: desc.usage,
            allocation: std::mem::ManuallyDrop::new(allocation),
            mapped_ptr: None,
        })
    }

    /// 请求持久映射；首次调用建立映射并保留到 `drop`（或下一次 `unmap`）。
    /// 返回的可变切片在下次映射/释放前保持有效，调用者负责同步边界。
    pub fn mapped_slice_mut(&mut self, offset: u64, size: u64) -> Result<&mut [u8]> {
        let end = offset
            .checked_add(size)
            .ok_or(Error::Invalid("映射范围上溢"))?;
        if end > self.size {
            return Err(Error::Invalid("映射访问越过缓冲区边界"));
        }
        if !self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            return Err(Error::Unsupported("内存不可映射"));
        }
        let address = match self.mapped_ptr {
            Some(address) => address,
            // 子分配：gpu-allocator 已持久映射整块，指针已含偏移；不能再 vkMapMemory 同一块内存。
            None if self.allocation.host_ptr().is_some() => {
                self.allocation.host_ptr().map_or(0, |p| p as usize)
            }
            None => {
                let _queue = self
                    .device
                    .queue_lock
                    .lock();
                self.device.idle_if_in_flight_locked()?;
                let pointer = unsafe {
                    self.device.device.map_memory(
                        self.allocation.handle,
                        0,
                        vk::WHOLE_SIZE,
                        vk::MemoryMapFlags::empty(),
                    )
                }?;
                let address = pointer as usize;
                self.mapped_ptr = Some(address);
                address
            }
        };
        // SAFETY: 上面已建立/复用本块持续映射；映射至少覆盖整个内存对象，
        // 返回的借用只在调用方持有时有效，期间不允许再 unmap 或 drop。
        let start = address as *mut u8;
        let start = unsafe { start.add(offset as usize) };
        Ok(unsafe { std::slice::from_raw_parts_mut(start, size as usize) })
    }

    /// 释放持久映射；不撤销借用（调用者必须先结束对切片的借用）。
    pub fn unmap(&mut self) {
        if self.mapped_ptr.take().is_some() {
            unsafe {
                self.device.device.unmap_memory(self.allocation.handle);
            }
        }
    }

    /// 宿主刷新/失效范围 `(偏移, 大小)`，供测试核对子分配的对齐前提。
    #[doc(hidden)]
    pub fn host_range_for_test(&self) -> (u64, u64) {
        self.allocation.host_range()
    }
    /// 地址只能用于具有 DEVICE_ADDRESS 分配标记且设备已启用 BDA 的缓冲。
    pub fn device_address(&self) -> Result<vk::DeviceAddress> {
        if !self.device.capabilities.buffer_device_address
            || !self
                .usage
                .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            return Err(Error::Unsupported("缓冲区未启用设备地址用途"));
        }
        let info = vk::BufferDeviceAddressInfo::default().buffer(self.handle);
        Ok(unsafe { self.device.device.get_buffer_device_address(&info) })
    }
    fn transfer(&self, offset: u64, bytes: &mut [u8], write: bool, sync: bool) -> Result<()> {
        if offset
            .checked_add(bytes.len() as u64)
            .is_none_or(|end| end > self.size)
        {
            return Err(Error::Invalid("映射访问越过缓冲区边界"));
        }
        if !self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            return Err(Error::Unsupported("内存不可映射"));
        }
        let _queue = self
            .device
            .queue_lock
            .lock();
        if sync {
            self.device.idle_if_in_flight_locked()?;
        }
        // 子分配直接用已持久映射的指针；专用分配每次自己 map/unmap。
        let shared_pointer = self.allocation.host_ptr();
        let pointer = match shared_pointer {
            Some(pointer) => pointer.cast(),
            None => unsafe {
                self.device.device.map_memory(
                    self.allocation.handle,
                    0,
                    vk::WHOLE_SIZE,
                    vk::MemoryMapFlags::empty(),
                )
            }?,
        };
        let (range_offset, range_size) = self.allocation.host_range();
        let range = [vk::MappedMemoryRange::default()
            .memory(self.allocation.handle)
            .offset(range_offset)
            .size(range_size)];
        let unmap = || {
            if shared_pointer.is_none() {
                unsafe { self.device.device.unmap_memory(self.allocation.handle) };
            }
        };
        let coherent = self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
        let result = if !write && !coherent {
            unsafe { self.device.device.invalidate_mapped_memory_ranges(&range) }
        } else {
            Ok(())
        };
        if let Err(error) = result {
            unmap();
            return Err(error.into());
        }
        unsafe {
            let region = pointer.cast::<u8>().add(offset as usize);
            if write {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), region, bytes.len());
            } else {
                std::ptr::copy_nonoverlapping(region, bytes.as_mut_ptr(), bytes.len());
            }
        }
        let result = if write && !coherent {
            unsafe { self.device.device.flush_mapped_memory_ranges(&range) }
        } else {
            Ok(())
        };
        unmap();
        Ok(result?)
    }
    pub fn write_bytes(&self, offset: u64, bytes: &[u8]) -> Result<()> {
        self.transfer(offset, &mut bytes.to_vec(), true, true)
    }
    /// 不等待在飞 GPU 工作的写入。
    ///
    /// 仅用于刚创建、GPU 从未访问过的缓冲（如上传暂存缓冲）；对可能被 GPU 使用的缓冲请用 `write_bytes`。
    pub fn write_bytes_unsynchronized(&self, offset: u64, bytes: &[u8]) -> Result<()> {
        self.transfer(offset, &mut bytes.to_vec(), true, false)
    }
    /// 在已确认本缓冲 GPU 操作完成后读回，保留非 coherent 内存失效处理，不做整设备等待。
    ///
    /// # Safety
    /// 调用者必须等待覆盖本缓冲全部在飞访问的 fence/timeline 提交凭据，
    /// 并保证复制期间没有其他 GPU 或宿主并发访问。GPU 写入还需已建立 HOST_READ 可见性。
    /// 此调用只依赖本缓冲完成，不推进设备全局完成 epoch。
    pub unsafe fn read_bytes_after_completion(&self, offset: u64, bytes: &mut [u8]) -> Result<()> {
        self.transfer(offset, bytes, false, false)
    }
    pub fn read_bytes(&self, offset: u64, bytes: &mut [u8]) -> Result<()> {
        self.transfer(offset, bytes, false, true)
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        if self.mapped_ptr.take().is_some() {
            unsafe {
                self.device.device.unmap_memory(self.allocation.handle);
            }
        }
        // SAFETY: allocation 只在此处取出一次，之后不会再被访问。
        let memory = unsafe { std::mem::ManuallyDrop::take(&mut self.allocation) }.into_deferred();
        self.device.bury(Doomed::Buffer(self.handle, memory));
    }
}
