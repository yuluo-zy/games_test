use super::{
    save_backup_metronome::{BackupDecision, SaveBackupMetronome},
    save_game::{GameSaveSections, SaveFileSection, SerializedJson},
    save_thread::CurrentSaveFile,
    shadow_file::ShadowFile,
};
use serde_json::{json, Value};

#[test]
fn close_file_resets_native_metronome_fields() {
    let mut metronome = SaveBackupMetronome::from_backup_counter(42);
    metronome.add_active_game_time(3.0);
    assert_eq!(metronome.active_game_time(), 3.0);
    // O：close_file 尾部 `+0x140=0`、`+0x144=0x501502f900000000`
    // （累计 0.0，阈值 f32 0x501502f9 = 1e10）。
    metronome.reset_after_close();
    assert_eq!(metronome.counter(), 0);
    assert_eq!(metronome.active_game_time(), 0.0);
    assert_eq!(metronome.active_game_threshold(), 1.0e10);
}

#[test]
fn next_session_kind_clamps_decoded_values_above_two_to_one() {
    use super::load_game::clamp_next_session_kind;
    assert_eq!(clamp_next_session_kind(0), 0);
    assert_eq!(clamp_next_session_kind(1), 1);
    assert_eq!(clamp_next_session_kind(2), 2);
    // O：原生 `if (2 < decoded) decoded = 1`，不是 min(2)。
    assert_eq!(clamp_next_session_kind(3), 1);
    assert_eq!(clamp_next_session_kind(0x14), 1);
}

#[test]
fn parse_saved_game_data_parses_all_native_sections() {
    use super::load_game::parse_saved_game_data;
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("history.json");
    let body = json!({
        "Version": 27,
        "BackwardCompatibility": 0.5,
        "Camera": {
            "yaw": 1.0, "pitch": 2.0, "position": [3.0, 4.0, 5.0],
            "camera_rig_arm_length": 6.0, "pitch_offset": 7.0, "yaw_offset": 8.0
        },
        "Time of day": {
            "time_of_day": 0.25, "sun_xz_rotation": 0.5, "latitude_degrees": 40.0
        },
        "Sheep": {"sheep": []},
        "Colors": {
            "freeform_wall": 1, "building": 2, "roof": 3,
            "building_wall_style": 1, "freehand_ty": 2, "halftimber": 4,
            "lantern_light": 5, "fence": 6, "window_light": 7, "flag": true,
            "windows": 8, "gates": 9, "flat_rim_style": 3,
            "lantern_brightness": 0.5, "stairs": 10
        },
        "History": {
            "chain_index": -1,
            "history_start": -1,
            "snapshots": [{"chain_index": -1, "snapshot_id": 7}],
            "edit_chain": [{"edits": []}]
        },
        "Identity": 123456789
    });
    std::fs::write(&path, serde_json::to_vec(&body).unwrap()).unwrap();
    let parsed = parse_saved_game_data(&path).unwrap();
    assert_eq!(parsed.version, 27);
    assert_eq!(parsed.camera.unwrap().yaw, 1.0);
    assert_eq!(parsed.time_of_day.time_of_day, 0.25);
    assert_eq!(parsed.time_of_day.latitude_degrees, 40.0);
    assert!(parsed.sheep.unwrap().sheep.is_empty());
    assert_eq!(parsed.colors.building.0, 2);
    assert_eq!(parsed.identity.unwrap().0, 123456789);
    assert_eq!(parsed.chain_index, -1);
    assert_eq!(parsed.history_start, -1);
    assert_eq!(parsed.edits.len(), 1);
    assert_eq!(parsed.snapshots.len(), 1);

    // 版本门（O：<3 / ≥0x1c 两条错误）。
    let mut low = body.clone();
    low["Version"] = json!(2);
    std::fs::write(&path, serde_json::to_vec(&low).unwrap()).unwrap();
    assert!(parse_saved_game_data(&path).is_err());
    let mut high = body.clone();
    high["Version"] = json!(0x1c);
    std::fs::write(&path, serde_json::to_vec(&high).unwrap()).unwrap();
    assert!(parse_saved_game_data(&path).is_err());
}

