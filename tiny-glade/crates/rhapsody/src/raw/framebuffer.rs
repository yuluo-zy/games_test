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
        Self::from_desc(FramebufferDesc::new(
            render_pass,
            attachments,
            extent,
            layers,
        ))
    }
    pub fn from_desc(desc: FramebufferDesc) -> Result<Self> {
        if desc.extent.width == 0 || desc.extent.height == 0 || desc.layers == 0 {
            return Err(Error::invalid_usage("帧缓冲区范围必须非零"));
        }
        if desc
            .attachments
            .iter()
            .any(|v| !Arc::ptr_eq(&v.texture.device, &desc.render_pass.device))
        {
            return Err(Error::invalid_usage("附件和渲染通道不属于同一设备"));
        }
        // 原版（framebuffer.rs:35/45）从附件推导尺寸：无附件 / 附件尺寸不一致都是 panic，这里以原文案返回错误。
        let sizes: Vec<(u32, u32)> = desc
            .attachments
            .iter()
            .map(|v| {
                let mip = v.desc.base_mip_level;
                (
                    (v.texture.desc.width >> mip.min(31)).max(1),
                    (v.texture.desc.height >> mip.min(31)).max(1),
                )
            })
            .collect();
        try_extent_from_attachment_sizes(&sizes)?;
        let handles: Vec<_> = desc.attachments.iter().map(|v| v.handle).collect();
        let info = vk::FramebufferCreateInfo::default()
            .render_pass(desc.render_pass.handle)
            .attachments(&handles)
            .width(desc.extent.width)
            .height(desc.extent.height)
            .layers(desc.layers);
        let handle = unsafe {
            desc.render_pass
                .device
                .device
                .create_framebuffer(&info, None)
        }?;
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
        self.render_pass
            .device
            .bury(super::graveyard::Doomed::Framebuffer(self.handle));
    }
}

/// 由附件尺寸推导帧缓冲尺寸：没有附件或尺寸不一致返回 `InvalidUsage`。
pub fn extent_from_attachment_sizes(sizes: &[(u32, u32)]) -> Result<(u32, u32)> {
    let Some(&first) = sizes.first() else {
        return Err(Error::invalid_usage("帧缓冲至少需要一个附件"));
    };
    if let Some(&other) = sizes[1..].iter().find(|&&size| size != first) {
        return Err(Error::invalid_usage(format!(
            "帧缓冲附件尺寸不一致：{first:?} 与 {other:?}"
        )));
    }
    Ok(first)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ErrorKind;

    #[test]
    fn extent_comes_from_the_common_attachment_size() {
        assert_eq!(extent_from_attachment_sizes(&[(64, 32), (64, 32)]).unwrap(), (64, 32));
    }

    #[test]
    fn empty_or_mismatched_attachments_are_invalid_usage() {
        assert_eq!(extent_from_attachment_sizes(&[]).unwrap_err().kind(), ErrorKind::InvalidUsage);
        assert_eq!(
            extent_from_attachment_sizes(&[(64, 32), (32, 32)]).unwrap_err().kind(),
            ErrorKind::InvalidUsage
        );
    }
}
