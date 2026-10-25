use crate::resources::audio_resources::{AudioCore,AudioInstance,EventInstance,Studio};
use crate::utils::audio_utils::AudioLoop;
use bevy_ecs::prelude::*;
use glam::Vec3;
pub const DECORATOR_LOOP_EVENTS: [&str; 16] = [
    "event:/SFX/WindowDrag",
    "event:/SFX/DecoratorMoveWoodenFloorTEMP",
    "event:/SFX/Lantern/MoveWall",
    "event:/SFX/Lantern/MoveGround",
    "event:/SFX/Lantern/Rotate",
    "event:/SFX/Lantern/Amb",
    "event:/SFX/Lantern/Amb",
    "event:/SFX/Lantern/Amb",
    "event:/SFX/ChimneyChangeHeight",
    "event:/SFX/ChimneyDrag",
    "event:/SFX/Flags/Rotate",
    "event:/SFX/Flags/Drag",
    "event:/SFX/Flags/DragGround",
    "event:/SFX/Cosmetics/Clutter_Rotate",
    "event:/SFX/Cosmetics/MoveGround",
    "event:/SFX/Cosmetics/MoveBuilding",
];

/// DecoratorAudioState: the 16 loops + per-loop should-play flag.
#[derive(Resource)]
pub struct DecoratorAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 16],
    pub should_play: [bool; 16],
    pub positions: [Vec3; 16],
}

impl<E: EventInstance> DecoratorAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        DecoratorAudioState {
            loops: DECORATOR_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
            should_play: [false; 16],
            positions: [Vec3::ZERO; 16],
        }
    }
}

