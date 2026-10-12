//! Actual wall-scoped decorator rows -> stair segments, collider cells and holes.
//! Render rows are a separate in-progress native consumer, never a fabricated mesh.
use crate::{
    BuildingStage, PublicWalls, WallCollectionState, wall_paths::WallPreparedPaths,
    wall_segments::WallProcessedMaskRegistry,
};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred};
use std::collections::HashMap;
use tiny_glade_reconstruction::{
    building_wall_holes::RawWallHole, building_wall_platform_stairs::*, building_wall_state::WallId,
};
#[derive(Event, Clone)]
pub struct SetPlatformDecoratorRows {
    pub wall: WallId,
    pub rows: Vec<WallPlatformDecoratorObserved>,
}
#[derive(Resource, Default)]
pub struct PlatformDecoratorRows {
    pub rows: HashMap<WallId, Vec<WallPlatformDecoratorObserved>>,
    revision: HashMap<WallId, u64>,
}
#[derive(Component, Clone, Debug)]
pub struct WallCpuPlatformStairs {
    pub wall: WallId,
    pub generation: u64,
    pub geometry: WallPlatformStairGeometry,
    pub holes: Vec<RawWallHole>,
    pub render_records: Vec<[u8; 96]>,
}
#[derive(Resource, Default)]
pub struct WallPlatformStairEntities(pub HashMap<WallId, Entity>);
#[derive(Resource, Default)]
pub struct WallPlatformStairState {
    pub holes: HashMap<WallId, Vec<RawWallHole>>,
    pub stairs_lengths: HashMap<WallId, f32>,
    pub pending: Vec<WallId>,
    pub rebuilt: Vec<WallId>,
    fingerprints: HashMap<WallId, Vec<u8>>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<PlatformDecoratorRows>()
        .init_resource::<WallPlatformStairEntities>()
        .init_resource::<WallPlatformStairState>()
        .add_event::<SetPlatformDecoratorRows>()
        .add_systems(
            bevy_app::Update,
            (consume, rebuild, finalize_stats, ApplyDeferred)
                .chain()
                .in_set(BuildingStage::PlatformStairs),
        );
}
fn consume(
    mut events: EventReader<SetPlatformDecoratorRows>,
    mut removes: EventReader<crate::RemoveWall>,
    mut rows: ResMut<PlatformDecoratorRows>,
) {
    for event in events.read() {
        rows.rows.insert(event.wall, event.rows.clone());
        *rows.revision.entry(event.wall).or_default() += 1;
    }
    for event in removes.read() {
        rows.rows.remove(&event.0);
        rows.revision.remove(&event.0);
    }
}
fn rebuild(
    mut commands: Commands,
    walls: Res<PublicWalls>,
    collection: Res<WallCollectionState>,
    paths: Res<WallPreparedPaths>,
    masks: Res<WallProcessedMaskRegistry>,
    rows: Res<PlatformDecoratorRows>,
    mut entities: ResMut<WallPlatformStairEntities>,
    mut state: ResMut<WallPlatformStairState>,
    previous: Query<&WallCpuPlatformStairs>,
    mut report: ResMut<crate::BuildingReport>,
) {
    state.pending.clear();
    state.rebuilt.clear();
    let mut ready = HashMap::new();
    let mut ids: Vec<_> = collection.raw_sources.keys().copied().collect();
    ids.sort_unstable_by_key(|w| w.0);
    for wall in ids {
        let Some(host) = walls.walls.get(&wall) else {
            continue;
        };
        let Some((path, processed)) = paths
            .current(wall, host.publication_revision)
            .zip(masks.records.get(&wall))
        else {
            continue;
        };
        if processed.output.steps.is_empty() {
            continue;
        }
        let Some((terrain, decorators)) = collection.terrain.as_ref().zip(rows.rows.get(&wall))
        else {
            state.pending.push(wall);
            continue;
        };
        if host.gable_profile_pending || processed.prepared_generation != path.generation {
            state.pending.push(wall);
            continue;
        }
        let mut key = host.projection.to_vec();
        key.extend(host.publication_revision.to_le_bytes());
        key.extend(path.generation.to_le_bytes());
        key.extend(processed.generation.to_le_bytes());
        key.extend(rows.revision[&wall].to_le_bytes());
        for h in &host.structural_arch_segment_heights {
            key.extend(h.to_le_bytes());
        }
        key.extend(terrain.minimum_y.to_le_bytes());
        key.extend(terrain.maximum_y.to_le_bytes());
        key.push(terrain.sampling_fix as u8);
        key.extend((terrain.encoded.as_ptr() as usize).to_le_bytes());
        if state.fingerprints.get(&wall) == Some(&key) {
            ready.insert(wall, ());
            continue;
        }
        let result = (|| {
            let blockers = collect_platform_stair_blockers(
                &collection.raw_sources[&wall].curve,
                terrain,
                decorators,
            )?;
            let segments = prepare_platform_stair_segments(WallPlatformStairSource {
                raw_curve: &collection.raw_sources[&wall].curve,
                public: &host.projection,
                prepared_positions: &path.path.positions,
                prepared_u: &path.path.points_u,
                segment_values: &path.path.segment_values,
                arch_heights: &host.structural_arch_segment_heights,
                steps: &processed.output.steps,
                blockers: &blockers,
            })?;
            let geometry = platform_stair_geometry(
                &host.projection,
                &path.path.positions,
                &path.path.points_u,
                &segments,
                terrain,
            )?;
            let holes = platform_stair_holes(
                &collection.raw_sources[&wall].curve,
                host.state
                    .max_y()
                    .map_err(|_| WallPlatformStairError::InvalidInput)?,
                &segments,
            )?;
            Ok::<_, WallPlatformStairError>((geometry, holes))
        })();
        match result {
            Ok((geometry, holes)) => {
                let mut render_records = Vec::new();
                let mut render_error = None;
                for cell in &geometry.cells {
                    match tiny_glade_reconstruction::building_wall_platform_stair_rows::platform_stair_rows(&host.source.shape,wall,cell) {
                        Ok(records)=>render_records.extend(records),
                        Err(error)=>{render_error=Some(error);break;},
                    }
                }
                if let Some(error) = render_error {
                    report
                        .faults
                        .push(format!("Platform stair render {}: {error:?}", wall.0));
                    state.pending.push(wall);
                    continue;
                }

                let generation = entities
                    .0
                    .get(&wall)
                    .and_then(|e| previous.get(*e).ok())
                    .map_or(1, |s| s.generation + 1);
                state.stairs_lengths.insert(wall, geometry.stairs_length);
                let component = WallCpuPlatformStairs {
                    wall,
                    generation,
                    geometry,
                    holes: holes.clone(),
                    render_records,
                };
                if let Some(entity) = entities.0.get(&wall) {
                    commands.entity(*entity).insert(component);
                } else {
                    let entity = commands.spawn(component).id();
                    entities.0.insert(wall, entity);
                }
                state.holes.insert(wall, holes);
                state.fingerprints.insert(wall, key);
                state.rebuilt.push(wall);
                ready.insert(wall, ());
            }
            Err(error) => {
                report
                    .faults
                    .push(format!("Platform stairs {}: {error:?}", wall.0));
                state.pending.push(wall);
            }
        }
    }
    let stale: Vec<_> = entities
        .0
        .keys()
        .filter(|wall| !ready.contains_key(wall))
        .copied()
        .collect();
    for wall in stale {
        if let Some(entity) = entities.0.remove(&wall) {
            commands.entity(entity).despawn();
        }
        state.holes.remove(&wall);
        state.stairs_lengths.remove(&wall);
        state.fingerprints.remove(&wall);
    }
}

fn finalize_stats(
    paths: Res<WallPreparedPaths>,
    raw: Res<crate::wall_segments::WallPathMaskRegistry>,
    collection: Res<WallCollectionState>,
    stairs: Res<WallPlatformStairState>,
    mut stats: ResMut<crate::wall_segments::WallPathMaterialStats>,
) {
    for (wall, masks) in &raw.records {
        let Some((path, source)) = paths
            .current(*wall, masks.publication_revision)
            .zip(collection.raw_sources.get(wall))
        else {
            continue;
        };
        let mut update =
            tiny_glade_reconstruction::building_wall_segmentation::material_stats_for_update(
                path.path.length,
                u8::from(source.special),
                masks.masks.len(),
            );
        if let Some(length) = stairs.stairs_lengths.get(wall) {
            update.add_stairs_length(*length);
        }
        if stats.0.get(wall) != Some(&update) {
            stats.0.insert(*wall, update);
        }
    }
}
