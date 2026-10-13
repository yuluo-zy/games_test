//! roof_visual_shape.rs — 屋顶视觉形状（WP12 系统向 facade）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// 屋顶包围尺寸（O：`RoofBoundsDims`；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct RoofBoundsDims {
    pub raw: Gap,
}

impl RoofBoundsDims {
    pub fn as_rectangle(&self) -> Gap {
        Gap
    }
}
