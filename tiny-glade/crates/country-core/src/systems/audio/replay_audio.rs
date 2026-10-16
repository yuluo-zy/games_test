//! B `replay_audio.rs`: undo/redo/deny-input sounds.
//!
//! O-level facts (chapter replay_audio.rs.md, decomp-0x140ad5800):
//! - `replay_audio_init_system` 0x140ad54d0: 2 × `instantiate_audio`
//!   (`event:/UI/Undo`, `event:/UI/Redo`).
//! - `replay_audio_system` 0x140ad5800: `PlayReplayAudio` kind 0 → start the
//!   stored Undo instance; kind 1 → start the stored Redo instance; any other
//!   kind → instantiate + start `event:/UI/DenyInput`. AppMode == 0xd (Replay)
//!   additionally drives a global FMOD parameter (name 待取证).

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::mode_manager::AppMode,
};
use crate::systems::audio::PlayReplayAudio;
use bevy_ecs::prelude::*;

/// `event:/UI/DenyInput` (O: rne-read).
pub const DENY_INPUT_EVENT: &str = "event:/UI/DenyInput";

/// ReplayAudioState: the two stored undo/redo instances (O: init).
#[derive(Resource)]
pub struct ReplayAudioState<E: EventInstance> {
    pub undo: AudioInstance<E>,
    pub redo: AudioInstance<E>,
}

impl<E: EventInstance> ReplayAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        ReplayAudioState {
            undo: core.instantiate_audio("event:/UI/Undo"),
            redo: core.instantiate_audio("event:/UI/Redo"),
        }
    }
}

/// `replay_audio_init_system` (0x140ad54d0).
pub fn replay_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(ReplayAudioState::<S::Event>::new(&core));
}

/// `replay_audio_system` (0x140ad5800).
pub fn replay_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<ReplayAudioState<S::Event>>,
    mut events: EventReader<PlayReplayAudio>,
    core: Res<AudioCore<S>>,
    mode: Res<AppMode>,
) {
    for event in events.read() {
        match event.kind {
            0 => {
                let _ = state.undo.start();
            }
            1 => {
                let _ = state.redo.start();
            }
            _ => {
                // Deny-input one-shot.
                let mut deny = core.instantiate_audio(DENY_INPUT_EVENT);
                let _ = deny.start();
            }
        }
    }
    // O: AppMode == 0xd (Replay) sets a global FMOD parameter at frame end.
    // The parameter name is 待取证 (decomp shows AudioCoreInner::set_parameter
    // with a .data name that is not resolved).
    let _replaying = mode.native_tag() == 0xd;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deny_input_event_name_is_original() {
        assert_eq!(DENY_INPUT_EVENT, "event:/UI/DenyInput");
    }
}
