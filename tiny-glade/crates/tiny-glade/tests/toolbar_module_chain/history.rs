//! T6 CPU boundary: real startup history-button geometry and device edges ->
//! original input translation -> mode manager -> History -> live InnerWalls.
//! A committed CreateWall is a legal prerequisite fixture, not proof of wall-tool
//! restoration. Disk transfer uses the production save worker and complete
//! no-snapshot load state machine in fresh processes. Menu-save input, wall
//! publication/collision/GPU pixels and the window remain separate acceptance.
use super::*;
use country_core::{
    resources::{
        color_id::WallColorIds,
        glade_color_customization::GladeColorCustomization,
        glade_settings::GladeSettings,
        history::{ReplayBuffer, serialized::SavedMultiEdit, wall_edit::{WallCompoundHistoryEdit, WallHistoryEdit}},
        roofs::geometry::surface::Curve2,
        session::{ActiveSession, SessionId, SessionMeta},
        time::GameTime,
        time_of_day::TimeOfDay,
        walls::{InnerWalls, WallId, WallState, publication_source::TerrainHeightGrid512},
    },
    systems::{
        history::{self as history_runtime, replay_wall},
        save::{
            load_game::{self, ActiveLoadOptions, LoadGameManager, LoadGameSource, LoadGameState},
            save_game::{self, AsyncSaveCmd, AutoSaveState, SerializedJson},
            save_thread::{SaveThreadHandle, shutdown_save_thread},
        },
        wall::wall_state::publish_public_wall_state::WallPublicationTerrain,
    },
};
use std::sync::Arc;
use toolbar::resouces_and_components::HistoryButton;

