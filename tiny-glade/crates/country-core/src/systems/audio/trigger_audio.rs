//! B `trigger_audio.rs`: audio trigger boxes — camera movement segment ×
//! `AudioTriggerLibrary` intersection drives the camera rustle / wind / whoosh
//! and the InForest snapshot.
//!
//! Fresh evidence (decomp-0x1409ea8d0 / 0x1409eafa0, disasm-0x1409eb3c0..):
//! - init: 4 × `instantiate_audio` — `event:/SFX/Camera Tree Rustle`,
//!   `event:/Amb/High Wind`, `event:/SFX/Camera Whoosh`, `snapshot:/InForest`.
//! - system (inside the `_DEV_OVERRIDE_IS_DEMO_MODE == 0`-independent path):
//!   early return when the shared editing speed `+0x58` is exactly 0 (non-NaN);
//!   camera eye position; camera movement segment vs every trigger box shrunk
//!   by 0.2 on each side (`Aabb3::intersect_segment`); a hit adds +0.5 to the
//!   audibility memory, else −0.1, clamped [0,1]; audibility > 0 starts the
//!   Camera Tree Rustle, otherwise it is stopped.
//!   Global params: `camera_height` = |eye| × ground factor (0.1 when the
//!   camera ground flag byte is set, else 1.0), `camera_speed` = smoothed wind
//!   volume, `InForest` = 0/1.
//!   High Wind starts every frame; its instance volume is `1.0`, or
//!   `1 + wind_volume × 0.5` when the in-forest state byte is set.
//!   Camera Whoosh starts every frame.
//!   InForest snapshot: stopped when `eye.y >= 11.0 || |eye| <= 70.0`,
//!   otherwise started; the `InForest` global parameter is set in both branches.

use crate::{
    asset_libraries::audio_trigger_library::{AudioTriggerKind, AudioTriggerLibrary},
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::audio::audio_update::EditingSpeed,
    systems::camera_rig::main_camera::MainCamera,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// Trigger one-shots (O: rne-read).
pub const TRIGGER_EVENTS: [&str; 4] = [
    "event:/SFX/Camera Tree Rustle",
    "event:/Amb/High Wind",
    "event:/SFX/Camera Whoosh",
    "snapshot:/InForest",
];

/// Native parameter names (O: 0x142ac51c0/0x142ac51cd/0x142ac51d9).
pub const CAMERA_HEIGHT_PARAM: &str = "camera_height";
pub const CAMERA_SPEED_PARAM: &str = "camera_speed";
pub const IN_FOREST_PARAM: &str = "InForest";

/// Ground-type → camera flag byte offset table (O: 0x142ac5670, six entries).
pub const GROUND_FLAG_OFFSETS: [usize; 6] = [0x8c, 0x144, 0x1c4, 0x264, 0x31c, 0x3a4];

/// Trigger box shrink applied before `intersect_segment`
/// (O: constants +0.2 / −0.2 at 0x142ac4cd0/0x142ac4ce0).
pub const TRIGGER_BOX_SHRINK: f32 = 0.2;

/// TriggerAudioState (O: four instances + `prev_valid@+0xc0`,
/// `prev_position@+0xc4`, `audibility@+0xd0`, `wind_volume@+0xd4`,
/// `in_forest_state@+0xd8`).
#[derive(Resource)]
pub struct TriggerAudioState<E: EventInstance> {
    pub tree_rustle: AudioInstance<E>,
    pub camera_whoosh: AudioInstance<E>,
    pub high_wind: AudioInstance<E>,
    pub in_forest: AudioInstance<E>,
    pub prev_valid: bool,
    pub prev_position: Vec3,
    pub audibility: f32,
    pub wind_volume: f32,
    /// O: `+0xd8` byte — written by the forest domain; boosts the wind volume
    /// and the wind target by ×10.
    pub in_forest_state: bool,
}

impl<E: EventInstance> TriggerAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        let [tree_rustle, camera_whoosh, high_wind, in_forest] =
            TRIGGER_EVENTS.map(|name| core.instantiate_audio(name));
        TriggerAudioState {
            tree_rustle,
            camera_whoosh,
            high_wind,
            in_forest,
            prev_valid: false,
            prev_position: Vec3::ZERO,
            audibility: 0.0,
            wind_volume: 0.0,
            in_forest_state: false,
        }
    }
}

/// `trigger_audio_init_system` (0x1409ea8d0).
pub fn trigger_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(TriggerAudioState::<S::Event>::new(&core));
}

/// The camera ground flag byte (O: `camera[0x128 + GROUND_FLAG_OFFSETS[ground]]`,
/// where `ground = camera[0x4d0]`).
fn ground_flag_set(camera: &MainCamera) -> bool {
    let bytes = camera.bytes();
    let ground = bytes[0x4d0] as usize;
    match GROUND_FLAG_OFFSETS.get(ground) {
        Some(offset) => bytes.get(0x128 + offset).copied().unwrap_or(0) != 0,
        // Native indexes the table unchecked; out-of-range ground types read
        // neighbouring memory. Treat them as "flag not set".
        None => false,
    }
}

