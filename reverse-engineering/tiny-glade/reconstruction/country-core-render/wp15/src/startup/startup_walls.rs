//! `startup/startup_walls.rs` — default glade boundary walls.
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_walls.rs`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `glade_startup_walls` @0x140a92670 — loads the wall brick mesh, then
//!   allocates the **9 brick-source ids** `{ (7,5),(4,4),(6,7),(3,7),(6,6),
//!   (2,3),(1,5),(7,7),(3,1) }` (u32 pairs `0x500000007` … `0x100000003`),
//!   and spawns **7** `(DrawableMeshBundle, WallBrickInstancedDrawProxy[+OcclusionCulled])`
//!   entities with marker `{kind: u16 @+0x48, sub: u8 @+0x4a}`:
//!   `{0,0},{0,0},{0,1},{1,0},{1,1},{0x100,0},{0x100,1},{0x100,2}` — i.e.
//!   `kind`s `{0,0,0,1,1,0x100,0x100,0x100}` and `sub`s `{0,0,1,0,1,0,1,2}`
//!   (the decompile unrolls 7 spawns; the proxy list above is the O data).
//! - `create_half_cube_mesh` (O inline into `glade_startup_walls`).
//! - `once_startup_walls` (O: `TerrainHeightsData<TerrainHeightSsbo>` +
//!   `PathRaster` + commands).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `BrickSourceId` (O named entity, `geometry::instanced_wall`): a brick source
/// (u32 pair `{kind, id}`). O default list for the boundary walls:
/// `[(7,5),(4,4),(6,7),(3,7),(6,6),(2,3),(1,5),(7,7),(3,1)]`.
pub const GLADE_BOUNDARY_BRICK_SOURCES: [(u32, u32); 9] = [
    (7, 5),
    (4, 4),
    (6, 7),
    (3, 7),
    (6, 6),
    (2, 3),
    (1, 5),
    (7, 7),
    (3, 1),
];

/// O marker kinds for the 7 boundary-wall draw entities.
pub const GLADE_BOUNDARY_WALL_KINDS: [u16; 7] = [0, 0, 0, 1, 1, 0x100, 0x100];
/// O marker sub-indices (the 8th proxy in the list is `{0x100,2}`).
pub const GLADE_BOUNDARY_WALL_SUBS: [u8; 7] = [0, 0, 1, 0, 1, 0, 1];

/// `glade_startup_walls` @0x140a92670 (O).
pub fn glade_startup_walls(_gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover glade_startup_walls (9 brick sources -> 7 WallBrickInstancedDrawProxy entities)")
}

/// `create_half_cube_mesh` (O inline).
pub fn create_half_cube_mesh() -> Vec<u8> {
    todo!("recover create_half_cube_mesh")
}

/// `once_startup_walls` (O: one-shot wall-state startup).
pub fn once_startup_walls(_terrain: &Gap, _path_raster: &Gap, _commands: &mut Gap) {
    todo!("recover once_startup_walls")
}
