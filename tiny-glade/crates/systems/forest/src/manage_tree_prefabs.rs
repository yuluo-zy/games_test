//! manage_tree_prefabs.rs — 森林树管理主系统（B 0x141bf16c0，O；re-analyst-A
//! 契约 `systems-recovery/groupA/forest/RE-ANALYSIS.md` §2）。
//!
//! 每帧两阶段（O）：
//! - **阶段 1（候选评估）**：对每个 0x50B 树候选记录：位置（`PlacementDataSsbo`
//!   position 0x140ad51a0）→ 越界 `|x|≥65 || |y|≥65`（0x142a83814，掩 0x14292e980）
//!   → 标记；否则采样 **6 栅格**（0x140a14df0，scale 130.0）：路径栅格 ×4
//!   （`PathRastersResources::as_float_grid` 0x140ae4c20）+ 2 附加栅格（Water/
//!   Garden，I 身份）；附加栅格 >0（路上/水上）→ 淘汰；否则密度 =
//!   `clamp(Σ4, 0, 1)`（0x142925904=1.0）；`is_too_close_to_shape`
//!   （`dist ≤ shape_radius + 7.0`，0x142a7cd48）→ 淘汰；通过 →
//!   `Circle2d::new(center, r=7.0)` + `WallSpatialHash::query_with_circle`
//!   （0x1422785d0 单态，与墙共享空间哈希）→ 无墙冲突才推候选 {tree_id u64,
//!   flag u8}（0x10B 条目）到 `MemCache`。
//! - **阶段 2（执行）**：遍历候选：
//!   - flag==1（新树）：`fastrand::f32`（0x140caf380）→ 缩放 `lerp(0.3, 0.75)`
//!     （0.75=0x1429edb90，0.3=0x1429258d8）；高度抖动 `+ fastrand*10.0`
//!     （0x142a95db0 高字=10.0）；地形高过滤 `y ∈ [-65, 65)`；`pack_2x16f_uint`
//!     （0x140c90df0）；push 0x50B 树记录（{Arc<String> prefab, u64 打包 pos,
//!     f32 高, f32 缩放, …}）到树库写侧（`InteractableForestPlacements`）；
//!     发 `SpawnPrefabCmd`。
//!   - flag==0（既有树）：id → 表查 → 存在位集校验 → 缺失则发
//!     `DespawnPrefabCmd`（0x142278920/0x142278cf0 bundle 族）。
//!
//! 常量：65.0（界）/ 7.0（树-形间距）/ 0.3..0.75（缩放）/ 10.0（高抖）/
//! 130.0（栅格尺度）。候选 flag 的精确编码（越界/栅格/形状分支的 0/1 语义）
//! 标注 I（re-analyst §2 两处读数有出入，见 RESTORE.md）。

use bevy_ecs::prelude::*;
use glam::Vec2;
use utils::geometry::Circle2d;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::components::TreeMarkedForDeletion;
use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::terrain_height::{TerrainHeightsData, TerrainHeightSsbo};
use country_core::resources::time::GameTime;
use country_core::resources::tree_storage::InteractableForestPlacements;
use country_core::resources::world_raster_defs::{GardenRaster, WaterRaster};
use country_core::systems::paths::PathRastersResources;
use country_core::systems::prefab::SpawnPrefabCmd;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

/// 世界边界（O：0x142a83814 = 65.0）。
pub const WORLD_BOUND: f32 = 65.0;
/// 树-形状最小间距（O：0x142a7cd48 = 7.0）。
pub const TREE_SHAPE_GAP: f32 = 7.0;
/// 新树缩放下限/上限（O：0x1429258d8 = 0.3 / 0x1429edb90 = 0.75）。
pub const TREE_SCALE_MIN: f32 = 0.3;
pub const TREE_SCALE_MAX: f32 = 0.75;
/// 高度抖动幅度（O：0x142a95db0 高字 = 10.0）。
pub const HEIGHT_JITTER: f32 = 10.0;
/// 栅格采样尺度（O：0x142a7b658 = 130.0）。
pub const RASTER_SCALE: f32 = 130.0;
/// 密度钳制上界（O：0x142925904 = 1.0）。
pub const DENSITY_MAX: f32 = 1.0;

/// 候选/执行缓冲（O：行 33 `impl$0::default`；0x10B 候选条目 {tree_id, flag} +
/// 帧内完成标记）。
#[derive(Default)]
pub struct MemCache {
    /// 候选条目（tree_id u64 + flag u8；语义 I）。
    pub candidates: Vec<(u64, u8, glam::Vec2)>,
    /// 帧内完成标记（O：末尾写 1）。
    pub finished: u8,
}

/// 树-形状间距判定（行 211，内联；O：`dist ≤ shape_radius + 7.0`）。
/// 形状 = Circle2d/Rectangle2d（re-analyst §3 0x1409ea800 辅助分派，细部 U——
/// 本实现以圆距离落地，矩形分派登记待补证）。
pub fn is_too_close_to_shape(circle: Circle2d, tree_pos: Vec2) -> bool {
    let delta = tree_pos - circle.center;
    delta.length() <= circle.radius + TREE_SHAPE_GAP
}

