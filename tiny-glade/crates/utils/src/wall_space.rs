use crate::comparative_space::{CircularComparativeSpace, ComparativeSpace};
use glam::{Vec2, Vec3};
#[derive(Clone, Copy, Debug)]
pub struct WallSpace {
    pub is_loop: bool,
    pub length: f32,
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)] // Generated original-output fixtures.
#[path = "oracle/wall_space_completion_oracle.rs"]
mod completion_oracle;
impl WallSpace {
    pub fn space_length(&self) -> f32 {
        self.length
    }
}
impl ComparativeSpace for WallSpace {
    fn distance(&self, a: f32, b: f32) -> f32 {
        if self.is_loop {
            CircularComparativeSpace {
                period: self.length,
            }
            .distance(a, b)
        } else {
            (a - b).abs()
        }
    }
    fn project(&self, value: f32) -> Option<f32> {
        if self.is_loop {
            Some(self.project_clamp(value))
        } else if value >= 0.0 && value <= self.length {
            Some(value)
        } else {
            None
        }
    }
    fn project_clamp(&self, value: f32) -> f32 {
        if self.is_loop {
            CircularComparativeSpace {
                period: self.length,
            }
            .project_clamp(value)
        } else {
            value.clamp(0.0, self.length)
        }
    }
    fn project_unbounded(&self, value: f32) -> f32 {
        if self.is_loop {
            self.project_clamp(value)
        } else {
            value
        }
    }
    fn lerp_clamp(&self, a: f32, b: f32, t: f32) -> f32 {
        if self.is_loop {
            CircularComparativeSpace {
                period: self.length,
            }
            .lerp_clamp(a, b, t)
        } else {
            self.project_clamp(a + self.subtract(b, a) * t)
        }
    }
    fn subtract(&self, a: f32, b: f32) -> f32 {
        if self.is_loop {
            CircularComparativeSpace {
                period: self.length,
            }
            .subtract(a, b)
        } else {
            a - b
        }
    }
}
/// Curve operations used to convert between wall and world coordinates.
pub trait CurveSampling {
    fn length(&self) -> f32;
    fn point_u(&self, index: usize) -> f32;
    fn position_at_u(&self, u: f32) -> Vec2;
    fn tangent_at_u(&self, u: f32) -> Vec2;

    /// Actual selected segment difference for native endpoint extrapolation.
    /// Canonical Curve2 overrides this with its raw point difference. Existing
    /// custom host implementations retain source compatibility through tangent;
    /// a normalized custom tangent is not proof of original Curve2 equivalence.
    fn raw_tangent_at_u(&self, u: f32) -> Vec2 {
        self.tangent_at_u(u)
    }

    fn approximate_u(&self, point: Vec2) -> f32;

    /// Unit direction used before the start; zero means no extrapolation.
    /// Polylines with repeated endpoints should override this method.
    fn start_tangent(&self) -> Vec2 {
        self.tangent_at_u(0.0).normalize_or_zero()
    }

