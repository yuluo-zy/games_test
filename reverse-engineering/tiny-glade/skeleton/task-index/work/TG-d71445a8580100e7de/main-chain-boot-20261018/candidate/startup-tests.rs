//! 启动流程的工程回归：只使用临时资产与记录后端，不连接真实用户存档或音频设备。
use super::*;
use crate::{
    meta_commander::{MetaCommand, MetaCommandFence, MetaCommander},
    resources::{
        prefab::PrefabLibrary,
        render::mesh_atlas_library::PrefabName,
        session::{NextSession, NextSessionKind, OnLoadEntry, SessionId},
    },
    systems::{
        audio::music::music_player::{MusicBackend, MusicEvent, MusicPlayer, StopMode},
        mode_manager::AppMode,
        save::load_game::{
            ActiveLoadOptions, LoadGameManager, LoadGameSource, LoadGameState, PostGameLoadCommand,
        },
    },
};
use bevy_ecs::prelude::*;
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};

const TREE: &str = r#"(parts: {"canopy": (render: (mesh: "canopy", shader: "tree_canopy"))})"#;

fn prefab_test_assets() -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("assets/prefabs");
    let meshes = directory.path().join("assets/meshes");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::create_dir_all(&meshes).unwrap();
    std::fs::write(
        meshes.join("canopy.json"),
        r#"{
        "indices":{"buffer":[0,1,2]},"attributes":["Vertex_Position"],
        "Vertex_Position":{"type":["float",3],"buffer":[[0,0,0],[1,0,0],[0,1,0]]}
    }"#,
    )
    .unwrap();
    std::fs::write(root.join("tree.ron"), TREE).unwrap();
    (directory, root)
}

#[test]
fn core_initialization_projects_the_host_ui_and_preserves_live_time_screen() {
    use crate::systems::{
        mode_manager::ModeManagerUiContext,
        ui::resources::ui_state::{DragState, UiInteraction, UiState, WidgetId},
    };
    let mut world = World::new();
    let mut ui = UiState::new(UiInteraction {
        native_tag: 1,
        native_phase: DragState::NONE.0,
        widget: WidgetId::from_name("esc_menu_canvas"),
    });
    ui.set_intersection_tag(0);
    world.insert_resource(ui);
    let screen = crate::resources::gui::TimeOfDayScreen {
        visible: true,
        time_of_day: 0.75,
        ..Default::default()
    };
    world.insert_resource(screen);
    run_game::initialize_core_resources(&mut world);
    let context = world.resource::<ModeManagerUiContext>();
    assert!(crate::systems::mode_manager::esc_menu_blocks(
        context.probe,
        context.esc_menu_canvas_id,
    ));
    assert_eq!(
        *world.resource::<crate::resources::gui::TimeOfDayScreen>(),
        screen
    );
    // The projection must keep following the one live UI on later frames.
    world.resource_mut::<UiState>().set_intersection_tag(2);
    let mut schedule = Schedule::default();
    schedule.add_systems(crate::systems::mode_manager::synchronize_mode_manager_ui_context);
    schedule.run(&mut world);
    let context = world.resource::<ModeManagerUiContext>();
    assert!(!crate::systems::mode_manager::esc_menu_blocks(
        context.probe,
        context.esc_menu_canvas_id,
    ));
}

#[test]
fn failed_reload_consumes_only_prefab_batch_and_keeps_materialized_old_assets() {
    use crate::render::nani::gpu_world::GpuWorldSubsetKind;
    use load_prefabs::{PrefabWatcher, load_prefabs_impl, reload_prefabs};
    let (_directory, root) = prefab_test_assets();
    let source = root.join("tree.ron");
    let mut library = PrefabLibrary::default();
    load_prefabs_impl(&root, &mut library).unwrap();
    let name = PrefabName::new("tree");
    let previous = library.get(&name).parts[0].render_mesh.clone();
    let mut watcher = PrefabWatcher::new(false);
    watcher.prefab_changes.lock().unwrap().insert(name.clone());
    let subset = GpuWorldSubsetKind::Canopy;
    watcher.atlas_changes.lock().unwrap().insert(subset);
    std::fs::write(source, "(").unwrap();
    assert!(reload_prefabs(&root, &mut watcher, &mut library).is_err());
    assert!(Arc::ptr_eq(
        &previous,
        &library.get(&name).parts[0].render_mesh
    ));
    // Native drains prefab changes before scan; atlas changes have another owner.
    assert!(watcher.prefab_changes.lock().unwrap().is_empty());
    assert!(watcher.atlas_changes.lock().unwrap().contains(&subset));
}

