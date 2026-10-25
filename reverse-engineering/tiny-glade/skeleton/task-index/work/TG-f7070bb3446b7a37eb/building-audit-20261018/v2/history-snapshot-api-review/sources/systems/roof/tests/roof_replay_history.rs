//! 屋顶回放/平顶檐历史链集成测试（自 country-core history tests 迁入：
//! replay_roof/route_roof_replay/set_flat_roof_rim_style 落位 system-roof）。
use country_core::resources::history::transactions::HistoryCursorStep;
use country_core::resources::history::transactions::HistoryEntry;
use country_core::resources::history::transactions::ObservedHistoryLedger;
use country_core::resources::roofs::customization::FlatRoofRimRequest;
use country_core::resources::walls::publication_fields::WallId as ObservedWallId;
use country_core::resources::history::EditType;
use country_core::resources::roofs::Roof;
use country_core::resources::roofs::commands::*;
use country_core::resources::time::GameTime;
use country_core::resources::walls::WallId;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::history::HistoryCpuStage;
use country_core::systems::history::HistoryReplayFailed;
use country_core::systems::history::enqueue_history_commands;
use country_core::systems::history::pop_empty_replay_bucket;
use country_core::systems::history::process_history_cursor_commands;
use country_core::systems::history::recording;
use country_core::systems::mode_manager::HistoryCommand;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;
use country_core::resources::history::{History, ReplayBucket, ReplayBuffer};

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
fn live_rim_record_undo_redo_share_one_history_and_do_not_rerecord() {
    let mut world = World::new();
    country_core::systems::history::register_history_events(&mut world);
    world.init_resource::<bevy_ecs::event::Events<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>>();
    world.insert_resource(empty_history());
    world.insert_resource(ReplayBuffer::new(1.0));
    world.insert_resource(GameTime {
        delta: 0.25,
        ..Default::default()
    });
    let id = WallId(3);
    // Explicit flat-roof fixture: circle with radius 2, height 0, rim 0.
    let mut bytes = [0; 88];
    bytes[..8].copy_from_slice(&id.0.to_le_bytes());
    bytes[0x14..0x18].copy_from_slice(&2.0f32.to_le_bytes());
    let entity = world.spawn(Roof::from_observed(bytes)).id();
    let mut walls = PublicWalls::default();
    walls.associate_roof(id, entity);
    world.insert_resource(walls);
    let mut replay = Schedule::default();
    replay.add_systems(
        (
            enqueue_history_commands,
            process_history_cursor_commands::<()>,
            system_roof::replay_roof::replay_roof,
            system_roof::edit_roof::set_flat_roof_rim_style,
            recording::record_roof_history::<()>,
            pop_empty_replay_bucket,
        )
            .chain(),
    );
    let _ = HistoryCpuStage::Replay;
    world
        .resource_mut::<History>()
        .start_input_recording(0)
        .unwrap();
    world
        .resource_mut::<Events<SetFlatRoofRimStyleCmd>>()
        .send(SetFlatRoofRimStyleCmd(FlatRoofRimRequest {
            wall_id: ObservedWallId(id.0),
            from: 0,
            to: 2,
            observed_record_history: 1,
        }));
    replay.run(&mut world);
    assert_eq!(world.get::<Roof>(entity).unwrap().observed()[0x3d], 2);
    world
        .resource_mut::<History>()
        .finish_input_recording()
        .unwrap();
    assert_eq!(world.resource::<History>().ledger().entries.len(), 1);
    for (command, expected, cursor) in [(HistoryCommand::UNDO, 0, -1), (HistoryCommand::REDO, 2, 0)]
    {
        world.resource_mut::<Events<HistoryCommand>>().send(command);
        replay.run(&mut world);
        assert_eq!(
            world.get::<Roof>(entity).unwrap().observed()[0x3d],
            expected
        );
        assert_eq!(world.resource::<History>().ledger().cursor, cursor);
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }
    assert_eq!(world.resource::<History>().ledger().entries.len(), 1);
    // Only the original live edit emitted a history record.
    assert_eq!(world.resource::<Events<RecordRoofHistory>>().len(), 1);
    assert!(world.resource::<Events<HistoryReplayFailed>>().is_empty());
    assert!(
        world
            .resource::<Events<recording::HistoryRecordingFailed>>()
            .is_empty()
    );
}

#[test]
fn unresolved_auxiliary_owner_does_not_advance_cursor() {
    let mut entry = HistoryEntry::<EditType>::from_edits(Vec::new());
    entry.observed_auxiliary_counts = [1, 0, 0];
    let mut history: History = History::from_observed(ObservedHistoryLedger {
        entries: vec![entry],
        cursor: 0,
        undo_anchor: -1,
        resolved_snapshots: Vec::new(),
        observed_count_58: 0,
        mutation_nonce: 0,
    });
    assert!(
        history
            .cursor_step_expanding(HistoryCursorStep::Undo, false, ReplayBucket::expand)
            .is_err()
    );
    assert_eq!(history.ledger().cursor, 0);
}
