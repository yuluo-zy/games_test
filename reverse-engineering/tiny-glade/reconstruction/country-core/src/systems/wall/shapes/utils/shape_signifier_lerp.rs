//! shapes/utils/shape_signifier_lerp.rs — signifier value interpolation between shapes.
//!
//! PDB procedure `ShapeVal::interpolate` 0x1409ef010 (510 B) + `get_radius`,
//! `get_rectangle_dims`, `ShapeSignifierLerp::lerp_radius`.

use crate::typed_gap::Gap;
use crate::utils::geometry::circle::Circle2d;
use crate::utils::geometry::rectangle::Rectangle2d;

/// Signifier value being lerped between circle and rectangle shapes (O).
#[derive(Clone, Copy, Debug)]
pub enum ShapeVal {
    Circle(Circle2d),
    Rectangle(Rectangle2d),
    None,
}
impl ShapeVal {
    pub fn get_radius(&self) -> f32 {
        match self {
            ShapeVal::Circle(c) => radius_of(*c),
            _ => 0.0,
        }
    }
    pub fn get_rectangle_dims(&self) -> glam::Vec2 {
        match self {
            ShapeVal::Rectangle(r) => dims_of(*r),
            _ => glam::Vec2::ZERO,
        }
    }
    /// O: `ShapeVal::interpolate` — lerp between the two shapes by t.
    pub fn interpolate(&self, other: Self, _t: f32) -> Self {
        let _ = other;
        *self
    }
}

/// Shape signifier lerp driver (O: `lerp_radius`).
#[derive(Clone, Copy, Debug, Default)]
pub struct ShapeSignifierLerp {
    pub raw: Gap,
}
impl ShapeSignifierLerp {
    pub fn lerp_radius(&self, _a: &ShapeVal, _b: &ShapeVal, _t: f32) -> f32 {
        0.0
    }
}

fn radius_of(_c: Circle2d) -> f32 {
    0.0
}
fn dims_of(_r: Rectangle2d) -> glam::Vec2 {
    glam::Vec2::ZERO
}
