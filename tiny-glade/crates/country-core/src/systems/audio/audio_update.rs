//! B `audio_update.rs`: per-frame / boot audio update — push AudioSettings
//! volumes to the 5 FMOD VCAs and the camera (or default) pose as the 3D
//! listener to Studio.
//!
//! O-level facts (chapter audio_update.rs.md, audio-misc ALGORITHMS §2):
//! - `audio_update_system` 0x1409c8dd0: `[AudioCore+0x10]` non-empty → 5×
//!   `set_vca_volume` (0x140a54810): Master@+0xf8 (`vca:/Master`),
//!   Music@+0xfc (`vca:/Music`), Music-DawnJingle@+0xfc, SFX@+0x100
//!   (`vca:/SFX`), Amb@+0x104 (`vca:/Amb`) — volumes from AudioSettings.
//!   Listener: camera rotation @+0xc0 (3×3) + translation @+0xf0 → SIMD
//!   position/orientation. Then `AudioCore::update` (0x140a53a00).
//! - `boot_audio_update_system` 0x1409c9320: same VCA section; listener uses a
//!   fixed default pose (does not read the camera).

use crate::resources::audio_resources::{AudioCore, ListenerAttributes, Studio};
use bevy_ecs::prelude::*;
use glam::{Mat3, Vec3};
pub use crate::resources::audio_settings::AudioSettings;

/// The 5 FMOD VCA names (O: audio-misc multi-baseline §2).
pub const VCA_MASTER: &str = "vca:/Master";
pub const VCA_MUSIC: &str = "vca:/Music";
pub const VCA_MUSIC_DAWN_JINGLE: &str = "vca:/Music-DawnJingle";
pub const VCA_SFX: &str = "vca:/SFX";
pub const VCA_AMB: &str = "vca:/Amb";

/// Push all five VCA volumes (O: the shared first section of both systems).
fn push_vca_volumes<S: Studio>(
    core: &mut AudioCore<S>,
    settings: &AudioSettings,
) -> Result<(), S::Error> {
    core.set_vca_volume(VCA_MASTER, settings.master_volume)?;
    core.set_vca_volume(VCA_MUSIC, settings.music_volume)?;
    core.set_vca_volume(VCA_MUSIC_DAWN_JINGLE, settings.music_volume)?;
    core.set_vca_volume(VCA_SFX, settings.sfx_volume)?;
    core.set_vca_volume(VCA_AMB, settings.ambience_volume)
}

/// Camera pose → 3D listener. The original reads the camera rotation @+0xc0
/// (3×3) and translation @+0xf0, then folds velocity 0.
pub fn camera_listener(rotation: Mat3, translation: Vec3) -> ListenerAttributes {
    ListenerAttributes {
        position: [translation.x, translation.y, translation.z],
        // Camera forward is -Z in the original convention (O: set_position uses
        // forward (0,0,-1)).
        forward: [-rotation.z_axis.x, -rotation.z_axis.y, -rotation.z_axis.z],
        up: [rotation.y_axis.x, rotation.y_axis.y, rotation.y_axis.z],
        velocity: [0.0, 0.0, 0.0],
    }
}

/// `audio_update_system` (0x1409c8dd0): per-frame update.
#[allow(clippy::too_many_arguments)]
pub fn audio_update_system<S: Studio + Send + Sync + 'static>(
    mut core: ResMut<AudioCore<S>>,
    config: Res<crate::game_config::GameConfig>,
    camera: Res<crate::systems::camera_rig::MainCamera>,
) {
    if !core.is_connected() {
        return;
    }
    let _ = push_vca_volumes(&mut *core, &config.audio);
    let inverse_view = camera.camera_matrices().inverse_view;
    let listener = camera_listener(Mat3::from_mat4(inverse_view), inverse_view.w_axis.truncate());
    let _ = core.update(listener);
}

/// `boot_audio_update_system` (0x1409c9320): startup variant with a fixed
/// default listener pose.
pub fn boot_audio_update_system<S: Studio + Send + Sync + 'static>(
    mut core: ResMut<AudioCore<S>>,
    config: Res<crate::game_config::GameConfig>,
) {
    if !core.is_connected() {
        return;
    }
    let _ = push_vca_volumes(&mut *core, &config.audio);
    let _ = core.update(default_listener());
}

/// Fixed default listener pose (O: boot variant constructs -1.0 constants).
fn default_listener() -> ListenerAttributes {
    ListenerAttributes {
        position: [0.0, 0.0, 0.0],
        forward: [0.0, 0.0, -1.0],
        up: [0.0, 1.0, 0.0],
        velocity: [0.0, 0.0, 0.0],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vca_names_and_offsets_are_documented_constants() {
        assert_eq!(VCA_MASTER, "vca:/Master");
        assert_eq!(VCA_MUSIC, "vca:/Music");
        assert_eq!(VCA_MUSIC_DAWN_JINGLE, "vca:/Music-DawnJingle");
        assert_eq!(VCA_SFX, "vca:/SFX");
        assert_eq!(VCA_AMB, "vca:/Amb");
    }

    #[test]
    fn default_listener_uses_the_original_constants() {
        let l = default_listener();
        assert_eq!(l.forward, [0.0, 0.0, -1.0]);
        assert_eq!(l.up, [0.0, 1.0, 0.0]);
        assert_eq!(l.velocity, [0.0, 0.0, 0.0]);
    }
}
