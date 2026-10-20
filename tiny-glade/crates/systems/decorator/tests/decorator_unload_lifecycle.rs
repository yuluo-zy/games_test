//! Tests the scoped canonical-owner consumer, not a full game unload verdict.
use std::{
    ops::Range,
    sync::{Arc, atomic::Ordering},
};

use bevy_ecs::{prelude::*, system::SystemState};
use country_core::{
    render::{
        sparse_instance_buffer::SparseInstanceBuffer,
        ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice},
    },
    resources::{
        roof_tile_ssbo::RoofTileSsbo,
        roofs::dormer_tiles::DormerTiles,
        stairs::{
            StairColliderHandle, StairGraphEdge, StairsState, StairsStructuralGraph,
            StairsVisualState, stairs_top_down::StairsTopDownMesh,
        },
        walls::{
            WallId,
            decorator::{DecoratorId, DecoratorIdGen, DecoratorType},
            decorator_archivist::LiveDecoratorArchivist,
            decorator_collision::DecoratorPhysicsColliders,
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr, Decorators},
            wall_stone_columns::{StoneColumn, StoneColumnCollisionStorage},
        },
    },
    rhapsody_contract::ManagedHandle,
    systems::audio::{PlayDecoratorSoundCmd, PlayStairsAudioCmd},
};
use system_decorator::{
    audio::{AudioEventKey, DecoratorAudioEvents},
    blueprint::{DecoratorBlueprint, DecoratorBlueprints, PrevDecoratorBlueprints},
    decorator_visual::deco_differential_state::{
        ChimneyAssemblyParams, DoorStairsAssemblyParams, WallAttachedDecoTracker,
    },
    decorator_visual_state::{DecoratorVisualState, PersistentVisualEntities},
    deferred_decorator_history::{DeferredDecoratorEditHistory, DeferredDecoratorRecord},
    lifecycle::{
        ExistingDecoratorUnloadOwners, reset_scoped_existing_decorator_owners_on_glade_unload,
    },
    ui_systems::{
        ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher,
        ui_place_decorator::PlaceDecoratorInteractionState,
    },
    window_lights::{GlobalWindowLightsControl, WindowLightState},
};

fn world() -> World {
    let mut world = World::new();
    world.init_resource::<DecoratorStorage>();
    world.init_resource::<DecoratorBlueprints>();
    world.init_resource::<PrevDecoratorBlueprints>();
    world.init_resource::<LiveDecoratorArchivist>();
    world.init_resource::<DeferredDecoratorEditHistory>();
    world.init_resource::<DecoratorVisualState>();
    world.init_resource::<DecoratorIdGen>();
    world.init_resource::<PlaceDecoratorInteractionState>();
    world.init_resource::<DecoratorAudioEvents>();
    world.init_resource::<GlobalWindowLightsControl>();
    world.init_resource::<DecoratorAffordanceDispatcher>();
    world.init_resource::<DecoratorPhysicsColliders>();
    world.init_resource::<StoneColumnCollisionStorage>();
    world.init_resource::<DormerTiles>();
    world.init_resource::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
    world.init_resource::<StairsState>();
    world.init_resource::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>();
    world.init_resource::<StairsVisualState>();
    world.init_resource::<StairsStructuralGraph>();
    world.insert_resource(StairsTopDownMesh {
        buffer: ManagedHandle(73),
        vertex_count: 6,
    });
    world
}

