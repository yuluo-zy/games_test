//! Prepared path -> original mask registry and separate material-stat inputs.
use crate::{PublicWalls, WallCollectionState, wall_paths::WallPreparedPaths};
use bevy_ecs::prelude::*;
use std::collections::HashMap;
use tiny_glade_reconstruction::{
    building_wall_platform_segmentation::{WallPlatformSegmentation, split_platform_wall_masks},
    building_wall_segmentation::{
        WallPathMaterialStatsUpdate, WallPathSegmentationMask, WallSegmentationInput,
        material_stats_for_update, wall_path_segmentation,
    },
    building_wall_state::WallId,
};
#[derive(Clone, Debug)]
pub struct WallPathMasks {
    pub masks: Vec<WallPathSegmentationMask>,
    pub publication_revision: u64,
    pub prepared_generation: u64,
    pub generation: u64,
}
#[derive(Resource, Default)]
pub struct WallPathMaskRegistry {
    pub records: HashMap<WallId, WallPathMasks>,
    pub rebuilt: Vec<WallId>,
    pub pending: Vec<WallId>,
}
#[derive(Resource, Default)]
pub struct WallPathMaterialStats(pub HashMap<WallId, WallPathMaterialStatsUpdate>);
#[derive(Clone, Debug)]
pub struct WallProcessedMasks {
    pub output: WallPlatformSegmentation,
    pub generation: u64,
    pub raw_mask_generation: u64,
    pub prepared_generation: u64,
    fingerprint: Vec<u8>,
}
#[derive(Resource, Default)]
pub struct WallProcessedMaskRegistry {
    pub records: HashMap<WallId, WallProcessedMasks>,
    pub pending: Vec<WallId>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<WallPathMaskRegistry>()
        .init_resource::<WallPathMaterialStats>()
        .init_resource::<WallProcessedMaskRegistry>()
        .add_systems(
            bevy_app::Update,
            process_platforms.in_set(crate::BuildingStage::PlatformSegmentation),
        )
        .add_systems(
            bevy_app::Update,
            rebuild.in_set(crate::BuildingStage::Segmentation),
        );
}
fn process_platforms(
    paths: Res<WallPreparedPaths>,
    raw: Res<WallPathMaskRegistry>,
    walls: Res<PublicWalls>,
    map: Res<crate::WallRoofMap>,
    roofs: Query<&crate::Roof>,
    mut processed: ResMut<WallProcessedMaskRegistry>,
    mut report: ResMut<crate::BuildingReport>,
) {
    processed.pending.clear();
    processed
        .records
        .retain(|wall, _| raw.records.contains_key(wall) && walls.walls.contains_key(wall));
    let mut ids: Vec<_> = raw.records.keys().copied().collect();
    ids.sort_unstable_by_key(|wall| wall.0);
    for wall in ids {
        let masks = &raw.records[&wall];
        let Some(host) = walls.walls.get(&wall) else {
            continue;
        };
        let Some(path) = paths.current(wall, host.publication_revision) else {
            processed.records.remove(&wall);
            processed.pending.push(wall);
            continue;
        };
        let roof = map.entity(wall).and_then(|entity| roofs.get(entity).ok());
        let mut fingerprint = host.projection.to_vec();
        fingerprint.extend(masks.generation.to_le_bytes());
        fingerprint.extend(path.generation.to_le_bytes());
        fingerprint.push(u8::from(roof.is_some()));
        if let Some(roof) = roof {
            fingerprint.extend(roof.0);
        }
        if processed
            .records
            .get(&wall)
            .is_some_and(|previous| previous.fingerprint == fingerprint)
        {
            continue;
        }
        match split_platform_wall_masks(
            &host.projection,
            &path.path.positions,
            &path.path.segment_values,
            &masks.masks,
            roof.map(|roof| &roof.0),
        ) {
            Ok(output) => {
                let generation = processed
                    .records
                    .get(&wall)
                    .map_or(1, |previous| previous.generation.wrapping_add(1));
                processed.records.insert(
                    wall,
                    WallProcessedMasks {
                        output,
                        generation,
                        raw_mask_generation: masks.generation,
                        prepared_generation: path.generation,
                        fingerprint,
                    },
                );
            }
            Err(error) => {
                processed.records.remove(&wall);
                processed.pending.push(wall);
                report
                    .faults
                    .push(format!("Platform wall {}: {error:?}", wall.0));
            }
        }
    }
}
fn rebuild(
    paths: Res<WallPreparedPaths>,
    walls: Res<PublicWalls>,
    source: Res<WallCollectionState>,
    mut registry: ResMut<WallPathMaskRegistry>,
    mut stats: ResMut<WallPathMaterialStats>,
    mut report: ResMut<crate::BuildingReport>,
) {
    registry.rebuilt.clear();
    registry.pending.clear();
    registry
        .records
        .retain(|wall, _| walls.walls.contains_key(wall) && source.raw_sources.contains_key(wall));
    stats
        .0
        .retain(|wall, _| walls.walls.contains_key(wall) && source.raw_sources.contains_key(wall));
    let mut ids: Vec<_> = source.raw_sources.keys().copied().collect();
    ids.sort_unstable_by_key(|wall| wall.0);
    for wall in ids {
        let Some(host) = walls.walls.get(&wall) else {
            continue;
        };
        let Some(path) = paths.current(wall, host.publication_revision) else {
            registry.records.remove(&wall);
            stats.0.remove(&wall);
            registry.pending.push(wall);
            continue;
        };
        if registry.records.get(&wall).is_some_and(|masks| {
            masks.publication_revision == host.publication_revision
                && masks.prepared_generation == path.generation
        }) {
            continue;
        }
        let raw = &source.raw_sources[&wall];
        let masks = match wall_path_segmentation(WallSegmentationInput {
            prepared_positions: &path.path.positions,
            path_values: &path.path.segment_values,
            shape: &raw.shape,
            observed_special_flag: u8::from(raw.special),
        }) {
            Ok(masks) => masks,
            Err(error) => {
                registry.records.remove(&wall);
                stats.0.remove(&wall);
                registry.pending.push(wall);
                report
                    .faults
                    .push(format!("Wall segmentation {}: {error:?}", wall.0));
                continue;
            }
        };
        stats.0.insert(
            wall,
            material_stats_for_update(path.path.length, u8::from(raw.special), masks.len()),
        );
        let generation = registry
            .records
            .get(&wall)
            .map_or(1, |old| old.generation.wrapping_add(1));
        registry.records.insert(
            wall,
            WallPathMasks {
                masks,
                publication_revision: host.publication_revision,
                prepared_generation: path.generation,
                generation,
            },
        );
        registry.rebuilt.push(wall);
    }
}
