//! Resolved decorator-input adapter backed by real Bevy entities. Original
//! wall-space placement and asset collision metadata are explicit inputs;
//! these component types do not claim the original visual Bundle/renderer ABI.
use crate::{BuildingReport, PublicWalls, RecalculateWallHoles, WallGableDirty};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred};
use std::collections::{HashMap, HashSet};
use tiny_glade_reconstruction::{
    building_opening_rules::{DecoratorOpeningInput, generate_wall_opening},
    building_wall_hole_invalidation::forward_recalculation,
    building_wall_holes::{PreviousWallHoles, RawWallHole, WallHoleStorage},
    building_wall_state::WallId,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpeningId(pub u64);
#[derive(Component, Clone, Copy, Debug)]
pub struct OpeningAuthority {
    pub id: OpeningId,
    pub wall: WallId,
    pub input: DecoratorOpeningInput,
}
#[derive(Component, Clone, Debug, Default)]
pub struct OpeningState {
    pub raw_holes: Vec<RawWallHole>,
    pub resolved_host: Option<WallId>,
    pub generation: u64,
    pub host_publication_revision: u64,
}
#[derive(Event, Clone, Copy, Debug)]
pub struct UpsertOpening(pub OpeningAuthority);
#[derive(Event, Clone, Copy, Debug)]
pub struct RemoveOpening(pub OpeningId);
/// Drop a derived projection while retaining its explicit world-space source.
#[derive(Event, Clone, Copy, Debug)]
pub struct WithdrawOpening(pub OpeningId);
#[derive(Event, Clone, Copy, Debug)]
pub struct OnWallChanged(pub WallId);
#[derive(Resource, Default)]
pub struct OpeningEntities(pub HashMap<OpeningId, Entity>);
#[derive(Resource, Default)]
pub struct WallOpenings {
    pub storage: HashMap<WallId, WallHoleStorage>,
    pub previous: HashMap<WallId, PreviousWallHoles>,
    pub geometry_rebuilt: Vec<WallId>,
    pub forwarded_recalculations: Vec<WallId>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<OpeningEntities>()
        .init_resource::<WallOpenings>()
        .add_event::<UpsertOpening>()
        .add_event::<RemoveOpening>()
        .add_event::<WithdrawOpening>()
        .add_event::<OnWallChanged>();
    app.world_mut().register_component::<OpeningAuthority>();
    app.world_mut().register_component::<OpeningState>();
    app.add_systems(
        bevy_app::Update,
        (
            consume_inputs,
            ApplyDeferred,
            rebuild_openings,
            gather_and_finalize,
        )
            .chain()
            .in_set(crate::BuildingStage::Openings),
    );
}
fn consume_inputs(
    mut commands: Commands,
    mut inputs: EventReader<UpsertOpening>,
    mut removals: EventReader<RemoveOpening>,
    mut withdrawals: EventReader<WithdrawOpening>,
    mut index: ResMut<OpeningEntities>,
    entities: Query<(), With<OpeningAuthority>>,
) {
    let mut pending = HashSet::new();
    for event in inputs.read() {
        if let Some(entity) = index
            .0
            .get(&event.0.id)
            .copied()
            .filter(|e| entities.contains(*e) || pending.contains(e))
        {
            commands.entity(entity).insert(event.0);
        } else {
            let entity = commands.spawn((event.0, OpeningState::default())).id();
            index.0.insert(event.0.id, entity);
            pending.insert(entity);
        }
    }
    for id in removals
        .read()
        .map(|event| event.0)
        .chain(withdrawals.read().map(|event| event.0))
    {
        if let Some(entity) = index.0.remove(&id) {
            commands.entity(entity).despawn();
        }
    }
}
fn rebuild_openings(
    mut recalculate: EventReader<RecalculateWallHoles>,
    mut gable: EventReader<WallGableDirty>,
    mut changed: EventWriter<OnWallChanged>,
    walls: Res<PublicWalls>,
    mut state: ResMut<WallOpenings>,
    mut openings: Query<(Ref<OpeningAuthority>, &mut OpeningState)>,
    mut report: ResMut<BuildingReport>,
) {
    state.forwarded_recalculations.clear();
    let mut dirty = HashSet::new();
    for wall in gable
        .read()
        .map(|event| event.0)
        .chain(recalculate.read().map(|event| event.0))
    {
        if let Some(id) = forward_recalculation(wall, walls.walls.contains_key(&wall)) {
            changed.write(OnWallChanged(id));
            state.forwarded_recalculations.push(id);
            dirty.insert(id);
        }
    }
    for (source, mut derived) in &mut openings {
        let Some(host) = walls.walls.get(&source.wall) else {
            if derived.resolved_host.is_some() {
                derived.raw_holes.clear();
                derived.resolved_host = None;
                derived.generation += 1;
            }
            continue;
        };
        // A successful explicit input replacement and host republish invalidate the
        // projection; notification-only roof edits also reach this actual producer.
        let needs_rebuild = derived.resolved_host != Some(source.wall)
            || derived.host_publication_revision != host.publication_revision
            || dirty.contains(&source.wall)
            || source.is_changed();
        if needs_rebuild {
            match generate_wall_opening(source.input, &host.source.shape) {
                Ok(raw) => {
                    derived.raw_holes = raw;
                    derived.resolved_host = Some(source.wall);
                    derived.host_publication_revision = host.publication_revision;
                    derived.generation += 1;
                }
                Err(error) => {
                    derived.raw_holes.clear();
                    derived.resolved_host = None;
                    report
                        .faults
                        .push(format!("Opening {}: {error:?}", source.id.0));
                }
            }
        }
    }
}
fn gather_and_finalize(
    walls: Res<PublicWalls>,
    openings: Query<&OpeningState>,
    stairs: Res<crate::wall_platform_stairs::WallPlatformStairState>,
    mut state: ResMut<WallOpenings>,
    mut report: ResMut<BuildingReport>,
) {
    state.previous.clear();
    state.geometry_rebuilt.clear();
    state
        .storage
        .retain(|wall, _| walls.walls.contains_key(wall));
    let mut previous = Vec::new();
    for (wall, storage) in &mut state.storage {
        if let Some(snapshot) = storage.advance_frame() {
            previous.push((*wall, snapshot));
        }
    }
    state.previous.extend(previous);
    for opening in &openings {
        if let Some(wall) = opening.resolved_host {
            let storage = state.storage.entry(wall).or_default();
            for hole in &opening.raw_holes {
                if let Err(error) = storage.add(*hole) {
                    report
                        .faults
                        .push(format!("Wall holes {}: {error:?}", wall.0));
                }
            }
        }
    }
    for (wall, holes) in &stairs.holes {
        if walls.walls.contains_key(wall) {
            let storage = state.storage.entry(*wall).or_default();
            for hole in holes {
                if let Err(error) = storage.add(*hole) {
                    report
                        .faults
                        .push(format!("Stair holes {}: {error:?}", wall.0));
                }
            }
        }
    }
    let mut rebuilt = Vec::new();
    for (id, storage) in &mut state.storage {
        let host = &walls.walls[id];
        // Original wall-constructor passes source length+28c and shape tag !=2.
        // Asset lookup and wall-space placement remain explicit input boundaries.
        let closed = host.source.shape.tag() != 2;
        match storage.finalize(host.source.source_curve_length, closed) {
            Ok(true) => rebuilt.push(*id),
            Ok(false) => {}
            Err(error) => report
                .faults
                .push(format!("Finalize holes {}: {error:?}", id.0)),
        }
    }
    state.geometry_rebuilt = rebuilt;
}