struct HistoryHarness {
    input: Harness,
    history: Schedule,
    expected_history: Option<serde_json::Value>,
    expected_wall: Option<serde_json::Value>,
}
impl HistoryHarness {
    fn new() -> Self {
        let mut input = Harness::new();
        history_runtime::register_history_events(&mut input.world);
        input.world.insert_resource(ReplayBuffer::new(1.0));
        input.world.insert_resource(InnerWalls::new());
        input.world.init_resource::<WallColorIds>();
        input.world.init_resource::<country_core::meta_commander::MetaCommander>();
        input.world.init_resource::<history_runtime::replay_mode::ReplayModeElapsedTime>();
        input.world.insert_resource(GameTime { delta: 0.05, ran: true, ..Default::default() });
        input.world.insert_resource(WallPublicationTerrain(TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0., maximum_y: 4., sampling_fix: true,
        }));
        // Alignment is the actual layout producer; no hand-built button extent.
        input.schedule.add_systems((
            toolbar::undo_redo_systems::undo_redo_auto_hide,
            toolbar::undo_redo_systems::undo_redo_alignment,
            toolbar::ui_interaction::ui_select_animation,
        ).chain().before(toolbar::taffy_update::taffy_perform_layout)
            .after(render_taffy_ui_to_widgets).in_set(InputSystemSet::Consume));
        let mut history = Schedule::default();
        history.add_systems((
            history_runtime::history_process_system::<MaybePendingSnapshotId>,
            history_runtime::replay_mode::replay_mode,
            replay_wall::replay_wall,
            replay_wall::apply_wall_replay_color,
            replay_wall::apply_wall_replay_audio,
            history_runtime::replay_pop_bucket_system,
            history_runtime::finish_replay::finish_replay,
        ).chain());
        Self { input, history, expected_history: None, expected_wall: None }
    }
    fn frame(&mut self, held: bool, position: Vec2) {
        self.input.frame(held, position);
        self.history.run(&mut self.input.world);
    }
    fn settle(&mut self) {
        for _ in 0..24 { self.frame(false, Vec2::ZERO); }
    }
    fn click(&mut self, kind: u8) {
        let (entity, position) = self.input.world.query::<(Entity, &HistoryButton, &Ui2dExtent)>()
            .iter(&self.input.world).find(|(_, button, _)| button.kind == kind)
            .map(|(entity, _, extent)| { assert!(extent.size.min_element() > 0.); (entity, extent.pos) })
            .expect("startup owns a real history button");
        assert!(position.x >= 0. && position.x < 960. && position.y >= 0. && position.y < 540.);
        self.frame(false, position);
        let node = self.input.world.get::<TaffyNode>(entity).unwrap().node;
        assert_eq!(self.input.world.resource::<UiState>().active_widget(),
            WidgetId::from_name(&format!("toolbar_button_{node:?}")));
        self.frame(true, position);
        // UiHistoryButtonPressed is consumed by translate_input next frame.
        self.frame(false, position);
        for _ in 0..12 { self.frame(false, position); }
        assert!(self.input.world.get::<UiPlaySelectAnimation>(entity).is_none(),
            "production pulse must finish and remove its component");
        assert_ne!(*self.input.world.resource::<AppMode>(), AppMode::Replay);
        assert!(self.input.world.resource::<ReplayBuffer>().is_empty());
        assert!(self.input.world.resource::<Events<history_runtime::HistoryReplayFailed>>().is_empty());
    }
    fn seed_committed_wall(&mut self) {
        let wall = WallState::new(Curve2::try_new(vec![[0., 0.], [4., 0.]], false).unwrap(), 2., 0, 0, false);
        let edit = WallHistoryEdit::Create {
            wall: country_core::resources::walls::publication_fields::WallId(1), source: wall.snapshot().clone(), color_id: 7,
            restored_height: 2., restored_elevation: 0.,
        };
        self.expected_wall = Some(wall_authority(&wall));
        self.input.world.resource_mut::<InnerWalls>().new_wall_with_id(WallId(1), wall);
        self.input.world.resource_mut::<WallColorIds>().0[1] = 7;
        let mut history = self.input.world.resource_mut::<History>();
        history.start_input_recording(AppMode::Path.native_tag() as u32).unwrap();
        history.add(EditType::Wall(Arc::new(WallCompoundHistoryEdit { groups: vec![vec![edit]] }))).unwrap();
        history.finish_input_recording().unwrap().expect("real nonempty history commit");
        drop(history);
        self.expected_history = Some(self.saved_history_contract());
    }
    fn saved_history_contract(&self) -> serde_json::Value {
        let history = self.input.world.resource::<History>();
        let edits: Vec<_> = history.ledger().entries.iter().cloned()
            .map(SavedMultiEdit).map(|entry| serde_json::to_value(entry).unwrap()).collect();
        serde_json::json!({
            "identity": history.identity(),
            "undo_anchor": history.ledger().undo_anchor,
            "edits": edits,
        })
    }
    fn assert_state(&self, exists: bool, cursor: i32) {
        let world = &self.input.world;
        assert_eq!(world.resource::<InnerWalls>().contains(WallId(1)), exists);
        let history = world.resource::<History>();
        assert_eq!(history.ledger().cursor, cursor);
        assert_eq!(history.ledger().entries.len(), 1, "replay must not re-record");
        assert!(!history.is_recording());
        assert_eq!(&self.saved_history_contract(), self.expected_history.as_ref().unwrap(),
            "identity, undo boundary and every serialized edit field must survive replay/load");
        if exists {
            let wall = world.resource::<InnerWalls>().get_unwrapped(WallId(1));
            assert_eq!(&wall_authority(wall), self.expected_wall.as_ref().unwrap(),
                "all authority fields must match the originally committed wall");
            assert_eq!(world.resource::<WallColorIds>().0[1], 7,
                "the real replay color consumer must update its actual owner");
        }
    }
    fn save(&mut self) {
        let world = &mut self.input.world;
        let history = world.resource::<History>();
        let sections = save_game::assemble_game_save_sections(
            load_game::LATEST_SAVE_VERSION, history.identity(),
            &country_core::systems::camera_rig::CameraRigParametersCache::default(),
            &GladeColorCustomization::default(),
            SerializedJson::new(&TimeOfDay::default()).unwrap(),
            SerializedJson::new(&Option::<country_core::resources::critters::sheep::control::SheepControlSnapshot>::None).unwrap(),
            SerializedJson::new(&history.ledger().cursor).unwrap(),
            SerializedJson::new(&history.ledger().undo_anchor).unwrap(),
            SerializedJson::new(&Vec::<serde_json::Value>::new()).unwrap(),
        );
        let edits = history.ledger().entries.iter().cloned().map(SavedMultiEdit)
            .map(|edit| SerializedJson::new(&edit).unwrap()).collect();
        let session = world.resource::<ActiveSession>().id;
        let mut auto = world.resource_mut::<AutoSaveState>();
        auto.send_command_expect(AsyncSaveCmd::OpenFile { session, sections, edits });
        auto.flush_synchronously().unwrap();
    }
}

