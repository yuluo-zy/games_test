//! Original B14226be70: wait for prefab creation after the legacy request,
//! evaluate the actual TreeStorage owners, then insert one starting edit.
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    components::ConvertedPreplacedTree,
    resources::{
        history::{
            EditType, History,
            snapshots::{HistoryEntry, insert_starting_edit},
            tree_edit::{DeleteTreeCmd, SpawnPreplacedTrees, TreeEdit, TreeEditData},
        },
        terrain_rock_mask::TerrainRockMaskSsbo,
        tree_storage::TreeStorage,
        world_raster_defs::{GardenRaster, WaterRaster},
    },
    systems::{
        paths::PathRastersResources, prefab::PrefabInstanceData,
        wall::wall_state::wall_spatial_hash::WallSpatialHash,
    },
};
use std::sync::Arc;

pub const TREE_WALL_GAP: f32 = 7.0;
#[derive(Default)]
pub struct Wait {
    pub count: u64,
    pub flag: bool,
    // 异步 readback 可能跨越 Game 的退出／重入；延迟工作只能消费原会话。
    epoch: Option<(country_core::resources::session::SessionId, bevy_ecs::component::Tick)>,
}

#[allow(clippy::too_many_arguments)]
pub fn convert_preplaced_trees<S: Send + Sync + 'static>(
    mut spawns: EventReader<SpawnPreplacedTrees>,
    mut deletes: EventWriter<DeleteTreeCmd>,
    mut wait: Local<Wait>,
    path_rasters: PathRastersResources,
    water: Res<WaterRaster>,
    garden: Res<GardenRaster>,
    walls: Res<WallSpatialHash>,
    library: Res<AssetSsboLibrary>,
    storage: Res<TreeStorage>,
    converted: Query<(&PrefabInstanceData, &ConvertedPreplacedTree)>,
    mut history: ResMut<History<S>>,
    readbacks: Option<Res<country_core::render::gpu_readback::GpuReadbacks>>,
    mut spawned_after: Local<u64>,
    session: Res<country_core::resources::session::ActiveSession>,
    loading: Res<bevy_state::state::State<country_core::app_loading_state::AppLoadingState>>,
) {
    let readbacks = readbacks.as_deref();
    let epoch = (session.id, loading.last_changed());
    // Native reader advances to the end of both event buffers, coalescing all
    // requests. Taking just .next() would retrigger the delay for each event.
    if spawns.read().count() != 0 {
        wait.count = 1;
        wait.flag = true;
        wait.epoch = Some(epoch);
        *spawned_after = readbacks.map_or(0, |readbacks| readbacks.submitted_frame());
        if std::env::var_os("TG_M6_TRACE_PREPLACED").is_some() {
            eprintln!("M6_PREPLACED request cursor={} entries={} submitted={}", history.ledger().cursor, history.ledger().entries.len(), *spawned_after);
        }
        return;
    }
    if !wait.flag {
        return;
    }
    if wait.epoch != Some(epoch) {
        // 原版在第二帧直接消费映射；异步宿主仍在等旧回读时若已卸载，
        // 不能把旧请求写进新会话的 History。新请求仍按原路径独立处理。
        wait.flag = false;
        wait.epoch = None;
        return;
    }
    wait.count = wait.count.wrapping_add(1);
    if wait.count <= 2 {
        return;
    }
    library
        .get::<TerrainRockMaskSsbo>()
        .expect("TerrainRockMaskSsbo must exist and expose its mapped readback");
    // The two-frame delay is not a GPU fence: wait for a rock mask the render
    // thread published from a frame submitted after the spawn request.
    let Some(rock) = readbacks
        .and_then(|readbacks| readbacks.get::<TerrainRockMaskSsbo>())
        .filter(|view| view.frame > *spawned_after)
        .map(|view| view.data)
    else {
        return;
    };
    wait.flag = false;
    wait.epoch = None;
    if std::env::var_os("TG_M6_TRACE_PREPLACED").is_some() {
        eprintln!("M6_PREPLACED convert cursor={} entries={} waited={} submitted_after={} readback_frame={}", history.ledger().cursor, history.ledger().entries.len(), wait.count, *spawned_after, readbacks.and_then(|r| r.get::<TerrainRockMaskSsbo>()).map_or(0, |r| r.frame));
    }
    log::info!("Adding converted pre-placed trees to history");
    let mut edits = Vec::new();
    for (id, entry) in storage.iter() {
        // Missing/dead Entity or either missing component is a native skip,
        // not a reverse lookup from every unrelated prefab entity in World.
        let Ok((placement, original_name)) = converted.get(entry.entity) else {
            continue;
        };
        let placement = &placement.0;
        let pos = glam::Vec2::new(placement.position.x, placement.position.z);
        let [a, b, c, d] = path_rasters
            .as_float_grid()
            .map(|grid| grid.bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0));
        let sum = ((a + b) + c) + d;
        let water = (**water)
            .as_ref()
            .bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0);
        let garden = (**garden)
            .as_ref()
            .bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0);
        // Original evaluates rock only when water/garden did not block.
        let mut blocked = water > 0.0
            || garden > 0.0
            || TerrainRockMaskSsbo::sample_mapped_at_world_xz(rock, pos.x, pos.y) != 0;
        if !blocked {
            // MAXSS(0,sum), MINSS(1,sum), UCOMISS/JA: unordered sums
            // follow the nonpositive branch, then the actual wall cell test.
            let lower = if 0.0 > sum { 0.0 } else { sum };
            let density = if 1.0 < lower { 1.0 } else { lower };
            blocked = density > 0.0;
            if !blocked {
                blocked = walls.has_occupied_cells_in_circle(pos, TREE_WALL_GAP);
            }
        }
        if blocked {
            deletes.write(DeleteTreeCmd { id: id.0 });
        } else {
            edits.push(EditType::Tree(Arc::new(TreeEdit::Place(
                TreeEditData::from_placement(
                    *id,
                    placement,
                    original_name.0.as_name_ref().as_str().to_owned(),
                    true,
                ),
            ))));
        }
    }
    // Native inserts a MultiEdit even for an empty accepted batch; the shared
    // starting-edit owner changes cursor/identity and preserves all aux owners.
    insert_starting_edit(
        history.ledger_mut(),
        HistoryEntry::from_edits(edits),
        fastrand::u64(..),
    );
}
