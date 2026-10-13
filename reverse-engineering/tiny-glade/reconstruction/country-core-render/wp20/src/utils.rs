//! `utils` — INTEGRATION MARKER facade for game-owned `utils` helpers
//! (wp5-wp16-owned). Real modules live in `reconstruction/country-core/src/
//! utils/`. This gate declares the exact surfaces the WP20 systems reference.

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `utils::calculate_hash` (O; name-hash, u64).
pub fn calculate_hash(value: impl core::hash::Hash) -> u64 {
    use core::hash::{BuildHasher, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut h = DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}

/// `utils::lerp` (O).
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a) * t + a
}

/// Game-owned `Transform` (wp6; 40 B `{translation@0x0, rotation@0xC,
/// scale@0x1C}`).
pub mod components {
    use glam::{Quat, Vec3};

    /// `utils::components::transform::Transform` (wp6; O layout).
    #[derive(
        bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq,
    )]
    pub struct Transform {
        pub translation: Vec3,
        pub rotation: Quat,
        pub scale: Vec3,
    }
    impl Transform {
        pub fn identity() -> Self {
            Transform {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            }
        }
        pub fn from_translation(translation: Vec3) -> Self {
            Transform {
                translation,
                ..Transform::identity()
            }
        }
        pub fn compute_matrix(&self) -> glam::Mat4 {
            glam::Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
        }
        pub fn forward(&self) -> Vec3 {
            self.rotation * -Vec3::Z
        }
    }

    /// Path alias so systems can write
    /// `crate::utils::components::transform::Transform` (matching the real
    /// `reconstruction/country-core/src/utils/components/transform.rs`).
    pub mod transform {
        pub use super::Transform;
    }
}

/// `utils::camera::CameraProjectionParams::calc_matrices` (O call from
/// `paused_main_camera_update`).
pub mod camera {
    use crate::Gap;

    /// `utils::camera::CameraProjectionParams` (O; fields U).
    pub struct CameraProjectionParams {
        pub raw: Gap,
    }
    impl CameraProjectionParams {
        pub fn calc_matrices(&self, _transform: &crate::utils::components::Transform, _aspect: f32) -> [glam::Mat4; 4] {
            [glam::Mat4::IDENTITY; 4]
        }
    }
}

/// `utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord` (O call from
/// `impl_raise_shape`).
pub mod project_cursor_onto_ray {
    use crate::Gap;

    /// `ProportionalScreenSpaceCoord::project_onto_ray` (O).
    pub struct ProportionalScreenSpaceCoord {
        pub raw: Gap,
    }
    impl ProportionalScreenSpaceCoord {
        pub fn project_onto_ray(
            &self,
            _origin: glam::Vec3,
            _direction: glam::Vec3,
            _cursor: glam::Vec2,
        ) -> f32 {
            0.0
        }
    }
}

/// `utils::geometry` — circle / rectangle / aabb intersection helpers used by
/// `utils_intersect::line_x_shape_overlap` (O call names).
pub mod geometry {
    use glam::{Vec2, Vec3};

    pub mod circle {
        use glam::Vec2;
        /// `Circle2d::intersect_ray` (O).
        #[derive(Clone, Copy, Debug)]
        pub struct Circle2d {
            pub center: Vec2,
            pub radius: f32,
        }
        impl Circle2d {
            pub fn intersect_ray(&self, _o: Vec2, _d: Vec2) -> Option<(f32, Vec2)> {
                None
            }
        }
    }
    pub mod rectangle {
        use glam::Vec2;
        /// `Rectangle2d::orientation_mat2` (O).
        #[derive(Clone, Copy, Debug, Default)]
        pub struct Rectangle2d(pub [f32; 2]);
        impl Rectangle2d {
            pub fn orientation_mat2(&self) -> glam::Mat2 {
                glam::Mat2::IDENTITY
            }
        }
    }
    pub mod aabb {
        use glam::Vec2;
        /// `Aabb2::from_center_dims` / `intersect_ray` (O).
        #[derive(Clone, Copy, Debug)]
        pub struct Aabb2 {
            pub min: Vec2,
            pub max: Vec2,
        }
        impl Aabb2 {
            pub fn from_center_dims(center: Vec2, dims: Vec2) -> Self {
                Aabb2 {
                    min: center - dims * 0.5,
                    max: center + dims * 0.5,
                }
            }
            pub fn intersect_ray(&self, _o: Vec2, _d: Vec2) -> Option<(f32, Vec2)> {
                None
            }
        }
        /// `RayBoxIntersection::is_hit` (O).
        #[derive(Clone, Copy, Debug)]
        pub struct RayBoxIntersection(pub f32);
        impl RayBoxIntersection {
            pub fn is_hit(&self) -> bool {
                false
            }
        }
    }
}

/// `utils::lerp` alias used by camera bodies (O).
pub use lerp as smoothstep;
