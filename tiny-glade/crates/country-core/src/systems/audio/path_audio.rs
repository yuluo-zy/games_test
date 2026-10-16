//! B `path_audio.rs`: path / cobblestone draw sounds — CobblestoneDraw loop +
//! 4 snappy loops, driven by the shared `update_draw_audio` template (also used
//! by garden_audio).
//!
//! O-level facts (chapter path_audio.rs.md, decomp-0x1409d08d0 / 0x1409d0f80,
//! rne-read): init creates 1 loop (`event:/SFX/CobblestoneDraw`) + 4 snappy
//! loops — `event:/SFX/Path Draw`, `event:/SFX/Erase Draw`,
//! `event:/SFX/Water/PathWaterDraw`, `event:/SFX/Water/PathWaterErase`.
//! `update_draw_audio` (0x14166d890/0x14166d9f0) is the shared template that
//! drives a loop set by a raster-sample "should draw" flag + position.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The five path loops (O: rne-read).
pub const PATH_LOOP_EVENTS: [&str; 5] = [
    "event:/SFX/CobblestoneDraw",
    "event:/SFX/Path Draw",
    "event:/SFX/Erase Draw",
    "event:/SFX/Water/PathWaterDraw",
    "event:/SFX/Water/PathWaterErase",
];

/// Path draw/erase events (native producers live in the `systems/paths`
/// domain 待接线; the fields mirror `GardenDrawEvent`'s shape).
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

/// PathAudioState: the five loops.
#[derive(Resource)]
pub struct PathAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 5],
}

impl<E: EventInstance> PathAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        PathAudioState {
            loops: PATH_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
        }
    }
}

/// `path_audio_init_system` (0x1409d08d0).
pub fn path_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(PathAudioState::<S::Event>::new(&core));
}

/// `update_draw_audio` (shared template, 0x14166d890/0x14166d9f0): drive each
/// loop in `loops[range]` by the raster-sample "should draw" flag and position.
pub fn update_draw_audio<E: EventInstance>(
    loops: &mut [AudioLoop<AudioInstance<E>>],
    should_draw: bool,
    position: Vec3,
) {
    for loop_player in loops.iter_mut() {
        if should_draw {
            let _ = loop_player.start_if_not_playing();
            let current = &mut loop_player.entries[loop_player.cursor];
            let _ = current.set_position([position.x, position.y, position.z]);
        } else {
            let _ = loop_player.stop(std::time::Duration::ZERO);
        }
    }
}

/// `path_audio_system` (0x1409d0f80): consume path draw/erase events and drive
/// the draw / erase loop pairs (O: two event readers + raster sampling).
pub fn path_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<PathAudioState<S::Event>>,
    mut draws: EventReader<PathDrawEvent>,
    mut erases: EventReader<PathEraseEvent>,
    _core: Res<AudioCore<S>>,
) {
    let drawing = draws.read().next().is_some();
    let erasing = erases.read().next().is_some();
    let draw_position = draws.read().last().map_or(Vec3::ZERO, |e| e.position);
    let erase_position = erases.read().last().map_or(Vec3::ZERO, |e| e.position);

    // O: CobblestoneDraw + Path Draw loops follow the draw brush.
    update_draw_audio(&mut state.loops[0..2], drawing, draw_position);
    // O: Erase Draw + PathWaterErase loops follow the erase brush.
    update_draw_audio(&mut state.loops[2..3], erasing, erase_position);
    update_draw_audio(&mut state.loops[3..5], drawing, draw_position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_loop_names_are_original() {
        assert_eq!(PATH_LOOP_EVENTS[0], "event:/SFX/CobblestoneDraw");
        assert_eq!(PATH_LOOP_EVENTS[1], "event:/SFX/Path Draw");
        assert_eq!(PATH_LOOP_EVENTS[2], "event:/SFX/Erase Draw");
        assert_eq!(PATH_LOOP_EVENTS[3], "event:/SFX/Water/PathWaterDraw");
        assert_eq!(PATH_LOOP_EVENTS[4], "event:/SFX/Water/PathWaterErase");
    }
}
