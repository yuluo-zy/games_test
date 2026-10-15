use super::{
    save_backup_metronome::{BackupDecision, SaveBackupMetronome},
    save_game::{GameSaveSections, SaveFileSection, SerializedJson},
    save_thread::CurrentSaveFile,
    shadow_file::ShadowFile,
};
use serde_json::{json, Value};

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
