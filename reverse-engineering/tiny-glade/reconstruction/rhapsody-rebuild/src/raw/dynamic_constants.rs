//! 动态常量页的可运行候选实现；原页大小/复用策略仍待整链对照。
//! 设备实际支持并启用 BDA 时提供真实地址；其余设备明确返回 Unsupported。
use super::{
    binding::{DescriptorSetBinding, ResourceOwner},
    buffer::{Buffer, BufferDesc},
    device::VulkanDevice,
    error::{Error, Result},
};
use ash::vk;
use std::sync::Arc;

pub struct DynamicConstantsChunk {
    pub buffer: Arc<Buffer>,
    pub offset: u64,
    pub size: u64,
}
impl DynamicConstantsChunk {
    pub fn write_bytes(&self, bytes: &[u8]) -> Result<()> {
        if bytes.len() as u64 > self.size {
            return Err(Error::Invalid("数据超过常量区段范围"));
        }
        self.buffer.write_bytes(self.offset, bytes)
    }
    pub fn device_address(&self) -> Result<vk::DeviceAddress> {
        self.buffer
            .device_address()?
            .checked_add(self.offset)
            .ok_or(Error::Invalid("设备地址溢出"))
    }
    pub fn into_gpu_lifetime_tracker(self) -> ResourceOwner {
        self.buffer
    }
    fn binding(&self, ty: vk::DescriptorType) -> DescriptorSetBinding {
        DescriptorSetBinding::Buffer {
            ty,
            info: vk::DescriptorBufferInfo {
                buffer: self.buffer.handle,
                offset: self.offset,
                range: self.size,
            },
            owner: self.buffer.clone(),
        }
    }
    pub fn into_uniform_buffer_binding(self) -> DescriptorSetBinding {
        self.binding(vk::DescriptorType::UNIFORM_BUFFER)
    }
    pub fn into_uniform_and_storage_buffer_binding(
        self,
    ) -> (DescriptorSetBinding, DescriptorSetBinding) {
        (
            self.binding(vk::DescriptorType::UNIFORM_BUFFER),
            self.binding(vk::DescriptorType::STORAGE_BUFFER),
        )
    }
    /// 候选接口返回安全的数据副本；不暴露受 GPU 生命周期约束的裸映射指针。
    pub fn into_uniform_buffer_binding_and_data(self) -> Result<(DescriptorSetBinding, Vec<u8>)> {
        let mut bytes = vec![
            0;
            usize::try_from(self.size)
                .map_err(|_| Error::Invalid("区段长度无法表示"))?
        ];
        self.buffer.read_bytes(self.offset, &mut bytes)?;
        Ok((self.binding(vk::DescriptorType::UNIFORM_BUFFER), bytes))
    }
}

struct Page {
    buffer: Arc<Buffer>,
    cursor: u64,
}
pub struct DynamicConstants {
    device: Arc<VulkanDevice>,
    page_size: u64,
    alignment: u64,
    pages: Vec<Page>,
}
impl DynamicConstants {
    /// alignment 必须来自本设备 uniform/storage offset 对齐要求的较大值。
    pub fn new(device: Arc<VulkanDevice>, page_size: u64, alignment: u64) -> Result<Self> {
        if page_size == 0 || alignment == 0 || !alignment.is_power_of_two() {
            return Err(Error::Invalid("常量页大小或对齐无效"));
        }
        Ok(Self {
            device,
            page_size,
            alignment,
            pages: Vec::new(),
        })
    }
    pub fn allocate(&mut self, size: u64) -> Result<DynamicConstantsChunk> {
        if size == 0 {
            return Err(Error::Invalid("常量区段大小不能为零"));
        }
        for page in &mut self.pages {
            let offset = align_up(page.cursor, self.alignment)?;
            if let Some(end) = offset
                .checked_add(size)
                .filter(|end| *end <= page.buffer.size)
            {
                page.cursor = end;
                return Ok(DynamicConstantsChunk {
                    buffer: page.buffer.clone(),
                    offset,
                    size,
                });
            }
        }
        let capacity = align_up(self.page_size.max(size), self.alignment)?;
        let buffer = Arc::new(Buffer::new(
            self.device.clone(),
            &BufferDesc::new_cpu_to_gpu(capacity, self.buffer_usage()),
        )?);
        self.pages.push(Page {
            buffer: buffer.clone(),
            cursor: size,
        });
        Ok(DynamicConstantsChunk {
            buffer,
            offset: 0,
            size,
        })
    }
    pub fn allocate_with_data(&mut self, bytes: &[u8]) -> Result<DynamicConstantsChunk> {
        let chunk = self.allocate(bytes.len() as u64)?;
        chunk.write_bytes(bytes)?;
        Ok(chunk)
    }
    pub fn allocate_buffer(&self, size: u64) -> Result<DynamicConstantsChunk> {
        let buffer = Arc::new(Buffer::new(
            self.device.clone(),
            &BufferDesc::new_cpu_to_gpu(size, self.buffer_usage()),
        )?);
        Ok(DynamicConstantsChunk {
            buffer,
            offset: 0,
            size,
        })
    }
    /// 先等待 GPU，且只复用没有任何外部 chunk/descriptor 持有的页。
    pub fn reset(&mut self) -> Result<()> {
        // wait_idle 已持队列锁，不能重复加锁造成 reset 永久等待。
        self.device.wait_idle()?;
        for page in &mut self.pages {
            if Arc::strong_count(&page.buffer) == 1 {
                page.cursor = 0;
            }
        }
        Ok(())
    }

    fn buffer_usage(&self) -> vk::BufferUsageFlags {
        let mut usage = vk::BufferUsageFlags::UNIFORM_BUFFER | vk::BufferUsageFlags::STORAGE_BUFFER;
        if self.device.capabilities.buffer_device_address {
            usage |= vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS;
        }
        usage
    }
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
}
fn align_up(value: u64, alignment: u64) -> Result<u64> {
    value
        .checked_add(alignment - 1)
        .map(|v| v & !(alignment - 1))
        .ok_or(Error::Invalid("常量区段对齐溢出"))
}
