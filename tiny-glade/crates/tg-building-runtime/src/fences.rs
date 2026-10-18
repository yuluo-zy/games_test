//! Prepared wall paths produce CPU fences with wall-scoped entity ownership.
//! Rendering uploads are handled by the consuming backend.
use crate::{
    BuildingReport, BuildingStage, PublicWalls, WallCollectionState, wall_paths::WallPreparedPaths,
};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred};
use country_core::systems::wall::construction::fences::FenceConstructionInput;
use country_core::systems::wall::construction::fences::FenceCpuOutput;
use country_core::systems::wall::construction::fences::FenceError;
use country_core::systems::wall::construction::fences::construct_fences_cpu;
use country_core::resources::walls::publication_fields::WallId;
use std::collections::{HashMap, HashSet};
#[derive(Component, Clone, Debug)]
pub struct WallCpuFences {
    pub wall: WallId,
    pub generation: u64,
    pub output: FenceCpuOutput,
}
#[derive(Resource, Default)]
pub struct FenceEntities(pub HashMap<WallId, Entity>);
#[derive(Resource, Default)]
pub struct FenceBuildState {
    fingerprints: HashMap<WallId, Vec<u8>>,
    pub pending: HashMap<WallId, FenceError>,
    pub rebuilt: Vec<WallId>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<FenceEntities>()
        .init_resource::<FenceBuildState>()
        .add_systems(
            bevy_app::Update,
            (rebuild, ApplyDeferred)
                .chain()
                .in_set(BuildingStage::Fences),
        );
}
fn withdraw(
    wall: WallId,
    commands: &mut Commands,
    entities: &mut FenceEntities,
    state: &mut FenceBuildState,
) {
    if let Some(entity) = entities.0.remove(&wall) {
        commands.entity(entity).despawn();
    }
    state.fingerprints.remove(&wall);
}
fn rebuild(
    mut commands: Commands,
    walls: Res<PublicWalls>,
    source: Res<WallCollectionState>,
    paths: Res<WallPreparedPaths>,
    mut entities: ResMut<FenceEntities>,
    mut state: ResMut<FenceBuildState>,
    previous: Query<&WallCpuFences>,
    mut report: ResMut<BuildingReport>,
) {
    state.rebuilt.clear();
    let active: HashSet<_> = source
        .raw_sources
        .keys()
        .copied()
        .filter(|wall| {
            walls
                .walls
                .get(wall)
                .is_some_and(|host| host.state.observed_296() == 1)
        })
        .collect();
    let mut stale: Vec<_> = entities
        .0
        .keys()
        .chain(state.pending.keys())
        .copied()
        .filter(|wall| !active.contains(wall))
        .collect();
    stale.sort_unstable_by_key(|wall| wall.0);
    stale.dedup();
    for wall in stale {
        withdraw(wall, &mut commands, &mut entities, &mut state);
        state.pending.remove(&wall);
    }
    let mut ids: Vec<_> = active.into_iter().collect();
    ids.sort_unstable_by_key(|wall| wall.0);
    for wall in ids {
        let host = &walls.walls[&wall];
        let raw = &source.raw_sources[&wall];
        let Some(path) = paths.current(wall, host.publication_revision) else {
            withdraw(wall, &mut commands, &mut entities, &mut state);
            state.pending.insert(wall, FenceError::InvalidInput);
            continue;
        };
        let mut fingerprint = host.observed_projection().to_vec();
        fingerprint.extend(host.publication_revision.to_le_bytes());
        fingerprint.extend(path.generation.to_le_bytes());
        if state.fingerprints.get(&wall) == Some(&fingerprint) {
            continue;
        }
        // The full threshold callback derives seed from WallId + range and
        // spacing extent from the actual authoritative raw Curve2.
        let input = FenceConstructionInput {
            wall,
            prepared_positions: &path.path.positions,
            segment_values: &path.path.segment_values,
            raw_positions: &raw.curve.points,
            observed_public_284: host.state.float_0x284(),
            observed_public_296: host.state.observed_296(),
        };
        let output = match construct_fences_cpu(input) {
            Ok(output) => output,
            Err(error) => {
                withdraw(wall, &mut commands, &mut entities, &mut state);
                state.pending.insert(wall, error);
                report
                    .faults
                    .push(format!("Fence CPU {}: {error:?}", wall.0));
                continue;
            }
        };
        let generation = entities
            .0
            .get(&wall)
            .and_then(|entity| previous.get(*entity).ok())
            .map_or(1, |old| old.generation.wrapping_add(1));
        let component = WallCpuFences {
            wall,
            generation,
            output,
        };
        if let Some(entity) = entities.0.get(&wall) {
            commands.entity(*entity).insert(component);
        } else {
            let entity = commands.spawn(component).id();
            entities.0.insert(wall, entity);
        }
        state.pending.remove(&wall);
        state.fingerprints.insert(wall, fingerprint);
        state.rebuilt.push(wall);
    }
}
