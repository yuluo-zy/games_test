//! `utils/geo.rs` — curve/point mesh helpers (`mesh_from_pts`,
//! `update_curve_mesh`, `rewire_enclosed_curve`).
//!
//! Source: B skeleton `crates/country-core/src/utils/geo.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `mesh_from_pts` @0x140a6c120 — copies the `Vec3` point array (stride 0xc),
//!   `Mesh::set_attribute<Vec<[f32;3]>>` (positions) + `vec::from_elem`
//!   (second attribute), builds the identity index buffer `0..n`, and stamps
//!   the transient mesh name-hash `anon_cc634b...` + the thread-local instance
//!   counter.
//! - `update_curve_mesh<It>` @0x14157eac0 (generic over point iterator).
//! - `rewire_enclosed_curve` @0x140a6ba50 (O).

#![allow(dead_code, unused_imports)]

use crate::utils::Mesh;
use crate::Gap;

/// `utils::geo::mesh_from_pts` @0x140a6c120 (O): curve/line mesh from points.
pub fn mesh_from_pts(points: &[glam::Vec3]) -> Mesh {
    todo!("recover mesh_from_pts (positions + identity indices + transient name hash)")
}

/// `update_curve_mesh<It>` @0x14157eac0 (O; generic over the point iterator).
pub fn update_curve_mesh<It: Iterator<Item = glam::Vec3>>(_points: It) -> Mesh {
    todo!("recover update_curve_mesh")
}

/// `rewire_enclosed_curve` @0x140a6ba50 (O).
pub fn rewire_enclosed_curve(_points: &mut [glam::Vec3]) {
    todo!("recover rewire_enclosed_curve")
}

/// marker gap alias.
pub fn _gap() -> Gap {
    Gap
}