/// `trigger_audio_system` (0x1409eafa0).
pub fn trigger_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<TriggerAudioState<S::Event>>,
    camera: Res<MainCamera>,
    triggers: Res<AudioTriggerLibrary>,
    core: Res<AudioCore<S>>,
    speed: Res<EditingSpeed>,
) {
    // O: `speed@+0x58 == 0.0` (non-NaN) → early return.
    let speed = speed.0;
    if speed == 0.0 && !speed.is_nan() {
        return;
    }

    let eye = camera.eye_position();

    // O: camera movement segment (previous eye → current eye); the stored
    // previous position updated every frame.
    let prev = if state.prev_valid {
        state.prev_position
    } else {
        eye
    };
    state.prev_position = eye;
    state.prev_valid = true;
    let movement = eye.distance(prev);

    // O: only when the movement / editing-speed ratio exceeds 1.0 is the
    // trigger-box intersection tested; the boxes are shrunk by 0.2 per side and
    // the first hit wins.
    let mut hit = false;
    if movement / speed > 1.0 {
        hit = triggers
            .get_triggers(AudioTriggerKind::Primary)
            .boxes()
            .any(|(_, box_shape)| {
                let shrunk = crate::utils_contract::Aabb3 {
                    min: box_shape.min + Vec3::splat(TRIGGER_BOX_SHRINK),
                    max: box_shape.max - Vec3::splat(TRIGGER_BOX_SHRINK),
                };
                shrunk.intersect_segment(prev, eye).is_some()
            });
    }

    // O: audibility + audibility += hit ? 0.5 : -0.1, clamped [0,1].
    let delta = if hit { 0.5 } else { -0.1 };
    state.audibility = (state.audibility + delta).clamp(0.0, 1.0);

    // O: audibility <= 0 → stop Camera Tree Rustle, else start it.
    if state.audibility <= 0.0 {
        let _ = state.tree_rustle.stop();
    } else {
        let _ = state.tree_rustle.start_if_not_playing();
    }

    // O: camera ground factor: 0.1 when the ground flag byte is set, else 1.0.
    let ground_factor = if ground_flag_set(&camera) { 0.1 } else { 1.0 };

    // O: `camera_height` = |eye| × ground factor.
    let _ = core.set_parameter(CAMERA_HEIGHT_PARAM, eye.length() * ground_factor);

    // O: wind target = ground factor × (movement / speed) × 0.0125,
    // ×10 when the in-forest state byte is set.
    let mut target = ground_factor * (movement / speed) * 0.0125;
    if state.in_forest_state {
        target *= 10.0;
    }
    // O: lerp factor 0.1 when the target drops below the current volume, else
    // 0.2 (table 0x142ac4cf8 = {0.2, 0.1}); clamp the result to ≤ 2.0.
    let factor = if target < state.wind_volume { 0.1 } else { 0.2 };
    state.wind_volume += (target - state.wind_volume) * factor;
    state.wind_volume = state.wind_volume.min(2.0);
    let _ = core.set_parameter(CAMERA_SPEED_PARAM, state.wind_volume);

    // O: Camera Whoosh and High Wind start every frame.
    let _ = state.camera_whoosh.start_if_not_playing();
    let _ = state.high_wind.start_if_not_playing();

    // O: High Wind instance volume = 1.0, or 1 + wind_volume × 0.5 in forest.
    let wind_volume = if state.in_forest_state {
        state.wind_volume * 0.5 + 1.0
    } else {
        1.0
    };
    let _ = state.high_wind.set_volume(wind_volume);

    // O: InForest snapshot gate: eye.y >= 11.0 || |eye| <= 70.0 stops it
    // (InForest = 0.0), otherwise it starts (InForest = 1.0).
    if eye.y >= 11.0 || eye.length() <= 70.0 {
        let _ = state.in_forest.stop();
        let _ = core.set_parameter(IN_FOREST_PARAM, 0.0);
    } else {
        let _ = state.in_forest.start_if_not_playing();
        let _ = core.set_parameter(IN_FOREST_PARAM, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_event_names_are_original() {
        assert_eq!(TRIGGER_EVENTS[0], "event:/SFX/Camera Tree Rustle");
        assert_eq!(TRIGGER_EVENTS[1], "event:/Amb/High Wind");
        assert_eq!(TRIGGER_EVENTS[2], "event:/SFX/Camera Whoosh");
        assert_eq!(TRIGGER_EVENTS[3], "snapshot:/InForest");
    }

    #[test]
    fn trigger_parameter_names_and_ground_table_are_original() {
        assert_eq!(CAMERA_HEIGHT_PARAM, "camera_height");
        assert_eq!(CAMERA_SPEED_PARAM, "camera_speed");
        assert_eq!(IN_FOREST_PARAM, "InForest");
        assert_eq!(GROUND_FLAG_OFFSETS, [0x8c, 0x144, 0x1c4, 0x264, 0x31c, 0x3a4]);
    }
}