    /// Unit direction used after the end; zero means no extrapolation.
    fn end_tangent(&self) -> Vec2 {
        self.tangent_at_u(1.0).normalize_or_zero()
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallSpaceCoord {
    pub u: f32,
    pub y: f32,
}

/// An axis-aligned region in wall coordinates, expressed with half extents.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallSpaceRegion {
    pub center: WallSpaceCoord,
    pub half_extent: Vec2,
}
impl WallSpaceRegion {
    /// Tests strict overlap, with a wrapping horizontal axis on closed walls.
    pub fn intersects(self, other: Self, space: &WallSpace) -> bool {
        use crate::comparative_space::ZeroBoundedComparativeSpace;
        space.regions_intersect(
            self.center.u,
            self.half_extent.x,
            other.center.u,
            other.half_extent.x,
        ) && ZeroBoundedComparativeSpace {
            upper_bound: f32::INFINITY,
        }
        .regions_intersect(
            self.center.y,
            self.half_extent.y,
            other.center.y,
            other.half_extent.y,
        )
    }
}
impl WallSpaceCoord {
    /// Constructs a coordinate projected onto the wall.
    pub fn new(space: &WallSpace, u: f32, y: f32) -> Self {
        Self {
            u: space.project_clamp(u),
            y,
        }
    }
    pub fn new_unchecked(u: f32, y: f32) -> Self {
        Self { u, y }
    }
    pub fn x(self) -> f32 {
        self.u
    }
    pub fn as_vec2(self) -> Vec2 {
        Vec2::new(self.u, self.y)
    }
    /// Applies an offset, returning `None` outside an open wall.
    pub fn with_offset(self, space: &WallSpace, offset: Vec2) -> Option<Self> {
        space.project(self.u + offset.x).map(|u| Self {
            u,
            y: self.y + offset.y,
        })
    }
    /// Applies an offset and projects the result onto the wall.
    pub fn with_offset_clamped(self, space: &WallSpace, offset: Vec2) -> Self {
        Self {
            u: space.project_clamp(self.u + offset.x),
            y: self.y + offset.y,
        }
    }
    /// Applies an offset, allowing extrapolation beyond open-wall endpoints.
    pub fn with_offset_unbounded(self, space: &WallSpace, offset: Vec2) -> Self {
        Self {
            u: space.project_unbounded(self.u + offset.x),
            y: self.y + offset.y,
        }
    }
    pub fn offset_x_clamped(self, space: &WallSpace, offset: f32) -> Self {
        Self {
            u: space.project_clamp(self.u + offset),
            y: self.y,
        }
    }
    /// Subtraction is the WallSpace policy (including half-period tie direction).
    pub fn subtract(self, space: &WallSpace, rhs: Self) -> Vec2 {
        Vec2::new(space.subtract(self.u, rhs.u), self.y - rhs.y)
    }
    pub fn with_y_offset(self, offset: f32) -> Self {
        Self {
            u: self.u,
            y: self.y + offset,
        }
    }
    pub fn with_y(self, y: f32) -> Self {
        Self { u: self.u, y }
    }
    // Reject ordered out-of-range t but permit NaN. Rewriting
    // this as RangeInclusive::contains would change that policy.
    #[allow(clippy::manual_range_contains)]
    pub fn from_curve_internal_coord(
        curve: &impl CurveSampling,
        index: usize,
        t: f32,
        y: f32,
    ) -> Self {
        assert!(!(t < 0.0 || t > 1.0));
        let a = curve.point_u(index);
        let b = curve.point_u(index + 1);
        Self {
            u: (a + (b - a) * t) * curve.length(),
            y,
        }
    }
    pub fn from_world_space(position: Vec3, curve: &impl CurveSampling) -> Self {
        Self {
            u: curve
                .approximate_u(Vec2::new(position.x, position.z))
                .clamp(0.0, 1.0)
                * curve.length(),
            y: position.y,
        }
    }
    pub fn get_pos_xz(self, curve: &impl CurveSampling) -> Vec2 {
        let p = self.to_world_space(curve);
        Vec2::new(p.x, p.z)
    }
    pub fn get_tangent(self, curve: &impl CurveSampling) -> Vec2 {
        curve.tangent_at_u((self.u / curve.length()).clamp(0.0, 1.0))
    }
    /// B140c92090/793B: real selected endpoint, original two f32 normalizations.
    pub fn to_world_space(self, curve: &impl CurveSampling) -> Vec3 {
        use crate::curve::{curve_u::CurveUBoundless, CurveCoordSemantic2D, CurveU};
        let length = curve.length();
        let normalized = self.u / length;
        if self.u < 0.0 || self.u > length {
            let u = CurveUBoundless::<CurveCoordSemantic2D>::new(normalized)
                .clamp()
                .get();
            let p = curve.position_at_u(u);
            let position = Vec3::new(p.x, self.y, p.y);
            let delta = curve.raw_tangent_at_u(u);
            let reciprocal = 1.0 / (delta.y * delta.y + delta.x * delta.x).sqrt();
            let q = delta * reciprocal;
            let reciprocal = 1.0 / (q.y * q.y + q.x * q.x).sqrt();
            // Preserve zero*NaN in the middle lane for a zero endpoint segment.
            let tangent = Vec3::new(q.x, 0.0, q.y) * reciprocal;
            if self.u > length {
                position + tangent * (self.u - length)
            } else {
                // Native actual direction is B-A and negativeU is subtracted.
                position - tangent * self.u
            }
        } else {
            let u = CurveU::<CurveCoordSemantic2D>::new(normalized).get();
            let p = curve.position_at_u(u);
            Vec3::new(p.x, self.y, p.y)
        }
    }
}

#[cfg(test)]
mod world_position_original_tests {
    use super::*;
    use crate::curve::Curve2;

    #[test]
    fn negative_distance_subtracts_actual_forward_endpoint_tangent() {
        let curve = Curve2::try_new(
            vec![Vec2::ZERO, Vec2::new(3.0, 4.0), Vec2::new(6.0, 0.0)],
            false,
        ).unwrap();
        let position = WallSpaceCoord::new_unchecked(-1.0, 2.0).to_world_space(&curve);
        assert_eq!(position, Vec3::new(0.6, 2.0, 0.8));
        assert_eq!(curve.raw_tangent_at_u(0.0), Vec2::new(3.0, 4.0));
    }

    #[test]
    fn nonfinite_normalized_coordinate_is_an_observable_failure() {
        let curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::X], false).unwrap();
        for u in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(std::panic::catch_unwind(|| {
                WallSpaceCoord::new_unchecked(u, 2.0).to_world_space(&curve)
            }).is_err());
        }
    }

    #[test]
    fn in_range_input_y_nan_propagates_without_an_extra_guard() {
        let curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::new(3.0, 4.0)], false).unwrap();
        let position = WallSpaceCoord::new_unchecked(2.5, f32::NAN).to_world_space(&curve);
        assert_eq!(position.x, 1.5);
        assert!(position.y.is_nan());
        assert_eq!(position.z, 2.0);
    }
}
