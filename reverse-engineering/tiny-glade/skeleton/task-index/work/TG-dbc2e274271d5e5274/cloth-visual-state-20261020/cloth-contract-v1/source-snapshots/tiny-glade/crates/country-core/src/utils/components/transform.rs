//! 游戏实体的平移、旋转和缩放组件；数学实现复用 glam 与共享 Transform。
//!
//! 保留 rotation / translation / scale 的字段序，兼容已有实例上传消费方。

use bevy_ecs::component::Component;
use glam::{Mat4, Quat, Vec3};

/// 游戏实体的局部变换。
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// 单位旋转四元数。
    pub rotation: Quat,
    /// 世界或父节点空间的位置。
    pub translation: Vec3,
    /// 三轴缩放。
    pub scale: Vec3,
}

impl Default for Transform {
    /// 默认缩放为一，保证新建实体可见。
    fn default() -> Self {
        Transform::identity()
    }
}

impl Transform {
    /// 单位变换。
    pub fn identity() -> Self {
        Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// 仅设置平移。
    pub fn from_translation(translation: Vec3) -> Self {
        Transform {
            translation,
            ..Transform::identity()
        }
    }

    /// 保持旋转和缩放，仅替换平移。
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    /// 仅设置缩放。
    pub fn from_scale(scale: Vec3) -> Self {
        Transform {
            scale,
            ..Transform::identity()
        }
    }

    /// 设置平移和缩放，旋转保持单位四元数。
    pub fn from_translation_scale(translation: Vec3, scale: Vec3) -> Self {
        Transform {
            translation,
            scale,
            ..Transform::identity()
        }
    }

    /// 按缩放、旋转、平移顺序构造矩阵。
    pub fn compute_affine(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// compute_affine 的矩阵接口别名。
    pub fn compute_matrix(&self) -> Mat4 {
        self.compute_affine()
    }

    /// 旋转后的本地 -Z 方向，不受缩放影响。
    pub fn forward(&self) -> Vec3 {
        self.rotation * -Vec3::Z
    }

    fn as_shared_transform(&self) -> ::utils::components::transform::Transform {
        ::utils::components::transform::Transform {
            t: self.translation,
            r: self.rotation,
            s: self.scale,
        }
    }

    /// 朝向目标；退化目标和 up 向量的处理委托共享实现。
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let mut transform = self.as_shared_transform();
        transform.look_at(target, up);
        self.rotation = transform.r;
    }

    /// 分量误差比较；NaN 和负容差均不相等。
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.as_shared_transform()
            .abs_diff_eq(&other.as_shared_transform(), max_abs_diff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout_is_rotation_translation_scale_at_0_0x10_0x1c() {
        assert_eq!(offset_of!(Transform, rotation), 0x0);
        assert_eq!(offset_of!(Transform, translation), 0x10);
        assert_eq!(offset_of!(Transform, scale), 0x1c);
        // 有效数据 0x28 B；Quat 的 16 字节对齐把大小补到 0x30。
        assert_eq!(align_of::<Transform>(), align_of::<Quat>());
        assert_eq!(size_of::<Transform>(), 0x30);
    }

    #[test]
    fn default_is_identity() {
        let d = Transform::default();
        assert_eq!(d.rotation, Quat::IDENTITY);
        assert_eq!(d.translation, Vec3::ZERO);
        assert_eq!(d.scale, Vec3::ONE);
        assert_eq!(d, Transform::identity());
    }

    #[test]
    fn affine_is_trs() {
        let t = Transform {
            rotation: Quat::from_rotation_y(core::f32::consts::FRAC_PI_2),
            translation: Vec3::new(1.0, 2.0, 3.0),
            scale: Vec3::new(2.0, 2.0, 2.0),
        };
        let p = t.compute_affine().transform_point3(Vec3::X);
        assert!((p - Vec3::new(1.0, 2.0, 1.0)).length() < 1e-5, "{p:?}");
        assert!((t.forward() - Vec3::NEG_X).length() < 1e-6);
    }
}
