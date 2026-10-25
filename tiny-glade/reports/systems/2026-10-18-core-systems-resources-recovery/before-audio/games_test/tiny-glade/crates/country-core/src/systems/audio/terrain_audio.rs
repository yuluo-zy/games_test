//! B `terrain_audio.rs`: terrain edit sounds — 4 loops + the terrain edit
//! event dispatch.
//!
//! Fresh evidence (decomp-0x1414613e0 / 0x1414618c0, disasm-0x141461c80..
//! 0x141463220, jump table 0x142c63274):
//! - init creates 4 loops — `event:/SFX/TerrainEditShape`,
//!   `event:/SFX/TerrainEditHeight`, `event:/SFX/Terrain Draw`,
//!   `event:/SFX/TerrainMove`.
//! - event slot stride 0x28: kind u32 @+8, position @+0xc (x,y,z), value@+0x18,
//!   value@+0x1c, value@+0x20.
//! - dispatch by native tag:
//!   0 → Terrain Draw loop (slot 2), no position/parameters;
//!   1 → TerrainEditShape (slot 0): `terrain_smoothness` = v18,
//!       editing speed = clamp01(inverse_lerp(0, 5, v1c / speed));
//!   2 → TerrainEditHeight (slot 1): `terrain_smoothness` = v18,
//!       `terrain_height` = inverse_lerp(0, 20, v1c),
//!       editing speed = clamp01(inverse_lerp(0, 5, v20 / speed));
//!   3 → TerrainMove (slot 3): same parameters as tag 2;
//!   4 → one-shot `event:/SFX/TerrainLowPoint`: set_position +
//!       `terrain_height` = inverse_lerp(0, 20, v18);
//!   5 → one-shot `event:/SFX/TerrainCreateChunk`: `terrain_smoothness` = v18,
//!       `editing_speed` = clamp01(inverse_lerp(0, 5, v1c / speed))
//!       — no Rust event variant carries native tag 5 (cross-domain gap,
//!       registered in the rebuild report).
//! - frame-end drive order: slots 2, 1, 0, 3; a flagged slot does
//!   `start_if_not_playing → set_parameter("editing_speed", slot_speed) →
//!   set_position(last event position)`, an unflagged slot is stopped; the
//!   flag is cleared. The speed divisor is the shared `+0x58` editing speed.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::audio::audio_update::EditingSpeed,
    systems::terrain::PlayTerrainSound,
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The four terrain edit loops (O: rne-read; slot order 0..3).
pub const TERRAIN_LOOP_EVENTS: [&str; 4] = [
    "event:/SFX/TerrainEditShape",
    "event:/SFX/TerrainEditHeight",
    "event:/SFX/Terrain Draw",
    "event:/SFX/TerrainMove",
];

/// One-shots (O: 0x142c62d58 / 0x142c62d90).
pub const TERRAIN_LOW_POINT_EVENT: &str = "event:/SFX/TerrainLowPoint";
pub const TERRAIN_CREATE_CHUNK_EVENT: &str = "event:/SFX/TerrainCreateChunk";

/// Instance parameters (O: 0x142c62d38 / 0x142c62d4a / 0x142c62943).
pub const TERRAIN_SMOOTHNESS_PARAM: &str = "terrain_smoothness";
pub const TERRAIN_HEIGHT_PARAM: &str = "terrain_height";
pub const TERRAIN_EDITING_SPEED_PARAM: &str = "editing_speed";

/// Frame-end drive order (O: decomp 0x1414622de..0x1414624f0).
pub const TERRAIN_DRIVE_ORDER: [usize; 4] = [2, 1, 0, 3];

/// TerrainAudioState: the four loops, per-slot should-play flags, the shared
/// last event position and per-slot editing-speed values.
#[derive(Resource)]
pub struct TerrainAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 4],
    pub should_play: [bool; 4],
    /// O: every event overwrites the shared frame position used by the
    /// frame-end `set_position`.
    pub position: Vec3,
    /// O: per-slot `editing_speed` values (native xmm7/xmm9/xmm8/xmm6).
    pub speed: [f32; 4],
}

impl<E: EventInstance> TerrainAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        TerrainAudioState {
            loops: TERRAIN_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
            should_play: [false; 4],
            position: Vec3::ZERO,
            speed: [0.0; 4],
        }
    }
}

/// `terrain_audio_init_system` (0x1414613e0).
pub fn terrain_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(TerrainAudioState::<S::Event>::new(&core));
}

/// Native `inverse_lerp` (0x140c904a0) — unclamped.
fn inverse_lerp(min: f32, max: f32, value: f32) -> f32 {
    (value - min) / (max - min)
}

fn clamp01(value: f32) -> f32 {
    value.max(0.0).min(1.0)
}

