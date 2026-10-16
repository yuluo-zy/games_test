//! system_tree — 树工具（组 A restorer，2026-10-16）。
//!
//! 原 crate（B skeleton）：`crates/systems/tree-tool/`（11 文件；24 符号 =
//! 业务 13 / 内联 3 / 胶水 8）。
//! 门禁（O，re-analyst-A 契约 §5）：ui_place_tree `mode==0x10`、
//! ui_delete_tree `mode==9 && submode==6`、ui_move_tree `mode==0xb`；
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
/// 0x142269170 发射（0x18B 命令记录 {u64 id?, u64 打包位置, u32}，re-analyst
/// §2）、place_tree 0x142269970 消费）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct PlaceTreeCmd {
    /// 命令 id 槽（I：0x18B 首槽；place_tree 以 next_index 定实际 id）。
    pub id: u64,
    /// 打包位置（O：pack_2x16f_uint——点击位置打包面）。
    pub packed_position: u64,
    /// 0x18B 记录载荷（I）。
    pub extra: u32,
}

/// `system_tree::impl$0::build` @0x140650520（O 面：注册树系统族 + 8 个
/// TypeId 检查/插入的资源 init，类型名 U）。
pub fn register_tree_tool_events(world: &mut World) {
    use country_core::resources::history::tree_edit::{DeleteTreeCmd, InsertTreeCmd, SpawnPreplacedTrees};
    use country_core::resources::path_stroke::PathDrawEvent;
    use country_core::resources::tree_storage::{HideTreesinAreaCmd, HideTreesNextToWallCmd, MoveTreeCmd};
    use country_core::systems::prefab::{DespawnPrefabCmd, SpawnPrefabCmd};
    use country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange;
    world.init_resource::<Events<PlaceTreeCmd>>();
    world.init_resource::<Events<InsertTreeCmd>>();
    world.init_resource::<Events<DeleteTreeCmd>>();
    world.init_resource::<Events<MoveTreeCmd>>();
    world.init_resource::<Events<SpawnPreplacedTrees>>();
    world.init_resource::<Events<HideTreesinAreaCmd>>();
    world.init_resource::<Events<HideTreesNextToWallCmd>>();
    world.init_resource::<Events<PathDrawEvent>>();
    world.init_resource::<Events<SpawnPrefabCmd>>();
    world.init_resource::<Events<DespawnPrefabCmd>>();
    world.init_resource::<Events<OnTerrainChange>>();
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
