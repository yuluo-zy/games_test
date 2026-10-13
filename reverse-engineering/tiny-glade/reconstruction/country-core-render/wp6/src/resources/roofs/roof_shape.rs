//! roof_shape.rs — 屋顶形状资源（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/roofs/roof_shape.rs`；PDB 命名实体 8 个：
//! `Roof` / `RoofShapeParams` / `RoofType` / `RoofRidgeDirection` /
//! `GableRoofStyle` / `FlatRoofStyle` / `FlatRoofRimStyle` / `FlatRoofMaterial`。
//! `Roof` 在 ECS 查询中作**组件**（`Query<&Roof>` / `Option<&Roof>`，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 屋顶形状（O：ECS `&Roof`；`nani_update_stone_floor` 读 +8 字节 bit0 与
/// `Circle2d::get_tangent_at_u` 路径；`nani_update_curved_halftimber` 读屋顶高度）。
/// 布局 I：≥0x58 字节（stone_floor 行 stride 0x58）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Roof(pub Gap);

/// 屋顶形状参数（O：命名实体；未消费）。
#[derive(Clone, Copy, Debug, Default)]
pub struct RoofShapeParams(pub Gap);
