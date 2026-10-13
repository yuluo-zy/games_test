//! `roof_tiles_gravity.rs` — W19 roof-tile gravity sim (0x140a29b10).
//!
//! URI `/roof_tiles_gravity.cs.hlsl` (+ `frame_constants_storage_buffer@1.1`, O).
//! `UnsafeWorldCell::get_resource` + `AssetSsboLibrary::get` + `World::query`
//! (RoofTiles components) + `Iterator::flat_map` + uniform (O). Per-tile
//! `PushConstants` dispatched 1D (closure$1, O).

use crate::typed_gap::Gap;

/// Per-tile gravity push constants (O name; fields U).
pub struct PushConstants(pub u32, pub u32);

pub fn roof_tiles_gravity_compute(world: &mut Gap, _roofs: &Gap) {
    let _ = world;
    unimplemented!()
}