#[test]
fn reload_preserves_unselected_and_deleted_owners_and_loads_new_names() {
    let (_directory, root) = prefab_test_assets();
    std::fs::write(root.join("unselected.ron"), TREE).unwrap();
    let mut library = PrefabLibrary::default();
    load_prefabs::load_prefabs_impl(&root, &mut library).unwrap();
    let deleted = PrefabName::new("tree");
    let unselected = PrefabName::new("unselected");
    let previous_deleted = library.get(&deleted).parts[0].render_mesh.clone();
    let previous_unselected = library.get(&unselected).parts[0].render_mesh.clone();
    std::fs::remove_file(root.join("tree.ron")).unwrap();
    std::fs::write(root.join("new.ron"), TREE).unwrap();
    let mut watcher = load_prefabs::PrefabWatcher::new(false);
    watcher
        .prefab_changes
        .lock()
        .unwrap()
        .insert(deleted.clone());
    let changed = load_prefabs::reload_prefabs(&root, &mut watcher, &mut library).unwrap();
    assert_eq!(changed, [PrefabName::new("new")].into_iter().collect());
    assert!(Arc::ptr_eq(
        &previous_deleted,
        &library.get(&deleted).parts[0].render_mesh
    ));
    assert!(Arc::ptr_eq(
        &previous_unselected,
        &library.get(&unselected).parts[0].render_mesh
    ));
    assert!(
        !library.get(&PrefabName::new("new")).parts[0]
            .render_mesh
            .indices
            .is_empty()
    );
}

#[test]
fn atlas_only_notification_does_not_scan_or_drain_in_prefab_stage() {
    use crate::render::nani::gpu_world::GpuWorldSubsetKind;
    let directory = tempfile::tempdir().unwrap();
    let mut watcher = load_prefabs::PrefabWatcher::new(false);
    let subset = GpuWorldSubsetKind::Canopy;
    watcher.atlas_changes.lock().unwrap().insert(subset);
    let changed = load_prefabs::reload_prefabs(
        &directory.path().join("absent"),
        &mut watcher,
        &mut PrefabLibrary::default(),
    )
    .unwrap();
    assert!(changed.is_empty());
    assert!(watcher.atlas_changes.lock().unwrap().contains(&subset));
}

#[test]
fn file_notifications_support_atomic_saves_and_removal() {
    use hotwatch::Event;
    let source = std::path::PathBuf::from("tree.ron");
    for event in [
        Event::Write(source.clone()),
        Event::Create(source.clone()),
        Event::Rename(source.clone(), source.clone()),
        Event::Remove(source),
        Event::Rescan,
    ] {
        assert!(load_prefabs::is_relevant_event(&event));
    }
}

fn command(tag: u64) -> MetaCommand {
    MetaCommand {
        tag,
        payloads: [tag; 6],
    }
}
fn options() -> ActiveLoadOptions {
    ActiveLoadOptions {
        ignore_snapshot: false,
        request_reveal: false,
        new_session: false,
    }
}

#[test]
fn on_load_transfers_full_commands_and_owned_fence_in_order() {
    let fence = MetaCommandFence::new_pair_blocked();
    assert_eq!(Arc::strong_count(&fence.0), 1, "构造不泄漏额外强引用");
    let mut request = NextSession::load(SessionId::new());
    request.append_fence(fence.clone());
    run_game::build_new_session_on_load(&mut request, [command(6), command(7), command(8)]);
    request.extend_on_load([command(99)]); // 终止标记之后的命令不会执行。
    let mut manager = LoadGameManager::default();
    manager.begin(
        LoadGameSource::None,
        options(),
        startup_session::make_post_load_commands(std::mem::take(&mut request.on_load)),
    );
    let mut commander = MetaCommander::default();
    let mut mode = AppMode::LoadGame;
    manager
        .finish_after_world_applied(&mut commander, &mut mode, |_| panic!("全部为已知动作"))
        .unwrap();
    for tag in [6, 7, 8] {
        assert_eq!(commander.consume(), Some(command(tag)));
    }
    assert!(commander.is_empty());
    assert!(!fence.is_blocked());
    assert_eq!(Arc::strong_count(&fence.0), 1);
    assert_eq!(manager.state, LoadGameState::Loaded);
    assert_eq!(mode, AppMode::default());
}

