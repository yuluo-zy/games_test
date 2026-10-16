//! B `roof_audio.rs`: roof edit sounds — 3 loops (RoofShape / RoofTipMove /
//! InterRoofMerlonsRearrange) plus the two one-shot helpers shared with the
//! wall audio systems (`instantiate_audio_w_position[_and_volume]`).
//!
//! O-level facts (chapter roof_audio.rs.md, audio-misc DIGEST §2.13):
//! - `roof_audio_init_system` 0x140aa2fd0: 3 × `instantiate_loop`:
//!   `event:/SFX/RoofShape`, `event:/SFX/RoofTipMove` (0x142adcb38),
//!   `event:/SFX/InterRoofMerlonsRearrange` (0x142adcb68).
//! - `instantiate_audio_w_position` 0x140aa3360 (4-arg `(core, name&str, pos)`):
//!   instantiate_audio (0x140a54440) → set_position (0x140a57da0) →
//!   start_if_not_playing (0x140a57300).
//! - `instantiate_audio_w_position_and_volume` 0x140aa3590: additionally
//!   `EventInstance::set_volume` when the instance is Active.
//! - `roof_audio_system` 0x140aa3860: drives the 3 loops' should-play /
//!   position / parameters plus one-shots per roof edit event.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, Studio},
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;

/// The three roof edit loops (O: rne-read).
pub const ROOF_LOOP_EVENTS: [&str; 3] = [
    "event:/SFX/RoofShape",
    "event:/SFX/RoofTipMove",
    "event:/SFX/InterRoofMerlonsRearrange",
];

/// `instantiate_audio_w_position` (0x140aa3360): instantiate → set_position →
/// start_if_not_playing.
pub fn instantiate_audio_w_position<S: Studio>(
    core: &AudioCore<S>,
    name: &str,
    position: [f32; 3],
) -> AudioInstance<S::Event> {
    let mut instance = core.instantiate_audio(name);
    let _ = instance.set_position(position);
    let _ = instance.start_if_not_playing();
    instance
}

/// `instantiate_audio_w_position_and_volume` (0x140aa3590): instantiate →
/// set_position → start_if_not_playing → set_volume (Active only). Used by the
/// wall one-shot kinds and elevation supports.
pub fn instantiate_audio_w_position_and_volume<S: Studio>(
    core: &AudioCore<S>,
    name: &str,
    position: [f32; 3],
    volume: f32,
) -> AudioInstance<S::Event> {
    let mut instance = core.instantiate_audio(name);
    let _ = instance.set_position(position);
    let _ = instance.start_if_not_playing();
    let _ = instance.set_volume(volume);
    instance
}

/// RoofAudioState (O: 3 ring loops + per-loop position/parameter state).
#[derive(Resource)]
pub struct RoofAudioState<E: crate::resources::audio_resources::EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 3],
    /// Accumulated edit-strength per loop (native `+0xa8/+0xac` speed sources).
    pub acc: [f32; 2],
}

impl<E: crate::resources::audio_resources::EventInstance> RoofAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        let loops = ROOF_LOOP_EVENTS.map(|name| {
            core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
        });
        RoofAudioState { loops, acc: [0.0, 0.0] }
    }
}

/// `roof_audio_init_system` (0x140aa2fd0).
pub fn roof_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    let state = RoofAudioState::<S::Event>::new(&core);
    commands.insert_resource(state);
}

/// `roof_audio_system` (0x140aa3860): per-frame loop driving — each of the 3
/// loops is started (and positioned) when its should-play flag is set, else
/// stopped. Roof edit one-shots are dispatched through
/// [`instantiate_audio_w_position`]. The exact edit-event type is 待取证
/// (native reads a 0x28-stride event stream; the roof producer domain is the
/// render/host).
pub fn roof_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<RoofAudioState<S::Event>>,
    _core: Res<AudioCore<S>>,
) {
    for loop_player in state.loops.iter_mut() {
        // should-play is set by the (待接线) edit-event consumer; frame-end
        // start_if_not_playing / stop mirrors the native loop driver.
        let play = loop_player.started;
        if play {
            let _ = loop_player.start_if_not_playing();
        } else {
            let _ = loop_player.stop(std::time::Duration::ZERO);
        }
    }
    // The native also consumes a roof edit event stream and one-shots; that
    // consumer wiring is 待接线 (roof producer domain).
}
