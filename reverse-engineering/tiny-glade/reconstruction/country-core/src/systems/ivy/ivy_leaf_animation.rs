//! ivy_leaf_animation.rs — 叶生长动画（O 0x140b0e380）。
//!
//! 源 `systems/ivy/ivy_leaf_animation.rs`；组件 `IvyLeafIsGrowing`。
//! 遍历 `IvyLeafStorage::all_leaves_mut`，推进新叶生长（尺寸/朝向）直至完成。

#![allow(dead_code)]

use bevy_ecs::component::Component;

/// 叶生长中标记（O：`IvyLeafIsGrowing`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct IvyLeafIsGrowing(pub bool);

/// `ivy_leaf_animation`（O 0x140b0e380）。
pub fn ivy_leaf_animation(_world: &mut bevy_ecs::world::World) {}
