//! `ui_2d_pass.rs` — final-graph UI 2D pass + MEI pass.
//!
//! `ui_2d_pass`: `WindowSize::as_vec2` + `World::query` (UI entities) + `sort_by` +
//! `RasterPass::new` + `DrawPacket::new` + uniform + `read_texture_view`/`read_texture`
//! + `set_depth_test_enable`/`set_blend_mode` (O). `mei_pass`: `Handle::desc` +
//! `RenderGraphRecorder::create` + same. URIs: `/ui_2d.raster.hlsl`, `/egui.raster.hlsl`,
//! `/text.raster.hlsl`, `/custom_cursor.raster.hlsl`. `ShapeStreamSsbo`/`TextPrimSsbo`
//! (O names; layout U).

use crate::typed_gap::Gap;

pub struct ShapeStreamSsbo(pub Gap);
pub struct TextPrimSsbo(pub Gap);

pub fn ui_2d_pass(world: &mut Gap, _window: &Gap, _ui: &Gap) {
    let _ = world;
    unimplemented!()
}

pub fn mei_pass(world: &mut Gap, _window: &Gap) {
    let _ = world;
    unimplemented!()
}
