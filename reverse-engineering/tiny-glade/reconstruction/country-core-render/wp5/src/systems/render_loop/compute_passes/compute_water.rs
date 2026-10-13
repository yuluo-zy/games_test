//! `compute_water.rs` — W2 water_rasterize_from_cpu, W13 water_spawn_vegetation.
//!
//! URIs (O): `/water/*` (rasterize → `_water_draw_water_mask`); spawn →
//! `/water/spawn_{boats, lilypads, reeds, waterwheels}.cs.hlsl` (boats/wheels URI
//! statically absent — runtime-dynamic, U). W2 (0x140a12680): `on_texture_processed`
//! + uniform + read_texture + write_texture + dispatch_2d. W13 (0x140a13970):
//! `ActiveSession::is_autumn` + `bind_terrain_heightmap` + write_texture +
//! write_draw_indirect_cmd_buffer + dispatch (O).

use crate::typed_gap::Gap;

/// Water-vegetation instance SSBOs (O names; layout U, `resources::` domain).
pub struct LilypadSsbo(pub Gap);
pub struct LilypadFlowerSsbo(pub Gap);
pub struct ReedSsbo(pub Gap);

pub fn water_rasterize_from_cpu(world: &mut Gap, _water: &Gap) {
    let _ = world;
    unimplemented!()
}

pub fn water_spawn_vegetation(world: &mut Gap, _water: &Gap) {
    let _ = world;
    unimplemented!()
}