#[test]
fn populated_owners_reset_without_despawning_or_sending_audio_and_remove_mesh_is_deferred() {
    let mut world = world();
    let entity = world.spawn_empty().id();
    let mut storage = world.resource_mut::<DecoratorStorage>();
    storage
        .per_wall
        .entry(WallId(0x1_0000_0007))
        .or_default()
        .removed_ids
        .insert(4);
    storage.terrain = Some(Decorators::default());
    storage.stairs = Some(Decorators::default());
    storage
        .decorator_to_storage
        .insert(DecoratorId(9), DecoratorStorageAddr::Terrain);
    storage
        .removed
        .entry(DecoratorStorageAddr::Terrain)
        .or_default()
        .insert(9);
    storage.mark_immutable();
    let shared = Arc::new(DecoratorBlueprint::new());
    world
        .resource_mut::<DecoratorBlueprints>()
        .map
        .insert(9, shared.clone());
    world
        .resource_mut::<PrevDecoratorBlueprints>()
        .map
        .insert(9, shared.clone());
    assert_eq!(Arc::strong_count(&shared), 3);
    world
        .resource_mut::<LiveDecoratorArchivist>()
        .decorator_cull_allow_any();
    world
        .resource_mut::<LiveDecoratorArchivist>()
        .culling
        .allowed_ids
        .insert(9);
    let mut history = world.resource_mut::<DeferredDecoratorEditHistory>();
    history.records.push(DeferredDecoratorRecord {
        kind: 4,
        f1: 9,
        f2: 13,
        f3: 23,
    });
    history.pending = true;
    let mut visual = world.resource_mut::<DecoratorVisualState>();
    visual.persistent.entries.insert(
        9,
        PersistentVisualEntities {
            sealed: 1,
            entities: vec![entity],
        },
    );
    visual.transient.entities.insert(10, vec![entity]);
    world
        .resource_mut::<DecoratorIdGen>()
        .reserve(DecoratorId(97));
    world.resource_mut::<PlaceDecoratorInteractionState>().id = Some(DecoratorId(9));
    let key = AudioEventKey {
        decorator_type: DecoratorType(1),
        flag: 2,
    };
    let mut audio = world.resource_mut::<DecoratorAudioEvents>();
    audio.play_sound(key, PlayDecoratorSoundCmd::at(3, glam::Vec3::ONE));
    audio.play_stairs(
        key,
        PlayStairsAudioCmd {
            kind: 4,
            position: glam::Vec3::ONE,
            values: [1.; 4],
            tail: [2.; 2],
            bytes: [3; 3],
        },
    );
    audio.set_state(8, 9);
    // Original unload has no audio EventWriter: existing global events survive.
    world.init_resource::<Events<PlayDecoratorSoundCmd>>();
    world
        .resource_mut::<Events<PlayDecoratorSoundCmd>>()
        .send(PlayDecoratorSoundCmd::at(8, glam::Vec3::ZERO));
    let mut lights = world.resource_mut::<GlobalWindowLightsControl>();
    lights.lights.insert(5, WindowLightState::default());
    lights.lights_on = true;
    let mut affordance = world.resource_mut::<DecoratorAffordanceDispatcher>();
    *affordance = DecoratorAffordanceDispatcher {
        state: 11,
        payload: 0x3f,
        ty: 0x3f,
        interaction: None,
    };
    world
        .resource_mut::<StoneColumnCollisionStorage>()
        .clear_and_get_mut(WallId(7))
        .push(StoneColumn { handle: 19 });
    let mut stairs = world.resource_mut::<StairsState>();
    stairs.add_edge(1, 2);
    stairs.make_backup();
    world
        .resource_mut::<StairsVisualState>()
        .reset_edge(StairGraphEdge::new(1, 2));
    world.resource_mut::<StairsStructuralGraph>().upsert_span(
        StairGraphEdge::new(1, 2),
        StairColliderHandle::new(3, 4, 5),
        2.,
    );

    let mut state = SystemState::<ExistingDecoratorUnloadOwners>::new(&mut world);
    reset_scoped_existing_decorator_owners_on_glade_unload(state.get_mut(&mut world));
    assert!(
        world.contains_resource::<StairsTopDownMesh>(),
        "command must stay queued"
    );
    assert!(
        world.get_entity(entity).is_ok(),
        "visual ID tables do not own ECS destruction"
    );
    assert_eq!(world.resource::<Events<PlayDecoratorSoundCmd>>().len(), 1);
    let storage = world.resource::<DecoratorStorage>();
    assert!(storage.per_wall.is_empty() && storage.removed.is_empty());
    assert!(storage.decorator_to_storage.is_empty());
    assert!(storage.terrain.is_none() && storage.stairs.is_none() && !storage.immutable);
    assert_eq!(
        storage.per_wall.capacity(),
        0,
        "release old keyed-owner capacity"
    );
    assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
    assert!(world.resource::<PrevDecoratorBlueprints>().map.is_empty());
    assert_eq!(Arc::strong_count(&shared), 1);
    assert!(
        !world
            .resource::<LiveDecoratorArchivist>()
            .is_decorator_cull_allowed(9)
    );
    assert!(
        world
            .resource::<LiveDecoratorArchivist>()
            .culling
            .allowed_ids
            .is_empty()
    );
    let history = world.resource::<DeferredDecoratorEditHistory>();
    assert!(!history.pending && history.records.is_empty());
    assert!(
        world
            .resource::<DecoratorVisualState>()
            .persistent
            .entries
            .is_empty()
    );
    assert!(
        world
            .resource::<DecoratorVisualState>()
            .transient
            .entities
            .is_empty()
    );
    assert_eq!(
        world.resource_mut::<DecoratorIdGen>().generate(),
        DecoratorId(0)
    );
    assert!(
        world
            .resource::<PlaceDecoratorInteractionState>()
            .id
            .is_none()
    );
    assert_eq!(world.resource::<DecoratorAudioEvents>().sounds().count(), 0);
    assert_eq!(
        world
            .resource::<DecoratorAudioEvents>()
            .stair_sounds()
            .count(),
        0
    );
    assert_eq!(world.resource::<DecoratorAudioEvents>().state(), [0; 4]);
    assert!(
        world
            .resource::<GlobalWindowLightsControl>()
            .lights
            .is_empty()
    );
    assert!(!world.resource::<GlobalWindowLightsControl>().lights_on);
    assert_eq!(
        *world.resource::<DecoratorAffordanceDispatcher>(),
        DecoratorAffordanceDispatcher {
            state: 6,
            payload: 0,
            ty: 0x3f,
            interaction: None,
        }
    );
    assert!(
        world
            .resource::<StoneColumnCollisionStorage>()
            .columns
            .is_empty()
    );
    assert!(
        world
            .resource::<StoneColumnCollisionStorage>()
            .cleared
            .is_empty()
    );
    let stairs = world.resource::<StairsState>();
    assert_eq!(stairs.graph().node_count(), 0);
    assert!(!stairs.is_backed_up() && stairs.endpoint().is_unset());
    assert_eq!(
        world
            .resource_mut::<StairsState>()
            .drain_dirty_edges()
            .count(),
        0
    );
    assert!(world.resource::<StairsVisualState>().edges().is_empty());
    assert!(!world.resource::<StairsVisualState>().is_dirty());
    assert_eq!(
        world
            .resource::<StairsStructuralGraph>()
            .graph()
            .node_count(),
        0
    );
    state.apply(&mut world);
    assert!(!world.contains_resource::<StairsTopDownMesh>());
    assert_eq!(shared.u50, 0, "outside Arc owner remains valid");
}

