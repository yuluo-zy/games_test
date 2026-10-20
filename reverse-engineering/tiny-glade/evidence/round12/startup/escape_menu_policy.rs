// Pending recovered policy, not registered and not a completed GUI replacement.
// Source: B gui_escape_menu@1403cf9b0, native 1403cfbb2..1403d0222.
// A real caller MUST supply TimelapseState::is_active (byte != 2) and the
// original MEI button used() values. T13's digital31 feedback branch excluded.
use country_core::{resources::gui::EscMenuState,
    systems::mode_manager::{AppMode, HistoryCommand}};

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
}

/// The callback preserves native event order even when the input and two MEI
/// buttons are used together. The caller applies Next using the actual setter.
pub fn escape_menu_policy(
    game_is_active: bool, main_menu_just_pressed: bool, timelapse_active: bool,
    menu: &EscMenuState, mode: AppMode, open_button_used: bool,
    close_button_used: bool, mut emit: impl FnMut(MenuEffect),
) {
    let mut opening = open_button_used;
    let mut closing = close_button_used;
    if game_is_active && main_menu_just_pressed {
        if timelapse_active {
            emit(MenuEffect::History(HistoryCommand::INTERRUPT));
        } else if menu.is_closed() {
            opening = true;
        } else {
            closing = true;
        }
    }
    if opening {
        emit(MenuEffect::Next(EscMenuState::Main));
        if mode == AppMode::Replay {
            // Original1403cffd5 writes03000300, distinct from interrupt02000200.
            emit(MenuEffect::History(HistoryCommand { bytes: [0, 3, 0, 3] }));
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
}
