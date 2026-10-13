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

impl Roof {
    /// O：屋顶类型字节（WP12 系统向；typed gap）。
    pub fn ty(&self) -> u8 {
        0
    }
    pub fn is_flat(&self) -> bool {
        true
    }
    pub fn is_flat_w_no_rim(&self) -> bool {
        true
    }
    pub fn height(&self) -> f32 {
        0.0
    }
    pub fn pivot_3d(&self) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
}

/// 屋顶形状参数（O：命名实体；WP12 系统向：`from_wall_shape` / `height_ws`）。
#[derive(Clone, Copy, Debug, Default)]
pub struct RoofShapeParams(pub Gap);

impl RoofShapeParams {
    /// O：从墙形状快照构造。
    pub fn from_wall_shape(_s: &crate::resources::walls::WallShapeSnapshot) -> Self {
        Self(Gap)
    }
    pub fn height_ws(&self) -> f32 {
        0.0
    }
}

/// O：`roof_height_01_from_wall_shape`（墙体形状 → 归一屋顶高）。
pub fn roof_height_01_from_wall_shape(_s: &crate::resources::walls::WallShapeSnapshot) -> f32 {
    0.0
}

/// O：`ridge_length_01_from_rectangle_ratio`。
pub fn ridge_length_01_from_rectangle_ratio(_r: f32) -> f32 {
    0.0
}
