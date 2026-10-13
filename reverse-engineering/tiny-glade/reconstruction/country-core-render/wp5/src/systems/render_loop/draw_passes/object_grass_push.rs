//! `object_grass_push.rs` — W9 object grass push pass (0x1409daaa0).
//!
//! puffin scope + `utils::get_res`/`get_res_mut` + `MainCamera::eye_position` +
//! `CursorTerrainRaycast::position` (closure$1: eye/cursor → `Vec3::extend` Vec4) +
//! `World::query` + `Iterator::chain` + `RasterPass::new` + `DrawPacket::new` +
//! uniform + `set_depth_test_enable`/`set_blend_mode` (O). Shader:
//! `_grass_object_push.raster` (O, sequence.md). `ObjectGrassPush` shared with
//! compute_passes::grass.

use crate::typed_gap::Gap;

pub fn object_grass_push_pass(world: &mut Gap, _grass: &Gap) {
    let _ = world;
    unimplemented!()
}
