//! `geometry/plane.rs` — `Plane` procedural quad mesh.
//!
//! Source: B skeleton `crates/country-core/src/geometry/plane.rs`.
//! O evidence (Ghidra, B patched EXE): `Plane -> Mesh::from` @0x140a06370
//! (a unit quad; exact UV/normal layout I).

#![allow(dead_code, unused_imports)]

/// `geometry::plane::Plane`.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Plane {
    /// size (I).
    pub size: glam::Vec2,
}

impl From<Plane> for crate::utils::Mesh {
    /// `impl From<Plane> for Mesh` @0x140a06370 (O).
    fn from(_p: Plane) -> crate::utils::Mesh {
        todo!("recover Plane -> Mesh")
    }
}
