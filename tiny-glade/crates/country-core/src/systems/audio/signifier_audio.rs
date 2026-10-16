//! B `signifier_audio.rs`: UI signifier stream sounds — 1 loop + 4 one-shots.
//!
//! O-level facts (chapter signifier_audio.rs.md, decomp-0x1409d6320 /
//! 0x1409d6b90, rne-read): init creates 1 loop + 4 × `instantiate_audio`
//! (`event:/UI/UI Boop`, `event:/Cursor/Grab`, `event:/Cursor/Release`,
//! `event:/UI/HoldTimeSiginfier` — original spelling — ,
//! `event:/UI/UiRotateGizmoMovingBack`). The system consumes `UiSignifierStream`
//! and drives FMOD parameters (FirstPersonOn/Off, Tool, and a further table —
//! the full 21-parameter mapping is 待取证).

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::ui::signifiers::UiSignifierStream,
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;

/// Signifier one-shots (O: rne-read).
pub const SIGNIFIER_EVENTS: [&str; 4] = [
    "event:/UI/UI Boop",
    "event:/Cursor/Grab",
    "event:/Cursor/Release",
    "event:/UI/UiRotateGizmoMovingBack",
];
/// The hold-time loop event (O string-table match; loop-vs-one-shot assignment
/// 待取证).
pub const SIGNIFIER_LOOP_EVENT: &str = "event:/UI/HoldTimeSiginfier";

/// SignifierAudioState.
#[derive(Resource)]
pub struct SignifierAudioState<E: EventInstance> {
    pub boop: AudioInstance<E>,
    pub cursor_grab: AudioInstance<E>,
    pub cursor_release: AudioInstance<E>,
    pub rotate_moving_back: AudioInstance<E>,
    pub hold_time: AudioLoop<AudioInstance<E>>,
}

impl<E: EventInstance> SignifierAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        let [boop, cursor_grab, cursor_release, rotate_moving_back] =
            SIGNIFIER_EVENTS.map(|name| core.instantiate_audio(name));
        SignifierAudioState {
            boop,
            cursor_grab,
            cursor_release,
            rotate_moving_back,
            hold_time: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(SIGNIFIER_LOOP_EVENT),
            ),
        }
    }
}

/// `signifier_audio_init_system` (0x1409d6320).
pub fn signifier_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(SignifierAudioState::<S::Event>::new(&core));
}

/// `signifier_audio_system` (0x1409d6b90): consume the UI signifier stream and
/// drive FMOD parameters (O: the native drives 21 parameters; FirstPersonOn/Off
/// and Tool are rne-read constants, the remainder 待取证).
pub fn signifier_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<SignifierAudioState<S::Event>>,
    mut stream: ResMut<UiSignifierStream>,
    _core: Res<AudioCore<S>>,
) {
    for record in stream.drain() {
        // O: each signifier record updates the parameter surface. The records'
        // tag → parameter mapping is 待取证; the per-instance parameter update
        // is the verified pattern.
        let _ = state.boop.set_parameter("Tool", f32::from(record.tag));
        let _ = state.cursor_grab.set_parameter("Tool", f32::from(record.tag));
        let _ = state
            .cursor_release
            .set_parameter("Tool", f32::from(record.tag));
        let _ = state
            .rotate_moving_back
            .set_parameter("Tool", f32::from(record.tag));
    }
    // O: frame-end hold-time loop toggle.
    let _ = state.hold_time.stop(std::time::Duration::ZERO);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signifier_event_names_are_original() {
        assert_eq!(SIGNIFIER_EVENTS[0], "event:/UI/UI Boop");
        assert_eq!(SIGNIFIER_EVENTS[1], "event:/Cursor/Grab");
        assert_eq!(SIGNIFIER_EVENTS[2], "event:/Cursor/Release");
        assert_eq!(SIGNIFIER_EVENTS[3], "event:/UI/UiRotateGizmoMovingBack");
        assert_eq!(SIGNIFIER_LOOP_EVENT, "event:/UI/HoldTimeSiginfier");
    }
}
