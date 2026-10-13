//! `grass.rs` — GPU grass generation (W29, address 0x140a60280).
//!
//! URIs (O): `/grass/{calc_tiles, tile_bundle_gen, tile_bundle_count, tiles_prefix,
//! bundles_prefix, calc_dispatch_args, calc_grasslet_bundle_ranges}.cs.hlsl` plus the
//! `_generate_grass` family. `dispatch_indirect_raw`; writes
//! DrawIndexedIndirectBuffer / DrawIndirectBuffer / AtomicCounterBuffer (O, sequence.md).
//! `inclusive_prefix_scan_u32_1m` is reused from `prefix_scan.rs` (O).

use crate::typed_gap::Gap;

/// Grass blade data (O name; layout U).
pub struct GrassBladeDataSsbo(pub Gap);
/// Object-grass push struct shared with draw_passes::object_grass_push (O name).
pub struct ObjectGrassPush(pub Gap);

/// Bevy component registration (O, line 107; Vec::push/pop).
pub fn register_required_components() {
    unimplemented!()
}

/// Precompute step (O, line 132): `bind_terrain_heightmap` + read_texture +
/// uniform + dispatch_1d/groups.
fn grass_precompute(_world: &mut Gap, _recorder: &mut Gap) {
    unimplemented!()
}

/// W29 entry: `grass_precompute` + `write_draw_indirect_cmd_buffer` +
/// uniform + read_texture; dispatch indirect (O).
pub fn grass_compute(world: &mut Gap, _hzb: &Gap) {
    let _ = world;
    unimplemented!()
}
