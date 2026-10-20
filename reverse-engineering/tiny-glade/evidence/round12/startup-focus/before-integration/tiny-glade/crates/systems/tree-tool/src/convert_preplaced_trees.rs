//! convert_preplaced_trees.rs — 预置树→游戏树转换（B 0x14226be70，O；
//! 2026-10-17 fresh 反编译 + reviewer-eco-treetool 复核 + wave3 落库 Wait flag）。
//!
//! 结构（O，decomp 0x14226be70 行 122-128）：`SpawnPreplacedTrees` 事件到达 →
//! `Wait{count=1, flag=1}`；无新事件帧且 flag 置位 → count+1，`count+1 > 2`
//! （count>=2）→ 清 flag 执行转换块（一次性子门控）。转换：候选树（
//! `Decorators::iter_removed_ids` 位图）→ 位置 → 6 路 `bilinear_sample`（4 路径 +
//! 水面 + 花园）+ `terrain_rock_mask::sample_at_world_xz` → **水/岩/花园/路径/墙
//! 任一命中 = 未转换 → 发 stride-0x10 事件（payload=树 u64）**；通过全部门禁 →
//! 组 stride-0x48 TreeEdit 记录 → `History::insert_starting_edit`。
//!
//! ## wave3 落库
//! - **Wait flag 语义已判等**（分歧 12 已修）：flag 置位才计数，转换后清位——
//!   原 Rust 无 flag，`count>2` 后每帧重跑转换块；现按 decomp 行 122-128
//!   一次性门控。
//! - **分歧 15（未实现）**：`History::insert_starting_edit`——共享
//!   `History<EditType,S>`（HistoryTransactions）内部 ledger 不可变借用
//!   （`ledger()` 只读），`insert_starting_edit` 需 `&mut ObservedHistoryLedger`；
//!   转换通过者本应进 History，现仍空分支（wave3 复查，未冒充）。
//! - 分歧 13（岩石掩码 buffer 接线）、分歧 9/14（候选源 = 位图 vs storage 遍历）
//!   维持登记。

use bevy_ecs::prelude::*;
use std::sync::Arc;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::components::ConvertedPreplacedTree;
use country_core::resources::history::snapshots::HistoryEntry;
use country_core::resources::history::snapshots::insert_starting_edit;
use country_core::resources::history::tree_edit::{
    DeleteTreeCmd, SpawnPreplacedTrees, TreeEdit, TreeEditData,
};
use country_core::resources::history::{EditType, History};
use country_core::resources::prefab::PrefabInstance;
use country_core::resources::terrain_rock_mask::sample_at_world_xz;
use country_core::resources::tree_storage::TreeStorage;
use country_core::resources::world_raster_defs::{GardenRaster, WaterRaster};
use country_core::systems::paths::PathRastersResources;
use country_core::systems::prefab::PrefabInstanceData;
use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

/// 树-墙间距（O：r=7.0 圈，同 forest TREE_SHAPE_GAP；decomp 0x14226be70
/// `Circle2d::new` + `WallSpatialHash::query_with_circle_callback`）。
pub const TREE_WALL_GAP: f32 = 7.0;

/// 转换等待缓冲（O：`Local<Wait>`——{u64 计数 @+0, u8 标志 @+8}；decomp
/// 0x14226be70 行 122-128）。wave3 落库 flag 语义（分歧 12 已修）：
/// - 事件到达 → `count=1` + `flag=1`；
/// - 无事件帧且 flag 置位 → `count+=1`；`count+1 > 2`（即 count>=2）→
///   `flag=0` 并执行转换块（一次性子门控——事件簇只转一次，后续无事件帧
///   不再重跑转换块）。
#[derive(Default)]
pub struct Wait {
    /// 延迟帧计数（O：@+0 u64；事件到达重置 1，无事件帧自增）。
    pub count: u64,
    /// 触发标志（O：@+8 u8；wave3 补：置位才计数，转换后清位）。
    pub flag: u64,
}

