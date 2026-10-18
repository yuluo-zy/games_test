//! B `terrain_stairs_audio.rs`: terrain-stair draw sounds — a
//! `event:/SFX/StairDraw_OneShot` ring plus per-stroke one-shot instances.
//!
//! Fresh evidence (decomp-0x141462dc0, disasm 0x141462dc0..0x141463242):
//! - `AppMode == 5` → early return.
//! - the system consumes one event stream; no new events → return.
//! - `editing_speed = clamp01(inverse_lerp(0, 60, mode == 0xd ? 1.0 :
//!   CursorTerrainRaycast.speed))` (the clamped value feeds both the one-shot
//!   and the ring `editing_speed` parameters).
//! - `Vec<AudioInstance>` retain: keeps instances that are Active **and**
//!   Playing (drop the rest).
//! - when the vector is empty and the ring's current instance is Active and
//!   Playing, an 8-byte bookkeeping event is pushed and a counter is
//!   incremented (the event type lives in the terrain-stairs domain; registered
//!   as a host gap).
//! - an SSBO buffer value (SsboId at 0x142c2e1a0, name assertion length 0x45) is
//!   compared with the stored value: equal → the ring is stopped and the system
//!   returns; different → store the value, instantiate
//!   `event:/SFX/StairDraw_OneShot`, start it with `editing_speed`, push it into
//!   the vector, start the ring and set the ring's `editing_speed`.
//!   The SSBO id / grid indexing is host-side (registered); this file consumes
//!   the host-provided value.

use crate::{
    resources::{
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
        audio_instance::PlaybackState,
        cursor_terrain_raycast::CursorTerrainRaycast,
    },
    systems::mode_manager::AppMode,
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
/// `event:/SFX/StairDraw_OneShot` (O: 0x142c62e70).
pub const STAIR_DRAW_ONE_SHOT: &str = "event:/SFX/StairDraw_OneShot";
/// Instance parameter (O: 0x142c62943).
pub const STAIR_EDITING_SPEED_PARAM: &str = "editing_speed";

/// Host face for the native stair-edit event stream (producer: terrain stairs
/// domain, 待接线). The audio handler only needs the per-frame presence.
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TerrainStairsAudioEvent;

/// O (native SSBO buffer first u32; the exact `SsboType::NAME`/id is 待取证 —
/// the terrain-domain writer fills this host face).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct StairsRasterValue(pub u32);

/// StairsCounter (native `+0x40` counter, incremented when a ring event is
/// pushed).
#[derive(Debug, Default, Clone, Copy)]
pub struct StairsCounter {
    pub count: u32,
}

/// TerrainStairsAudioState (O: ring + retained instances + last raster value).
#[derive(Resource)]
pub struct TerrainStairsAudioState<E: EventInstance> {
    pub ring: AudioLoop<AudioInstance<E>>,
    pub instances: Vec<AudioInstance<E>>,
    pub counter: StairsCounter,
    /// Last observed host SSBO value (O: the state comparison).
    pub last_raster: u32,
}

impl<E: EventInstance> TerrainStairsAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        TerrainStairsAudioState {
            ring: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(STAIR_DRAW_ONE_SHOT),
            ),
            instances: Vec::new(),
            counter: StairsCounter::default(),
            last_raster: 0,
        }
    }
}

/// `terrain_stairs_audio_init_system` (native inline init).
pub fn terrain_stairs_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(TerrainStairsAudioState::<S::Event>::new(&core));
}

fn clamp01(value: f32) -> f32 {
    value.max(0.0).min(1.0)
}

/// `terrain_stairs_audio_system` (0x141462dc0).
pub fn terrain_stairs_audio_system<S: Studio + Send + Sync + 'static>(
    mode: Res<AppMode>,
    mut state: ResMut<TerrainStairsAudioState<S::Event>>,
    mut events: EventReader<TerrainStairsAudioEvent>,
    core: Res<AudioCore<S>>,
    raster: Res<StairsRasterValue>,
    raycast: Res<CursorTerrainRaycast>,
) {
    // O: AppMode == 5 (terrain edit) → return.
    if mode.native_tag() == 5 {
        return;
    }
    // O: no new stair-edit events → return (the ring is not stopped).
    if events.read().next().is_none() {
        return;
    }

    // O: editing_speed = clamp01(inverse_lerp(0, 60, mode == Replay ? 1.0 :
    // raycast.speed)).
    let source = if mode.native_tag() == 0xd {
        1.0
    } else {
        raycast.speed
    };
    let editing_speed = clamp01(source / 60.0);

    // O: retain Active + Playing instances only.
    state.instances.retain(|instance| match instance {
        AudioInstance::Active { event, .. } => {
            event.get_playback_state().map_or(false, |s| s == PlaybackState::Playing)
        }
        _ => false,
    });

    // O: empty vector + ring current Active and Playing → push bookkeeping
    // event / counter (the native event type is the terrain-stairs host gap).
    if state.instances.is_empty() {
        let ring_current = &state.ring.entries[state.ring.cursor];
        let playing = match ring_current {
            AudioInstance::Active { event, .. } => event
                .get_playback_state()
                .map_or(false, |s| s == PlaybackState::Playing),
            _ => false,
        };
        if playing {
            state.counter.count = state.counter.count.wrapping_add(1);
        }
    }

    // O: SSBO value compare.
    if raster.0 == state.last_raster {
        // O: same value → stop the ring + return.
        let _ = state.ring.stop_now();
        return;
    }
    state.last_raster = raster.0;

    // O: changed → StairDraw_OneShot + editing_speed, push, ring start + param.
    let mut one_shot = core.instantiate_audio(STAIR_DRAW_ONE_SHOT);
    let _ = one_shot.start_if_not_playing();
    let _ = one_shot.set_parameter(STAIR_EDITING_SPEED_PARAM, editing_speed);
    state.instances.push(one_shot);
    let ring = &mut state.ring;
    let _ = ring.start_if_not_playing();
    let cursor = ring.cursor;
    let _ = ring.entries[cursor].set_parameter(STAIR_EDITING_SPEED_PARAM, editing_speed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stair_draw_one_shot_event_and_param_are_original() {
        assert_eq!(STAIR_DRAW_ONE_SHOT, "event:/SFX/StairDraw_OneShot");
        assert_eq!(STAIR_EDITING_SPEED_PARAM, "editing_speed");
    }
}
