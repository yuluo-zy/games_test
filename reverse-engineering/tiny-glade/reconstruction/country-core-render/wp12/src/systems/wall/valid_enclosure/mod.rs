//! valid_enclosure/mod.rs — wall enclosure shape parameters (circle / rectangle / curve).
//!
//! PDB procedures: `ShapeParameters::{expand, center, as_curve2, as_curve3, as_circle,
//! as_rectangle, perimeter, contains_point, contains_shape, get_normal_at_u, get_pos_at_u,
//! get_closest_u_from_pos}` (see address-map 0x1411ff2a0..0x1411ff6f0 + serde impls).
//! Business logic O (decompiled leaf sizes); shape payloads are typed gaps.

use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;
use crate::utils::curve::{Curve, CurveInternalCoord, CurveU};
use crate::utils::geometry::circle::Circle2d;
use crate::utils::geometry::rectangle::Rectangle2d;

/// Enclosure shape parameters (O: enum with curve/circle/rectangle variants).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ShapeParameters {
    Curve(Curve),
    Circle(Circle2d),
    Rectangle(Rectangle2d),
}

impl ShapeParameters {
    pub fn expand(&self, _amount: f32) -> Self {
        self.clone()
    }
    pub fn center(&self) -> glam::Vec3 {
        match self {
            ShapeParameters::Curve(c) => {
                let points = c.points_with_u();
                if points.is_empty() {
                    glam::Vec3::ZERO
                } else {
                    let (first, _) = points[0];
                    let (last, _) = *points.last().unwrap();
                    (first + last) * 0.5
                }
            }
            _ => glam::Vec3::ZERO,
        }
    }
    pub fn as_curve2(&self) -> Curve {
        match self {
            ShapeParameters::Curve(c) => c.clone(),
            ShapeParameters::Circle(c) => c.as_curve2(),
            ShapeParameters::Rectangle(r) => r.as_curve2(),
        }
    }
    pub fn as_curve3(&self) -> Curve {
        self.as_curve2()
    }
    pub fn as_circle(&self) -> Option<Circle2d> {
        match self {
            ShapeParameters::Circle(c) => Some(*c),
            _ => None,
        }
    }
    pub fn as_rectangle(&self) -> Option<Rectangle2d> {
        match self {
            ShapeParameters::Rectangle(r) => Some(*r),
            _ => None,
        }
    }
    pub fn perimeter(&self) -> f32 {
        match self {
            ShapeParameters::Curve(c) => c.length_ws(),
            _ => 0.0,
        }
    }
    pub fn contains_point(&self, _p: glam::Vec3) -> bool {
        false
    }
    pub fn contains_shape(&self, _other: &ShapeParameters) -> bool {
        false
    }
    pub fn get_normal_at_u(&self, _u: CurveU) -> glam::Vec3 {
        glam::Vec3::Y
    }
    pub fn get_pos_at_u(&self, u: CurveU) -> glam::Vec3 {
        self.as_curve2().get_pos_at_u(u)
    }
    pub fn get_closest_u_from_pos(&self, _p: glam::Vec3) -> CurveU {
        CurveU { raw: Gap }
    }

    /// Facade helper used by structure_outline (O: builds params from a public wall).
    pub fn from_wall(_state: &crate::resources::walls::public_wall_state::PublicWallState) -> Self {
        // The original reconstructs the shape from the wall's shape snapshot (typed gap).
        ShapeParameters::Curve(Curve { raw: Gap })
    }
    pub fn _coord(&self, _c: CurveInternalCoord) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
}
