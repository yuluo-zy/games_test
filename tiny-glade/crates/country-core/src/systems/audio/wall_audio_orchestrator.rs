//! B `wall_audio_orchestrator.rs`: wall material audio coordination.
//!
//! Fresh evidence (decomp-0x141234bf0):
//! - `hash_one<GlobalId>` looks the request up in the wall state table (0x2a8
//!   stride), asserts `w.simplified_curve_2d.is_valid()` (panic on failure),
//!   then `MaterialStats::which_material_changed_the_most()` /
//!   `MaterialStats::max_pillar_movement()` decide a `PlayWallAudio` event
//!   (`EventWriter::write<PlayWallAudio>`) and an
//!   `indexmap::Bucket<(DecoratorId, DecoratorId), AlignKind>` entry is written
//!   into the stairs assembly parameters.
//! - The previous `set_parameter(param, value)` model was a self-approximation
//!   and is dropped: the native never touches an FMOD instance here.
//!
//! Cross-domain gap (registered): the wall table (`MaterialStats` methods), the
//! `simplified_curve_2d` validity and the `AlignKind` map are wall/stairs-domain
//! state. This file declares the host face (`WallMaterialAudioHost`) and emits
//! the `PlayWallAudio` events; the host fills the decisions.

use crate::{
    resources::audio_resources::{AudioCore, Studio},
    systems::audio::{PlayWallAudio, RequestWallMaterialAudio},
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// Host decision for one wall request (the native MaterialStats computation).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WallMaterialChange {
    /// `PlayWallAudio.kind`.
    pub kind: u8,
    /// Generic scalar payload.
    pub value: f32,
    /// Position passed to the one-shot / loop.
    pub position: Vec3,
    /// Volume.
    pub volume: f32,
    /// Flag byte.
    pub flag: u8,
}

/// Host face for the wall/stairs domain inputs consumed by the orchestrator.
#[derive(Resource, Debug, Default)]
pub struct WallMaterialAudioHost {
    /// `w.simplified_curve_2d.is_valid()` for the requested wall (native
    /// panics when false).
    pub curve_valid: bool,
    /// Per-`GlobalId` material-change decision (`MaterialStats` output).
    pub changes: std::collections::HashMap<u64, WallMaterialChange>,
    /// `(DecoratorId, DecoratorId) → AlignKind` assembly params written by the
    /// orchestrator (`indexmap::Bucket` host face).
    pub alignments: Vec<((u32, u32), u8)>,
}

/// `wall_audio_orchestrator` (0x141234bf0).
pub fn wall_audio_orchestrator<S: Studio + Send + Sync + 'static>(
    mut events: EventReader<RequestWallMaterialAudio>,
    host: Res<WallMaterialAudioHost>,
    mut wall_audio: EventWriter<PlayWallAudio>,
    _core: Res<AudioCore<S>>,
) {
    for request in events.read() {
        // O: `assert!(w.simplified_curve_2d.is_valid())`.
        assert!(
            host.curve_valid,
            "assertion failed: w.simplified_curve_2d.is_valid()"
        );
        // O: MaterialStats decision → PlayWallAudio event.
        if let Some(change) = host.changes.get(&request.id) {
            wall_audio.write(PlayWallAudio {
                kind: change.kind,
                value: change.value,
                position: change.position,
                volume: change.volume,
                flag: change.flag,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_face_defaults_are_empty() {
        let host = WallMaterialAudioHost::default();
        assert!(!host.curve_valid);
        assert!(host.changes.is_empty());
        assert!(host.alignments.is_empty());
    }
}
