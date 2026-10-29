use super::{
    DeletedAutoClutter, defered_autoclutter_history::RecordAutoClutterHistory,
    edit_autoclutter::DeleteAutoClutterCmd,
};
use crate::resources::history::building_transactions::RoofHistoryEdit;
use crate::resources::history::building_transactions::RoofReplayCommand;
use crate::resources::history::transactions::ObservedHistoryLedger;
use crate::resources::history::EditType;
use crate::resources::history::History;
use crate::resources::history::ReplayBucket;
use crate::resources::history::ReplayBuffer;
use crate::resources::history::ReplayCommand;
use crate::resources::history::auto_clutter_edit::AutoClutterEdit;
use crate::resources::history::auto_clutter_edit::AutoClutterReplayCmd;
use crate::resources::time::GameTime;
use crate::systems::history::*;
use crate::systems::mode_manager::HistoryCommand;
use bevy_ecs::prelude::*;
use std::sync::Arc;

fn empty_history() -> History<()> {
    History::<()>::from_observed(ObservedHistoryLedger {
        entries: Vec::new(),
        cursor: -1,
        undo_anchor: -1,
        resolved_snapshots: Vec::new(),
        observed_count_58: 0,
        mutation_nonce: 0,
    })
}

#[test]
fn autoclutter_delete_commit_undo_redo_use_one_live_owner() {
    let mut world = World::new();
    register_history_events(&mut world);
    world.insert_resource(empty_history());
    world.insert_resource(ReplayBuffer::new(2.0));
    world.insert_resource(GameTime {
        delta: 0.05,
        ..Default::default()
    });
    let mut schedule = Schedule::default();
    schedule.add_systems(
        (
            enqueue_history_commands,
            process_history_cursor_commands::<()>,
            replay_auto_clutter::replay_auto_clutter,
            super::edit_autoclutter::delete_auto_clutter::<()>,
            super::defered_autoclutter_history::defered_autoclutter_history::<()>,
            pop_empty_replay_bucket,
        )
            .chain(),
    );
    world
        .resource_mut::<History<()>>()
        .start_input_recording(0)
        .unwrap();
    for id in [5, 8] {
        world
            .resource_mut::<Events<DeleteAutoClutterCmd>>()
            .send(DeleteAutoClutterCmd(id));
    }
    schedule.run(&mut world);
    assert!(world.resource::<DeletedAutoClutter>().is_deleted(5));
    assert!(world.resource::<DeletedAutoClutter>().is_deleted(8));
    world
        .resource_mut::<History<()>>()
        .finish_input_recording()
        .unwrap();
    let history = world.resource::<History<()>>();
    assert_eq!(history.ledger().entries.len(), 1);
    let edits = &history.ledger().entries[0].edits;
    assert_eq!(edits.len(), 2);
    for (edit, id) in edits.iter().zip([5, 8]) {
        let EditType::AutoClutter(edit) = edit else {
            panic!("wrong history owner");
        };
        assert_eq!(**edit, AutoClutterEdit::Delete { id });
    }
    for (command, is_deleted, cursor) in [
        (HistoryCommand::UNDO, false, -1),
        (HistoryCommand::REDO, true, 0),
    ] {
        world.resource_mut::<Events<HistoryCommand>>().send(command);
        schedule.run(&mut world);
        // Application is at elapsed==0; completion waits for the strict bound.
        assert_eq!(
            world.resource::<DeletedAutoClutter>().is_deleted(5),
            is_deleted
        );
        assert_eq!(
            world.resource::<DeletedAutoClutter>().is_deleted(8),
            is_deleted
        );
        assert_eq!(world.resource::<History<()>>().ledger().cursor, cursor);
        assert!(!world.resource::<ReplayBuffer>().is_empty());
        schedule.run(&mut world); // old elapsed == 0.05: retain
        assert!(!world.resource::<ReplayBuffer>().is_empty());
        schedule.run(&mut world); // old elapsed == 0.1 > 0.05: remove
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }
    assert_eq!(world.resource::<History<()>>().ledger().entries.len(), 1);
    assert_eq!(
        world.resource::<Events<RecordAutoClutterHistory>>().len(),
        2
    );
    assert!(
        world
            .resource::<Events<recording::HistoryRecordingFailed>>()
            .is_empty()
    );
    assert!(world.resource::<Events<HistoryReplayFailed>>().is_empty());
}

