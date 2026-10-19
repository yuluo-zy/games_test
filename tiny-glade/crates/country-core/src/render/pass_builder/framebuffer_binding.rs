//! framebuffer_binding.rs — ColorAttachment / DepthAttachment（RasterPass 附件描述）。
//!
//! 证据（B_a4_draw.md）。O = 机器码观测，I = 推断，U = 未知（不编造）：
//! - O `ColorAttachment::desc` 0x1409092f0（21 B）：
//!   `ptr = *(self + 8 + (tag == 1) * 0x10); return ptr + 4`，即两个变体都持有一个指向
//!   `Texture`（`{ id: u32 @0, TextureDesc @4 }`）的引用，只是引用所在偏移不同（tag0 @+8，tag1 @+0x18）。
//! - O 数组元素 0x20 B（`RasterPass::new_impl` 以 `r12 = count << 5` 迭代）；tag1 在 `+4` 处多带 16 B
//!   视图字段（`from_iter` 0x14157b150：`movups [r12 + 4]`）。
//! - O 添加到 pass 时的访问类型：tag0 → `0x36`，tag1 → `0x2e`，同步组 1
//!   （`from_iter`：`r8b = (tag == 0) * 8 + 0x2e`，`r9d = 1`）。
//! - O `DepthAttachment`（`new_impl` 0x140867970 读 `*depth`）：tag0 → 访问 `0x2f`（可写），
//!   tag1 → `0x17`（只读），其它 tag（2）→ 无深度；`+4` = 层，`+8` = 纹理引用，`+0x10` = u32（非 0 → 清除），
//!   `+0x14` = f32 清除值。
//! - U 访问类型数值对应的 `vk_sync_fork::AccessType` 变体名（只知数值）。
//!
//! 旧版 `RasterFramebufferBinding::new_*`（inline，无 PDB 过程，且其 5 槽/`color_count` 字段在机器码里没有对应物）
//! 已删除：`RasterPass::new_impl` 直接接受 `&[ColorAttachment]` + `&DepthAttachment`。

use crate::rhapsody_types::Texture;

/// 颜色附件作为 RG 资源访问时的访问类型（O 数值；名称 U）。
pub const COLOR_ACCESS_TEXTURE: u8 = 0x36;
/// tag1（视图/句柄变体）的访问类型（O 数值；名称 U）。
pub const COLOR_ACCESS_VIEW: u8 = 0x2e;
/// 深度附件：可写（O 数值）。
pub const DEPTH_ACCESS_WRITE: u8 = 0x2f;
/// 深度附件：只读（O 数值）。
pub const DEPTH_ACCESS_READ: u8 = 0x17;

/// 附件引用的纹理视图：机器码里读取的字段（`id` @0、`format` @0xc、`sample_count` @0x21、extent）。
/// 是 `rhapsody::resource::Texture` 被读取部分的最小替身；原类型字段布局见
/// `rhapsody_types::Texture`（那里的 `format/width/height` 本身也是 U 占位）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttachmentTexture {
    /// 纹理 id（`*ptr`，O u32）。
    pub id: u32,
    /// `TextureDesc::extent_2d`（`raster_single_object` 读取）。
    pub extent: (u32, u32),
    /// `ptr + 0xc`（O：`RenderPassAttachmentDesc::new(format)` 的参数）。
    pub format: u32,
    /// `ptr + 0x21`（O：`RenderPassAttachmentDesc::sample_count` 的参数）。
    pub sample_count: u8,
}

impl From<&Texture> for AttachmentTexture {
    /// I：占位 `Texture` 的 `width/height/format` 直接映射，`sample_count` 缺省 1（占位类型无此字段）。
    fn from(t: &Texture) -> Self {
        AttachmentTexture {
            id: t.id as u32,
            extent: (t.width, t.height),
            format: t.format,
            sample_count: 1,
        }
    }
}

/// 视图字段（tag1 的 `+4..+0x14`，16 B）：机器码只整块搬运，字段含义 U。
pub type ColorViewWords = [u32; 4];

/// 颜色附件（0x20 B，tag @0）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAttachment {
    /// tag0：直接引用纹理（引用 @+8）。
    Texture(AttachmentTexture),
    /// tag1：带 16 B 视图字（@+4）的引用（引用 @+0x18）。
    View {
        view: ColorViewWords,
        texture: AttachmentTexture,
    },
    /// tag2（B 阶段补，i_api；O：`main_pass` 的 "G-buffer"、`ui_2d_pass` 的 "mei"、`feedback_report_bg` 用 `mov dword [..], 2`）：
    /// 布局与 tag0 相同（引用在 `+8`，无视图字——`from_iter` 只对 `tag == 1` 取 `[+4]` 的 16 B），
    /// 访问类型与 tag1 相同（`(tag == 0) * 8 + 0x2e` → 0x2e），`RasterPass::render` 的描述里 `+4` 覆盖为 2。变体名 U。
    Tag2(AttachmentTexture),
}

impl ColorAttachment {
    /// 判别值（O：`cmp dword ptr [r12], 1`）。
    pub fn tag(&self) -> u32 {
        match self {
            ColorAttachment::Texture(_) => 0,
            ColorAttachment::View { .. } => 1,
            ColorAttachment::Tag2(_) => 2,
        }
    }

