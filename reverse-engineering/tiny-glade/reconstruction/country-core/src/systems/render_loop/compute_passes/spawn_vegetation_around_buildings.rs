//! `spawn_vegetation_around_buildings.rs` — W14 vegetation around buildings (0x1408e4cc0).
//!
//! URIs (O): `/garden/spawn_{bushes_buildings, flowers_buildings}.cs.hlsl`. uniform +
//! `bind_terrain_heightmap` + read_texture + write_draw_indirect_cmd_buffer +
//! dispatch (O).

pub fn spawn_vegetation_around_buildings(world: &mut crate::typed_gap::Gap, _buildings: &crate::typed_gap::Gap) {
    let _ = world;
    unimplemented!()
}
