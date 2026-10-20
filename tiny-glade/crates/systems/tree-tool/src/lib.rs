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

/// `<system_tree::TreePlugin as bevy_app::Plugin>::build`（0x140650520）。
///
/// 机器码（逐指令 + 各系统 `Box<dyn System>` 类型名核对）：
/// 1. `Schedules::add_systems(OnExit(AppLoadingState::Game), reset_tree_tool)`（0x1406505b7；
///    `MOV DL,1` = `AppLoadingState::Game`，label 单体 0x140e9b080 里 intern 的是 `OnExit<_>`）。
/// 2. `SubApp::add_systems(Update, (replay_tree.in_set(GameSystemSet(0x28)),
///    (ui_delete_tree, ui_move_tree, ui_place_tree, place_tree, hide_trees, process_tree_cmds,
///    despawn_trees, tree_move_bend, recalc_tree_position_on_terrain_height_change,
///    convert_preplaced_trees).chain_ignore_deferred().in_set(GameSystemSet(0x1b))))`
///    （in_set 0x1406505c9 `DL=0x28`；chain_ignore_deferred 0x1406505d2，顺序取自单体 0x140e9d530
///    的 `into_configs` 调用序；in_set_inner 0x140650602 `MOV byte,0x1b`；add_systems 0x1406506ab）。
///    无 `run_if`。
/// 3. `App::insert_resource(TreeStorage::default())`（0x14065072f；空 HashMap + next_index 0）。
/// 4. 7 次 `register_event`（均带 TypeId 判存在）：PlaceTreeCmd（0x140650795）、MoveTreeCmd
///    （0x1406507f1）、DeleteTreeCmd（0x14065084d）、InsertTreeCmd（0x1406508a9）、
///    SpawnPreplacedTrees（0x140650905）、HideTreesinAreaCmd（0x140650961）、
///    HideTreesNextToWallCmd（0x1406509c5 尾调 0x140e9fbb0）。
/// 泛型实参取宿主已有的 `MaybePendingSnapshotId`。
pub struct TreePlugin;

impl bevy_app::Plugin for TreePlugin {
    fn build(&self, app: &mut bevy_app::App) {
        use bevy_app::Update;
        use bevy_state::state::OnExit;
        use country_core::app_loading_state::AppLoadingState;
        use country_core::resources::history::tree_edit::{DeleteTreeCmd, InsertTreeCmd, SpawnPreplacedTrees};
        use country_core::resources::history::MaybePendingSnapshotId as S;
        use country_core::resources::tree_storage::{
            HideTreesNextToWallCmd, HideTreesinAreaCmd, MoveTreeCmd, TreeStorage,
        };
        use country_core::systems::GameSystemSet;

        app.add_systems(OnExit(AppLoadingState::Game), startup_tree_tool::reset_tree_tool);
        app.add_systems(
            Update,
            (
                replay_tree::<S>.in_set(GameSystemSet(0x28)),
                (
                    ui_delete_tree::<S>,
                    ui_move_tree::<S>,
                    ui_place_tree,
                    place_tree::<S>,
                    hide_trees::<S>,
                    process_tree_cmds::<S>,
                    despawn_trees,
                    tree_move_bend,
                    recalc_tree_position_on_terrain_height_change,
                    convert_preplaced_trees::<S>,
                )
                    .chain_ignore_deferred()
                    .in_set(GameSystemSet(0x1b)),
            ),
        );
        app.insert_resource(TreeStorage::default());
        app.add_event::<PlaceTreeCmd>();
        app.add_event::<MoveTreeCmd>();
        app.add_event::<DeleteTreeCmd>();
        app.add_event::<InsertTreeCmd>();
        app.add_event::<SpawnPreplacedTrees>();
        app.add_event::<HideTreesinAreaCmd>();
        app.add_event::<HideTreesNextToWallCmd>();
    }
}
