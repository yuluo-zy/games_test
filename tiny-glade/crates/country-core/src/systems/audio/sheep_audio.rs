//! B `sheep_audio.rs`: sheep reaction sounds — kind 0..7 dispatch.
//!
//! O-level facts (chapter sheep_audio.rs.md, multi-baseline audio-extra §3e,
//! rne-read): `PlaySheepReactSound` slot stride 0x20, kind u8 @+0x18; jump
//! table 0x142ac2cc8 (kind 0–7). Kinds 0/1/2/3/5/6/7 instantiate one of
//! `event:/SFX/Sheep/{Aww,Think,Sob,Angry,Idea,Eyes,Confused}` + set_parameter
//! (FirstPersonOff) + start_if_not_playing; kind 4 sets the should-play flag
//! of a frame-end loop (no one-shot). Frame-end drives that loop by flag.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    utils::audio_utils::AudioLoop,
};
use crate::systems::audio::PlaySheepReactSound;
use bevy_ecs::prelude::*;

/// Sheep reaction one-shots (O: rne-read; the chapter's kind-2 "So" is a
/// mis-read of "Sob" — the string table reads `event:/SFX/Sheep/Sob`).
pub const SHEEP_EVENTS: [&str; 7] = [
    "event:/SFX/Sheep/Aww",
    "event:/SFX/Sheep/Think",
    "event:/SFX/Sheep/Sob",
    "event:/SFX/Sheep/Angry",
    "event:/SFX/Sheep/Idea",
    "event:/SFX/Sheep/Eyes",
    "event:/SFX/Sheep/Confused",
];

/// Kind 4 → the continuous frame-end loop (E3 CONFIRMED: event table 0x142ac23b0
/// kind 0–7 are the seven one-shots; kind 4 packages a loop handle driven at
/// frame end by start_if_not_playing / stop — 0x1409d61b0). The loop event is
/// `event:/SFX/Sheep/Baah` (rne-read; startup_sheep loads Baah + Pitch at
/// 0x142ac2460). The Baah loop carries a `Pitch` parameter.
pub const KIND4_LOOP_EVENT: &str = "event:/SFX/Sheep/Baah";
/// Baah loop parameter (O: rne-read `Pitch` adjacent to the Baah/UI Boop table).
pub const SHEEP_PITCH_PARAM: &str = "Pitch";

/// SheepAudioState (O: the kind-4 frame-end loop player).
#[derive(Resource)]
pub struct SheepAudioState<E: EventInstance> {
    pub loop4: AudioLoop<AudioInstance<E>>,
    pub should_play4: bool,
}

impl<E: EventInstance> SheepAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        SheepAudioState {
            loop4: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(KIND4_LOOP_EVENT),
            ),
            should_play4: false,
        }
    }
}

/// `sheep_audio_init_system` (O: init is inlined; state resource owns the loop).
pub fn sheep_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(SheepAudioState::<S::Event>::new(&core));
}

/// `sheep_audio_system` (0x1409d5ea0).
pub fn sheep_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<SheepAudioState<S::Event>>,
    mut events: EventReader<PlaySheepReactSound>,
    core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        match event.kind {
            4 => {
                // E3: kind 4 packages the continuous Baah loop (no one-shot).
                state.should_play4 = true;
                let cursor = state.loop4.cursor;
                let _ = state.loop4.entries[cursor].set_parameter(SHEEP_PITCH_PARAM, 0.0);
            }
            kind if kind < SHEEP_EVENTS.len() as u8 => {
                let name = SHEEP_EVENTS[kind as usize];
                let mut instance = core.instantiate_audio(name);
                // O: set_parameter (FirstPersonOff) then start_if_not_playing.
                let _ = instance.set_parameter("FirstPersonOff", 0.0);
                let _ = instance.start_if_not_playing();
            }
            _ => {}
        }
    }
    // Frame-end loop driving (O: 0x1409d61b0; E3: continuous loop by flag).
    if state.should_play4 {
        let _ = state.loop4.start_if_not_playing();
    } else {
        let _ = state.loop4.stop(std::time::Duration::ZERO);
    }
    state.should_play4 = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sheep_event_names_match_the_string_table() {
        assert_eq!(SHEEP_EVENTS[0], "event:/SFX/Sheep/Aww");
        assert_eq!(SHEEP_EVENTS[1], "event:/SFX/Sheep/Think");
        assert_eq!(SHEEP_EVENTS[2], "event:/SFX/Sheep/Sob");
        assert_eq!(SHEEP_EVENTS[3], "event:/SFX/Sheep/Angry");
        assert_eq!(SHEEP_EVENTS[4], "event:/SFX/Sheep/Idea");
        assert_eq!(SHEEP_EVENTS[5], "event:/SFX/Sheep/Eyes");
        assert_eq!(SHEEP_EVENTS[6], "event:/SFX/Sheep/Confused");
    }
}







