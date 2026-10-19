//! B140751960: raw136B masks -> processed136B masks + native32B step ranges.
//! Original set22 runs this after relationship/invalid-roof updates and before
//! arches/lintels, using chain_ignore_deferred (no inserted Commands barrier).
use crate::{
    construct_arches::WallPathSegmentationMasks,
    construct_lintels::WallPathSegmentationMasksMinusStairs,
    construct_platform_stairs::PlatformStairsStorage,
};
use bevy_ecs::prelude::*;
use country_core::resources::{
    roofs::{
        Roof,
        commands::{CreateRoofCmd, SwitchRoofTypeCmd},
    },
    walls::{
        WallId,
        public_walls::{OnWallChanged, PublicWalls},
    },
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::wall::construction::platform_segmentation::split_platform_wall_masks;

pub fn split_wall_path_segment_into_platform_stairs(
    mut changed: EventReader<OnWallChanged>,
    mut create_roof: EventReader<CreateRoofCmd>,
    mut switch_type: EventReader<SwitchRoofTypeCmd>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut raw: ResMut<WallPathSegmentationMasks>,
    mut processed: ResMut<WallPathSegmentationMasksMinusStairs>,
    mut steps: ResMut<PlatformStairsStorage>,
    _signifiers: Res<UiSignifierStream>,
) {
    let mut recalc: Vec<u64> = changed
        .read()
        .map(|e| e.0.0)
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .chain(switch_type.read().map(|e| e.wall.0))
        .collect();
    // Native751c..751d performs unsigned sort followed by adjacent dedup.
    recalc.sort_unstable();
    recalc.dedup();
    for wall_id in recalc {
        let masks = raw
            .masks
            .get_mut(&wall_id)
            .expect("platform segmentation requires raw masks for every event wall");
        let id = WallId(wall_id);
        let wall = walls
            .get(id)
            .expect("platform segmentation event must reference a published wall");
        let ranges = steps.segments.entry(wall_id).or_default();
        ranges.clear(); // Native clears even when the non-flat/shape>=2 clone branch follows.
        let roof = walls.roof_component(id, |entity| roofs.get(entity).ok());
        let projection = wall.building_projection();
        let path = wall
            .prepared_path
            .as_ref()
            .expect("platform segmentation requires prepared wall path");
        let result = split_platform_wall_masks(
            projection.observed_bytes(),
            &path.positions,
            &path.segment_values,
            masks,
            roof.map(Roof::observed),
        )
        .expect("invalid authoritative platform segmentation input");
        processed.masks.insert(wall_id, result.processed_masks);
        *ranges = result.steps;
    }
}
