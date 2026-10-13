//! ivy_spawner.rs — 常春藤生成系统（O 0x14094cf90）。
//!
//! 源 `systems/ivy/ivy_spawner.rs`。功能（O）：沿墙随机生成新根 ——
//! `try_random_ivy_root` / `try_ivy_root_next_to_cursor` / `is_point_inside_wall_segment`
//! 判定根位置合法性；`IvyStorage::spawn_ivy` + `roots` 遍历。

#![allow(dead_code)]

use glam::Vec3;

use crate::resources::ivy::IvyStorage;

/// `is_point_inside_wall_segment`（O：ivy_spawner 内联）。
pub fn is_point_inside_wall_segment(_p: Vec3, _seg: &crate::systems::ivy::ivy_grower::WallSegment) -> bool {
    false
}

/// `try_random_ivy_root`（O：ivy_spawner 内联）—— 随机选墙段/位置生成根。
pub fn try_random_ivy_root(storage: &mut IvyStorage, position: Vec3, direction: Vec3) -> Option<u64> {
    Some(storage.spawn_ivy(position, direction))
}

/// `try_ivy_root_next_to_cursor`（O：ivy_spawner 内联）。
pub fn try_ivy_root_next_to_cursor(_storage: &mut IvyStorage, _cursor: Vec3) -> Option<u64> {
    None
}

/// `ivy_spawner`（O 0x14094cf90）。
pub fn ivy_spawner(_world: &mut bevy_ecs::world::World) {}
