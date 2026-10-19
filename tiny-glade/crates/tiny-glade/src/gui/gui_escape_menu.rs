//! Original B Escape GUI and its complete MEI caller tree.
//!
//! Native1403cf9b0/3634B, wishlist1403d07f0/477B and all17 slot
//! monomorphizations are retained under evidence/round12/escape-ui. T13's
//! digital31 and ReportBug(false) producers are explicitly excluded by user.
//! State, save, sound, replay and shutdown owners remain their canonical types.
use bevy_ecs::prelude::*;
use bevy_state::state::State;
use country_core::{
    app_loading_state::AppLoadingState,
    game_config::GameConfig,
    game_profile::{GameProfile, SaveGameProfileCmd},
    input::action_set::MainActionSet,
    resources::{
        events::EscapeMenuJustOpened,
        gui::{EscMenuState, LoadMenuState, NextEscMenuState, PhotomodeScreen, SettingsMenuState},
        session::SavedSessionList,
        shutdown::ShutdownGame,
    },
    systems::{
        audio::PlayMenuUiSound,
        mode_manager::{AppMode, HistoryCommand, RequestAppModeChange, RestoreAppTo},
        save::save_game::SaveGameCmd,
        steam::SteamOverlayOpenResources,
        timelapse_replay::TimelapseState,
    },
};
use toolbar::{
    MeiContext,
    mei::{
        core::NodeContext,
        widgets::{LabelContent, MenuButtonContent, PillButtonContent},
    },
};

pub const COMMUNITY_SERVER_URL: &str = "https://discord.gg/jcae7kejne";
pub const NEWSLETTER_URL: &str =
    "https://dashboard.mailerlite.com/forms/10395/78076204263933906/share";
pub const STORE_URL: &str = "https://store.steampowered.com/app/2198150/Tiny_Glade/";
pub type SteamOverlay = SteamOverlayOpenResources;

/// Ordered effects of the original GUI. The host applies each immediately;
/// this also exposes a semantic trace for independent interaction validation.
#[derive(Debug)]
pub enum MenuEffect {
    Next(EscMenuState),
    History(HistoryCommand),
    RequestMode(AppMode),
    RestoreAfterReplay(AppMode),
    Opened,
    Sound(u8),
    SaveGame,
    SaveProfile,
    Shutdown,
}

