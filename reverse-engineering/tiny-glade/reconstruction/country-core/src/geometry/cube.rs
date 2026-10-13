//! `geometry/cube.rs` — `Cube` / `Box` (axis-aligned cube) procedural mesh.
//!
//! Source: B skeleton `crates/country-core/src/geometry/cube.rs`.
//! PDB named entities: `Box`, `Cube`.
//! O evidence (Ghidra, B patched EXE): `Box -> Mesh::from` @0x140a05530 —
//! reads `Box` (two Vec3 corners + scale) and builds a `Mesh` (8 corners /
//! 6 faces / 36 indices; exact vertex normals/UVs I). `Box::new` inlined into
//! `spawn_human_ref` @0x14089d5a0.

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `geometry::cube::Cube` (O named entity).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {
    /// half-extent (I).
    pub half_extent: f32,
}

/// `geometry::cube::Box` (O named entity; axis-aligned box).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Box {
    /// min corner (O read pattern: first Vec3).
    pub min: glam::Vec3,
    /// max corner.
    pub max: glam::Vec3,
}

impl Box {
    /// `Box::new` (O inline into `spawn_human_ref`).
    pub fn new(_min: glam::Vec3, _max: glam::Vec3) -> Box {
        Box { min: _min, max: _max }
    }
}

impl From<Box> for crate::utils::Mesh {
    /// `impl From<Box> for Mesh` @0x140a05530 (O).
    fn from(_b: Box) -> crate::utils::Mesh {
        todo!("recover Box -> Mesh (8 corners / 36 indices)")
    }
}

impl From<Cube> for crate::utils::Mesh {
    /// `impl From<Cube> for Mesh` (O; 6 callers).
    fn from(_c: Cube) -> crate::utils::Mesh {
        todo!("recover Cube -> Mesh")
    }
}

/// typed-gap marker alias to keep geometry self-contained.
pub fn _gap() -> Gap {
    Gap
}
