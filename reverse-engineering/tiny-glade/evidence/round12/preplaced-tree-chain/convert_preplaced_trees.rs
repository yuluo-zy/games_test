//! Original B14226be70: wait for prefab creation after the legacy request,
//! evaluate the actual TreeStorage owners, then insert one starting edit.
use bevy_ecs::prelude::*;
use std::sync::Arc;
use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
    components::ConvertedPreplacedTree,
    resources::{history::{EditType, History, snapshots::{HistoryEntry, insert_starting_edit},
        tree_edit::{DeleteTreeCmd, SpawnPreplacedTrees, TreeEdit, TreeEditData}},
        terrain_rock_mask::TerrainRockMaskSsbo, tree_storage::TreeStorage,
        world_raster_defs::{GardenRaster, WaterRaster}},
    systems::{paths::PathRastersResources, prefab::PrefabInstanceData,
        wall::wall_state::wall_spatial_hash::WallSpatialHash}};

pub const TREE_WALL_GAP: f32 = 7.0;
#[derive(Default)]
pub struct Wait { pub count: u64, pub flag: bool }

#[allow(clippy::too_many_arguments)]
pub fn convert_preplaced_trees<S: Send + Sync + 'static>(
    mut spawns: EventReader<SpawnPreplacedTrees>, mut deletes: EventWriter<DeleteTreeCmd>,
    mut wait: Local<Wait>, path_rasters: PathRastersResources,
    water: Res<WaterRaster>, garden: Res<GardenRaster>, walls: Res<WallSpatialHash>,
    library: Res<AssetSsboLibrary>, storage: Res<TreeStorage>,
    converted: Query<(&PrefabInstanceData, &ConvertedPreplacedTree)>, mut history: ResMut<History<S>>,
) {
    // Native reader advances to the end of both event buffers, coalescing all
    // requests. Taking just .next() would retrigger the delay for each event.
    if spawns.read().count() != 0 { wait.count = 1; wait.flag = true; return; }
    if !wait.flag { return; }
    wait.count = wait.count.wrapping_add(1);
    if wait.count <= 2 { return; }
    wait.flag = false;
    log::info!("Adding converted pre-placed trees to history");
    let rock = library.get::<TerrainRockMaskSsbo>()
        .expect("TerrainRockMaskSsbo must exist and expose its mapped readback");
    let mut edits = Vec::new();
    for (id, entry) in storage.iter() {
        // Missing/dead Entity or either missing component is a native skip,
        // not a reverse lookup from every unrelated prefab entity in World.
        let Ok((placement, original_name)) = converted.get(entry.entity) else { continue; };
        let placement = &placement.0;
        let pos = glam::Vec2::new(placement.position.x, placement.position.z);
        let [a, b, c, d] = path_rasters.as_float_grid().map(|grid|
            grid.bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0));
        let sum = ((a + b) + c) + d;
        let water = (**water).as_ref().bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0);
        let garden = (**garden).as_ref().bilinear_sample_at_world_xz(pos.x, pos.y, 130.0, 130.0);
        // Original evaluates rock only when water/garden did not block.
        let mut blocked = water > 0.0 || garden > 0.0
            || TerrainRockMaskSsbo::sample_mapped_at_world_xz(&rock.mapped[..], pos.x, pos.y) != 0;
        if !blocked {
            // MAXSS(0,sum), MINSS(1,sum), UCOMISS/JA: unordered sums
            // follow the nonpositive branch, then the actual wall cell test.
            let lower = if 0.0 > sum { 0.0 } else { sum };
            let density = if 1.0 < lower { 1.0 } else { lower };
            blocked = density > 0.0;
            if !blocked { blocked = walls.has_occupied_cells_in_circle(pos, TREE_WALL_GAP); }
        }
        if blocked { deletes.write(DeleteTreeCmd { id: id.0 }); }
        else {
            edits.push(EditType::Tree(Arc::new(TreeEdit::Place(TreeEditData::from_placement(
                *id, placement, original_name.0.as_name_ref().to_string(), true,
            )))));
        }
    }
    // Native inserts a MultiEdit even for an empty accepted batch; the shared
    // starting-edit owner changes cursor/identity and preserves all aux owners.
    insert_starting_edit(history.ledger_mut(), HistoryEntry::from_edits(edits), fastrand::u64(..));
}
