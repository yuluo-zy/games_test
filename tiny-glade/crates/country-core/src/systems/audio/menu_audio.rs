//! B `menu_audio.rs`: pause-menu snapshot + 13 menu UI one-shots.
//!
//! O-level facts (chapter menu_audio.rs.md, multi-baseline audio-extra §3d,
//! rne-read): init instantiate `snapshot:/PauseMenu` (0x142ac6db8);
//! `menu_audio` gates the snapshot by `(AppMode|8)==0xa` (stop) vs
//! start_if_not_playing, and dispatches `PlayMenuUiSound` kind 0..12
//! (slot stride 0x10, kind @+0x8) through instantiate_audio +
//! start_if_not_playing. Event names rne-read (linear kind→name order).

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    resources::gui::EscMenuState,
};
use crate::systems::audio::PlayMenuUiSound;
use bevy_ecs::prelude::*;

/// `snapshot:/PauseMenu` (O).
pub const PAUSE_MENU_SNAPSHOT: &str = "snapshot:/PauseMenu";

/// kind → `event:/UI/...` (O: rne-read; linear order 0..12).
pub const MENU_EVENTS: [&str; 13] = [
    "event:/UI/Menu/Hover",            // 0
    "event:/UI/Ui2dHover",             // 1
    "event:/UI/Menu/Select",           // 2
    "event:/UI/Menu/Settings",         // 3
    "event:/UI/Menu/SettingsTab",      // 4
    "event:/UI/Menu/SliderTick",       // 5
    "event:/UI/Menu/ShowSessions",     // 6
    "event:/UI/Menu/NewSession",       // 7
    "event:/UI/Menu/StartSession",     // 8
    "event:/UI/Menu/Back",             // 9
    "event:/UI/Menu/Open",             // 10
    "event:/UI/Menu/Close",            // 11
    "event:/UI/Ui2dLockedClickNew",    // 12
];

/// MenuAudioState: the pause-menu snapshot instance (O: init).
#[derive(Resource)]
pub struct MenuAudioState<E: EventInstance> {
    pub snapshot: AudioInstance<E>,
}

impl<E: EventInstance> MenuAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        MenuAudioState {
            snapshot: core.instantiate_audio(PAUSE_MENU_SNAPSHOT),
        }
    }
}

/// `menu_audio_init_system` (0x140a0b340).
pub fn menu_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(MenuAudioState::<S::Event>::new(&core));
}

/// `menu_audio` (0x140a0b4c0).
pub fn menu_audio<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<MenuAudioState<S::Event>>,
    mut events: EventReader<PlayMenuUiSound>,
    core: Res<AudioCore<S>>,
    menu: Res<EscMenuState>,
) {
    // PDB run_unsafe@140b27720 identifies EscMenuState, not AppMode.
    // `(tag|8)==0xa` → stop the pause-menu snapshot, else
    // start_if_not_playing.
    if menu.view().tag | 8 == 0xa {
        let _ = state.snapshot.stop();
    } else {
        let _ = state.snapshot.start_if_not_playing();
    }
    for event in events.read() {
        let kind = usize::from(event.kind);
        if kind >= MENU_EVENTS.len() {
            continue; // O: kind ≥ 13 out of bounds → skip.
        }
        let mut instance = core.instantiate_audio(MENU_EVENTS[kind]);
        let _ = instance.start_if_not_playing();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_kind_names_are_in_string_table_order() {
        assert_eq!(MENU_EVENTS[0], "event:/UI/Menu/Hover");
        assert_eq!(MENU_EVENTS[4], "event:/UI/Menu/SettingsTab");
        assert_eq!(MENU_EVENTS[12], "event:/UI/Ui2dLockedClickNew");
    }
}
