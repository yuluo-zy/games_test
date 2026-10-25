//! framebuffer_binding.rs — ColorAttachment / DepthAttachment / RasterFramebufferBinding。
//!
//! 机器码锚点：
//! - `ColorAttachment::desc` 0x1409092f0（行 90；反编译空——真实逻辑被内联，typed gap）。
//! - `ColorAttachment::handle_mut`（inline 4）；`DepthAttachment::layer`（inline 1）。
//! - `RasterFramebufferBinding::new_color/new_depth/new_color_depth`（inline 19/1/22）。

use rhapsody::raw::texture::Texture;

/// 颜色附件（0xcB = 3×u32）。
/// 字段：纹理句柄（+0x00）、加载/存储操作或视图信息（+0x04/+0x08）——精确语义 U。
#[derive(Clone, Copy, Debug)]
pub struct ColorAttachment {
    pub texture: u32,
    pub word_1: u32,
    pub word_2: u32,
}

impl ColorAttachment {
    /// `desc`（0x1409092f0）：附件描述（load/store/format 派生）；真实逻辑内联，U。
    pub fn desc(&self) -> rhapsody::raw::shader::RenderPassAttachmentDesc {
        rhapsody::raw::shader::RenderPassAttachmentDesc::new()
    }

    /// `handle_mut`（inline 4）：纹理句柄可变访问。
    pub fn handle_mut(&mut self) -> &mut u32 {
        &mut self.texture
    }
}

/// 深度附件。
#[derive(Clone, Copy, Debug)]
pub struct DepthAttachment {
    pub texture: u32,
    pub layer: u32,
}

impl DepthAttachment {
    /// `layer`（inline 1）。
    pub fn layer(&self) -> u32 {
        self.layer
    }
}

/// 光栅 framebuffer 绑定（附件集合）。
#[derive(Clone, Debug, Default)]
pub struct RasterFramebufferBinding {
    pub colors: [ColorAttachment; 5],
    pub color_count: usize,
    pub depth: Option<DepthAttachment>,
}

impl RasterFramebufferBinding {
    /// `new_color`（inline 22）：纯颜色附件绑定。
    pub fn new_color(colors: impl IntoIterator<Item = ColorAttachment>) -> RasterFramebufferBinding {
        let mut b = RasterFramebufferBinding::default();
        for (i, c) in colors.into_iter().take(5).enumerate() {
            b.colors[i] = c;
            b.color_count = i + 1;
        }
        b
    }

    /// `new_depth`（inline 1）：纯深度绑定。
    pub fn new_depth(depth: DepthAttachment) -> RasterFramebufferBinding {
        RasterFramebufferBinding {
            depth: Some(depth),
            ..Default::default()
        }
    }

    /// `new_color_depth`（inline 19）：颜色 + 深度。
    pub fn new_color_depth(
        colors: impl IntoIterator<Item = ColorAttachment>,
        depth: DepthAttachment,
    ) -> RasterFramebufferBinding {
        let mut b = Self::new_color(colors);
        b.depth = Some(depth);
        b
    }
}

/// 纹理解析（rhapsody 边界辅助）。
pub trait AsTexture {
    fn texture(&self) -> &Texture;
}