#[test]
fn completion_retry_does_not_duplicate_commands_or_release_fences_early() {
    let fence = MetaCommandFence::new_pair_blocked();
    let mut manager = LoadGameManager::default();
    manager.begin(
        LoadGameSource::None,
        options(),
        vec![
            PostGameLoadCommand::SessionEntry(OnLoadEntry::Batch {
                commands: vec![command(6)],
            }),
            PostGameLoadCommand::SessionEntry(OnLoadEntry::Fence(fence.clone())),
            PostGameLoadCommand::DropArc(serde_json::Value::Null),
            PostGameLoadCommand::SwitchAppMode(AppMode::default()),
        ],
    );
    let mut commander = MetaCommander::default();
    let mut mode = AppMode::LoadGame;
    assert!(
        manager
            .finish_after_world_applied(&mut commander, &mut mode, |_| anyhow::bail!(
                "域资源暂不可用"
            ))
            .is_err()
    );
    assert!(fence.is_blocked());
    assert_eq!(mode, AppMode::LoadGame);
    manager
        .finish_after_world_applied(&mut commander, &mut mode, |_| Ok(()))
        .unwrap();
    assert_eq!(commander.consume(), Some(command(6)));
    assert!(commander.is_empty());
    assert!(!fence.is_blocked());
}

#[test]
fn empty_request_is_consumed_once_and_absence_is_safe_on_later_frames() {
    let mut world = World::new();
    world.init_resource::<MetaCommander>();
    world.init_resource::<AppMode>();
    world.init_resource::<crate::game_profile::GameProfile>();
    world.init_resource::<crate::resources::session::SavedSessionList>();
    world.init_resource::<crate::resources::glade_settings::GladeSettings>();
    world.init_resource::<crate::systems::save::save_game::AutoSaveState>();
    world.init_resource::<LoadGameManager>();
    world.init_resource::<Events<crate::resources::session::NewSessionStartedCmd>>();
    world.init_resource::<Events<crate::resources::session::SessionLoadedCmd>>();
    world.init_resource::<Events<crate::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>>();
    let mut request = NextSession::load(SessionId::new());
    request.kind = NextSessionKind::None;
    world.insert_resource(request);
    let mut schedule = Schedule::default();
    run_game::register_session_systems(&mut schedule);
    for _ in 0..3 {
        schedule.run(&mut world);
    }
    assert!(!world.contains_resource::<NextSession>());
    assert_eq!(
        world.resource::<LoadGameManager>().state,
        LoadGameState::Loaded
    );
}

