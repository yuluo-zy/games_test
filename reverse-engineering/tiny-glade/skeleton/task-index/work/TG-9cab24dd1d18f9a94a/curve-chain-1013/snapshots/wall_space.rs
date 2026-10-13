use crate::comparative_space::{CircularComparativeSpace, ComparativeSpace};
use glam::{Vec2, Vec3};
#[derive(Clone, Copy, Debug)]
pub struct WallSpace {
    pub is_loop: bool,
    pub length: f32,
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)] // Generated original-output fixtures.
mod completion_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../evidence/foundation-completion-2026-10-13/wall-heightmap/wall_tests.rs"
    ));

    #[test]
    fn region_axes_use_strict_touching_and_wall_topology() {
        let region = |u, y| WallSpaceRegion {
            center: WallSpaceCoord::new_unchecked(u, y),
            half_extent: Vec2::splat(0.5),
        };
        let mut space = WallSpace {
            is_loop: false,
            length: 10.0,
        };
        assert!(!region(0.2, 1.0).intersects(region(9.8, 1.0), &space));
        space.is_loop = true;
        assert!(region(0.2, 1.0).intersects(region(9.8, 1.0), &space));
        assert!(!region(0.2, 1.0).intersects(region(9.8, 2.0), &space));
        assert!(!region(0.0, 1.0).intersects(region(1.0, 1.0), &space));
        assert_eq!(
            WallSpaceCoord::new_unchecked(0.0, -0.0)
                .offset_x_clamped(&space, 2.0)
                .y
                .to_bits(),
            (-0.0f32).to_bits()
        );
    }
}
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
/// Typed bridge to the actual recovered curve. This avoids duplicating its
/// segment lookup/nearest-coordinate algorithm inside a wall wrapper.
pub trait CurveSampling {
    fn length(&self) -> f32;
    fn point_u(&self, index: usize) -> f32;
    fn position_at_u(&self, u: f32) -> Vec2;
    fn tangent_at_u(&self, u: f32) -> Vec2;
    fn approximate_u(&self, point: Vec2) -> f32;
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallSpaceCoord {
    pub u: f32,
    pub y: f32,
}

/// Semantic region adapter. Center/half extent represent the observed caller
/// arithmetic; this is not a claim about the original struct's field layout.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallSpaceRegion {
    pub center: WallSpaceCoord,
    pub half_extent: Vec2,
}
impl WallSpaceRegion {
    /// Original IPI 0x6240 delegates both axes to strict regions_intersect.
    /// Horizontal distance follows wall topology; the height axis is linear.
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
    /// Original `new` inline at B 0x1408109c4 delegates to project_clamp.
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
    /// B caller 0x14080fb00 projects the sum, retaining failure on an open wall.
    pub fn with_offset(self, space: &WallSpace, offset: Vec2) -> Option<Self> {
        space.project(self.u + offset.x).map(|u| Self {
            u,
            y: self.y + offset.y,
        })
    }
    /// B inline 0x14080f763/0x14080f788 uses project_clamp on the sum.
    pub fn with_offset_clamped(self, space: &WallSpace, offset: Vec2) -> Self {
        Self {
            u: space.project_clamp(self.u + offset.x),
            y: self.y + offset.y,
        }
    }
    /// B chimney caller delegates horizontal projection to project_unbounded.
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
    // The original rejects ordered out-of-range t but permits NaN. Rewriting
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
    pub fn to_world_space(self, curve: &impl CurveSampling) -> Vec3 {
        let length = curve.length();
        let normalized = self.u / length;
        let u = normalized.clamp(0.0, 1.0);
        let mut p = curve.position_at_u(u);
        if self.u < 0.0 {
            p += curve.tangent_at_u(0.0).normalize() * self.u;
        } else if self.u > length {
            p += curve.tangent_at_u(1.0).normalize() * (self.u - length);
        }
        Vec3::new(p.x, self.y, p.y)
    }
}
