//! Compile the current startup sources against identity-checked cached support
//! libraries. This is a bounded source probe, not a fresh workspace build.
pub use country_core::{
    asset_libraries, game_config, game_profile, meta_commander, render, resources,
    rhapsody_types, systems, utils, utils_contract,
};

#[path = "../../crates/country-core/src/startup/load_prefabs.rs"]
pub mod load_prefabs;
#[path = "../../crates/country-core/src/startup/startup_session.rs"]
pub mod startup_session;
#[path = "../../crates/country-core/src/startup/run_game.rs"]
pub mod run_game;
#[path = "../../crates/country-core/src/startup/load_glade_audio.rs"]
pub mod load_glade_audio;
#[path = "../../crates/country-core/src/startup/startup_terrain.rs"]
pub mod startup_terrain;
#[path = "../../crates/country-core/src/startup/load_textures.rs"]
pub mod load_textures;
pub mod startup {
    pub use crate::{load_glade_audio, load_prefabs, load_textures, run_game, startup_session, startup_terrain};
}

use bevy_ecs::prelude::*;
use resources::{
    glade_settings::GladeSettings,
    render::mesh_atlas_library::PrefabName,
    session::{NewSessionStartedCmd, NextSession, SavedSessionList, SessionId, SessionLoadedCmd},
};
use startup::load_prefabs::{load_prefabs_impl, reload_prefabs, PrefabWatcher};

fn scratch(tag: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("startup-review-{tag}-{}", std::process::id()));
    assert!(!path.exists(), "probe directory already exists: {path:?}");
    std::fs::create_dir(&path).unwrap();
    path
}

const PREFAB: &str = r#"(parts: {"canopy": (
    render: (mesh: "canopy", shader: "tree"),
    rt: (mesh: Aabb, summer_albedo: [1.0,0.5,0.25],
         autumn_albedo: [0.5,0.5,0.5], winter_albedo: [0.0,0.0,0.0]),
)})"#;

#[test]
fn failed_prefab_reload_consumes_the_only_retry_notification() {
    let root = scratch("retry");
    let file = root.join("tree.ron");
    std::fs::write(&file, "(").unwrap();
    let mut watcher = PrefabWatcher::new(false);
    watcher.prefab_changes.lock().unwrap().insert(PrefabName::new("tree"));
    let mut library = resources::prefab::PrefabLibrary::default();
    assert!(reload_prefabs(&root, &mut watcher, &mut library).is_err());
    assert!(watcher.prefab_changes.lock().unwrap().is_empty());
    std::fs::write(file, PREFAB).unwrap();
    let changed = reload_prefabs(&root, &mut watcher, &mut library).unwrap();
    assert!(changed.0.is_empty() && changed.1.is_empty());
    assert!(library.inner.is_empty(), "fixed source was never loaded");
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn rescan_retains_a_prefab_deleted_from_disk() {
    let root = scratch("deletion");
    let file = root.join("tree.ron");
    std::fs::write(&file, PREFAB).unwrap();
    let mut library = resources::prefab::PrefabLibrary::default();
    load_prefabs_impl(&root, &mut library).unwrap();
    let name = PrefabName::new("tree");
    assert!(library.inner.contains_key(&name));
    std::fs::remove_file(file).unwrap();
    let mut watcher = PrefabWatcher::new(false);
    watcher.prefab_changes.lock().unwrap().insert(name.clone());
    let changed = reload_prefabs(&root, &mut watcher, &mut library).unwrap();
    assert!(changed.0.contains(&name));
    assert!(library.inner.contains_key(&name), "deleted asset remains available");
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn session_loader_rejects_its_own_on_load_producer() {
    use systems::save::{
        load_completion_checker::DisplayCorruptedSaveFileWarning,
        load_game::LoadGameManager, save_game::AutoSaveState,
    };
    let mut world = World::new();
    world.init_resource::<SavedSessionList>();
    world.init_resource::<game_profile::GameProfile>();
    world.init_resource::<AutoSaveState>();
    world.init_resource::<LoadGameManager>();
    world.init_resource::<systems::mode_manager::AppMode>();
    world.init_resource::<GladeSettings>();
    world.init_resource::<meta_commander::MetaCommander>();
    world.init_resource::<Events<NewSessionStartedCmd>>();
    world.init_resource::<Events<SessionLoadedCmd>>();
    world.init_resource::<Events<DisplayCorruptedSaveFileWarning>>();
    let command = meta_commander::MetaCommand { tag: 8, payloads: [0; 6] };
    let mut next = NextSession::load(SessionId::new());
    startup::run_game::build_new_session_on_load(&mut next, [command; 3]);
    assert!(!next.on_load.is_empty());
    world.insert_resource(next);
    let mut schedule = Schedule::default();
    schedule.add_systems(startup::startup_session::load_game_session);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
    let error = result.expect_err("current loader rejects owned on_load entries before file I/O");
    let message = error.downcast_ref::<String>().map(String::as_str)
        .or_else(|| error.downcast_ref::<&str>().copied()).unwrap_or("");
    assert!(message.contains("owned on_load conversion is not restored"), "{message}");
}

use std::{convert::Infallible, sync::Mutex};
use systems::audio::music::music_player::{MusicBackend, MusicEvent, MusicPlayer, StopMode};
struct DisabledEvent;
impl MusicEvent for DisabledEvent {
    type Error = Infallible;
    fn start(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn stop(&mut self, _: StopMode) -> Result<(), Self::Error> { Ok(()) }
    fn set_paused(&mut self, _: bool) -> Result<(), Self::Error> { Ok(()) }
    fn paused(&self) -> Result<bool, Self::Error> { Ok(false) }
    fn playback_state(&self) -> Result<resources::audio_instance::PlaybackState, Self::Error> {
        Ok(resources::audio_instance::PlaybackState::Stopped)
    }
    fn timeline_position(&self) -> Result<i32, Self::Error> { Ok(0) }
    fn install_beat_callback(&mut self) -> Result<(), Self::Error> { Ok(()) }
}
#[derive(Default)]
struct Backend(Mutex<Vec<(String, f32)>>);
impl MusicBackend for Backend {
    type Event = DisabledEvent;
    fn instantiate(&self, _: &str) -> Result<Option<Self::Event>, Infallible> { Ok(None) }
    fn set_global_parameter(&self, name: &str, value: f32) -> Result<(), Infallible> {
        self.0.lock().unwrap().push((name.into(), value));
        Ok(())
    }
}

#[test]
fn winter_audio_reuses_volume_for_both_semantic_parameters() {
    let backend = Backend::default();
    let mut player = MusicPlayer::init(&backend);
    startup::load_glade_audio::load_glade_audio(&mut player, &backend, std::path::Path::new("unused"), 4);
    assert_eq!(*backend.0.lock().unwrap(), [
        ("GladeTypeIsHalloween".into(), 0.25), ("IsHalloween".into(), 0.25),
    ]);
    assert_eq!(resources::session::GladeType::Winter.audio_id(), 4.0);
}
