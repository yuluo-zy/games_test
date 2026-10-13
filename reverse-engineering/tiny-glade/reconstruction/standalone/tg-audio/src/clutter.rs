//! clutter_audio_system (0x1408c02f0), B §3f. Jump table 0x142aa1688, kinds 0..=0x26 (39).
use crate::backend::AudioBackendRes;
use crate::events::PlayClutterSoundCmd;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

pub const CLUTTER_KIND_COUNT: usize = 39;

/// Names are all `event:/SFX/Cosmetics/<X>`. Doc spot-checks give (name, length, string VA) but
/// NOT the kind each belongs to, so the kind->name table is
/// UNKNOWN(evidence: B §3f/§5, "其余逐 kind 未钉死"). `ClutterNames::default()` is all-None and
/// the system then does nothing; callers may inject a table once recovered.
pub const CLUTTER_SPOT_CHECKS: [(&str, usize, u64); 18] = [
    ("DoorBell", 0x1d, 0x142aa1299), ("BoxFish", 0x1c, 0x142aa125e), ("RackBrooms", 0x1f, 0x142aa127a),
    ("OldenLog", 0x1d, 0x142aa1167), ("Veggiestall", 0x20, 0x142aa123e),
    ("ClothlineConnectionAppear", 0x2e, 0x142aa1013), ("Shutters", 0x1d, 0x142aa0fd2),
    ("Bucket", 0x1b, 0x142aa0ebe), ("BoxWaterCan", 0x20, 0x142aa0e9e), ("ClayPot", 0x1c, 0x142aa0f0d),
    ("Anvil", 0x1a, 0x142aa11e5), ("Firewood", 0x1d, 0x142aa0f7c), ("Ladder", 0x1b, 0x142aa0f47),
    ("Basket", 0x1b, 0x142aa110c), ("Pumpkin", 0x1c, 0x142aa1127), ("BoxSack", 0x1c, 0x142aa0e82),
    ("ClothlineAppear", 0x24, 0x142aa0fef), ("Box", 0x26, 0x142aa0e6a),
];

#[derive(Resource, Clone, Debug)]
pub struct ClutterNames(pub [Option<String>; CLUTTER_KIND_COUNT]);

impl Default for ClutterNames {
    fn default() -> Self {
        ClutterNames(std::array::from_fn(|_| None))
    }
}

/// instantiate_audio + set_position + start_if_not_playing (set_parameter UNKNOWN).
pub fn clutter_audio_system(
    mut backend: ResMut<AudioBackendRes>,
    names: Res<ClutterNames>,
    mut events: EventReader<PlayClutterSoundCmd>,
) {
    for ev in events.read() {
        let Some(Some(name)) = names.0.get(ev.kind as usize) else { continue };
        let id = backend.instantiate_audio(name);
        backend.set_position(id, ev.position);
        backend.start_if_not_playing(id);
    }
}

/// Clutter part of the audio plugin.
pub struct ClutterAudioPlugin;

impl Plugin for ClutterAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayClutterSoundCmd>()
            .init_resource::<ClutterNames>()
            .add_systems(Update, clutter_audio_system.in_set(crate::plugin::AudioFrame::Clutter));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::Call;
    use crate::test_util::{app_with, frame};
    use glam::Vec3;

    #[test]
    fn unknown_table_is_inert() {
        let (mut app, rec) = app_with(ClutterAudioPlugin);
        let evs: Vec<_> = (0..=0x26).map(|k| PlayClutterSoundCmd { position: Vec3::ZERO, kind: k }).collect();
        assert!(frame(&mut app, &rec, evs).is_empty());
    }

    #[test]
    fn injected_table_plays_and_bounds() {
        let (mut app, rec) = app_with(ClutterAudioPlugin);
        app.world_mut().resource_mut::<ClutterNames>().0[5] = Some("event:/SFX/Cosmetics/Test".into());
        let p = Vec3::new(1.0, 0.0, 2.0);
        let calls = frame(
            &mut app,
            &rec,
            [PlayClutterSoundCmd { position: p, kind: 5 }, PlayClutterSoundCmd { position: p, kind: 0x27 }],
        );
        assert_eq!(calls.len(), 3);
        assert_eq!(calls[1], Call::SetPosition { id: 0, pos: p });
    }

    #[test]
    fn spot_checks_recorded_but_not_mapped_to_kinds() {
        // Doc lengths do not all fit `event:/SFX/Cosmetics/<name>` (e.g. Box = 0x26), so the
        // (name,len) pairs are kept as raw evidence only.
        assert_eq!(CLUTTER_SPOT_CHECKS.len(), 18);
        assert_eq!(CLUTTER_KIND_COUNT, 0x26 + 1);
    }
}
