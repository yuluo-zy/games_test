//! Framebuffer 持有 render pass 与附件，避免先销毁其依赖资源。
use super::{
    error::{Error, Result},
    render_pass::RenderPass,
    texture::TextureView,
};
use ash::vk;
use std::sync::Arc;

/// 帧缓冲创建描述符；`Framebuffer::new` 的输入集合。
#[derive(Clone)]
pub struct FramebufferDesc {
    pub render_pass: Arc<RenderPass>,
    pub attachments: Vec<Arc<TextureView>>,
    pub extent: vk::Extent2D,
    pub layers: u32,
}
impl FramebufferDesc {
    pub fn new(
        render_pass: Arc<RenderPass>,
        attachments: Vec<Arc<TextureView>>,
        extent: vk::Extent2D,
        layers: u32,
    ) -> Self {
        Self {
            render_pass,
            attachments,
            extent,
            layers,
        }
    }
}
pub struct Framebuffer {
    pub render_pass: Arc<RenderPass>,
    pub attachments: Vec<Arc<TextureView>>,
    pub handle: vk::Framebuffer,
    pub extent: vk::Extent2D,
    pub layers: u32,
}
impl Framebuffer {
    pub fn new(
        render_pass: Arc<RenderPass>,
        attachments: Vec<Arc<TextureView>>,
        extent: vk::Extent2D,
        layers: u32,
    ) -> Result<Self> {
        Self::from_desc(FramebufferDesc::new(render_pass, attachments, extent, layers))
    }
    pub fn from_desc(desc: FramebufferDesc) -> Result<Self> {
        if desc.extent.width == 0 || desc.extent.height == 0 || desc.layers == 0 {
            return Err(Error::Invalid("帧缓冲区范围必须非零"));
        }
        if desc
            .attachments
            .iter()
            .any(|v| !Arc::ptr_eq(&v.texture.device, &desc.render_pass.device))
        {
            return Err(Error::Invalid("附件和渲染通道不属于同一设备"));
        }
        let handles: Vec<_> = desc.attachments.iter().map(|v| v.handle).collect();
        let info = vk::FramebufferCreateInfo::default()
            .render_pass(desc.render_pass.handle)
            .attachments(&handles)
            .width(desc.extent.width)
            .height(desc.extent.height)
            .layers(desc.layers);
        let handle = unsafe { desc.render_pass.device.device.create_framebuffer(&info, None) }?;
        Ok(Self {
            render_pass: desc.render_pass,
            attachments: desc.attachments,
            handle,
            extent: desc.extent,
            layers: desc.layers,
        })
    }
}
impl Drop for Framebuffer {
    fn drop(&mut self) {
        let _guard = self
            .render_pass
            .device
            .queue_lock
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            let _ = self.render_pass.device.device.device_wait_idle();
            self.render_pass
                .device
                .device
                .destroy_framebuffer(self.handle, None);
        }
    }
}
