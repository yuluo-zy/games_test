//! `visibility_queries.rs` — W31 visibility queries (0x140a2f2e0).
//!
//! visibility-query family (U exact shader). read_texture + `bind_terrain_heightmap`
//! + dispatch_1d (O).

use crate::typed_gap::Gap;

pub fn run_visibility_queries(world: &mut crate::typed_gap::Gap, _view: &Gap) {
    let _ = world;
    unimplemented!()
}
