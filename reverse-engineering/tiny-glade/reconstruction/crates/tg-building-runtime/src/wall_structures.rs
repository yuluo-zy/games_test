//! Real special-wall masks -> complete lintel instances and public third cache.
use crate::{
    BuildingStage, PublicWalls, WallCollectionState, wall_paths::WallPreparedPaths,
    wall_segments::WallProcessedMaskRegistry,
};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred};
use std::collections::{HashMap, HashSet};
use tiny_glade_reconstruction::{
    building_wall_state::WallId,
    building_wall_structures::{
        WallSegmentSource, WallWoodenPlankInstance64B, construct_lintel_supports,
    },
};
#[derive(Component, Clone, Debug)]
pub struct WallCpuLintels {
    pub wall: WallId,
    pub generation: u64,
    pub instances: Vec<WallWoodenPlankInstance64B>,
}
#[derive(Resource, Default)]
pub struct WallLintelEntities(pub HashMap<WallId, Entity>);
#[derive(Resource, Default)]
pub struct WallLintelBuildState {
    fingerprints: HashMap<WallId, Vec<u8>>,
    pub pending: Vec<WallId>,
    pub rebuilt: Vec<WallId>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<WallLintelEntities>()
        .init_resource::<WallLintelBuildState>()
        .add_systems(
            bevy_app::Update,
            (rebuild, ApplyDeferred)
                .chain()
                .in_set(BuildingStage::WallStructures),
        );
}
fn rebuild(
    mut commands: Commands,
    mut walls: ResMut<PublicWalls>,
    source: Res<WallCollectionState>,
    paths: Res<WallPreparedPaths>,
    masks: Res<WallProcessedMaskRegistry>,
    mut entities: ResMut<WallLintelEntities>,
    mut state: ResMut<WallLintelBuildState>,
    previous: Query<&WallCpuLintels>,
    mut report: ResMut<crate::BuildingReport>,
) {
    state.pending.clear();
    state.rebuilt.clear();
    let active: HashSet<_> = source
        .raw_sources
        .keys()
        .copied()
        .filter(|wall| {
            walls
                .walls
                .get(wall)
                .is_some_and(|host| host.projection[0x296] != 0)
        })
        .collect();
    let stale: Vec<_> = entities
        .0
        .keys()
        .copied()
        .filter(|wall| !active.contains(wall))
        .collect();
    for wall in stale {
        if let Some(entity) = entities.0.remove(&wall) {
            commands.entity(entity).despawn();
        }
        state.fingerprints.remove(&wall);
        if let Some(host) = walls.walls.get_mut(&wall) {
            host.segment_heights = host.base_segment_heights.clone();
        }
    }
    let mut ids: Vec<_> = active.into_iter().collect();
    ids.sort_unstable_by_key(|wall| wall.0);
    for wall in ids {
        let host = walls.walls.get_mut(&wall).unwrap();
        let input = paths
            .current(wall, host.publication_revision)
            .zip(masks.records.get(&wall));
        let Some((path, masks)) = input.filter(|(path, masks)| {
            !host.gable_profile_pending && masks.prepared_generation == path.generation
        }) else {
            if let Some(entity) = entities.0.remove(&wall) {
                commands.entity(entity).despawn();
            }
            state.fingerprints.remove(&wall);
            host.segment_heights = host.base_segment_heights.clone();
            state.pending.push(wall);
            continue;
        };
        let mut fingerprint = host.projection.to_vec();
        fingerprint.extend(host.publication_revision.to_le_bytes());
        fingerprint.extend(path.generation.to_le_bytes());
        fingerprint.extend(masks.generation.to_le_bytes());
        for value in host
            .arch_segment_heights
            .iter()
            .chain(&host.base_segment_heights)
        {
            fingerprint.extend(value.to_le_bytes());
        }
        if state.fingerprints.get(&wall) == Some(&fingerprint) {
            continue;
        }
        // Initialize once per source rebuild, then accumulate across all masks.
        // Other original writers (arches) remain a separate recovery milestone.
        let mut third = host.base_segment_heights.clone();
        let mut instances = Vec::new();
        let mut failed = None;
        let p284 = f32::from_le_bytes(host.projection[0x284..0x288].try_into().unwrap());
        let p288 = f32::from_le_bytes(host.projection[0x288..0x28c].try_into().unwrap());
        for mask in &masks.output.processed_masks {
            match construct_lintel_supports(
                WallSegmentSource {
                    mask,
                    segment_values: &path.path.segment_values,
                    arch_heights: &host.arch_segment_heights,
                    prepared_positions: &path.path.positions,
                    prepared_u: &path.path.points_u,
                    regular: host.source.shape.tag() < 2,
                    closed: host.source.shape.tag() != 2,
                },
                p284,
                p288,
                wall.0,
                &mut third,
            ) {
                Ok(records) => instances.extend(records),
                Err(error) => {
                    failed = Some(error);
                    break;
                }
            }
        }
        if let Some(error) = failed {
            if let Some(entity) = entities.0.remove(&wall) {
                commands.entity(entity).despawn();
            }
            state.fingerprints.remove(&wall);
            host.segment_heights = host.base_segment_heights.clone();
            state.pending.push(wall);
            report
                .faults
                .push(format!("Wall lintel {}: {error:?}", wall.0));
            continue;
        }
        host.segment_heights = third;
        let generation = entities
            .0
            .get(&wall)
            .and_then(|entity| previous.get(*entity).ok())
            .map_or(1, |old| old.generation.wrapping_add(1));
        let component = WallCpuLintels {
            wall,
            generation,
            instances,
        };
        if let Some(entity) = entities.0.get(&wall) {
            commands.entity(*entity).insert(component);
        } else {
            let entity = commands.spawn(component).id();
            entities.0.insert(wall, entity);
        }
        state.fingerprints.insert(wall, fingerprint);
        state.rebuilt.push(wall);
    }
}
