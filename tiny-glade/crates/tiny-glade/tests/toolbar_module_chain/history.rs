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
    expected_circle: Option<serde_json::Value>,
}
impl HistoryHarness {
    fn new() -> Self {
        let mut input = Harness::new();
        history_runtime::register_history_events(&mut input.world);
        input.world.insert_resource(ReplayBuffer::new(1.0));
        input.world.insert_resource(InnerWalls::new());
        input.world.init_resource::<WallColorIds>();
        input.world.init_resource::<country_core::resources::walls::decorator::DecoratorIdGen>();
        input.world.init_resource::<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>();
        input.world.init_resource::<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        input.world.init_resource::<country_core::systems::collision::world::RaycastWorld>();
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
            |mut storage: ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>| storage.mark_mutable(),
            history_runtime::history_process_system::<MaybePendingSnapshotId>,
            history_runtime::replay_mode::replay_mode,
            replay_wall::replay_wall,
            history_runtime::replay_decorator::replay_decorator,
            replay_wall::apply_wall_replay_color,
            replay_wall::apply_wall_replay_audio,
            history_runtime::replay_pop_bucket_system,
            history_runtime::finish_replay::finish_replay,
        ).chain());
        Self { input, history, expected_history: None, expected_wall: None, expected_circle: None }
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
        // MorphPremadeShape has a real replay animation, unlike CreateWall's
        // instant prerequisite. Wait for its owners, with a bounded failure.
        for _ in 0..400 {
            if self.input.world.resource::<ReplayBuffer>().is_empty()
                && self.input.world.resource::<history_runtime::PendingHistoryCommands>().0.is_empty()
                && *self.input.world.resource::<AppMode>() != AppMode::Replay { break; }
            self.frame(false, position);
        }
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
        let circle_scene = self.expected_circle.is_some();
        assert_eq!(world.resource::<InnerWalls>().contains(WallId(1)), circle_scene || exists);
        let history = world.resource::<History>();
        assert_eq!(history.ledger().cursor, cursor);
        assert_eq!(history.ledger().entries.len(), if circle_scene { 3 } else { 1 }, "replay must not re-record");
        assert!(!history.is_recording());
        assert_eq!(&self.saved_history_contract(), self.expected_history.as_ref().unwrap(),
            "identity, undo boundary and every serialized edit field must survive replay/load");
        if circle_scene || exists {
            let wall = world.resource::<InnerWalls>().get_unwrapped(WallId(1));
            assert_eq!(&wall_authority(wall), self.expected_wall.as_ref().unwrap(),
                "all authority fields must match the originally committed wall");
            assert_eq!(world.resource::<WallColorIds>().0[1], 7,
                "the real replay color consumer must update its actual owner");
        }
        if let Some(expected) = &self.expected_circle {
            let state = if exists { &expected["after"] } else { &expected["before"] };
            assert_eq!(wall_authority(world.resource::<InnerWalls>().get_unwrapped(WallId(2))), state["wall"]);
            use country_core::resources::walls::{decorator::DecoratorId, decorator_storage::{DecoratorStorage, DecoratorStorageAddr}};
            let info = world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Wall(WallId(2))).unwrap().get(DecoratorId(101)).unwrap();
            assert_eq!(serde_json::to_value(country_core::resources::history::decorator_edit::DecoratorHistoryState::from(info)).unwrap(), state["decorator"]);
            assert_eq!(world.resource::<WallColorIds>().0[2], 9);
        }
    }
    /// Circle creation/decorator placement are explicit prerequisites. Only
    /// resize is under test: its history must come from the real widget chain.
    fn record_circle_resize(&mut self) {
        use country_core::{resources::{walls::{
            public_walls::{PublicWalls, WallEntities, WallPublicationDirty, WallSpatialHash},
            wall_trigger_volume::WallTriggerVolume,
            decorator::{DecoratorId, DecoratorInfo},
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr, Decorators},
            decorator_archivist::LiveDecoratorArchivist,
        }, history::decorator_edit::DecoratorHistoryState}, systems::{
            wall::{shapes::circle::ui_edit_circle::{self, AnimatedCircleSignifierResource}, valid_enclosure::{Shape, ShapeParameters}},
            ui::resources::widget_params::WidgetGeometry,
        }};
        let world = &mut self.input.world;
        let wall = WallId(2);
        let circle = ::utils::geometry::Circle2d::new_rotated(Vec2::new(2., 3.), 2., 0.4);
        // Original create_circle samples at0.2; UI resize samples at0.5.
        let authority = WallState::new(country_core::systems::wall::shapes::to_authority_curve(&circle.as_curve2(0.2)), 4., 0, 0, false);
        let mut snapshot = authority.snapshot().clone();
        snapshot.source.shape = ShapeParameters::from_shape(Shape::Circle(circle)).to_snapshot();
        snapshot.source.curve_kind_tag = 0;
        let authority = WallState::from_snapshot(snapshot, 0);
        let before_wall = wall_authority(&authority);
        let create = WallHistoryEdit::Create {
            wall: country_core::resources::walls::publication_fields::WallId(wall.0), source: authority.snapshot().clone(), color_id: 9,
            restored_height: 4., restored_elevation: 0.,
        };
        world.resource_mut::<InnerWalls>().new_wall_with_id(wall, authority);
        world.resource_mut::<WallColorIds>().0[2] = 9;
        world.resource_mut::<History>().start_input_recording(AppMode::Path.native_tag() as u32).unwrap();
        world.resource_mut::<History>().add(EditType::Wall(Arc::new(WallCompoundHistoryEdit { groups: vec![vec![create]] }))).unwrap();
        world.resource_mut::<DecoratorStorage>().mark_mutable();
        world.resource_mut::<LiveDecoratorArchivist>().archive_storage_decorators(0, &Decorators::default(), Some(wall));
        let mut info = DecoratorInfo::default();
        let anchor = circle.center + Vec2::X * circle.radius;
        system_decorator::decorator_on_wall_height_changed::set_anchor_ws(&mut info, glam::Vec3::new(anchor.x, 1.5, anchor.y));
        system_decorator::decorator_on_wall_height_changed::set_wall_coord(&mut info, ::utils::wall_space::WallSpaceCoord::new_unchecked(0., 1.5));
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            storage.get_or_insert_default(DecoratorStorageAddr::Wall(wall)).insert(DecoratorId(101), info);
            storage.decorator_to_storage.insert(DecoratorId(101), DecoratorStorageAddr::Wall(wall));
        }
        let before_decorator = serde_json::to_value(DecoratorHistoryState::from(&info)).unwrap();
        let mut finish = Schedule::default();
        finish.add_systems(history_runtime::recording::history_finish_recording_system::<MaybePendingSnapshotId>);
        world.send_event(history_runtime::recording::FinishInputRecordingCmd);
        finish.run(world);
        assert_eq!(world.resource::<History>().ledger().entries.len(), 2);
        world.resource_mut::<DecoratorStorage>().mark_mutable();

        world.init_resource::<PublicWalls>();
        world.init_resource::<WallEntities>();
        let terrain = world.resource::<WallPublicationTerrain>().0.clone();
        let paths = country_core::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(std::array::from_fn(|_| Arc::from(vec![0u8; 512 * 512])), true).unwrap();
        world.resource_scope(|world, mut public: Mut<PublicWalls>| {
            public.consume_with_paths(&mut world.resource_mut::<InnerWalls>(), &terrain, Some(&paths), &WallPublicationDirty::default(), &WallSpatialHash::default());
        });
        let mut trigger = WallTriggerVolume::default();
        trigger.recompute(world.resource::<PublicWalls>().get(wall).unwrap()).unwrap();
        let entity = world.spawn(trigger).id();
        world.resource_mut::<WallEntities>().0.insert(wall, entity);
        world.insert_resource(country_core::resources::glade_border::GladeBorder(country_core::resources::glade_border::BorderShape::Circle(::utils::geometry::Circle2d::new(Vec2::ZERO, 65.))));
        world.init_resource::<AnimatedCircleSignifierResource>();
        world.insert_resource(bevy_time::Time::<()>::default());
        world.init_resource::<Events<country_core::systems::camera_rig::main_camera::MainCameraMoved>>();
        world.init_resource::<Events<country_core::resources::glade_border::DisplayOutOfBorder>>();
        world.init_resource::<Events<country_core::resources::glade_border::FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<country_core::systems::wall::wall_highlight::HighlightWallCmd>>();
        world.init_resource::<Events<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>>();
        world.init_resource::<Events<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>>();
        world.init_resource::<Events<country_core::systems::audio::PlayBuildingAudio>>();
        world.init_resource::<Events<country_core::systems::audio::RequestWallMaterialAudio>>();
        world.init_resource::<Events<country_core::resources::tree_storage::HideTreesinAreaCmd>>();
        world.init_resource::<Events<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>>();
        world.insert_resource(AppMode::Editing(mode_manager::EditSubject::None));
        world.insert_resource(UiState::at_startup());
        world.insert_resource(WindowSize::new(1280, 720));
        let inverse = glam::Mat4::from_translation(glam::Vec3::new(3., 2., 10.));
        let mut camera = MainCamera::new(1280. / 720.);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &inverse.inverse());
        camera.write_matrix(0xc0, &inverse);
        world.insert_resource(camera);
        world.insert_resource(CursorPositionSS(Vec2::new(640., 360.)));
        world.insert_resource(MouseDepthValue { view_depth: 100., ..Default::default() });
        let mut resize = Schedule::default();
        resize.add_systems(ui_edit_circle::ui_edit_circle::<MaybePendingSnapshotId>);
        let mut picker = Schedule::default();
        picker.add_systems((hover_intersection, ui_update).chain_ignore_deferred());
        let mut start = Schedule::default();
        start.add_systems(history_runtime::recording::history_start_recording::<MaybePendingSnapshotId>);
        resize.run(world);
        let widget = WidgetId::from_name(&format!("edit_circle_wall_{wall}"));
        assert!(matches!(&world.resource::<UiState>().widget(widget).unwrap().geometry, Some(WidgetGeometry::TriangleMesh(tris)) if !tris.is_empty()));
        picker.run(world);
        assert_eq!(world.resource::<UiState>().active_widget(), widget);
        // ui_update consumes the frame registry. The producer must register
        // the next frame before the next real input edge, as in the host.
        resize.run(world);
        use country_core::input::action_set::DigitalActionPhase;
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::JustPressed;
        picker.run(world);
        assert_eq!(world.resource::<UiState>().widget_context(widget).dragged(), 3);
        start.run(world);
        assert!(world.resource::<History>().is_recording());
        resize.run(world);
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::Held;
        world.resource_mut::<GameTime>().delta = 0.1;
        world.resource_mut::<CursorPositionSS>().0.x = 1100.;
        picker.run(world);
        assert_eq!(world.resource::<UiState>().widget_context(widget).dragged(), 4);
        resize.run(world);
        assert_ne!(ShapeParameters::from_snapshot(world.resource::<InnerWalls>().get_unwrapped(wall).get_enclosure_params().unwrap()).as_circle().radius.to_bits(), circle.radius.to_bits());
        world.resource_scope(|world, mut public: Mut<PublicWalls>| {
            public.consume_with_paths(&mut world.resource_mut::<InnerWalls>(), &terrain, Some(&paths), &WallPublicationDirty::default(), &WallSpatialHash::default());
        });
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::JustReleased;
        picker.run(world);
        resize.run(world);
        assert!(world.resource::<History>().pending().unwrap().edits.iter().any(|edit| matches!(edit, EditType::Wall(edit) if matches!(&edit.groups[0][0], WallHistoryEdit::MorphPremadeShape(_)))));
        finish.run(world);
        let history = world.resource::<History>();
        assert_eq!(history.ledger().entries.len(), 3);
        assert!(!history.is_recording());
        assert!(history.ledger().entries[2].auxiliary.decorator_diff.as_ref().is_some_and(|diff| !diff.is_empty()));
        let after_authority = world.resource::<InnerWalls>().get_unwrapped(wall);
        let direct_after_wall = wall_authority(after_authority);
        assert_ne!(direct_after_wall["shape"], before_wall["shape"]);
        let after_shape = ShapeParameters::from_snapshot(after_authority.get_enclosure_params().unwrap());
        let ui_curve = country_core::systems::wall::shapes::to_authority_curve(&after_shape.as_circle().as_curve2(0.5));
        assert_eq!(after_authority.source().curve.points, ui_curve.points);
        assert_eq!(after_authority.source().curve.points_u, ui_curve.points_u);
        assert_eq!(after_authority.source().curve.length.to_bits(), ui_curve.length.to_bits());
        // Original Morph replay140ac01ff..21a explicitly samples at0.2
        // (1429cd068). Its payload records shapes, not the transient UI Vec.
        // Verify the complete replay authority, preserving every other field.
        let mut replay_snapshot = after_authority.snapshot().clone();
        replay_snapshot.source.curve = country_core::systems::wall::shapes::to_authority_curve(&after_shape.as_curve2());
        let after_wall = wall_authority(&WallState::from_snapshot(replay_snapshot, after_authority.freehand_ty()));
        let after_info = world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Wall(wall)).unwrap().get(DecoratorId(101)).unwrap();
        let after_decorator = serde_json::to_value(DecoratorHistoryState::from(after_info)).unwrap();
        assert_ne!(after_decorator, before_decorator);
        self.expected_circle = Some(serde_json::json!({
            "before": { "wall": before_wall, "decorator": before_decorator },
            "after": { "wall": after_wall, "decorator": after_decorator },
            "direct_ui_after": direct_after_wall,
        }));
        world.insert_resource(AppMode::Path);
        world.insert_resource(UiState::at_startup());
        world.insert_resource(WindowSize::new(960, 540));
        world.insert_resource(MainCamera::new(1.));
        world.insert_resource(ActionSetStates::default());
        world.resource_mut::<GameTime>().delta = 0.05;
        self.expected_history = Some(self.saved_history_contract());
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
    for scene in ["wall", "circle"] {
      let scene_root = root.path().join(scene);
      std::fs::create_dir_all(&scene_root).unwrap();
      for phase in ["write", "restore", "restore-applied"] {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "history::history_process_entry", "--nocapture"])
            .env("TG_TOOLBAR_HISTORY_PHASE", phase)
            .env("TG_TOOLBAR_HISTORY_SCENE", scene)
            .env("TG_TOOLBAR_HISTORY_ROOT", &scene_root)
            .status().unwrap();
        assert!(status.success(), "fresh history process failed during {scene}/{phase}");
        assert!(scene_root.join(format!("completed-{phase}")).is_file(), "child must execute its actual phase");
      }
    }
}

