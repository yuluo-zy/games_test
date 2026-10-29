//! Real session-control integration through parsed ownership and the save worker.
//! Real original nonempty assets + MEI keyboard + state transition + async save.
//! Stops at actual HistoryCmd/LoadingGameComplete publication: domain replay,
//! full unload texture/device/audio side effects and presented GPU remain separate gates.
use bevy_app::{App, PostUpdate, Update};
use bevy_ecs::{prelude::*, schedule::IntoScheduleConfigs};
use bevy_input::mouse::MouseWheel;
use bevy_state::{
    app::{AppExtStates, StatesPlugin},
    state::{NextState, OnEnter, OnExit, State},
};
use bevy_time::Time;
use country_core::{
    app_loading_state::AppLoadingState,
    game_config::GameConfig,
    game_profile::GameProfile,
    input::action_set::{DigitalActionPhase, DigitalActionState, MainActionSet},
    meta_commander::MetaCommander,
    resources::{
        glade_settings::GladeSettings,
        gui::{EscMenuState, LoadMenuState, NextEscMenuState},
        history::History,
        mouse_terrain_raycast::CursorPositionSS,
        session::{
            ActiveSession, NextSession, NextSessionKind, SavedSessionList, SessionId, SessionMeta,
        },
        time_of_day::TimeOfDay,
        walls::InnerWalls,
        window::WindowSize,
    },
    startup::{clearing::begin_glade_unload, startup_session::load_game_session},
    systems::{
        audio::PlayMenuUiSound,
        camera_rig::main_camera::MainCamera,
        mode_manager::{AppMode, ClearCanvasCmd, HistoryCommand},
        save::{
            add_save_load_events_and_resources,
            load_game::{
                LoadGameManager, LoadGameSource, LoadGameState, LoadingGameComplete,
                load_game_system,
            },
            save_game::{
                AsyncSaveCmd, AutoSaveState, DeleteSessionCmd, SaveFileSection, defer_json,
            },
            save_thread::SaveThreadHandle,
        },
        ui::resources::{
            ui_state::UiState,
            widget_params::{UiFocus, UiNavState},
        },
    },
};
use glam::Vec2;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};
use toolbar::{
    MeiContext,
    menu::{load::ThumbnailScrollContent, load_ui},
};
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stage(u8);
#[derive(Resource, Default)]
struct SourceReceipts(Vec<(SessionId, bool, usize, usize)>);
fn observe_source(
    active: Res<ActiveSession>,
    manager: Res<LoadGameManager>,
    mut receipt: ResMut<SourceReceipts>,
) {
    match &manager.source {
        LoadGameSource::Parsed { data, .. } => receipt.0.push((
            active.id,
            true,
            data.edits.len(),
            data.edits.as_ptr() as usize,
        )),
        LoadGameSource::Path(_) => receipt.0.push((active.id, false, 0, 0)),
        LoadGameSource::None => panic!("OnEnter must publish an actual load owner"),
    }
}
fn app(installed: &Path) -> App {
    let mut mei = MeiContext::default();
    mei.font_table = Arc::new(
        tg_text::FontTable::from_ron(
            &std::fs::read_to_string(installed.join("assets/fonts/table.ron")).unwrap(),
        )
        .unwrap(),
    );
    for name in [
        "menu/back",
        "menu/delete_warning",
        "menu/switch_glade",
        "menu/load_backup",
        "menu/backup_load_error",
        "other/hint_text_centered",
    ] {
        mei.configs.insert(
            name.into(),
            Arc::new(
                toolbar::mei::config::read_node_config_from_file(
                    &installed.join(format!("assets/ui/{name}.ron")),
                )
                .unwrap(),
            ),
        );
    }
    let mut a = App::new();
    a.add_plugins(StatesPlugin).init_state::<AppLoadingState>();
    add_save_load_events_and_resources(a.world_mut());
    country_core::resources::session::SessionFlowPlugin.build(a.world_mut());
    a.add_event::<MouseWheel>()
        .add_event::<PlayMenuUiSound>()
        .add_event::<DeleteSessionCmd>()
        .add_event::<country_core::resources::gui::EscapeMenuJustClosed>()
        .init_resource::<country_core::resources::gui::PrevEscMenuState>()
        .add_event::<ClearCanvasCmd>()
        .add_event::<HistoryCommand>()
        .add_event::<country_core::resources::history::tree_edit::SpawnPreplacedTrees>()
        .insert_resource(mei)
        .insert_resource(History::<
            country_core::resources::history::MaybePendingSnapshotId,
        >::empty_glade())
        .insert_resource(EscMenuState::Closed)
        .insert_resource(NextEscMenuState::default())
        .insert_resource(GameConfig::default())
        .insert_resource(GameProfile::default())
        .insert_resource(GladeSettings::default())
        .insert_resource(MetaCommander::default())
        .insert_resource(AppMode::default())
        .insert_resource(TimeOfDay::default())
        .insert_resource(InnerWalls::default())
        .insert_resource(MainCamera::new(1.6))
        .insert_resource(MainActionSet::default())
        .insert_resource(UiNavState::default())
        .insert_resource(UiState::at_startup())
        .insert_resource(WindowSize::new(1600, 1000))
        .insert_resource(CursorPositionSS(Vec2::splat(500.)))
        .insert_resource(Time::<()>::default())
        .init_resource::<SourceReceipts>();
    a.configure_sets(Update, (Stage(6), Stage(12), Stage(36), Stage(39)).chain());
    a.add_systems(
        Update,
        toolbar::mei::systems::mei_pre_update.in_set(Stage(6)),
    );
    a.add_systems(
        Update,
        load_game_system
            .in_set(Stage(12))
            .run_if(bevy_state::condition::in_state(AppLoadingState::Game)),
    );
    a.add_systems(
        Update,
        load_ui::ui
            .in_set(Stage(36))
            .run_if(bevy_state::condition::in_state(AppLoadingState::Game)),
    );
    a.add_systems(
        Update,
        country_core::resources::gui::advance_menu_state_system.in_set(Stage(39)),
    );
    a.add_systems(
        OnEnter(AppLoadingState::Game),
        (load_game_session, observe_source).chain(),
    );
    // This is the genuine native beginning of unload. Other complete GPU/audio
    // unload consumers are independently validated in the host integration.
    a.add_systems(OnExit(AppLoadingState::Game), begin_glade_unload);
    a.add_systems(PostUpdate, tiny_glade::app_systems::advance_reload_state);
    a
}
fn step(a: &mut App) {
    *a.world_mut().resource_mut::<MainActionSet>() = MainActionSet::default();
    a.update();
}
fn press(a: &mut App, slot: usize) {
    *a.world_mut().resource_mut::<MainActionSet>() = MainActionSet::default();
    a.world_mut().resource_mut::<MainActionSet>().digital[slot] = DigitalActionState {
        phase: DigitalActionPhase::JustPressed,
        timer_seconds: 0.,
    };
    a.update();
}
fn select_second(a: &mut App) {
    let id = a
        .world()
        .resource::<MeiContext>()
        .nodes
        .iter()
        .find_map(|(id, n)| {
            let any: &dyn std::any::Any = n.content.as_ref();
            any.downcast_ref::<ThumbnailScrollContent>().map(|_| *id)
        })
        .unwrap();
    // Actual retained MEI keyboard-focus owner; no UiInteraction is seeded.
    {
        let mut nav = a.world_mut().resource_mut::<UiNavState>();
        nav.focus = UiFocus {
            tag: 2,
            _pad: 0,
            payload: id.0,
        };
        nav.hold = true;
    }
    press(a, 21);
    press(a, 21);
    press(a, 19);
}
fn complete_load_publication(a: &mut App) {
    assert_eq!(
        a.world().resource::<LoadGameManager>().state,
        LoadGameState::ApplyCounter
    );
    step(a); // native state1 emits signed history delta then advances to2
    let commands: Vec<_> = a
        .world_mut()
        .resource_mut::<Events<HistoryCommand>>()
        .drain()
        .collect();
    assert_eq!(commands.len(), 26);
    assert!(commands.iter().all(|c| c.bytes == [1, 1, 1, 3]));
    step(a); // native state2 publishes actual LoadingGameComplete
    assert!(a.world().resource::<LoadGameManager>().options.is_none());
    assert_eq!(
        a.world_mut()
            .resource_mut::<Events<LoadingGameComplete>>()
            .drain()
            .count(),
        1
    );
    a.world_mut()
        .resource_mut::<AutoSaveState>()
        .flush_synchronously()
        .unwrap();
}
#[test]
fn real_load_menu_parsed_owner_survives_missing_primary_across_two_session_state_transitions() {
    let installed = std::fs::canonicalize(PathBuf::from(
        std::env::var_os("TG_INSTALL_ROOT").expect("set TG_INSTALL_ROOT"),
    ))
    .unwrap();
    let account = tempfile::tempdir().unwrap();
    paths::initialize_paths_with_installation(91913, Some(account.path()), &installed).unwrap();
    let previous = std::env::current_dir().unwrap();
    std::env::set_current_dir(&installed).unwrap();
    let fixture = installed.join("assets/starting-builds/full/15-lake-house/history.json");
    let original: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&fixture).unwrap()).unwrap();
    assert_eq!(original["Version"], 27);
    assert_eq!(
        original["History"]["edit_chain"].as_array().unwrap().len(),
        26
    );
    let ids = [SessionId::new(), SessionId::new()];
    for (i, id) in ids.iter().enumerate() {
        std::fs::create_dir_all(id.save_folder_path()).unwrap();
        std::fs::copy(&fixture, id.save_folder_path().join("history.json")).unwrap();
        SessionMeta::default().write_to_disk(*id).unwrap();
        std::fs::OpenOptions::new()
            .write(true)
            .open(id.save_folder_path().join("history.json"))
            .unwrap()
            .set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(1_750_000_000 + i as u64))
            .unwrap();
        // Deliberately absent snapshot file exercises the real no-valid-snapshot
        // fallback to ALL26 original edits; no fake empty snapshot is inserted.
    }
    let listed = country_core::resources::session::load_saved_session_list();
    assert_eq!(listed.len(), 2);
    let first = listed[0].id;
    let second = listed[1].id;
    let mut a = app(&installed);
    a.insert_resource(SavedSessionList { sessions: listed });
    assert_eq!(
        a.world().resource::<NextSession>().kind,
        NextSessionKind::LoadSavedGame
    );
    // Actual host SessionFlowPlugin produced this initial request; no CLI or
    // manually inserted initial NextSession is needed for ordinary startup.
    a.world_mut()
        .resource_mut::<NextState<AppLoadingState>>()
        .set(AppLoadingState::Game);
    step(&mut a);
    assert_eq!(a.world().resource::<ActiveSession>().id, first);
    assert_eq!(
        a.world().resource::<SourceReceipts>().0,
        [(first, false, 0, 0)]
    );
    complete_load_publication(&mut a);
    assert_eq!(a.world().resource::<History>().ledger().entries.len(), 26);
    // A real deferred section remains queued in the active first file. Native
    // CloseFile must flush it before the following OpenFile replaces that owner.
    a.world_mut().resource_mut::<AutoSaveState>().send_command_expect(AsyncSaveCmd::ChangeSection{section:SaveFileSection::TimeOfDay,value:defer_json(serde_json::json!({"time_of_day":0.625,"sun_xz_rotation":0.,"latitude_degrees":59.3293}))});
    a.world_mut()
        .insert_resource(EscMenuState::Load(LoadMenuState::default()));
    step(&mut a);
    select_second(&mut a);
    let request = a.world().resource::<NextSession>();
    assert_eq!(request.kind, NextSessionKind::LoadParsedGame);
    assert_eq!(request.id, second);
    let parsed_pointer = request
        .parsed_data()
        .expect("UI must retain its real parser result")
        .edits
        .as_ptr() as usize;
    assert!(matches!(
        a.world().resource::<NextState<AppLoadingState>>(),
        NextState::Pending(AppLoadingState::Reload)
    ));
    // UI already owns the parsed nonempty history. Neither startup nor load
    // may reopen the primary path; this triggers the exact previous regression.
    std::fs::remove_file(second.save_folder_path().join("history.json")).unwrap();
    step(&mut a);
    assert_eq!(
        *a.world().resource::<State<AppLoadingState>>().get(),
        AppLoadingState::Reload
    );
    assert_eq!(a.world().resource::<ActiveSession>().id, first);
    assert!(a.world().contains_resource::<NextSession>());
    assert_eq!(
        a.world_mut()
            .resource_mut::<Events<ClearCanvasCmd>>()
            .drain()
            .map(|e| e.0)
            .collect::<Vec<_>>(),
        [false]
    );
    step(&mut a);
    assert_eq!(
        *a.world().resource::<State<AppLoadingState>>().get(),
        AppLoadingState::Game
    );
    assert_eq!(a.world().resource::<ActiveSession>().id, second);
    assert!(!a.world().contains_resource::<NextSession>());
    assert_eq!(
        a.world().resource::<SourceReceipts>().0,
        [(first, false, 0, 0), (second, true, 26, parsed_pointer)]
    );
    complete_load_publication(&mut a);
    assert_eq!(a.world().resource::<History>().ledger().entries.len(), 26);
    let old: serde_json::Value = serde_json::from_slice(
        &std::fs::read(first.save_folder_path().join("history.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(old["Time of day"]["time_of_day"], 0.625);
    let new: serde_json::Value = serde_json::from_slice(
        &std::fs::read(second.save_folder_path().join("history.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(new["Identity"], original["Identity"]);
    assert_eq!(new["History"]["edit_chain"].as_array().unwrap().len(), 26);
    // Disconnect all actual senders and JOIN the real worker before deleting
    // account storage. This observes the worker's real final flush branch.
    let auto = a.world_mut().remove_resource::<AutoSaveState>().unwrap();
    let SaveThreadHandle {
        join,
        command_sender,
        error_state,
    } = a.world_mut().remove_resource::<SaveThreadHandle>().unwrap();
    drop(auto);
    drop(command_sender);
    join.join().unwrap();
    drop(error_state);
    drop(a);
    std::env::set_current_dir(previous).unwrap();
}

#[test]
fn floor_owner_survives_schedule_across_real_game_reload_transition() {
    use bevy_ecs::schedule::ApplyDeferred;
    use country_core::{
        asset_libraries::{audio_trigger_library::AudioTriggerLibrary, placement_data_library::AssetPlacementDataLibrary},
        components::drawable::NaniBundle,
        render::{test_device::TestDevice, nani::{gpu_world::{GpuWorld, GpuWorldSubset, GpuWorldSubsetKind, InstanceKey, OwnedGpuWorldSubset}, instance_data::StoneFloorInstanceData, systems::{nani_maintain_instances_ecs, nani_update_roof_floor_for_roofs_ecs}}},
        resources::{audio_resources::{AudioCore, DisabledStudio}, rt::RtWorld, tree_storage::InteractableForestPlacements,
            roofs::geometry::surface::Curve2,
            walls::{Floor, WallState, public_walls::{OnWallChanged, OnWallDeleted, PublicWalls, WallPublicationDirty, WallSpatialHash}, publication_source::TerrainHeightGrid512},
            render::mesh_atlas_library::MeshAtlasLibrary},
        startup::clearing::{uninit_clearing, despawn_glade_entities},
        systems::{audio::game_loading_audio::GameLoadingAudioState, wall::valid_enclosure::{Shape, ShapeParameters}},
    };
    use system_wall_constructor::construct_floors::construct_floors;
    let device = TestDevice(1);
    let mut gpu = GpuWorld::new(&device);
    gpu.assign_subset(GpuWorldSubsetKind::RoofFloor, GpuWorldSubset::Owned(
        OwnedGpuWorldSubset::new::<StoneFloorInstanceData>(&device, GpuWorldSubsetKind::RoofFloor, 0)));
    let mut inner = InnerWalls::new();
    let id = inner.new_wall(WallState::new(Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap(), 7.0, 0, 0, false));
    inner.update_wall_state_enclosure(id, ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d {
        center: Vec2::new(1.0, 2.0), radius: 3.0, rotation: 0.0,
    })).to_snapshot());
    inner.update_elevation(id, 4.0);
    let terrain = TerrainHeightGrid512 { encoded: vec![0u16;512*512].into(), minimum_y:0.0, maximum_y:0.0, sampling_fix:false };
    let mut walls = PublicWalls::default();
    walls.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
    let core = AudioCore::<DisabledStudio> { studio: None, banks: Default::default() };
    let loading = GameLoadingAudioState::new(&core);
    let mut app = App::new();
    app.add_plugins(StatesPlugin).init_state::<AppLoadingState>();
    app.add_event::<OnWallChanged>().add_event::<OnWallDeleted>().add_event::<ClearCanvasCmd>();
    app.insert_resource(walls).insert_resource(gpu).insert_resource(MeshAtlasLibrary::default());
    app.insert_resource(AssetPlacementDataLibrary::new()).insert_resource(AudioTriggerLibrary::default());
    app.insert_resource(RtWorld::default()).insert_resource(InteractableForestPlacements::default());
    let (sender, _receiver) = crossbeam_channel::unbounded();
    let mut autosave = AutoSaveState::default();
    autosave.attach_command_sender(sender);
    app.insert_resource(core).insert_resource(loading).insert_resource(autosave);
    app.add_systems(Update, (construct_floors, ApplyDeferred, nani_maintain_instances_ecs, ApplyDeferred, nani_update_roof_floor_for_roofs_ecs).chain().run_if(bevy_state::condition::in_state(AppLoadingState::Game)));
    app.add_systems(OnExit(AppLoadingState::Game), (begin_glade_unload, (uninit_clearing::<DisabledStudio>, despawn_glade_entities)).chain());
    app.add_systems(PostUpdate, tiny_glade::app_systems::advance_reload_state);
    app.world_mut().resource_mut::<NextState<AppLoadingState>>().set(AppLoadingState::Game);
    app.update();
    app.world_mut().resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
    app.update();
    let (old, key) = app.world_mut().query::<(Entity, &Floor, &InstanceKey)>().iter(app.world()).map(|(e, _, k)| (e, *k)).next().unwrap();
    assert_eq!(app.world().resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::RoofFloor).instance_count(), 1);
    app.world_mut().resource_mut::<NextState<AppLoadingState>>().set(AppLoadingState::Reload);
    app.update();
    assert!(app.world().get_entity(old).is_err());
    assert_eq!(app.world().resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::RoofFloor).instance_count(), 0);
    app.update();
    assert_eq!(*app.world().resource::<State<AppLoadingState>>().get(), AppLoadingState::Game);
    app.world_mut().resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
    app.update();
    let (new, new_key) = app.world_mut().query::<(Entity, &Floor, &InstanceKey)>().iter(app.world()).map(|(e, _, k)| (e, *k)).next().unwrap();
    assert_ne!(old, new);
    assert_ne!(key.id, new_key.id);
    assert_eq!(app.world().resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::RoofFloor).instance_count(), 1);
    let _ = std::marker::PhantomData::<NaniBundle<StoneFloorInstanceData>>;
}

