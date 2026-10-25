use crate::resources::audio_resources::{AudioCore,AudioInstance,EventInstance,Studio};
use crate::utils::audio_utils::AudioLoop;
use bevy_ecs::prelude::*;
use glam::Vec3;
pub const PATH_LOOP_EVENTS: [&str; 5] = [
    "event:/SFX/CobblestoneDraw",
    "event:/SFX/Path Draw",
    "event:/SFX/Erase Draw",
    "event:/SFX/Water/PathWaterDraw",
    "event:/SFX/Water/PathWaterErase",
];

/// Path draw / erase events (native producers live in the `systems/paths`
/// domain 待接线; the fields mirror `GardenDrawEvent`'s shape). The native slot
/// stride is 0x20 with the observed flag byte at +0x14.
#[derive(Event, Clone, Copy, Debug)]
pub struct PathDrawEvent {
    pub position: Vec3,
    pub observed_flag: bool,
}
#[derive(Event, Clone, Copy, Debug)]
pub struct PathEraseEvent {
    pub position: Vec3,
    pub observed_flag: bool,
}
/// Third path stream (native `closure_env_5` reads it; the producer lives in
/// the paths domain 待接线).
#[derive(Event, Clone, Copy, Debug)]
pub struct PathEditEvent {
    pub position: Vec3,
    pub observed_flag: bool,
}

/// PathAudioState: the five loops.
#[derive(Resource)]
pub struct PathAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 5],
    /// Raster gate value sampled by the host each frame (native
    /// `bilinear_sample_at_world_xz` result; the gate threshold is registered
    /// with the raster domain).
    pub raster_gate: f32,
}

impl<E: EventInstance> PathAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        PathAudioState {
            loops: std::array::from_fn(|index| {
                // Native1409d0987/a1f/ab7/b64/c05 set each returned ring+36.
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(PATH_LOOP_EVENTS[index])).noloop()
            }),
            raster_gate: 0.0,
        }
    }
}

