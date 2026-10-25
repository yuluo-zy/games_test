use super::*;
use bevy_app::{App, Update};
use bevy_input::mouse::MouseWheel;
use country_core::{
    input::action_set::{ActionSetStates, DigitalActionPhase, DigitalActionState},
    resources::{mouse_terrain_raycast::CursorPositionSS, window::WindowSize},
    systems::{audio::PlayMenuUiSound, ui::resources::widget_params::UiNavState},
};
use std::{any::Any, path::Path, sync::Arc};
use tg_text::Edit;

fn app() -> App {
    let mut mei = MeiContext::default();
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/welcome");
    mei.font_table = Arc::new(
        tg_text::FontTable::from_ron(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/welcome/font-table.ron"
        )))
        .unwrap(),
    );
    for (key, file) in [
        ("menu/welcome", "welcome.ron"),
        ("menu/welcome_demo", "welcome_demo.ron"),
        ("steamdeck/welcome_1", "steamdeck_welcome_1.ron"),
        ("steamdeck/welcome_2", "steamdeck_welcome_2.ron"),
    ] {
        let config = crate::mei::config::read_node_config_from_file(&fixtures.join(file)).unwrap();
        mei.configs.insert(key.into(), Arc::new(config));
    }
    let mut vermeer = VermeerState::default();
    vermeer.deactivate();
    let mut app = App::new();
    app.insert_resource(mei)
        .insert_resource(vermeer)
        .init_resource::<WelcomeScreenState>()
        .init_resource::<EscMenuState>()
        .init_resource::<GameConfig>()
        .init_resource::<GameProfile>()
        .insert_resource(GameTime {
            delta: 0.15,
            ..Default::default()
        })
        .insert_resource(bevy_time::Time::<()>::default())
        .insert_resource(WindowSize::new(960, 540))
        // Interactive nodes register even without pointer hover, including
        // the opaque EmptyContent root. Supply the canonical owner just as
        // the application does; this fixture still tests keyboard handling.
        .insert_resource(CursorPositionSS(glam::Vec2::splat(-100.)))
        .insert_resource(country_core::systems::ui::resources::ui_state::UiState::at_startup())
        .init_resource::<UiNavState>()
        .init_resource::<ActionSetStates>()
        .add_event::<MouseWheel>()
        .add_event::<PlayMenuUiSound>()
        .add_event::<SaveGameProfileCmd>()
        .add_systems(Update, (crate::mei::systems::mei_pre_update, ui).chain());
    app
}

fn save_count(app: &mut App) -> usize {
    app.world_mut()
        .resource_mut::<Events<SaveGameProfileCmd>>()
        .drain()
        .count()
}

fn confirm(app: &mut App) {
    // pre_update's native navigation retain gate requires active nav hold.
    app.world_mut().resource_mut::<UiNavState>().hold = true;
    app.world_mut().resource_mut::<ActionSetStates>().digital[19] = DigitalActionState {
        phase: DigitalActionPhase::JustPressed,
        timer_seconds: 0.,
    };
    app.update();
    app.world_mut().resource_mut::<ActionSetStates>().digital[19] = DigitalActionState::default();
}

fn buttons(app: &App) -> Vec<&MenuButtonContent> {
    app.world()
        .resource::<MeiContext>()
        .nodes
        .values()
        .filter_map(|node| {
            let any: &dyn Any = node.content.as_ref();
            any.downcast_ref::<MenuButtonContent>()
        })
        .collect()
}

