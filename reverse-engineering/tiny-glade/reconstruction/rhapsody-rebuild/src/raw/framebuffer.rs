//! Framebuffer 持有 render pass 与附件，避免先销毁其依赖资源。
use super::{
    error::{Error, Result},
    render_pass::RenderPass,
    texture::TextureView,
};
use ash::vk;
use std::sync::Arc;
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
        if extent.width == 0 || extent.height == 0 || layers == 0 {
            return Err(Error::Invalid("帧缓冲区范围必须非零"));
        }
        if attachments
            .iter()
            .any(|v| !Arc::ptr_eq(&v.texture.device, &render_pass.device))
        {
            return Err(Error::Invalid("附件和渲染通道不属于同一设备"));
        }
        let handles: Vec<_> = attachments.iter().map(|v| v.handle).collect();
        let info = vk::FramebufferCreateInfo::default()
            .render_pass(render_pass.handle)
            .attachments(&handles)
            .width(extent.width)
            .height(extent.height)
            .layers(layers);
        let handle = unsafe { render_pass.device.device.create_framebuffer(&info, None) }?;
        Ok(Self {
            render_pass,
            attachments,
            handle,
            extent,
            layers,
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
