//! `transparency_pass.rs` — W44 refraction copy, W45 water/ice surface,
//! W46 pre-TAA transparency, W53 post-TAA transparency.
//!
//! Addresses (O): 0x1409bf320 (refraction_copy_pass), 0x1409bf800
//! (water_surface_pass), 0x1409c08d0 (ice_surface_pass), 0x1409c3a80
//! (pre_taa_transparency_pass), 0x1409c40b0 (post_taa_transparency_pass).
//! Branches: refraction + water/ice gated by `\*0x14362c098 == 0`; water vs ice by
//! `is_winter` (frame-wiring §3).
//! Water URIs (O): `/water/{deferred_rt, calc_deferred_rt_args}.cs.hlsl` +
//! `_water_plane.raster` + `_water_plane_refraction_copy`/`_waterfall_refraction_copy`.
//! Ice URIs (O): `/water/ice_{deferred_rt, refl_march, refl_resolve, refr_resolve}.cs.hlsl`
//! + `_ice_plane` + `/water/ice_cracks.raster.hlsl`. Both use dispatch_indirect_raw +
//! bind_ray_tracing (software BVH).

use crate::consume::core::RtBvhSsbo;
use crate::typed_gap::Gap;

/// W44 — copy refracted water plane (O).
pub fn refraction_copy_pass(world: &mut Gap, _view: &Gap, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}

/// W45 — non-winter water surface: raster plane + deferred-RT compute +
/// indirect dispatch args (O).
pub fn water_surface_pass(world: &mut Gap, _view: &Gap, _bvh: &RtBvhSsbo, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}

/// W45 — winter ice surface: deferred RT + refl/refl_resolve/refr_resolve compute
/// + ice cracks raster (O).
pub fn ice_surface_pass(world: &mut Gap, _view: &Gap, _bvh: &RtBvhSsbo, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}

/// W46 — pre-TAA transparency (window glass / cloth, blend).
pub fn pre_taa_transparency_pass(world: &mut Gap, _view: &Gap, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}

/// W53 — post-TAA transparency (`Vec::is_empty` guard).
pub fn post_taa_transparency_pass(world: &mut Gap, _view: &Gap, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}
