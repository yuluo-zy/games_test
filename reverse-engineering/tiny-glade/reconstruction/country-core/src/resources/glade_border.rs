//! glade_border.rs — 小树林边界资源（WP12 facade）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 小树林边界（O：`GladeBorder`；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GladeBorder {
    pub raw: Gap,
}

impl GladeBorder {
    pub fn is_pos_inside(&self, _p: glam::Vec3) -> bool {
        true
    }
}
