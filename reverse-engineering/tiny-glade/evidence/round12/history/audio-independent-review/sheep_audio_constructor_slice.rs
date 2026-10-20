use crate::resources::audio_resources::{AudioCore,AudioInstance,EventInstance,Studio};
use crate::utils::audio_utils::AudioLoop;
use bevy_ecs::prelude::*;
use glam::Vec3;
pub const SHEEP_EVENTS: [&str; 7] = [
    "event:/SFX/Sheep/Aww",
    "event:/SFX/Sheep/Think",
    "event:/SFX/Sheep/Sob",
    "event:/SFX/Sheep/Angry",
    "event:/SFX/Sheep/Idea",
    "event:/SFX/Sheep/Eyes",
    "event:/SFX/Sheep/Confused",
];

/// Original inlined SheepAudioState constructors140b6a92c/1414d9b82 use
/// event name142ac2338 (20 bytes), separate from startup sheep's Baah event.
pub const KIND4_LOOP_EVENT: &str = "event:/SFX/Sheep/Pet";
/// Sheep reaction pitch parameter.
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

