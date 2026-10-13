//! roof_animation.rs — RoofAnimation / RoofMarkedForDeletion（WP9 扩展）。
//!
//! PDB 骨架 `resources/roofs/roof_animation.rs`（5 行）。命名实体 `RoofAnimation`（**组件**，
//! `Option<&RoofAnimation>`，nani 系统消费）、`RoofMarkedForDeletion`（**Resource**，delete_roof 写）。
//! 函数：`is_being_animated`（0x1408dbc20）、`new`（0x1408dbc40）、impl$2/impl$4
//! register_required_components（0x1408e06d0/0x1408e0740，bevy 胶水）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 屋顶动画参数（~8B；WP6 已恢复 nani 消费，stone_floor 行 stride 0x8）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofAnimation {
    pub height_param: f32,
}

impl RoofAnimation {
    /// RoofAnimation::new（O：0x1408dbc40）。
    pub fn new() -> Self {
        Self { height_param: 0.0 }
    }
    /// RoofAnimation::is_being_animated（O：0x1408dbc20；正文简，读标记）。
    pub fn is_being_animated(&self) -> bool {
        // 标记位语义 U（decomp void()）；此处以非零高度参数为占位，防臆造。
        self.height_param != 0.0
    }
    /// 布局占位（U：动画高度/衰减参数集）。
    #[allow(dead_code)]
    pub fn _gap(&self) -> Gap {
        Gap
    }
}

/// 待删屋顶（Resource，delete_roof::mark_roof_for_deletion 写入）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RoofMarkedForDeletion(pub Gap);
