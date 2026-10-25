//! wall_trigger_volume.rs — 墙触发体积（WP12 系统向 facade）。
//!
//! O 名；布局 U（typed gap）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// 墙触发体积（O 名；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct WallTriggerVolume {
    pub raw: Gap,
}

impl WallTriggerVolume {
    pub fn is_valid(&self) -> bool {
        true
    }
}

/// 墙触发体积参数（O 名；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct WallTriggerVolumeParams {
    pub raw: Gap,
}

impl WallTriggerVolumeParams {
    /// O：形状编辑默认参数。
    pub fn edit_shape_default() -> Self {
        Self { raw: Gap }
    }
    /// O：u 区间。
    pub fn u_range(&self) -> (f32, f32) {
        (0.0, 1.0)
    }
}
