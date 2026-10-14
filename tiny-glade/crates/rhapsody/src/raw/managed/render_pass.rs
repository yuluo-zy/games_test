//! 托管渲染通道。持有 raw 通道并复用统一命令跟踪/退役机制。
use super::{handle::Managed, resources::ResourceManager};
use crate::raw::{device::VulkanDevice, error::Result, render_pass::RenderPass};
use ash::vk;
use std::sync::Arc;
pub type ManagedRenderPass = Managed<RenderPass>;
impl Managed<RenderPass> {
    pub fn new_render_pass(
        store: &ResourceManager<RenderPass>,
        device: Arc<VulkanDevice>,
        desc: &vk::RenderPassCreateInfo<'_>,
    ) -> Result<Self> {
        Ok(store.allocate(RenderPass::new(device, desc)?))
    }
    pub fn raw(&self) -> vk::RenderPass {
        self.handle
    }
}