#[test]
fn welcome_desktop_real_template_confirm_consumer_and_profile_contract() {
    let mut app = app();
    let persisted_before = ron::to_string(app.world().resource::<GameProfile>()).unwrap();
    app.update();
    assert_eq!(
        *app.world().resource::<WelcomeScreenState>(),
        WelcomeScreenState::Showing { elapsed: 0. }
    );
    assert!(app.world().resource::<MeiContext>().nodes.is_empty());
    app.update();
    assert!(app.world().resource::<MeiContext>().nodes.len() > 8);
    assert!(
        app.world()
            .resource::<MeiContext>()
            .nodes
            .values()
            .any(|n| n.local_opacity == 0.5)
    );
    let button = buttons(&app);
    assert_eq!(button.len(), 1);
    assert_eq!(
        button[0].pending_text.as_deref(),
        Some(format!("{} 🐑", lang_lib::lookup("Welcome.LetsGo")).as_str())
    );
    assert_eq!((button[0].hotkey, button[0].action), (19, 19));
    assert_eq!(
        button[0]
            .text
            .editor()
            .with_buffer(|b| b.metrics().font_size),
        27.
    );
    assert_eq!(save_count(&mut app), 0);
    confirm(&mut app);
    assert_eq!(
        *app.world().resource::<WelcomeScreenState>(),
        WelcomeScreenState::Closed
    );
    let profile = app.world().resource::<GameProfile>();
    assert!(profile.welcome_was_dismissed());
    assert!(!profile.tutorial);
    assert_eq!(ron::to_string(profile).unwrap(), persisted_before);
    assert!(app.world().resource::<EscMenuState>().is_closed());
    assert_eq!(save_count(&mut app), 1);
    app.update();
    assert_eq!(save_count(&mut app), 0);
}

#[test]
fn welcome_waits_for_closed_menu_and_reveal_precedes_cancel() {
    let mut app = app();
    *app.world_mut().resource_mut::<EscMenuState>() = EscMenuState::Main;
    app.update();
    assert_eq!(
        *app.world().resource::<WelcomeScreenState>(),
        WelcomeScreenState::Pending
    );
    *app.world_mut().resource_mut::<EscMenuState>() = EscMenuState::Closed;
    app.update();
    app.update();
    app.world_mut()
        .resource_mut::<VermeerState>()
        .activate(false, true);
    *app.world_mut().resource_mut::<EscMenuState>() = EscMenuState::Main;
    app.update();
    assert_eq!(
        *app.world().resource::<WelcomeScreenState>(),
        WelcomeScreenState::Showing { elapsed: 0. }
    );
    assert!(
        !app.world()
            .resource::<GameProfile>()
            .welcome_was_dismissed()
    );
    assert_eq!(save_count(&mut app), 0);
    assert!(app.world().resource::<MeiContext>().nodes.is_empty());
    app.world_mut().resource_mut::<VermeerState>().deactivate();
    app.update();
    assert_eq!(
        *app.world().resource::<WelcomeScreenState>(),
        WelcomeScreenState::Closed
    );
    assert!(
        app.world()
            .resource::<GameProfile>()
            .welcome_was_dismissed()
    );
    assert_eq!(save_count(&mut app), 1);
}

#[test]
fn welcome_skip_and_existing_dismissal_do_not_save_or_draw() {
    for dismissed in [false, true] {
        let mut app = app();
        app.world_mut()
            .resource_mut::<GameConfig>()
            .dev
            .skip_welcome = !dismissed;
        if dismissed {
            app.world_mut()
                .resource_mut::<GameProfile>()
                .dismiss_welcome();
        }
        app.update();
        assert_eq!(
            *app.world().resource::<WelcomeScreenState>(),
            WelcomeScreenState::Closed
        );
        assert!(app.world().resource::<MeiContext>().nodes.is_empty());
        assert_eq!(
            app.world()
                .resource::<GameProfile>()
                .welcome_was_dismissed(),
            dismissed
        );
        assert_eq!(save_count(&mut app), 0);
    }
}

#[test]
fn welcome_demo_uses_its_real_distinct_template() {
    let mut full = app();
    full.update();
    full.update();
    let mut demo = app();
    demo.world_mut().resource_mut::<GameConfig>().dev.demo = true;
    demo.update();
    demo.update();
    let ids = |app: &App| {
        let mut ids: Vec<_> = app
            .world()
            .resource::<MeiContext>()
            .nodes
            .keys()
            .copied()
            .collect();
        ids.sort();
        ids
    };
    assert_ne!(ids(&full), ids(&demo));
    assert_eq!(buttons(&demo).len(), 1);
    confirm(&mut demo);
    assert_eq!(save_count(&mut demo), 1);
}

