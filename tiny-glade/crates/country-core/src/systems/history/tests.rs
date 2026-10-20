use super::*;
use crate::resources::history::EditType;
use crate::resources::history::transactions::HistoryEntry;
use crate::resources::history::transactions::ObservedHistoryLedger;
use crate::resources::roofs::Roof;
use crate::resources::roofs::commands::*;
use crate::resources::roofs::customization::FlatRoofRimRequest;
use crate::resources::time::GameTime;
use crate::resources::walls::WallId;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::publication_fields::WallId as ObservedWallId;

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
fn auxiliary_owner_counts_travel_with_bucket_and_advance_cursor() {
    // fresh replay_edit 0x140af6170：aux 计数非零不再拒绝；History +0x18/+0x20
    // 两 Arc 随桶携带（ReplayBucketAux），由 replay_pop_bucket 恢复。
    let mut entry = HistoryEntry::<EditType>::from_edits(Vec::new());
    entry.observed_auxiliary_counts = [1, 0, 0];
    let mut history: History<()> = History::<()>::from_observed(ObservedHistoryLedger {
        entries: vec![entry],
        cursor: 0,
        undo_anchor: -1,
        resolved_snapshots: Vec::new(),
        observed_count_58: 0,
        mutation_nonce: 0,
    });
    let bucket = history
        .cursor_step_expanding(HistoryCursorStep::Undo, false, ReplayBucket::expand)
        .expect("aux owner diffs no longer block expansion")
        .expect("entry exists");
    assert!(bucket.commands.is_empty());
    assert_eq!(history.ledger().cursor, -1);
}

#[test]
fn captured_event_records_dispatch_the_same_entry_effects() {
    let mut world = World::new();
    world.init_resource::<PendingHistoryCommands>();
    world.init_resource::<PendingHistoryControl>();
    world.init_resource::<Events<HistoryControlProcessed>>();
    world.init_resource::<Events<crate::systems::timelapse_replay::InterruptTimelapse>>();
    world.insert_resource(ReplayBuffer::new(1.0));
    // 记录：@+9=tag 3（动画即时化+清队），@+0xb=flag 7。
    let mut record = [0u8; 16];
    record[9] = 3;
    record[11] = 7;
    world
        .resource_mut::<PendingHistoryControl>()
        .0
        .push_back(HistoryControlRecord { bytes: record });
    world
        .resource_mut::<PendingHistoryCommands>()
        .0
        .push_back(HistoryCommand::UNDO);
    let mut schedule = Schedule::default();
    schedule.add_systems(process_history_control_stream);
    schedule.run(&mut world);
    assert!(world.resource::<PendingHistoryCommands>().0.is_empty());
    assert_eq!(world.resource::<Events<HistoryControlProcessed>>().len(), 1);
}

#[test]
fn original_load_commands_preserve_queue_and_suppress_audio() {
    let mut pending = PendingHistoryCommands::default();
    let mut replay = ReplayBuffer::new(1.0);
    for _ in 0..13 {
        assert!(
            accept_history_command(
                HistoryCommand {
                    bytes: [1, 1, 1, 3]
                },
                &mut pending,
                &mut replay
            )
            .is_none()
        );
    }
    assert_eq!(pending.0.len(), 13);
    assert!(
        pending
            .0
            .iter()
            .all(|command| command.bytes == [1, 1, 1, 3])
    );
    let event = accept_history_command(HistoryCommand::UNDO, &mut pending, &mut replay).unwrap();
    assert_eq!(event.kind, 0);
    assert!(pending.0.is_empty());
}

