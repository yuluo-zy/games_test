//! B `trigger_audio.rs`: audio trigger boxes — camera segment ×
//! `AudioTriggerLibrary` intersection drives the "InForest" ambience.
//!
//! O-level facts (chapter trigger_audio.rs.md, decomp-0x1409ea8d0 /
//! 0x1409eafa0, rne-read):
//! - init: 4 × `instantiate_audio` — `event:/SFX/Camera Tree Rustle`,
//!   `event:/Amb/High Wind`, `event:/SFX/Camera Whoosh`, `snapshot:/InForest`.
//! - system: camera movement segment × every `Aabb3` trigger box; a hit adds
//!   `+0.5` to audibility (else `−0.1`), clamped `[0,1]`. Audibility ≤ 0 stops
//!   High Wind; the wind volume is `segment_len / threshold × 0.0125 × distance
//!   falloff`, lerp-smoothed. Camera `z ≥ 11` or `dist ≤ 70` stops the InForest
//!   snapshot; otherwise start + set_parameter.

use crate::{
    asset_libraries::audio_trigger_library::{AudioTriggerKind, AudioTriggerLibrary},
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
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

/// `Aabb3::intersect_segment` input — the camera's movement segment (native
/// reads the camera transform; modeled as a resource so the camera_rig domain
/// can wire it 待接线).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CameraSegment {
    pub from: Vec3,
    pub to: Vec3,
}

/// TriggerAudioState (O: the four instances + audibility @+0x34 + volume @+0x35).
#[derive(Resource)]
pub struct TriggerAudioState<E: EventInstance> {
    pub tree_rustle: AudioInstance<E>,
    pub high_wind: AudioInstance<E>,
    pub camera_whoosh: AudioInstance<E>,
    pub in_forest: AudioInstance<E>,
    pub audibility: f32,
    pub wind_volume: f32,
}

impl<E: EventInstance> TriggerAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        let [tree_rustle, high_wind, camera_whoosh, in_forest] =
            TRIGGER_EVENTS.map(|name| core.instantiate_audio(name));
        TriggerAudioState {
            tree_rustle,
            high_wind,
            camera_whoosh,
            in_forest,
            audibility: 0.0,
            wind_volume: 0.0,
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

/// `trigger_audio_system` (0x1409eafa0).
pub fn trigger_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<TriggerAudioState<S::Event>>,
    camera: Res<CameraSegment>,
    triggers: Res<AudioTriggerLibrary>,
    _core: Res<AudioCore<S>>,
) {
    let segment = camera.to - camera.from;
    let segment_len = segment.length();

    // O: intersect the camera segment with every trigger box.
    let hit = triggers
        .get_triggers(AudioTriggerKind::Primary)
        .boxes()
        .any(|(_, box_shape)| box_shape.intersect_segment(camera.from, camera.to).is_some());

    // O: audibility +0.5 on hit else −0.1, clamp [0,1].
    state.audibility = (state.audibility + if hit { 0.5 } else { -0.1 }).clamp(0.0, 1.0);

    // O: audibility ≤ 0 → stop High Wind.
    if state.audibility <= 0.0 {
        let _ = state.high_wind.stop();
    } else {
        let _ = state.high_wind.start_if_not_playing();
        // O: volume = segment / threshold × 0.0125 × distance falloff,
        // lerp-smoothed toward the target.
        let target = (segment_len / 70.0).clamp(0.0, 1.0) * 0.0125;
        state.wind_volume += (target - state.wind_volume) * 0.05;
        let volume = state.wind_volume;
        let _ = state.high_wind.set_volume(volume);
    }

    // O: camera z ≥ 11 or dist ≤ 70 → stop InForest snapshot, else start.
    let z_gate = camera.to.z >= 11.0 || segment_len <= 70.0;
    if z_gate {
        let _ = state.in_forest.stop();
    } else {
        let _ = state.in_forest.start_if_not_playing();
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
}
