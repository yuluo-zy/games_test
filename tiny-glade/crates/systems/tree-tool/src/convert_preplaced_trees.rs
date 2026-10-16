//! convert_preplaced_trees.rs — 预置树→游戏树转换（B 0x14226be70，O；
//! re-analyst-A 契约 §2）。
//!
//! 语义（O）：`SpawnPreplacedTrees` 事件 → 预置树数据（PlacementDataSsbo 族）
//! + 6 栅格采样（0x140a14df0，同 forest）：密度 = clamp(Σ4, 0, 1) ≤ 0 且
//! 附加栅格为 0 且非形状近（0x1409ea800）→ `WallSpatialHash::query_with_circle`
//! （0x1425defd0 单态，r=7.0）无墙 → 转 0x48B 树记录（`DeleteTreeCmd` 待删除 +
//! 转换），History 链路（0x140a84ec0）。

use bevy_ecs::prelude::*;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::components::ConvertedPreplacedTree;
use country_core::resources::history::tree_edit::{DeleteTreeCmd, SpawnPreplacedTrees};
use country_core::resources::history::History;
use country_core::resources::tree_storage::TreeStorage;
use country_core::resources::world_raster_defs::{GardenRaster, WaterRaster};
use country_core::systems::paths::PathRastersResources;
use country_core::systems::prefab::PrefabInstanceData;
use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

/// 树-墙间距（O：r=7.0 圈，同 forest TREE_SHAPE_GAP）。
pub const TREE_WALL_GAP: f32 = 7.0;

/// 转换等待缓冲（O：`Local<Wait>`——转换分帧处理，语义 I）。
#[derive(Default)]
pub struct Wait {
    /// typed gap。
    pub _gap: u64,
}

/// `convert_preplaced_trees` @0x14226be70（O 面）。
#[allow(clippy::too_many_arguments)]
pub fn convert_preplaced_trees<S: Send + Sync + 'static>(
    mut spawns: EventReader<SpawnPreplacedTrees>,
    mut deletes: EventWriter<DeleteTreeCmd>,
    _wait: Local<Wait>,
    path_rasters: PathRastersResources,
    water: Res<WaterRaster>,
    garden: Res<GardenRaster>,
    walls: Res<WallSpatialHash>,
    _library: Res<AssetSsboLibrary>,
    storage: Res<TreeStorage>,
    converted: Query<(Entity, &PrefabInstanceData, &ConvertedPreplacedTree)>,
    _history: ResMut<History<S>>,
) {
    for _cmd in spawns.read() {
        // O 面：候选评估（6 栅格密度 ≤ 0 + 附加栅格 0 + 非形状近 + 无墙）。
        // 墙冲突查询在**候选树位置**（O：0x1425defd0 单态族 r=7.0；
        // 候选位置 = 已转换树实体面——PlacementDataSsbo::position 0x140ad51a0
        // 与 0x48B 转换记录字段明细 U，re-analyst §6）。
        for (entity, data, _converted) in converted.iter() {
            let pos = glam::Vec2::new(data.position.x, data.position.z);
            // O（RE-ANALYSIS-2）：6 路栅格密度评估（同 forest）——排除面 >0
            // 或密度 >0 → 不转换。
            let mut path_sum = 0.0f32;
            for slot in 0..4 {
                path_sum += path_rasters
                    .as_float_grid(slot)
                    .bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0);
            }
            let density = path_sum.clamp(0.0, 1.0);
            let water_level = (**water)
                .as_ref()
                .bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0);
            let garden_level = (**garden)
                .as_ref()
                .bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0);
            if water_level > 0.0 || garden_level > 0.0 || density > 0.0 {
                continue;
            }
            let mut blocked = false;
            walls.query_with_circle(pos, TREE_WALL_GAP, |_s, _d| {
                blocked = true;
            });
            if !blocked {
                // O：转换 → 删除预置树（真实 id = TreeStorage 按实体反查索引；
                // 预置树记录字段 U，re-analyst §6）。
                let id = storage
                    .iter()
                    .find(|(_i, e)| e.entity == entity)
                    .map(|(i, _e)| i.0)
                    .unwrap_or(0);
                deletes.write(DeleteTreeCmd { id });
            }
        }
    }
}
