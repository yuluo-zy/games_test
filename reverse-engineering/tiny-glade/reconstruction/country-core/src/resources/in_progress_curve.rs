//! in_progress_curve.rs — 编辑中曲线资源（WP12 facade）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 编辑中曲线（O：`InProgressCurve`；徒手墙原始输入曲线）。
#[derive(Resource, Clone, Debug, Default)]
pub struct InProgressCurve {
    pub raw: Gap,
}

impl InProgressCurve {
    pub fn is_raw_input_curve_valid(&self) -> bool {
        false
    }
    pub fn last_raw_input_point(&self) -> Option<glam::Vec3> {
        None
    }
    pub fn first_raw_input_point(&self) -> Option<glam::Vec3> {
        None
    }
    pub fn add_weight_to_the_last_point(&mut self, _w: f32) {}
    pub fn raw_input_curve_length_ws(&self) -> f32 {
        0.0
    }
}
