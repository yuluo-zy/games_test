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
//! 布局（O：`compute_affine` 反编译读 self 10 个 f32）：`{ translation: Vec3 @0x0,
//! rotation: Quat @0xC, scale: Vec3 @0x1C }` = 40B，与 bevy/glam Transform 兼容。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use glam::{Mat4, Quat, Vec3};

/// 游戏自有 `Transform`（O 布局；B 兼容 bevy 的 {translation, rotation, scale}）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Transform {
    /// 平移（+0x0，O）。
    pub translation: Vec3,
    /// 旋转四元数（+0xC，O）。
    pub rotation: Quat,
    /// 缩放（+0x1C，O）。
    pub scale: Vec3,
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
