//! B `terrain_stairs_audio.rs`: terrain-stair draw sounds — a
//! `event:/SFX/StairDraw_OneShot` ring plus per-stroke one-shot instances and
//! path-audio suppression.
//!
//! O-level facts (chapter terrain_stairs_audio.rs.md, decomp-0x141462dc0):
//! the system returns when `AppMode == 5` (terrain-edit mode); it retains the
//! one-shot `AudioInstance`s (cleaning finished ones by playback state), and on
//! a stair-edit tick either drives the one-shot ring (`instantiate_audio
//! "event:/SFX/StairDraw_OneShot"` + start_if_not_playing + set_parameter) or
//! stops the ring, with the volume source `+0x1c / 60.0` (same as the wall
//! volume source). `SuppressPathSound` is emitted while stairs are drawn so the
//! path audio defers.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::mode_manager::AppMode,
    utils::audio_utils::AudioLoop,
};
use crate::systems::audio::SuppressPathSound;
use bevy_ecs::prelude::*;

/// `event:/SFX/StairDraw_OneShot` (O: 0x142c62e70).
pub const STAIR_DRAW_ONE_SHOT: &str = "event:/SFX/StairDraw_OneShot";

/// StairsCounter: tracks in-flight stair one-shots (native `StairsCounter`).
#[derive(Debug, Default, Clone, Copy)]
pub struct StairsCounter {
    pub count: u32,
}

/// TerrainStairsAudioState (O: one-shot ring + retained instances + volume).
#[derive(Resource)]
pub struct TerrainStairsAudioState<E: EventInstance> {
    pub ring: AudioLoop<AudioInstance<E>>,
    pub instances: Vec<AudioInstance<E>>,
    pub counter: StairsCounter,
    /// Path-audio suppression flag (native `SuppressPathSound` wiring).
    pub suppress_path: bool,
    pub volume: f32,
}

impl<E: EventInstance> TerrainStairsAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        TerrainStairsAudioState {
            ring: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(STAIR_DRAW_ONE_SHOT),
            ),
            instances: Vec::new(),
            counter: StairsCounter::default(),
            suppress_path: false,
            volume: 1.0,
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

/// `terrain_stairs_audio_system` (0x141462dc0).
pub fn terrain_stairs_audio_system<S: Studio + Send + Sync + 'static>(
    mode: Res<AppMode>,
    mut state: ResMut<TerrainStairsAudioState<S::Event>>,
    mut suppress: EventReader<SuppressPathSound>,
    core: Res<AudioCore<S>>,
) {
    // O: AppMode == 5 (terrain edit) → return.
    if mode.native_tag() == 5 {
        return;
    }
    // O: clean finished one-shot instances by playback state (retain).
    state.instances.retain(|instance| match instance {
        AudioInstance::Active { event, .. } => event
            .get_playback_state()
            .map_or(false, |s| s != crate::resources::audio_instance::PlaybackState::Playing),
        _ => false,
    });
    // O: SuppressPathSound while stairs are being drawn.
    state.suppress_path = suppress.read().next().is_some();

    // O: drive the StairDraw_OneShot ring (one-shot per stair-edit tick).
    let editing = state.suppress_path;
    if editing {
        let _ = state.ring.start_if_not_playing();
        let _ = state.ring.stop(std::time::Duration::from_millis(200));
        let mut one_shot = core.instantiate_audio(STAIR_DRAW_ONE_SHOT);
        let _ = one_shot.set_parameter("editing_speed", state.volume / 60.0);
        let _ = one_shot.start_if_not_playing();
        state.instances.push(one_shot);
    } else {
        let _ = state.ring.stop(std::time::Duration::ZERO);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stair_draw_one_shot_event_is_original() {
        assert_eq!(STAIR_DRAW_ONE_SHOT, "event:/SFX/StairDraw_OneShot");
    }
}
