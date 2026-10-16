//! forest — 森林树管理与动画（组 A restorer，2026-10-16）。
//!
//! 原 crate（B skeleton）：`crates/systems/forest/`（3 文件；10 符号 = 业务 4 /
//! 内联 2 / 胶水 4；`TwoWaySearcher` 判 dep 不恢复）。
//! - `manage_tree_prefabs`（0x141bf16c0）：森林树候选评估 + 生成/回收两阶段。
//! - `handplaced_tree_animation`（0x142279500）：手放树 canopy 相机距离淡出。
//! - `forest_tree_animation`（0x14227a0d0）：森林树 sway/bend 动画。
//! - `is_too_close_to_shape`（行 211，内联）：树-形状间距判定。

pub mod manage_tree_prefabs;
pub mod tree_animation;

use bevy_ecs::prelude::*;

use crate::manage_tree_prefabs::manage_tree_prefabs;
use crate::tree_animation::{forest_tree_animation, handplaced_tree_animation};

/// `forest::impl$0::build` @0x140650c50（O 面：1 个 `init_resource`（36B 默认
/// 实例 0x142b8fc58——候选 `InteractableForestPlacements`，I）+ `add_systems` ×2
/// （0x140ea7c90 / 0x140ea7ff0——3 系统分两组注册，re-analyst-A 契约 §2）。
pub fn register_forest_events(world: &mut World) {
    use country_core::resources::events::ConstructTimberframeCmd;
    use country_core::resources::tree_storage::InteractableForestPlacements;
    use country_core::systems::prefab::{DespawnPrefabCmd, SpawnPrefabCmd};
    world.init_resource::<InteractableForestPlacements>();
    world.init_resource::<Events<SpawnPrefabCmd>>();
    world.init_resource::<Events<DespawnPrefabCmd>>();
    world.init_resource::<Events<ConstructTimberframeCmd>>();
}

/// 注册 CPU 系统（O：build 0x140650c50 的 add_systems 面）。
pub fn add_forest_cpu_systems<C: Send + Sync + 'static>(schedule: &mut Schedule) {
    schedule.add_systems(manage_tree_prefabs::<C>);
    schedule.add_systems(handplaced_tree_animation);
    schedule.add_systems(forest_tree_animation);
}

/// `forest::impl$0::build`（0x140650c50，O 面）。
pub fn build<C: Send + Sync + 'static>(world: &mut World, schedule: &mut Schedule) {
    register_forest_events(world);
    add_forest_cpu_systems::<C>(schedule);
}