/// The native template/button policy reads current state throughout this call;
/// writing Next does not change which other panels are built in the same frame.
#[allow(clippy::too_many_arguments)]
pub fn build_escape_menu(
    mei: &mut MeiContext,
    game_is_active: bool,
    main_menu_just_pressed: bool,
    timelapse_active: bool,
    menu: &EscMenuState,
    mode: AppMode,
    photo_screen: &PhotomodeScreen,
    config: &GameConfig,
    profile: &GameProfile,
    saved_session_count: usize,
    overlay: &mut SteamOverlay,
    mut emit: impl FnMut(MenuEffect),
) {
    let build = mei.safe_frame("demo build", false).template("menu/back");
    if !photo_screen.enabled {
        // Native format metadata: full uses `v{version}`, demo `DEMO v{version}`.
        let text = if config.dev.demo {
            "DEMO v1.14.3"
        } else {
            "v1.14.3"
        };
        build.with_slot("build_id", |slot| {
            slot.label(text)
                .content_mut::<LabelContent>()
                .text
                .set_metrics(20.)
                .expect("escape build label metrics");
        });
    }

    let (mut opening, mut closing) = (false, false);
    if game_is_active && main_menu_just_pressed {
        if timelapse_active {
            emit(MenuEffect::History(HistoryCommand::INTERRUPT));
        } else if menu.is_closed() {
            opening = true;
        } else {
            closing = true;
        }
    }
    // The original independent digital31 feedback branch is excluded (T13).
    if menu.is_closed() && !photo_screen.enabled {
        let back = mei
            .safe_frame("back", false)
            .template("menu/back_w_bug_report");
        back.with_slot("buttons", |slot| {
            // Same original closure also creates the excluded report-bug pill.
            let settings = slot.back_button("menu/settings_icon");
            settings.content_mut::<PillButtonContent>().image = "menu/settings_icon".into();
            settings.content_mut::<PillButtonContent>().strength =
                if config.ui.auto_hide_toolbar { 0.5 } else { 1. };
            opening |= settings.node().used();
        });
    } else if matches!(menu, EscMenuState::Main) {
        let back = mei.safe_frame("back", false).template("menu/back");
        back.with_slot("button", |slot| {
            let button = slot.back_button("menu/back");
            button.content_mut::<PillButtonContent>().action = 18;
            closing |= button.node().used();
        });
    }
    if opening {
        emit(MenuEffect::Next(EscMenuState::Main));
        if mode == AppMode::Replay {
            emit(MenuEffect::History(HistoryCommand {
                bytes: [0, 3, 0, 3],
            }));
            emit(MenuEffect::RestoreAfterReplay(AppMode::default()));
        } else if mode != AppMode::Photomode {
            emit(MenuEffect::RequestMode(AppMode::default()));
        }
        emit(MenuEffect::Opened);
        emit(MenuEffect::Sound(10));
        emit(MenuEffect::SaveGame);
        emit(MenuEffect::SaveProfile);
    }
    if closing {
        emit(MenuEffect::Sound(11));
        emit(MenuEffect::Next(EscMenuState::Closed));
    }

    let show_discord =
        profile.tutorial_cooldowns.time_since_mouse_moved > 600. || saved_session_count > 2;
    match menu {
        EscMenuState::Main => {
            if config.dev.demo {
                let social = mei.safe_frame("wishlist", true).template("menu/wishlist");
                if show_discord {
                    social.with_slot("discord", |slot| social_discord(slot, 70., overlay));
                }
                social.with_slot("wishlist", |slot| wishlist_button(slot, 1., overlay));
            } else {
                let social = mei
                    .safe_frame("frame_discord", true)
                    .template("menu/discord");
                social.with_slot("newsletter", |slot| social_newsletter(slot, overlay));
                social.with_slot("discord", |slot| social_discord(slot, 84., overlay));
            }
            let screen = mei.safe_frame("esc screen", true).template("menu/esc");
            screen.with_slot("logo", |slot| top_logo(slot, config.dev.demo));
            screen.with_slot("contents", |slot| {
                let continue_button = menu_button(slot, "Menu.Continue");
                continue_button.node_mut().default_focus();
                continue_button.decorate_hovered();
                if continue_button.node().used() {
                    emit(MenuEffect::Sound(11));
                    emit(MenuEffect::Next(EscMenuState::Closed));
                }
                let new = menu_button(slot, "Menu.New");
                new.node_mut().set_sounds(0, 6);
                if new.node().used() {
                    emit(MenuEffect::Next(EscMenuState::NewSession));
                }
                let load = menu_button(slot, "Menu.Load");
                load.node_mut().set_sounds(0, 6);
                if load.node().used() {
                    emit(MenuEffect::Next(EscMenuState::Load(
                        LoadMenuState::default(),
                    )));
                }
            });
            screen.with_slot("buttons", |slot| {
                // Original bug pill / ReportBug(false) effect excluded (T13).
                let settings = slot.pill_button("settings", "menu/settings_icon");
                settings.node_mut().set_sounds(0, 3);
                if settings.node().used() {
                    emit(MenuEffect::Next(EscMenuState::Settings(
                        SettingsMenuState::Audio,
                    )));
                }
                let quit = slot.pill_button("quit", "menu/exit_icon");
                if quit.node().used() {
                    emit(MenuEffect::Next(EscMenuState::Quit));
                }
            });
            screen.with_slot("credits", |slot| {
                let credits = menu_button(slot, "Menu.Credits");
                credits
                    .content_mut::<MenuButtonContent>()
                    .text
                    .set_metrics(25.)
                    .expect("escape credits metrics");
                if credits.node().used() {
                    emit(MenuEffect::Next(EscMenuState::Credits));
                }
            });
        }
        EscMenuState::Quit => {
            let screen = mei
                .safe_frame("esc screen", true)
                .template(if config.dev.demo {
                    "menu/demo-exit"
                } else {
                    "menu/exit"
                });
            screen.with_slot("logo", |slot| top_logo(slot, config.dev.demo));
            if config.dev.demo {
                if show_discord {
                    screen.with_slot("discord", |slot| social_discord(slot, 50., overlay));
                }
                screen.with_slot("wishlist", |slot| wishlist_button(slot, 0.75, overlay));
            }
            screen.with_slot("buttons", |slot| {
                let back = menu_button(slot, "Menu.Back");
                back.node_mut().set_sounds(0, 9);
                back.content_mut::<MenuButtonContent>().action = 18;
                back.node_mut().default_focus();
                if back.node().used() {
                    emit(MenuEffect::Next(EscMenuState::Main));
                }
                if menu_button(slot, "Menu.Quit").node().used() {
                    emit(MenuEffect::Shutdown);
                }
            });
        }
        _ => {}
    }
}

