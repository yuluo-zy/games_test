//! visual.rs — 楼梯可视化资源（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/stairs/visual.rs`；PDB 命名实体 5 个：
//! `MagicSupports` / `StairsEdgeVisualState` / `StairsFloatingBrickParams` /
//! `StairsFloorMeshChunk` / `StairsVisualState`。
//! `StairsFloorMeshChunk` 在 ECS 查询中作**组件**
//! （`Query<(&NaniActiveInstance, &StairsFloorMeshChunk), Or<(Changed<Chunk>,
//! Changed<NaniActiveInstance>)>>`，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 楼梯地板 mesh 块（O：ECS `&StairsFloorMeshChunk`；`nani_update_stairs_floor` 读）。
/// 布局 U（typed gap；mesh/顶点数据）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct StairsFloorMeshChunk(pub Gap);

/// 楼梯可视化状态（O：命名实体；未消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsVisualState(pub Gap);