#[test]
fn unsaved_file_list_uses_the_native_0x80_byte_entry() {
    use super::save_thread::{UnsavedFileEntry, UnsavedFileList};
    assert_eq!(std::mem::size_of::<UnsavedFileEntry>(), 0x80);
    let list = UnsavedFileList::default();
    let path = std::path::PathBuf::from("session/history.json");
    list.add(UnsavedFileEntry::from_path(path.clone()));
    assert_eq!(list.len(), 1);
    assert!(list.contains_file(&path));
    assert!(!list.contains_file(std::path::Path::new("session/other.json")));
}

#[test]
fn save_game_to_memory_serializes_edit_items_inside_the_function() {
    use super::save_game::{save_game_to_memory, GameSaveSections};
    let values = std::array::from_fn(|_| SerializedJson::new(&json!([])).unwrap());
    let sections = GameSaveSections { values };
    // 传域类型（serde_json::Value）而非预序列化文本：函数内完成 serde。
    let (bytes, structure) =
        save_game_to_memory(sections, vec![json!({"edit": 1}), json!({"edit": 2})]);
    assert_eq!(structure.history_edit_offsets.len(), 2);
    let parsed: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        parsed["History"]["edit_chain"],
        json!([{"edit": 1}, {"edit": 2}])
    );
}

