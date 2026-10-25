//! `systems/ui/debug_visualize_volumes.rs` — debug volume visualization.
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/debug_visualize_volumes.rs`.
//! PDB named entities: `DebugRoofs`, `DebugVolume`.
//! O evidence (Ghidra, B patched EXE):
//! - `debug_visualize_volumes` @0x1414558c0 — draws triangle volumes
//!   (`utils::geometry::intersect::Triangle` list → line/sphere debug viz).
//! - `debug_visualize_roof_uvs` @0x141454e60 (2 callers) — roof UV wireframe.
//! - `impl$0::register_required_components` (O bevy-ECS component registration).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `DebugRoofs` (O named entity; resource).
#[derive(Clone, Debug, Default)]
pub struct DebugRoofs(pub Gap);

/// `DebugVolume` (O named entity).
#[derive(Clone, Debug, Default)]
pub struct DebugVolume(pub Gap);

pub fn debug_visualize_volumes(_commands: &mut Gap, _triangles: &[crate::utils::Triangle]) {
    todo!("recover debug_visualize_volumes")
}

pub fn debug_visualize_roof_uvs(_commands: &mut Gap, _roofs: &DebugRoofs) {
    todo!("recover debug_visualize_roof_uvs")
}
