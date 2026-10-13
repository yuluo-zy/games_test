//! `stairs.rs` — W20 stair generation (0x140a68a50).
//!
//! URIs (O): `/rocky_terrain/stairs/stairs.cs.hlsl` (compute) +
//! `_stairs_indirect.raster` (draw). uniform + `bind_terrain_heightmap` +
//! read_texture + write_draw_indirect_cmd_buffer + read_draw_indirect_cmd_buffer +
//! dispatch_2d + dispatch (O). `StairsTransformSsbo` (O name; layout U).

use crate::typed_gap::Gap;

pub struct StairsTransformSsbo(pub Gap);

pub fn stairs(world: &mut Gap, _rocky: &Gap) {
    let _ = world;
    unimplemented!()
}
