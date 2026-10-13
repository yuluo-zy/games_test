//! drawing_input_smoother.rs — 绘制输入平滑（WP12 facade）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 平滑后的原始输入（O：`RawInputSmoothed`；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RawInputSmoothed {
    pub raw: Gap,
}

impl RawInputSmoothed {
    pub fn is_empty(&self) -> bool {
        true
    }
    pub fn point_count(&self) -> usize {
        0
    }
    pub fn points(&self) -> Vec<glam::Vec3> {
        Vec::new()
    }
}
