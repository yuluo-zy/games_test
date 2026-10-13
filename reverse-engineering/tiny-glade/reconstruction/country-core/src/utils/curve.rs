//! utils/curve.rs — game-owned curve helpers (wp12 wall-domain facade).
//!
//! `utils::curve::Curve` / `CurveU` / geometry / `Transform` are game-owned
//! utilities in the `utils` crate (local/path dep). Per project policy their
//! **math** is reused from `glam` 0.29.3; the shell (method names, coordinate
//! semantics, layout) is recovered from B PDB evidence. All non-O fields are
//! typed gaps.

use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

/// Curve coordinate semantics (game-owned marker; math reuse from glam).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CurveCoordSemantic2D {}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CurveCoordSemantic3D {}

/// 2D curve coordinate inside the curve parameterization.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurveInternalCoord {
    pub point_index: u32,
    pub fraction: f32,
}
impl CurveInternalCoord {
    pub fn i0() -> Self {
        Self {
            point_index: 0,
            fraction: 0.0,
        }
    }
    pub fn i1() -> Self {
        Self {
            point_index: 0,
            fraction: 1.0,
        }
    }
    pub fn new(point_index: u32, fraction: f32) -> Self {
        Self {
            point_index,
            fraction,
        }
    }
}

/// Curve u-value (game-owned coordinate).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurveU {
    pub raw: Gap,
}
impl CurveU {
    pub fn new(_v: f32) -> Self {
        Self { raw: Gap }
    }
    pub fn u(&self) -> f32 {
        0.0
    }
    pub fn lerp(&self, _rhs: Self, _t: f32) -> Self {
        Self { raw: Gap }
    }
    pub fn max(&self, _rhs: Self) -> Self {
        Self { raw: Gap }
    }
    pub fn min(&self, _rhs: Self) -> Self {
        Self { raw: Gap }
    }
}
impl Eq for CurveU {}
impl PartialOrd for CurveU {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

/// Unbounded curve u-value; clamps to [0,1] for sampling.
#[derive(Clone, Copy, Debug)]
pub struct CurveUBoundless {
    pub raw: Gap,
}
impl CurveUBoundless {
    pub fn new() -> Self {
        Self { raw: Gap }
    }
    pub fn clamp(&self, _min: Self, _max: Self) -> Self {
        Self { raw: Gap }
    }
    pub fn is_within_01_range(&self) -> bool {
        true
    }
}
impl Default for CurveUBoundless {
    fn default() -> Self {
        Self { raw: Gap }
    }
}

/// The game-owned Curve shell. Points / u-values are typed gaps (WP11 state geometry).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Curve {
    pub raw: Gap,
}
impl Curve {
    pub fn is_valid(&self) -> bool {
        true
    }
    pub fn is_empty(&self) -> bool {
        false
    }
    pub fn crashy_new_from_points<T>(_points: &[T]) -> Self {
        Self { raw: Gap }
    }
    pub fn crashy_slice(&self, _low: CurveU, _high: CurveU) -> Self {
        Self { raw: Gap }
    }
    pub fn try_slice(&self, _low: CurveU, _high: CurveU) -> Option<Self> {
        Some(Self { raw: Gap })
    }
    pub fn crashy_resample_remap_per_segment<S2, T2, S3, T3, F>(&self, _f: F) -> Self
    where
        F: FnMut(&T2) -> T3,
    {
        Self { raw: Gap }
    }
    pub fn start_pos(&self) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
    pub fn end_pos(&self) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
    pub fn get_pos_at_u(&self, _u: CurveU) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
    pub fn get_pos_at_coord(&self, _c: CurveInternalCoord) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
    pub fn get_u_at_coord(&self, _c: CurveInternalCoord) -> CurveU {
        CurveU { raw: Gap }
    }
    pub fn get_coord_at_u(&self, _u: CurveU) -> CurveInternalCoord {
        CurveInternalCoord::i0()
    }
    pub fn get_approx_coord_from_pos(&self, _pos: glam::Vec3) -> CurveInternalCoord {
        CurveInternalCoord::i0()
    }
    pub fn get_tangent_at_u(&self, _u: CurveU) -> glam::Vec3 {
        glam::Vec3::X
    }
    pub fn get_tangent_at_coord(&self, _c: CurveInternalCoord) -> glam::Vec3 {
        glam::Vec3::X
    }
    pub fn get_unnormalized_tangent_at_coord(&self, _c: CurveInternalCoord) -> glam::Vec3 {
        glam::Vec3::X
    }
    pub fn get_curve_segment_from_u(&self, _u: CurveU, _u2: CurveU) -> Self {
        Self { raw: Gap }
    }
    pub fn length_ws(&self) -> f32 {
        0.0
    }
    pub fn point_count(&self) -> u32 {
        0
    }
    pub fn point_u(&self, _index: usize) -> f32 {
        0.0
    }
    pub fn points_with_u(&self) -> Vec<(glam::Vec3, f32)> {
        Vec::new()
    }
    pub fn segments_with_u(&self) -> Vec<(glam::Vec3, glam::Vec3, f32)> {
        Vec::new()
    }
    pub fn segments(&self) -> Vec<(glam::Vec3, glam::Vec3)> {
        Vec::new()
    }
    pub fn bounding_box_2d(&self) -> crate::utils::geometry::aabb::Aabb2 {
        crate::utils::geometry::aabb::Aabb2 {
            min: glam::Vec2::splat(f32::MAX),
            max: glam::Vec2::splat(f32::MIN),
        }
    }
}
