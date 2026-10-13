//! `shadow_pass.rs` — W23 shadow-cascade raster (0x1409cc270).
//!
//! `RasterPass::new` + `DepthAttachment::layer` (shadow atlas layer) +
//! `CommonDrawParams::new` + iterate DrawListItem → `to_draw_packet` +
//! `RasterPipelineOpts::set_depth_compare_mode` (O). Shader URI:
//! `_mei_shadow.raster.*` (I: no literal `_shadow_pass` file).

use crate::typed_gap::Gap;

/// Rasterize the shadow atlas for the sun cascades (O).
pub fn shadow_pass(world: &mut Gap, _light: &crate::consume::core::DirectionalLight, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}