// Inspect the complete portable state plus original authority fields that are
// not all represented by the save schema; CPU equality is not GPU publication.
fn wall_authority(wall: &WallState) -> serde_json::Value {
    let snapshot = wall.snapshot();
    let source = &snapshot.source;
    serde_json::json!({
        "saved": serde_json::to_value(wall).unwrap(),
        "points": source.curve.points,
        "points_u": source.curve.points_u,
        "length": source.curve.length,
        "curve_kind_tag": source.curve_kind_tag,
        "terrain_floor": source.terrain_floor,
        "shape": source.shape.bytes(),
        "style": source.style,
        "extra": source.extra,
        "special": source.special,
        "observed_path_scalar": source.observed_path_scalar,
        "clutter_version": source.clutter_version,
        "observed_source_flag_64": source.observed_source_flag_64,
        "source_60_bits": snapshot.metadata.source_60_bits,
        "source_66": snapshot.metadata.source_66,
        "source_67": snapshot.metadata.source_67,
        "source_69": snapshot.metadata.source_69,
        "height": wall.height(),
        "elevation": wall.elevation(),
    })
}

fn install_save_owner(world: &mut World, session: SessionId) {
    // startup_session creates this directory before publishing ActiveSession.
    // This fixture starts with an already-created session, so supply the same
    // filesystem prerequisite; save worker must still serialize and sync it.
    std::fs::create_dir_all(session.save_folder_path()).unwrap();
    world.insert_resource(ActiveSession::new(session, SessionMeta::default()));
    let mut auto = AutoSaveState::default();
    auto.session = session;
    let worker = SaveThreadHandle::spawn_with_unsaved(auto.unsaved_files.clone());
    auto.attach_command_sender(worker.command_sender.clone());
    world.insert_resource(auto);
    world.insert_resource(worker);
}

#[test]
fn real_history_buttons_roundtrip_disk_in_fresh_processes() {
    let root = tempfile::tempdir().unwrap();
    for phase in ["write", "restore", "restore-applied"] {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "history::history_process_entry", "--nocapture"])
            .env("TG_TOOLBAR_HISTORY_PHASE", phase)
            .env("TG_TOOLBAR_HISTORY_ROOT", root.path())
            .status().unwrap();
        assert!(status.success(), "fresh history process failed during {phase}");
        assert!(root.path().join(format!("completed-{phase}")).is_file(), "child must execute its actual phase");
    }
}

