//! Bounded integration of the genuine Escape GUI with MEI input, owning state,
//! save/audio events and deferred shutdown. Original installed templates/fonts
//! are required; this test never substitutes empty templates or skips assets.
use bevy_app::{App, Update};
use bevy_ecs::{event::Events, schedule::IntoScheduleConfigs};
use bevy_input::mouse::MouseWheel;
use bevy_state::state::State;
use bevy_time::Time;
use country_core::{
    app_loading_state::AppLoadingState,
    game_config::GameConfig,
    game_profile::{GameProfile, SaveGameProfileCmd},
    input::action_set::{DigitalActionPhase, DigitalActionState, MainActionSet},
    resources::{
        events::EscapeMenuJustOpened,
        gui::{EscMenuState, NextEscMenuState, PhotomodeScreen, SettingsMenuState},
        mouse_terrain_raycast::CursorPositionSS,
        session::SavedSessionList,
        shutdown::ShutdownGame,
        window::WindowSize,
    },
    systems::{
        audio::PlayMenuUiSound,
        mode_manager::{AppMode, HistoryCommand, RequestAppModeChange, RestoreAppTo},
        save::save_game::SaveGameCmd,
        steam::SteamOverlayOpenResources,
        timelapse_replay::TimelapseState,
        ui::resources::{
            ui_state::{UiInteraction, UiState, WidgetId},
            widget_params::UiNavState,
        },
    },
};
use glam::Vec2;
use std::{path::PathBuf, sync::Arc};
use tiny_glade::gui::gui_escape_menu::do_gui_escape_menu;
use toolbar::{
    MeiContext,
    mei::{
        core::GlobalId,
        widgets::{MenuButtonContent, PillButtonContent},
    },
};

fn setup(menu: EscMenuState, mode: AppMode) -> App {
    let root = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("set TG_INSTALL_ROOT"));
    let mut mei = MeiContext::default();
    mei.font_table = Arc::new(
        tg_text::FontTable::from_ron(
            &std::fs::read_to_string(root.join("assets/fonts/table.ron")).unwrap(),
        )
        .unwrap(),
    );
    for name in [
        "menu/back",
        "menu/back_w_bug_report",
        "menu/esc",
        "menu/exit",
        "menu/demo-exit",
        "menu/discord",
        "menu/wishlist",
    ] {
        mei.configs.insert(
            name.into(),
            Arc::new(
                toolbar::mei::config::read_node_config_from_file(
                    &root.join(format!("assets/ui/{name}.ron")),
                )
                .unwrap(),
            ),
        );
    }
    let mut app = App::new();
    app.add_event::<MouseWheel>()
        .add_event::<HistoryCommand>()
        .add_event::<EscapeMenuJustOpened>()
        .add_event::<SaveGameCmd>()
        .add_event::<SaveGameProfileCmd>()
        .add_event::<PlayMenuUiSound>()
        .add_event::<RequestAppModeChange>()
        .insert_resource(mei)
        .insert_resource(menu)
        .insert_resource(mode)
        .insert_resource(State::new(AppLoadingState::Game))
        .insert_resource(GameConfig::default())
        .insert_resource(GameProfile::default())
        .insert_resource(NextEscMenuState::default())
        .insert_resource(PhotomodeScreen::default())
        .insert_resource(RestoreAppTo::default())
        .insert_resource(TimelapseState::Inactive)
        .insert_resource(SavedSessionList::default())
        .insert_resource(SteamOverlayOpenResources::default())
        .insert_resource(MainActionSet::default())
        .insert_resource(UiNavState::default())
        .insert_resource(UiState::at_startup())
        .insert_resource(CursorPositionSS(Vec2::splat(500.)))
        .insert_resource(WindowSize::new(1600, 1000))
        .insert_resource(Time::<()>::default())
        .add_systems(
            Update,
            (toolbar::mei::systems::mei_pre_update, do_gui_escape_menu).chain(),
        );
    app
}
fn press(app: &mut App, slot: usize) {
    app.world_mut().resource_mut::<MainActionSet>().digital[slot] = DigitalActionState {
        phase: DigitalActionPhase::JustPressed,
        timer_seconds: 0.,
    };
}
fn drain<E: bevy_ecs::event::Event>(app: &mut App) -> Vec<E> {
    app.world_mut()
        .resource_mut::<Events<E>>()
        .drain()
        .collect()
}

#[test]
fn keyboard_replay_exit_and_real_save_capture_events() {
    let mut app = setup(EscMenuState::Closed, AppMode::Replay);
    press(&mut app, 27);
    app.update();
    assert!(matches!(
        app.world().resource::<NextEscMenuState>().0,
        Some(EscMenuState::Main)
    ));
    assert_eq!(
        drain::<HistoryCommand>(&mut app),
        vec![HistoryCommand {
            bytes: [0, 3, 0, 3]
        }]
    );
    assert_eq!(app.world().resource::<RestoreAppTo>().0, AppMode::default());
    assert!(drain::<RequestAppModeChange>(&mut app).is_empty());
    assert_eq!(drain::<EscapeMenuJustOpened>(&mut app).len(), 1);
    assert_eq!(drain::<SaveGameCmd>(&mut app).len(), 1);
    assert_eq!(drain::<SaveGameProfileCmd>(&mut app).len(), 1);
    assert!(
        drain::<PlayMenuUiSound>(&mut app)
            .iter()
            .any(|s| s.kind == 10)
    );
}

