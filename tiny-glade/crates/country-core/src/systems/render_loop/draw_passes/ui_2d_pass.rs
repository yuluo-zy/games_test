//! `ui_2d_pass.rs` — 最终图 UI 2D pass（`ui_2d_pass` 0x1409c4470，8843 B）与 MEI pass（`mei_pass` 0x1409c6700，3255 B）。
//!
//! # `ui_2d_pass`（O）
//! `Res<AssetShaderLibrary>` ×3 取 `get_raster_handle_by_name("ui_2d")` / `("ui_2d_color_button")` / 之后 `("ui_2d_ws")`
//! （未命中 `expect` panic）；`Res<WindowSize>`（`+0` 宽、`+4` 高，转 f32）；
//! `Query<(&Ui2dExtent, &Ui2dImage, Option<&CustomizationIcon>, Option<&UiInteractionState>, Option<&UiDrawOrder>, Option<&UiInactive>)>` 收集进 `Vec`（元素 0x30 B）并按第 5 项（`+0x20`，指向 u64 的 `Option<&_>`）
//! **稳定排序**（`sort_by`，`insertion_sort_shift_left`/`driftsort` 同 draw_lists）；
//! `extent_2d(target)` → `RasterPass::new_impl("UI 2D"(5), [tag0 = target], 无深度, 视口 = target)`；
//! 每个元素一次 `DrawPacket::new` + `DrawBuilder::new(variant 0)`，**无 `bind_common`**：
//! - 有 `tint`（元素 `+0x10` 非空）→ 着色器 `ui_2d_color_button`：uniform `0x120d1fbd3e93b50f` = Vec2(extent[0], extent[1])、
//!   `-0x590372c7d4be463b` = Vec2(extent[2], extent[3])、`0x69fdab9b991c4787` = Vec2(窗口宽, 窗口高)、
//!   `-0x154d5a253117dd2a` = f32(tint+0x18)、`0x0d8ea64f9bd82ef1` = bool(tint+0x21)；
//! - 无 `tint` → 着色器 `ui_2d`：同三个 Vec2，`0x35594af7455f3925` = bool(`+0x28` 的 Option 非空)、
//!   `0x4c5803d8a5526c67` = bool(button+2)；
//! - 两者都：`NameHash::compute(image.name)` → `TextureViewDesc::default().view_type(1).base_layer(image.layer).layer_count(1,1)`，
//!   `bind_texture_to_draw_builder(TextureFromDisk(hash), 绑定名 0x740686c2e74bfe87, 访问 0x1d, view)`；
//!   状态 `byte0 &= 0x7f`、`byte3 = (byte3 & 0x9f) | 0x40`；`draw_no_vao(6)`。
//! 世界空间元素（`Query<(&Ui2dWsExtent, &Ui2dImage)>`，着色器 `ui_2d_ws`，**不排序**）：
//! `write_uniform<u32>(-0x3a5498c04f79ae0b, 1)`、Vec3(`0x120d…`) = ws.+8 起 12 B、Vec2(`-0x5903…`) = ws.+0/+4、Vec2(`0x69fd…`) = 窗口，
//! 同样的图像绑定与状态改写，`draw_no_vao(6)`。两个循环共用同一个 `RasterPass`，最后 `render` 一次。
//!
//! Ui2dExtent/Ui2dImage/UiDrawOrder 已有共享组件定义；下面的 owned 绘制快照从组件转换。color_icon 等其余读取视图仍保留各自证据边界。
//!
//! # `mei_pass`（O）
//! `AssetShaderLibrary` → `("text")`；`WindowSize`；`TextRenderer`（tg_text，外部库）`render()` → 文本图元数 `n` 与图集纹理；
//! `extent_2d(target)` + `TextureDesc::new_2d(…, 格式 0x2b)` → `RenderGraph::create_raw_resource("mei temp"(8))`；
//! `RasterPass("Mei 2D"(6), [tag @ebx = temp 纹理], 无深度)`；`DrawBuilder::new(variant 0)`：
//! `write_uniform<Vec2>(0x0a5a0625b7860559, (宽 as f32, 高 as f32))` → `TextureFromDisk(0x579f39ab91a01865)` @ `-0x7ea40afc96996874`（访问 0x1d）
//! → 原生纹理（图集）@ `0x753991f424563352`（0x1d）→ `read_storage_buffer<TextPrimSsbo>(-0x284141834374fd41)` →
//! `read_storage_buffer<ShapeStreamSsbo>(-0x6ed09d503dcccc4d)` → 状态 `byte0 &= 0x7f`、`byte3 = (b3&0x9f)|0x40` →
//! `draw_no_vao(6 * n)` → `render`；然后 `("mei_shadow")` → `raster_single_object("UI shadow combine", [tag0 = target], shader, 闭包)`，
//! 闭包（`closure_env$0`，随 raster_single 0x1409cdec0）：`TextureFromDisk(0x579f39ab91a01865)` @ `-0x7ea40afc96996874` 与
//! mei temp（RG 句柄）@ `0x11ba5697cc40b3dd`，状态改写同上。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation, clippy::unnecessary_sort_by)]
use glam::Vec2;

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::name_hash::NameHash;
use crate::render::pass_builder::draw_builder::{
    ACCESS_SAMPLED_IMAGE, BindTexture, DrawBuilder, UniformValue,
};
use crate::render::pass_builder::draw_stream::DrawPacket;
use crate::render::pass_builder::framebuffer_binding::{AttachmentTexture, ColorAttachment};
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::raster_single::raster_single_object;
use crate::render::ssbo::{StorageElement, TypedStorageBuffer, rhapsody_boundary::GraphicsDevice};
use crate::resources::texture_presets::TextureFromDisk;
use crate::rhapsody_contract::ManagedHandle;
use crate::systems::render_loop::draw_passes::custom_cursor_pass::overlay_state;
use crate::systems::render_loop::draw_passes::d_main_support::{ColorSpec, DepthSpec, DrawPassCtx};

