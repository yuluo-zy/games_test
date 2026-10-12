//! Buffer 创建、内存绑定、映射读写候选；不声称此描述符等于原 16 字节 ABI。
use super::{
    allocator::MemoryBlock,
    device::VulkanDevice,
    error::{Error, Result},
};
use ash::vk;
use std::sync::Arc;
#[derive(Clone, Copy, Debug)]
pub struct BufferDesc {
    pub size: u64,
    pub usage: vk::BufferUsageFlags,
    pub required_memory: vk::MemoryPropertyFlags,
    pub preferred_memory: vk::MemoryPropertyFlags,
}
impl BufferDesc {
    pub fn new_gpu_only(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self {
            size,
            usage,
            required_memory: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            preferred_memory: vk::MemoryPropertyFlags::DEVICE_LOCAL,
        }
    }
    pub fn new_cpu_to_gpu(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self {
            size,
            usage,
            required_memory: vk::MemoryPropertyFlags::HOST_VISIBLE,
            preferred_memory: vk::MemoryPropertyFlags::HOST_COHERENT,
        }
    }
    pub fn new_gpu_to_cpu(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self {
            size,
            usage,
            required_memory: vk::MemoryPropertyFlags::HOST_VISIBLE,
            preferred_memory: vk::MemoryPropertyFlags::HOST_CACHED
                | vk::MemoryPropertyFlags::HOST_COHERENT,
        }
    }
}
pub struct Buffer {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::Buffer,
    pub size: u64,
    pub usage: vk::BufferUsageFlags,
    allocation: MemoryBlock,
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
        let allocation = match MemoryBlock::allocate_with_flags(
            Arc::clone(&device),
            requirements,
            desc.required_memory,
            desc.preferred_memory,
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
                .bind_buffer_memory(handle, allocation.handle, 0)
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
            allocation,
        })
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
    fn transfer(&self, offset: u64, bytes: &mut [u8], write: bool) -> Result<()> {
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
            .lock()
            .map_err(|_| Error::Invalid("队列锁失效"))?;
        unsafe {
            self.device.device.device_wait_idle()?;
        }
        let pointer = unsafe {
            self.device.device.map_memory(
                self.allocation.handle,
                0,
                vk::WHOLE_SIZE,
                vk::MemoryMapFlags::empty(),
            )
        }?;
        let range = [vk::MappedMemoryRange::default()
            .memory(self.allocation.handle)
            .offset(0)
            .size(vk::WHOLE_SIZE)];
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
            unsafe {
                self.device.device.unmap_memory(self.allocation.handle);
            }
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
        unsafe {
            self.device.device.unmap_memory(self.allocation.handle);
        }
        Ok(result?)
    }
    pub fn write_bytes(&self, offset: u64, bytes: &[u8]) -> Result<()> {
        self.transfer(offset, &mut bytes.to_vec(), true)
    }
    pub fn read_bytes(&self, offset: u64, bytes: &mut [u8]) -> Result<()> {
        self.transfer(offset, bytes, false)
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        let _lock = self
            .device
            .queue_lock
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            let _ = self.device.device.device_wait_idle();
            self.device.device.destroy_buffer(self.handle, None);
        }
    }
}
