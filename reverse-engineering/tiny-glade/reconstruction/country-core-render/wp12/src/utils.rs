//! utils — game-owned utility facades consumed by `systems::wall` (WP12).
//!
//! `utils::curve::Curve` / `CurveU` / geometry / `Transform` are game-owned utilities in the
//! `utils` crate (local/path dep). Per project policy their **math** is reused from `glam`
//! 0.29.3; the shell (method names, coordinate semantics, layout) is recovered from B PDB
//! evidence. All non-O fields are typed gaps.

use glam::{Mat4, Quat, Vec3};

use crate::typed_gap::Gap;

// ---------------------------------------------------------------------------
// utils::components::transform::Transform — O layout (wp6 recovery, reused).
// ---------------------------------------------------------------------------
pub mod components {
    pub mod transform {
        use glam::{Mat4, Quat, Vec3};

        /// Game-owned 40-byte transform (O: `compute_affine` reads 10×f32):
        /// `{ translation Vec3 @0x0, rotation Quat @0xC, scale Vec3 @0x1C }`.
        #[derive(Clone, Copy, Debug, Default)]
        pub struct Transform {
            pub translation: Vec3,
            pub rotation: Quat,
            pub scale: Vec3,
        }

        impl Transform {
            pub const IDENTITY: Self = Self {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            };
            pub fn from_translation(t: Vec3) -> Self {
                Self {
                    translation: t,
                    ..Self::IDENTITY
                }
            }
            pub fn from_scale(s: Vec3) -> Self {
                Self {
                    scale: s,
                    ..Self::IDENTITY
                }
            }
            pub fn from_translation_scale(t: Vec3, s: Vec3) -> Self {
                Self {
                    translation: t,
                    scale: s,
                    ..Self::IDENTITY
                }
            }
            pub fn compute_matrix(&self) -> Mat4 {
                Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
            }
            pub fn compute_affine(&self) -> Mat4 {
                self.compute_matrix()
            }
            pub fn forward(&self) -> Vec3 {
                self.rotation * Vec3::Z
            }
        }
    }
}

// ---------------------------------------------------------------------------
// utils::curve — Curve / CurveU / CurveUBoundless facades.
// ---------------------------------------------------------------------------
pub mod curve {
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
    #[derive(Clone, Debug, Serialize, Deserialize)]
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
            crate::utils::geometry::aabb::Aabb2 { raw: Gap }
        }
    }
}

// ---------------------------------------------------------------------------
// utils::geometry — Rectangle2d / Circle2d / Aabb2 facades.
// ---------------------------------------------------------------------------
pub mod geometry {
    use crate::typed_gap::Gap;

    pub mod circle {
        use crate::typed_gap::Gap;
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
        pub struct Circle2d {
            pub raw: Gap,
        }
        impl Circle2d {
            pub fn as_curve2(&self) -> crate::utils::curve::Curve {
                crate::utils::curve::Curve { raw: Gap }
            }
            pub fn get_tangent_at_u(&self, _u: f32) -> glam::Vec3 {
                glam::Vec3::X
            }
        }
    }

    pub mod rectangle {
        use crate::typed_gap::Gap;
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
        pub struct Rectangle2d {
            pub raw: Gap,
        }
        impl Rectangle2d {
            pub fn as_curve2(&self) -> crate::utils::curve::Curve {
                crate::utils::curve::Curve { raw: Gap }
            }
            pub fn lerp(&self, _other: Self, _t: f32) -> Self {
                Self { raw: Gap }
            }
        }
    }

    pub mod aabb {
        use crate::typed_gap::Gap;

