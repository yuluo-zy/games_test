//! `startup/clearing.rs` — the glade "clearing" region setup/teardown
//! (boundary trees/autoclutter around the buildable clearing).
//!
//! Source: B skeleton `crates/country-core/src/startup/clearing.rs`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `init_clearing` @0x140a78aa0 — reads `ActiveSession` glade-type byte
//!   @+0x58:
//!     - `== 4` (winter): loads a snow clearing mesh + shader, spawns a
//!       `DrawableMeshBundle` clearing plane;
//!     - `== 1` (olden): `glade_themes::olden::startup_olden()` +
//!       `insert_resource<PreplacedAutoClutter>`;
//!     - `== 2` (flowery): `glade_themes::flowery::startup_flowery()`;
//!     - `!= 1`: `remove_resource<PreplacedAutoClutter>`.
//!   Then loads clearing placements JSON → `PlacementDataSsbo` +
//!   `util_add_clearing_shader` (inline host) + tree-storage filters
//!   (`ForestTree` collection from `PlacementDataSsbo`).
//! - `uninit_clearing` @0x140a7a5b0 — clears the placement/SSBO maps, the
//!   `(RtMeshHandle, DirtyCow<RtMesh>)` map, the audio trigger slotmap
//!   (`AudioTriggerCollectionHandle -> Aabb3`), a 0x1440-byte grass/tree
//!   array (stride 0xc0), drops clearing-name `Arc<str>`s, and switches audio
//!   snapshot (to `"snapshot:/Loading"` when required).
//! - `PlacementConfigEntry` (O: serde deserialized placement config rows).

#![allow(dead_code, unused_imports)]

use serde::Deserialize;

use crate::Gap;

/// `PlacementConfigEntry` (O named; serde JSON placement config row).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct PlacementConfigEntry {
    pub _gap: Gap,
}

/// `init_clearing` @0x140a78aa0 (O).
pub fn init_clearing(_session: &Gap, _gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover init_clearing (winter mesh / olden+flowery themes / autoclutter resource / placements)")
}

/// `uninit_clearing` @0x140a7a5b0 (O).
pub fn uninit_clearing(_resources: &mut Gap) {
    todo!("recover uninit_clearing (map clears + audio snapshot switch)")
}

/// `util_add_clearing_shader` (O inline into `init_clearing`).
pub fn util_add_clearing_shader(_gpu: &Gap, _assets: &Gap) {
    todo!("recover util_add_clearing_shader")
}