#[test]
fn m06_collision_subscribers_follow_real_game_reload_and_wall_producer() {
    use bevy_app::Startup;
    use country_core::{
        geometry::instanced_wall::InstancedWall,
        resources::{
            roofs::geometry::surface::Curve2,
            terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
            walls::{WallState, arches::{ArchSegments, LintelSegments},
                public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash},
                publication_source::TerrainHeightGrid512,
                wall_trigger_volume::{Quad, WallTriggerVolume}},
        },
        systems::{collision::{intersect_meshes::{initialize_mesh_intersection_subscriber, MeshMeshIntersectionSubscriber},
            world::{flush_rapier_colliders, update_rapier_colliders, RaycastWorld}},
            wall::construction::bricks::WallBrickInstance},
    };
    use system_wall_constructor::construct_elevation_supports::{OverhangSupportDirtySubscriptions,
        overhang_subscribe_to_tabloids};
    use glam::Vec3;

    let mut app = App::new();
    app.add_plugins(StatesPlugin).init_state::<AppLoadingState>();
    app.init_resource::<OverhangSupportDirtySubscriptions>();
    app.insert_resource(RaycastWorld::new());
    app.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
    app.insert_resource(ArchSegments::default()).insert_resource(LintelSegments::default());
    let mut inner = InnerWalls::new();
    let wall = inner.new_wall(WallState::new(Curve2::try_new(vec![[-2., -2.], [2., -2.]], false).unwrap(), 2., 0, 0, false));
    let mut public = PublicWalls::default();
    public.consume_with_paths(&mut inner, &TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
        minimum_y: 0., maximum_y: 0., sampling_fix: false }, None,
        &WallPublicationDirty::default(), &WallSpatialHash::default());
    app.insert_resource(public);
    // 使用生产系统和真实状态转换；本夹具不运行完整游戏装配或保存流程。
    app.add_systems(Startup, (overhang_subscribe_to_tabloids, initialize_mesh_intersection_subscriber));
    app.add_systems(OnExit(AppLoadingState::Game), |mut world: ResMut<RaycastWorld>| world.on_level_reload());
    app.add_systems(Update, (update_rapier_colliders, flush_rapier_colliders).chain()
        .run_if(bevy_state::condition::in_state(AppLoadingState::Game)));
    app.add_systems(PostUpdate, tiny_glade::app_systems::advance_reload_state);
    app.update();
    assert_eq!(app.world().resource::<OverhangSupportDirtySubscriptions>().subscribers.len(), 10);
    app.world_mut().resource_mut::<NextState<AppLoadingState>>().set(AppLoadingState::Game);
    app.update();
    app.world_mut().resource_mut::<NextState<AppLoadingState>>().set(AppLoadingState::Reload);
    app.update();
    assert_eq!(*app.world().resource::<State<AppLoadingState>>().get(), AppLoadingState::Reload);
    app.update();
    assert_eq!(*app.world().resource::<State<AppLoadingState>>().get(), AppLoadingState::Game);
    assert_eq!(app.world().resource::<OverhangSupportDirtySubscriptions>().subscribers.len(), 10);

    // 真正的墙实体进入 update_rapier_colliders，再经生产 flush 发布差量。
    let mut volume = WallTriggerVolume::default();
    volume.quads.push(Quad { wall_uv: [Vec2::ZERO, Vec2::X, Vec2::Y, Vec2::ONE],
        positions: [Vec3::new(-2., 0., -2.), Vec3::new(2., 0., -2.),
            Vec3::new(-2., 4., -2.), Vec3::new(2., 4., -2.)],
        prepared_u: [0., 1.], endpoint_normals: [Vec3::Z; 2], segment_heights: [3.; 2] });
    app.world_mut().spawn((volume, InstancedWall::<WallBrickInstance>::new(wall.0, Vec::new())));
    app.update();
    assert!(app.world_mut().resource_mut::<OverhangSupportDirtySubscriptions>().subscribers[0].take().is_some());
    let diff = app.world_mut().resource_mut::<MeshMeshIntersectionSubscriber>().0.take().expect("新墙碰撞差量");
    assert!(diff.added.iter().any(|handle| handle.slot == 0));
}

