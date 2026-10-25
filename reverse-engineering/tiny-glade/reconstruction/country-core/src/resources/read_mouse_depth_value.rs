//! read_mouse_depth_value.rs — 鼠标深度回读资源（WP12 facade）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 鼠标深度值（O：`MouseDepthValue`；布局 U）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct MouseDepthValue {
    pub raw: Gap,
}
