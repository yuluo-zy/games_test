//! system_tree — 树工具（组 A restorer，2026-10-16）。
//!
//! 原 crate（B skeleton）：`crates/systems/tree-tool/`（11 文件；24 符号 =
//! 业务 13 / 内联 3 / 胶水 8）。
//! 门禁（O，2026-10-17 fresh 反编译）：ui_place_tree `mode==0x10`（Tree）、
//! ui_delete_tree `mode==9 && submode==6`（Eraser(Tree)）、ui_move_tree
//! `mode_native==0x10`（Tree，`uVar8-5==0xb` 即 0x10；re-analyst 记 0xb 误）；
//! 进程全局 0x14362c098 共用（非 0 早退）。

pub mod convert_preplaced_trees;
pub mod hide_trees;
pub mod process_tree_cmds;
pub mod recalc_tree_position_on_terrain_height_change;
pub mod replay_tree;
pub mod startup_tree_tool;
pub mod tree_move_bend;
pub mod ui_delete_tree;
pub mod ui_move_tree;
pub mod ui_place_tree;

use bevy_ecs::prelude::*;

use crate::convert_preplaced_trees::convert_preplaced_trees;
use crate::hide_trees::hide_trees;
use crate::process_tree_cmds::{despawn_trees, process_tree_cmds};
use crate::recalc_tree_position_on_terrain_height_change::recalc_tree_position_on_terrain_height_change;
use crate::replay_tree::replay_tree;
use crate::tree_move_bend::tree_move_bend;
use crate::ui_delete_tree::ui_delete_tree;
use crate::ui_move_tree::ui_move_tree;
use crate::ui_place_tree::{place_tree, ui_place_tree};

/// 放置树命令（O：`system_tree::ui_place_tree::PlaceTreeCmd`；ui_place_tree
/// 0x142269170 发射、place_tree 0x142269970 消费）。
///
/// Fresh 反编译（2026-10-17）：事件槽 stride **0x18** = bevy EventInstance
/// （u64 id @+0）+ 载荷 Vec3 位置（x @+8、y @+0xc、z @+0x10）。写者：
/// `*(slot+8) = CONCAT44(remapped_y, x)`、`*(slot+0x10) = z`（decomp
/// ui_place_tree 0x142269170 行 155-167）。载荷**仅位置**，无 id/打包字段。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct PlaceTreeCmd {
    /// 点击位置（世界；y 为地形高度，水下重映射后）。
    pub position: glam::Vec3,
}

/// `system_tree::impl$0::build` @0x140650520（O 面：注册树系统族 + 7 个
/// 事件 init——PlaceTreeCmd/MoveTreeCmd/DeleteTreeCmd/InsertTreeCmd/
/// SpawnPreplacedTrees/HideTreesinAreaCmd/HideTreesNextToWallCmd；TypeId 守卫。
/// PathDrawEvent/SpawnPrefabCmd/DespawnPrefabCmd/OnTerrainChange 属其它插件域
/// （terrain/prefab），不在此注册）。
pub fn register_tree_tool_events(world: &mut World) {
    use country_core::resources::history::tree_edit::{DeleteTreeCmd, InsertTreeCmd, SpawnPreplacedTrees};
    use country_core::resources::tree_storage::{HideTreesinAreaCmd, HideTreesNextToWallCmd, MoveTreeCmd};
    world.init_resource::<Events<PlaceTreeCmd>>();
    world.init_resource::<Events<InsertTreeCmd>>();
    world.init_resource::<Events<DeleteTreeCmd>>();
    world.init_resource::<Events<MoveTreeCmd>>();
    world.init_resource::<Events<SpawnPreplacedTrees>>();
    world.init_resource::<Events<HideTreesinAreaCmd>>();
    world.init_resource::<Events<HideTreesNextToWallCmd>>();
}

/// 注册 CPU 系统（O：build 0x140650520 的 add_systems 面）。
pub fn add_tree_tool_cpu_systems<S: Send + Sync + 'static>(schedule: &mut Schedule) {
    schedule.add_systems((
        convert_preplaced_trees::<S>,
        hide_trees::<S>,
        process_tree_cmds::<S>,
        despawn_trees,
        recalc_tree_position_on_terrain_height_change,
        replay_tree::<S>,
        tree_move_bend,
        ui_delete_tree::<S>,
        ui_move_tree::<S>,
        ui_place_tree,
        place_tree::<S>,
    ));
}

/// `system_tree::impl$0::build`（0x140650520，O 面）。
pub fn build<S: Send + Sync + 'static>(world: &mut World, schedule: &mut Schedule) {
    register_tree_tool_events(world);
    add_tree_tool_cpu_systems::<S>(schedule);
}
