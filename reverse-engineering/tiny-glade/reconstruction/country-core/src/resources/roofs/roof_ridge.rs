//! roof_ridge.rs — 屋顶屋脊尺寸（WP12 系统向 facade）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// 屋脊尺寸（O：`RoofRidgeDims`；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct RoofRidgeDims {
    pub raw: Gap,
}

impl RoofRidgeDims {
    /// O：由屋顶 + 墙状态计算。
    pub fn calc(
        _roof: &super::roof_shape::Roof,
        _wall: &crate::resources::walls::public_wall_state::PublicWallState,
    ) -> Self {
        Self { raw: Gap }
    }
    /// O：由比例计算。
    pub fn from(_r: f32) -> Self {
        Self { raw: Gap }
    }
}
