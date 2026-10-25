//! `wireframe_pass.rs` — debug wireframe pass.
//!
//! `RasterPass::new` + `CommonDrawParams::new` + iterate → `to_draw_packet` +
//! `set_polygon_mode` + `set_depth_test_enable` (O).

pub fn wireframe_pass(world: &mut crate::typed_gap::Gap, _lists: &crate::typed_gap::Gap) {
    let _ = world;
    unimplemented!()
}
