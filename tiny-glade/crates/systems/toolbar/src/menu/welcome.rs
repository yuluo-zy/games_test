//! Original welcome141ebe130/2158B and both native button slots.
//! Canonical profile/state owners are preserved. Registration belongs to the
//! gameplay host's existing Game/set36; this does not open a synthetic menu.
use crate::mei::{
    core::{MeiContext, NodeContext},
    widgets::MenuButtonContent,
};
use bevy_ecs::prelude::*;
use country_core::{
    game_config::GameConfig,
    game_profile::{GameProfile, SaveGameProfileCmd},
    resources::{gui::EscMenuState, onboarding::WelcomeScreenState, time::GameTime},
    systems::{render_loop::vermeer::VermeerState, steam::SteamClientResource},
};

/// Original Local u64: page0 is the first SteamDeck message, page1 the second.
#[derive(Default)]
pub struct SteamdeckWelcomeMsg(pub u64);

/// SSE MAXSS(0,t), MINSS(1,t), followed by the original separate arithmetic.
/// NaNs and the source signed zero are retained by the min/max instructions.
fn welcome_opacity(elapsed: f32) -> f32 {
    let t = elapsed / f32::from_bits(0x3e99999a);
    let t = if t.is_nan() || t >= 0. { t } else { 0. };
    let t = if t.is_nan() || t <= 1. { t } else { 1. };
    let square = t * t;
    let twice = t + t;
    (3. - twice) * square
}

fn confirm_button(slot: NodeContext, text: &str, metrics: f32) -> bool {
    let button = slot.menu_button(text).menu_hotkey(19);
    button
        .content_mut::<MenuButtonContent>()
        .text
        .set_metrics(metrics * 1.5)
        .expect("original welcome button metrics");
    button.node().used()
}

/// Ten original SystemParams, independently identified by typed run_if caller.
pub fn ui(
    mut mei: ResMut<MeiContext>,
    mut welcome: ResMut<WelcomeScreenState>,
    vermeer: Res<VermeerState>,
    time: Res<GameTime>,
    menu: Res<EscMenuState>,
    config: Res<GameConfig>,
    mut profile: ResMut<GameProfile>,
    mut save_profile: EventWriter<SaveGameProfileCmd>,
    steam: Option<Res<SteamClientResource>>,
    mut deck_message: Local<SteamdeckWelcomeMsg>,
) {
    if config.dev.skip_welcome || profile.welcome_was_dismissed() {
        *welcome = WelcomeScreenState::Closed;
        return;
    }
    let on_deck = steam
        .as_ref()
        .and_then(|steam| steam.client.as_ref())
        .is_some_and(|client| client.is_steam_running_on_steam_deck());

    // The native initial-state branch does not draw on the same call.
    match *welcome {
        WelcomeScreenState::Pending => {
            if menu.is_closed() {
                *welcome = WelcomeScreenState::Showing { elapsed: 0. };
            }
            return;
        }
        WelcomeScreenState::Closed => return,
        WelcomeScreenState::Showing { .. } => {}
    }
    if vermeer.is_active() {
        *welcome = WelcomeScreenState::Showing { elapsed: 0. };
        return;
    }
    if !menu.is_closed() {
        profile.dismiss_welcome();
        save_profile.write(SaveGameProfileCmd);
        *welcome = WelcomeScreenState::Closed;
        return;
    }

    let WelcomeScreenState::Showing { elapsed } = &mut *welcome else {
        unreachable!()
    };
    let frame = mei.safe_frame("welcome", true);
    let used = if on_deck {
        assert!(
            deck_message.0 <= 1,
            "original SteamdeckWelcomeMsg has only two pages"
        );
        let screen = frame.template(if deck_message.0 == 0 {
            "steamdeck/welcome_1"
        } else {
            "steamdeck/welcome_2"
        });
        screen.context.node_mut().local_opacity = if deck_message.0 == 0 {
            *elapsed += time.delta;
            welcome_opacity(*elapsed)
        } else {
            1.
        };
        let last_page = deck_message.0 == 1;
        let text = if last_page {
            format!("{} 🐑", lang_lib::lookup("Welcome.LetsGo"))
        } else {
            "➡️".to_owned()
        };
        let clicked = screen.with_slot("buttons", |slot| confirm_button(slot, &text, 27.));
        if clicked && !last_page {
            deck_message.0 += 1;
            false
        } else {
            clicked
        }
    } else {
        let screen = frame.template(if config.dev.demo {
            "menu/welcome_demo"
        } else {
            "menu/welcome"
        });
        *elapsed += time.delta;
        screen.context.node_mut().local_opacity = welcome_opacity(*elapsed);
        let text = format!("{} 🐑", lang_lib::lookup("Welcome.LetsGo"));
        screen.with_slot("buttons", |slot| confirm_button(slot, &text, 18.))
    };
    if used {
        profile.dismiss_welcome();
        save_profile.write(SaveGameProfileCmd);
        *welcome = WelcomeScreenState::Closed;
    }
}

#[cfg(test)]
mod tests;