#[test]
fn history_process_entry() {
    let Ok(phase) = std::env::var("TG_TOOLBAR_HISTORY_PHASE") else { return; };
    let root = std::path::PathBuf::from(std::env::var_os("TG_TOOLBAR_HISTORY_ROOT").unwrap());
    paths::initialize_paths(0, Some(&root)).unwrap();
    let mut h = HistoryHarness::new();
    let circle_scene = std::env::var("TG_TOOLBAR_HISTORY_SCENE").as_deref() == Ok("circle");
    let applied_cursor = if circle_scene { 2 } else { 0 };
    let session_file = root.join("session-id.json");
    if phase == "write" {
        let session = SessionId::new();
        install_save_owner(&mut h.input.world, session);
        h.seed_committed_wall();
        if circle_scene { h.record_circle_resize(); }
        std::fs::write(&session_file, serde_json::to_vec(&serde_json::json!({
            "session": session,
            "history": h.expected_history.as_ref().unwrap(),
            "wall": h.expected_wall.as_ref().unwrap(),
            "circle": h.expected_circle.as_ref(),
        })).unwrap()).unwrap();
        h.settle();
        // The real UI producer's0.5 curve was checked before returning above;
        // all subsequent circle checks follow the original0.2 replay contract.
        if !circle_scene { h.assert_state(true, applied_cursor); }
        h.click(0);
        h.assert_state(false, applied_cursor - 1);
        h.click(1);
        h.assert_state(true, applied_cursor);
        h.click(0);
        h.assert_state(false, applied_cursor - 1);
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
        h.expected_circle = expected.get("circle").filter(|value| !value.is_null()).cloned();
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
        for _ in 0..400 {
            h.history.run(&mut h.input.world);
            if h.input.world.resource::<ReplayBuffer>().is_empty()
                && h.input.world.resource::<history_runtime::PendingHistoryCommands>().0.is_empty()
                && !matches!(*h.input.world.resource::<AppMode>(), AppMode::LoadGame | AppMode::Replay) { break; }
        }
        h.assert_state(phase == "restore-applied", if phase == "restore-applied" { applied_cursor } else { applied_cursor - 1 });
        h.settle();
        assert!(!matches!(*h.input.world.resource::<AppMode>(), AppMode::LoadGame | AppMode::Replay),
            "LoadingGameComplete must reach mode manager and finish_replay");
        if phase == "restore" {
            h.click(1);
            h.assert_state(true, applied_cursor);
            h.save();
        }
        h.click(0);
        h.assert_state(false, applied_cursor - 1);
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
