//! `mouse_read_terrain_stroke_index.rs` — W6 mouse terrain-stroke index read (0x1409768b0).
//!
//! URI `/read_terrain_stroke.cs.hlsl`. `CursorTerrainRaycast::position` +
//! read_texture + push_constants + dispatch (O). `MouseTerrainRaycastSsbo`
//! (O name; layout U).

use crate::typed_gap::Gap;

pub struct MouseTerrainRaycastSsbo(pub Gap);

pub fn mouse_read_terrain_stroke_index(world: &mut Gap, _cursor: &Gap) {
    let _ = world;
    unimplemented!()
}