fn populate_tracker<P: Clone>(tracker: &mut WallAttachedDecoTracker<P>, params: &P) {
    let wall = DecoratorStorageAddr::Wall(WallId(7));
    for id in 1..=3 {
        tracker.add(wall, DecoratorId(id), params);
    }
    tracker.with_diffs(|_, log| log.with_each_deco_diff(|_, _| {}).with_removed(|_, _| {}));
    tracker.remove(wall, DecoratorId(2));
    tracker.add(wall, DecoratorId(2), params); // Changed
    tracker.remove(wall, DecoratorId(3)); // Removed
    tracker.add(wall, DecoratorId(4), params); // Added alongside unchanged Current
}

#[test]
fn both_strict_trackers_drop_all_variants_and_allow_the_same_ids_in_the_next_session() {
    let mut world = world();
    let chimney = ChimneyAssemblyParams { words: [0; 10] };
    let door = DoorStairsAssemblyParams { words: [0; 8] };
    populate_tracker(
        &mut world.resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>(),
        &chimney,
    );
    populate_tracker(
        &mut world.resource_mut::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>(),
        &door,
    );
    let mut state = SystemState::<ExistingDecoratorUnloadOwners>::new(&mut world);
    reset_scoped_existing_decorator_owners_on_glade_unload(state.get_mut(&mut world));
    state.apply(&mut world);
    let mut chimney_tracker =
        world.resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
    chimney_tracker.with_diffs(|_, _| panic!("old session dirty walls survived"));
    populate_tracker(&mut chimney_tracker, &chimney);
    let mut door_tracker =
        world.resource_mut::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>();
    door_tracker.with_diffs(|_, _| panic!("old session dirty walls survived"));
    populate_tracker(&mut door_tracker, &door);
    reset_scoped_existing_decorator_owners_on_glade_unload(state.get_mut(&mut world));
    state.apply(&mut world);
}

