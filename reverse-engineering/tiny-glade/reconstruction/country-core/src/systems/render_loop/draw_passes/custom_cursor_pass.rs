//! `custom_cursor_pass.rs` — custom cursor pass.
//!
//! puffin scope + `WindowSize::as_vec2` + `utils::get_res` + `RasterPass::new` +
//! `DrawPacket::new` + uniform + read_texture + `set_depth_test_enable` +
//! `set_blend_mode` (O). `CustomCursorSsbo` (O name; layout U).
//! Shader: `/custom_cursor.raster.hlsl`.

use crate::typed_gap::Gap;

pub struct CustomCursorSsbo(pub Gap);

pub fn custom_cursor_pass(world: &mut Gap, _cursor: &Gap) {
    let _ = world;
    unimplemented!()
}
