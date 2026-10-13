//! sheep_audio_system (0x1409d5ea0), B §3e. Jump table 0x142ac2cc8, kinds 0..=7.
use crate::backend::AudioBackendRes;
use crate::events::PlaySheepReactSound;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

/// (name, doc length). kind 2 ("So") name/length UNKNOWN(evidence: B §3e): the address gap
/// 0x142ac23da->0x142ac23ee is 20 bytes, which does not fit "event:/SFX/Sheep/So" (19).
/// kind 4 -> frame-tail start/stop branch (0x1409d61b0), no one-shot; per-case set_parameter UNKNOWN.
pub const SHEEP_KINDS: [Option<(&str, usize)>; 8] = [
    Some(("event:/SFX/Sheep/Aww", 0x14)),
    Some(("event:/SFX/Sheep/Think", 0x16)),
    None, // UNKNOWN(evidence: B §3e)
    Some(("event:/SFX/Sheep/Angry", 0x16)),
    None, // kind 4: tail branch, not a one-shot (B §3e)
    Some(("event:/SFX/Sheep/Idea", 0x15)),
    Some(("event:/SFX/Sheep/Eyes", 0x15)),
    Some(("event:/SFX/Sheep/Confused", 0x19)),
];

pub fn sheep_audio_system(mut backend: ResMut<AudioBackendRes>, mut events: EventReader<PlaySheepReactSound>) {
    for ev in events.read() {
        let Some(Some((name, _))) = SHEEP_KINDS.get(ev.kind as usize) else { continue };
        let id = backend.instantiate_audio(name);
        backend.start_if_not_playing(id);
    }
}

/// Sheep part of the audio plugin.
pub struct SheepAudioPlugin;

impl Plugin for SheepAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySheepReactSound>()
            .add_systems(Update, sheep_audio_system.in_set(crate::plugin::AudioFrame::Sheep));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::Call;
    use crate::test_util::{app_with, frame};

    #[test]
    fn lengths() {
        for e in SHEEP_KINDS.iter().flatten() {
            assert_eq!(e.0.len(), e.1);
        }
    }

    #[test]
    fn each_kind() {
        for (k, e) in SHEEP_KINDS.iter().enumerate() {
            let (mut app, rec) = app_with(SheepAudioPlugin);
            let calls = frame(&mut app, &rec, [PlaySheepReactSound { kind: k as u8 }]);
            match e {
                Some((n, _)) => assert_eq!(
                    calls,
                    vec![Call::InstantiateAudio { name: (*n).into(), id: 0 }, Call::Start { id: 0 }]
                ),
                None => assert!(calls.is_empty(), "kind {k}"),
            }
        }
    }

    #[test]
    fn oob_kind() {
        let (mut app, rec) = app_with(SheepAudioPlugin);
        assert!(frame(&mut app, &rec, [PlaySheepReactSound { kind: 8 }]).is_empty());
    }
}
