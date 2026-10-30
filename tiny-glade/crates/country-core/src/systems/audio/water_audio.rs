//! B `water_audio.rs`: water draw / erase / ice sounds — 4 loops.
//!
//! O-level facts (chapter water_audio.rs.md, decomp-0x140873140 /
//! 0x140873630, rne-read): init creates 4 loops (gated on the non-demo build)
//! — `event:/SFX/Water/WaterDraw`, `event:/SFX/Water/WaterErase`,
//! `event:/SFX/Water/IceDraw`, `event:/SFX/Water/IceErase`. The system
//! signature (procs.tsv) is
//! `(Res<AppMode>, ResMut<WaterAudioState>, EventReader<WaterDrawEvent>,
//!  EventReader<WaterEraseEvent>, Res<CursorTerrainRaycast>, Res<ActiveSession>)`.

use crate::{
    resources::{
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
        cursor_terrain_raycast::CursorTerrainRaycast,
        session::ActiveSession,
    },
    systems::{
        audio::garden_audio::update_draw_audio_volume,
        backend::input::translate_input::DemoModeEnabled,
        mode_manager::AppMode,
    },
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The four water loops (O: rne-read).
pub const WATER_LOOP_EVENTS: [&str; 4] = [
    "event:/SFX/Water/WaterDraw",
    "event:/SFX/Water/WaterErase",
    "event:/SFX/Water/IceDraw",
    "event:/SFX/Water/IceErase",
];

// Water draw/erase events live in `resources::water` (native module; wired by
// the water-domain restorer). Fields {position, radius} mirror the garden draw
// shape; the ice-loop gating by water-raster sampling stays in the water domain.
pub use crate::resources::water::{WaterDrawEvent, WaterEraseEvent};

/// WaterAudioState: the four loops.
#[derive(Resource)]
pub struct WaterAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 4],
}

impl<E: EventInstance> WaterAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        WaterAudioState {
            loops: WATER_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
        }
    }
}

/// `water_audio_init_system` (0x140873140): creates the four loops unless the
/// build is demo mode (O: `_DEV_OVERRIDE_IS_DEMO_MODE == 0` gate).
pub fn water_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
    demo: Option<Res<DemoModeEnabled>>,
) {
    // O (decomp-0x140873140): `if (_DEV_OVERRIDE_IS_DEMO_MODE == 0)` before the
    // four instantiate_loop calls — demo builds insert no water state.
    if demo.is_some_and(|d| d.0) {
        return;
    }
    let state = WaterAudioState::<S::Event>::new(&core);
    commands.insert_resource(state);
}

/// `water_audio_system` (0x140873630): winter routes the two readers to the
/// ice pair and stops the water pair; other seasons do the reverse. The volume is the garden editing-speed
/// source (Replay → 1.0, else `CursorTerrainRaycast.speed / 60.0`).
#[allow(clippy::too_many_arguments)]
pub fn water_audio_system<S: Studio + Send + Sync + 'static>(
    mode: Res<AppMode>,
    mut state: ResMut<WaterAudioState<S::Event>>,
    mut draws: EventReader<WaterDrawEvent>,
    mut erases: EventReader<WaterEraseEvent>,
    raycast: Res<CursorTerrainRaycast>,
    session: Res<ActiveSession>,
) {
    let volume = if mode.native_tag() == 0xd {
        crate::systems::audio::garden_audio::REPLAY_VOLUME
    } else {
        raycast.speed / crate::systems::audio::garden_audio::VOLUME_NORMALIZER
    };
    let draw_last = draws.read().last().copied();
    let erase_last = erases.read().last().copied();
    let (draw_pos, draw_radius) =
        draw_last.map_or((Vec3::ZERO, 0.0), |e| (e.position, e.radius));
    let (erase_pos, erase_radius) =
        erase_last.map_or((Vec3::ZERO, 0.0), |e| (e.position, e.radius));

    let (draw_loop, erase_loop, stop_first, stop_second) = water_loop_routing(session.is_winter());
    update_draw_audio_volume(&mut state.loops[draw_loop], draw_last.is_some(), draw_pos, draw_radius, volume);
    update_draw_audio_volume(&mut state.loops[erase_loop], erase_last.is_some(), erase_pos, erase_radius, volume);
    let _ = state.loops[stop_first].stop_now();
    let _ = state.loops[stop_second].stop_now();
}

/// 1408738f5..c4d: reader 3 feeds offsets 0/0x70 and reader 4 feeds
/// 0x38/0xa8; the inactive pair is stopped after both template calls.
fn water_loop_routing(winter: bool) -> (usize, usize, usize, usize) {
    if winter { (2, 3, 0, 1) } else { (0, 1, 2, 3) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_loop_names_are_original() {
        assert_eq!(WATER_LOOP_EVENTS[0], "event:/SFX/Water/WaterDraw");
        assert_eq!(WATER_LOOP_EVENTS[1], "event:/SFX/Water/WaterErase");
        assert_eq!(WATER_LOOP_EVENTS[2], "event:/SFX/Water/IceDraw");
        assert_eq!(WATER_LOOP_EVENTS[3], "event:/SFX/Water/IceErase");
    }

    #[test]
    fn seasonal_event_routing_matches_native_loop_offsets() {
        use crate::resources::session::GladeType;
        for glade in GladeType::ALL {
            let route = water_loop_routing(glade.is_winter());
            assert_eq!(route, if glade == GladeType::Winter { (2, 3, 0, 1) } else { (0, 1, 2, 3) });
        }
        assert_eq!(WATER_LOOP_EVENTS[water_loop_routing(true).0], "event:/SFX/Water/IceDraw");
        assert_eq!(WATER_LOOP_EVENTS[water_loop_routing(true).1], "event:/SFX/Water/IceErase");
    }
}
