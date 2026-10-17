//! 托管缓冲区：raw 创建与映射操作复用 Vulkan 实现，销毁由公共资源存储延后。
use super::{handle::Managed, resources::ResourceManager};
use crate::raw::{
    buffer::{Buffer, BufferDesc},
    device::Device,
    error::Result,
};
use ash::vk;
use std::sync::Arc;
pub type ManagedBuffer = Managed<Buffer>;
impl Managed<Buffer> {
    pub fn new_buffer(
        store: &ResourceManager<Buffer>,
        device: Arc<Device>,
        desc: &BufferDesc,
    ) -> Result<Self> {
        store.allocate(Buffer::new(device, desc)?)
    }
    pub fn raw(&self) -> vk::Buffer {
        self.handle
    }
    /// 分配大小（字节）。
    pub fn allocation(&self) -> u64 {
        self.resource().size
    }
}