struct DeckSdk;
impl country_core::resources::steam::SteamSdk for DeckSdk {
    fn is_steam_running_on_steam_deck(&self) -> bool {
        true
    }
    fn user_steam_id(&self) -> u64 {
        unreachable!()
    }
    fn run_frame(&self) {
        unreachable!()
    }
    fn connected_controllers(&self) -> Vec<u64> {
        unreachable!()
    }
    fn action_set_handle(&self, _: &str) -> Option<u64> {
        unreachable!()
    }
    fn analog_action_handle(&self, _: &str) -> Option<u64> {
        unreachable!()
    }
    fn digital_action_handle(&self, _: &str) -> Option<u64> {
        unreachable!()
    }
    fn analog_action_data(&self, _: u64, _: u64) -> country_core::resources::steam::AnalogActionData {
        unreachable!()
    }
    fn digital_action_data(&self, _: u64, _: u64) -> country_core::resources::steam::DigitalActionData {
        unreachable!()
    }
    fn activate_action_set(&self, _: u64, _: u64) {
        unreachable!()
    }
    fn init_input(&self) -> bool {
        unreachable!()
    }
    fn show_binding_panel(&self, _: u64) {
        unreachable!()
    }
    fn activate_game_overlay_to_web_page(&self, _: &str) {
        unreachable!()
    }
    fn hook_screenshots(&self, _: bool) {
        unreachable!()
    }
    fn show_floating_gamepad_text_input(&self, _: u32, _: i32, _: i32, _: i32, _: i32) {
        unreachable!()
    }
    fn analog_action_origins(&self, _: u64, _: u64, _: u64) -> Vec<u64> {
        unreachable!()
    }
    fn digital_action_origins(&self, _: u64, _: u64, _: u64) -> Vec<u64> {
        unreachable!()
    }
    fn glyph_for_action_origin(&self, _: u64) -> String {
        unreachable!()
    }
}

#[test]
fn welcome_deck_preserves_both_pages_and_only_last_confirmation_saves() {
    let mut app = app();
    app.insert_resource(SteamClientResource {
        client: Some(Arc::new(DeckSdk)),
    });
    app.update();
    app.update();
    assert_eq!(buttons(&app)[0].pending_text.as_deref(), Some("➡️"));
    assert_eq!(
        buttons(&app)[0]
            .text
            .editor()
            .with_buffer(|b| b.metrics().font_size),
        40.5
    );
    confirm(&mut app);
    assert!(
        !app.world()
            .resource::<GameProfile>()
            .welcome_was_dismissed()
    );
    assert_eq!(save_count(&mut app), 0);
    let elapsed = *app.world().resource::<WelcomeScreenState>();
    app.update();
    assert_eq!(*app.world().resource::<WelcomeScreenState>(), elapsed);
    assert!(
        app.world()
            .resource::<MeiContext>()
            .nodes
            .values()
            .all(|n| n.local_opacity == 1.)
    );
    assert_eq!(
        buttons(&app)[0].pending_text.as_deref(),
        Some(format!("{} 🐑", lang_lib::lookup("Welcome.LetsGo")).as_str())
    );
    confirm(&mut app);
    assert_eq!(
        *app.world().resource::<WelcomeScreenState>(),
        WelcomeScreenState::Closed
    );
    assert_eq!(save_count(&mut app), 1);
}

#[test]
fn welcome_fade_preserves_native_limits_midpoint_and_nan() {
    assert_eq!(welcome_opacity(-1.), 0.);
    assert_eq!(welcome_opacity(0.), 0.);
    assert_eq!(welcome_opacity(0.15), 0.5);
    assert_eq!(welcome_opacity(0.3), 1.);
    assert_eq!(welcome_opacity(f32::INFINITY), 1.);
    assert!(welcome_opacity(f32::NAN).is_nan());
}