        #[derive(Clone, Copy, Debug)]
        pub struct Aabb2 {
            pub raw: Gap,
        }
        impl Aabb2 {
            pub fn from_points(_points: &[glam::Vec3]) -> Self {
                Self { raw: Gap }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// utils math helpers (game-owned policy; math reuse from glam).
// ---------------------------------------------------------------------------
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
pub fn inverse_lerp(a: f32, b: f32, v: f32) -> f32 {
    (v - a) / (b - a)
}
pub fn smoothstep(a: f32, b: f32, v: f32) -> f32 {
    let t = ((v - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
pub fn perspective_lerp(_a: glam::Vec3, _b: glam::Vec3, _t: f32) -> glam::Vec3 {
    glam::Vec3::ZERO
}
pub fn t_of_point_projection_onto_segment(
    _p: glam::Vec3,
    _a: glam::Vec3,
    _b: glam::Vec3,
) -> f32 {
    0.0
}
pub fn project_point_onto_line(_p: glam::Vec3, _a: glam::Vec3, _b: glam::Vec3) -> glam::Vec3 {
    glam::Vec3::ZERO
}
pub fn project_point_onto_segment(_p: glam::Vec3, _a: glam::Vec3, _b: glam::Vec3) -> glam::Vec3 {
    glam::Vec3::ZERO
}
pub fn distance_segment_point(_a: glam::Vec3, _b: glam::Vec3, _p: glam::Vec3) -> f32 {
    0.0
}
pub fn distance_line_point(_a: glam::Vec3, _b: glam::Vec3, _p: glam::Vec3) -> f32 {
    0.0
}
pub fn time_continuous_lerp(_a: f32, _b: f32, _speed: f32, _dt: f32) -> f32 {
    0.0
}
pub fn is_inside_map_bounds(_pos: glam::Vec3) -> bool {
    true
}

pub mod comparative_space {
    use crate::typed_gap::Gap;
    pub struct ComparativeSpace {
        pub raw: Gap,
    }
    impl ComparativeSpace {
        pub fn project_unbounded(&self, _p: glam::Vec3) -> glam::Vec3 {
            glam::Vec3::ZERO
        }
    }
}

pub mod swizzlers {
    /// Zero-cost 2D->3D swizzles observed in wall geometry paths.
    pub trait Swizzle2 {
        fn x0y(self) -> glam::Vec3;
        fn xvy(self) -> glam::Vec3;
    }
    impl Swizzle2 for glam::Vec2 {
        fn x0y(self) -> glam::Vec3 {
            glam::Vec3::new(self.x, 0.0, self.y)
        }
        fn xvy(self) -> glam::Vec3 {
            glam::Vec3::new(self.x, self.y, 0.0)
        }
    }
}

pub mod rdp {
    use crate::typed_gap::Gap;
    #[derive(Clone, Debug)]
    pub struct Rdp {
        pub raw: Gap,
    }
}

// ---------------------------------------------------------------------------
// country_core::utils — project cursor onto ray, world->screen, debug viz, child wall.
// ---------------------------------------------------------------------------
pub mod project_cursor_onto_ray {
    use crate::typed_gap::Gap;

    pub struct ProportionalScreenSpace {
        pub raw: Gap,
    }
    impl ProportionalScreenSpace {
        pub fn from_window_size(_w: u32, _h: u32) -> Self {
            Self { raw: Gap }
        }
        pub fn transform_screen_space(&self, _p: glam::Vec2) -> glam::Vec2 {
            glam::Vec2::ZERO
        }
    }
    pub struct ProportionalScreenSpaceCoord {
        pub raw: Gap,
    }
    impl ProportionalScreenSpaceCoord {
        pub fn from_screen_space(_p: glam::Vec2, _s: &ProportionalScreenSpace) -> Self {
            Self { raw: Gap }
        }
    }
}

pub fn world_space_to_screen_space(_p: glam::Vec3, _camera: Gap, _w: Gap, _h: Gap) -> glam::Vec2 {
    glam::Vec2::ZERO
}

pub mod child_wall {
    use crate::typed_gap::Gap;
    #[derive(Clone, Debug)]
    pub struct ChildWall {
        pub raw: Gap,
    }
    impl ChildWall {
        pub fn new() -> Self {
            Self { raw: Gap }
        }
    }
    impl Default for ChildWall {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod debug_viz {
    use crate::typed_gap::Gap;

    pub struct DebugLine {
        pub raw: Gap,
    }
    impl DebugLine {
        pub fn new(_a: glam::Vec3, _b: glam::Vec3) -> Self {
            Self { raw: Gap }
        }
        pub fn color(self, _c: glam::Vec4) -> Self {
            self
        }
        pub fn dashed(self, _on: f32, _off: f32) -> Self {
            self
        }
        pub fn line_width(self, _w: f32) -> Self {
            self
        }
    }
    pub struct DebugSphere {
        pub raw: Gap,
    }
    impl DebugSphere {
        pub fn new(_c: glam::Vec3, _r: f32) -> Self {
            Self { raw: Gap }
        }
        pub fn radius(self, _r: f32) -> Self {
            self
        }
        pub fn line_width(self, _w: f32) -> Self {
            self
        }
    }
    pub struct DebugRectangle2d {
        pub raw: Gap,
    }
    impl DebugRectangle2d {
        pub fn new(_a: glam::Vec2, _b: glam::Vec2) -> Self {
            Self { raw: Gap }
        }
        pub fn color(self, _c: glam::Vec4) -> Self {
            self
        }
    }
}