/// `manage_tree_prefabs` @0x141bf16c0（O；14 参面按 BASELINE）。
///
/// 6 栅格采样、密度累计与 0x50B 树记录字段明细 U（re-analyst §6）；本实现落地
/// O 常量规则（界/间距/缩放/墙冲突），附加栅格（Water/Garden）与形状表判定
/// 登记待补证。
#[allow(clippy::too_many_arguments)]
pub fn manage_tree_prefabs<C: Send + Sync + 'static>(
    _signifiers: Res<UiSignifierStream>,
    _library: Res<AssetSsboLibrary>,
    mut placements: ResMut<InteractableForestPlacements>,
    _commands: Commands,
    mut spawns: EventWriter<SpawnPrefabCmd>,
    _marked: Query<&TreeMarkedForDeletion>,
    _constructs: EventReader<ConstructTimberframeCmd>,
    walls: Res<WallSpatialHash>,
    _time: Res<GameTime>,
    _heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    path_rasters: PathRastersResources,
    water: Res<WaterRaster>,
    garden: Res<GardenRaster>,
    mut cache: Local<MemCache>,
) {
    cache.candidates.clear();

    // 阶段 1（O 结构）：候选评估。0x50B 树记录字段明细 U——以
    // InteractableForestTree（{id, position, scale} 语义）承载。
    for record in placements.records.iter() {
        let pos = Vec2::new(record.position.x, record.position.z);
        // O：越界（|x|≥65 || |y|≥65）→ 跳过（精确 flag 编码 I，见模块头）。
        if pos.x.abs() >= WORLD_BOUND || pos.y.abs() >= WORLD_BOUND {
            continue;
        }
        // O（RE-ANALYSIS-2）：6 路栅格采样——4 路地面栅格（路径 ×4，
        // as_float_grid 0x140ae4c20）+ 排除面（Water/Garden，I 身份按槽序）：
        //   - 排除面任一 > 0 → 跳过（树不落水面/花园）；
        //   - 密度 = clamp(Σ4, 0, 1)；密度 > 0 || is_too_close_to_shape → 跳过。
        let mut path_sum = 0.0f32;
        for slot in 0..4 {
            path_sum += path_rasters
                .as_float_grid(slot)
                .bilinear_sample_at_world_xz(pos.x, pos.y, RASTER_SCALE, RASTER_SCALE);
        }
        let density = path_sum.clamp(0.0, DENSITY_MAX);
        let water_level = (**water)
            .as_ref()
            .bilinear_sample_at_world_xz(pos.x, pos.y, RASTER_SCALE, RASTER_SCALE);
        let garden_level = (**garden)
            .as_ref()
            .bilinear_sample_at_world_xz(pos.x, pos.y, RASTER_SCALE, RASTER_SCALE);
        if water_level > 0.0 || garden_level > 0.0 {
            continue;
        }
        if density > 0.0 {
            continue;
        }
        // O：墙冲突（Circle2d(center, 7.0) + query_with_circle 单态族）。
        let mut blocked_by_wall = false;
        walls.query_with_circle(pos, TREE_SHAPE_GAP, |_segment, _distance| {
            blocked_by_wall = true;
        });
        if blocked_by_wall {
            continue;
        }
        // O：通过 → 推候选 {tree_id, flag=1, 位置}（0x10B 条目 + 位置面）。
        cache.candidates.push((record.id, 1, pos));
    }

    // 阶段 2（O 结构）：执行。
    for (tree_id, flag, pos) in cache.candidates.iter().copied() {
        if flag != 0 {
            // 新树（O）：fastrand 缩放 lerp(0.3, 0.75) + 高度抖动 fastrand*10.0；
            // 0x50B 树记录 push + SpawnPrefabCmd（记录字段 U）。
            let _scale = fastrand::f32() * (TREE_SCALE_MAX - TREE_SCALE_MIN) + TREE_SCALE_MIN;
            let _jitter = fastrand::f32() * HEIGHT_JITTER;
            // 修复（reviewer FIX 3.2）：新树记录真实位置（候选评估的格点位置面；
            // 0x50B 记录字段剩余 U）。
            placements.records.push(country_core::resources::tree_storage::InteractableForestTree {
                id: tree_id,
                position: glam::Vec3::new(pos.x, 0.0, pos.y),
                scale: _scale,
                _gap: 0,
            });
            spawns.write(SpawnPrefabCmd { _gap: tree_id });
        } else {
            // 既有树（O：id → 表查 → 缺失则 despawn——bundle 族
            // FUN_142278920/FUN_142278cf0 经 Commands；实体映射 = TreeStorage
            // 存在位集族，re-analyst §2/§5，本处无 TreeStorage 参数面——以
            // placements 记录修剪落地等价簿记，实体 despawn 登记宿主接线）。
            placements.records.retain(|r| r.id != tree_id);
        }
    }
    cache.finished = 1;
}
