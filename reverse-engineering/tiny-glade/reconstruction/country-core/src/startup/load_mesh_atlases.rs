//! `startup/load_mesh_atlases.rs` — mesh-atlas generation (nani atlases).
//!
//! Source: B skeleton `crates/country-core/src/startup/load_mesh_atlases.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `load_mesh_atlases` @0x14129f400 / `reload_mesh_atlases` @0x1412a0820 —
//!   builds the nani mesh atlases + `load_nani_meshes` @0x1415f5b60.
//! - `generate_roof_floor_meshes` / `generate_curved_halftimber_meshes` /
//!   `generate_plaster_mesh` / `generate_stairs_floor_mesh` (inline) —
//!   procedural atlas parts (FlatMap over `Range<u32>` point arrays).
//! - `MeshConfig` / `MeshAttribAdd` (O named; ron-deserialized atlas configs).

#![allow(dead_code, unused_imports)]

use serde::Deserialize;

use crate::Gap;

/// `MeshConfig` (O named; ron atlas-config row).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct MeshConfig {
    pub _gap: Gap,
}

/// `MeshAttribAdd` (O named; attribute-add instruction).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct MeshAttribAdd {
    pub _gap: Gap,
}

pub fn load_mesh_atlases(_gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover load_mesh_atlases")
}

pub fn reload_mesh_atlases(_gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover reload_mesh_atlases")
}

pub fn load_nani_meshes(_gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover load_nani_meshes")
}
