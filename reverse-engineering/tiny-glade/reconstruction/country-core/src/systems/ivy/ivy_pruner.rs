//! ivy_pruner.rs — 常春藤修剪系统（O）。
//!
//! 源 `systems/ivy/ivy_pruner.rs`；命令 `RemoveIvyCmd`。
//! `ivy_pruner`（0x140997410）：沿修剪刷移除受影响常春藤点（`Ivy::remove_root`/
//! `remove_last_point`/`IvyStorage::remove` + `IvyLeafStorage::delete_affected_leaves`）；
//! `is_point_still_valid`（0x140998dc0）判定点是否仍在墙有效区；
//! `ivy_pruner_cmd`（0x140999330）处理命令；`remove_ivy_at_location`
//! （0x140999860）。

#![allow(dead_code)]

use glam::Vec3;

/// 移除常春藤命令（O：`RemoveIvyCmd`）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct RemoveIvyCmd(pub Vec3);

/// `is_point_still_valid`（O 0x140998dc0）：点在墙有效区内。
pub fn is_point_still_valid(_p: Vec3, _wall: &crate::systems::collision::world::WallCurveSource) -> bool {
    false
}

/// `remove_ivy_at_location`（O 0x140999860）：位置 → 移除受影响点/叶。
pub fn remove_ivy_at_location(_world: &mut bevy_ecs::world::World, _location: Vec3) {}

/// `ivy_pruner`（O 0x140997410）。
pub fn ivy_pruner(_world: &mut bevy_ecs::world::World) {}

/// `ivy_pruner_cmd`（O 0x140999330）。
pub fn ivy_pruner_cmd(_world: &mut bevy_ecs::world::World) {}
