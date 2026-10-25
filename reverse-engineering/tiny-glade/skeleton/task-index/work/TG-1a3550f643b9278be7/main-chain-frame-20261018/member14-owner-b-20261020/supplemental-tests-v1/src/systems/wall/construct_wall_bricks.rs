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


#[cfg(test)]
mod preassembly_base_transition_tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::geometry::instanced_wall::BrickTransformSsbo;
    use crate::render::sparse_instance_buffer::SparseInstanceBuffer;
    use crate::render::test_device::TestDevice;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::{InnerWalls, WallState};
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::systems::wall::wall_state::path_preparation::PathRasters512;
    use std::sync::Arc;

    fn fixture(source_flag_65: u8) -> (World, Schedule, WallId, Entity) {
        let wall = WallId(0x700000031);
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(wall, WallState::new(
            Curve2::try_new(vec![[0.,0.],[4.,0.]], false).unwrap(),
            2., 0, source_flag_65, false,
        ));
        let mut public = PublicWalls::default();
        public.consume(&mut inner, &TerrainHeightGrid512 {
            encoded: vec![0u16;512*512].into(), minimum_y: 0., maximum_y: 4., sampling_fix: true,
        });
        // Actual published wall, before its actual path/height preparation.
        assert!(public.get(wall).unwrap().prepared_path.is_none());
        let mut storage = SparseInstanceBuffer::<BrickTransformSsbo>::try_new(&TestDevice(1)).unwrap();
        let owner = InstancedWall::new_awaiting_production(wall.0, &storage);
        storage.writer(owner.chimney_allocation().unwrap()).push(&BrickTransformSsbo { seed: 41, ..Default::default() }).unwrap();
        storage.writer(owner.door_stairs_allocation().unwrap()).push(&BrickTransformSsbo { seed: 42, ..Default::default() }).unwrap();
        let library = AssetSsboLibrary::new(); library.register::<BrickTransformSsbo>(storage);
        let mut world = World::new(); world.insert_resource(public); world.insert_resource(library);
        world.init_resource::<WallEntities>(); world.init_resource::<WallHoles>();
        world.init_resource::<WallBrickBuildState>(); world.init_resource::<RtWorld>();
        world.init_resource::<Events<OnWallChanged>>(); world.init_resource::<Events<OnWallDeleted>>();
        world.init_resource::<Events<RecalculateWallBricks>>(); world.init_resource::<Events<WallGableChanged>>();
        world.init_resource::<Events<WallRoofPresenceChanged>>(); world.init_resource::<Events<RoofGeometryChanged>>();
        world.init_resource::<Events<WallBricksChanged>>(); world.init_resource::<Events<UpdateRtWorldEvent>>();
        let entity = world.spawn(owner).id(); world.resource_mut::<WallEntities>().0.insert(wall, entity);
        let mut schedule = Schedule::default(); schedule.add_systems(construct_wall_bricks);
        (world, schedule, wall, entity)
    }

    fn prepare_actual_path(world: &mut World, wall: WallId) {
        // Explicit four physical input buffers, passed through the real path
        // and height algorithms. No fabricated prepared path/Trigger/brick Vec.
        let paths = PathRasters512::from_encoded(
            std::array::from_fn(|_| Arc::<[u8]>::from(vec![0u8;512*512])), true,
        ).unwrap();
        world.resource_mut::<PublicWalls>().get_mut(wall).unwrap().prepare_paths(&paths).unwrap();
    }
    fn lane_headers(world: &World, entity: Entity) -> [usize;3] {
        let owner = world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap();
        [owner.allocation().unwrap(), owner.chimney_allocation().unwrap(), owner.door_stairs_allocation().unwrap()]
            .map(|lane| lane.handle.0.as_deref().unwrap() as *const _ as usize)
    }
    fn assert_deco_payloads(world: &World, entity: Entity) {
        let owner = world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap();
        let storage = world.resource::<AssetSsboLibrary>().get::<BrickTransformSsbo>().unwrap();
        for (lane, seed) in [(owner.chimney_allocation().unwrap(),41), (owner.door_stairs_allocation().unwrap(),42)] {
            assert!(storage.owns_allocation(lane));
            let ranges = storage.allocation_to_draw_multi_instanced(lane);
            assert_eq!(ranges.len(),1); assert_eq!(ranges[0].1,1);
            assert_eq!(storage.data[ranges[0].0 as usize].seed,seed);
        }
    }
    fn request(world: &mut World, wall: WallId) {
        world.resource_mut::<Events<RecalculateWallBricks>>().write(RecalculateWallBricks(wall));
    }

    #[test]
    fn awaiting_real_failure_then_geometry_success_preserves_lanes_and_rejects_later_missing_trigger() {
        let (mut world, mut schedule, wall, entity) = fixture(0);
        let headers = lane_headers(&world,entity);
        request(&mut world,wall); schedule.run(&mut world);
        assert!(matches!(world.resource::<WallBrickBuildState>().failures.get(&wall),Some(WallBrickBuildError::MissingPreparedPath)));
        assert!(world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap().is_awaiting_production());
        assert!(world.get::<WallTriggerVolume>(entity).is_none());
        assert!(world.resource::<Events<WallBricksChanged>>().is_empty());
        assert!(world.resource::<Events<UpdateRtWorldEvent>>().is_empty());
        assert!(world.resource::<RtWorld>().meshes.is_empty());
        assert_eq!(lane_headers(&world,entity),headers); assert_deco_payloads(&world,entity);

        prepare_actual_path(&mut world,wall);
        request(&mut world,wall); schedule.run(&mut world);
        assert!(!world.resource::<WallBrickBuildState>().failures.contains_key(&wall));
        let owner = world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap();
        assert!(!owner.is_awaiting_production()); assert!(owner.is_upload_pending());
        assert!(!owner.pending_instances().is_empty());
        let trigger = world.get::<WallTriggerVolume>(entity).unwrap();
        assert!(trigger.is_valid()); assert!(!trigger.default_tris().is_empty());
        let positions: Vec<_> = trigger.default_tris().iter().flat_map(|t|t.points).collect();
        assert_eq!(world.resource::<RtWorld>().get(RtMeshHandle::from_wall_id(wall)).unwrap().positions,positions);
        let output: Vec<_> = world.resource_mut::<Events<WallBricksChanged>>().drain().collect();
        assert_eq!(output.len(),1); assert_eq!(output[0].wall,wall); assert_eq!(output[0].entity,entity);
        assert_eq!(world.resource_mut::<Events<UpdateRtWorldEvent>>().drain().count(),1);
        assert_eq!(lane_headers(&world,entity),headers); assert_deco_payloads(&world,entity);

        // Exercise the same actual CPU shadow writer after real construction.
        world.resource_scope(|world, mut library: Mut<AssetSsboLibrary>| {
            let mut owner = world.get_mut::<InstancedWall<BrickTransformSsbo>>(entity).unwrap();
            assert!(owner.upload_to_storage_buffer(library.get_mut::<BrickTransformSsbo>().unwrap()).unwrap());
        });
        assert!(!world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap().is_upload_pending());
        assert_eq!(lane_headers(&world,entity),headers); assert_deco_payloads(&world,entity);

        // Only AwaitingProduction may lack the real Trigger. A later component
        // loss must fail without manufacturing a default or consuming success.
        world.entity_mut(entity).remove::<WallTriggerVolume>();
        request(&mut world,wall); schedule.run(&mut world);
        assert!(matches!(world.resource::<WallBrickBuildState>().failures.get(&wall),Some(WallBrickBuildError::StaleWallEntity(e)) if *e==entity));
        assert!(world.get::<WallTriggerVolume>(entity).is_none());
        assert!(!world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap().is_upload_pending());
        assert!(world.resource::<Events<WallBricksChanged>>().is_empty());
        assert!(world.resource::<Events<UpdateRtWorldEvent>>().is_empty());
        assert_eq!(lane_headers(&world,entity),headers); assert_deco_payloads(&world,entity);
    }

    #[test]
    fn native_empty_brick_branch_still_requires_actual_trigger_before_awaiting_success() {
        let (mut world, mut schedule, wall, entity) = fixture(1);
        let headers=lane_headers(&world,entity);
        // Original source+65 special branch really returns an empty brick Vec;
        // this does not authorize bypassing Trigger construction.
        assert!(build_wall_bricks(wall,world.resource::<PublicWalls>().get(wall).unwrap(),None,world.resource::<WallHoles>()).unwrap().is_empty());
        request(&mut world,wall); schedule.run(&mut world);
        assert!(matches!(world.resource::<WallBrickBuildState>().failures.get(&wall),Some(WallBrickBuildError::Trigger(WallTriggerError::MissingPreparedPath))));
        assert!(world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap().is_awaiting_production());
        assert!(world.get::<WallTriggerVolume>(entity).is_none());
        assert!(world.resource::<Events<WallBricksChanged>>().is_empty());
        assert!(world.resource::<Events<UpdateRtWorldEvent>>().is_empty());
        assert_eq!(lane_headers(&world,entity),headers); assert_deco_payloads(&world,entity);

        prepare_actual_path(&mut world,wall);
        request(&mut world,wall); schedule.run(&mut world);
        assert!(!world.resource::<WallBrickBuildState>().failures.contains_key(&wall));
        let owner = world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap();
        assert!(!owner.is_awaiting_production()); assert!(owner.is_upload_pending());
        assert!(owner.pending_instances().is_empty());
        assert!(world.get::<WallTriggerVolume>(entity).unwrap().is_valid());
        assert_eq!(world.resource::<Events<WallBricksChanged>>().len(),1);
        assert_eq!(world.resource::<Events<UpdateRtWorldEvent>>().len(),1);
        assert_eq!(lane_headers(&world,entity),headers); assert_deco_payloads(&world,entity);
    }
}