#[test]
fn autoclutter_replay_preserves_native_timing_and_front_bucket() {
    use replay_auto_clutter::advance_auto_clutter;
    let edit = Arc::new(AutoClutterEdit::Delete { id: 42 });
    let mut deleted = DeletedAutoClutter::default();
    let mut command = AutoClutterReplayCmd::expanded_shared(edit.clone(), false, false);
    assert!(!advance_auto_clutter(&mut command, &mut deleted, 1.0, 0.0));
    deleted.0.remove(&42);
    assert!(!advance_auto_clutter(&mut command, &mut deleted, 1.0, 0.0));
    assert!(deleted.is_deleted(42)); // no synthetic started flag
    command.duration = f32::NAN;
    command.elapsed = 0.06;
    assert!(advance_auto_clutter(&mut command, &mut deleted, 1.0, 0.01));
    let mut instant = AutoClutterReplayCmd::expanded_shared(edit.clone(), true, true);
    assert!(advance_auto_clutter(&mut instant, &mut deleted, 1.0, 1.0));
    assert_eq!(instant.elapsed, 0.0);
    assert!(!deleted.is_deleted(42));

    let mut world = World::new();
    world.insert_resource(deleted);
    world.insert_resource(GameTime::default());
    let mut buffer = ReplayBuffer::new(1.0);
    buffer.push(ReplayBucket {
        entry_index: 0,
        undo: false,
        commands: vec![
            ReplayCommand::AutoClutter(AutoClutterReplayCmd::expanded_shared(
                edit.clone(),
                false,
                true,
            )),
            ReplayCommand::Roof(RoofReplayCommand::expanded_shared(
                Arc::new(RoofHistoryEdit::direction(1, 0, 1)),
                false,
                true,
            )),
        ],
        timing: vec![Default::default(); 2],
    });
    buffer.push(ReplayBucket {
        entry_index: 1,
        undo: true,
        commands: vec![ReplayCommand::AutoClutter(
            AutoClutterReplayCmd::expanded_shared(edit, true, true),
        )],
        timing: vec![Default::default(); 1],
    });
    world.insert_resource(buffer);
    let mut schedule = Schedule::default();
    schedule.add_systems(replay_auto_clutter::replay_auto_clutter);
    schedule.run(&mut world);
    let buffer = world.resource::<ReplayBuffer>();
    assert_eq!(buffer.buckets[0].commands.len(), 1);
    assert!(matches!(
        buffer.buckets[0].commands[0],
        ReplayCommand::Roof(_)
    ));
    assert_eq!(buffer.buckets[1].commands.len(), 1);
    assert!(world.resource::<DeletedAutoClutter>().is_deleted(42));
}

#[test]
fn debug_spawn_single_uses_payload_rotation_and_batch_uses_zero() {
    use super::debug_spawn_clutter::{
        DebugSpawnAllClutterCmd, DebugSpawnClutterCmd, SpawnAutoClutter, debug_spawn_all_clutter,
    };
    use super::registration::register_ui_clutter_resources;
    use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
    use crate::systems::mode_manager::ClutterMeshTy;

    let mut world = World::new();
    register_ui_clutter_resources(&mut world);
    world.insert_resource(empty_history());
    world.insert_resource(crate::systems::mode_manager::AppMode::default());
    world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
    world
        .resource_mut::<Events<DebugSpawnClutterCmd>>()
        .send(DebugSpawnClutterCmd {
            ty: ClutterMeshTy::PlantPotV1,
            position: glam::Vec3::new(1.0, 2.0, 3.0),
            rotation: 1.5,
        });
    let mut schedule = Schedule::default();
    schedule.add_systems(debug_spawn_all_clutter::<()>);
    schedule.run(&mut world);
    let mut events = world.resource_mut::<Events<SpawnAutoClutter>>();
    let spawned: Vec<_> = events.drain().collect();
    assert_eq!(spawned.len(), 1);
    assert_eq!(spawned[0].rotation, 1.5);
    assert_eq!(spawned[0].position, glam::Vec3::new(1.0, 2.0, 3.0));
    // 批量：42 条，rotation=0。
    world
        .resource_mut::<Events<DebugSpawnAllClutterCmd>>()
        .send(DebugSpawnAllClutterCmd);
    schedule.run(&mut world);
    let mut events = world.resource_mut::<Events<SpawnAutoClutter>>();
    let spawned: Vec<_> = events.drain().collect();
    assert_eq!(spawned.len(), 42);
    assert!(spawned.iter().all(|event| event.rotation == 0.0));
}

#[test]
fn focused_requests_preserve_original_native_producer_fields() {
    use super::ui_focus_decoclutter::{request_focused_decorator_move, request_focused_decorator_rotation};
    use crate::resources::walls::{decorator::{DecoratorId, DecoratorInfo}, decorator_storage::{DecoratorStorage, DecoratorStorageAddr}, WallId};
    use crate::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
    let id = DecoratorId(0x87654321);
    let widget = WidgetId(0x1122334455667788);
    for (addr, expected) in [(DecoratorStorageAddr::Wall(WallId(0xabcdef)), [0,0xabcdef]), (DecoratorStorageAddr::Terrain, [1,0]), (DecoratorStorageAddr::Stair, [2,0])] {
        let mut storage = DecoratorStorage::default();
        storage.get_or_insert_default(addr).insert(id, DecoratorInfo { ty: 0x3e, ..Default::default() });
        storage.decorator_to_storage.insert(id, addr);
        let mut ui = UiState::new(UiInteraction { native_tag: 2, native_phase: 3, widget });
        ui.set_intersection_tag(0);
        ui.set_intersection_point(glam::Vec3::new(11.25,7.5,-2.75));
        let rotate = request_focused_decorator_rotation(&ui,widget,id,&storage,1.25,4.75).unwrap();
        let moving = request_focused_decorator_move(&ui,widget,id,&storage).unwrap();
        assert_eq!(rotate.widget,widget);
        assert_eq!(rotate.decorator_id,id);
        assert_eq!(rotate.amount.to_bits(),4.0f32.to_bits());
        assert_eq!(rotate.ty.0,0x3e);
        assert_eq!(rotate.hit.handle,[0x8765432100000004,expected[0],expected[1]]);
        assert_eq!(rotate.hit.position,glam::Vec3::new(11.25,7.5,-2.75));
        assert_eq!(rotate.hit.normal,glam::Vec3::Y);
        assert_eq!(rotate.hit.toi.to_bits(),0);
        assert_eq!(moving.hit,rotate.hit);
        assert!(request_focused_decorator_move(&ui,WidgetId(123),id,&storage).is_none());
        ui.set_intersection_tag(2);
        assert!(request_focused_decorator_rotation(&ui,widget,id,&storage,1.25,4.75).is_none());
    }
}
