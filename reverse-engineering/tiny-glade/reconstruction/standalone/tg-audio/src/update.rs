//! audio_update_system (0x1409c8dd0) / boot_audio_update_system (0x1409c9320), A §2.
use crate::backend::{AudioBackend, AudioBackendRes, Listener};
use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;

/// AudioSettings volumes (+0xf8 master, +0xfc music, +0x100 sfx, +0x104 amb).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct AudioSettings {
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
    pub amb: f32,
}

/// Which AudioSettings field feeds each VCA, in call order (A §2a table).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Field {
    Master,
    Music,
    Sfx,
    Amb,
}
pub const VCAS: [(&str, Field); 5] = [
    ("vca:/Master", Field::Master),
    ("vca:/Music", Field::Music),
    ("vca:/Music-DawnJingle", Field::Music),
    ("vca:/SFX", Field::Sfx),
    ("vca:/Amb", Field::Amb),
];

impl AudioSettings {
    fn get(&self, f: Field) -> f32 {
        match f {
            Field::Master => self.master,
            Field::Music => self.music,
            Field::Sfx => self.sfx,
            Field::Amb => self.amb,
        }
    }
}

fn set_vcas(b: &mut dyn AudioBackend, s: &AudioSettings) {
    for (n, f) in VCAS {
        b.set_vca_volume(n, s.get(f));
    }
}

/// Listener pose derived from the camera.
/// UNKNOWN(evidence: A §2a): camera transform -> listener math; the host supplies the pose.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct ListenerPose(pub Listener);

/// Runtime: 5 VCA volumes then listener from the camera. Skipped until the host inserts both
/// [`AudioSettings`] and [`ListenerPose`].
pub fn audio_update_system(mut backend: ResMut<AudioBackendRes>, s: Res<AudioSettings>, pose: Res<ListenerPose>) {
    set_vcas(backend.get(), &s);
    backend.update_listener(Some(pose.0));
}

/// Boot: same VCAs, default listener pose (values UNKNOWN, A §2b). Skipped without
/// [`AudioSettings`].
pub fn boot_audio_update_system(mut backend: ResMut<AudioBackendRes>, s: Res<AudioSettings>) {
    set_vcas(backend.get(), &s);
    backend.update_listener(None);
}

/// Update part of the audio plugin (`audio_update_system` + `boot_audio_update_system`).
pub struct AudioUpdatePlugin;

impl Plugin for AudioUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            boot_audio_update_system
                .in_set(crate::plugin::AudioInit::BootUpdate)
                .run_if(resource_exists::<AudioSettings>),
        )
        .add_systems(
            Update,
            audio_update_system
                .in_set(crate::plugin::AudioFrame::Update)
                .run_if(resource_exists::<AudioSettings>.and(resource_exists::<ListenerPose>)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::Call;
    use crate::test_util::build;
    use glam::Vec3;

    const S: AudioSettings = AudioSettings { master: 0.9, music: 0.5, sfx: 0.7, amb: 0.3 };

    #[test]
    fn runtime_vcas_and_listener() {
        let (mut app, rec) = build(AudioUpdatePlugin);
        let l = Listener { position: Vec3::Y, forward: Vec3::NEG_Z, up: Vec3::Y };
        app.insert_resource(S).insert_resource(ListenerPose(l));
        app.update();
        let calls = rec.calls();
        // Startup boot (6 calls) then the Update system (6 calls); inspect the runtime half.
        assert_eq!(calls.len(), 12);
        let calls = &calls[6..];
        let vals: Vec<_> = calls
            .iter()
            .filter_map(|c| if let Call::SetVca { vca, volume } = c { Some((vca.as_str(), *volume)) } else { None })
            .collect();
        assert_eq!(
            vals,
            vec![
                ("vca:/Master", 0.9),
                ("vca:/Music", 0.5),
                ("vca:/Music-DawnJingle", 0.5),
                ("vca:/SFX", 0.7),
                ("vca:/Amb", 0.3)
            ]
        );
        assert_eq!(calls[5], Call::UpdateListener(Some(l)));
    }

    #[test]
    fn boot_matches_runtime_vcas_with_default_listener() {
        // no ListenerPose: the runtime system is skipped, only the boot one runs
        let (mut app, rec) = build(AudioUpdatePlugin);
        app.insert_resource(S);
        app.update();
        let calls = rec.calls();
        assert_eq!(calls.len(), 6);
        assert_eq!(calls[5], Call::UpdateListener(None));
        let vcas: Vec<_> = calls.iter().filter(|c| matches!(c, Call::SetVca { .. })).collect();
        assert_eq!(vcas.len(), 5);
    }

    #[test]
    fn skipped_without_settings() {
        let (mut app, rec) = build(AudioUpdatePlugin);
        app.update();
        assert!(rec.calls().is_empty());
    }
}
