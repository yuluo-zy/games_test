use super::{
    DeletedAutoClutter, defered_autoclutter_history::RecordAutoClutterHistory,
    edit_autoclutter::DeleteAutoClutterCmd,
};
use crate::{
    recovered_building::{
        building_history_transactions::{RoofHistoryEdit, RoofReplayCommand},
        history_transactions::ObservedHistoryLedger,
    },
    resources::{
        history::{
            EditType, History, ReplayBucket, ReplayBuffer, ReplayCommand,
            auto_clutter_edit::{AutoClutterEdit, AutoClutterReplayCmd},
        },
        time::GameTime,
    },
    systems::{history::*, mode_manager::HistoryCommand},
};
use bevy_ecs::prelude::*;
use std::sync::Arc;

fn empty_history() -> History {
    History::from_observed(ObservedHistoryLedger {
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
        .resource_mut::<History>()
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
        .resource_mut::<History>()
        .finish_input_recording()
        .unwrap();
    let history = world.resource::<History>();
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
        assert_eq!(world.resource::<History>().ledger().cursor, cursor);
        assert!(!world.resource::<ReplayBuffer>().is_empty());
        schedule.run(&mut world); // old elapsed == 0.05: retain
        assert!(!world.resource::<ReplayBuffer>().is_empty());
        schedule.run(&mut world); // old elapsed == 0.1 > 0.05: remove
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }
    assert_eq!(world.resource::<History>().ledger().entries.len(), 1);
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
    });
    buffer.push(ReplayBucket {
        entry_index: 1,
        undo: true,
        commands: vec![ReplayCommand::AutoClutter(
            AutoClutterReplayCmd::expanded_shared(edit, true, true),
        )],
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
