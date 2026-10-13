//! `path_pebbles.rs` — W17 path pebbles (0x140958f70).
//!
//! URI `/path_pebbles.cs.hlsl`. write_draw_indirect_cmd_buffer + uniform +
//! read_texture + `bind_terrain_heightmap` + dispatch (O). `PebblePlacementSsbo`
//! (O name; layout U).

use crate::typed_gap::Gap;

pub struct PebblePlacementSsbo(pub Gap);

pub fn path_pebbles_compute(world: &mut Gap, _pebbles: &Gap) {
    let _ = world;
    unimplemented!()
}