    /// `desc`（0x1409092f0）：两个变体都返回被引用纹理的描述。
    pub fn desc(&self) -> &AttachmentTexture {
        match self {
            ColorAttachment::Texture(t) | ColorAttachment::Tag2(t) => t,
            ColorAttachment::View { texture, .. } => texture,
        }
    }

    /// 纹理 id 的可变访问（旧代码的 `handle_mut`，inline）。
    pub fn handle_mut(&mut self) -> &mut u32 {
        match self {
            ColorAttachment::Texture(t) | ColorAttachment::Tag2(t) => &mut t.id,
            ColorAttachment::View { texture, .. } => &mut texture.id,
        }
    }

    /// O（`from_iter`）：加入 RG pass 时使用的访问类型。
    pub fn access_type(&self) -> u8 {
        match self {
            ColorAttachment::Texture(_) => COLOR_ACCESS_TEXTURE,
            ColorAttachment::View { .. } | ColorAttachment::Tag2(_) => COLOR_ACCESS_VIEW,
        }
    }

    /// 视图字（tag0 为 0）。
    pub fn view_words(&self) -> ColorViewWords {
        match self {
            ColorAttachment::Texture(_) | ColorAttachment::Tag2(_) => [0; 4],
            ColorAttachment::View { view, .. } => *view,
        }
    }
}

/// 清除值（深度附件 `+0x10` u32 非 0 → 清除，值在 `+0x14`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DepthClear {
    /// `+0x10 == 0`：保持内容（加载）。
    Load,
    /// `+0x10 != 0`：清除到该深度值（`RenderPassAttachmentDesc::clear_input`）。
    Clear(f32),
}

/// 深度附件（tag @0；tag2 在原版是 `Option::None` 的 niche）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DepthAttachment {
    /// tag0 = 可写，tag1 = 只读（O）。
    pub writable: bool,
    /// `+4`：`TextureViewDesc::base_layer` 的参数。
    pub layer: u32,
    pub texture: AttachmentTexture,
    pub clear: DepthClear,
}

impl DepthAttachment {
    /// `layer`（inline 1）：阴影图集层号等。
    pub fn layer(&self) -> u32 {
        self.layer
    }

    /// 判别值（O：`new_impl` 的 `test r15d; je` / `cmp r15d, 1`）。
    pub fn tag(&self) -> u32 {
        if self.writable { 0 } else { 1 }
    }

    /// O：加入 RG pass 的访问类型。
    pub fn access_type(&self) -> u8 {
        if self.writable {
            DEPTH_ACCESS_WRITE
        } else {
            DEPTH_ACCESS_READ
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tex(id: u32) -> AttachmentTexture {
        AttachmentTexture {
            id,
            extent: (64, 32),
            format: 37,
            sample_count: 1,
        }
    }

    #[test]
    fn color_desc_returns_texture_for_both_variants() {
        // O 0x1409092f0：两个变体都解到纹理描述。
        let a = ColorAttachment::Texture(tex(5));
        let b = ColorAttachment::View {
            view: [1, 2, 3, 4],
            texture: tex(9),
        };
        assert_eq!(a.desc().id, 5);
        assert_eq!(b.desc().id, 9);
        assert_eq!((a.tag(), b.tag()), (0, 1));
    }

    #[test]
    fn color_access_types_match_from_iter() {
        // O 0x14157b150：tag0 → 0x36，tag1 → 0x2e。
        assert_eq!(ColorAttachment::Texture(tex(1)).access_type(), 0x36);
        assert_eq!(
            ColorAttachment::View {
                view: [0; 4],
                texture: tex(1)
            }
            .access_type(),
            0x2e
        );
        assert_eq!(ColorAttachment::Texture(tex(1)).view_words(), [0; 4]);
    }

    #[test]
    fn tag2_has_view_access_no_view_words_and_texture_at_plus8() {
        // O 0x14157b150：`r8b = (tag == 0) * 8 + 0x2e` → tag2 = 0x2e；只有 tag1 搬 16 B 视图字；引用偏移 `(tag == 1) * 0x10 + 8`。
        let c = ColorAttachment::Tag2(tex(12));
        assert_eq!(c.tag(), 2);
        assert_eq!(c.access_type(), 0x2e);
        assert_eq!(c.view_words(), [0; 4]);
        assert_eq!(c.desc().id, 12);
        let mut c = c;
        *c.handle_mut() = 3;
        assert_eq!(c.desc().id, 3);
    }

    #[test]
    fn depth_tags_and_access() {
        let w = DepthAttachment {
            writable: true,
            layer: 3,
            texture: tex(7),
            clear: DepthClear::Clear(1.0),
        };
        let r = DepthAttachment {
            writable: false,
            ..w
        };
        assert_eq!((w.tag(), w.access_type()), (0, 0x2f));
        assert_eq!((r.tag(), r.access_type()), (1, 0x17));
        assert_eq!(w.layer(), 3);
    }

    #[test]
    fn handle_mut_edits_the_referenced_texture_id() {
        let mut c = ColorAttachment::View {
            view: [0; 4],
            texture: tex(2),
        };
        *c.handle_mut() = 11;
        assert_eq!(c.desc().id, 11);
    }

    #[test]
    fn from_placeholder_texture_maps_fields() {
        let t = Texture {
            id: 4,
            width: 16,
            height: 8,
            format: 9,
        };
        let a = AttachmentTexture::from(&t);
        assert_eq!(
            (a.id, a.extent, a.format, a.sample_count),
            (4, (16, 8), 9, 1)
        );
    }
}
