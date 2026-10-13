//! `geometry/sphere.rs` — `Sphere` procedural mesh.
//!
//! Source: B skeleton `crates/country-core/src/geometry/sphere.rs`.
//! PDB named entity: `Sphere`.
//! O evidence (Ghidra, B patched EXE):
//! - `Sphere::new` @0x140b4af60.
//! - `Sphere -> Mesh::try_from` @0x140a06b00 (8 callers) — iterates
//!   `Vec3A` vertices (`Copied<Iter<Vec3A>>` map) to build the sphere mesh;
//!   returns `Result` (invalid radius → Err).

#![allow(dead_code, unused_imports)]

/// `geometry::sphere::Sphere`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    /// radius.
    pub radius: f32,
}

impl Sphere {
    /// `Sphere::new` @0x140b4af60 (O).
    pub fn new(radius: f32) -> Sphere {
        Sphere { radius }
    }
}

impl TryFrom<Sphere> for crate::utils::Mesh {
    type Error = ();
    /// `impl TryFrom<Sphere> for Mesh` @0x140a06b00 (O).
    fn try_from(_s: Sphere) -> Result<crate::utils::Mesh, ()> {
        todo!("recover Sphere -> Mesh (Vec3A vertex iteration)")
    }
}
