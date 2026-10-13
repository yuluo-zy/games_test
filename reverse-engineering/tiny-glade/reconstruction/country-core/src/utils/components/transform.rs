//! utils/components/transform.rs — 游戏自有 `Transform`（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`crates/utils/src/components/transform.rs`
//! （`7959b9d1...`；**utils crate，游戏自有 util**，非 bevy 依赖——按项目边界恢复外壳，
//! 数学复用 glam 0.29.3）。
//!
//! PDB 方法（O）：`from_translation` / `with_translation` / `from_scale` /
//! `from_translation_scale` / `identity` / `compute_matrix` / `compute_affine`
//! (0x1407a45b0) / `forward` (0x1407a4700) / `look_at` (0x140c9ce00) /
//! `abs_diff_eq` (0x140c9d0d0) / `Default` (0x140c9d180)。
//!
//! 布局（O，B 阶段 i_infra 按机器码修正；旧文档写成 `{translation @0, rotation @0xC, scale @0x1C}` 是错的）：
//! `{ rotation: Quat @0x0, translation: Vec3 @0x10, scale: Vec3 @0x1C }`，有效数据 0x28 B、因 `Quat` 16 字节对齐而占 0x30 B。
//! 证据：`compute_affine`（0x1407a45b0）`movaps xmm8,[rdx]`（四元数）、`movaps xmm2,[rdx+0x10]`（平移）、`movups xmm1,[rdx+0x1c]`
//! （缩放）；`Default`（0x140c9d180，50 B）写 `[rcx]=(0,0,0,1)`（旋转恒等）、`[rcx+0x10]=0`/`[rcx+0x18]=0`（平移零）、
//! `[rcx+0x1c]=(1,1)`/`[rcx+0x24]=1`（缩放一）。即 bevy 的 `{translation, rotation, scale}` 字段序并**不**适用于游戏自有 `Transform`
//! （按字段名访问的代码不受影响，只有按偏移的消费者——critters 的 `ButterflySsbo::from_components` 等——依赖它）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use glam::{Mat4, Quat, Vec3};

/// 游戏自有 `Transform`（O 布局：旋转 @0、平移 @0x10、缩放 @0x1c；见模块文档）。
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// 旋转四元数（+0x0，O）。
    pub rotation: Quat,
    /// 平移（+0x10，O）。
    pub translation: Vec3,
    /// 缩放（+0x1C，O）。
    pub scale: Vec3,
}

impl Default for Transform {
    /// O 0x140c9d180：恒等变换（旋转 (0,0,0,1)、平移 0、缩放 1）——不是 `derive(Default)` 的全零缩放。
    fn default() -> Self {
        Transform::identity()
    }
}

impl Transform {
    /// `Transform::IDENTITY` / `impl$1::default`（0x140c9d180，O）。
    pub fn identity() -> Self {
        Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// `from_translation`（O PDB LF_FUNC_ID）。
    pub fn from_translation(translation: Vec3) -> Self {
        Transform {
            translation,
            ..Transform::identity()
        }
    }

    /// `with_translation`（O PDB LF_FUNC_ID）。
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    /// `from_scale`（O PDB LF_FUNC_ID）。
    pub fn from_scale(scale: Vec3) -> Self {
        Transform {
            scale,
            ..Transform::identity()
        }
    }

    /// `from_translation_scale`（O PDB LF_FUNC_ID）。
    pub fn from_translation_scale(translation: Vec3, scale: Vec3) -> Self {
        Transform {
            translation,
            scale,
            ..Transform::identity()
        }
    }

    /// `compute_affine`（0x1407a45b0，O）：TRS 仿射矩阵（glam 复用）。
    /// 反编译结构：旋转列 × scale + 平移进第 4 列。
    pub fn compute_affine(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// `compute_matrix`（O PDB LF_FUNC_ID；与 compute_affine 同族）。
    pub fn compute_matrix(&self) -> Mat4 {
        self.compute_affine()
    }

    /// `forward`（0x1407a4700，O）：本地 -Z 方向经旋转。
    pub fn forward(&self) -> Vec3 {
        self.rotation * -Vec3::Z
    }

    /// `look_at`（0x140c9ce00，O）。
    pub fn look_at(&mut self, _target: Vec3, _up: Vec3) {
        todo!("recover look_at")
    }

    /// `abs_diff_eq`（0x140c9d0d0，O）。
    pub fn abs_diff_eq(&self, _other: &Self, _max_abs_diff: f32) -> bool {
        todo!("recover abs_diff_eq")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    /// 布局（O：`compute_affine` 0x1407a45b0 的读偏移、`Default` 0x140c9d180 的写偏移）。
    #[test]
    fn layout_is_rotation_translation_scale_at_0_0x10_0x1c() {
        assert_eq!(offset_of!(Transform, rotation), 0x0);
        assert_eq!(offset_of!(Transform, translation), 0x10);
        assert_eq!(offset_of!(Transform, scale), 0x1c);
        // 有效数据 0x28 B；Quat 的 16 字节对齐把大小补到 0x30。
        assert_eq!(align_of::<Transform>(), align_of::<Quat>());
        assert_eq!(size_of::<Transform>(), 0x30);
    }

    /// O 0x140c9d180：`Default` 是恒等变换（`derive(Default)` 会给出零缩放）。
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
