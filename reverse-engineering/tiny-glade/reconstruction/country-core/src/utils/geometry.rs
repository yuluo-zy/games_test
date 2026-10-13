//! utils/geometry.rs — game-owned geometry helpers (wp12 wall-domain facade):
//! `circle::Circle2d` / `rectangle::Rectangle2d` / `aabb::Aabb2`.

use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

pub mod circle {
    use serde::{Deserialize, Serialize};

    use crate::typed_gap::Gap;

    #[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
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
    use serde::{Deserialize, Serialize};

    use crate::typed_gap::Gap;

    #[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
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
    use glam::Vec2;

    use crate::typed_gap::Gap;

    /// `utils::geometry::aabb::Aabb2`（WP21 恢复：16B = `{min, max}` Vec2；
    /// `from_center_dims` 供 `wall_holes::finalize`；`dims_wall_space` 用
    /// `max - min`）。
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Aabb2 {
        pub min: Vec2,
        pub max: Vec2,
    }
    impl Aabb2 {
        pub fn from_points(points: &[glam::Vec3]) -> Self {
            let mut out = Aabb2 {
                min: Vec2::splat(f32::MAX),
                max: Vec2::splat(f32::MIN),
            };
            for p in points {
                out.min = out.min.min(p.truncate());
                out.max = out.max.max(p.truncate());
            }
            out
        }
        /// `Aabb2::from_center_dims`（WP21；O：`wall_holes::finalize` 调用）。
        pub fn from_center_dims(center: Vec2, dims: Vec2) -> Self {
            Aabb2 {
                min: center - dims * 0.5,
                max: center + dims * 0.5,
            }
        }
        /// `intersect_ray`（WP20 `utils_intersect` 命名；typed gap）。
        pub fn intersect_ray(&self, _origin: Vec2, _direction: Vec2) -> Option<RayBoxIntersection> {
            None
        }
    }

    /// `RayBoxIntersection`（O 名；`is_hit` 谓词；typed gap）。
    #[derive(Clone, Copy, Debug, Default)]
    pub struct RayBoxIntersection(pub Gap);
    impl RayBoxIntersection {
        pub fn is_hit(&self) -> bool {
            false
        }
    }
}
