//! `compute_garden.rs` — W12 garden_rasterize_from_cpu, W16 garden_spawn_vegetation.
//!
//! URIs (O): `/garden/marching_squares.cs.hlsl` (rasterize), `/garden/spawn_{
//! bushes_garden, flowers_garden, small_flowers_garden}.cs.hlsl` (spawn).
//! W12 (0x140a22140): `on_texture_processed` + uniform + read_texture +
//! write_texture + dispatch_2d. W16 (0x140a22b80): `bind_terrain_heightmap` +
//! write_draw_indirect_cmd_buffer + dispatch/dispatch_2d (O).

use crate::typed_gap::Gap;

/// CPU-visible garden instance SSBOs (O names; layout U, `resources::` domain).
pub struct BushSsbo(pub Gap);
pub struct FlowerSsbo(pub Gap);
pub struct Flower03Ssbo(pub Gap);
pub struct GardenStoneSsbo(pub Gap);

pub fn garden_rasterize_from_cpu(world: &mut Gap, _garden: &Gap) {
    let _ = world;
    unimplemented!()
}

pub fn garden_spawn_vegetation(world: &mut Gap, _garden: &Gap) {
    let _ = world;
    unimplemented!()
}