fn menu_button(parent: NodeContext, key: &str) -> NodeContext {
    parent.menu_button(&lang_lib::lookup(key).to_uppercase())
}
fn top_logo(parent: NodeContext, demo: bool) {
    parent.image(
        "top logo",
        if demo {
            "menu/logo_w_branches_demo"
        } else {
            "menu/logo_w_branches"
        },
        300.,
    );
}
fn social_discord(parent: NodeContext, height: f32, overlay: &mut SteamOverlay) {
    let image = if parent.node().hover() {
        "menu/discord_logo"
    } else {
        "menu/discord_logo_2"
    };
    if parent.image("discord_img", image, height).node().used() {
        open_community_server(overlay);
    }
}
fn social_newsletter(parent: NodeContext, overlay: &mut SteamOverlay) {
    let image = if parent.node().hover() {
        "menu/newsletter"
    } else {
        "menu/newsletter_2"
    };
    if parent.image("newsletter_img", image, 84.).node().used() {
        open_newsletter(overlay);
    }
}

/// Native byte length of the uppercase localized label controls its metrics.
/// Keep the original order: image, localization, linearstep, lerp, text, bounds,
/// align, then used/open. The image is retained but isn't itself the URL button.
pub fn wishlist_button(parent: NodeContext, scale: f32, overlay: &mut SteamOverlay) {
    parent.image("wishlist sheep", "menu/feedback/aww", 60. * scale);
    let text = lang_lib::lookup("Menu.Wishlist").to_uppercase();
    let factor = ((text.len() as f32 - 10.) / (30. - 10.)).clamp(0., 1.);
    let metrics = utils::float_grid::lerp(40., 30., factor) * scale;
    let button = parent.menu_button(&text);
    let content = button.content_mut::<MenuButtonContent>();
    content
        .text
        .set_metrics(metrics)
        .expect("wishlist text metrics");
    content
        .text
        .set_max_width(350. * scale)
        .expect("wishlist width");
    content.text.set_align(tg_text::Align::Center);
    if button.node().used() {
        overlay.open_overlay_or_browser_to_webpage(STORE_URL);
    }
}
pub fn open_community_server(overlay: &mut SteamOverlay) {
    overlay.open_overlay_or_browser_to_webpage(COMMUNITY_SERVER_URL);
}
pub fn open_newsletter(overlay: &mut SteamOverlay) {
    overlay.open_overlay_or_browser_to_webpage(NEWSLETTER_URL);
}

/// B GUI39 wrapper. All event/resource identities are the real owner types.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn do_gui_escape_menu(
    actions: Res<MainActionSet>,
    (mode, loading, menu, photo): (
        Res<AppMode>,
        Res<State<AppLoadingState>>,
        Res<EscMenuState>,
        Res<PhotomodeScreen>,
    ),
    (config, profile): (Res<GameConfig>, Res<GameProfile>),
    mut next: ResMut<NextEscMenuState>,
    mut mei: ResMut<MeiContext>,
    mut history: EventWriter<HistoryCommand>,
    mut opened: EventWriter<EscapeMenuJustOpened>,
    mut save: EventWriter<SaveGameCmd>,
    mut sounds: EventWriter<PlayMenuUiSound>,
    mut overlay: ResMut<SteamOverlay>,
    mut save_profile: EventWriter<SaveGameProfileCmd>,
    mut commands: Commands,
    (mut request_mode, mut restore): (EventWriter<RequestAppModeChange>, ResMut<RestoreAppTo>),
    timelapse: Res<TimelapseState>,
    sessions: Res<SavedSessionList>,
) {
    build_escape_menu(
        &mut mei,
        *loading.get() == AppLoadingState::Game,
        actions.digital[27].just_pressed(),
        timelapse.is_active(),
        &menu,
        *mode,
        &photo,
        &config,
        &profile,
        sessions.sessions.len(),
        &mut overlay,
        |effect| match effect {
            MenuEffect::Next(state) => next.set(state),
            MenuEffect::History(command) => {
                history.write(command);
            }
            MenuEffect::RequestMode(mode) => {
                request_mode.write(RequestAppModeChange(mode));
            }
            MenuEffect::RestoreAfterReplay(mode) => restore.0 = mode,
            MenuEffect::Opened => {
                opened.write(EscapeMenuJustOpened);
            }
            MenuEffect::Sound(kind) => {
                sounds.write(PlayMenuUiSound { kind });
            }
            MenuEffect::SaveGame => {
                save.write(SaveGameCmd);
            }
            MenuEffect::SaveProfile => {
                save_profile.write(SaveGameProfileCmd);
            }
            MenuEffect::Shutdown => {
                commands.insert_resource(ShutdownGame);
            }
        },
    );
}
