//! Staged central integration test. Install under tiny-glade/tests, then run
//! explicitly with TG_INSTALL_ROOT. No Cargo/run success is claimed here.
//! Uses shipped nonempty v27 history/templates and actual MEI input consumers.
use bevy_app::{App, Update};
use bevy_ecs::{
    event::Events,
    schedule::{IntoScheduleConfigs, common_conditions::on_event},
};
use bevy_input::mouse::MouseWheel;
use bevy_state::state::NextState;
use bevy_time::Time;
use country_core::{
    app_loading_state::AppLoadingState,
    game_config::GameConfig,
    input::action_set::{DigitalActionPhase, DigitalActionState, MainActionSet},
    resources::{
        gui::{EscMenuState, LoadMenuBackupState, LoadMenuState, NextEscMenuState},
        history::{History, HistoryIdentity},
        mouse_terrain_raycast::CursorPositionSS,
        session::{
            ActiveSession, GladeType, NextSession, NextSessionKind, SavedSessionList, SessionId,
            SessionMeta, SessionMutator,
        },
        window::WindowSize,
    },
    systems::{
        audio::PlayMenuUiSound,
        save::{
            load_completion_checker::SessionLoadReadiness,
            save_game::{
                AsyncSaveCmd, AutoSaveState, DeleteSessionCmd, GameSaveSections, SaveFileSection,
                SerializedJson,
            },
            save_thread::SaveThreadHandle,
        },
        ui::resources::{
            ui_state::{UiInteraction, UiState, WidgetId},
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
    mei::{core::GlobalId, widgets::MenuButtonContent},
    menu::{load::ThumbnailScrollContent, load_ui},
};

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for e in std::fs::read_dir(from).unwrap() {
        let p = e.unwrap().path();
        let q = to.join(p.file_name().unwrap());
        if p.is_dir() {
            copy_tree(&p, &q)
        } else {
            std::fs::copy(p, q).unwrap();
        }
    }
}
fn app(state: LoadMenuState, active: SessionId, installed: &Path) -> App {
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
    let mut app = App::new();
    app.add_event::<MouseWheel>()
        .add_event::<PlayMenuUiSound>()
        .add_event::<DeleteSessionCmd>()
        .insert_resource(mei)
        .insert_resource(ActiveSession::new(active, SessionMeta::default()))
        .insert_resource(History::empty_glade())
        .insert_resource(AutoSaveState::default())
        .insert_resource(EscMenuState::Load(state))
        .insert_resource(NextEscMenuState::default())
        .insert_resource(NextState::<AppLoadingState>::Unchanged)
        .insert_resource(SavedSessionList {
            sessions: country_core::resources::session::load_saved_session_list(),
        })
        .insert_resource(GameConfig::default())
        .insert_resource(MainActionSet::default())
        .insert_resource(UiNavState::default())
        .insert_resource(UiState::at_startup())
        .insert_resource(WindowSize::new(1600, 1000))
        .insert_resource(CursorPositionSS(Vec2::splat(500.)))
        .insert_resource(Time::<()>::default())
        .add_systems(
            Update,
            (
                toolbar::mei::systems::mei_pre_update,
                load_ui::ui,
                (
                    country_core::resources::session::delete_session_system
                        .run_if(on_event::<DeleteSessionCmd>),
                    country_core::resources::session::refresh_session_list_system
                        .run_if(on_event::<DeleteSessionCmd>),
                )
                    .chain(),
            )
                .chain(),
        );
    app
}
fn press(app: &mut App, slot: usize) {
    let mut actions = app.world_mut().resource_mut::<MainActionSet>();
    *actions = MainActionSet::default();
    actions.digital[slot] = DigitalActionState {
        phase: DigitalActionPhase::JustPressed,
        timer_seconds: 0.,
    };
    drop(actions);
    app.update();
}
fn focus_scroll(app: &mut App) {
    let id = app
        .world()
        .resource::<MeiContext>()
        .nodes
        .iter()
        .find_map(|(id, n)| {
            let any: &dyn std::any::Any = n.content.as_ref();
            any.downcast_ref::<ThumbnailScrollContent>().map(|_| *id)
        })
        .unwrap();
    let mut nav = app.world_mut().resource_mut::<UiNavState>();
    nav.focus = UiFocus {
        tag: 2,
        _pad: 0,
        payload: id.0,
    };
    nav.hold = true;
}
fn select_index(app: &mut App, index: usize) {
    focus_scroll(app);
    if index == 0 {
        press(app, 20);
    } else {
        for _ in 0..index {
            press(app, 21);
        }
    }
    press(app, 19);
}
fn label_id(app: &App, text: &str) -> GlobalId {
    app.world()
        .resource::<MeiContext>()
        .nodes
        .iter()
        .find_map(|(id, n)| {
            let any: &dyn std::any::Any = n.content.as_ref();
            any.downcast_ref::<MenuButtonContent>()
                .filter(|c| c.pending_text.as_deref() == Some(text))
                .map(|_| *id)
        })
        .unwrap_or_else(|| panic!("actual menu text is absent: {text}"))
}
fn click_label(app: &mut App, text: &str) {
    let id = label_id(app, text);
    let mut mei = app.world_mut().resource_mut::<MeiContext>();
    for n in mei.nodes.values_mut() {
        n.size = Vec2::ZERO;
    }
    let n = mei.nodes.get_mut(&id).unwrap();
    n.pos = Vec2::new(10., 10.);
    n.size = Vec2::new(200., 60.);
    drop(mei);
    *app.world_mut().resource_mut::<MainActionSet>() = MainActionSet::default();
    app.world_mut()
        .insert_resource(CursorPositionSS(Vec2::new(20., 20.)));
    app.world_mut().insert_resource(UiState::new(UiInteraction {
        native_tag: 2,
        native_phase: 1,
        widget: WidgetId::from_name(&id.0.to_string()),
    }));
    app.update();
}
fn sounds(app: &mut App) -> Vec<u8> {
    app.world_mut()
        .resource_mut::<Events<PlayMenuUiSound>>()
        .drain()
        .map(|s| s.kind)
        .collect()
}
fn backup_date(time: SystemTime) -> String {
    utils::local_time::system_time_to_local_time(time)
        .format(
            &time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                .unwrap(),
        )
        .unwrap()
}

#[test]
fn original_nonempty_load_modal_delete_theme_and_sync_backup_owner_chains() {
    let installed =
        PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("set TG_INSTALL_ROOT"));
    let installed = std::fs::canonicalize(installed).unwrap();
    let account = tempfile::tempdir().unwrap();
    paths::initialize_paths_with_installation(91901, Some(account.path()), &installed).unwrap();
    let previous = std::env::current_dir().unwrap();
    std::env::set_current_dir(&installed).unwrap();
    let fixture = installed.join("assets/starting-builds/full/15-lake-house");
    let original: serde_json::Value =
        serde_json::from_slice(&std::fs::read(fixture.join("history.json")).unwrap()).unwrap();
    assert_eq!(original["Version"], 27);
    assert!(
        !original["History"]["edit_chain"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let ids: [SessionId; 4] = std::array::from_fn(|_| SessionId::new());
    for (i, id) in ids.iter().enumerate() {
        copy_tree(&fixture, &id.save_folder_path());
        SessionMeta::default().write_to_disk(*id).unwrap();
        std::fs::File::open(id.save_folder_path().join("history.json"))
            .unwrap()
            .set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(1_750_000_000 + i as u64))
            .unwrap();
    }
    let listed = country_core::resources::session::load_saved_session_list();
    assert_eq!(listed.len(), 4);
    let active = listed[0].id;
    let selected = listed[1].id;

    // Real keyboard navigation/confirm through the retained scroll content.
    let mut current = app(LoadMenuState::default(), active, &installed);
    current.update();
    select_index(&mut current, 1);
    assert!(matches!(
        current.world().resource::<NextEscMenuState>().0,
        Some(EscMenuState::Closed)
    ));
    assert!(!current.world().contains_resource::<NextSession>());
    assert!(sounds(&mut current).contains(&8));
    let mut normal = app(LoadMenuState::default(), active, &installed);
    normal.update();
    select_index(&mut normal, 2);
    let request = normal.world().resource::<NextSession>();
    assert_eq!(request.kind, NextSessionKind::LoadParsedGame);
    assert_eq!(request.id, selected);
    let parsed = request
        .parsed_data
        .as_ref()
        .expect("real owning parsed payload");
    assert!(!parsed.edits.is_empty());
    assert_eq!(parsed.version, 27);
    assert!(parsed.migration_metadata.is_some());
    assert!(matches!(
        normal.world().resource::<NextState<AppLoadingState>>(),
        NextState::Pending(AppLoadingState::Reload)
    ));
    assert!(sounds(&mut normal).contains(&8));

    let mut demo = app(LoadMenuState::default(), active, &installed);
    demo.world_mut().resource_mut::<GameConfig>().dev.demo = true;
    demo.update();
    select_index(&mut demo, 4);
    assert!(demo.world().resource::<NextEscMenuState>().0.is_none());
    assert!(!demo.world().contains_resource::<NextSession>());
    assert!(sounds(&mut demo).contains(&12));

    // Real delete modal input -> canonical event consumer -> rename -> refresh.
    let mut deleting = app(
        LoadMenuState {
            warning_session: Some(selected),
            ..Default::default()
        },
        active,
        &installed,
    );
    deleting.update();
    click_label(&mut deleting, &lang_lib::lookup("Continue.Yep"));
    assert!(!selected.save_folder_path().exists());
    assert!(
        paths::deleted_saves()
            .join(selected.0.to_string())
            .join("history.json")
            .is_file()
    );
    assert!(
        deleting
            .world()
            .resource::<SavedSessionList>()
            .sessions
            .iter()
            .all(|s| s.id != selected)
    );

    let theme_id = listed[2].id;
    let metadata = SessionMeta {
        glade: GladeType::Summer,
        mutators: vec![SessionMutator::Halloween2025],
        image: None,
    };
    metadata.write_to_disk(theme_id).unwrap();
    let mut switching = app(
        LoadMenuState {
            switch_glade_type: Some(theme_id),
            ..Default::default()
        },
        active,
        &installed,
    );
    switching.update();
    click_label(&mut switching, &lang_lib::lookup("New.Glade.Winter"));
    let metadata =
        SessionMeta::try_load_from_path(theme_id.save_folder_path().join("meta.json")).unwrap();
    assert_eq!(metadata.glade, GladeType::Winter);
    assert_eq!(metadata.mutators, vec![SessionMutator::Halloween2025]);
    assert_eq!(switching.world().resource::<NextSession>().id, theme_id);
    assert!(
        switching
            .world()
            .resource::<NextSession>()
            .parsed_data
            .as_ref()
            .is_some_and(|d| !d.edits.is_empty())
    );

    // Backup restore uses the real synchronous save worker, preserving the
    // current active history before transferring the selected parsed backup.
    let restore_id = listed[3].id;
    let folder = restore_id.backup_folder_path();
    std::fs::create_dir_all(&folder).unwrap();
    let valid = folder.join("history-auto-1.json");
    let broken = folder.join("history-auto-2.json");
    std::fs::copy(fixture.join("history.json"), &valid).unwrap();
    std::fs::write(&broken, b"invalid JSON").unwrap();
    let valid_time = SystemTime::UNIX_EPOCH + Duration::from_secs(1_740_000_000);
    let bad_time = valid_time + Duration::from_secs(10);
    std::fs::File::open(&valid)
        .unwrap()
        .set_modified(valid_time)
        .unwrap();
    std::fs::File::open(&broken)
        .unwrap()
        .set_modified(bad_time)
        .unwrap();
    let mut restoring = app(
        LoadMenuState {
            backups: Some(LoadMenuBackupState {
                session: restore_id,
                page: 0,
            }),
            ..Default::default()
        },
        active,
        &installed,
    );
    let source_identity = original["Identity"].as_u64().unwrap();
    let active_identity = source_identity ^ 1;
    restoring
        .world_mut()
        .resource_mut::<History>()
        .set_identity(HistoryIdentity(active_identity));
    let mut current_payload = original.clone();
    current_payload["Identity"] = active_identity.into();
    let sections = GameSaveSections {
        values: std::array::from_fn(|i| {
            let section = SaveFileSection::ALL[i];
            let parent = if i < 7 {
                &current_payload
            } else {
                &current_payload["History"]
            };
            SerializedJson::new(&parent[section.json_key()]).unwrap()
        }),
    };
    let edits = current_payload["History"]["edit_chain"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| SerializedJson::new(e).unwrap())
        .collect();
    let worker = SaveThreadHandle::spawn_with_unsaved(
        restoring
            .world()
            .resource::<AutoSaveState>()
            .unsaved_files
            .clone(),
    );
    worker
        .command_sender
        .send(AsyncSaveCmd::OpenFile {
            session: active,
            sections,
            edits,
        })
        .unwrap();
    restoring
        .world_mut()
        .resource_mut::<AutoSaveState>()
        .attach_command_sender(worker.command_sender.clone());
    restoring.update();
    click_label(&mut restoring, &backup_date(bad_time));
    assert!(folder.join("corrupted-history-auto-2.json").is_file());
    assert!(!restoring.world().contains_resource::<NextSession>());
    // The error owns the renamed corrupted path; the next real list rebuild
    // discovers that name before its row can draw the error panel.
    restoring.world_mut().insert_resource(UiState::at_startup());
    restoring
        .world_mut()
        .insert_resource(CursorPositionSS(Vec2::splat(500.)));
    restoring.update();
    assert!(
        restoring
            .world()
            .resource::<MeiContext>()
            .nodes
            .values()
            .any(|n| {
                let any: &dyn std::any::Any = n.content.as_ref();
                any.downcast_ref::<toolbar::mei::widgets::LabelContent>()
                    .is_some_and(|c| {
                        c.pending_text
                            .as_deref()
                            .is_some_and(|s| s.starts_with("Context: "))
                    })
            })
    );
    click_label(&mut restoring, &backup_date(valid_time));
    let request = restoring.world().resource::<NextSession>();
    assert_eq!(request.id, restore_id);
    assert_eq!(request.kind, NextSessionKind::LoadParsedGame);
    assert_eq!(
        request.parsed_data.as_ref().unwrap().identity,
        Some(HistoryIdentity(source_identity))
    );
    let manual: Vec<_> = std::fs::read_dir(&folder)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("history-manual-")
        })
        .collect();
    assert_eq!(manual.len(), 1);
    let saved: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&manual[0]).unwrap()).unwrap();
    assert_eq!(saved["Identity"], active_identity);
    assert_eq!(saved["Camera"], original["Camera"]);
    assert_eq!(saved["History"], original["History"]);
    drop(restoring);
    drop(worker.command_sender);
    worker.join.join().unwrap();
    std::env::set_current_dir(previous).unwrap();
}
