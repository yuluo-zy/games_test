//! `startup/startup_terrain.rs` — terrain grid startup (index generation +
//! terrain plane entity).
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_terrain.rs`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `grid_gen_cw` @0x140928100 — generates a triangle-strip grid index buffer
//!   (CW winding): per cell 6 `u32` indices
//!   `{x+i, x+j, x+i+1, x+i+1, x+j, x+k}` with row-stride offsets; recursion
//!   chunks `(end-start)` by 4. `grid_gen_ccw` @0x140928350 is the CCW twin.
//!   Exact stride semantics I (the +1 cell step is O).
//! - `startup_terrain` @0x140926bf0 — bumps the transient-mesh instance
//!   counter, `grid_gen_cw`, `load_mesh_into_library`, `load_shader_into_library`,
//!   spawns a `(KeepOnLevelUnload, DrawableMeshBundle)` terrain entity; in demo
//!   mode (`_DEV_OVERRIDE_IS_DEMO_MODE`) additionally a
//!   `(KeepOnLevelUnload, DrawableMeshBundle, DesignatedRenderPass)` entity
//!   (pass selector 0x10).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `grid_gen_cw` @0x140928100 (O): append 6 CW triangle indices per grid cell.
/// `start`..`end` = X range, `rows` = row count, `stride` = row stride
/// (I: `start+stride+1` offset pattern observed).
pub fn grid_gen_cw(indices: &mut Vec<u32>, start: u32, end: u32, rows: u32, stride: u32) {
    for row in 0..rows {
        for col in 0..(end - start) {
            let base = start + col;
            let next = base + stride;
            // CW quad: {tl, bl, tr, tr, bl, br} (exact corner order I).
            indices.push(base);
            indices.push(next);
            indices.push(base + 1);
            indices.push(base + 1);
            indices.push(next);
            indices.push(next + 1);
        }
    }
}

/// `grid_gen_ccw` @0x140928350 (O): CCW twin.
pub fn grid_gen_ccw(indices: &mut Vec<u32>, start: u32, end: u32, rows: u32, stride: u32) {
    for row in 0..rows {
        for col in 0..(end - start) {
            let base = start + col;
            let next = base + stride;
            indices.push(base);
            indices.push(base + 1);
            indices.push(next);
            indices.push(next);
            indices.push(base + 1);
            indices.push(next + 1);
        }
    }
}

/// `startup_terrain` @0x140926bf0 (O): build + spawn the terrain plane.
pub fn startup_terrain(_gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover startup_terrain (grid_gen + DrawableMeshBundle spawn + demo-mode DesignatedRenderPass)")
}

/// `startup_terrain::generate_grid_cw` (O inline host of `startup_terrain`).
pub fn generate_grid_cw(indices: &mut Vec<u32>) {
    grid_gen_cw(indices, 0, 0, 0, 1);
}