struct CpuDevice;
impl GraphicsDevice for CpuDevice {
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        _: &str,
        _: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        Ok(Buffer {
            id: 1,
            byte_size: desc.size as u64,
            ..Buffer::default()
        })
    }
    fn update_buffer_bytes(
        &self,
        _: &Buffer,
        _: &[u8],
        _: &[Range<usize>],
        _: usize,
    ) -> Result<(), BackendError> {
        Ok(())
    }
}

#[test]
fn dormer_allocation_drop_retires_to_its_same_buffer_then_reuses_capacity_across_sessions() {
    let mut world = world();
    let mut buffer = SparseInstanceBuffer::<RoofTileSsbo>::new_storage_buffer(&CpuDevice, None);
    let capacity = buffer.free_chunks.read().len();
    for session in 0..2 {
        let allocation = buffer.new_allocation();
        buffer
            .writer(&allocation)
            .push(&RoofTileSsbo {
                roof_id: session,
                ..RoofTileSsbo::default()
            })
            .unwrap();
        world
            .resource_mut::<DormerTiles>()
            .allocations
            .insert(WallId(7), allocation);
        assert_eq!(buffer.free_chunks.read().len(), capacity - 1);
        let mut state = SystemState::<ExistingDecoratorUnloadOwners>::new(&mut world);
        reset_scoped_existing_decorator_owners_on_glade_unload(state.get_mut(&mut world));
        assert_eq!(buffer.waste.lock().len(), 1);
        assert_eq!(
            buffer.byte_map[0], 1,
            "drop queues, it does not drain waste"
        );
        assert_ne!(buffer.active_bitset[0].load(Ordering::Relaxed) & 1, 0);
        state.apply(&mut world);
        buffer.take_out_waste();
        assert!(buffer.waste.lock().is_empty());
        assert_eq!(buffer.byte_map[0], 0);
        assert_eq!(buffer.active_bitset[0].load(Ordering::Relaxed) & 1, 0);
        assert_eq!(buffer.free_chunks.read().len(), capacity);
        buffer.take_out_waste();
        assert_eq!(
            buffer.free_chunks.read().len(),
            capacity,
            "no duplicate retirement"
        );
    }
}

#[test]
fn absent_required_owner_fails_before_any_reset_or_final_command() {
    let mut world = world();
    world
        .resource_mut::<DecoratorIdGen>()
        .reserve(DecoratorId(55));
    world.remove_resource::<StairsVisualState>();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut state = SystemState::<ExistingDecoratorUnloadOwners>::new(&mut world);
        reset_scoped_existing_decorator_owners_on_glade_unload(state.get_mut(&mut world));
        state.apply(&mut world);
    }));
    assert!(result.is_err());
    assert_eq!(
        world.resource_mut::<DecoratorIdGen>().generate(),
        DecoratorId(56)
    );
    assert!(world.contains_resource::<StairsTopDownMesh>());
}
