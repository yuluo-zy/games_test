//! 托管缓冲区：raw 创建与映射操作复用 Vulkan 实现，销毁由公共资源存储延后。
use super::{handle::Managed, resources::ResourceManager};
use crate::raw::{
    buffer::{Buffer, BufferDesc},
    device::VulkanDevice,
    error::Result,
};
use ash::vk;
use std::sync::Arc;
pub type ManagedBuffer = Managed<Buffer>;
impl Managed<Buffer> {
    pub fn new_buffer(
        store: &ResourceManager<Buffer>,
        device: Arc<VulkanDevice>,
        desc: &BufferDesc,
    ) -> Result<Self> {
        Ok(store.allocate(Buffer::new(device, desc)?))
    }
    pub fn raw(&self) -> vk::Buffer {
        self.handle
    }
    /// 分配大小（字节）。
    pub fn allocation(&self) -> u64 {
        self.resource().size
    }
    /// 持久映射的可变切片（委托 raw Buffer 映射）。
    /// 需要独占访问（无其他 clone 句柄）；共享句柄无法安全暴露 `&mut`。
    pub fn mapped_slice_mut(
        &mut self,
        offset: u64,
        size: u64,
    ) -> Result<&mut [u8]> {
        let inner = std::sync::Arc::get_mut(&mut self.0)
            .ok_or(crate::raw::error::Error::Invalid("句柄被共享，无法独占映射"))?;
        let buffer = std::sync::Arc::get_mut(&mut inner.resource)
            .ok_or(crate::raw::error::Error::Invalid("缓冲区被共享"))?;
        buffer.mapped_slice_mut(offset, size)
    }
}
