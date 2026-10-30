//! Camera transforms, infinite reverse-Z perspective, and lens state.
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
#[derive(Clone, Copy, Debug)]
pub struct CameraProjectionParams {
    pub aspect: f32,
    pub near: f32,
    pub vertical_fov_degrees: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct CameraProjectionMatrices {
    pub projection: Mat4,
    pub inverse_projection: Mat4,
}
impl CameraProjectionParams {
    /// Builds an infinite reverse-Z perspective projection and its inverse.
    ///
    /// # Panics
    /// Panics for nonpositive aspect/near values, or FOV outside `(0, 180)`.
    pub fn calc_matrices(self) -> CameraProjectionMatrices {
        assert!(
            self.aspect.is_finite() && self.aspect > 0.0,
            "aspect must be finite and positive"
        );
        assert!(
            self.near.is_finite() && self.near > 0.0,
            "near plane must be finite and positive"
        );
        assert!(
            self.vertical_fov_degrees.is_finite()
                && self.vertical_fov_degrees > 0.0
                && self.vertical_fov_degrees < 180.0,
            "vertical FOV must be between 0 and 180 degrees"
        );
        let angle = self.vertical_fov_degrees.to_radians() * 0.5;
        let vertical = angle.cos() / angle.sin();
        let horizontal = vertical / self.aspect;
        let projection = Mat4::from_cols(
            Vec4::new(horizontal, 0.0, 0.0, 0.0),
            Vec4::new(0.0, vertical, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, -1.0),
            Vec4::new(0.0, 0.0, self.near, 0.0),
        );
        let inverse_projection = Mat4::from_cols(
            Vec4::new(1.0 / horizontal, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0 / vertical, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0 / self.near),
            Vec4::new(0.0, 0.0, -1.0, 0.0),
        );
        CameraProjectionMatrices {
            projection,
            inverse_projection,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct CameraBodyMatrices {
    pub view: Mat4,
    pub inverse_view: Mat4,
}
impl CameraBodyMatrices {
    /// Builds mutually inverse view matrices from a position and unit quaternion.
    pub fn from_position_rotation(position: Vec3, rotation: Quat) -> Self {
        let inverse_view = Mat4::from_rotation_translation(rotation, position);
        // Rigid rotations invert by transposition, avoiding a general matrix inverse.
        let inverse_rotation = Mat4::from_quat(rotation).transpose();
        let view = inverse_rotation * Mat4::from_translation(-position);
        Self { view, inverse_view }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct CameraMatrices {
    pub projection: Mat4,
    pub inverse_view: Mat4,
}
impl CameraMatrices {
    pub fn eye_position(self) -> Vec3 {
        self.inverse_view.transform_point3(Vec3::ZERO)
    }
    pub fn eye_direction(self) -> Vec3 {
        self.inverse_view.transform_vector3(Vec3::NEG_Z).normalize()
    }
    pub fn eye_right(self) -> Vec3 {
        self.inverse_view.transform_vector3(Vec3::X).normalize()
    }
    pub fn aspect_ratio(self) -> f32 {
        self.projection.y_axis.y / self.projection.x_axis.x
    }
    pub fn xz_projective_rotation(self) -> Quat {
        // 原140c96e70把Vec4轴乘完整inverse_view；w=0项也参与，
        // 不能用忽略平移列的transform_vector3替代异常值/负零传播。
        let matrix = self.inverse_view;
        let y_zero = matrix.y_axis * 0.0;
        let w_zero = matrix.w_axis * 0.0;
        let right = ((matrix.x_axis + y_zero) + matrix.z_axis * 0.0) + w_zero;
        let reciprocal = 1.0 / ((right.y * right.y + right.x * right.x)
            + right.z * right.z).sqrt();
        let projected = Vec2::new(right.z * reciprocal, -right.x * reciprocal);
        let projected = if projected.x.abs().max(projected.y.abs()) > 0.00001 {
            projected
        } else {
            let forward = ((matrix.x_axis * 0.0 + y_zero) - matrix.z_axis) + w_zero;
            let reciprocal = 1.0 / ((forward.y * forward.y + forward.x * forward.x)
                + forward.z * forward.z).sqrt();
            Vec2::new(forward.x * reciprocal, forward.z * reciprocal)
        };
        crate::xz_rotation_from_forward(projected)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct CameraLens {
    pub field_0: f32,
    pub vertical_fov_degrees: f32,
    pub target_vertical_fov_degrees: f32,
    pub field_c: f32,
    pub field_10: f32,
    pub field_14: f32,
    pub field_18: f32,
    pub field_1c: f32,
    pub field_20: f32,
    pub field_24: f32,
    pub field_28: bool,
}
impl CameraLens {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        field_0: f32,
        fov: f32,
        field_c: f32,
        field_10: f32,
        field_28: bool,
        field_18: f32,
        field_1c: f32,
        field_20: f32,
    ) -> Self {
        Self {
            field_0,
            vertical_fov_degrees: fov,
            target_vertical_fov_degrees: fov,
            field_c,
            field_10,
            field_14: fov,
            field_18,
            field_1c,
            field_20,
            field_24: field_18,
            field_28,
        }
    }
    pub fn immediately_set_vertical_fov_degrees(&mut self, fov: f32) {
        self.vertical_fov_degrees = fov;
        self.target_vertical_fov_degrees = fov;
    }
}

#[cfg(test)]
#[path = "camera/tests.rs"]
mod tests;

use bevy_ecs::resource::Resource;

#[derive(Resource, Clone, Debug)]
pub struct TaaJitterSet(pub Vec<Vec2>);

impl Default for TaaJitterSet {
    fn default() -> Self {
        // Original range bytes are {start:1,end:128,exhausted:false}.
        Self(
            (1..=128)
                .map(|n| {
                    fn radical_inverse(mut index: u32, base: u32) -> f32 {
                        let reciprocal = 1.0 / base as f32;
                        let mut fraction = reciprocal;
                        let mut value = 0.0;
                        loop {
                            value += (index % base) as f32 * fraction;
                            index = (index as f32 * reciprocal) as u32;
                            fraction *= reciprocal;
                            if index == 0 {
                                break;
                            }
                        }
                        value - 0.5
                    }
                    Vec2::new(radical_inverse(n, 2), radical_inverse(n, 3))
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod native_taa_tests {
    use super::*;
    #[test]
    fn original_range_is_inclusive_one_through_128() {
        let jitter = TaaJitterSet::default();
        assert_eq!(jitter.0.len(), 128);
        assert_eq!(jitter.0[0].x, 0.0);
        assert_eq!(jitter.0[1].x, -0.25);
        assert_eq!(jitter.0[2].x, 0.25);
        assert_eq!(jitter.0[127].x, -0.49609375);
    }
}
