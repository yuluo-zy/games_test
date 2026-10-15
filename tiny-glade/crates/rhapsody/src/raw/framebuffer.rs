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
            return Err(Error::Invalid("帧缓冲区范围必须非零"));
        }
        if desc
            .attachments
            .iter()
            .any(|v| !Arc::ptr_eq(&v.texture.device, &desc.render_pass.device))
        {
            return Err(Error::Invalid("附件和渲染通道不属于同一设备"));
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

/// 原 `RenderPassApi` 帧缓冲尺寸推导中的两处 panic 文案（O）：
/// - `panic!("Must have at least one framebuffer attachment")`：`framebuffer.rs:45:25`，
///   机器码 0x1416f776f..0x1416f7787（字符串 0x142fd9448 len 0x2d，Location 0x142fd94a0）；
/// - `assert_eq!(size, first_size, "Framebuffer attachments must all have the same size")`：
///   `framebuffer.rs:35:21`，机器码 0x1418c38aa..0x1418c38dd（片段 0x142fd94f0 len 0x33，Location 0x142fd9500）。
pub const NO_ATTACHMENT_MESSAGE: &str = "Must have at least one framebuffer attachment";
pub const SIZE_MISMATCH_MESSAGE: &str = "Framebuffer attachments must all have the same size";

/// [`extent_from_attachment_sizes`] 的 `Result` 形态：错误文案就是原 panic 文案。
pub fn try_extent_from_attachment_sizes(sizes: &[(u32, u32)]) -> Result<(u32, u32)> {
    let Some(&first) = sizes.first() else {
        return Err(Error::Invalid(NO_ATTACHMENT_MESSAGE));
    };
    if sizes[1..].iter().any(|&size| size != first) {
        return Err(Error::Invalid(SIZE_MISMATCH_MESSAGE));
    }
    Ok(first)
}

/// 由附件尺寸推导帧缓冲尺寸：空 panic；任一尺寸与首个不同 panic（原版为 assert_eq，
/// 其 Debug 左右值格式 `assertion `left == right` failed: <msg>` 由 core 生成，这里同样使用 assert_eq!）。
/// 注意：原版是否把该推导放进 `vkCreateFramebuffer` 的输入（extent 来自附件而非描述符）已读到，
/// 但 `Framebuffer::from_desc` 的 `extent` 仍由调用方给出，故此函数不接入创建路径。
pub fn extent_from_attachment_sizes(sizes: &[(u32, u32)]) -> (u32, u32) {
    let Some(&first) = sizes.first() else {
        panic!("Must have at least one framebuffer attachment");
    };
    for &size in &sizes[1..] {
        assert_eq!(
            size, first,
            "Framebuffer attachments must all have the same size"
        );
    }
    first
}

#[cfg(test)]
mod original_panic_tests {
    use super::*;

    #[test]
    fn extent_comes_from_the_common_attachment_size() {
        assert_eq!(extent_from_attachment_sizes(&[(64, 32), (64, 32)]), (64, 32));
    }

    #[test]
    #[should_panic(expected = "Must have at least one framebuffer attachment")]
    fn empty_attachments_panic_with_original_message() {
        extent_from_attachment_sizes(&[]);
    }

    #[test]
    #[should_panic(expected = "Framebuffer attachments must all have the same size")]
    fn mismatched_sizes_panic_with_original_message() {
        extent_from_attachment_sizes(&[(64, 32), (32, 32)]);
    }

    #[test]
    fn result_form_carries_the_original_messages() {
        assert_eq!(try_extent_from_attachment_sizes(&[(8, 4), (8, 4)]).unwrap(), (8, 4));
        assert_eq!(
            try_extent_from_attachment_sizes(&[]).unwrap_err().to_string(),
            NO_ATTACHMENT_MESSAGE
        );
        assert_eq!(
            try_extent_from_attachment_sizes(&[(8, 4), (8, 5)]).unwrap_err().to_string(),
            SIZE_MISMATCH_MESSAGE
        );
    }

    #[test]
    fn messages_have_original_lengths() {
        assert_eq!(NO_ATTACHMENT_MESSAGE.len(), 0x2d);
        assert_eq!(SIZE_MISMATCH_MESSAGE.len(), 0x33);
    }
}
// 另：原 `GraphicsDevice::create_framebuffer`（0x140bfbef0）的 vkCreateFramebuffer 失败**不 panic**：
// 0x140bfbfa2..0x140bfbfb0 返回 Err 变体（tag 9，+0x20 存 vk::Result），成功为 tag 0xb（托管句柄）。
// 这与 Rust 版 `Result<_, Error>` 传出一致。