/// Original ShapeStream is a u32 word stream; ctor14160fb70 allocates
/// 0x400000 bytes, storage usage0x22/memory1/align4, None has no upload.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShapeStreamSsbo(pub u32);

/// Reuse the actual tg-text48B primitive. Original packaged text vertex
/// shader PrimPacked matches all eight member offsets and array stride48;
/// no parallel guessed packet schema is created in country-core.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextPrimSsbo(pub tg_text::Primitive);

impl StorageElement for ShapeStreamSsbo {
    const NAME: &'static str = "ShapeStream";
    const BUFFER_LEN: usize = 1048576;
}
impl StorageElement for TextPrimSsbo {
    const NAME: &'static str = "TextPrim";
    const BUFFER_LEN: usize = 131072;
}
impl LibrarySsbo for ShapeStreamSsbo {
    type Storage = TypedStorageBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        TypedStorageBuffer::new_storage_buffer(device, initial)
    }
}
impl LibrarySsbo for TextPrimSsbo {
    type Storage = TypedStorageBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        TypedStorageBuffer::new_storage_buffer(device, initial)
    }
}

impl SsboType for ShapeStreamSsbo {
    /// O：`get_rg_buffer<ShapeStreamSsbo>`（0x1412cbb70）里的 `SsboId` 立即数。
    fn rg_buffer_hash() -> u64 {
        0x069f_8bc2_1227_db88
    }
}
impl SsboType for TextPrimSsbo {
    /// O：`get_rg_buffer<TextPrimSsbo>`（0x1412d1b10）：`-0x1665dcb6219b5b0`。
    fn rg_buffer_hash() -> u64 {
        0xfe99_a234_9de6_4a50
    }
}

pub const UI_2D_SHADER: &str = "ui_2d";
pub const UI_2D_COLOR_BUTTON_SHADER: &str = "ui_2d_color_button";
pub const UI_2D_WS_SHADER: &str = "ui_2d_ws";
pub const UI_2D_PASS_NAME: &str = "UI 2D";
pub const MEI_TEXT_SHADER: &str = "text";
pub const MEI_SHADOW_SHADER: &str = "mei_shadow";
pub const MEI_PASS_NAME: &str = "Mei 2D";
pub const MEI_TEMP_NAME: &str = "mei temp";
pub const MEI_COMBINE_PASS_NAME: &str = "UI shadow combine";
/// `TextureDesc::new_2d` 的格式参数（O：`mov r8d, 0x2b`）。
pub const MEI_TEMP_FORMAT: u32 = 0x2b;

