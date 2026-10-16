//! B `garden_audio.rs`: garden draw/erase sounds — 4 loops driven by the shared
//! `path_audio::update_draw_audio` template.
//!
//! O-level facts (chapter garden_audio.rs.md, decomp-0x140a88fa0 /
//! 0x140a89480, rne-read): init creates 4 loops — `event:/SFX/Garden/GardenDraw`,
//! `event:/SFX/Garden/GardenDrawWater`, `event:/SFX/Garden/GardenErase`,
//! `event:/SFX/Garden/GardenEraseWater`. `garden_audio_system` consumes
//! `GardenDrawEvent`/`GardenEraseEvent` (systems::garden producers) and drives
//! the draw / erase loop pairs via the shared template + raster sampling.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::{
        audio::path_audio::update_draw_audio,
        garden::{GardenDrawEvent, GardenEraseEvent},
    },
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;

/// The four garden loops (O: rne-read).
pub const GARDEN_LOOP_EVENTS: [&str; 4] = [
    "event:/SFX/Garden/GardenDraw",
    "event:/SFX/Garden/GardenDrawWater",
    "event:/SFX/Garden/GardenErase",
    "event:/SFX/Garden/GardenEraseWater",
];

/// GardenAudioState: the four loops.
#[derive(Resource)]
pub struct GardenAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 4],
}

impl<E: EventInstance> GardenAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        GardenAudioState {
            loops: GARDEN_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
        }
    }
}

/// `garden_audio_init_system` (0x140a88fa0).
pub fn garden_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(GardenAudioState::<S::Event>::new(&core));
}

/// `garden_audio_system` (0x140a89480): fold garden draw/erase events into the
/// loop should-play flags, then drive the draw / erase pairs at the brush
/// position (O: the native samples the garden raster and calls
/// `update_draw_audio<garden_closure>` four times).
pub fn garden_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<GardenAudioState<S::Event>>,
    mut draws: EventReader<GardenDrawEvent>,
    mut erases: EventReader<GardenEraseEvent>,
    _core: Res<AudioCore<S>>,
) {
    let draw_events: Vec<_> = draws.read().copied().collect();
    let erase_events: Vec<_> = erases.read().copied().collect();

    let drawing = !draw_events.is_empty();
    let erasing = !erase_events.is_empty();
    let draw_position = draw_events.last().map_or(glam::Vec3::ZERO, |e| e.position);
    let erase_position = erase_events.last().map_or(glam::Vec3::ZERO, |e| e.position);

    // O: draw loop pair follows the draw brush; erase pair the erase brush.
    update_draw_audio(&mut state.loops[0..2], drawing, draw_position);
    update_draw_audio(&mut state.loops[2..4], erasing, erase_position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garden_loop_names_are_original() {
        assert_eq!(GARDEN_LOOP_EVENTS[0], "event:/SFX/Garden/GardenDraw");
        assert_eq!(GARDEN_LOOP_EVENTS[1], "event:/SFX/Garden/GardenDrawWater");
        assert_eq!(GARDEN_LOOP_EVENTS[2], "event:/SFX/Garden/GardenErase");
        assert_eq!(GARDEN_LOOP_EVENTS[3], "event:/SFX/Garden/GardenEraseWater");
    }
}
