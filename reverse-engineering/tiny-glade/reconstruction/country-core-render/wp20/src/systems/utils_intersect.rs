//! `systems/utils_intersect.rs` — line × shape overlap test.
//!
//! PDB: `line_x_shape_overlap @0x14175ada0` (900 B).
//!
//! O: for a line (origin + direction, with t-range params) against a shape
//! tag (`@+0x28` bit 0: 0 = circle, 1 = rectangle):
//!   - circle: `Circle2d::intersect_ray`; t ∈ [0,1] valid.
//!   - rectangle: `Rectangle2d::orientation_mat2` (two OBB axes) +
//!     `Aabb2::from_center_dims`/`intersect_ray` + `RayBoxIntersection::is_hit`.
//! t values wrap with `fmod`; uses `utils::lerp`. Returns whether the line
//! overlaps the shape.

#![allow(dead_code, unused_imports)]

use glam::Vec2;

use crate::utils::geometry::aabb::{Aabb2, RayBoxIntersection};
use crate::utils::geometry::circle::Circle2d;
use crate::utils::geometry::rectangle::Rectangle2d;
use crate::utils::lerp;

/// `line_x_shape_overlap` @0x14175ada0 (O).
pub fn line_x_shape_overlap(
    _origin: Vec2,
    _direction: Vec2,
    _shape_tag: u8,
    _shape: &LineShape,
) -> bool {
    todo!("recover line_x_shape_overlap (circle/rectangle ray tests + t wrap)")
}

/// Line target shape (I; O discriminator bit 0 at the shape source).
#[derive(Clone, Copy, Debug)]
pub enum LineShape {
    Circle(Circle2d),
    Rectangle { rect: Rectangle2d, aabb: Aabb2 },
}
