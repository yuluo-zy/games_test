//! Actual ECS completion observer with a nonempty marker in an isolated account.
//! Original608B only unlinks after completion and a non13/14 mode; it does not
//! announce readiness, restore AppMode, or wait for converter/render work.
use bevy_app::{App, Update};
use country_core::{resources::session::{ActiveSession,SessionId,SessionMeta},
    systems::{mode_manager::AppMode,save::{load_game::LoadingGameComplete,
        load_completion_checker::check_load_completion_system}}};

#[test]
fn original_complete_event_sticky_local_mode_gate_and_unlink_error_policy() {
    let account=tempfile::tempdir().unwrap();
    paths::initialize_paths_with_installation(0,Some(account.path()),account.path()).unwrap();
    let id=SessionId::new();
    let folder=id.save_folder_path();
    std::fs::create_dir_all(&folder).unwrap();
    let marker=folder.join("loading.tmp");
    std::fs::write(&marker,b"actual unfinished load").unwrap();
    let mut app=App::new();
    app.add_event::<LoadingGameComplete>();
    app.insert_resource(ActiveSession::new(id,SessionMeta::default()));
    app.insert_resource(AppMode::default());
    app.add_systems(Update,check_load_completion_system);
    app.update();
    assert!(marker.exists(),"no observed complete event must preserve marker");
    app.insert_resource(AppMode::LoadGame);
    app.world_mut().send_event(LoadingGameComplete{restore_to:AppMode::default()});
    app.world_mut().send_event(LoadingGameComplete{restore_to:AppMode::default()});
    app.update();
    assert!(marker.exists(),"native14 postpones unlink");
    assert_eq!(*app.world().resource::<AppMode>(),AppMode::LoadGame);
    app.insert_resource(AppMode::Replay);
    app.update();
    assert!(marker.exists(),"native13 postpones unlink after event buffer advances");
    app.insert_resource(AppMode::default());
    app.update();
    assert!(!marker.exists());
    // The Local bit is not reset after success or when switching sessions.
    std::fs::write(&marker,b"new marker after first unlink").unwrap();
    app.update();
    assert!(!marker.exists());
    let second=SessionId::new();
    std::fs::create_dir_all(second.save_folder_path()).unwrap();
    let next=second.save_folder_path().join("loading.tmp");
    std::fs::write(&next,b"second session marker").unwrap();
    app.insert_resource(ActiveSession::new(second,SessionMeta::default()));
    app.update();
    assert!(!next.exists());
    // Original drops NotFound and all other unlink errors; an existing directory
    // is a deterministic non-NotFound error and must not become App failure.
    std::fs::create_dir(&next).unwrap();
    app.update();
    assert!(next.is_dir());
    assert_eq!(*app.world().resource::<AppMode>(),AppMode::default());
}
