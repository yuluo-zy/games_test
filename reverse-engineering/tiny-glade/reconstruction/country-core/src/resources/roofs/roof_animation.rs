//! roof_animation.rs — 屋顶动画资源（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/roofs/roof_animation.rs`；PDB 命名实体：
//! `RoofAnimation` / `RoofMarkedForDeletion`。
//! `RoofAnimation` 在 ECS 查询中作**组件**（`Option<&RoofAnimation>`，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 屋顶动画（O：ECS `Option<&RoofAnimation>`；`nani_update_stone_floor` 读）。
/// 布局 I：~8 字节（stone_floor 行 stride 0x8；动画高度参数）。
/// 屋顶动画（O：ECS `&RoofAnimation`；WP12 系统向 `is_being_animated`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofAnimation(pub Gap);

impl RoofAnimation {
    pub fn is_being_animated(&self) -> bool {
        false
    }
}

/// 待删屋顶（O：命名实体；未消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RoofMarkedForDeletion(pub Gap);
