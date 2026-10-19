//! forest — 森林树管理与动画（组 A restorer，2026-10-17 还原）。
//!
//! 原 crate（B skeleton）：`crates/systems/forest/`（3 文件；10 符号 = 业务 4 /
//! 内联 2 / 胶水 4；`TwoWaySearcher` 判 dep 不恢复）。
//! - `manage_tree_prefabs`（0x141bf16c0）：森林树候选评估 + 生成/回收两阶段。
//! - `handplaced_tree_animation`（0x142279500）：手放树 canopy 淡入/淡出动画。
//! - `forest_tree_animation`（0x14227a0d0）：森林树生长/收缩动画 + 回收。
//! - `is_too_close_to_shape`（行 211，内联）：树-形状间距判定。
//!
//! `build`（0x140650c50，O）只做 Bevy 注册胶水：`insert_resource<InteractableForestPlacements>` +
//! `(manage_tree_prefabs, forest_tree_animation, handplaced_tree_animation).chain_ignore_deferred().in_set(Set)`
//! 注册到 Update。

pub mod manage_tree_prefabs;
pub mod tree_animation;

use bevy_ecs::prelude::*;

use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::tree_storage::InteractableForestPlacements;
use country_core::systems::prefab::{DespawnPrefabCmd, SpawnPrefabCmd};

use crate::manage_tree_prefabs::manage_tree_prefabs;
use crate::tree_animation::{forest_tree_animation, handplaced_tree_animation};

/// forest 系统集合（O：build 0x140650c50 `intern<dyn SystemSet>` + `in_set_inner`；原集合
/// 为 1 字节自定义 SystemSet 值（0x24 静态），名 U——按 core 域惯例落地为链集合）。
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ForestSystems {
    /// 树 prefab 管理 + 手放树动画 + 森林树动画（O：`chain_ignore_deferred` 串行链）。
    Trees,
}

/// 资源注册（O：build 0x140650c50 `App::insert_resource<InteractableForestPlacements>`）。
///
/// ⚠ 事件资源（SpawnPrefabCmd / DespawnPrefabCmd / ConstructTimberframeCmd）原 build 未注册
/// （`register_event` 面不在 0x140650c50）；prefab/timberframe 域宿主注册。此处保留
/// `init_resource` 使本 crate 独立可运行（接线惯例，非原 build 面，见 RESTORE.md §1）。
pub fn register_forest_events(world: &mut World) {
    world.init_resource::<InteractableForestPlacements>();
    // Current standalone adapter shares the actual two-zero initial state
    // proven in B run_game_thread14059f41d; preserve an existing App owner.
    world.init_resource::<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>();
    world.init_resource::<Events<SpawnPrefabCmd>>();
    world.init_resource::<Events<DespawnPrefabCmd>>();
    world.init_resource::<Events<ConstructTimberframeCmd>>();
}

/// 注册 CPU 系统（O：build 0x140650c50 的 add_systems 面：`(manage_tree_prefabs,
/// forest_tree_animation, handplaced_tree_animation).chain_ignore_deferred().in_set(ForestSystems)`）。
pub fn add_forest_cpu_systems<C: Send + Sync + 'static>(schedule: &mut Schedule) {
    schedule.add_systems(
        (
            manage_tree_prefabs::<C>,
            forest_tree_animation,
            handplaced_tree_animation,
        )
            .chain_ignore_deferred()
            .in_set(ForestSystems::Trees),
    );
}

/// `forest::impl$0::build`（0x140650c50，O 面）。
pub fn build<C: Send + Sync + 'static>(world: &mut World, schedule: &mut Schedule) {
    register_forest_events(world);
    add_forest_cpu_systems::<C>(schedule);
}