#[test]
fn second_stream_conditional_clear_only_matches_last_pending_command() {
    let mut world = World::new();
    world.init_resource::<PendingHistoryCommands>();
    world.init_resource::<PendingHistoryControl>();
    world.init_resource::<Events<HistoryControlProcessed>>();
    world.init_resource::<Events<crate::systems::timelapse_replay::InterruptTimelapse>>();
    world.insert_resource(ReplayBuffer::new(1.0));
    // tag 0：队尾 byte1==1（redo）才清队。
    let mut record = [0u8; 16];
    record[9] = 0;
    record[11] = 0x11;
    world
        .resource_mut::<PendingHistoryCommands>()
        .0
        .push_back(HistoryCommand::REDO);
    world
        .resource_mut::<PendingHistoryControl>()
        .0
        .push_back(HistoryControlRecord { bytes: record });
    let mut schedule = Schedule::default();
    schedule.add_systems(process_history_control_stream);
    schedule.run(&mut world);
    assert!(world.resource::<PendingHistoryCommands>().0.is_empty());
    // tag 0 但队尾是 undo（byte1==0）→ 不清队，默认路径压入记录 @+8 命令。
    let mut record = [0u8; 16];
    record[9] = 0;
    record[8] = 0xAB; // u32 @+8
    record[11] = 0x12;
    world
        .resource_mut::<PendingHistoryCommands>()
        .0
        .push_back(HistoryCommand::UNDO);
    world
        .resource_mut::<PendingHistoryControl>()
        .0
        .push_back(HistoryControlRecord { bytes: record });
    let mut schedule = Schedule::default();
    schedule.add_systems(process_history_control_stream);
    schedule.run(&mut world);
    let pending = world.resource::<PendingHistoryCommands>();
    assert_eq!(pending.0.len(), 2);
    assert_eq!(pending.0.back().unwrap().bytes[0], 0xAB);
}

