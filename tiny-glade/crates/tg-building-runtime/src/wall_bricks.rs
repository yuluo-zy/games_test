//! Builds CPU brick instances from wall authority, height caches, and openings.
use crate::{
    BuildingReport, PublicWalls, Roof, WallCollectionState, WallRoofMap, openings::WallOpenings,
    wall_paths::WallPreparedPaths,
};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred};
use country_core::systems::wall::construction::parameters::ConstructionTopPolicy;
use country_core::systems::wall::construction::parameters::construction_policy;
use country_core::systems::wall::construction::parameters::prepared_construction_arrays;
use country_core::systems::wall::construction::bricks::WallBrickInstance;
use country_core::systems::wall::construction::production::PlainWallProductionInput;
use country_core::systems::wall::construction::production::WallProductionBranch;
use country_core::systems::wall::construction::production::WallTopBounds;
use country_core::systems::wall::construction::production::produce_wall;
use country_core::resources::walls::publication_fields::WallId;
use std::collections::HashMap;

#[derive(Component, Clone, Debug)]
pub struct WallCpuBricks {
    pub wall: WallId,
    pub instances: Vec<WallBrickInstance>,
    pub generation: u64,
}
#[derive(Resource, Default)]
pub struct WallBrickEntities(pub HashMap<WallId, Entity>);
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PendingWallProductionReason {
    MissingRawAuthority,
    MissingPreparedPath,
    PendingHeightCache,
    ConstructionPolicy,
    SpecialConstruction,
    UnfinishedHoles,
    InvalidConstructionArrays,
    OriginalProducerBranch,
}
#[derive(Resource, Default)]
pub struct WallBrickBuildState {
    fingerprints: HashMap<WallId, Vec<u32>>,
    fingerprint_scratch: Vec<u32>,
    pub rebuilt: Vec<WallId>,
    /// Per-wall working-array preparations attempted during this update.
    pub prepared_arrays_this_frame: usize,
    pub pending: HashMap<WallId, PendingWallProductionReason>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<WallBrickEntities>()
        .init_resource::<WallBrickBuildState>()
        .add_systems(
            bevy_app::Update,
            (construct, ApplyDeferred)
                .chain()
                .in_set(crate::BuildingStage::WallGeometry),
        );
}
fn withdraw(
    wall: WallId,
    commands: &mut Commands,
    entities: &mut WallBrickEntities,
    state: &mut WallBrickBuildState,
    reason: PendingWallProductionReason,
) {
    if let Some(entity) = entities.0.remove(&wall) {
        commands.entity(entity).despawn();
    }
    state.fingerprints.remove(&wall);
    state.pending.insert(wall, reason);
}
fn construct(
    mut commands: Commands,
    walls: Res<PublicWalls>,
    collection: Res<WallCollectionState>,
    paths: Res<WallPreparedPaths>,
    holes: Res<WallOpenings>,
    roof_map: Res<WallRoofMap>,
    roofs: Query<&Roof>,
    mut state: ResMut<WallBrickBuildState>,
    mut entities: ResMut<WallBrickEntities>,
    output: Query<&WallCpuBricks>,
    mut report: ResMut<BuildingReport>,
) {
    state.rebuilt.clear();
    state.prepared_arrays_this_frame = 0;
    state
        .pending
        .retain(|wall, _| walls.walls.contains_key(wall));
    state
        .fingerprints
        .retain(|wall, _| walls.walls.contains_key(wall));
    let orphaned: Vec<_> = entities
        .0
        .keys()
        .filter(|wall| !walls.walls.contains_key(wall))
        .copied()
        .collect();
    for wall in orphaned {
        if let Some(entity) = entities.0.remove(&wall) {
            commands.entity(entity).despawn();
        }
    }
    let mut ids: Vec<_> = walls.walls.keys().copied().collect();
    ids.sort_unstable_by_key(|wall| wall.0);
    for wall in ids {
        let host = &walls.walls[&wall];
        let Some(raw) = collection.raw_sources.get(&wall) else {
            withdraw(
                wall,
                &mut commands,
                &mut entities,
                &mut state,
                PendingWallProductionReason::MissingRawAuthority,
            );
            continue;
        };
        if raw.special {
            withdraw(
                wall,
                &mut commands,
                &mut entities,
                &mut state,
                PendingWallProductionReason::SpecialConstruction,
            );
            continue;
        }
        let Some(prepared) = paths.current(wall, host.publication_revision) else {
            withdraw(
                wall,
                &mut commands,
                &mut entities,
                &mut state,
                PendingWallProductionReason::MissingPreparedPath,
            );
            continue;
        };
        if host.gable_profile_pending || host.arch_segment_heights.is_empty() {
            withdraw(
                wall,
                &mut commands,
                &mut entities,
                &mut state,
                PendingWallProductionReason::PendingHeightCache,
            );
            continue;
        }
        let roof = roof_map
            .entity(wall)
            .and_then(|entity| roofs.get(entity).ok());
        let projection = crate::wall_projection::WallProjection::new(host.observed_projection());
        let policy = match construction_policy(
            wall,
            host.source.outline.len() as u64,
            host.source.source_curve_length,
            raw.clutter_version,
            raw.style,
            &raw.shape,
            projection.observed_f32_0x274(),
            host.state.float_0x284(),
            host.state.max_y().ok(),
            roof.map(|roof| &roof.0),
            host.state.observed_296(),
        ) {
            Ok(policy) => policy,
            Err(_) => {
                withdraw(
                    wall,
                    &mut commands,
                    &mut entities,
                    &mut state,
                    PendingWallProductionReason::ConstructionPolicy,
                );
                continue;
            }
        };
        let top = match policy.top {
            ConstructionTopPolicy::Plain => WallTopBounds::PLAIN,
            ConstructionTopPolicy::Gable {
                height,
                intervals,
                tag,
            } => WallTopBounds {
                height,
                intervals,
                tag,
            },
        };
        let finalized = match holes.storage.get(&wall) {
            Some(storage) => match storage.finalized_holes() {
                Ok(value) => value,
                Err(_) => {
                    withdraw(
                        wall,
                        &mut commands,
                        &mut entities,
                        &mut state,
                        PendingWallProductionReason::UnfinishedHoles,
                    );
                    continue;
                }
            },
            None => &[],
        };
        // A stable wall still needs content comparison because height arrays
        // are publicly editable. Reuse one scratch buffer across those checks.
        let mut key = std::mem::take(&mut state.fingerprint_scratch);
        key.clear();
        key.extend([
            host.publication_revision as u32,
            (host.publication_revision >> 32) as u32,
            prepared.generation as u32,
            (prepared.generation >> 32) as u32,
            policy.seed as u32,
            (policy.seed >> 32) as u32,
            policy.brick_source_id,
            policy.base.to_bits(),
            policy.wall_float.to_bits(),
            policy.row_end_perturbation.to_bits(),
            host.source.source_curve_length.to_bits(),
            raw.shape.tag(),
            policy.kind as u32,
            top.tag as u32,
            top.height.to_bits(),
            u32::from(policy.lower_flat_no_rim_arches),
        ]);
        for option in [policy.options.width_padding, policy.options.thickness_scale] {
            key.push(u32::from(option.is_some()));
            key.push(option.unwrap_or_default().to_bits());
        }
        key.extend(top.intervals.into_iter().flatten().map(f32::to_bits));
        // Construction arrays are a deterministic projection of these source
        // arrays and the policy above. Compare inputs before allocating them.
        for values in [&host.arch_segment_heights, &host.segment_heights] {
            key.push(values.len() as u32);
            key.extend(values.iter().map(|value| value.to_bits()));
        }
        key.push(finalized.len() as u32);
        for hole in finalized {
            key.extend(hole.aabb.iter().map(|value| value.to_bits()));
            key.push(hole.origin as u32);
        }
        if state.fingerprints.get(&wall) == Some(&key) && entities.0.contains_key(&wall) {
            state.fingerprint_scratch = key;
            continue;
        }
        state.prepared_arrays_this_frame += 1;
        let construction_arches = policy.constructor_arch_heights(&host.arch_segment_heights);
        let arrays = match prepared_construction_arrays(
            &prepared.path.positions,
            &construction_arches,
            &host.segment_heights,
        ) {
            Ok(arrays) => arrays,
            Err(_) => {
                state.fingerprint_scratch = key;
                withdraw(
                    wall,
                    &mut commands,
                    &mut entities,
                    &mut state,
                    PendingWallProductionReason::InvalidConstructionArrays,
                );
                continue;
            }
        };
        let hole_input: Vec<_> = finalized
            .iter()
            .map(|hole| (hole.aabb, hole.origin))
            .collect();
        let input = PlainWallProductionInput {
            curve: &arrays.curve,
            arch_heights: &arrays.arch_heights,
            second_heights: &arrays.second_heights,
            third_heights: &arrays.third_heights,
            holes: &hole_input,
            holes_length: host.source.source_curve_length,
            closed: raw.shape.tag() != 2,
            seed: policy.seed,
            brick_source_id: policy.brick_source_id,
            base: policy.base,
            row_end_perturbation: policy.row_end_perturbation,
            width_padding: policy.options.width_padding,
            thickness_scale: policy.options.thickness_scale,
            wall_float: policy.wall_float,
        };
        let instances = match produce_wall(
            input,
            WallProductionBranch {
                kind: policy.kind,
                top,
                fence_fit: false,
            },
        ) {
            Ok(instances) => instances,
            Err(error) => {
                state.fingerprint_scratch = key;
                withdraw(
                    wall,
                    &mut commands,
                    &mut entities,
                    &mut state,
                    PendingWallProductionReason::OriginalProducerBranch,
                );
                report
                    .faults
                    .push(format!("Raw wall brick production {}: {error:?}", wall.0));
                continue;
            }
        };
        if let Some(entity) = entities.0.get(&wall).copied() {
            let generation = output
                .get(entity)
                .map_or(1, |value| value.generation.wrapping_add(1));
            commands.entity(entity).insert(WallCpuBricks {
                wall,
                instances,
                generation,
            });
        } else {
            let entity = commands
                .spawn(WallCpuBricks {
                    wall,
                    instances,
                    generation: 1,
                })
                .id();
            entities.0.insert(wall, entity);
        }
        state.pending.remove(&wall);
        state.fingerprint_scratch = state.fingerprints.insert(wall, key).unwrap_or_default();
        state.rebuilt.push(wall);
    }
}