pub const U_POS_A: u64 = 0x120d_1fbd_3e93_b50f;
pub const U_POS_B: u64 = 0xa6fc_8d38_2b41_b9c5; // -0x590372c7d4be463b
pub const U_WINDOW: u64 = 0x69fd_ab9b_991c_4787;
pub const U_ALPHA: u64 = 0xeab2_a5da_cee8_22d6; // -0x154d5a253117dd2a
pub const U_FLAG: u64 = 0x0d8e_a64f_9bd8_2ef1;
/// `UiInactive` 存在性 uniform（O：`test r13,r13; setne`；原先误称 `U_BUTTON_EXTRA`）。
pub const U_INACTIVE: u64 = 0x3559_4af7_455f_3925;
pub const U_BUTTON_FLAG: u64 = 0x4c58_03d8_a552_6c67;
pub const U_WS_FLAG: u64 = 0xc5ab_673f_b086_51f5; // -0x3a5498c04f79ae0b
pub const IMAGE_BINDING: u64 = 0x7406_86c2_e74b_fe87;
pub const IMAGE_VERTEX_COUNT: u32 = 6;
pub const MEI_WINDOW_UNIFORM: u64 = 0x0a5a_0625_b786_0559;
pub const MEI_FONT_FILE: u64 = 0x579f_39ab_91a0_1865;
pub const MEI_FONT_BINDING: u64 = 0x815b_f503_6966_978c; // -0x7ea40afc96996874
pub const MEI_ATLAS_BINDING: u64 = 0x7539_91f4_2456_3352;
pub const MEI_TEXTPRIM_BINDING: u64 = 0xd7be_be7c_bc8b_02bf; // -0x284141834374fd41
pub const MEI_SHAPESTREAM_BINDING: u64 = 0x912f_62af_c233_33b3; // -0x6ed09d503dcccc4d
pub const MEI_TEMP_BINDING: u64 = 0x11ba_5697_cc40_b3dd;

/// Vec2 uniform（8 字节）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2Uniform(pub Vec2);
impl UniformValue for Vec2Uniform {
    fn write_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.0.x.to_le_bytes());
        out.extend_from_slice(&self.0.y.to_le_bytes());
    }
}
/// Vec3 uniform（12 字节）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3Uniform(pub glam::Vec3);
impl UniformValue for Vec3Uniform {
    fn write_bytes(&self, out: &mut Vec<u8>) {
        for c in [self.0.x, self.0.y, self.0.z] {
            out.extend_from_slice(&c.to_le_bytes());
        }
    }
}

/// `Ui2dImage` 被读取的部分：`+8/+0x10` 名字（&str）、`+0x18` 层号。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ui2dImageRef {
    pub name: String,
    pub layer: u32,
}

impl From<&crate::components::Ui2dImage> for Ui2dImageRef {
    fn from(image: &crate::components::Ui2dImage) -> Self {
        Self {
            name: image.name.clone(),
            layer: image.layer,
        }
    }
}

/// 视图描述（`view_type(1).base_layer(layer).layer_count(1, 1)`；A4 的 `TextureViewDesc` 是单元占位，这里只记参数）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageView {
    pub view_type: u32,
    pub base_layer: u32,
    pub layer_count: (u32, u32),
}

impl Ui2dImageRef {
    /// `NameHash::compute(name)`。
    pub fn file_hash(&self) -> u64 {
        NameHash::compute(&self.name).0
    }
    /// O：`view_type(1)`、`base_layer(layer)`、`layer_count(1, 1)`。
    pub fn view(&self) -> ImageView {
        ImageView {
            view_type: 1,
            base_layer: self.layer,
            layer_count: (1, 1),
        }
    }
}

