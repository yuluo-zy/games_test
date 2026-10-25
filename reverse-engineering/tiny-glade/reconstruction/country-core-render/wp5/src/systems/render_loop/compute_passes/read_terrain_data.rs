//! `read_terrain_data.rs` — W5 CPU terrain readback (0x14098c680).
//!
//! URIs (O): `/terrain_editing/read_terrain_height.cs.hlsl`,
//! `/terrain_editing/read_terrain_rock_mask.cs.hlsl`, `/water/read_water_mask.cs.hlsl`,
//! `/garden/read_garden_mask.cs.hlsl`. Each dispatch_1d; `read_terrain_data`
//! composes them + `ScheduledTerrainReadback::on_gpu_copy_issued` (O).

pub fn read_terrain_data(world: &mut crate::typed_gap::Gap, _terrain: &crate::typed_gap::Gap) {
    let _ = world;
    unimplemented!()
}

fn read_terrain_height(_world: &crate::typed_gap::Gap, _height: &crate::typed_gap::Gap) {
    // O: bind_terrain_heightmap + dispatch_1d.
    unimplemented!()
}

fn read_terrain_rock_mask(_world: &crate::typed_gap::Gap, _mask: &crate::typed_gap::Gap) {
    // O: read_texture + dispatch_1d.
    unimplemented!()
}

fn read_water_mask(_world: &crate::typed_gap::Gap, _mask: &crate::typed_gap::Gap) {
    // O: read_texture + dispatch_1d.
    unimplemented!()
}

fn read_garden_mask(_world: &crate::typed_gap::Gap, _mask: &crate::typed_gap::Gap) {
    // O: read_texture + dispatch_1d.
    unimplemented!()
}
