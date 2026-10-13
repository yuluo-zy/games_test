//! `custom_cursor_pass.rs` — 自定义光标 pass（0x1409d1710，2455 B）。着色器 `custom_cursor`（`/custom_cursor.raster.hlsl`）。
//!
//! O 顺序：puffin scope → `Res<AssetShaderLibrary>` → `get_raster_handle_by_name("custom_cursor"(0xd))`（未命中 `expect` panic）→
//! `Res<WindowSize>`（`+0` 宽 u32、`+4` 高 u32；`as_vec2`）→ 光标资源（TypeId 0x3d7b56e7…）：**`+0xf4` 字节为 0 则整个 pass 不执行**
//! （`je → 结尾`）→ `extent_2d(target)` → `RasterPass::new_impl("Custom cursor"(0xd), [tag0 = target], 无深度, 视口 = target)`
//! → `DrawPacket::new(head = [0, …])`/`DrawBuilder::new(variant 0)` →
//! `write_uniform<[u32;2]>(0x0a5a0625b7860559, [w, h])`（`Vec2` → `f32 as u32` 饱和转换：`cvttss2si` + 与 0 / 4294967040.0 比较的 cmov；
//! 构建器常量块为 `None` 时跳过）→ `bind_texture_to_draw_builder(TextureFromDisk(0xf9f14d14bcc4e780), 绑定名 0x9f0d228ee3dba429, 访问 0x1d, 无 view)` →
//! `read_storage_buffer<CustomCursorSsbo>(槽 "CustomCursorSsbo", 0x19ce7a8f29cef00d)` → 状态 `byte0 &= 0x7f`、
//! `byte3 = (byte3 & 0x9f) | 0x40` → `draw_no_vao(6)` → `render`。

use crate::render::pass_builder::draw_builder::{ACCESS_SAMPLED_IMAGE, BindTexture, DrawBuilder};
use crate::render::pass_builder::draw_stream::{DrawPacket, RasterState};
use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::resources::texture_presets::TextureFromDisk;
use crate::rhapsody_contract::{Handle, ManagedHandle};
use crate::systems::render_loop::draw_passes::d_main_support::{ColorSpec, DepthSpec, DrawPassCtx};

/// 光标 SSBO（O：`read_access_buffer<CustomCursorSsbo>` 0x140971a20；布局 U）。
pub struct CustomCursorSsbo(pub crate::typed_gap::Gap);

impl SsboType for CustomCursorSsbo {
    /// O：`get_rg_buffer<CustomCursorSsbo>`（0x1412c7550）里的 `SsboId` 立即数 `-0x2f0f7d81800dd867`。
    fn rg_buffer_hash() -> u64 {
        0xd0f0_827e_7ff2_2799
    }
}

pub const SHADER_NAME: &str = "custom_cursor";
pub const PASS_NAME: &str = "Custom cursor";
pub const WINDOW_SIZE_UNIFORM: u64 = 0x0a5a_0625_b786_0559;
pub const CURSOR_TEXTURE_FILE: u64 = 0xf9f1_4d14_bcc4_e780;
pub const CURSOR_TEXTURE_BINDING: u64 = 0x9f0d_228e_e3db_a429;
pub const CURSOR_SSBO_BINDING: u64 = 0x19ce_7a8f_29ce_f00d;
pub const CURSOR_SSBO_SLOT: &str = "CustomCursorSsbo";
pub const CURSOR_VERTEX_COUNT: u32 = 6;

/// `WindowSize::as_vec2` → `write_uniform<[u32;2]>` 的饱和转换（O：`cvtsi2ss` 后 `cvttss2si` + cmov 夹到 `[0, u32::MAX]`）。
pub fn window_uniform(width: u32, height: u32) -> [u32; 2] {
    [(width as f32) as u32, (height as f32) as u32]
}

/// 状态改写（O：`and byte [+0xa8], 0x7f; byte3 = (byte3 & 0x9f) | 0x40`），UI 叠加类 pass 共用。
pub fn overlay_state(state: RasterState) -> RasterState {
    RasterState(state.clear_byte0_bit7().0).set_byte3_field_2()
}

/// `custom_cursor_pass`。`window` = `Res<WindowSize>`，`cursor_visible` = 光标资源 `+0xf4` 字节是否非 0。
pub fn custom_cursor_pass(
    ctx: &mut DrawPassCtx<'_>,
    target: &AttachmentTexture,
    window: (u32, u32),
    cursor_visible: bool,
) {
    let shader = ctx
        .shader(SHADER_NAME)
        .expect("called `Option::expect()` on a `None` value");
    if !cursor_visible {
        return;
    }
    let mut pass = ctx.raster_pass(PASS_NAME, &[ColorSpec::tag0(*target)], DepthSpec::None, *target);
    let state = pass.host.default_raster_state();
    let packet = DrawPacket::new(shader, [0, 0], state);
    let [w, h] = window_uniform(window.0, window.1);
    let mut builder = DrawBuilder::new(&mut pass, packet, 0)
        .uniform(WINDOW_SIZE_UNIFORM, [w, h])
        .bind_texture_to_draw_builder(
            BindTexture::FromDisk(TextureFromDisk(CURSOR_TEXTURE_FILE)),
            CURSOR_TEXTURE_BINDING,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .read_storage_buffer::<CustomCursorSsbo>(CURSOR_SSBO_SLOT, CURSOR_SSBO_BINDING);
    let p = builder.packet_mut();
    p.state = overlay_state(p.state);
    builder.draw_no_vao(CURSOR_VERTEX_COUNT);
    pass.render();
}

#[allow(dead_code)]
fn _unused(_: Handle<crate::render::shader::RasterShaderProgram>, _: ManagedHandle) {}