#[derive(Resource, Default)]
struct Backend {
    parameters: Mutex<Vec<(String, f32)>>,
}
struct Event;
impl MusicEvent for Event {
    type Error = Infallible;
    fn start(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn stop(&mut self, _: StopMode) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_paused(&mut self, _: bool) -> Result<(), Self::Error> {
        Ok(())
    }
    fn paused(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
    fn playback_state(
        &self,
    ) -> Result<crate::resources::audio_instance::PlaybackState, Self::Error> {
        Ok(crate::resources::audio_instance::PlaybackState::Stopped)
    }
    fn timeline_position(&self) -> Result<i32, Self::Error> {
        Ok(0)
    }
    fn install_beat_callback(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl MusicBackend for Backend {
    type Event = Event;
    fn instantiate(&self, _: &str) -> Result<Option<Event>, Infallible> {
        Ok(Some(Event))
    }
    fn set_global_parameter(&self, name: &str, value: f32) -> Result<(), Infallible> {
        self.parameters.lock().unwrap().push((name.into(), value));
        Ok(())
    }
}

#[test]
fn theme_audio_reads_config_and_replaces_playlist_without_growing_instances() {
    use crate::resources::session::GladeType;
    let directory = tempfile::tempdir().unwrap();
    let theme_dir = directory.path().join("winter");
    std::fs::create_dir(&theme_dir).unwrap();
    std::fs::write(
        theme_dir.join("audio.json"),
        r#"{"ambience":"amb", "music":["winter-a","winter-b"],"select_glade_stinger":"choose"}"#,
    )
    .unwrap();
    let backend = Backend::default();
    let mut player = MusicPlayer::init(&backend);
    for _ in 0..3 {
        player.current_track = Some(0);
        player.playlist.push(99);
        load_glade_audio::load_theme_audio(&mut player, &backend, &theme_dir, GladeType::Winter)
            .unwrap();
        assert_eq!(player.track_names, ["winter-a", "winter-b"]);
        assert_eq!(player.tracks.len(), 2);
        assert!(player.playlist.is_empty());
        assert_eq!(player.current_track, None);
    }
    assert_eq!(
        &backend.parameters.lock().unwrap()[..2],
        &[("GladeType".into(), 4.0), ("IsHalloween".into(), 0.0)]
    );
    assert_eq!(player.glade_theme_flag, 0.25);
    let calls_before_failure = backend.parameters.lock().unwrap().len();
    std::fs::write(theme_dir.join("audio.json"), "{").unwrap();
    assert!(
        load_glade_audio::load_theme_audio(&mut player, &backend, &theme_dir, GladeType::Winter)
            .is_err()
    );
    assert_eq!(player.track_names, ["winter-a", "winter-b"]);
    assert_eq!(player.tracks.len(), 2);
    assert_eq!(
        backend.parameters.lock().unwrap().len(),
        calls_before_failure
    );
}

#[test]
fn grid_rejects_overflow_before_appending_any_indices() {
    let mut indices = vec![123];
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        startup_terrain::grid_gen_cw(&mut indices, 0, 1, u32::MAX, 1);
    }));
    assert!(result.is_err());
    assert_eq!(indices, [123]);
}

#[test]
fn legacy_integer_cannot_be_reinterpreted_as_an_arc() {
    let mut request = NextSession::load(SessionId::new());
    request.on_load.push(OnLoadEntry::Command {
        tag: crate::resources::session::ON_LOAD_VARIANT_MASK | 2,
        payload_a: 1,
        payload_b: 0,
        payload_c: 0,
    });
    let mut manager = LoadGameManager::default();
    assert!(
        crate::systems::save::load_game::advance_load_game(&mut manager, &request, None).is_err()
    );
}

#[test]
fn ecs_completion_consumes_owned_actions_once_and_emits_one_event() {
    use crate::systems::save::load_game::{LoadingGameComplete, try_load_game_system};
    let fence = MetaCommandFence::new_pair_blocked();
    let mut manager = LoadGameManager::default();
    manager.begin(
        LoadGameSource::None,
        options(),
        vec![
            PostGameLoadCommand::SessionEntry(OnLoadEntry::Batch {
                commands: vec![command(6)],
            }),
            PostGameLoadCommand::SessionEntry(OnLoadEntry::Fence(fence.clone())),
            PostGameLoadCommand::SwitchAppMode(AppMode::default()),
        ],
    );
    // 模拟域已经完成世界应用；Loading 阶段不会伪造此状态。
    manager.state = LoadGameState::Loaded;
    let mut world = World::new();
    world.insert_resource(manager);
    world.insert_resource(AppMode::LoadGame);
    world.init_resource::<MetaCommander>();
    world.init_resource::<Events<LoadingGameComplete>>();
    try_load_game_system(&mut world).unwrap();
    try_load_game_system(&mut world).unwrap();
    assert_eq!(world.resource::<Events<LoadingGameComplete>>().len(), 1);
    assert!(!fence.is_blocked());
    let mut commander = world.resource_mut::<MetaCommander>();
    assert_eq!(commander.consume(), Some(command(6)));
    assert!(commander.is_empty());
    assert!(world.resource::<LoadGameManager>().options.is_none());
}

#[test]
fn ecs_completion_rejects_unknown_actions_before_releasing_signals() {
    use crate::systems::save::load_game::{LoadingGameComplete, try_load_game_system};
    let fence = MetaCommandFence::new_pair_blocked();
    let mut manager = LoadGameManager::default();
    manager.begin(
        LoadGameSource::None,
        options(),
        vec![
            PostGameLoadCommand::SessionEntry(OnLoadEntry::Fence(fence.clone())),
            PostGameLoadCommand::DropArc(serde_json::Value::Null),
            PostGameLoadCommand::SwitchAppMode(AppMode::default()),
        ],
    );
    manager.state = LoadGameState::Loaded;
    let mut world = World::new();
    world.insert_resource(manager);
    world.insert_resource(AppMode::LoadGame);
    world.init_resource::<MetaCommander>();
    world.init_resource::<Events<LoadingGameComplete>>();
    assert!(try_load_game_system(&mut world).is_err());
    assert!(fence.is_blocked());
    assert!(world.resource::<Events<LoadingGameComplete>>().is_empty());
    assert!(world.resource::<LoadGameManager>().options.is_some());
}
