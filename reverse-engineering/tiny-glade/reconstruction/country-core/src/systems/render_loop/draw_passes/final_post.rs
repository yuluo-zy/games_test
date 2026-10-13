//! `final_post.rs` — final post (tonemap/grading/LUT) for the final graph.
//!
//! `final_post`: `ImageFormationConfigs::current` + `ColorGradingState::flattened`
//! + `RenderGraphRecorder::create`; closures bind grading LUTs via
//! `GradingLut::lut_name_to_asset_name` + read_texture + uniform (O, lines 120/244).
//! `resolve_to_swapchain`: `Handle::desc` eq + `get_resource`;
//! `_blit_resolve_to_swapchain` / `_blit_resolve_msaa_to_swapchain` /
//! `_blit_blit_stretch_decode_srgb`. Shader: `_final_post.raster`
//! (tonemap / grading_lut / icc_lut / tony_mc_mapface_lut / shadow_mask_mean).
//! Final graph execute belongs to tiny_glade::render_loop (R12–R14).

use crate::typed_gap::Gap;

/// Final-post resource bundle (O name; layout U).
pub struct FinalPostResources(pub Gap);

/// W? final graph — post-process the main image (O).
pub fn final_post(world: &mut Gap, _resources: &FinalPostResources, _grading: &Gap) {
    let _ = world;
    unimplemented!()
}

/// Resolve the post result to the swapchain image (O).
pub fn resolve_to_swapchain(world: &mut Gap, _acquired: &Gap) {
    let _ = world;
    unimplemented!()
}
