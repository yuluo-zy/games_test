//! `ui_pass.rs` — UI pass + loading-screen pass (final graph).
//!
//! `ui_pass` / `loading_screen_pass`: `RasterPass::new` + `CommonDrawParams::new` +
//! iterate → `to_draw_packet` + `set_blend_mode` (O). Shaders: `/ui_2d.raster.hlsl`,
//! `/loading_screen.raster.hlsl`.

pub fn ui_pass(world: &mut crate::typed_gap::Gap, _lists: &crate::typed_gap::Gap) {
    let _ = world;
    unimplemented!()
}

pub fn loading_screen_pass(world: &mut crate::typed_gap::Gap, _lists: &crate::typed_gap::Gap) {
    let _ = world;
    unimplemented!()
}
