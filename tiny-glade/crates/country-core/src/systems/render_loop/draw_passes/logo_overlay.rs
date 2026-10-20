//! `logo_overlay.rs` — 启动 logo 叠加（`render_logo_overlay` 0x1408a5f50，1151 B；`LogoOverlayPlugin::build` 0x1408a5f40，8 B）。
//!
//! O：`Res<AssetShaderLibrary>` → `get_raster_handle_by_name("logo_overlay"(0xc))`（未命中 `expect`，logo_overlay.rs:71）→
//! `extent_2d(target)` → `RasterPass::new_impl("Logo overlay"(0xc), [tag0 = target], 无深度, 视口 = target)` →
//! `DrawPacket::new` / `DrawBuilder::new(variant 0)` → `optional_uniform<[u32;2]>(0x531bb9318eea4971 = 视口名, extent_2d(target))` →
//! `bind_texture_to_draw_builder(TextureFromDisk(0x75b4be0d27cfedd3), 绑定名 0x11ba5697cc40b3dd, 访问 0x1d, 无 view)` →
//! 状态 `byte0 &= 0x7f`、`byte3 = (byte3 & 0x9f) | 0x40` → `draw_no_vao(6)` → `render`。
//! `build`：8 B 尾调用 `SubApp::add_systems`（Startup）注册 `logo_overlay::startup` 0x1414b4b10，它内联
//! `"/logo_overlay.raster.hlsl"` + `"logo_overlay"` 的着色器注册；宿主侧由 `load_shaders::COMMON_SHADERS` 承担。

use crate::render::pass_builder::draw_builder::{ACCESS_SAMPLED_IMAGE, BindTexture, DrawBuilder};
use crate::render::pass_builder::draw_stream::DrawPacket;
use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::resources::texture_presets::TextureFromDisk;
use crate::systems::render_loop::draw_passes::custom_cursor_pass::overlay_state;
use crate::systems::render_loop::draw_passes::d_main_support::{ColorSpec, DepthSpec, DrawPassCtx};

pub const SHADER_NAME: &str = "logo_overlay";
pub const PASS_NAME: &str = "Logo overlay";
pub const VIEWPORT_UNIFORM: u64 = 0x531b_b931_8eea_4971;
pub const LOGO_TEXTURE_FILE: u64 = 0x75b4_be0d_27cf_edd3;
pub const LOGO_TEXTURE_BINDING: u64 = 0x11ba_5697_cc40_b3dd;
pub const LOGO_VERTEX_COUNT: u32 = 6;

/// `LogoOverlayPlugin`（单元结构体；`build` 无动作）。
#[derive(Clone, Copy, Debug, Default)]
pub struct LogoOverlayPlugin;

impl LogoOverlayPlugin {
    /// O 0x1408a5f40：`MOV RCX,RDX; JMP 0x140b65940`（`add_systems<Startup, logo_overlay::startup>`）。
    /// 着色器注册见 `startup::load_shaders::COMMON_SHADERS`，此处不再重复。
    pub fn build(&self) {}
}

/// `render_logo_overlay(ctx, world, target)`。
pub fn render_logo_overlay(ctx: &mut DrawPassCtx<'_>, target: &AttachmentTexture) {
    let shader = ctx
        .shader(SHADER_NAME)
        .expect("called `Option::expect()` on a `None` value");
    let mut pass = ctx.raster_pass(
        PASS_NAME,
        &[ColorSpec::tag0(*target)],
        DepthSpec::None,
        *target,
    );
    let state = pass.host.default_raster_state();
    let packet = DrawPacket::new(shader, [0, 0], state);
    let mut builder = DrawBuilder::new(&mut pass, packet, 0)
        .optional_uniform(VIEWPORT_UNIFORM, [target.extent.0, target.extent.1])
        .bind_texture_to_draw_builder(
            BindTexture::FromDisk(TextureFromDisk(LOGO_TEXTURE_FILE)),
            LOGO_TEXTURE_BINDING,
            ACCESS_SAMPLED_IMAGE,
            None,
        );
    let p = builder.packet_mut();
    p.state = overlay_state(p.state);
    builder.draw_no_vao(LOGO_VERTEX_COUNT);
    pass.render();
}
