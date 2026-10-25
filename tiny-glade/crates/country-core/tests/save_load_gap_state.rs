//! Real load-state adapter over a temporary Motomoto file and canonical owners.
//! Transport observation verifies no OpenFile/ready notification on partial apply.
use bevy_ecs::{event::Events, world::World};
use country_core::{
    game_config::GameConfig,
    resources::{
        history::HistoryIdentity,
        session::{ActiveSession, SessionId, SessionMeta},
        time_of_day::TimeOfDay,
        walls::InnerWalls,
        world_raster::UploadWorldRasterBuffersToStagingCmd,
    },
    systems::{
        camera_rig::{CameraRigParametersCache, MainCamera},
        mode_manager::{AppMode, HistoryCommand},
        save::{
            load_game::{ActiveLoadOptions, LoadGameManager, LoadGameSource, LoadGameState,
                LoadingGameComplete, ParsedGameData, ParsedLoadMetadata, PostGameLoadCommand,
                try_load_game_system},
            save_game::{AsyncSaveCmd, AutoSaveState},
        },
    },
};

fn snapshot_file(root: &std::path::Path, broken: bool) {
    let mut writer = motomoto::Writer::new(std::io::Cursor::new(Vec::new()));
    let mut chunk = writer.add_chunk("Snapshot").unwrap();
    serde_json::to_writer(&mut chunk, &serde_json::json!({
        "Version": 2,
        "Raster": if broken { vec!["PathRaster"] } else { vec![] },
    })).unwrap();
    chunk.finish().unwrap();
    std::fs::write(root.join("7.snapshot"), writer.finish().unwrap().into_inner()).unwrap();
}

fn world_for_load(root: &std::path::Path) -> (World, crossbeam_channel::Receiver<AsyncSaveCmd>) {
    let mut world = World::new();
    let (sender, receiver) = crossbeam_channel::unbounded();
    let mut autosave = AutoSaveState::default();
    autosave.attach_command_sender(sender);
    world.insert_resource(autosave);
    world.insert_resource(AppMode::LoadGame);
    world.insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()));
    world.insert_resource(GameConfig::default());
    world.insert_resource(MainCamera::new(1.0));
    world.insert_resource(TimeOfDay::default());
    world.insert_resource(InnerWalls::new());
    world.init_resource::<country_core::resources::stairs::stairs_state::StairsState>();
    world.init_resource::<Events<UploadWorldRasterBuffersToStagingCmd>>();
    world.init_resource::<Events<LoadingGameComplete>>();
    world.init_resource::<Events<HistoryCommand>>();
    let parsed = ParsedGameData {
        migration_metadata: Some(ParsedLoadMetadata::VERSION_27),
        version: 27,
        camera: Some(CameraRigParametersCache::default()),
        identity: Some(HistoryIdentity(73)),
        chain_index: -1,
        history_start: -1,
        snapshots: vec![serde_json::json!({"chain_index": -1, "snapshot_id": 7})],
        ..ParsedGameData::default()
    };
    let mut manager = LoadGameManager::new();
    manager.begin(LoadGameSource::Parsed {
        data: Box::new(parsed), snapshot_root: root.to_path_buf(),
    }, ActiveLoadOptions {
        ignore_snapshot: false, request_reveal: false, new_session: false,
    }, vec![PostGameLoadCommand::SwitchAppMode(AppMode::default())]);
    world.insert_resource(manager);
    (world, receiver)
}

#[test]
fn partial_snapshot_apply_fails_terminally_before_save_open_or_load_completion() {
    let account = tempfile::tempdir().unwrap();
    paths::initialize_paths_with_installation(0, Some(account.path()), account.path()).unwrap();
    let snapshot_root = tempfile::tempdir().unwrap();
    snapshot_file(snapshot_root.path(), true);
    let (mut world, receiver) = world_for_load(snapshot_root.path());
    let error = try_load_game_system(&mut world).unwrap_err();
    assert!(error.to_string().contains("history snapshot application incomplete"));
    assert!(error.to_string().contains("file carried no raster chunks"));
    let manager = world.resource::<LoadGameManager>();
    assert_eq!(manager.state, LoadGameState::Failed);
    assert!(manager.options.is_none());
    assert!(manager.post_load_commands.is_empty());
    assert!(matches!(manager.source, LoadGameSource::None));
    assert!(receiver.try_recv().is_err(), "partial World must not produce OpenFile");
    assert!(world.resource::<Events<LoadingGameComplete>>().is_empty());
    for _ in 0..3 {
        try_load_game_system(&mut world).unwrap();
    }
    assert_eq!(world.resource::<LoadGameManager>().state, LoadGameState::Failed);
    assert!(receiver.try_recv().is_err());
    assert!(world.resource::<Events<LoadingGameComplete>>().is_empty());

    // The same adapter still opens and completes a legitimate empty-raster
    // snapshot. No downstream GPU or full-game readiness is inferred here.
    snapshot_file(snapshot_root.path(), false);
    let (mut complete, receiver) = world_for_load(snapshot_root.path());
    try_load_game_system(&mut complete).unwrap();
    assert_eq!(complete.resource::<LoadGameManager>().state, LoadGameState::ApplyCounter);
    assert!(matches!(receiver.try_recv().unwrap(), AsyncSaveCmd::OpenFile { .. }));
    try_load_game_system(&mut complete).unwrap();
    assert_eq!(complete.resource::<LoadGameManager>().state, LoadGameState::Loaded);
    assert!(complete.resource::<Events<LoadingGameComplete>>().is_empty());
    try_load_game_system(&mut complete).unwrap();
    assert_eq!(complete.resource::<Events<LoadingGameComplete>>().len(), 1);
    assert!(complete.resource::<LoadGameManager>().options.is_none());
}
