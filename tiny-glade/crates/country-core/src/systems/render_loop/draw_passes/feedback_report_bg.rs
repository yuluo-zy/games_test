//! `feedback_report_bg.rs` — 反馈报告背景 pass（0x1408a5a70，1231 B）。着色器 `blit_stretch_decode_srgb`
//! （`/logo_overlay.raster.hlsl` 同一路径族，名字取自字符串池）。
//!
//! O 顺序：
//! 1. 资源 A（TypeId -0x2c72519e…）首个 dword `!= 8` → 直接返回（O：`cmp dword [rax], 8; jne ret`）；
//! 2. 资源 B（反馈报告状态，TypeId -0x48166b2a…）：`+0x44` 字节 `!= 1` → 返回；`+0x3e` 字节 `>= 7` → 返回；
//! 3. `Res<AssetShaderLibrary>` → `get_raster_handle_by_name("blit_stretch_decode_srgb"(0x18))`，未命中 `expect` panic（feedback_report_bg.rs:77）；
//! 4. `RasterPass::new_impl("Feedback report bg"(0x12), [tag2 = target], 无深度, 视口 = target extent)`；
//! 5. `DrawPacket::new(head = [0, …], shader)` → `DrawBuilder::new(variant 0)`（**不调用 `bind_common`**）→
//!    `bind_texture_to_draw_builder(&Texture = B+8（原生纹理）, 绑定名 0x11ba5697cc40b3dd, 访问 0x1d, 无 view)` →
//!    状态 `byte0 &= 0x7f` → `draw_no_vao(3)` → `render`。

use crate::render::pass_builder::draw_builder::{ACCESS_SAMPLED_IMAGE, BindTexture, DrawBuilder};
use crate::render::pass_builder::draw_stream::DrawPacket;
use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::rhapsody_contract::ManagedHandle;
use crate::systems::render_loop::draw_passes::d_main_support::{ColorSpec, DepthSpec, DrawPassCtx};

pub const SHADER_NAME: &str = "blit_stretch_decode_srgb";
pub const PASS_NAME: &str = "Feedback report bg";
pub const BG_TEXTURE_BINDING: u64 = 0x11ba_5697_cc40_b3dd;
/// 资源 A 的门值（O：`cmp dword [rax], 8`）。
pub const GATE_STATE: u32 = 8;
/// 资源 B：`+0x3e` 页号上限（O：`cmp byte [r15+0x3e], 7; jae ret`）。
pub const MAX_PAGE_EXCLUSIVE: u8 = 7;

/// 反馈报告状态（资源 B，只含被读取的字段）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug)]
pub struct FeedbackReportBg {
    /// `+0x44`：必须 == 1。
    pub visible: u8,
    /// `+0x3e`：必须 < 7。
    pub page: u8,
    /// `+0x08` 起的原生纹理（句柄 + 描述）。
    pub texture: ManagedHandle,
    pub texture_desc: AttachmentTexture,
}

/// `render_feedback_report_bg(ctx, world, target)`；`state` = 资源 A 的 dword，`report` = 资源 B。
pub fn render_feedback_report_bg(
    ctx: &mut DrawPassCtx<'_>,
    target: &AttachmentTexture,
    state: u32,
    report: &FeedbackReportBg,
) {
    if state != GATE_STATE || report.visible != 1 || report.page >= MAX_PAGE_EXCLUSIVE {
        return;
    }
    let shader = ctx
        .shader(SHADER_NAME)
        .expect("called `Option::expect()` on a `None` value");
    let mut pass = ctx.raster_pass(
        PASS_NAME,
        &[ColorSpec::tag2(*target)],
        DepthSpec::None,
        *target,
    );
    let state = pass.host.default_raster_state();
    let packet = DrawPacket::new(shader, [0, 0], state);
    let mut builder = DrawBuilder::new(&mut pass, packet, 0).bind_texture_to_draw_builder(
        BindTexture::Raw {
            texture: &report.texture,
            desc: &report.texture_desc,
        },
        BG_TEXTURE_BINDING,
        ACCESS_SAMPLED_IMAGE,
        None,
    );
    let p = builder.packet_mut();
    p.state = p.state.clear_byte0_bit7();
    builder.draw_no_vao(3);
    pass.render();
}