/// `CustomizationIcon`（`resources::color_icon`）被读取的两个字段：`+0x18` f32、`+0x21` 字节（字段真名 U）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CustomizationIconRead {
    pub alpha: f32,
    pub flag: bool,
}

/// `UiInteractionState` 被读取的字节（`+2`；字段真名 U）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiInteractionStateRead {
    pub flag: bool,
}

/// `Query<(&Ui2dExtent, &Ui2dImage, Option<&CustomizationIcon>, Option<&UiInteractionState>, Option<&UiDrawOrder>,
/// Option<&UiInactive>)>` 一项（元组 0x30 B；类型名取自 PDB `QueryState::new` / `driftsort_main` 实例，O）。
#[derive(Clone, Debug, PartialEq)]
pub struct Ui2dDrawElement {
    /// `Ui2dExtent`：4 个 f32。
    pub extent: [f32; 4],
    pub image: Ui2dImageRef,
    /// Some uses color-button animation_t/is_original_color; None uses the
    /// plain image's darken_semitransparent/highlight uniforms. Verified against
    /// the shipped shader reflection and SPIR-V, including their FNV name keys.
    pub icon: Option<CustomizationIconRead>,
    /// 元素 `+0x18` = `Option<&UiInteractionState>`（color button 路径读取 `+2`；`None` 当 false，I）。
    pub interaction: UiInteractionStateRead,
    /// 元素 `+0x20` = `Option<&UiDrawOrder>`：排序键（`Option<&u64>`，`None` 最小）。
    pub order: Option<u64>,
    /// 元素 `+0x28` = `Option<&UiInactive>`：`test r13, r13; setne` → `U_INACTIVE`。
    pub inactive: bool,
}

/// 世界空间元素（`Ui2dWsExtent`：`+0/+4` Vec2、`+8` 起 Vec3）。
#[derive(Clone, Debug, PartialEq)]
pub struct Ui2dWsElement {
    pub extent: Vec2,
    pub position: glam::Vec3,
    pub image: Ui2dImageRef,
}

fn bind_image<'p>(b: DrawBuilder<'p>, image: &Ui2dImageRef) -> DrawBuilder<'p> {
    let view = crate::rhapsody_contract::TextureViewDesc::new()
        .view_type(1)
        .base_layer(image.layer)
        .layer_count(Some(1));
    b.bind_texture_to_draw_builder(
        BindTexture::FromDisk(TextureFromDisk(image.file_hash())),
        IMAGE_BINDING,
        ACCESS_SAMPLED_IMAGE,
        Some(&view),
    )
}

fn finish_draw(mut b: DrawBuilder<'_>) {
    let p = b.packet_mut();
    p.state = overlay_state(p.state);
    b.draw_no_vao(IMAGE_VERTEX_COUNT);
}

/// `ui_2d_pass(ctx, world, target, …)`；`window` = `Res<WindowSize>`；`elements` 为查询结果（未排序）。
pub fn ui_2d_pass(
    ctx: &mut DrawPassCtx<'_>,
    target: &AttachmentTexture,
    window: (u32, u32),
    elements: &[Ui2dDrawElement],
    ws_elements: &[Ui2dWsElement],
) {
    let ui = ctx
        .shader(UI_2D_SHADER)
        .expect("called `Option::expect()` on a `None` value");
    let button = ctx
        .shader(UI_2D_COLOR_BUTTON_SHADER)
        .expect("called `Option::expect()` on a `None` value");
    let ws = ctx
        .shader(UI_2D_WS_SHADER)
        .expect("called `Option::expect()` on a `None` value");
    let win = Vec2::new(window.0 as f32, window.1 as f32);
    // `sort_by`：稳定排序，键 = 元素 `+0x20` 的 `Option<&u64>`。
    let mut sorted: Vec<&Ui2dDrawElement> = elements.iter().collect();
    sorted.sort_by(|a, b| a.order.cmp(&b.order));
    let mut pass = ctx.raster_pass(
        UI_2D_PASS_NAME,
        &[ColorSpec::tag0(*target)],
        DepthSpec::None,
        *target,
    );
    for e in sorted {
        let state = pass.host.default_raster_state();
        let shader = if e.icon.is_some() {
            button.clone()
        } else {
            ui.clone()
        };
        let packet = DrawPacket::new(shader, [1, 0], state);
        let mut b = DrawBuilder::new(&mut pass, packet, 0)
            .uniform(U_POS_A, Vec2Uniform(Vec2::new(e.extent[0], e.extent[1])))
            .uniform(U_POS_B, Vec2Uniform(Vec2::new(e.extent[2], e.extent[3])))
            .uniform(U_WINDOW, Vec2Uniform(win));
        b = match e.icon {
            Some(t) => b.uniform(U_ALPHA, t.alpha).uniform(U_FLAG, t.flag),
            None => b
                .uniform(U_INACTIVE, e.inactive)
                .uniform(U_BUTTON_FLAG, e.interaction.flag),
        };
        finish_draw(bind_image(b, &e.image));
    }
    for e in ws_elements {
        let state = pass.host.default_raster_state();
        let packet = DrawPacket::new(ws.clone(), [1, 0], state);
        let b = DrawBuilder::new(&mut pass, packet, 0)
            .uniform(U_WS_FLAG, 1u32)
            .uniform(U_POS_A, Vec3Uniform(e.position))
            .uniform(U_POS_B, Vec2Uniform(e.extent))
            .uniform(U_WINDOW, Vec2Uniform(win));
        finish_draw(bind_image(b, &e.image));
    }
    pass.render();
}

