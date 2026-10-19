//! B `path_audio.rs`: path / cobblestone draw sounds — CobblestoneDraw loop +
//! 4 snappy loops, driven by the shared `update_draw_audio` template.
//!
//! Fresh evidence (decomp-0x1409d0f80, disasm-0x1409d1280..0x1409d15e0):
//! - 5 loops at state+0x00/+0x38/+0x70/+0xa8/+0xe0 (`event:/SFX/CobblestoneDraw`
//!   + the four snappy loops).
//! - three event streams are drained; the first stream's "has new events" flag
//!   gates the whole block.
//! - no events: loops at +0x70 (index 2), +0x38 (1), +0x00 (0), +0xe0 (4) are
//!   stopped, then the loop at +0xa8 (index 3) is driven.
//! - events: the loops at +0x70, +0x38, +0x00, +0xe0 are driven through
//!   `update_draw_audio` (the +0x00 call uses the path closure; the others the
//!   garden closure), then +0xa8 is driven (garden closure).
//!   The +0x70 predicate is `event != null && byte@+0x14 == 0 && raster < gate`;
//!   the +0x38 predicate is `event != null && (byte@+0x14 & 1) != 0 && raster < gate`.
//!   The remaining closure predicates (position/radius sources) live in the
//!   host-side closures and are registered as cross-domain wiring.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The five path loops (O: rne-read, state order).
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

/// `path_audio_init_system` (0x1409d08d0).
pub fn path_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(PathAudioState::<S::Event>::new(&core));
}

/// The shared `update_draw_audio` template: a loop is started and positioned
/// when `should_draw`, otherwise stopped.
fn update_draw_audio<E: EventInstance>(
    loop_player: &mut AudioLoop<AudioInstance<E>>,
    should_draw: bool,
    position: Vec3,
) {
    if should_draw {
        let _ = loop_player.start_if_not_playing();
        let current = &mut loop_player.entries[loop_player.cursor];
        let _ = current.set_position([position.x, position.y, position.z]);
    } else {
        let _ = loop_player.stop_now();
    }
}

/// `path_audio_system` (0x1409d0f80).
///
/// The native raster gate is `raster < threshold`; the host writes the
/// already-compared boolean into `PathAudioState::raster_gate` (0.0/1.0) so the
/// audio domain does not depend on the world-raster resource.
pub fn path_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<PathAudioState<S::Event>>,
    mut draws: EventReader<PathDrawEvent>,
    mut erases: EventReader<PathEraseEvent>,
    mut edits: EventReader<PathEditEvent>,
    _core: Res<AudioCore<S>>,
) {
    let draw: Vec<PathDrawEvent> = draws.read().copied().collect();
    let erase: Vec<PathEraseEvent> = erases.read().copied().collect();
    let edit: Vec<PathEditEvent> = edits.read().copied().collect();
    let raster_ok = state.raster_gate > 0.0;

    // O: the first stream's event presence gates the block (native
    // `in_stack_00000038` booleans).
    if draw.is_empty() {
        // O: no events → stop the four loops (order 2, 1, 0, 4), then drive 3.
        for slot in [2usize, 1, 0, 4] {
            let _ = state.loops[slot].stop_now();
        }
    } else {
        let draw_last = draw.last().copied();
        let erase_last = erase.last().copied();
        // +0x70: `event != null && byte@+0x14 == 0 && raster < gate`.
        let slot2 = draw_last.filter(|e| !e.observed_flag);
        update_draw_audio(
            &mut state.loops[2],
            raster_ok && slot2.is_some(),
            slot2.map_or(Vec3::ZERO, |e| e.position),
        );
        // +0x38: `event != null && (byte@+0x14 & 1) != 0 && raster < gate`.
        let slot1 = draw_last.filter(|e| e.observed_flag);
        update_draw_audio(
            &mut state.loops[1],
            raster_ok && slot1.is_some(),
            slot1.map_or(Vec3::ZERO, |e| e.position),
        );
        // +0x00: path closure (third stream).
        let slot0 = edit.last().copied();
        update_draw_audio(
            &mut state.loops[0],
            raster_ok && slot0.is_some(),
            slot0.map_or(Vec3::ZERO, |e| e.position),
        );
        // +0xe0: garden closure.
        let slot4 = erase_last;
        update_draw_audio(
            &mut state.loops[4],
            raster_ok && slot4.is_some(),
            slot4.map_or(Vec3::ZERO, |e| e.position),
        );
    }

    // O: +0xa8 is driven unconditionally after the if/else (garden closure).
    let slot3 = erase.last().copied();
    update_draw_audio(
        &mut state.loops[3],
        raster_ok && slot3.is_some(),
        slot3.map_or(Vec3::ZERO, |e| e.position),
    );
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
