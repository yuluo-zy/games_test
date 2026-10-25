//! Candidate integration test for tiny-glade/tests/original_session_control.rs.
//! Requires session-lifecycle-combined.patch + parsed guard + Load36 registration.
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
        std::fs::File::open(id.save_folder_path().join("history.json"))
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
