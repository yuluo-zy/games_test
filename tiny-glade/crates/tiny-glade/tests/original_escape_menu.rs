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

/// 此测试只使用CPU控件协议。真实OS输入仍暂停；子进程隔离全局路径和语言表。
#[test]
fn settings_router_builds_pages_and_persists_changes_across_processes() {
    const NAME: &str = "settings_router_builds_pages_and_persists_changes_across_processes";
    if let Some(path) = std::env::var_os("TG_SETTINGS_RESTORE_FILE") {
        let install = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
        let account = PathBuf::from(std::env::var_os("TG_SETTINGS_ACCOUNT").unwrap());
        paths::initialize_paths_with_installation(0, Some(&account), &install).unwrap();
        assert_eq!(tiny_glade::app_config::settings_file_path(), PathBuf::from(path));
        let config = tiny_glade::app_config::get_app_config(&Default::default()).unwrap();
        assert_eq!(config.lang_override.as_deref(), Some("de-DE"));
        assert_eq!(
            config.audio.master_volume.to_bits(),
            std::env::var("TG_SETTINGS_EXPECT_MASTER_BITS")
                .unwrap()
                .parse::<u32>()
                .unwrap()
        );
        return;
    }
    if std::env::var_os("TG_SETTINGS_ROUTER_CHILD").is_none() {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", NAME, "--nocapture"])
            .env("TG_SETTINGS_ROUTER_CHILD", "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout).to_string()
                + &String::from_utf8_lossy(&output.stderr)
        );
        return;
    }
    use country_core::{
        render::{adaptive_render_state::AdaptiveRenderState, render_config::FrameRenderConfig},
        resources::{
            gui::{EscapeMenuJustClosed, PrevEscMenuState},
            mouse_capture::OsCursorVisibilityCmd,
            onboarding::TutorialOp,
            session::{ActiveSession, SessionId, SessionMeta},
            time_of_day::TimeOfDay,
        },
    };
    use tiny_glade::{
        app_config,
        gui::settings::{self, LastAppliedVideoSettings},
    };
    use toolbar::mei::widgets::{ImageContent, LabelContent, tweaker::TweakerContent};
    let install = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
    let account = tempfile::tempdir().unwrap();
    paths::initialize_paths_with_installation(0, Some(account.path()), &install).unwrap();
    lang_lib::configure(install.join("assets/lang"), true).unwrap();
    lang_lib::load_locale("en-US").unwrap();
    let mut app = setup(EscMenuState::Main, AppMode::default());
    {
        let mut mei = app.world_mut().resource_mut::<MeiContext>();
        for name in [
            "menu/settings",
            "menu/image-settings",
            "menu/settings_see_thru",
            "menu/settings-info",
            "menu/settings-info-frame",
            "menu/key_remapping",
            "menu/language",
        ] {
            mei.configs.insert(
                name.into(),
                Arc::new(
                    toolbar::mei::config::read_node_config_from_file(
                        &install.join(format!("assets/ui/{name}.ron")),
                    )
                    .unwrap(),
                ),
            );
        }
    }
    let frame = FrameRenderConfig::at_startup(
        WindowSize::new(1600, 1000),
        &app.world().resource::<GameConfig>().video,
    );
    app.add_plugins((bevy_input::InputPlugin, settings::SettingsMenuPlugin))
        .insert_resource(frame)
        .insert_resource(AdaptiveRenderState::at_startup(false))
        .insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()))
        .init_resource::<TimeOfDay>()
        .init_resource::<PrevEscMenuState>()
        .init_resource::<app_config::PanicReproStateResource>()
        .add_event::<EscapeMenuJustClosed>()
        .add_event::<OsCursorVisibilityCmd>()
        .add_event::<TutorialOp>()
        .add_event::<app_config::WriteSettingsFileCmd>()
        .add_systems(
            Update,
            settings::do_gui_escape_menu_settings
                .after(do_gui_escape_menu)
                .before(country_core::resources::gui::advance_menu_state_system),
        )
        .add_systems(
            Update,
            country_core::resources::gui::advance_menu_state_system,
        );
    let mut save_cursor = bevy_ecs::event::EventCursor::<app_config::WriteSettingsFileCmd>::default();
    let frame = |app: &mut App, cursor: &mut bevy_ecs::event::EventCursor<app_config::WriteSettingsFileCmd>| {
        app.update();
        app_config::consume_settings_at_frame_tail(app.world(), cursor)
    };
    let click = |app: &mut App, text: &str, cursor: &mut bevy_ecs::event::EventCursor<app_config::WriteSettingsFileCmd>| {
        let id = *app
            .world()
            .resource::<MeiContext>()
            .nodes
            .iter()
            .find(|(_, node)| {
                let content: &dyn std::any::Any = node.content.as_ref();
                content
                    .downcast_ref::<MenuButtonContent>()
                    .is_some_and(|c| c.pending_text.as_deref() == Some(text))
            })
            .unwrap_or_else(|| panic!("页面缺真实按钮: {text}"))
            .0;
        pointer_click(app, id);
        frame(app, cursor);
        frame(app, cursor);
    };
    app.update();
    let settings = *app
        .world()
        .resource::<MeiContext>()
        .nodes
        .iter()
        .find(|(_, node)| {
            let content: &dyn std::any::Any = node.content.as_ref();
            content
                .downcast_ref::<PillButtonContent>()
                .is_some_and(|c| c.image == "menu/settings_icon")
        })
        .unwrap()
        .0;
    pointer_click(&mut app, settings);
    app.update();
    app.update();
    assert!(matches!(
        app.world().resource::<EscMenuState>(),
        EscMenuState::Settings(SettingsMenuState::Audio)
    ));
    assert!(
        app.world()
            .resource::<LastAppliedVideoSettings>()
            .0
            .is_some()
    );
    assert!(
        app.world()
            .resource::<MeiContext>()
            .nodes
            .values()
            .any(|node| {
                let content: &dyn std::any::Any = node.content.as_ref();
                content
                    .downcast_ref::<ImageContent>()
                    .is_some_and(|image| image.color == 0xffffffc8)
            })
    );
    let master = lang_lib::lookup("Menu.Settings.Audio.MasterVolume");
    let row = *app
        .world()
        .resource::<MeiContext>()
        .nodes
        .iter()
        .find(|(_, node)| {
            node.content.type_name().ends_with("TweakerContent")
                && node.children.iter().any(|child| {
                    let content: &dyn std::any::Any = app.world().resource::<MeiContext>().nodes
                        [child]
                        .content
                        .as_ref();
                    content
                        .downcast_ref::<LabelContent>()
                        .is_some_and(|label| label.pending_text.as_deref() == Some(&*master))
                })
        })
        .unwrap()
        .0;
    // Dispatch the original right-action to the real tweaker input handler;
    // the settings producer consumes its resulting delta on this frame.
    use toolbar::mei::core::NodeContent;
    let action = country_core::input::action_set::ActionSetAction::Digital {
        slot: 21,
        state: DigitalActionState { phase: DigitalActionPhase::JustPressed, timer_seconds: 0.0 },
    };
    assert!(toolbar::mei::core::NodeContext::new(&mut app.world_mut().resource_mut::<MeiContext>(), row)
        .content_mut::<TweakerContent>()
        .on_input_phase(&action, [true, false]));
    app.update();
    let master_bits = app
        .world()
        .resource::<GameConfig>()
        .audio
        .master_volume
        .to_bits();
    assert_eq!(master_bits, 1.1f32.to_bits());
    assert!(!app_config::settings_file_path().is_file(), "frame tail has not run yet");
    assert!(app_config::consume_settings_at_frame_tail(app.world(), &mut save_cursor));
    assert!(app_config::settings_file_path().is_file());
    let persisted: GameConfig = ron::from_str(&std::fs::read_to_string(app_config::settings_file_path()).unwrap()).unwrap();
    assert_eq!(persisted.audio.master_volume.to_bits(), master_bits);
    assert!(!app_config::consume_settings_at_frame_tail(app.world(), &mut save_cursor));
    // 此CPU入口没有安装需要GPU设备的mei_post_update；撤回本测试注入的单帧delta。
    // 原prepare_layout清理合同由Tweaker控件测试验证，此处仅验页面消费与磁盘持久化。
    toolbar::mei::core::NodeContext::new(&mut app.world_mut().resource_mut::<MeiContext>(), row)
        .content_mut::<TweakerContent>()
        .delta = 0;
    click(
        &mut app,
        &lang_lib::lookup("Menu.Settings.Controls").to_uppercase(),
        &mut save_cursor,
    );
    assert!(matches!(
        app.world().resource::<EscMenuState>(),
        EscMenuState::Settings(SettingsMenuState::Controls)
    ));
    assert!(
        app.world()
            .resource::<MeiContext>()
            .nodes
            .values()
            .any(|node| {
                let content: &dyn std::any::Any = node.content.as_ref();
                content
                    .downcast_ref::<MenuButtonContent>()
                    .is_some_and(|c| {
                        c.pending_text.as_deref()
                            == Some(&*lang_lib::lookup("Menu.Settings.Controls.KeyBindings"))
                    })
            })
    );
    click(&mut app, "文/A", &mut save_cursor);
    assert!(matches!(
        app.world().resource::<EscMenuState>(),
        EscMenuState::Settings(SettingsMenuState::Language)
    ));
    click(&mut app, "Deutsch", &mut save_cursor);
    assert_eq!(
        app.world()
            .resource::<GameConfig>()
            .lang_override
            .as_deref(),
        Some("de-DE")
    );
    click(&mut app, &lang_lib::lookup("Menu.Back"), &mut save_cursor);
    assert!(matches!(
        app.world().resource::<EscMenuState>(),
        EscMenuState::Main
    ));
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", NAME, "--nocapture"])
        .env("TG_SETTINGS_RESTORE_FILE", app_config::settings_file_path())
        .env("TG_SETTINGS_ACCOUNT", account.path())
        .env("TG_SETTINGS_EXPECT_MASTER_BITS", master_bits.to_string())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout).to_string()
            + &String::from_utf8_lossy(&output.stderr)
    );
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
