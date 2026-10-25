//! hide_trees.rs — 墙旁 / 区域隐藏树（B 0x14226a260，O；2026-10-17 fresh 反编译 +
//! 反汇编复核）。
//!
//! ## fresh 复核结论（2026-10-17 restorer-tree-forest-remaining）
//!
//! 系统签名（PDB `file_interface_candidates`）：
//! `Query<(&mut TreeAnimation, &PrefabInstanceData, &PrefabInstance, &TreeComponent)>,
//!  Res<TreeStorage>, EventReader<HideTreesinAreaCmd>, EventReader<HideTreesNextToWallCmd>,
//!  Res<WallSpatialHash>, EventWriter<DeleteTreeCmd>, ResMut<History>`。
//!
//! 两个事件分支（0x14226a3ce 起 / 0x14226a8bc 起）：
//! - **NextToWall（事件 stride 0x18）**：逐 `TreeStorage::iter()` 条目取候选
//!   （0x14226a42e 目标 0x140a1b100 为 ICF 折叠的 hashmap iter——*不是*
//!   `Decorators::iter_removed_ids`，旧报告误判）；从 Query 的 0x30 B 放置记录取
//!   `PlacementDataSsbo::position`（0x140ad51a0）；`Circle2d::new(pos.x, pos.z, 0.4)`
//!   （0x140c97540）+ `WallSpatialHash::query_with_circle_callback`（0x1425df320，
//!   回调对墙段最近距离 `< 0.4` 时清标志）；命中后写可见性数组
//!   （`u32[id] = tick`、`byte[record + id*0x10 + 0xc] = 1`），事件 payload byte
//!   `+8`（事件基址 +0x10）== 0 时跳过删除，否则 `DeleteTreeCmd{id}` + `History::add`。
//! - **Area（事件 stride 0x30）**：shape bit @payload+0（事件基址 +8）；
//!   `bit==0 → Circle2d::with_expanded_diameter(delta@+0x20)`、
//!   `bit==1 → Rectangle2d::with_expanded_size(delta@+0x20)`，再 `contains_point`
//!   （0x140c980c0/0x140c980f0/0x140c9a730/0x140c9a630）；距离/寿命门
//!   `f32@+0x1c`（`<=0` 跳过，或 `value - delta <= 4.0` 才继续）；删除门
//!   byte@payload+0x1c（事件基址 +0x24）== 1。
//!
//! ## 未实现（core 类型缺口）
//!
//! - `HideTreesinAreaCmd` 现仅 `{center, radius}`，缺 **shape bit@+0**、
//!   `f32@+0x1c` 距离门、`delta@+0x20`、删除门 byte@+0x24（stride 0x30 载荷）。
//! - `HideTreesNextToWallCmd` 现仅 `{center, radius}`，native 为 stride 0x18
//!   `{?, 删除门 byte@+8}`；中心/半径不来自命令（来自树位置 + 常量 0.4）。
//! - 隐藏写目标是 native 可见性数组 + 记录 `+0xc` 字节；core 无对应资源/字段。
//! - `TreeStorage` 条目在 core 仅 `{entity, position}`；native 条目 0x48 B 记录的
//!   `+0x10` 可见性索引与 `+0x14` 派发槽未建模。
//!
//! 解锁条件：上述字段落入 country-core（`tree_storage.rs`）后，本函数可按上方
//! 两个分支逐语句重建。当前保持诚实空体，**不以近似体冒充**。

use bevy_ecs::prelude::*;

use country_core::components::{PrefabInstanceData, TreeAnimation, TreeComponent};
use country_core::resources::history::tree_edit::DeleteTreeCmd;
use country_core::resources::history::History;
use country_core::resources::prefab::PrefabInstance;
use country_core::resources::tree_storage::{
    HideTreesinAreaCmd, HideTreesNextToWallCmd, TreeStorage,
};
use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

/// `hide_trees` @0x14226a260 — **未实现（core-type-gap，见模块文档）**。
///
/// 未实现面：两个命令载荷（shape bit / 距离门 / 删除门）与可见性写目标在
/// country-core 无承载字段；候选源已复核为 `TreeStorage::iter()`（非
/// `Decorators::iter_removed_ids`）。解锁后可按模块文档逐语句重建。
#[allow(clippy::too_many_arguments)]
pub fn hide_trees<S: Send + Sync + 'static>(
    mut trees: Query<(
        &mut TreeAnimation,
        &PrefabInstanceData,
        &PrefabInstance,
        &TreeComponent,
    )>,
    storage: Res<TreeStorage>,
    mut area_cmds: EventReader<HideTreesinAreaCmd>,
    mut wall_cmds: EventReader<HideTreesNextToWallCmd>,
    walls: Res<WallSpatialHash>,
    mut deletes: EventWriter<DeleteTreeCmd>,
    mut history: ResMut<History<S>>,
) {
    // 诚实空体：country-core 事件载荷/可见性数组未落库（见模块文档「未实现」）。
    let _ = (
        &mut trees,
        &storage,
        &mut area_cmds,
        &mut wall_cmds,
        &walls,
        &mut deletes,
        &mut history,
    );
}
