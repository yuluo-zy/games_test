//! `main_pass.rs` — W25 main_pass, W30 main_pass_oc2, W34 late_forward_and_deferred_pass.
//!
//! Addresses (O): 0x140833bc0 / 0x140835a40 / 0x1408373b0.
//! `with_stencil_write` (line 22): `set_stencil_test_enable` + `set_stencil_pass_op`.
//! Each pass: puffin scope + `RasterPass::new` + `CommonDrawParams::new` +
//! iterate `DrawListItem` → `to_draw_packet` + `DrawBuilder::stencil_ref` +
//! `with_stencil_write`. main_pass additionally checks
//! `ShaderRefactoringState::disabled`; late_forward guards `impl$0::is_empty`.
//! Shaders: nani_* raster + `/deferred/*_light_sun_sky.raster` (W34).

use crate::consume::pass_builder::DrawBuilder;
use crate::consume::core::FrameConstants;
use crate::typed_gap::Gap;

/// Enable stencil write on the main pass (O, line 22).
fn with_stencil_write(draw: &mut DrawBuilder) {
    let _ = draw;
    unimplemented!()
}

/// W25 — main G-buffer pass: depth/albedo/velocity (O).
pub fn main_pass(world: &mut Gap, _frame: &FrameConstants, _view: &Gap, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}

/// W30 — main pass occlusion pass 2, post-HZB (O).
pub fn main_pass_oc2(world: &mut Gap, _frame: &FrameConstants, _view: &Gap, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}

/// W34 — lighting / deferred shading combination (O).
pub fn late_forward_and_deferred_pass(world: &mut Gap, _frame: &FrameConstants, _view: &Gap, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}