#[test]
fn stairs_history_edit_serde_matches_native_single_edge_shape() {
    use crate::resources::history::stairs_history_edit::StairsHistoryEdit;
    use crate::resources::stairs::stairs_state::StairGraphEdge;
    let edit = StairsHistoryEdit::SupportsRecreate(StairGraphEdge::new(3, 4));
    let json = serde_json::to_string(&edit).unwrap();
    // 发行存档里的实际形状是元组数组：{"SupportsRecreate":{"edge":[11,12]}}
    // （assets/starting-builds/**/history.json），不是 {"a":..,"b":..} 对象。
    assert_eq!(json, r#"{"SupportsRecreate":{"edge":[3,4]}}"#);
    let back: StairsHistoryEdit = serde_json::from_str(&json).unwrap();
    assert_eq!(back, edit);
    let delete = StairsHistoryEdit::SupportsDelete(StairGraphEdge::new(7, 8));
    assert_eq!(
        serde_json::to_string(&delete).unwrap(),
        r#"{"SupportsDelete":{"edge":{"a":7,"b":8}}}"#
    );
}

#[test]
fn missing_prefab_returns_observed_constant() {
    use crate::resources::history::tree_edit::missing_prefab;
    assert_eq!(missing_prefab(), "MISSING_TREE_PREFAB_NAME");
}

struct CountingColliders(usize);
impl crate::resources::walls::decorator_storage::DecoratorColliderRemover for CountingColliders {
    fn remove_decorator_collider(&mut self, _id: crate::resources::walls::decorator::DecoratorId) {
        self.0 += 1;
    }
}

fn empty_archivist() -> crate::resources::walls::decorator_archivist::changes::DecoratorArchivist<()>
{
    use crate::resources::walls::decorator_archivist::DecoratorCullPolicy;
    crate::resources::walls::decorator_archivist::changes::DecoratorArchivist {
        walls: Default::default(),
        special1: None,
        special2: None,
        collision_a: Default::default(),
        collision_b: Default::default(),
        culling: DecoratorCullPolicy {
            allowed_ids: Default::default(),
            allow_any: false,
        },
    }
}

#[test]
fn replay_pop_bucket_nonempty_path_extracts_without_popping() {
    // fresh 0x140b38250：前桶命令非空 → 只 extract_diff + drop map，不弹桶。
    let mut replay = ReplayBuffer::new(1.0);
    replay.push(ReplayBucket {
        entry_index: 0,
        undo: false,
        commands: vec![crate::resources::history::ReplayCommand::Tree(
            Default::default(),
        )],
        timing: vec![Default::default(); 1],
    });
    let mut storage = crate::resources::walls::decorator_storage::DecoratorStorage::default();
    let mut id_gen = crate::resources::walls::decorator::DecoratorIdGen::default();
    let mut colliders = CountingColliders(0);
    let events = crate::resources::walls::decorator::DeferredDecoratorOpEvents::default();
    let mut stairs = crate::resources::stairs::stairs_state::StairsState::empty(
        crate::resources::stairs::stairs_state::ObservedStairEndpoint::from_observed([0; 24]),
    );
    let mut archivist = empty_archivist();
    let mut raycast = crate::systems::collision::world::RaycastWorld::new();
    assert!(
        replay_pop_bucket(
            &mut replay,
            &mut storage,
            &mut id_gen,
            &mut colliders,
            &events,
            &mut stairs,
            &mut archivist,
            &mut raycast,
        )
        .is_none()
    );
    assert_eq!(replay.len(), 1);
}

#[test]
fn replay_pop_bucket_empty_path_pops_and_restores_aux() {
    // fresh 0x140b38250：命令空 → pop_front → restore_decorators（aux 非空）
    // → stairs 恢复 → extract_diff → flush_colliders。
    use crate::resources::history::decorator_edit::{
        DecoratorHistoryDiffMap, DecoratorStorageAddrHistoryState, DecoratorStorageHistoryDiff,
    };
    use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
    let mut diffs = DecoratorHistoryDiffMap::default();
    diffs.insert(
        DecoratorStorageAddr::Terrain,
        DecoratorStorageHistoryDiff {
            before: DecoratorStorageAddrHistoryState::default(),
            after: DecoratorStorageAddrHistoryState::default(),
        },
    );
    let aux = crate::resources::history::replay_cmd_buffer::ReplayBucketAux {
        decorator_diff: Some(std::sync::Arc::new(diffs)),
        stairs_diff: None,
        cull_allow: None,
    };
    let mut replay = ReplayBuffer::new(1.0);
    replay.push_with_aux(
        ReplayBucket {
            entry_index: 4,
            undo: true,
            commands: Vec::new(),
            timing: Vec::new(),
        },
        aux,
    );
    let mut storage = crate::resources::walls::decorator_storage::DecoratorStorage::default();
    let mut id_gen = crate::resources::walls::decorator::DecoratorIdGen::default();
    let mut colliders = CountingColliders(0);
    let events = crate::resources::walls::decorator::DeferredDecoratorOpEvents::default();
    let mut stairs = crate::resources::stairs::stairs_state::StairsState::empty(
        crate::resources::stairs::stairs_state::ObservedStairEndpoint::from_observed([0; 24]),
    );
    let mut archivist = empty_archivist();
    let mut raycast = crate::systems::collision::world::RaycastWorld::new();
    let finished = replay_pop_bucket(
        &mut replay,
        &mut storage,
        &mut id_gen,
        &mut colliders,
        &events,
        &mut stairs,
        &mut archivist,
        &mut raycast,
    )
    .expect("empty bucket pops");
    assert_eq!(finished.entry_index, 4);
    assert!(finished.undo);
    assert!(replay.is_empty());
    assert!(replay.aux.is_empty());
}

#[test]
fn history_finish_recording_gates_on_recording_and_changed_owners() {
    use crate::resources::history::decorator_edit::{
        DecoratorHistoryDiffMap, DecoratorStorageAddrHistoryState, DecoratorStorageHistoryDiff,
    };
    use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
    use crate::systems::history::recording::history_finish_recording;
    let mut history: History<()> = empty_history();
    let mut edges = std::collections::HashSet::new();
    let empty_diff = DecoratorHistoryDiffMap::default();
    let empty_stairs = crate::resources::stairs::stairs_state::StairsHistoryDiff {
        before_checksum: 0,
        after_checksum: 0,
        removed: Vec::new(),
        added: Vec::new(),
    };
    // 未录制 → None。
    assert!(
        history_finish_recording(&mut history, &empty_diff, &empty_stairs, &mut edges, false)
            .is_none()
    );
    history.start_input_recording(0).unwrap();
    // 无变更 → None，pending 保留。
    assert!(
        history_finish_recording(&mut history, &empty_diff, &empty_stairs, &mut edges, false)
            .is_none()
    );
    assert!(history.is_recording());
    // 有装饰 diff → 提交并替换 aux Arc。
    let mut diffs = DecoratorHistoryDiffMap::default();
    diffs.insert(
        DecoratorStorageAddr::Terrain,
        DecoratorStorageHistoryDiff {
            before: DecoratorStorageAddrHistoryState::default(),
            after: DecoratorStorageAddrHistoryState::default(),
        },
    );
    let commit = history_finish_recording(&mut history, &diffs, &empty_stairs, &mut edges, false)
        .expect("changed owners commit an entry");
    assert_eq!(commit.entry_index, 0);
    assert!(!history.is_recording());
    assert!(history.decorator_diff.is_some());
}

#[test]
fn tree_edit_serializes_native_field_names() {
    use crate::resources::history::tree_edit::TreeEditData;
    let data = TreeEditData {
        storage_index: None,
        position: glam::Vec3::new(1.0, 2.0, 3.0),
        scale: 0.5,
        rotation_degrees: 45.0,
        id: 9,
        prefab_name: "Oak".to_string(),
        is_preplaced: true,
        previous_position: glam::Vec3::ZERO,
    };
    // serde_json::Value 的 object 是 BTreeMap（排序），改断言序列化文本的 entry 序。
    let json = serde_json::to_string(&data).unwrap();
    assert_eq!(
        json,
        r#"{"position":[1.0,2.0,3.0],"scale":0.5,"rotation_degrees":45.0,"id":9,"prefab_name":"Oak","is_preplaced":true}"#
    );
}

#[test]
fn quiet_load_starts_a_real_replay_without_emitting_audio() {
    let mut world = World::new();
    register_history_events(&mut world);
    let mut history = empty_history();
    history
        .ledger_mut()
        .entries
        .push(HistoryEntry::from_edits(Vec::new()));
    world.insert_resource(history);
    world.insert_resource(ReplayBuffer::new(1.0));
    world
        .resource_mut::<Events<HistoryCommand>>()
        .send(HistoryCommand {
            bytes: [1, 1, 1, 3],
        });
    let mut schedule = Schedule::default();
    schedule.add_systems(history_process_system::<()>);
    schedule.run(&mut world);
    assert!(
        world
            .resource::<Events<crate::systems::audio::PlayReplayAudio>>()
            .is_empty()
    );
    assert_eq!(
        world
            .resource::<Events<history_process::NewReplayCommandEvent>>()
            .len(),
        1
    );
    assert_eq!(
        core::mem::size_of::<history_process::NewReplayCommandEvent>(),
        0
    );
    assert_eq!(world.resource::<History<()>>().ledger().cursor, 0);
    assert_eq!(world.resource::<ReplayBuffer>().buckets.len(), 1);
}

#[test]
fn clear_interrupts_timelapse_and_emits_the_distinct_audio_kind() {
    let mut world = World::new();
    register_history_events(&mut world);
    world.insert_resource(empty_history());
    world.insert_resource(ReplayBuffer::new(1.0));
    world
        .resource_mut::<PendingHistoryCommands>()
        .0
        .push_back(HistoryCommand::REDO);
    world
        .resource_mut::<Events<HistoryCommand>>()
        .send(HistoryCommand {
            bytes: [0, 0, 0, 2],
        });
    let mut schedule = Schedule::default();
    schedule.add_systems(history_process_system::<()>);
    schedule.run(&mut world);
    assert!(world.resource::<PendingHistoryCommands>().0.is_empty());
    assert_eq!(
        world
            .resource::<Events<crate::systems::timelapse_replay::InterruptTimelapse>>()
            .len(),
        1
    );
    let mut audio =
        bevy_ecs::event::EventCursor::<crate::systems::audio::PlayReplayAudio>::default();
    assert_eq!(
        audio
            .read(world.resource())
            .map(|event| event.kind)
            .collect::<Vec<_>>(),
        [2]
    );
    assert!(
        world
            .resource::<Events<history_process::NewReplayCommandEvent>>()
            .is_empty()
    );
}

#[test]
fn recording_consumes_only_current_event_and_returns_before_queue_pop() {
    let mut world = World::new();
    register_history_events(&mut world);
    let mut history = empty_history();
    history
        .ledger_mut()
        .entries
        .push(HistoryEntry::from_edits(Vec::new()));
    history.start_input_recording(0).unwrap();
    world.insert_resource(history);
    world.insert_resource(ReplayBuffer::new(1.0));
    world
        .resource_mut::<PendingHistoryCommands>()
        .0
        .push_back(HistoryCommand::REDO);
    let quiet_redo = HistoryCommand {
        bytes: [1, 1, 1, 3],
    };
    // Native consumes this first event while recording, then returns. If it
    // were retained, the next run would clear the old pending command.
    world
        .resource_mut::<Events<HistoryCommand>>()
        .send(HistoryCommand {
            bytes: [0, 3, 0, 0],
        });
    world
        .resource_mut::<Events<HistoryCommand>>()
        .send(quiet_redo);
    let mut schedule = Schedule::default();
    schedule.add_systems(history_process_system::<()>);
    schedule.run(&mut world);
    assert_eq!(world.resource::<PendingHistoryCommands>().0.len(), 1);
    assert_eq!(world.resource::<History<()>>().ledger().cursor, -1);
    assert!(world.resource::<ReplayBuffer>().is_empty());
    assert!(
        world
            .resource::<Events<crate::systems::audio::PlayReplayAudio>>()
            .is_empty()
    );
    assert!(
        world
            .resource::<Events<crate::systems::timelapse_replay::InterruptTimelapse>>()
            .is_empty()
    );
    assert!(
        world
            .resource::<Events<history_process::NewReplayCommandEvent>>()
            .is_empty()
    );
    // No Events::update/clear between runs: the same system's EventReader
    // must retain the second event and must not reread the first one.
    world
        .resource_mut::<History<()>>()
        .finish_input_recording()
        .unwrap();
    schedule.run(&mut world);
    assert_eq!(world.resource::<History<()>>().ledger().cursor, 0);
    assert_eq!(world.resource::<PendingHistoryCommands>().0.len(), 1);
    assert_eq!(
        world.resource::<PendingHistoryCommands>().0.front(),
        Some(&quiet_redo)
    );
    assert_eq!(world.resource::<ReplayBuffer>().buckets.len(), 1);
    assert_eq!(
        world
            .resource::<Events<history_process::NewReplayCommandEvent>>()
            .len(),
        1
    );
}

#[test]
fn recording_without_new_event_does_not_gate_existing_queue_selection() {
    let mut world = World::new();
    register_history_events(&mut world);
    let mut history = empty_history();
    history
        .ledger_mut()
        .entries
        .push(HistoryEntry::from_edits(Vec::new()));
    history.start_input_recording(0).unwrap();
    world.insert_resource(history);
    world.insert_resource(ReplayBuffer::new(1.0));
    world
        .resource_mut::<PendingHistoryCommands>()
        .0
        .push_back(HistoryCommand::REDO);
    let mut schedule = Schedule::default();
    schedule.add_systems(history_process_system::<()>);
    schedule.run(&mut world);
    assert!(world.resource::<History<()>>().is_recording());
    assert_eq!(world.resource::<History<()>>().ledger().cursor, 0);
    assert!(world.resource::<PendingHistoryCommands>().0.is_empty());
    assert_eq!(world.resource::<ReplayBuffer>().buckets.len(), 1);
    assert_eq!(
        world
            .resource::<Events<history_process::NewReplayCommandEvent>>()
            .len(),
        1
    );
}

/// 契约第 9 步：NewReplayCommandEvent 清空旧 allowed 集合；archivist 缺失时
/// 系统不运行（run_if resource_exists），新 cull 集合由 replay_decorator 注入。
#[cfg(feature = "native-physics")]
#[test]
fn new_replay_command_clears_previous_allowed_ids() {
    use crate::resources::walls::decorator_archivist::LiveDecoratorArchivist;
    let mut world = World::new();
    world.init_resource::<LiveDecoratorArchivist>();
    world.init_resource::<Events<history_process::NewReplayCommandEvent>>();
    world
        .resource_mut::<LiveDecoratorArchivist>()
        .culling
        .allowed_ids
        .insert(3);
    let mut schedule = Schedule::default();
    schedule.add_systems(history_clear_previous_state);
    schedule.run(&mut world);
    assert!(world
        .resource::<LiveDecoratorArchivist>()
        .culling
        .allowed_ids
        .contains(&3));
    world
        .resource_mut::<Events<history_process::NewReplayCommandEvent>>()
        .send(history_process::NewReplayCommandEvent);
    schedule.run(&mut world);
    assert!(world
        .resource::<LiveDecoratorArchivist>()
        .culling
        .allowed_ids
        .is_empty());
}