#[test]
fn timelapse_interrupt_and_photo_preservation_use_canonical_state() {
    for active in [TimelapseState::Running, TimelapseState::Restoring] {
        let mut app = setup(EscMenuState::Closed, AppMode::Replay);
        *app.world_mut().resource_mut::<TimelapseState>() = active;
        press(&mut app, 27);
        app.update();
        assert_eq!(
            drain::<HistoryCommand>(&mut app),
            vec![HistoryCommand::INTERRUPT]
        );
        assert!(app.world().resource::<NextEscMenuState>().0.is_none());
        assert!(drain::<EscapeMenuJustOpened>(&mut app).is_empty());
        assert!(drain::<SaveGameCmd>(&mut app).is_empty());
    }
    let mut app = setup(EscMenuState::Closed, AppMode::Photomode);
    app.world_mut().resource_mut::<PhotomodeScreen>().enabled = true;
    press(&mut app, 27);
    app.update();
    assert!(drain::<RequestAppModeChange>(&mut app).is_empty());
    assert!(drain::<HistoryCommand>(&mut app).is_empty());
    assert_eq!(drain::<EscapeMenuJustOpened>(&mut app).len(), 1);
    assert!(
        app.world()
            .resource::<MeiContext>()
            .nodes
            .values()
            .all(|n| n.content.type_name() != "toolbar::mei::widgets::LabelContent")
    );
}

fn pointer_click(app: &mut App, id: GlobalId) {
    let mut mei = app.world_mut().resource_mut::<MeiContext>();
    for node in mei.nodes.values_mut() {
        node.size = Vec2::ZERO;
    }
    let node = mei.nodes.get_mut(&id).unwrap();
    node.pos = Vec2::new(10., 10.);
    node.size = Vec2::new(200., 60.);
    app.world_mut()
        .insert_resource(CursorPositionSS(Vec2::new(20., 20.)));
    app.world_mut().insert_resource(UiState::new(UiInteraction {
        native_tag: 2,
        native_phase: 1,
        widget: WidgetId::from_name(&id.0.to_string()),
    }));
}
#[test]
fn actual_pointer_settings_then_quit_deferred_resource() {
    let mut app = setup(EscMenuState::Main, AppMode::default());
    app.update();
    let settings = app
        .world()
        .resource::<MeiContext>()
        .nodes
        .iter()
        .find_map(|(id, n)| {
            let any: &dyn std::any::Any = n.content.as_ref();
            any.downcast_ref::<PillButtonContent>()
                .filter(|c| c.image == "menu/settings_icon")
                .map(|_| *id)
        })
        .unwrap();
    pointer_click(&mut app, settings);
    app.update();
    assert!(matches!(
        app.world().resource::<NextEscMenuState>().0,
        Some(EscMenuState::Settings(SettingsMenuState::Audio))
    ));
    assert!(
        drain::<PlayMenuUiSound>(&mut app)
            .iter()
            .any(|s| s.kind == 3)
    );

    let mut app = setup(EscMenuState::Quit, AppMode::default());
    app.update();
    let label = lang_lib::lookup("Menu.Quit").to_uppercase();
    let quit = app
        .world()
        .resource::<MeiContext>()
        .nodes
        .iter()
        .find_map(|(id, n)| {
            let any: &dyn std::any::Any = n.content.as_ref();
            any.downcast_ref::<MenuButtonContent>()
                .filter(|c| c.pending_text.as_deref() == Some(label.as_str()))
                .map(|_| *id)
        })
        .unwrap();
    assert!(!app.world().contains_resource::<ShutdownGame>());
    pointer_click(&mut app, quit);
    app.update();
    assert!(app.world().contains_resource::<ShutdownGame>());
}

#[test]
fn original_back_cancel_hotkey_closes_main_without_reopening() {
    let mut app = setup(EscMenuState::Main, AppMode::default());
    app.update();
    drain::<PlayMenuUiSound>(&mut app);
    press(&mut app, 18);
    app.update();
    assert!(matches!(
        app.world().resource::<NextEscMenuState>().0,
        Some(EscMenuState::Closed)
    ));
    assert!(drain::<EscapeMenuJustOpened>(&mut app).is_empty());
    assert!(drain::<SaveGameCmd>(&mut app).is_empty());
    assert!(
        drain::<PlayMenuUiSound>(&mut app)
            .iter()
            .any(|s| s.kind == 11)
    );
}