#[test]
fn terrain_reload_same_saved_session_rejects_previous_gpu_lifecycle() {
    // 存档路径是进程级状态；相同ID重入在独立子进程运行同一真实场景链。
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "terrain_clear_owner_runs_during_real_reload_transition", "--nocapture"])
        .env("TG_M1_SAME_SESSION_RELOAD", "1").status().unwrap();
    assert!(status.success(), "同存档ID Reload联合回归失败");
}

#[test]
fn terrain_clear_owner_runs_during_real_reload_transition() {
    use country_core::{render::gpu_readback::{GpuReadbacks, TerrainReadbackEpoch, TerrainCopyEpoch},
        resources::terrain_height::{TerrainHeightSsbo, VisualTerrainHeightSsbo, TerrainHeightsData}};
    use tiny_glade::{gpu_readback::ReadbackPublisher, render_thread::FrameFeedback};
    use country_core::{
        asset_libraries::{audio_trigger_library::AudioTriggerLibrary, placement_data_library::AssetPlacementDataLibrary,
            ssbo_library::AssetSsboLibrary, texture_library::AssetTextureLibrary},
        render::{device::GraphicsDeviceResource, nani::gpu_world::GpuWorld, load_texture::boundary::TextureDesc},
        resources::{audio_resources::{AudioCore, DisabledStudio}, color_id::*, history::ReplayBuffer,
            rt::RtWorld, terrain_editing::TerrainEditingData, tree_storage::InteractableForestPlacements,
            walls::{WallHighlight, decorator_archivist::LiveDecoratorArchivist},
            world_raster::DirtyTilePublisher, world_raster_defs::*},
        startup::clearing::{uninit_clearing, despawn_glade_entities},
        systems::{audio::game_loading_audio::GameLoadingAudioState, clear_canvas::{clear_canvas, register_clear_canvas_resources},
            terrain::{self, delayed_terrain_changed_msg::ScheduledTerrainReadback, ui_edit_terrain::TerrainEditUiState},
            wall::wall_state::wall_spatial_hash::WallSpatialHash},
    };
    let installed = std::fs::canonicalize(PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("set TG_INSTALL_ROOT"))).unwrap();
    let account = tempfile::tempdir().unwrap();
    paths::initialize_paths_with_installation(91914, Some(account.path()), &installed).unwrap();
    let previous = std::env::current_dir().unwrap();
    std::env::set_current_dir(&installed).unwrap();
    let fixture = installed.join("assets/starting-builds/full/15-lake-house/history.json");
    let first = SessionId::new();
    let sessions = [first, if std::env::var_os("TG_M1_SAME_SESSION_RELOAD").is_some() { first } else { SessionId::new() }];
    for id in sessions {
        std::fs::create_dir_all(id.save_folder_path()).unwrap();
        std::fs::copy(&fixture, id.save_folder_path().join("history.json")).unwrap();
        SessionMeta::default().write_to_disk(id).unwrap();
    }
    let device = rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()).expect("真实 Vulkan 设备");
    let buffers = Arc::new(tiny_glade::render_host::BufferHost::new(device.clone()));
    let textures = Arc::new(tiny_glade::render_host::TextureHost::new(device.clone()));
    let mut library = AssetTextureLibrary::new(textures.as_ref());
    library.create_preset_textures_with(textures.as_ref(), |_| TextureDesc::new_2d([4, 4], 37).sampled().mip_levels(1).array_elements(1));
    let ssbos = AssetSsboLibrary::new();
    ssbos.add_new::<WallColorIdSsbo>(buffers.as_ref());
    ssbos.add_new::<GateColorIdSsbo>(buffers.as_ref());
    ssbos.add_new::<HalftimberColorIdSsbo>(buffers.as_ref());
    ssbos.add_new::<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>(buffers.as_ref());
    ssbos.add_new::<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>(buffers.as_ref());
    ssbos.add_new::<TerrainHeightSsbo>(buffers.as_ref());
    ssbos.add_new::<VisualTerrainHeightSsbo>(buffers.as_ref());
    let mut app = app(&installed);
    app.insert_resource(NextSession::load(sessions[0]));
    app.insert_resource(GraphicsDeviceResource::new(country_core::rhapsody_contract::GraphicsDevice::new(device.clone()).with_buffer_host(buffers.clone())));
    app.insert_resource(library).insert_resource(ssbos).insert_resource(GpuWorld::new(buffers.as_ref()));
    register_clear_canvas_resources(app.world_mut(), textures);
    app.insert_resource(AssetPlacementDataLibrary::new()).insert_resource(AudioTriggerLibrary::default());
    app.insert_resource(RtWorld::default()).insert_resource(InteractableForestPlacements::default());
    app.insert_resource(InnerWalls::new()).init_resource::<WallSpatialHash>();
    app.init_resource::<WallColorIds>().init_resource::<GateColorIds>().init_resource::<HalftimberColorIds>();
    app.insert_resource(History::<country_core::resources::history::MaybePendingSnapshotId>::empty_glade()).insert_resource(ReplayBuffer::new(1.0));
    app.init_resource::<LiveDecoratorArchivist>().init_resource::<WallHighlight>();
    app.insert_resource(TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default()));
    // 与真实 App 一致，先注册事件维护，再初始化兼容资源。
    app.add_event::<country_core::resources::window::CursorMovedScreenSpace>()
        .add_event::<terrain::edit_terrain_stroke::EditTerrainStrokeCmd>()
        .add_event::<terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>()
        .add_event::<terrain::draw_terrain_stroke::EvTerrainStrokeChanged>()
        .add_event::<terrain::PlayTerrainSound>()
        .add_event::<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>();
    terrain::register_terrain_events(app.world_mut());
    app.insert_resource(ScheduledTerrainReadback::default());
    app.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
    app.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
    app.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
    app.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
    app.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
    app.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
    let core = AudioCore::<DisabledStudio> { studio: None, banks: Default::default() };
    app.insert_resource(GameLoadingAudioState::new(&core)).insert_resource(core);
    let (sender, _receiver) = crossbeam_channel::unbounded();
    let mut autosave = AutoSaveState::default(); autosave.attach_command_sender(sender); app.insert_resource(autosave);
    app.insert_resource(UiState::at_startup());
    app.insert_resource(country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast { tag: 1, pos: glam::Vec3::new(1.0, 2.0, 3.0), ..Default::default() });
    app.insert_resource(country_core::resources::glade_border::GladeBorder(country_core::resources::glade_border::BorderShape::Circle(utils::geometry::Circle2d::new(Vec2::ZERO, 100.0))));
    app.insert_resource(AppMode::Terrain);
    app.add_systems(OnExit(AppLoadingState::Game), (tiny_glade::toolbar_plugin::clear_toolbar_session, (uninit_clearing::<DisabledStudio>, despawn_glade_entities)).chain().after(begin_glade_unload));
    app.add_systems(Update, terrain::ui_draw_terrain::ui_draw_terrain.run_if(bevy_state::condition::in_state(AppLoadingState::Game)));
    app.add_systems(Update, clear_canvas.run_if(bevy_ecs::schedule::common_conditions::on_event::<ClearCanvasCmd>));
    app.add_systems(Update, country_core::systems::save::load_completion_checker::check_load_completion_system.after(load_game_system));
    app.world_mut().resource_mut::<NextState<AppLoadingState>>().set(AppLoadingState::Game);
    app.update();
    assert_eq!(app.world().resource::<ActiveSession>().id, sessions[0]);
    for _ in 0..3 { app.update(); }
    assert_eq!(app.world().resource::<LoadGameManager>().state, LoadGameState::Loaded);
    // 在旧会话提交真实GPU上传和回读票据；回调留到同一World加载新会话后执行。
    let mut readback_publisher = ReadbackPublisher::new(device.clone());
    app.init_resource::<TerrainHeightsData<TerrainHeightSsbo>>();
    app.init_resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>();
    app.add_event::<terrain::delayed_terrain_changed_msg::OnTerrainChange>();
    fn upload_height_fields(world: &World, device: &Arc<rhapsody::device::Device>,
        buffers: &tiny_glade::render_host::BufferHost, values: [u32; 2])
        -> (rhapsody::sync::SyncSet, rhapsody::transfer::TransferManager) {
        let handles = {
            let library = world.resource::<AssetSsboLibrary>();
            [library.get::<TerrainHeightSsbo>().unwrap().buffer,
                library.get::<VisualTerrainHeightSsbo>().unwrap().buffer]
        };
        let mut points = rhapsody::sync::SyncSet::EMPTY;
        let mut uploads = rhapsody::transfer::TransferManager::new(device.clone(), Default::default()).unwrap();
        let deadline = std::time::Instant::now() + Duration::from_secs(20);
        while !buffers.poll_pending_uploads().unwrap() {
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(1));
        }
        for (handle, word) in handles.iter().zip(values) {
            let words = vec![word; 0x20000];
            let ticket = uploads.enqueue_upload(buffers.managed_buffer(handle).unwrap().shared(),
                0, bytemuck::cast_slice(&words), rhapsody::transfer::UploadOptions {
                    key: rhapsody::transfer::UploadKey(handle.id), epoch: 1, priority: Default::default(),
                }).unwrap();
            let deadline = std::time::Instant::now() + Duration::from_secs(20);
            loop {
                uploads.flush_uploads(8).unwrap();
                if let rhapsody::transfer::UploadStatus::Submitted(point) = ticket.status() {
                    points.insert(point);
                    break;
                }
                assert!(std::time::Instant::now() < deadline, "旧会话GPU上传超时");
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        (points, uploads)
    }
    let (points, _old_uploads) = upload_height_fields(app.world(), &device, &buffers, [0x2345_1234, 0x6789_4567]);
    let mut submitted = FrameFeedback::default();
    readback_publisher.finish_frame(app.world_mut(), &buffers, Some(points), &mut submitted);
    assert!(submitted.error.is_none());
    for step in submitted.apply { step(app.world_mut()); }
    assert_eq!(app.world().resource::<GpuReadbacks>().submitted_frame(), 1);
    assert!(readback_publisher.stats().issued >= 2);
    let deadline = std::time::Instant::now() + Duration::from_secs(20);
    while readback_publisher.stats().published < readback_publisher.stats().issued {
        let mut feedback = FrameFeedback::default();
        readback_publisher.finish_frame(app.world_mut(), &buffers, None, &mut feedback);
        assert!(feedback.error.is_none());
        for step in feedback.apply { step(app.world_mut()); }
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
    let old_epoch = TerrainReadbackEpoch::capture(app.world());
    let old_revision = app.world().resource::<GpuReadbacks>().get::<TerrainHeightSsbo>().unwrap().revision;
    // 另取同一旧帧的真实票据，保留到Reload后；同时保留已发布的旧缓存。
    let mut late_publisher = ReadbackPublisher::new(device.clone());
    let mut old_feedback = FrameFeedback::default();
    late_publisher.finish_frame(app.world_mut(), &buffers, Some(points), &mut old_feedback);
    assert!(old_feedback.error.is_none());
    let mut delayed_height = Schedule::default();
    delayed_height.add_systems(terrain::delayed_terrain_changed_msg::delayed_terrain_changed_msg);
    delayed_height.run(app.world_mut());
    app.world_mut().resource_mut::<ScheduledTerrainReadback>().on_terrain_modified();
    let tick = app.world().read_change_tick().get();
    app.world_mut().resource_mut::<ScheduledTerrainReadback>().on_gpu_copy_issued(tick);
    app.insert_resource(TerrainCopyEpoch(old_epoch));
    // 不消费copy_issued，覆盖旧确认位与旧缓存一起跨Reload的情况。
    app.world_mut().insert_resource(AppMode::Terrain);
    let widget = country_core::systems::ui::resources::ui_state::WidgetId::clearing_canvas(terrain::ui_draw_terrain::DRAW_CANVAS_NAME);
    for phase in [3, 4] {
        app.world_mut().resource_mut::<UiState>().replace_interaction(country_core::systems::ui::resources::ui_state::UiInteraction { native_tag: 2, native_phase: phase, widget });
        app.update();
    }
    assert_eq!(app.world().resource::<terrain::ui_draw_terrain::TerrainDrawState>().state, 2);
    app.world_mut().insert_resource(country_core::resources::time::GameTime { delta: 0.05, ..Default::default() });
    let mut edit = Schedule::default();
    edit.add_systems((terrain::ui_edit_terrain::ui_edit_terrain, terrain::edit_terrain_stroke::edit_terrain_stroke).chain());
    let edit_widget = country_core::systems::ui::resources::ui_state::WidgetId::from_name("terrain_edit_0__canvas");
    for phase in [3, 4] {
        app.world_mut().resource_mut::<UiState>().replace_interaction(country_core::systems::ui::resources::ui_state::UiInteraction { native_tag: 2, native_phase: phase, widget: edit_widget });
        if phase == 4 { app.world_mut().send_event(country_core::resources::window::CursorMovedScreenSpace(Vec2::new(100.0, 0.0))); }
        edit.run(app.world_mut());
    }
    let old_cache = *app.world().resource::<TerrainEditUiState>();
    assert_eq!(old_cache.hover, glam::Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(old_cache.previous_height_ws, 2.5);
    assert!(app.world().resource::<TerrainEditingData>().strokes[&tg_terrain::schema::StrokeId(0)].falloff > old_cache.previous_falloff);
    // 历史 pending 仅在 release 时产生；拖动中确切活跃的是参数快照。
    assert!(app.world().resource::<terrain::deferred_terrain_history::PendingTerrainHistory>().is_empty());
    app.world_mut().resource_mut::<TerrainEditingData>().cursor_height_override = Some(7.0);
    app.world().resource::<ScheduledTerrainReadback>().on_terrain_modified();
    let pending_before = app.world().resource::<ScheduledTerrainReadback>().pending.load(std::sync::atomic::Ordering::SeqCst);
    assert!(pending_before > 0);
    assert!(!sessions[0].save_folder_path().join("loading.tmp").exists(), "真实加载完成消费者应解除同存档重载门控");
    app.insert_resource(NextSession::load(sessions[1]));
    app.world_mut().resource_mut::<NextState<AppLoadingState>>().set(AppLoadingState::Reload);
    app.update();
    assert_eq!(*app.world().resource::<State<AppLoadingState>>().get(), AppLoadingState::Reload);
    let terrain = app.world().resource::<TerrainEditingData>();
    assert!(terrain.strokes.is_empty()); assert_eq!(terrain.next_index, 0);
    assert_eq!(terrain.cursor_height_override, Some(7.0));
    assert_eq!(app.world().resource::<TerrainEditUiState>().previous_height_ws, old_cache.previous_height_ws);
    assert_eq!(app.world().resource::<ScheduledTerrainReadback>().pending.load(std::sync::atomic::Ordering::SeqCst), pending_before);
    app.update();
    assert_eq!(*app.world().resource::<State<AppLoadingState>>().get(), AppLoadingState::Game);
    assert_eq!(app.world().resource::<ActiveSession>().id, sessions[1]);
    assert_eq!(app.world().resource::<SourceReceipts>().0.iter().map(|entry| entry.0).collect::<Vec<_>>(), sessions);
    for _ in 0..3 { app.update(); }
    assert_eq!(app.world().resource::<LoadGameManager>().state, LoadGameState::Loaded);
    let before_authority = app.world().resource::<TerrainHeightsData<TerrainHeightSsbo>>().heights().to_vec();
    let before_visual = app.world().resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>().heights().to_vec();
    let mut authority_dirty = app.world_mut().resource_mut::<TerrainHeightsData<TerrainHeightSsbo>>().subscribe();
    let mut visual_dirty = app.world_mut().resource_mut::<TerrainHeightsData<VisualTerrainHeightSsbo>>().subscribe();
    let _ = authority_dirty.take();
    let _ = visual_dirty.take();
    let before_changes = app.world().resource::<Events<terrain::delayed_terrain_changed_msg::OnTerrainChange>>().len();
    let deadline = std::time::Instant::now() + Duration::from_secs(20);
    while late_publisher.stats().published < late_publisher.stats().issued {
        late_publisher.finish_frame(app.world_mut(), &buffers, None, &mut old_feedback);
        assert!(old_feedback.error.is_none());
        assert!(std::time::Instant::now() < deadline, "跨Reload旧GPU票据超时");
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(!old_feedback.apply.is_empty());
    for step in old_feedback.apply { step(app.world_mut()); }
    for _ in 0..3 { delayed_height.run(app.world_mut()); }
    let readbacks = app.world().resource::<GpuReadbacks>();
    assert_eq!(readbacks.get::<TerrainHeightSsbo>().unwrap().epoch, Some(old_epoch));
    assert_eq!(readbacks.get::<TerrainHeightSsbo>().unwrap().revision, old_revision, "晚到旧反馈不得再次更新缓存");
    assert_eq!(readbacks.get::<VisualTerrainHeightSsbo>().unwrap().epoch, Some(old_epoch));
    assert_eq!(app.world().resource::<TerrainHeightsData<TerrainHeightSsbo>>().heights().iter().zip(&before_authority).position(|(a,b)|a!=b), None,
        "旧会话回调不能覆盖新会话权威高度");
    assert_eq!(app.world().resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>().heights().iter().zip(&before_visual).position(|(a,b)|a!=b), None,
        "旧会话回调不能覆盖新会话视觉高度");
    assert!(authority_dirty.take().is_none(), "旧反馈不能发布权威脏区");
    assert!(visual_dirty.take().is_none(), "旧反馈不能发布视觉脏区");
    assert_eq!(app.world().resource::<Events<terrain::delayed_terrain_changed_msg::OnTerrainChange>>().len(), before_changes);
    // 新会话合法GPU反馈仍须发布，不能以丢弃所有回读掩盖旧会话污染。
    let new_epoch = TerrainReadbackEpoch::capture(app.world());
    assert_ne!(old_epoch, new_epoch);
    let new_values = [0x1357_2468u32, 0x369c_147a];
    let (points, _new_uploads) = upload_height_fields(app.world(), &device, &buffers, new_values);
    app.world_mut().resource_mut::<ScheduledTerrainReadback>().on_terrain_modified();
    let tick = app.world().read_change_tick().get();
    app.world_mut().resource_mut::<ScheduledTerrainReadback>().on_gpu_copy_issued(tick);
    app.insert_resource(TerrainCopyEpoch(new_epoch));
    let mut feedback = FrameFeedback::default();
    readback_publisher.finish_frame(app.world_mut(), &buffers, Some(points), &mut feedback);
    assert!(feedback.error.is_none());
    for step in feedback.apply { step(app.world_mut()); }
    delayed_height.run(app.world_mut());
    let deadline = std::time::Instant::now() + Duration::from_secs(20);
    while readback_publisher.stats().published < readback_publisher.stats().issued {
        let mut feedback = FrameFeedback::default();
        readback_publisher.finish_frame(app.world_mut(), &buffers, None, &mut feedback);
        assert!(feedback.error.is_none());
        for step in feedback.apply { step(app.world_mut()); }
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
    for _ in 0..3 { delayed_height.run(app.world_mut()); }
    for (actual, word) in [app.world().resource::<TerrainHeightsData<TerrainHeightSsbo>>().heights(),
        app.world().resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>().heights()].into_iter().zip(new_values) {
        let expected: Vec<u16> = (0..512*512).map(|i| if i%2==0 {word as u16} else {(word>>16) as u16}).collect();
        assert_eq!(actual.iter().zip(&expected).position(|(a,b)|a!=b), None, "新会话合法反馈必须完整发布");
    }
    assert_eq!(authority_dirty.take().unwrap().tile_indices().count(), 4096);
    assert_eq!(visual_dirty.take().unwrap().tile_indices().count(), 4096);
    assert_eq!(app.world().resource::<Events<terrain::delayed_terrain_changed_msg::OnTerrainChange>>().len(), before_changes + 1);
    app.world_mut().insert_resource(AppMode::Terrain);
    app.update();
    assert_eq!(app.world().resource::<terrain::ui_draw_terrain::TerrainDrawState>().state, 0);
    assert!(app.world().resource::<terrain::deferred_terrain_history::PendingTerrainHistory>().is_empty());
    // 新会话重新捕获时必须覆盖旧参数快照，再由 release 生成新历史载荷。
    app.world_mut().resource_mut::<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>().pos = glam::Vec3::new(-2.0, 4.0, 1.0);
    app.world_mut().resource_mut::<UiState>().replace_interaction(country_core::systems::ui::resources::ui_state::UiInteraction { native_tag: 2, native_phase: 3, widget });
    app.update();
    assert!(app.world().resource::<TerrainEditingData>().strokes.contains_key(&tg_terrain::schema::StrokeId(0)));
    for phase in [3, 4, 5] {
        app.world_mut().resource_mut::<UiState>().replace_interaction(country_core::systems::ui::resources::ui_state::UiInteraction { native_tag: 2, native_phase: phase, widget: edit_widget });
        if phase == 4 { app.world_mut().send_event(country_core::resources::window::CursorMovedScreenSpace(Vec2::new(50.0, 0.0))); }
        edit.run(app.world_mut());
    }
    assert_eq!(app.world().resource::<TerrainEditUiState>().hover, glam::Vec3::new(-2.0, 4.0, 1.0));
    let hints = app.world().resource::<Events<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>>();
    assert!(!hints.get_cursor().read(hints).any(|hint| hint.tag == 1), "新拖动开始必须重置旧 Local 累计量");
    assert!(matches!(app.world().resource::<terrain::deferred_terrain_history::PendingTerrainHistory>().edit.as_ref(),
        Some(tg_terrain::schema::TerrainEdit::EditTerrainStroke { prev_height_ws, prev_falloff, .. })
        if *prev_height_ws == 4.0 && *prev_falloff == 0.4));
    std::env::set_current_dir(previous).unwrap();
}

// M06 有界 CPU 验收：起始墙是前置夹具，后续样式和剥离量必须经过真实命令
// 和记录系统。独立进程读取保存线程写出的文件并执行实际历史回放。
mod m06_wall_persistence {
    use super::*;
    use country_core::{
        resources::{
            glade_color_customization::GladeColorCustomization,
            history::{MaybePendingSnapshotId, ReplayBuffer, EditType, serialized::SavedMultiEdit,
                wall_edit::{WallCompoundHistoryEdit, WallHistoryEdit}},
            roofs::geometry::surface::Curve2,
            time::GameTime,
            walls::{WallId, WallState, public_walls::PublicWalls, publication_source::TerrainHeightGrid512},
        },
        systems::{
            history::{self as runtime, replay_wall},
            wall::{SetWallStyleCmd, SetFreehandHeightTypeCmd, SetPlasterPeelAmountCmd,
                set_wall_style::set_wall_style,
                wall_state::publish_public_wall_state::WallPublicationTerrain},
            save::{load_game::{self, ActiveLoadOptions}, save_game::{self, SerializedJson},
                save_thread::shutdown_save_thread},
        },
    };

    type H = History<MaybePendingSnapshotId>;

    fn floor_fixture_app() -> App {
        use bevy_ecs::schedule::ApplyDeferred;
        use country_core::{resources::{color_id::{WallColorIds, HalftimberColorIds, SetWallColorIdCmd, SetHalftimberColorIdCmd, SetGateColorIdCmd},
                critters::sheep::CmdCheckForSheepInsideBuilding, roofs::commands::CreateRoofCmd,
                tree_storage::HideTreesinAreaCmd,
                walls::public_walls::{OnWallChanged, OnWallDeleted, WallPublicationDirty, WallSpatialHash}},
            systems::{history::recording::HistoryRecordingFailed, mode_manager::RequestToolbarCategoryChange,
                wall::{eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer,
                    shapes::{CreateRectangleCmd, rectangle::create_rectangle::create_rectangle},
                    wall_state::publish_public_wall_state::publish_public_wall_state}}};
        let mut a = fixture_app();
        install_snapshot_owners(&mut a);
        // CPU shadow 验证实际 Nani owner 分配和退役；不宣称真实 GPU 上传。
        use country_core::render::{test_device::TestDevice, nani::{gpu_world::{GpuWorld, GpuWorldSubset,
            GpuWorldSubsetKind, OwnedGpuWorldSubset}, instance_data::StoneFloorInstanceData,
            systems::{nani_maintain_instances_ecs, nani_update_roof_floor_for_roofs_ecs}}};
        let device = TestDevice(1);
        let mut gpu = GpuWorld::new(&device);
        gpu.assign_subset(GpuWorldSubsetKind::RoofFloor, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<StoneFloorInstanceData>(&device, GpuWorldSubsetKind::RoofFloor, 0)));
        a.insert_resource(gpu).init_resource::<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>();
        a.init_resource::<WallColorIds>().init_resource::<HalftimberColorIds>();
        country_core::resources::roofs::register_roof_events(a.world_mut());
        a.add_event::<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>();
        a.add_systems(Update, system_roof::replay_roof::replay_roof.before(runtime::replay_pop_bucket_system));
        a.init_resource::<WallPublicationDirty>().init_resource::<WallSpatialHash>().init_resource::<AnimateWallBulldozeBuffer>();
        a.add_event::<CreateRectangleCmd>().add_event::<CreateRoofCmd>()
            .add_event::<SetWallColorIdCmd>().add_event::<SetHalftimberColorIdCmd>().add_event::<SetGateColorIdCmd>()
            .add_event::<HideTreesinAreaCmd>().add_event::<RequestToolbarCategoryChange>()
            .add_event::<CmdCheckForSheepInsideBuilding>().add_event::<HistoryRecordingFailed>()
            .add_event::<OnWallChanged>().add_event::<OnWallDeleted>();
        a.add_systems(Update, (create_rectangle, publish_public_wall_state,
            system_wall_constructor::construct_floors::construct_floors, ApplyDeferred,
            nani_maintain_instances_ecs, ApplyDeferred, nani_update_roof_floor_for_roofs_ecs).chain()
            .after(runtime::replay_pop_bucket_system));
        a
    }

    /// 原矩形命令创建墙，再由正式发布器发事件驱动楼板；高度与海拔故意不同。
    #[test]
    fn rectangle_command_publishes_floor_from_elevation() {
        use country_core::{components::drawable::NaniInstanceDesc, resources::walls::Floor,
            systems::wall::shapes::CreateRectangleCmd, utils::components::transform::Transform};
        let mut a = floor_fixture_app();
        a.world_mut().resource_mut::<H>().start_input_recording(AppMode::Path.native_tag() as u32).unwrap();
        a.world_mut().resource_mut::<Events<CreateRectangleCmd>>().send(CreateRectangleCmd {
            center: Vec2::new(1., 2.), dimensions: Vec2::new(6., 4.), orientation: Vec2::X,
            previous_wall: None, height: 7.25, elevation: 1.75,
        });
        a.update();
        a.world_mut().resource_mut::<H>().finish_input_recording().unwrap().unwrap();
        let wall = a.world().resource::<InnerWalls>().get_unwrapped(WallId(1));
        assert_eq!((wall.height(), wall.elevation()), (7.25, 1.75));
        let floors: Vec<_> = a.world_mut().query::<(&Floor, &Transform, &NaniInstanceDesc)>()
            .iter(a.world()).map(|(floor, transform, _)| (floor.wall_id, transform.translation)).collect();
        assert_eq!(floors, vec![(WallId(1), glam::Vec3::new(1., 1.75, 2.))]);
    }

    /// 合成 UI 相位驱动真实抬升系统，Local 跨三帧保持，不直接写墙或历史。
    fn raise_floor_wall(a: &mut App) {
        use country_core::{resources::{glade_border::MaxDemoElevationReached,
                walls::{decorator_archivist::LiveDecoratorArchivist, decorator_storage::DecoratorStorage,
                    decorator_collision::DecoratorPhysicsColliders}},
            systems::{audio::{PlayBuildingAudio, RequestWallMaterialAudio}, collision::world::RaycastWorld,
                mode_manager::EditSubject, ui::{resources::ui_state::{UiInteraction, WidgetId}, signifiers::UiSignifierStream},
                ui_raise_shape::{ui_raise_shape, impl_raise_shape, HandleRaiseShapeUiInteraction}}};
        a.init_resource::<DecoratorStorage>().init_resource::<LiveDecoratorArchivist>()
            .init_resource::<DecoratorPhysicsColliders>().init_resource::<RaycastWorld>()
            .init_resource::<UiSignifierStream>()
            .add_event::<RequestWallMaterialAudio>().add_event::<PlayBuildingAudio>()
            .add_event::<MaxDemoElevationReached>().add_event::<HandleRaiseShapeUiInteraction>();
        a.insert_resource(AppMode::Editing(EditSubject::None));
        a.insert_resource(CursorPositionSS(Vec2::new(500., -30000.)));
        a.insert_resource(WindowSize::new(1000, 1000));
        let mut camera = MainCamera::default();
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &glam::Mat4::IDENTITY);
        a.insert_resource(camera);
        let widget = WidgetId::from_name(&format!("raise_wall_elevation_{}", WallId(1)));
        let mut schedule = Schedule::default();
        schedule.add_systems((ui_raise_shape, impl_raise_shape).chain());
        a.world_mut().resource_mut::<H>().start_input_recording(AppMode::Path.native_tag() as u32).unwrap();
        for phase in [3, 4, 0] {
            let mut ui = UiState::new(UiInteraction { native_tag: 2, native_phase: phase, widget });
            ui.set_intersection_tag(0);
            ui.set_intersection_point(glam::Vec3::new(2., 2., 0.));
            a.insert_resource(ui);
            schedule.run(a.world_mut());
            a.world_mut().resource_mut::<UiSignifierStream>().take_entries();
            a.update();
        }
        a.world_mut().resource_mut::<H>().finish_input_recording().unwrap().unwrap();
    }

    fn floor_state(a: &mut App) -> Vec<(WallId, glam::Vec3)> {
        use country_core::{components::drawable::NaniInstanceDesc, resources::walls::Floor,
            render::nani::gpu_world::InstanceKey,
            utils::components::transform::Transform};
        a.world_mut().query::<(&Floor, &Transform, &NaniInstanceDesc, &InstanceKey)>().iter(a.world())
            .map(|(floor, transform, _, _)| (floor.wall_id, transform.translation)).collect()
    }

    #[test]
    fn m06_floor_edit_save_restart() {
        let account = tempfile::tempdir().unwrap();
        let installed = std::fs::canonicalize(PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("set TG_INSTALL_ROOT"))).unwrap();
        for phase in ["write", "restore"] {
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "m06_wall_persistence::m06_floor_edit_process", "--nocapture"])
                .env("TG_M06_FLOOR_PHASE", phase).env("TG_M06_WALL_ACCOUNT", account.path())
                .env("TG_INSTALL_ROOT", &installed).status().unwrap();
            assert!(status.success(), "M06 楼板子进程 {phase} 失败");
            assert!(account.path().join(format!("floor-completed-{phase}")).is_file());
        }
    }

    #[test]
    fn m06_floor_edit_process() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::systems::wall::shapes::CreateRectangleCmd;
        let Ok(phase) = std::env::var("TG_M06_FLOOR_PHASE") else { return; };
        let account = PathBuf::from(std::env::var_os("TG_M06_WALL_ACCOUNT").unwrap());
        let installed = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
        paths::initialize_paths_with_installation(91917, Some(&account), &installed).unwrap();
        let session_path = account.join("m06-floor-session.json");
        let mut a = floor_fixture_app();
        if phase == "write" {
            let session = SessionId::new(); attach_save(&mut a, session);
            std::fs::write(&session_path, serde_json::to_vec(&session).unwrap()).unwrap();
            a.world_mut().resource_mut::<H>().start_input_recording(AppMode::Path.native_tag() as u32).unwrap();
            a.world_mut().resource_mut::<Events<CreateRectangleCmd>>().send(CreateRectangleCmd {
                center: Vec2::new(1., 2.), dimensions: Vec2::new(6., 4.), orientation: Vec2::X,
                previous_wall: None, height: 7.25, elevation: 0.,
            });
            a.update();
            a.world_mut().resource_mut::<H>().finish_input_recording().unwrap().unwrap();
            assert!(floor_state(&mut a).is_empty(), "零海拔不能生成楼板");
            consume_roof_creation(&mut a);
            raise_floor_wall(&mut a);
            let elevated = a.world().resource::<InnerWalls>().get_unwrapped(WallId(1)).elevation();
            assert!(elevated > 0.0);
            assert_eq!(floor_state(&mut a), vec![(WallId(1), glam::Vec3::new(1., elevated, 2.))]);
            let floor_entity = a.world_mut().query_filtered::<Entity, With<country_core::resources::walls::Floor>>()
                .iter(a.world()).next().unwrap();
            let before = a.world().resource::<country_core::render::nani::gpu_world::GpuWorld>()
                .owned_subset_mut(country_core::render::nani::gpu_world::GpuWorldSubsetKind::RoofFloor).instance_count();
            send_history(&mut a, HistoryCommand::UNDO);
            assert_eq!(a.world().resource::<InnerWalls>().get_unwrapped(WallId(1)).elevation(), 0.);
            assert!(floor_state(&mut a).is_empty());
            assert!(a.world().get_entity(floor_entity).is_err());
            assert_eq!(a.world().resource::<country_core::render::nani::gpu_world::GpuWorld>()
                .owned_subset_mut(country_core::render::nani::gpu_world::GpuWorldSubsetKind::RoofFloor).instance_count(), before - 1);
            send_history(&mut a, HistoryCommand::REDO);
            assert_eq!(a.world().resource::<InnerWalls>().get_unwrapped(WallId(1)).elevation(), elevated);
            assert_eq!(floor_state(&mut a).len(), 1);
            std::fs::write(account.join("m06-floor-elevation.json"), serde_json::to_vec(&elevated).unwrap()).unwrap();
            save(&mut a);
            a.world_mut().resource_mut::<GameTime>().delta = 0.;
            // 同一个系统实例保留 Local<SaveGameReplayModTracker>；刚完成撤销／重做
            // 后 pop_counter 非零，首帧只建立稳定性基线，第二帧才可捕获快照。
            let mut save_schedule = Schedule::default();
            save_schedule.add_systems(save_game::save_game_system);
            save_schedule.run(a.world_mut());
            a.world_mut().resource_mut::<Events<save_game::SaveGameCmd>>().send(save_game::SaveGameCmd);
            save_schedule.run(a.world_mut());
            assert!(!a.world().resource::<Events<save_game::SavingGameComplete>>().is_empty());
            let snapshots = &a.world().resource::<H>().ledger().resolved_snapshots;
            assert_eq!(snapshots.len(), 1, "保存完成不等于已捕获稳定快照");
            let snapshot = snapshots[0].value.snapshot_id().expect("保存线程未完成快照");
            assert!(session.save_folder_path().join("snapshots").join(format!("{}.snapshot", snapshot.0)).is_file());
            shutdown_save_thread(a.world_mut()).unwrap();
        } else {
            assert_eq!(phase, "restore");
            let session: SessionId = serde_json::from_slice(&std::fs::read(session_path).unwrap()).unwrap();
            let elevated: f32 = serde_json::from_slice(&std::fs::read(account.join("m06-floor-elevation.json")).unwrap()).unwrap();
            attach_save(&mut a, session);
            let mut manager = LoadGameManager::new();
            manager.begin(LoadGameSource::Path(session.save_folder_path().join("history.json")),
                ActiveLoadOptions { ignore_snapshot: false, request_reveal: false, new_session: false },
                country_core::startup::startup_session::make_post_load_commands(vec![]));
            a.insert_resource(manager).insert_resource(AppMode::LoadGame);
            load_game::try_load_game_system(a.world_mut()).unwrap();
            assert_eq!(a.world().resource::<LoadGameManager>().state, LoadGameState::ApplyCounter);
            assert!(a.world().resource::<InnerWalls>().contains(WallId(1)), "快照应用未恢复矩形墙");
            a.update();
            assert_eq!(a.world().resource::<InnerWalls>().get_unwrapped(WallId(1)).elevation(), elevated);
            assert_eq!(floor_state(&mut a), vec![(WallId(1), glam::Vec3::new(1., elevated, 2.))]);
            consume_roof_creation(&mut a);
            for _ in 0..2 { load_game::try_load_game_system(a.world_mut()).unwrap(); a.update(); }
            assert_eq!(a.world().resource::<LoadGameManager>().state, LoadGameState::Loaded);
            send_history(&mut a, HistoryCommand::UNDO);
            assert!(floor_state(&mut a).is_empty(), "撤销抬升必须销毁楼板");
            send_history(&mut a, HistoryCommand::REDO);
            assert_eq!(floor_state(&mut a).len(), 1);
            // 撤销实际抬升，再撤销矩形创建；墙删除必须由历史回放和发布器传播。
            send_history(&mut a, HistoryCommand::UNDO);
            send_history(&mut a, HistoryCommand::UNDO);
            assert!(!a.world().resource::<InnerWalls>().contains(WallId(1)));
            assert!(floor_state(&mut a).is_empty());
            shutdown_save_thread(a.world_mut()).unwrap();
        }
        std::fs::write(account.join(format!("floor-completed-{phase}")), b"passed").unwrap();
    }

    fn fixture_app() -> App {
        let mut a = App::new();
        add_save_load_events_and_resources(a.world_mut());
        runtime::register_history_events(a.world_mut());
        a.world_mut().init_resource::<Events<SetWallStyleCmd>>();
        a.world_mut().init_resource::<Events<SetFreehandHeightTypeCmd>>();
        a.world_mut().init_resource::<Events<SetPlasterPeelAmountCmd>>();
        a.world_mut().init_resource::<Events<country_core::systems::audio::PlayWallUiSound>>();
        a.insert_resource(H::empty_glade()).insert_resource(ReplayBuffer::new(1.0));
        a.insert_resource(InnerWalls::new()).insert_resource(PublicWalls::default());
        a.insert_resource(GameTime { delta: 1.0, ran: true, ..Default::default() });
        a.insert_resource(WallPublicationTerrain(TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0., maximum_y: 4., sampling_fix: true,
        }));
        a.insert_resource(TimeOfDay::default()).insert_resource(GladeSettings::default());
        a.insert_resource(GameConfig::default()).insert_resource(MainCamera::new(1.6));
        a.insert_resource(AppMode::default()).insert_resource(MetaCommander::default());
        a.add_systems(Update, (
            set_wall_style::<MaybePendingSnapshotId>,
            runtime::history_process_system::<MaybePendingSnapshotId>, replay_wall::replay_wall,
            replay_wall::apply_wall_replay_color, replay_wall::apply_wall_replay_audio,
            runtime::replay_pop_bucket_system,
        ).chain());
        a
    }

    fn attach_save(a: &mut App, session: SessionId) {
        std::fs::create_dir_all(session.save_folder_path()).unwrap();
        a.insert_resource(ActiveSession::new(session, SessionMeta::default()));
        let mut auto = AutoSaveState::default();
        auto.session = session;
        let worker = SaveThreadHandle::spawn_with_unsaved(auto.unsaved_files.clone());
        auto.attach_command_sender(worker.command_sender.clone());
        a.insert_resource(auto).insert_resource(worker);
    }

    fn send_history(a: &mut App, command: HistoryCommand) {
        a.world_mut().resource_mut::<Events<HistoryCommand>>().send(command);
        for _ in 0..24 { a.update(); }
        assert!(a.world().resource::<ReplayBuffer>().is_empty());
        assert!(a.world().resource::<Events<runtime::HistoryReplayFailed>>().is_empty());
    }

    fn assert_wall(a: &App, style: u8, peel: f32) {
        let wall = a.world().resource::<InnerWalls>().get_unwrapped(WallId(1));
        assert_eq!(wall.snapshot().source.style, style);
        assert_eq!(wall.snapshot().metadata.source_60_bits, peel.to_bits());
        assert_eq!(wall.snapshot().source.curve.points, vec![[0., 0.], [4., 0.]]);
    }

    fn save(a: &mut App) {
        let history = a.world().resource::<H>();
        let sections = save_game::assemble_game_save_sections(
            load_game::LATEST_SAVE_VERSION, history.identity(),
            &country_core::systems::camera_rig::CameraRigParametersCache::default(),
            &GladeColorCustomization::default(), SerializedJson::new(&TimeOfDay::default()).unwrap(),
            SerializedJson::new(&Option::<country_core::resources::critters::sheep::control::SheepControlSnapshot>::None).unwrap(),
            SerializedJson::new(&history.ledger().cursor).unwrap(),
            SerializedJson::new(&history.ledger().undo_anchor).unwrap(),
            SerializedJson::new(&Vec::<serde_json::Value>::new()).unwrap(),
        );
        let edits = history.ledger().entries.iter().cloned().map(SavedMultiEdit)
            .map(|entry| SerializedJson::new(&entry).unwrap()).collect();
        let session = a.world().resource::<ActiveSession>().id;
        let mut auto = a.world_mut().resource_mut::<AutoSaveState>();
        auto.send_command_expect(AsyncSaveCmd::OpenFile { session, sections, edits });
        auto.flush_synchronously().unwrap();
    }

    #[test]
    fn m06_wall_edit_history_save_restart() {
        let account = tempfile::tempdir().unwrap();
        let installed = std::fs::canonicalize(PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("set TG_INSTALL_ROOT"))).unwrap();
        assert!(installed.join("assets/starting-builds/full/15-lake-house/history.json").is_file());
        for phase in ["write", "restore"] {
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "m06_wall_persistence::m06_wall_process", "--nocapture"])
                .env("TG_M06_WALL_PHASE", phase).env("TG_M06_WALL_ACCOUNT", account.path())
                .env("TG_INSTALL_ROOT", &installed).status().unwrap();
            assert!(status.success(), "M06 子进程 {phase} 失败");
            assert!(account.path().join(format!("completed-{phase}")).is_file());
        }
    }

    #[test]
    fn m06_wall_process() {
        let Ok(phase) = std::env::var("TG_M06_WALL_PHASE") else { return; };
        let account = PathBuf::from(std::env::var_os("TG_M06_WALL_ACCOUNT").unwrap());
        let installed = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
        paths::initialize_paths_with_installation(91915, Some(&account), &installed).unwrap();
        let session_path = account.join("m06-session.json");
        let mut a = fixture_app();
        if phase == "write" {
            let session = SessionId::new();
            attach_save(&mut a, session);
            std::fs::write(&session_path, serde_json::to_vec(&session).unwrap()).unwrap();
            // 仅起始墙使用已提交夹具；后续编辑由命令 producer 生成记录。
            let wall = WallState::new(Curve2::try_new(vec![[0., 0.], [4., 0.]], false).unwrap(), 2., 0, 0, false);
            let create = WallHistoryEdit::Create { wall: country_core::resources::walls::publication_fields::WallId(1), source: wall.snapshot().clone(),
                color_id: 7, restored_height: 2., restored_elevation: 0. };
            a.world_mut().resource_mut::<InnerWalls>().new_wall_with_id(WallId(1), wall);
            {
                let mut history = a.world_mut().resource_mut::<H>();
                history.start_input_recording(AppMode::Path.native_tag() as u32).unwrap();
                history.add(EditType::Wall(Arc::new(WallCompoundHistoryEdit { groups: vec![vec![create]] }))).unwrap();
                history.finish_input_recording().unwrap().unwrap();
                history.start_input_recording(AppMode::Path.native_tag() as u32).unwrap();
            }
            a.world_mut().resource_mut::<Events<SetWallStyleCmd>>().send(SetWallStyleCmd {
                wall_id: WallId(1), from: 0, to: 2, record_history: 1, play_sound: 0,
            });
            a.world_mut().resource_mut::<Events<SetPlasterPeelAmountCmd>>().send(SetPlasterPeelAmountCmd {
                wall_id: WallId(1), from: 0., to: 0.75, record_history: 1,
            });
            a.update();
            a.world_mut().resource_mut::<H>().finish_input_recording().unwrap().unwrap();
            assert_eq!(a.world().resource::<H>().ledger().entries.len(), 2);
            assert!(a.world().resource::<Events<runtime::recording::HistoryRecordingFailed>>().is_empty());
            assert_wall(&a, 2, 0.75);
            send_history(&mut a, HistoryCommand::UNDO); assert_wall(&a, 0, 0.);
            send_history(&mut a, HistoryCommand::REDO); assert_wall(&a, 2, 0.75);
            save(&mut a);
            shutdown_save_thread(a.world_mut()).unwrap();
            assert!(session.save_folder_path().join("history.json").is_file());
        } else {
            assert_eq!(phase, "restore");
            let session: SessionId = serde_json::from_slice(&std::fs::read(session_path).unwrap()).unwrap();
            attach_save(&mut a, session);
            let mut manager = LoadGameManager::new();
            manager.begin(LoadGameSource::Path(session.save_folder_path().join("history.json")),
                ActiveLoadOptions { ignore_snapshot: false, request_reveal: false, new_session: false },
                country_core::startup::startup_session::make_post_load_commands(vec![]));
            a.insert_resource(manager).insert_resource(AppMode::LoadGame);
            for _ in 0..3 { load_game::try_load_game_system(a.world_mut()).unwrap(); a.update(); }
            for _ in 0..24 { a.update(); }
            assert_eq!(a.world().resource::<LoadGameManager>().state, LoadGameState::Loaded);
            assert!(a.world().resource::<Events<runtime::HistoryReplayFailed>>().is_empty());
            assert_eq!(a.world().resource::<H>().ledger().entries.len(), 2);
            assert_wall(&a, 2, 0.75);
            send_history(&mut a, HistoryCommand::UNDO); assert_wall(&a, 0, 0.);
            send_history(&mut a, HistoryCommand::REDO); assert_wall(&a, 2, 0.75);
            shutdown_save_thread(a.world_mut()).unwrap();
        }
        std::fs::write(account.join(format!("completed-{phase}")), b"passed").unwrap();
    }

    // 屋顶必须经生产 CreateRoofCmd 与实体/关联系统；快照由 save_game_system
    // 读取实际 World，不注入 Roofs JSON 或假关联。
    fn install_snapshot_owners(a: &mut App) {
        use country_core::resources::{
            stairs::{stairs_state::StairsState, stairs_removed_supports::StairsRemovedSupports},
            terrain_editing::TerrainEditingData,
            terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
            world_raster::{DirtyTilePublisher, UploadWorldRasterBuffersToStagingCmd},
            world_raster_defs::{PathRaster, WaterRaster, GardenRaster, Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster},
            walls::decorator::DecoratorIdGen,
        };
        a.insert_resource(TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default()));
        a.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        a.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        a.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        a.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
        a.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        a.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        a.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        let mut stairs = StairsState::new(); stairs.make_backup();
        a.insert_resource(stairs).insert_resource(StairsRemovedSupports::default());
        a.init_resource::<DecoratorIdGen>();
        a.init_resource::<country_core::resources::image_formation::ImageFormationConfigs>();
        a.init_resource::<GladeColorCustomization>();
        a.world_mut().init_resource::<Events<UploadWorldRasterBuffersToStagingCmd>>();
        a.world_mut().init_resource::<Events<country_core::resources::events::ShutdownGameCmd>>();
    }

    fn publish_wall(a: &mut App) {
        use country_core::resources::walls::public_walls::{WallPublicationDirty, WallSpatialHash};
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0., maximum_y: 4., sampling_fix: true };
        a.world_mut().resource_scope(|world, mut public: Mut<PublicWalls>| {
            public.consume_with_paths(&mut world.resource_mut::<InnerWalls>(), &terrain, None,
                &WallPublicationDirty::default(), &WallSpatialHash::default());
        });
        assert!(a.world().resource::<PublicWalls>().get(WallId(1)).is_some());
    }

    fn consume_roof_creation(a: &mut App) -> Entity {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::roofs::Roof;
        a.world_mut().run_system_once(system_roof::create_roof::create_roof).unwrap();
        a.world_mut().run_system_once(system_roof::create_roof::wall_roof_hashmap).unwrap();
        let entity = a.world().resource::<PublicWalls>().roof_entity(WallId(1)).expect("真实屋顶关联");
        assert_eq!(a.world().get::<Roof>(entity).unwrap().wall_id(), WallId(1));
        entity
    }

    #[test]
    fn m06_roof_snapshot_across_processes() {
        let account = tempfile::tempdir().unwrap();
        let installed = std::fs::canonicalize(PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("set TG_INSTALL_ROOT"))).unwrap();
        for phase in ["write", "restore"] {
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "m06_wall_persistence::m06_roof_snapshot_process", "--nocapture"])
                .env("TG_M06_ROOF_PHASE", phase).env("TG_M06_WALL_ACCOUNT", account.path())
                .env("TG_INSTALL_ROOT", &installed).status().unwrap();
            assert!(status.success(), "M06 屋顶快照子进程 {phase} 失败");
            assert!(account.path().join(format!("roof-completed-{phase}")).is_file());
        }
    }

    #[test]
    fn m06_roof_snapshot_process() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::{resources::roofs::{commands::CreateRoofCmd, creation::RoofCreateRequestObserved},
            systems::wall::valid_enclosure::{Shape, ShapeParameters}};
        let Ok(phase) = std::env::var("TG_M06_ROOF_PHASE") else { return; };
        let account = PathBuf::from(std::env::var_os("TG_M06_WALL_ACCOUNT").unwrap());
        let installed = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
        paths::initialize_paths_with_installation(91916, Some(&account), &installed).unwrap();
        let session_path = account.join("m06-roof-session.json");
        let mut a = fixture_app(); install_snapshot_owners(&mut a);
        if phase == "write" {
            let session = SessionId::new(); attach_save(&mut a, session);
            std::fs::write(&session_path, serde_json::to_vec(&session).unwrap()).unwrap();
            let wall = WallState::new(Curve2::try_new(vec![[0., 0.], [4., 0.]], false).unwrap(), 7., 0, 0, false);
            a.world_mut().resource_mut::<InnerWalls>().new_wall_with_id(WallId(1), wall);
            a.world_mut().resource_mut::<InnerWalls>().update_wall_state_enclosure(WallId(1),
                ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d { center: Vec2::new(1., 2.), radius: 3., rotation: 0. })).to_snapshot());
            let source = a.world().resource::<InnerWalls>().get_unwrapped(WallId(1)).snapshot().clone();
            let create = WallHistoryEdit::Create { wall: country_core::resources::walls::publication_fields::WallId(1), source,
                color_id: 0, restored_height: 7., restored_elevation: 0. };
            {
                let mut history = a.world_mut().resource_mut::<H>();
                history.start_input_recording(AppMode::Path.native_tag() as u32).unwrap();
                history.add(EditType::Wall(Arc::new(WallCompoundHistoryEdit { groups: vec![vec![create]] }))).unwrap();
                history.finish_input_recording().unwrap().unwrap();
                history.start_input_recording(AppMode::Path.native_tag() as u32).unwrap();
            }
            a.world_mut().resource_mut::<Events<SetWallStyleCmd>>().send(SetWallStyleCmd {
                wall_id: WallId(1), from: 0, to: 2, record_history: 1, play_sound: 0,
            });
            a.world_mut().resource_mut::<Events<SetPlasterPeelAmountCmd>>().send(SetPlasterPeelAmountCmd {
                wall_id: WallId(1), from: 0., to: 0.75, record_history: 1,
            });
            a.update(); a.world_mut().resource_mut::<H>().finish_input_recording().unwrap().unwrap();
            publish_wall(&mut a);
            let mut request = [0u8; 64]; request[8..16].copy_from_slice(&1u64.to_le_bytes());
            a.world_mut().resource_mut::<Events<CreateRoofCmd>>().send(CreateRoofCmd(RoofCreateRequestObserved::from_observed(request)));
            let roof = consume_roof_creation(&mut a);
            let roof_value = a.world().get::<country_core::resources::roofs::Roof>(roof).unwrap();
            std::fs::write(account.join("m06-roof-expected.json"), serde_json::to_vec(roof_value).unwrap()).unwrap();
            save(&mut a); // 先打开同一真实 session 文件。
            // 游戏正常帧先建立 save_game 的初始比较缓存；随后用户才请求快照。
            a.world_mut().resource_mut::<GameTime>().delta = 0.0;
            a.world_mut().run_system_once(save_game::save_game_system).unwrap();
            a.world_mut().resource_mut::<Events<save_game::SaveGameCmd>>().send(save_game::SaveGameCmd);
            a.world_mut().run_system_once(save_game::save_game_system).unwrap();
            assert!(!a.world().resource::<Events<save_game::SavingGameComplete>>().is_empty());
            let refs = &a.world().resource::<H>().ledger().resolved_snapshots;
            assert_eq!(refs.len(), 1);
            let id = refs[0].value.snapshot_id().expect("保存线程已完成快照");
            assert!(session.save_folder_path().join("snapshots").join(format!("{}.snapshot", id.0)).is_file());
            shutdown_save_thread(a.world_mut()).unwrap();
        } else {
            assert_eq!(phase, "restore");
            let session: SessionId = serde_json::from_slice(&std::fs::read(session_path).unwrap()).unwrap();
            attach_save(&mut a, session);
            let saved: serde_json::Value = serde_json::from_slice(&std::fs::read(session.save_folder_path().join("history.json")).unwrap()).unwrap();
            eprintln!("M06 屋顶保存引用: chain_index={}, snapshots={}", saved["History"]["chain_index"], saved["History"]["snapshots"]);
            let mut manager = LoadGameManager::new();
            manager.begin(LoadGameSource::Path(session.save_folder_path().join("history.json")),
                ActiveLoadOptions { ignore_snapshot: false, request_reveal: false, new_session: false },
                country_core::startup::startup_session::make_post_load_commands(vec![]));
            a.insert_resource(manager).insert_resource(AppMode::LoadGame);
            load_game::try_load_game_system(a.world_mut()).unwrap();
            eprintln!("M06 首次加载: manager={:?}, cursor={}, snapshot_count={}, 墙是否存在={}",
                a.world().resource::<LoadGameManager>().state,
                a.world().resource::<H>().ledger().cursor,
                a.world().resource::<H>().ledger().resolved_snapshots.len(),
                a.world().resource::<InnerWalls>().contains(WallId(1)));
            assert_wall(&a, 2, 0.75);
            publish_wall(&mut a); // 屋顶请求必须等待权威墙已发布。
            let roof = consume_roof_creation(&mut a);
            let actual = serde_json::to_value(a.world().get::<country_core::resources::roofs::Roof>(roof).unwrap()).unwrap();
            // 从捕获的 f32 JSON 读取回原 Roof，再按相同 serde 路径比较；
            // Value 解析十进制为 f64 与 Roof 的 f32→f64 表示并不等价。
            let expected: country_core::resources::roofs::Roof = serde_json::from_slice(&std::fs::read(account.join("m06-roof-expected.json")).unwrap()).unwrap();
            assert_eq!(actual, serde_json::to_value(&expected).unwrap(), "恢复的真实屋顶字段必须与捕获值一致");
            for _ in 0..2 { load_game::try_load_game_system(a.world_mut()).unwrap(); a.update(); }
            assert_eq!(a.world().resource::<LoadGameManager>().state, LoadGameState::Loaded);
            send_history(&mut a, HistoryCommand::UNDO); assert_wall(&a, 0, 0.);
            send_history(&mut a, HistoryCommand::REDO); assert_wall(&a, 2, 0.75);
            assert_eq!(a.world().resource::<PublicWalls>().roof_entity(WallId(1)), Some(roof));
            shutdown_save_thread(a.world_mut()).unwrap();
        }
        std::fs::write(account.join(format!("roof-completed-{phase}")), b"passed").unwrap();
    }
}
