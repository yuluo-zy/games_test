//! CPU brick section of the original external system-wall-constructor owner
//! (B 0x140757800), exposed here so restored authority has an executable consumer.
//! The seeded producer, trigger geometry and original trigger→RT mesh output
//! are real. GPU allocation/upload remains with the existing InstancedWall
//! consumer; the complete external corner/skirt/signifier owner is separate.
use crate::geometry::instanced_wall::InstancedWall;
use crate::resources::roofs::Roof;
use crate::resources::roofs::commands::RoofGeometryChanged;
use crate::resources::roofs::commands::WallGableChanged;
use crate::resources::roofs::commands::WallRoofPresenceChanged;
use crate::resources::rt::{RtMeshHandle, RtWorld, UpdateRtWorldEvent};
use crate::resources::walls::WallId;
use crate::resources::walls::public_walls::OnWallChanged;
use crate::resources::walls::public_walls::OnWallDeleted;
use crate::resources::walls::public_walls::PublicWallState;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::public_walls::WallEntities;
use crate::resources::walls::publication_fields::WallId as ObservedWallId;
use crate::resources::walls::wall_holes::HoleStorageError;
use crate::resources::walls::wall_holes::HoleStoragePhase;
use crate::resources::walls::wall_holes::WallHoles;
use crate::resources::walls::wall_trigger_volume::WallTriggerError;
use crate::resources::walls::wall_trigger_volume::WallTriggerVolume;
use crate::systems::wall::construction::bricks::WallBrickInstance;
use crate::systems::wall::construction::parameters::ConstructionSourceError;
use crate::systems::wall::construction::parameters::ConstructionTopPolicy;
use crate::systems::wall::construction::parameters::construction_policy;
use crate::systems::wall::construction::parameters::prepared_construction_arrays;
use crate::systems::wall::construction::production::PlainWallProductionInput;
use crate::systems::wall::construction::production::WallProductionBranch;
use crate::systems::wall::construction::production::WallProductionError;
use crate::systems::wall::construction::production::WallTopBounds;
use crate::systems::wall::construction::production::produce_wall;
use bevy_ecs::prelude::*;
use hashbrown::{HashMap, HashSet};

pub use crate::resources::roofs::commands::RecalculateWallBricks;
/// Actual completed brick output. Trigger/RT consumers can observe this event
/// without pretending their own work has already happened.
#[derive(Event, Clone, Copy, Debug)]
pub struct WallBricksChanged {
    pub wall: WallId,
    pub entity: Entity,
}

#[derive(Clone, Debug)]
pub enum WallBrickBuildError {
    MissingPreparedPath,
    MissingHeightCache,
    Holes(HoleStorageError),
    Source(ConstructionSourceError),
    Production(WallProductionError),
    Trigger(WallTriggerError),
    StaleWallEntity(Entity),
}
#[derive(Resource, Default)]
pub struct WallBrickBuildState {
    pub failures: HashMap<WallId, WallBrickBuildError>,
}

/// B special +296 deliberately skips from_curve and carries an empty brick Vec
/// into normal entity create/update (140758117..12d, 1407581b1..1b5).
/// This is the observed branch, not an unknown alternate producer.
pub fn build_wall_bricks(
    id: WallId,
    wall: &PublicWallState,
    roof: Option<&[u8; 88]>,
    holes: &WallHoles,
) -> Result<Vec<WallBrickInstance>, WallBrickBuildError> {
    if wall.authority.is_special() {
        return Ok(Vec::new());
    }
    let prepared = wall
        .prepared_path
        .as_ref()
        .ok_or(WallBrickBuildError::MissingPreparedPath)?;
    let heights = wall
        .arch_segment_heights()
        .ok_or(WallBrickBuildError::MissingHeightCache)?;
    let policy = construction_policy(
        ObservedWallId(id.0),
        wall.geometry.outline.len() as u64,
        wall.wall_space_length(),
        // Public+295 is FreehandHeightType (CALL 140833230 at 1407581ec).
        // The recovered leaf's historical clutter_version parameter name is wrong.
        wall.authority.freehand_ty(),
        wall.authority.style(),
        wall.shape(),
        wall.geometry.metrics.minimum_position_y,
        wall.geometry.metrics.constrained_extra,
        wall.calculated_max_y(),
        roof,
        0,
    )
    .map_err(WallBrickBuildError::Source)?;
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
    let arches = policy.constructor_arch_heights(heights);
    let arrays = prepared_construction_arrays(&prepared.positions, &arches, &wall.segment_heights)
        .map_err(WallBrickBuildError::Source)?;
    let finalized = holes
        .storage
        .get(&id)
        .map(|storage| storage.finalized_holes())
        .transpose()
        .map_err(WallBrickBuildError::Holes)?
        .unwrap_or(&[]);
    let hole_input: Vec<_> = finalized
        .iter()
        .map(|hole| (hole.aabb, hole.origin))
        .collect();
    produce_wall(
        PlainWallProductionInput {
            curve: &arrays.curve,
            arch_heights: &arrays.arch_heights,
            second_heights: &arrays.second_heights,
            third_heights: &arrays.third_heights,
            holes: &hole_input,
            holes_length: wall.wall_space_length(),
            closed: wall.shape().tag() != 2,
            seed: policy.seed,
            brick_source_id: policy.brick_source_id,
            base: policy.base,
            row_end_perturbation: policy.row_end_perturbation,
            width_padding: policy.options.width_padding,
            thickness_scale: policy.options.thickness_scale,
            wall_float: policy.wall_float,
        },
        WallProductionBranch {
            kind: policy.kind,
            top,
            fence_fit: false,
        },
    )
    .map_err(WallBrickBuildError::Production)
}

