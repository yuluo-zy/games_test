//! Builds arch instances, derived heights, and gate commands from processed masks.
use crate::{
    BuildingStage, PublicWalls, Roof, WallCollectionState, WallRoofMap,
    wall_paths::WallPreparedPaths, wall_segments::WallProcessedMaskRegistry,
};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred};
use country_core::systems::wall::construction::arches::ConstructGateCmd120Projection;
use country_core::systems::wall::construction::arches::RegularWallArchesInput;
use country_core::systems::wall::construction::arches::construct_regular_wall_arches;
use country_core::systems::wall::construction::freehand_arches::FreehandWallArchesInput;
use country_core::systems::wall::construction::freehand_arches::construct_freehand_wall_arches;
use country_core::systems::wall::construction::bricks::WallBrickInstance;
use country_core::resources::walls::publication_fields::WallId;
use std::collections::{HashMap, HashSet};
#[derive(Event, Clone, Debug)]
pub struct ConstructGateCmd(pub ConstructGateCmd120Projection);
#[derive(Component, Clone, Debug)]
pub struct WallCpuArches {
    pub wall: WallId,
    pub generation: u64,
    pub instances: Vec<WallBrickInstance>,
}
#[derive(Resource, Default)]
pub struct WallArchEntities(pub HashMap<WallId, Entity>);
#[derive(Resource, Default)]
pub struct WallArchesBuildState {
    fingerprints: HashMap<WallId, Vec<u8>>,
    derived_heights: HashMap<WallId, (Vec<f32>, Vec<f32>)>,
    pub pending: Vec<WallId>,
    pub rebuilt: Vec<WallId>,
}
pub fn register(app: &mut bevy_app::App) {
    app.add_event::<ConstructGateCmd>()
        .init_resource::<WallArchEntities>()
        .init_resource::<WallArchesBuildState>()
        .add_systems(
            bevy_app::Update,
            (rebuild, ApplyDeferred)
                .chain()
                .in_set(BuildingStage::WallArches),
        );
}
fn rebuild(
    mut commands: Commands,
    mut walls: ResMut<PublicWalls>,
    source: Res<WallCollectionState>,
    paths: Res<WallPreparedPaths>,
    masks: Res<WallProcessedMaskRegistry>,
    roof_map: Res<WallRoofMap>,
    roofs: Query<&Roof>,
    mut entities: ResMut<WallArchEntities>,
    mut state: ResMut<WallArchesBuildState>,
    previous: Query<&WallCpuArches>,
    mut gates: EventWriter<ConstructGateCmd>,
    mut report: ResMut<crate::BuildingReport>,
) {
    state.pending.clear();
    state.rebuilt.clear();
    let active: HashSet<_> = source
        .raw_sources
        .keys()
        .copied()
        .filter(|id| {
            walls
                .walls
                .get(id)
                .is_some_and(|w| w.state.observed_296() == 0)
        })
        .collect();
    let stale: Vec<_> = entities
        .0
        .keys()
        .copied()
        .filter(|id| !active.contains(id))
        .collect();
    for id in stale {
        if let Some(e) = entities.0.remove(&id) {
            commands.entity(e).despawn();
        }
        state.fingerprints.remove(&id);
        state.derived_heights.remove(&id);
        if let Some(w) = walls.walls.get_mut(&id) {
            // Lintels run before this stage and own the special wall's heights.
            w.arch_segment_heights
                .clone_from(&w.structural_arch_segment_heights);
        }
    }
    let mut ids: Vec<_> = active.into_iter().collect();
    ids.sort_unstable_by_key(|id| id.0);
    for wall in ids {
        let host = walls.walls.get_mut(&wall).unwrap();
        let pair = paths
            .current(wall, host.publication_revision)
            .zip(masks.records.get(&wall));
        let Some((path, record)) = pair.filter(|(p, m)| {
            !host.gable_profile_pending
                && m.prepared_generation == p.generation
                && !host.structural_arch_segment_heights.is_empty()
        }) else {
            if let Some(e) = entities.0.remove(&wall) {
                commands.entity(e).despawn();
            }
            state.fingerprints.remove(&wall);
            state.derived_heights.remove(&wall);
            host.arch_segment_heights = host.structural_arch_segment_heights.clone();
            host.segment_heights = host.base_segment_heights.clone();
            state.pending.push(wall);
            continue;
        };
        let roof = roof_map.entity(wall).and_then(|e| roofs.get(e).ok());
        let mut fingerprint = host.projection.to_vec();
        fingerprint.extend(host.publication_revision.to_le_bytes());
        fingerprint.extend(path.generation.to_le_bytes());
        fingerprint.extend(record.generation.to_le_bytes());
        fingerprint.push(u8::from(roof.is_some()));
        if let Some(r) = roof {
            fingerprint.extend(r.0);
        }
        for f in host
            .structural_arch_segment_heights
            .iter()
            .chain(&host.base_segment_heights)
        {
            fingerprint.extend(f.to_le_bytes());
        }
        if state.fingerprints.get(&wall) == Some(&fingerprint) {
            // Base/gable stages may reset these arrays on a redundant dirty
            // event even when the construction inputs are unchanged.
            if let Some((arch_heights, segment_heights)) = state.derived_heights.get(&wall) {
                if !host
                    .arch_segment_heights
                    .iter()
                    .map(|value| value.to_bits())
                    .eq(arch_heights.iter().map(|value| value.to_bits()))
                {
                    host.arch_segment_heights.clone_from(arch_heights);
                }
                if !host
                    .segment_heights
                    .iter()
                    .map(|value| value.to_bits())
                    .eq(segment_heights.iter().map(|value| value.to_bits()))
                {
                    host.segment_heights.clone_from(segment_heights);
                }
                continue;
            }
        }
        // Start from upstream caches so maxima and smoothing cannot accumulate
        // across repeated rebuilds of the same inputs.
        let mut arch_heights = host.structural_arch_segment_heights.clone();
        let mut segment_heights = host.base_segment_heights.clone();
        let wall_height_parameter = host.state.float_0x284();
        let path_scalar = host.state.float_0x288();
        let source_length = host.state.published_source_curve_length();
        let tag = host.source.shape.tag();
        let result = if tag == 2 {
            construct_freehand_wall_arches(
                FreehandWallArchesInput {
                    masks: &record.output.processed_masks,
                    prepared_positions: &path.path.positions,
                    prepared_u: &path.path.points_u,
                    path_values: &path.path.segment_values,
                    public_284: wall_height_parameter,
                    public_288: path_scalar,
                    observed_294: host.state.observed_294(),
                    clutter_version: crate::wall_projection::WallProjection::new(
                        host.observed_projection(),
                    )
                    .clutter_version(),
                    source_curve_length: source_length,
                    wall_id: wall,
                    has_roof: roof.is_some(),
                },
                &mut arch_heights,
                &mut segment_heights,
            )
        } else {
            construct_regular_wall_arches(
                RegularWallArchesInput {
                    masks: &record.output.processed_masks,
                    prepared_positions: &path.path.positions,
                    prepared_u: &path.path.points_u,
                    path_values: &path.path.segment_values,
                    shape_tag: tag,
                    public_284: wall_height_parameter,
                    observed_294: host.state.observed_294(),
                    source_curve_length: source_length,
                    wall_id: wall,
                    has_roof: roof.is_some(),
                },
                &mut arch_heights,
                &mut segment_heights,
            )
        };
        let output = match result {
            Ok(v) => v,
            Err(e) => {
                if let Some(entity) = entities.0.remove(&wall) {
                    commands.entity(entity).despawn();
                }
                state.fingerprints.remove(&wall);
                state.derived_heights.remove(&wall);
                host.arch_segment_heights = host.structural_arch_segment_heights.clone();
                host.segment_heights = host.base_segment_heights.clone();
                state.pending.push(wall);
                report.faults.push(format!("Wall arches {}: {e:?}", wall.0));
                continue;
            }
        };
        state
            .derived_heights
            .insert(wall, (arch_heights.clone(), segment_heights.clone()));
        host.arch_segment_heights = arch_heights;
        host.segment_heights = segment_heights;
        for gate in output.gate_commands {
            gates.write(ConstructGateCmd(gate));
        }
        let generation = entities
            .0
            .get(&wall)
            .and_then(|e| previous.get(*e).ok())
            .map_or(1, |v| v.generation.wrapping_add(1));
        let component = WallCpuArches {
            wall,
            generation,
            instances: output.instances,
        };
        if let Some(e) = entities.0.get(&wall) {
            commands.entity(*e).insert(component);
        } else {
            entities.0.insert(wall, commands.spawn(component).id());
        }
        state.fingerprints.insert(wall, fingerprint);
        state.rebuilt.push(wall);
    }
}
