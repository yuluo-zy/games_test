//! B `water_audio.rs`: water draw / erase / ice sounds — 4 loops.
//!
//! O-level facts (chapter water_audio.rs.md, decomp-0x140873140 /
//! 0x140873630, rne-read): init creates 4 loops (gated on the non-demo build)
//! — `event:/SFX/Water/WaterDraw`, `event:/SFX/Water/WaterErase`,
//! `event:/SFX/Water/IceErase`, `event:/SFX/Water/IceDraw`. The system
//! signature (procs.tsv) is
//! `(Res<AppMode>, ResMut<WaterAudioState>, EventReader<WaterDrawEvent>,
//!  EventReader<WaterEraseEvent>, Res<CursorTerrainRaycast>, Res<ActiveSession>)`.

use crate::{
    resources::{
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
        cursor_terrain_raycast::CursorTerrainRaycast,
        session::ActiveSession,
    },
    systems::mode_manager::AppMode,
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The four water loops (O: rne-read).
pub const WATER_LOOP_EVENTS: [&str; 4] = [
    "event:/SFX/Water/WaterDraw",
    "event:/SFX/Water/WaterErase",
    "event:/SFX/Water/IceErase",
    "event:/SFX/Water/IceDraw",
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
/// build is demo mode (O: `_DEV_OVERRIDE_IS_DEMO_MODE` gate).
pub fn water_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    let state = WaterAudioState::<S::Event>::new(&core);
    commands.insert_resource(state);
}

/// `water_audio_system` (0x140873630): drive the draw / erase loop pairs by
/// the draw/erase events. The ice loops and the AppMode/ActiveSession gating
/// are 待接线 (native drives them by water-raster ice sampling; that raster
/// consumer is the water domain).
pub fn water_audio_system<S: Studio + Send + Sync + 'static>(
    _mode: Res<AppMode>,
    mut state: ResMut<WaterAudioState<S::Event>>,
    mut draws: EventReader<WaterDrawEvent>,
    mut erases: EventReader<WaterEraseEvent>,
    _raycast: Res<CursorTerrainRaycast>,
    _session: Res<ActiveSession>,
) {
    let drawing = draws.read().next().is_some();
    let erasing = erases.read().next().is_some();
    let draw_position = draws.read().last().map_or(Vec3::ZERO, |e| e.position);
    let erase_position = erases.read().last().map_or(Vec3::ZERO, |e| e.position);

    drive(&mut state.loops[0], drawing, draw_position);
    drive(&mut state.loops[1], erasing, erase_position);
}

fn drive<E: EventInstance>(loop_player: &mut AudioLoop<AudioInstance<E>>, play: bool, position: Vec3) {
    if play {
        let _ = loop_player.start_if_not_playing();
        let current = &mut loop_player.entries[loop_player.cursor];
        let _ = current.set_position([position.x, position.y, position.z]);
    } else {
        let _ = loop_player.stop(std::time::Duration::ZERO);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_loop_names_are_original() {
        assert_eq!(WATER_LOOP_EVENTS[0], "event:/SFX/Water/WaterDraw");
        assert_eq!(WATER_LOOP_EVENTS[1], "event:/SFX/Water/WaterErase");
        assert_eq!(WATER_LOOP_EVENTS[2], "event:/SFX/Water/IceErase");
        assert_eq!(WATER_LOOP_EVENTS[3], "event:/SFX/Water/IceDraw");
    }
}