pub fn finalize_wall_holes(
    public: Res<PublicWalls>,
    mut holes: ResMut<WallHoles>,
    mut recalculate: EventWriter<RecalculateWallBricks>,
    mut state: ResMut<WallBrickBuildState>,
) {
    for (&id, storage) in &mut holes.storage {
        if storage.phase() == HoleStoragePhase::Finalized {
            continue;
        }
        let Some(wall) = public.get(id) else {
            continue;
        };
        match storage.finalize(wall.wall_space_length(), wall.shape().tag() != 2) {
            Ok(true) => {
                recalculate.write(RecalculateWallBricks(id));
            }
            Ok(false) => {}
            Err(error) => {
                state.failures.insert(id, WallBrickBuildError::Holes(error));
            }
        }
    }
}

/// Match the original union/dedup of wall changes and explicit recalculation.
/// Roof events are translated here because that external constructor owner has
/// been migrated into this local CPU wiring layer.
#[derive(bevy_ecs::system::SystemParam)]
pub struct WallBrickRequests<'w, 's> {
    changed: EventReader<'w, 's, OnWallChanged>,
    recalculate: EventReader<'w, 's, RecalculateWallBricks>,
    gable: EventReader<'w, 's, WallGableChanged>,
    presence: EventReader<'w, 's, WallRoofPresenceChanged>,
    geometry: EventReader<'w, 's, RoofGeometryChanged>,
    deleted: EventReader<'w, 's, OnWallDeleted>,
}

/// B @140759669..96cf / @1407599c6..9a28. Keep the existing mesh transform,
/// replace its albedo and complete triangle list from the actual trigger
/// cache. Reusing a wall must not append its previous geometry a second time.
pub fn update_wall_rt_mesh(id: WallId, trigger: &WallTriggerVolume, world: &mut RtWorld) {
    let mesh = world.get_mut(RtMeshHandle::from_wall_id(id));
    mesh.set_wall_albedo(id);
    mesh.positions.clear();
    mesh.positions.extend(
        trigger
            .default_tris()
            .iter()
            .flat_map(|triangle| triangle.points),
    );
}

pub fn construct_wall_bricks(
    mut requests: WallBrickRequests,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    holes: Res<WallHoles>,
    mut entities: ResMut<WallEntities>,
    mut bricks: Query<(
        &mut InstancedWall<WallBrickInstance>,
        Option<&mut WallTriggerVolume>,
    )>,
    mut commands: Commands,
    mut state: ResMut<WallBrickBuildState>,
    mut output: EventWriter<WallBricksChanged>,
    mut rt_world: ResMut<RtWorld>,
    mut rt_updates: EventWriter<UpdateRtWorldEvent>,
) {
    let mut ids = HashSet::new();
    ids.extend(requests.changed.read().map(|event| event.0));
    ids.extend(requests.recalculate.read().map(|event| event.0));
    ids.extend(requests.gable.read().map(|event| event.0));
    ids.extend(requests.presence.read().map(|event| event.0));
    ids.extend(requests.geometry.read().map(|event| event.wall));
    for event in requests.deleted.read() {
        state.failures.remove(&event.0);
    }
    for id in ids {
        let Some(wall) = public.get(id) else {
            continue;
        };
        let roof = public
            .roof_entity(id)
            .and_then(|entity| roofs.get(entity).ok());
        let instances = match build_wall_bricks(id, wall, roof.map(Roof::observed), &holes) {
            Ok(instances) => instances,
            Err(error) => {
                state.failures.insert(id, error);
                continue;
            }
        };
        let entity = if let Some(entity) = entities.wall_entity(id) {
            let Ok((mut value, trigger)) = bricks.get_mut(entity) else {
                state
                    .failures
                    .insert(id, WallBrickBuildError::StaleWallEntity(entity));
                continue;
            };
            if value.id != id.0 {
                state.failures.insert(id, WallBrickBuildError::StaleWallEntity(entity));
                continue;
            }
            if let Some(mut trigger) = trigger {
                if let Err(error) = trigger.recompute(wall) {
                    state.failures.insert(id, WallBrickBuildError::Trigger(error));
                    continue;
                }
                value.update(instances);
                update_wall_rt_mesh(id, &trigger, &mut rt_world);
            } else {
                if !value.is_awaiting_production() {
                    state.failures.insert(id, WallBrickBuildError::StaleWallEntity(entity));
                    continue;
                }
                let mut trigger = WallTriggerVolume::default();
                if let Err(error) = trigger.recompute(wall) {
                    state.failures.insert(id, WallBrickBuildError::Trigger(error));
                    continue;
                }
                value.update(instances);
                update_wall_rt_mesh(id, &trigger, &mut rt_world);
                commands.entity(entity).insert(trigger);
            }
            rt_updates.write(UpdateRtWorldEvent);
            entity
        } else {
            let mut trigger = WallTriggerVolume::default();
            if let Err(error) = trigger.recompute(wall) {
                state
                    .failures
                    .insert(id, WallBrickBuildError::Trigger(error));
                continue;
            }
            update_wall_rt_mesh(id, &trigger, &mut rt_world);
            rt_updates.write(UpdateRtWorldEvent);
            let entity = commands
                .spawn((InstancedWall::new(id.0, instances), trigger))
                .id();
            entities.0.insert(id, entity);
            entity
        };
        state.failures.remove(&id);
        output.write(WallBricksChanged { wall: id, entity });
    }
}
