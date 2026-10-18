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
    systems::audio::roof_audio::instantiate_audio_w_position,
    systems::ui::signifiers::UiSignifierStream,
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;

/// Camera first-person one-shot events (O: decomp-0x1409d6b90 string assembly
/// `event:/UI/camera/FirstPersonOff|On` — lowercase "camera", record tag 0/1).
pub const FIRST_PERSON_OFF: &str = "event:/UI/camera/FirstPersonOff";
pub const FIRST_PERSON_ON: &str = "event:/UI/camera/FirstPersonOn";

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

/// The UI tool-change one-shot (O: decomp-0x1409d6b90 string assembly
/// `event:/UI/UI Tool Change`, length 0x18).
pub const UI_TOOL_CHANGE_EVENT: &str = "event:/UI/UI Tool Change";
/// Native global parameter name (O: 0x142ac2940, length 4); the 21-value
/// mapping is registered as a cross-domain gap.
pub const TOOL_PARAM: &str = "Tool";

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
/// drive FMOD one-shots / parameters.
///
/// O (decomp-0x1409d6b90): the native dispatches each signifier record to the
/// camera FirstPersonOff/On one-shots (`event:/UI/Camera/FirstPersonOff|On`,
/// selected by the record tag) and drives the full 21-parameter surface (the
/// remainder 待对账); the hold-time loop is stopped at frame end. The record
/// tag → parameter mapping is rebuilt at the surface where verified.
pub fn signifier_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<SignifierAudioState<S::Event>>,
    mut stream: ResMut<UiSignifierStream>,
    core: Res<AudioCore<S>>,
) {
    // O: native `event:/UI/Camera/FirstPersonOff|On` one-shots (record tag 0/1).
    for record in stream.drain() {
        let position = [record.position.x, record.position.y, record.position.z];
        match record.tag {
            0 => {
                let _ = instantiate_audio_w_position(&core, FIRST_PERSON_OFF, position);
            }
            1 => {
                let _ = instantiate_audio_w_position(&core, FIRST_PERSON_ON, position);
            }
            // O: the remaining signifier kinds drive the 21-parameter global
            // surface and the `event:/UI/UI Tool Change` one-shot. The native
            // record layout carries the tool/parameter values that require the
            // UI-signifier producer stream (cross-domain gap, registered); the
            // previous per-instance "Tool" writes were a self-approximation and
            // are dropped.
            _ => {}
        }
    }
    // O: frame-end hold-time loop toggle.
    let _ = state.hold_time.stop(std::time::Duration::ZERO);
    let _ = &state.boop;
    let _ = &state.cursor_grab;
    let _ = &state.cursor_release;
    let _ = &state.rotate_moving_back;
    let _ = &core;
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
        assert_eq!(FIRST_PERSON_OFF, "event:/UI/camera/FirstPersonOff");
        assert_eq!(FIRST_PERSON_ON, "event:/UI/camera/FirstPersonOn");
        assert_eq!(UI_TOOL_CHANGE_EVENT, "event:/UI/UI Tool Change");
    }
}
