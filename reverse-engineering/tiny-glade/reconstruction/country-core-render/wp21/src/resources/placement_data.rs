//! placement_data.rs — 放置数据 SSBO（WP21）。
//!
//! O：`PlacementDataSsbo` 记录 `{ position: Vec3 @+0x0, scale: f32 @+0xc, angle: f32
//! @+0x10 }`；`to_affine3a`（0xad5020）绕 Y 轴旋转×scale（sin/cos(angle/2)），`position`
//!（0xad51a0）取位置。

use glam::{Mat4, Vec3};

use crate::typed_gap::Gap;

/// `PlacementDataSsbo`（O：放置记录 → Affine3A）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlacementDataSsbo {
    /// +0x0 位置（O）。
    pub position: Vec3,
    /// +0xc 缩放（O）。
    pub scale: f32,
    /// +0x10 旋转角（弧度，绕 Y；O）。
    pub angle: f32,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl Default for PlacementDataSsbo {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            scale: 1.0,
            angle: 0.0,
            _gap: Gap::default(),
        }
    }
}

impl PlacementDataSsbo {
    /// `to_affine3a`（O 0xad5020）：绕 Y 旋转矩阵×scale + 平移。
    ///
    /// 矩阵按 O 写序转录：m00=scale*(1−2s²)、m02=−scale*2sc、m20=scale*2sc、
    /// m22=scale*(1−2s²)、m11=scale（s=sin(a/2), c=cos(a/2)）。
    pub fn to_affine3a(&self) -> Mat4 {
        let (s, c) = (self.angle * 0.5).sin_cos();
        let two_sc = 2.0 * s * c;
        let one_minus = 1.0 - 2.0 * s * s;
        Mat4::from_cols_array(&[
            self.scale * one_minus,
            0.0,
            -self.scale * two_sc,
            0.0,
            0.0,
            self.scale,
            0.0,
            0.0,
            self.scale * two_sc,
            0.0,
            self.scale * one_minus,
            0.0,
            self.position.x,
            self.position.y,
            self.position.z,
            self.position.z,
        ])
    }
    /// `to_transform`（O 名，6 内联）。
    pub fn to_transform(&self) -> Mat4 {
        self.to_affine3a()
    }
    /// `position`（O 0xad51a0）。
    pub fn position(&self) -> Vec3 {
        self.position
    }
}

/// `default_anim_t`（O 名，2 内联；放置动画参数 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DefaultAnimT(pub f32);

pub fn default_anim_t() -> f32 {
    0.0
}