/// `terrain_audio_system` (0x1414618c0).
pub fn terrain_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<TerrainAudioState<S::Event>>,
    mut events: EventReader<PlayTerrainSound>,
    core: Res<AudioCore<S>>,
    speed: Res<EditingSpeed>,
) {
    let editing_speed = speed.0;
    for sound in events.read() {
        let tag = sound.native_tag();
        let (position, value0, value1, value2) = match sound {
            PlayTerrainSound::StrokeChanged => (None, 0.0, 0.0, 0.0),
            PlayTerrainSound::Falloff {
                position,
                falloff,
                magnitude,
            } => (Some(*position), *falloff, *magnitude, 0.0),
            PlayTerrainSound::Height {
                position,
                falloff,
                height,
                magnitude,
            } => (Some(*position), *falloff, *height, *magnitude),
            PlayTerrainSound::Points {
                position,
                falloff,
                height,
                magnitude,
            } => (Some(*position), *falloff, *height, *magnitude),
            PlayTerrainSound::Removed {
                position,
                falloff,
                magnitude: _,
            } => (Some(*position), *falloff, 0.0, 0.0),
        };

        let set_param = |state: &mut TerrainAudioState<S::Event>, slot: usize, name: &str, value: f32| {
            let cursor = state.loops[slot].cursor;
            let _ = state.loops[slot].entries[cursor].set_parameter(name, value);
        };

        match tag {
            // 0 — Terrain Draw loop: only the flag (no position/parameters).
            0 => {
                state.should_play[2] = true;
            }
            // 1 — TerrainEditShape (slot 0).
            1 => {
                state.should_play[0] = true;
                state.position = position.unwrap_or(Vec3::ZERO);
                state.speed[0] = clamp01(inverse_lerp(0.0, 5.0, value1 / editing_speed));
                set_param(&mut state, 0, TERRAIN_SMOOTHNESS_PARAM, value0);
            }
            // 2 — TerrainEditHeight (slot 1).
            2 => {
                state.should_play[1] = true;
                state.position = position.unwrap_or(Vec3::ZERO);
                state.speed[1] = clamp01(inverse_lerp(0.0, 5.0, value2 / editing_speed));
                set_param(&mut state, 1, TERRAIN_SMOOTHNESS_PARAM, value0);
                set_param(
                    &mut state,
                    1,
                    TERRAIN_HEIGHT_PARAM,
                    inverse_lerp(0.0, 20.0, value1),
                );
            }
            // 3 — TerrainMove (slot 3).
            3 => {
                state.should_play[3] = true;
                state.position = position.unwrap_or(Vec3::ZERO);
                state.speed[3] = clamp01(inverse_lerp(0.0, 5.0, value2 / editing_speed));
                set_param(&mut state, 3, TERRAIN_SMOOTHNESS_PARAM, value0);
                set_param(
                    &mut state,
                    3,
                    TERRAIN_HEIGHT_PARAM,
                    inverse_lerp(0.0, 20.0, value1),
                );
            }
            // 4 — TerrainLowPoint one-shot.
            4 => {
                let position = position.unwrap_or(Vec3::ZERO);
                let mut instance = core.instantiate_audio(TERRAIN_LOW_POINT_EVENT);
                let _ = instance.set_position([position.x, position.y, position.z]);
                let _ = instance.start_if_not_playing();
                let _ = instance.set_parameter(
                    TERRAIN_HEIGHT_PARAM,
                    inverse_lerp(0.0, 20.0, value0),
                );
            }
            // 5 — TerrainCreateChunk one-shot (native tag 5; no Rust event
            // variant carries it — cross-domain gap).
            _ => {}
        }
    }

    // O: frame-end drive order [2, 1, 0, 3].
    for slot in TERRAIN_DRIVE_ORDER {
        let play = state.should_play[slot];
        let speed = state.speed[slot];
        let position = state.position;
        let loop_player = &mut state.loops[slot];
        if play {
            let _ = loop_player.start_if_not_playing();
            let cursor = loop_player.cursor;
            let _ = loop_player.entries[cursor]
                .set_parameter(TERRAIN_EDITING_SPEED_PARAM, speed);
            let _ = loop_player.entries[cursor]
                .set_position([position.x, position.y, position.z]);
        } else {
            let _ = loop_player.stop(std::time::Duration::ZERO);
        }
        state.should_play[slot] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terrain_loop_names_are_original() {
        assert_eq!(TERRAIN_LOOP_EVENTS[0], "event:/SFX/TerrainEditShape");
        assert_eq!(TERRAIN_LOOP_EVENTS[1], "event:/SFX/TerrainEditHeight");
        assert_eq!(TERRAIN_LOOP_EVENTS[2], "event:/SFX/Terrain Draw");
        assert_eq!(TERRAIN_LOOP_EVENTS[3], "event:/SFX/TerrainMove");
    }

    #[test]
    fn terrain_drive_order_and_names_match_the_decomp() {
        assert_eq!(TERRAIN_DRIVE_ORDER, [2, 1, 0, 3]);
        assert_eq!(TERRAIN_SMOOTHNESS_PARAM, "terrain_smoothness");
        assert_eq!(TERRAIN_HEIGHT_PARAM, "terrain_height");
        assert_eq!(TERRAIN_LOW_POINT_EVENT, "event:/SFX/TerrainLowPoint");
        assert_eq!(TERRAIN_CREATE_CHUNK_EVENT, "event:/SFX/TerrainCreateChunk");
    }
}
