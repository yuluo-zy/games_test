//! `stairs_collider.rs` — 楼梯碰撞体注册标记资源（B `resources/stairs/stairs_collider.rs`，
//! 1 胶水，无独立业务函数）。
//!
//! rapier 边界：实际碰撞体注册/撤除在 `systems/collision/world.rs`（R1 域）与
//! `resources/stairs/structural_graph`（本域）；本文件仅标记资源。
//! 章节：`chapters/resources/stairs/stairs_collider.rs.md`。
use bevy_ecs::resource::Resource;

/// 楼梯碰撞体注册标记（B `StairsCollider`，default 胶水）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsCollider;