#[test]
fn create_grow_replace_shrink_retry_and_sync_actual_shadow_file() {
    // This test owns an isolated directory. No game profile/save root is read.
    let root = tempfile::tempdir().unwrap();
    let directory = root.path().join("session");
    let values = [
        json!(27),
        json!({"id":"fixture"}),
        json!(0),
        json!({"x":1}),
        json!(12),
        json!([]),
        json!({}),
        json!(2),
        json!(-1),
        json!([]),
    ];
    let sections = GameSaveSections {
        values: values.map(|value| SerializedJson::new(&value).unwrap()),
    };
    let edits = [
        SerializedJson::new(&json!({"fixture":1})).unwrap(),
        SerializedJson::new(&json!({"fixture":"x".repeat(10000)})).unwrap(),
    ];
    let mut file = CurrentSaveFile::create(&directory, sections, &edits);
    // Eager creation fails, but retains the complete protocol image and error.
    assert!(!file.shadow_file.is_open());
    assert!(file.shadow_file.last_error().is_some());
    assert_eq!(
        file.shadow_file.dirty_blocks().collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    std::fs::create_dir(&directory).unwrap();
    assert!(file.sync_all());
    assert!(file.shadow_file.last_error().is_none());
    let old_offsets = file.save_file_structure.history_edit_offsets.clone();
    let original_range = file.save_file_structure.section_ranges[&SaveFileSection::Camera].clone();
    let camera = SerializedJson::new(&json!({"x":"large".repeat(200)})).unwrap();
    file.change_section(SaveFileSection::Camera, &camera);
    let new_range = file.save_file_structure.section_ranges[&SaveFileSection::Camera].clone();
    assert_eq!(new_range.start, original_range.start);
    let growth = new_range.end - original_range.end;
    assert!(growth > 0);
    assert_eq!(
        file.save_file_structure.history_edit_offsets,
        old_offsets.iter().map(|x| x + growth).collect::<Vec<_>>()
    );
    let smaller = SerializedJson::new(&json!({"x":2})).unwrap();
    file.change_section(SaveFileSection::Camera, &smaller);
    assert_eq!(
        file.save_file_structure.section_ranges[&SaveFileSection::Camera],
        new_range
    );
    assert!(file.replace_edits(1, &[SerializedJson::new(&json!({"fixture":3})).unwrap()]));
    assert!(file.sync_all());
    let bytes = std::fs::read(directory.join("history.json")).unwrap();
    assert_eq!(bytes, file.shadow_file.data());
    let parsed: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(parsed["Camera"], json!({"x":2}));
    assert_eq!(
        parsed["History"]["edit_chain"],
        json!([{"fixture":1},{"fixture":3}])
    );
    assert!(file.shadow_file.dirty_blocks().next().is_none());
    assert!(file.replace_edits(0, &[]));
    assert_eq!(
        serde_json::from_slice::<Value>(file.shadow_file.data()).unwrap()["History"]["edit_chain"],
        json!([])
    );

    let mut isolated = ShadowFile::create(vec![7; 9000], &directory, "blocks.bin");
    isolated.write_at(4094, &[1, 2, 3, 4]);
    assert_eq!(isolated.dirty_blocks().collect::<Vec<_>>(), vec![0, 1]);
    isolated.truncate(4096);
    assert_eq!(isolated.dirty_blocks().collect::<Vec<_>>(), vec![0]);
    assert!(isolated.sync_all());
    assert_eq!(
        std::fs::read(directory.join("blocks.bin")).unwrap().len(),
        4096
    );

    let mut clock = SaveBackupMetronome::from_backup_counter(598);
    clock.add_active_game_time(100.0);
    assert_eq!(
        clock.consider_backing_up(),
        Some(BackupDecision {
            counter: 600,
            remove_counter: None
        })
    );
    assert_eq!(clock.counter(), 601);
    assert_eq!(clock.active_game_time(), 5.0);
    assert_eq!(clock.consider_backing_up(), None);
    assert_eq!(clock.counter(), 606);
    let mut clock = SaveBackupMetronome::from_backup_counter(1500);
    clock.add_active_game_time(1.0);
    assert_eq!(
        clock.consider_backing_up(),
        Some(BackupDecision {
            counter: 1500,
            remove_counter: Some(900)
        })
    );
}

#[test]
fn backup_scan_and_snapshot_gc_release_cancel_and_delete_actual_files() {
    use super::{
        save_backup_list::{BackupSaveFileKind, SaveBackupList},
        snapshot_gc::{parse_save_file_snapshots, snapshot_gc_thread},
    };
    use hashbrown::HashSet;
    use std::fs;

    let root = tempfile::tempdir().unwrap();
    let backups = root.path().join("backups");
    fs::create_dir(&backups).unwrap();
    let auto = backups.join("history-auto-600.json");
    let manual = backups.join("history-manual-fixture.bak");
    let body = br#"{"ignored":{},"History":{"snapshots":[{"chain_index":-1,"snapshot_id":7}],"edit_chain":BROKEN"#;
    fs::write(&auto, body).unwrap();
    fs::write(&manual, br#"{"History":{"snapshots":[]}}"#).unwrap();
    fs::write(backups.join("ignored.txt"), b"not a save").unwrap();
    fs::create_dir(backups.join("directory.json")).unwrap();
    let list = SaveBackupList::new_from_directory(&backups);
    assert_eq!(list.files().len(), 2);
    assert_eq!(list.backup_metronome_counter(), 601);
    assert_eq!(
        list.files()
            .iter()
            .find(|file| file.path == auto)
            .unwrap()
            .kind,
        BackupSaveFileKind::Auto
    );
    assert_eq!(
        list.files()
            .iter()
            .find(|file| file.path == manual)
            .unwrap()
            .kind,
        BackupSaveFileKind::Manual
    );
    assert_eq!(parse_save_file_snapshots(&body[..])[0].chain_index, -1);
    assert!(parse_save_file_snapshots(&b"{broken"[..]).is_empty());

    let snapshot_dir = root.path().join("snapshots");
    fs::create_dir(&snapshot_dir).unwrap();
    let paths: Vec<_> = [
        "4.snapshot",
        "7.snapshot",
        "9.snapshot",
        "invalid.snapshot",
        "notes.txt",
    ]
    .map(|name| snapshot_dir.join(name))
    .into();
    for path in &paths {
        fs::write(path, b"fixture").unwrap();
    }
    let sections = GameSaveSections {
        values: std::array::from_fn(|_| SerializedJson::new(&json!([])).unwrap()),
    };
    let mut current = CurrentSaveFile::create(root.path(), sections, &[]);
    let job = current.start_snapshot_gc(
        vec![auto.clone(), manual.clone()],
        paths.clone(),
        HashSet::from([4]),
    );
    assert!(current.is_backup_locked(&auto));
    job.join().unwrap();
    assert_eq!(current.consume_snapshot_gc_release(), Some(auto.clone()));
    assert!(!current.is_backup_locked(&auto));
    assert_eq!(current.consume_snapshot_gc_release(), Some(manual));
    assert_eq!(current.consume_snapshot_gc_release(), None);
    assert!(paths[0].exists());
    assert!(paths[1].exists());
    assert!(!paths[2].exists());
    assert!(!paths[3].exists());
    assert!(paths[4].exists());

    // A save switch drops the old receiver: the next release aborts before
    // the deletion phase, including when the backup itself failed to open.
    fs::write(&paths[2], b"keep after cancellation").unwrap();
    let (sender, receiver) = crossbeam_channel::unbounded();
    drop(receiver);
    snapshot_gc_thread(
        vec![backups.join("missing.json")],
        vec![paths[2].clone()],
        HashSet::new(),
        sender,
    );
    assert!(paths[2].exists());
}

#[test]
fn execute_save_cmd_changes_sections_and_flushes_to_disk() {
    use super::{
        save_game::AsyncSaveCmd,
        save_thread::{execute_save_cmd, CurrentSaveFile, UnsavedFileList},
    };
    let root = tempfile::tempdir().unwrap();
    let unsaved = UnsavedFileList::default();
    let values = [
        json!(27),
        json!({"id":"fixture"}),
        json!(0),
        json!({"x":1}),
        json!(12),
        json!([]),
        json!({}),
        json!(2),
        json!(-1),
        json!([]),
    ];
    let sections = GameSaveSections {
        values: values.map(|value| SerializedJson::new(&value).unwrap()),
    };
    let mut current = Some(CurrentSaveFile::create(root.path(), sections, &[]));
    let camera = SerializedJson::new(&json!({"x": "large".repeat(200)})).unwrap();
    execute_save_cmd(
        &mut current,
        AsyncSaveCmd::ChangeSection {
            section: SaveFileSection::Camera,
            value: super::save_game::defer_json(camera),
        },
        &unsaved,
    );
    let (reply, completed) = crossbeam_channel::bounded(1);
    execute_save_cmd(&mut current, AsyncSaveCmd::Flush { reply }, &unsaved);
    completed.recv().unwrap().unwrap();
    let bytes = std::fs::read(root.path().join("history.json")).unwrap();
    let parsed: Value = serde_json::from_slice(&bytes).unwrap();
    let written = parsed["Camera"]["x"].as_str().unwrap();
    assert_eq!(written.len(), 1000);
    // CloseFile 清档并 flush 前一档；后续 ChangeSection 应 panic。
    execute_save_cmd(
        &mut current,
        AsyncSaveCmd::CloseFile {
            session: crate::resources::session::SessionId::new(),
        },
        &unsaved,
    );
    assert!(current.is_none());
}

#[test]
#[should_panic(expected = "No active save file, but AsyncSaveCmd::ChangeSection has been issued")]
fn change_section_without_active_file_panics() {
    use super::{
        save_game::AsyncSaveCmd, save_thread::execute_save_cmd, save_thread::UnsavedFileList,
    };
    let mut current: Option<CurrentSaveFile> = None;
    let unsaved = UnsavedFileList::default();
    execute_save_cmd(
        &mut current,
        AsyncSaveCmd::ChangeSection {
            section: SaveFileSection::Camera,
            value: super::save_game::defer_json(SerializedJson::new(&json!(1)).unwrap()),
        },
        &unsaved,
    );
}

#[test]
fn save_thread_exits_when_the_command_channel_disconnects() {
    use super::save_thread::{save_thread, AutosaveErrorState, UnsavedFileList};
    let (sender, receiver) = crossbeam_channel::unbounded();
    let (error_sender, _errors) = AutosaveErrorState::channel();
    let handle = std::thread::spawn(move || {
        save_thread(
            receiver,
            error_sender,
            std::sync::Arc::new(UnsavedFileList::default()),
        )
    });
    drop(sender);
    handle.join().unwrap();
}

#[test]
fn save_thread_empty_poll_keeps_looping_until_all_senders_disconnect() {
    use super::save_thread::{save_thread, AutosaveErrorState, UnsavedFileList};
    let (sender, receiver) = crossbeam_channel::unbounded();
    let second_owner = sender.clone();
    let (error_sender, _errors) = AutosaveErrorState::channel();
    let handle = std::thread::spawn(move || {
        save_thread(
            receiver,
            error_sender,
            std::sync::Arc::new(UnsavedFileList::default()),
        )
    });
    drop(sender);
    std::thread::sleep(std::time::Duration::from_millis(30));
    assert!(!handle.is_finished(), "Empty is not Disconnected");
    drop(second_owner);
    handle.join().unwrap();
}

#[test]
fn on_load_fence_terminates_both_replay_passes() {
    use super::load_game::{advance_load_game, ReplayedOnLoadCommand};
    use crate::meta_commander::{MetaCommand, MetaCommandFence};
    use crate::resources::session::NextSessionKind;
    use crate::resources::session::{
        NextSession, NextSessionTail, NextSessionTemplate, SessionId, ON_LOAD_VARIANT_MASK,
    };

    let mut session = NextSession::create(
        &NextSessionTemplate::new(SessionId::new(), NextSessionKind::LoadOnLoad),
        &NextSessionTail::EMPTY,
    );
    session.extend_on_load([
        MetaCommand {
            tag: 1,
            payloads: [10, 0, 0, 0, 0, 0],
        },
        MetaCommand {
            tag: 2,
            payloads: [20, 0, 0, 0, 0, 0],
        },
    ]);
    // 变体 0 会话加载命令（payload = fence）+ 两遍回放终止哨兵。
    session.append_fence(MetaCommandFence::new_pair_blocked());
    session.append_replay_terminator();
    let mut manager = super::load_game::LoadGameManager::default();
    manager.state = super::load_game::LoadGameState::Loaded;
    let (parsed, commands) = advance_load_game(&mut manager, &session, None).unwrap();
    assert!(parsed.is_none());
    // 第一遍：批量展开为两条 MetaCommand + fence 变体 0 写 manager 状态；
    // 第二遍：两条 Apply + fence 变体 0 SwitchAppMode（忽略）；终止哨兵后不再回放。
    let meta = commands
        .iter()
        .filter(|c| matches!(c, ReplayedOnLoadCommand::MetaCommand(_)))
        .count();
    let apply = commands
        .iter()
        .filter(|c| matches!(c, ReplayedOnLoadCommand::Apply { .. }))
        .count();
    assert_eq!(meta, 2);
    assert_eq!(apply, 2);
    assert!(ON_LOAD_VARIANT_MASK != 0);
}

#[test]
fn on_load_variant2_fence_release_unblocks_inline() {
    use super::load_game::{advance_load_game, LoadGameManager, LoadGameState};
    use crate::meta_commander::MetaCommandFence;
    use crate::resources::session::{
        NextSession, NextSessionKind, NextSessionTail, NextSessionTemplate, SessionId,
    };
    let mut session = NextSession::create(
        &NextSessionTemplate::new(SessionId::new(), NextSessionKind::LoadOnLoad),
        &NextSessionTail::EMPTY,
    );
    let fence = MetaCommandFence::new_pair_blocked();
    let check = fence.clone();
    // 变体 2 fence 释放条目 + 终止哨兵。
    session.append_fence_release(fence);
    session.append_replay_terminator();
    let mut manager = LoadGameManager::default();
    manager.state = LoadGameState::Loaded;
    let (parsed, _commands) = advance_load_game(&mut manager, &session, None).unwrap();
    assert!(parsed.is_none());
    // 变体2 内联释放（O 0x14146d030 首遍 uVar23==2）：fence 已被 unblock（+0x10 清 0）。
    assert!(!check.is_blocked());
}