/// `TextRenderer::render` 的产出（外部库 `tg_text`，只取被 `mei_pass` 使用的部分）。
#[derive(Clone, Debug)]
pub struct MeiTextOutput {
    /// 文本图元数 `n`（`draw_no_vao(6 * n)`）。
    pub prim_count: u32,
    /// 字形图集（原生纹理）。
    pub atlas: ManagedHandle,
    pub atlas_desc: AttachmentTexture,
}

/// `mei_pass`：`temp` = `RenderGraph::create_raw_resource("mei temp")` 创建出的 RG 纹理（由调用方经图录制器给出）。
pub fn mei_pass(
    ctx: &mut DrawPassCtx<'_>,
    target: &AttachmentTexture,
    temp: &AttachmentTexture,
    window: (u32, u32),
    text: &MeiTextOutput,
) {
    mei_pass_with_shadows(ctx, target, Some(temp), window, text);
}

/// Native cast_ui_shadows argument: without it MEI writes directly to the
/// existing target and does not allocate/read a shadow intermediate.
pub fn mei_pass_with_shadows(
    ctx: &mut DrawPassCtx<'_>,
    target: &AttachmentTexture,
    temp: Option<&AttachmentTexture>,
    window: (u32, u32),
    text: &MeiTextOutput,
) {
    let text_shader = ctx
        .shader(MEI_TEXT_SHADER)
        .expect("called `Option::expect()` on a `None` value");
    let mut pass = ctx.raster_pass(
        MEI_PASS_NAME,
        &[temp.map_or_else(|| ColorSpec::tag0(*target), |temp| ColorSpec::tag1(*temp))],
        DepthSpec::None,
        temp.copied().unwrap_or(*target),
    );
    let state = pass.host.default_raster_state();
    let packet = DrawPacket::new(text_shader, [1, 0], state);
    let mut b = DrawBuilder::new(&mut pass, packet, 0)
        .uniform(
            MEI_WINDOW_UNIFORM,
            Vec2Uniform(Vec2::new(window.0 as f32, window.1 as f32)),
        )
        .bind_texture_to_draw_builder(
            BindTexture::FromDisk(TextureFromDisk(MEI_FONT_FILE)),
            MEI_FONT_BINDING,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .bind_texture_to_draw_builder(
            BindTexture::Raw {
                texture: &text.atlas,
                desc: &text.atlas_desc,
            },
            MEI_ATLAS_BINDING,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .read_storage_buffer::<TextPrimSsbo>("TextPrim", MEI_TEXTPRIM_BINDING)
        .read_storage_buffer::<ShapeStreamSsbo>("ShapeStream", MEI_SHAPESTREAM_BINDING);
    let p = b.packet_mut();
    p.state = overlay_state(p.state);
    b.draw_no_vao(6 * text.prim_count);
    pass.render();

    let Some(temp) = temp else { return; };
    let shadow = ctx
        .shader(MEI_SHADOW_SHADER)
        .expect("called `Option::expect()` on a `None` value");
    let host = ctx.host();
    raster_single_object::<1, _>(
        MEI_COMBINE_PASS_NAME,
        &[ColorAttachment::Texture(*target)],
        shadow,
        host,
        |b| {
            let mut b = b
                .bind_texture_to_draw_builder(
                    BindTexture::FromDisk(TextureFromDisk(MEI_FONT_FILE)),
                    MEI_FONT_BINDING,
                    ACCESS_SAMPLED_IMAGE,
                    None,
                )
                .bind_texture_to_draw_builder(
                    BindTexture::RgHandle(temp),
                    MEI_TEMP_BINDING,
                    ACCESS_SAMPLED_IMAGE,
                    None,
                );
            let p = b.packet_mut();
            p.state = overlay_state(p.state);
            b
        },
    );
}

#[cfg(test)]
mod native_ssbo_constructor_tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::render::ssbo::{rhapsody_boundary::slice_as_bytes, tests::MockDevice};

    #[test]
    fn original_text_and_shape_factories_use_real_library_buffers_without_none_upload() {
        let device = MockDevice::default();
        let library = AssetSsboLibrary::new();
        library.add_new::<TextPrimSsbo>(&device);
        library.add_new::<ShapeStreamSsbo>(&device);
        assert_eq!(
            library.get::<TextPrimSsbo>().unwrap().buffer.byte_size,
            0x600000
        );
        assert_eq!(
            library.get::<ShapeStreamSsbo>().unwrap().buffer.byte_size,
            0x400000
        );
        let rows = device.created.borrow();
        assert_eq!(rows.len(), 2);
        for (row, (name, size)) in rows
            .iter()
            .zip([("TextPrim", 0x600000), ("ShapeStream", 0x400000)])
        {
            assert_eq!(
                (
                    row.1.as_str(),
                    row.0.size,
                    row.0.usage,
                    row.0.memory,
                    row.0.alignment
                ),
                (name, size, 0x22, 1, 4)
            );
            assert!(
                row.2.is_none(),
                "original None must not manufacture zero contents"
            );
        }
        assert_eq!(
            TextPrimSsbo::rg_buffer_hash(),
            utils::name_hash::NameHash::compute("TextPrim").0
        );
        assert_eq!(
            ShapeStreamSsbo::rg_buffer_hash(),
            utils::name_hash::NameHash::compute("ShapeStream").0
        );
        assert_eq!(core::mem::size_of::<TextPrimSsbo>(), 48);
        assert_eq!(core::mem::align_of::<TextPrimSsbo>(), 4);
        assert_eq!(core::mem::size_of::<ShapeStreamSsbo>(), 4);
    }

    #[test]
    fn text_primitive_reuses_original_member_offsets_and_forwards_real_tg_text_packet() {
        use core::mem::offset_of;
        assert_eq!(offset_of!(tg_text::Primitive, pos_min), 0);
        assert_eq!(offset_of!(tg_text::Primitive, pos_extent), 8);
        assert_eq!(offset_of!(tg_text::Primitive, uv_min), 16);
        assert_eq!(offset_of!(tg_text::Primitive, uv_extent), 24);
        assert_eq!(offset_of!(tg_text::Primitive, color), 32);
        assert_eq!(offset_of!(tg_text::Primitive, stream), 36);
        assert_eq!(offset_of!(tg_text::Primitive, padding), 40);
        let primitive = TextPrimSsbo(tg_text::Primitive::rect([2., 3.], [4., 5.], 0x12345678));
        let device = MockDevice::default();
        let initial = [primitive];
        let bytes = slice_as_bytes(&initial);
        let storage = <TextPrimSsbo as LibrarySsbo>::new_storage_buffer(&device, Some(bytes));
        assert_eq!(storage.buffer.byte_size, 0x600000);
        assert_eq!(device.created.borrow()[0].2.as_deref(), Some(bytes));
    }
}
