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
use crate::systems::wall::construction::parameters::PreparedConstructionArrays;
use crate::systems::wall::construction::parameters::WallConstructionPolicy;
use crate::systems::wall::construction::parameters::construction_policy;
use crate::systems::wall::construction::parameters::prepared_construction_arrays;
use crate::systems::wall::construction::production::PlainWallProductionInput;
use crate::systems::wall::construction::production::WallProductionBranch;
use crate::systems::wall::construction::production::WallProductionError;
use crate::systems::wall::construction::production::WallTopBounds;
use crate::systems::wall::construction::production::produce_wall_into;
use bevy_ecs::prelude::*;
use hashbrown::HashMap;

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
    let mut instances = Vec::new();
    if let Some(prepared) = prepare_wall_bricks(id, wall, roof)? {
        let finalized = finalized_wall_holes(id, holes)?;
        produce_prepared_wall(wall, &prepared, finalized, &mut instances)?;
    }
    Ok(instances)
}

/// 原父链在主体网格之后继续使用同一份采样和构造策略；不重复采样或复制数组。
pub struct WallBrickPreparedGeometry {
    pub arrays: PreparedConstructionArrays,
    pub kind: u8,
    pub top: WallTopBounds,
    policy: WallConstructionPolicy,
}

fn prepare_wall_bricks(
    id: WallId,
    wall: &PublicWallState,
    roof: Option<&[u8; 88]>,
) -> Result<Option<WallBrickPreparedGeometry>, WallBrickBuildError> {
    if wall.authority.is_special() {
        return Ok(None);
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
    Ok(Some(WallBrickPreparedGeometry { arrays, kind: policy.kind, top, policy }))
}

fn finalized_wall_holes(id: WallId, holes: &WallHoles)
    -> Result<&[crate::resources::walls::wall_holes::FinalizedWallHole], WallBrickBuildError> {
    holes
        .storage
        .get(&id)
        .map(|storage| storage.finalized_holes())
        .transpose()
        .map_err(WallBrickBuildError::Holes)
        .map(|holes| holes.unwrap_or(&[]))
}

fn produce_prepared_wall(
    wall: &PublicWallState,
    prepared: &WallBrickPreparedGeometry,
    finalized: &[crate::resources::walls::wall_holes::FinalizedWallHole],
    instances: &mut Vec<WallBrickInstance>,
) -> Result<(), WallBrickBuildError> {
    let arrays = &prepared.arrays;
    let policy = prepared.policy;
    let hole_input: Vec<_> = finalized
        .iter()
        .map(|hole| (hole.aabb, hole.origin))
        .collect();
    produce_wall_into(
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
            top: prepared.top,
            fence_fit: false,
        },
        instances,
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

/// 正式外层墙构造器复用同一逐墙处理；分组只适配当前Bevy参数数量。
/// core砖构造入口保留自身职责，完整半木/裙边由原wall-constructor owner同步补齐。
#[derive(bevy_ecs::system::SystemParam)]
pub struct WallBrickConstruction<'w, 's> {
    requests: WallBrickRequests<'w, 's>,
    public: Res<'w, PublicWalls>,
    roofs: Query<'w, 's, &'static Roof>,
    holes: ResMut<'w, WallHoles>,
    entities: ResMut<'w, WallEntities>,
    bricks: Query<'w, 's, (ComponentWrite<InstancedWall<WallBrickInstance>>, Option<&'static mut WallTriggerVolume>)>,
    commands: Commands<'w, 's>,
    state: ResMut<'w, WallBrickBuildState>,
    output: EventWriter<'w, WallBricksChanged>,
    plaster: EventWriter<'w, crate::resources::walls::plaster::RecalculatePlasterCmd>,
    rt_world: ResMut<'w, RtWorld>,
    rt_updates: EventWriter<'w, UpdateRtWorldEvent>,
}

/// 原140758cf3..14075925f：先完成附加砖及局部洞，再让主体追加到同一数组。
/// 返回的局部最终化洞口只供本次主体使用；None沿用全局洞，special跳过主体。
pub type WallBrickBeforeProduction<'a> = dyn FnMut(WallId, &PublicWallState, Option<&Roof>, Option<&WallBrickPreparedGeometry>, &mut WallHoles, &mut Vec<WallBrickInstance>) -> Option<crate::resources::walls::wall_holes::WallHoleStorage> + 'a;

impl WallBrickConstruction<'_, '_> {
    pub fn run_with(self, mut before_production: Option<&mut WallBrickBeforeProduction<'_>>) {
        let Self { mut requests, public, roofs, mut holes, mut entities, mut bricks,
            mut commands, mut state, mut output, mut plaster, mut rt_world, mut rt_updates } = self;
    let mut ids = Vec::new();
    ids.extend(requests.changed.read().map(|event| event.0));
    // 原重算事件在合并前先过滤缺失墙；OnWallChanged 则不做此过滤。
    ids.extend(requests.recalculate.read().map(|event| event.0).filter(|id| public.get(*id).is_some()));
    ids.extend(requests.gable.read().map(|event| event.0).filter(|id| public.get(*id).is_some()));
    ids.extend(requests.presence.read().map(|event| event.0).filter(|id| public.get(*id).is_some()));
    ids.extend(requests.geometry.read().map(|event| event.wall).filter(|id| public.get(*id).is_some()));
    for event in requests.deleted.read() {
        state.failures.remove(&event.0);
    }
    ids.sort_unstable_by_key(|id| id.0);
    ids.dedup();
    for id in ids {
        let wall = public.get(id).expect("原 construct_walls: changed wall missing from PublicWalls");
        // B 1407580f8..117：查得墙后、special 与网格构造之前发布。
        plaster.write(crate::resources::walls::plaster::RecalculatePlasterCmd(id));
        let roof = public
            .roof_entity(id)
            .and_then(|entity| roofs.get(entity).ok());
        let geometry = match prepare_wall_bricks(id, wall, roof.map(Roof::observed)) {
            Ok(prepared) => prepared,
            Err(error) => {
                state.failures.insert(id, error);
                continue;
            }
        };
        let mut instances = Vec::new();
        let local_holes = before_production.as_deref_mut().and_then(|prepare| {
            prepare(id, wall, roof, geometry.as_ref(), &mut holes, &mut instances)
        });
        if let Some(prepared) = geometry.as_ref() {
            let finalized = match local_holes.as_ref() {
                Some(local) => local.finalized_holes().map_err(WallBrickBuildError::Holes),
                None => finalized_wall_holes(id, &holes),
            };
            let result = finalized.and_then(|finalized| produce_prepared_wall(wall, prepared, finalized, &mut instances));
            if let Err(error) = result {
                state.failures.insert(id, error);
                continue;
            }
        }
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
}

pub fn construct_wall_bricks(params: WallBrickConstruction) {
    params.run_with(None);
}

use crate::render::extract::ComponentWrite;

#[cfg(test)]
mod plaster_request_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use crate::resources::walls::plaster::RecalculatePlasterCmd;

    fn world() -> World {
        let mut world = World::new();
        crate::systems::wall::registration::register_wall_cpu_resources(&mut world);
        world.init_resource::<Events<RecalculatePlasterCmd>>();
        world
    }

    #[derive(Resource, Default)]
    struct BeforeCommitTrace(Vec<WallId>);

    #[test]
    fn before_production_reuses_same_wall_pipeline_and_skips_unprepared_paths() {
        use crate::resources::{roofs::geometry::surface::Curve2,
            walls::{InnerWalls, WallState, publication_source::TerrainHeightGrid512,
                public_walls::{WallPublicationDirty, WallSpatialHash}}};
        fn observe(params: WallBrickConstruction, mut trace: ResMut<BeforeCommitTrace>) {
            params.run_with(Some(&mut |id, _: &PublicWallState, _: Option<&Roof>, geometry: Option<&WallBrickPreparedGeometry>, holes: &mut WallHoles, instances: &mut Vec<WallBrickInstance>| {
                assert!(geometry.is_none(), "special原分支必须跳过额外几何");
                trace.0.push(id);
                holes.storage.entry(id).or_insert_with(crate::resources::walls::wall_holes::WallHoleStorage::new_finalized);
                // 显式夹具记录，核实追加进入同一实例owner，而不是第二份构造输出。
                instances.push(WallBrickInstance { seed: 0x1357, ..Default::default() });
                None
            }));
        }
        let mut inner = InnerWalls::default();
        let curve = Curve2::try_new(vec![[0., 0.], [1., 0.]], false).unwrap();
        // special原分支产空砖后进入实例阶段；普通墙缺prepared path须在回调前失败。
        let special = inner.new_wall(WallState::new(curve.clone(), 2., 0, 1, false));
        let regular = inner.new_wall(WallState::new(curve, 2., 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: true,
        }, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        // 真实路径准备使special墙可继续完成trigger／实例提交；普通墙刻意保留缺失输入。
        let paths = crate::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
            std::array::from_fn(|_| std::sync::Arc::<[u8]>::from(vec![0; 512 * 512])), true).unwrap();
        public.get_mut(special).unwrap().prepare_paths(&paths).unwrap();
        let mut world = world();
        world.insert_resource(public);
        world.init_resource::<BeforeCommitTrace>();
        world.send_event(OnWallChanged(regular));
        world.send_event(OnWallChanged(special));
        world.send_event(OnWallChanged(special));
        world.run_system_once(observe).unwrap();
        assert_eq!(world.resource::<BeforeCommitTrace>().0, [special]);
        let entity = world.resource::<WallEntities>().wall_entity(special).unwrap();
        let instance = world.get::<InstancedWall<WallBrickInstance>>(entity).unwrap();
        assert_eq!(instance.pending_instances().len(), 1);
        assert_eq!(instance.pending_instances()[0].seed, 0x1357);
        assert!(world.resource::<WallHoles>().storage.get(&special).unwrap().finalized_holes().unwrap().is_empty());
        assert!(matches!(world.resource::<WallBrickBuildState>().failures.get(&regular),
            Some(WallBrickBuildError::MissingPreparedPath)));
        let events = world.resource::<Events<RecalculatePlasterCmd>>();
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn before_production_local_holes_cut_body_and_keep_extra_bricks_first() {
        use crate::resources::{roofs::geometry::surface::Curve2,
            walls::{InnerWalls, WallState, publication_source::TerrainHeightGrid512,
                public_walls::{WallPublicationDirty, WallSpatialHash},
                wall_holes::{RawWallHole, WallHoleStorage}}};
        fn with_local_cut(params: WallBrickConstruction) {
            params.run_with(Some(&mut |_, wall, _, prepared, _, instances| {
                assert!(prepared.is_some());
                instances.push(WallBrickInstance { seed: 0x2468, ..Default::default() });
                let mut local = WallHoleStorage::new_in_progress();
                // 明确局部夹具：覆盖全部主体，原全局洞口不变。
                local.add(RawWallHole { center: [1., 1.], dimensions: [4., 4.], kind: 2 }).unwrap();
                local.finalize(wall.wall_space_length(), false).unwrap();
                Some(local)
            }));
        }
        let mut inner = InnerWalls::default();
        let id = inner.new_wall(WallState::new(
            Curve2::try_new(vec![[0., 0.], [2., 0.]], false).unwrap(), 2., 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: true,
        }, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let paths = crate::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
            std::array::from_fn(|_| std::sync::Arc::<[u8]>::from(vec![0; 512 * 512])), true).unwrap();
        public.get_mut(id).unwrap().prepare_paths(&paths).unwrap();
        assert!(!build_wall_bricks(id, public.get(id).unwrap(), None, &WallHoles::default()).unwrap().is_empty());
        let mut world = world();
        world.insert_resource(public);
        world.send_event(OnWallChanged(id));
        world.run_system_once(with_local_cut).unwrap();
        let entity = world.resource::<WallEntities>().wall_entity(id).unwrap();
        let instances = world.get::<InstancedWall<WallBrickInstance>>(entity).unwrap().pending_instances();
        assert_eq!(instances.len(), 1, "主体必须使用前置回调的局部洞口，而非先生成主体");
        assert_eq!(instances[0].seed, 0x2468);
        assert!(world.resource::<WallHoles>().storage.is_empty());
    }

    #[test]
    fn missing_recalculation_is_filtered_but_missing_change_panics() {
        let mut world = world();
        world.send_event(RecalculateWallBricks(WallId(19)));
        world.run_system_once(construct_wall_bricks).unwrap();
        assert!(world.resource::<Events<RecalculatePlasterCmd>>().is_empty());

        world.send_event(OnWallChanged(WallId(19)));
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            world.run_system_once(construct_wall_bricks).unwrap();
        }));
        assert!(panic.is_err(), "原 changed 缺墙必须失败");
    }

    #[test]
    fn plaster_requests_are_sorted_deduplicated_and_precede_mesh_failure() {
        use crate::resources::{roofs::geometry::surface::Curve2, walls::{InnerWalls, WallState,
            public_walls::{WallPublicationDirty, WallSpatialHash},
            publication_source::TerrainHeightGrid512}};
        let mut inner = InnerWalls::default();
        let make_wall = || WallState::new(Curve2::try_new(vec![[0., 0.], [1., 0.]], false).unwrap(), 2., 2, 5, false);
        let a = inner.new_wall(make_wall());
        let b = inner.new_wall(make_wall());
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0., maximum_y: 0., sampling_fix: true };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        assert!(public.get(a).is_some() && public.get(b).is_some());
        let mut world = world();
        world.insert_resource(public);
        world.send_event(OnWallChanged(b));
        world.send_event(OnWallChanged(a));
        world.send_event(RecalculateWallBricks(b));
        world.run_system_once(construct_wall_bricks).unwrap();
        let events = world.resource::<Events<RecalculatePlasterCmd>>();
        let ids: Vec<_> = events.get_cursor().read(events).map(|event| event.0).collect();
        assert_eq!(ids, [a, b]);
        assert!(world.resource::<WallBrickBuildState>().failures.contains_key(&a));
        assert!(world.resource::<WallBrickBuildState>().failures.contains_key(&b));
    }
}