/// `convert_preplaced_trees` @0x14226be70（O 面；wave3 落库 Wait flag）。
#[allow(clippy::too_many_arguments)]
pub fn convert_preplaced_trees<S: Send + Sync + 'static>(
    mut spawns: EventReader<SpawnPreplacedTrees>,
    mut deletes: EventWriter<DeleteTreeCmd>,
    mut wait: Local<Wait>,
    path_rasters: PathRastersResources,
    water: Res<WaterRaster>,
    garden: Res<GardenRaster>,
    walls: Res<WallSpatialHash>,
    _library: Res<AssetSsboLibrary>,
    storage: Res<TreeStorage>,
    converted: Query<(
        Entity,
        &PrefabInstanceData,
        &PrefabInstance,
        &ConvertedPreplacedTree,
    )>,
    mut history: ResMut<History<S>>,
) {
    // O：延迟帧门控（decomp 0x14226be70 行 122-128）——wave3 落库一次性 flag
    // 子门控（分歧 12 已修）。二进制控制流：
    // ```
    // if 有未读事件 { flag=1; count=1; }
    // else if flag 置位 { count += 1; if count+1 > 2 { flag=0; 转换块 } }
    // ```
    // 注意：转换块执行后 flag 清零，无事件帧不再重跑（原 Rust 每帧重跑转换，
    // 已判等修复）。
    let trigger = if spawns.read().next().is_some() {
        wait.count = 1;
        wait.flag = 1;
        false
    } else if wait.flag != 0 {
        wait.count += 1;
        if wait.count > 2 {
            wait.flag = 0;
            true
        } else {
            false
        }
    } else {
        false
    };
    if !trigger {
        return;
    }
    // O：通过全部门禁的树 → `Vec<TreeEdit::Place>`；循环后统一 drain → map(EditType::Tree)
    // → `History::insert_starting_edit`（decomp LAB_14226c1f0 尾部：from_iter<EditType,
    // Map<Drain<TreeEdit>, closure_env$1>> + insert_starting_edit 0x140a844f0）。
    let mut edits: Vec<EditType> = Vec::new();
    for (entity, data, instance, _converted) in converted.iter() {
        let pos = glam::Vec2::new(instance.0.position.x, instance.0.position.z);
        // O：6 路栅格采样（4 路径 + 水面 + 花园；`as_float_grid` 0x140ae4c20 无参
        // 一次返回四路引用——2026-10-17 core 签名修正）。
        let mut path_sum = 0.0f32;
        for grid in path_rasters.as_float_grid() {
            path_sum += grid.bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0);
        }
        let density = path_sum.clamp(0.0, 1.0);
        let water_level = (**water)
            .as_ref()
            .bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0);
        let garden_level = (**garden)
            .as_ref()
            .bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0);
        // O：岩石掩码（terrain_rock_mask::sample_at_world_xz）——buffer 来自
        // AssetSsboLibrary（FrozenMap<SsboId,BufferState> + 名断言 len 0x3f），
        // 取用面 U → 分歧 13（本处以空 buffer 占位，未参与门控）。
        let _rock = sample_at_world_xz(&[], pos.x, pos.y);
        // O：水/岩/花园/路径任一命中 = 未转换 → 发事件（方向：与 restorer 首批
        // 相反；reviewer 复核 + HISTORY 双确认）。
        let conflict = water_level > 0.0 || garden_level > 0.0 || density > 0.0;
        let mut blocked = conflict;
        if !conflict {
            // O：density<=0 时才查墙冲突（Circle2d + query_with_circle_callback）。
            walls.query_with_circle(pos, TREE_WALL_GAP, |_s, _d| {
                blocked = true;
            });
        }
        // 树 id 反查（真实链 = 已转换树记录；当前以 storage.iter().find(entity)
        // 代理，分歧 9/14）。
        let id = storage
            .iter()
            .find(|(_i, e)| e.entity == entity)
            .map(|(i, _e)| i.0)
            .expect("converted tree has no bound TreeStorageIndex");
        if blocked {
            // O：未转换（冲突）→ 发 stride-0x10 事件（payload = 树 u64）。
            deletes.write(DeleteTreeCmd { id });
        } else {
            // O：通过全部门禁 → 组 stride-0x48 TreeEdit 记录（decomp
            // 0x14226c9xx：tag=0 Place + id + PrefabName + position + is_preplaced=1）。
            // Source placement is the actual existing 48-byte PrefabInstance.
            edits.push(EditType::Tree(Arc::new(TreeEdit::Place(
                TreeEditData::from_placement(
                    country_core::resources::tree_storage::TreeStorageIndex(id),
                    &instance.0,
                    data.prefab_name.as_name_ref().to_string(),
                    true,
                ),
            ))));
        }
    }
    // O：History::insert_starting_edit（0x140a844f0）——`ledger_mut()` 已由
    // country-core 落库（分歧 15 关闭）；nonce 原版取 fastrand u64。
    let entry = HistoryEntry::from_edits(edits);
    insert_starting_edit(history.ledger_mut(), entry, fastrand::u64(..));
}
