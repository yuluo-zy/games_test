//! 托管帧缓冲。raw 层持有附件与渲染通道依赖，退役时统一释放所有权。
use super::{handle::Managed, resources::ResourceManager};
use crate::raw::{
    error::Result, framebuffer::Framebuffer, render_pass::RenderPass, texture::TextureView,
};
use ash::vk;
use std::sync::Arc;
pub type ManagedFramebuffer = Managed<Framebuffer>;
impl Managed<Framebuffer> {
    pub fn new_framebuffer(
        store: &ResourceManager<Framebuffer>,
        pass: Arc<RenderPass>,
        views: Vec<Arc<TextureView>>,
        extent: vk::Extent2D,
        layers: u32,
    ) -> Result<Self> {
        Ok(store.allocate(Framebuffer::new(pass, views, extent, layers)?))
    }
    pub fn raw(&self) -> vk::Framebuffer {
        self.handle
    }
}