#[test]
fn history_process_entry() {
    let Ok(phase) = std::env::var("TG_TOOLBAR_HISTORY_PHASE") else { return; };
    let root = std::path::PathBuf::from(std::env::var_os("TG_TOOLBAR_HISTORY_ROOT").unwrap());
    paths::initialize_paths(0, Some(&root)).unwrap();
    let mut h = HistoryHarness::new();
    let session_file = root.join("session-id.json");
    if phase == "write" {
        let session = SessionId::new();
        install_save_owner(&mut h.input.world, session);
        h.seed_committed_wall();
        std::fs::write(&session_file, serde_json::to_vec(&serde_json::json!({
            "session": session,
            "history": h.expected_history.as_ref().unwrap(),
            "wall": h.expected_wall.as_ref().unwrap(),
        })).unwrap()).unwrap();
        h.settle();
        h.assert_state(true, 0);
        h.click(0);
        h.assert_state(false, -1);
        h.click(1);
        h.assert_state(true, 0);
        h.click(0);
        h.assert_state(false, -1);
        // Save at the undo cursor: the future edit must remain available.
        h.save();
        shutdown_save_thread(&mut h.input.world).unwrap();
        assert!(session.save_folder_path().join("history.json").is_file());
    } else {
        assert!(phase == "restore" || phase == "restore-applied");
        let expected: serde_json::Value = serde_json::from_slice(&std::fs::read(session_file).unwrap()).unwrap();
        let session: SessionId = serde_json::from_value(expected["session"].clone()).unwrap();
        h.expected_history = Some(expected["history"].clone());
        h.expected_wall = Some(expected["wall"].clone());
        install_save_owner(&mut h.input.world, session);
        h.input.world.init_resource::<TimeOfDay>();
        h.input.world.init_resource::<GladeSettings>();
        let mut manager = LoadGameManager::new();
        manager.begin(LoadGameSource::Path(session.save_folder_path().join("history.json")),
            ActiveLoadOptions { ignore_snapshot: false, request_reveal: false, new_session: false },
            country_core::startup::startup_session::make_post_load_commands(vec![]));
        h.input.world.insert_resource(manager);
        h.input.world.insert_resource(AppMode::LoadGame);
        // Exercise complete production Loading -> ApplyCounter -> Loaded.
        for _ in 0..3 { load_game::try_load_game_system(&mut h.input.world).unwrap(); h.history.run(&mut h.input.world); }
        assert_eq!(h.input.world.resource::<LoadGameManager>().state, LoadGameState::Loaded);
        // Loading an applied cursor must replay the real saved CreateWall;
        // loading an undone cursor must retain its future entry for real redo.
        for _ in 0..24 { h.history.run(&mut h.input.world); }
        h.assert_state(phase == "restore-applied", if phase == "restore-applied" { 0 } else { -1 });
        h.settle();
        assert!(!matches!(*h.input.world.resource::<AppMode>(), AppMode::LoadGame | AppMode::Replay),
            "LoadingGameComplete must reach mode manager and finish_replay");
        if phase == "restore" {
            h.click(1);
            h.assert_state(true, 0);
            h.save();
        }
        h.click(0);
        h.assert_state(false, -1);
        shutdown_save_thread(&mut h.input.world).unwrap();
    }
    std::fs::write(root.join(format!("completed-{phase}")), b"passed").unwrap();
}

#[test]
fn history_press_before_reload_does_not_undo_the_reentered_session() {
    use bevy_state::state::OnExit;
    use country_core::app_loading_state::AppLoadingState;
    use country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed;
    let mut h = HistoryHarness::new();
    h.seed_committed_wall();
    h.settle();
    let (old_button, position) = h.input.world.query::<(Entity, &HistoryButton, &Ui2dExtent)>()
        .iter(&h.input.world).find(|(_, button, _)| button.kind == 0)
        .map(|(entity, _, extent)| (entity, extent.pos)).unwrap();
    h.frame(false, position);
    h.frame(true, position);
    // Real render_taffy callback has produced the deferred input; next frame's
    // translate has not run, so this specifically covers the dangerous window.
    let buttons = h.input.world.resource::<Events<UiHistoryButtonPressed>>();
    assert_eq!(buttons.get_cursor().read(buttons).map(|button| button.0).collect::<Vec<_>>(), [0]);
    h.assert_state(true, 0);
    let mut exit = Schedule::new(OnExit(AppLoadingState::Game));
    exit.add_systems((tiny_glade::toolbar_plugin::clear_toolbar_session,
        country_core::startup::clearing::despawn_glade_entities).chain());
    h.input.world.add_schedule(exit);
    h.input.world.run_schedule(OnExit(AppLoadingState::Game));
    h.input.world.flush();
    assert!(h.input.world.get_entity(old_button).is_err());
    assert!(h.input.world.resource::<Events<UiHistoryButtonPressed>>().is_empty());
    let mut enter = Schedule::default();
    enter.add_systems(toolbar::toolbar_startup::toolbar_setup);
    enter.run(&mut h.input.world);
    h.input.world.flush();
    // Keep the physical button held across reentry before releasing. Neither
    // stale deferred input nor old captured ownership may become an Undo.
    for _ in 0..24 { h.frame(true, Vec2::ZERO); }
    h.frame(false, Vec2::ZERO);
    h.assert_state(true, 0);
    assert!(h.input.world.resource::<Events<HistoryCommand>>().is_empty());
    h.click(0);
    h.assert_state(false, -1);
}
