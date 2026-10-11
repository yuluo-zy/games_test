//! Prepared raw wall authority -> original RNG/noise/height producer -> 96B bricks.
use crate::{
    BuildingReport, PublicWalls, Roof, WallCollectionState, WallRoofMap, openings::WallOpenings,
    wall_paths::WallPreparedPaths,
};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred};
use std::collections::HashMap;
use tiny_glade_reconstruction::{
    building_wall_construction_sources::{
        ConstructionTopPolicy, construction_policy, prepared_construction_arrays,
    },
    building_wall_constructor::WallBrickInstance,
    building_wall_production::{
        PlainWallProductionInput, WallProductionBranch, WallTopBounds, produce_wall,
    },
    building_wall_state::WallId,
};

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
    pub rebuilt: Vec<WallId>,
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
        let field =
            |offset| f32::from_le_bytes(host.projection[offset..offset + 4].try_into().unwrap());
        let policy = match construction_policy(
            wall,
            host.source.outline.len() as u64,
            host.source.source_curve_length,
            raw.clutter_version,
            raw.style,
            &raw.shape,
            field(0x274),
            field(0x284),
            host.state.max_y().ok(),
            roof.map(|roof| &roof.0),
            host.projection[0x296],
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
        let construction_arches = policy.constructor_arch_heights(&host.arch_segment_heights);
        let arrays = match prepared_construction_arrays(
            &prepared.path.positions,
            &construction_arches,
            &host.segment_heights,
        ) {
            Ok(arrays) => arrays,
            Err(_) => {
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
        let hole_input: Vec<_> = finalized
            .iter()
            .map(|hole| (hole.aabb, hole.origin))
            .collect();
        let mut key = vec![
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
        ];
        key.extend(top.intervals.into_iter().flatten().map(f32::to_bits));
        for values in [
            &arrays.arch_heights,
            &arrays.second_heights,
            &arrays.third_heights,
        ] {
            key.push(values.len() as u32);
            key.extend(values.iter().map(|value| value.to_bits()));
        }
        key.push(hole_input.len() as u32);
        for (aabb, origin) in &hole_input {
            key.extend(aabb.iter().map(|value| value.to_bits()));
            key.push(*origin as u32);
        }
        if state.fingerprints.get(&wall) == Some(&key) && entities.0.contains_key(&wall) {
            continue;
        }
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
        state.fingerprints.insert(wall, key);
        state.rebuilt.push(wall);
    }
}
