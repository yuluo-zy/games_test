//! menu_audio (0x140a0b4c0) / init (0x140a0b340), B §3d.
use crate::backend::{AudioBackendRes, InstanceId};
use crate::events::PlayMenuUiSound;
use crate::resources::AppMode;
use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;

pub const PAUSE_SNAPSHOT: &str = "snapshot:/PauseMenu";

/// kind 0..=12. `Some` only where the doc names the event. Kinds 3/5/6/10 are UNKNOWN
/// (evidence: B §3d "逐条未钉死"); address spacing suggests Settings/SliderTick/ShowSessions/Open
/// but that is inference and no behavior is built on it. Full-name prefixes `event:/UI/` for
/// kind 1 is derived from the 19-byte spacing 0x142ac6e44->0x142ac6e57.
pub const MENU_KINDS: [Option<&str>; 13] = [
    Some("event:/UI/Menu/Hover"),
    Some("event:/UI/Ui2dHover"),
    Some("event:/UI/Menu/Select"),
    None, // UNKNOWN(evidence: B §3d)
    Some("event:/UI/Menu/SettingsTab"),
    None, // UNKNOWN(evidence: B §3d)
    None, // UNKNOWN(evidence: B §3d)
    Some("event:/UI/Menu/NewSession"),
    Some("event:/UI/Menu/StartSession"),
    Some("event:/UI/Menu/Back"),
    None, // UNKNOWN(evidence: B §3d)
    Some("event:/UI/Menu/Close"),
    Some("event:/UI/Ui2dLockedClickNew"),
];

#[derive(Resource, Debug)]
pub struct MenuAudioState {
    pub snapshot: InstanceId,
}

/// Init: instantiate the PauseMenu snapshot (B §3d; other init instances UNKNOWN).
pub fn menu_audio_init_system(mut commands: Commands, mut backend: ResMut<AudioBackendRes>) {
    let snapshot = backend.instantiate_audio(PAUSE_SNAPSHOT);
    commands.insert_resource(MenuAudioState { snapshot });
}

/// AppMode gate `(mode | 8) == 0xa` stops the snapshot, otherwise starts it.
pub fn menu_audio(
    mut backend: ResMut<AudioBackendRes>,
    state: Res<MenuAudioState>,
    mode: Res<AppMode>,
    mut events: EventReader<PlayMenuUiSound>,
) {
    let b = backend.get();
    if (mode.0 | 8) == 0xa {
        b.stop_instance(state.snapshot);
    } else {
        b.start_if_not_playing(state.snapshot);
    }
    for ev in events.read() {
        let Some(Some(name)) = MENU_KINDS.get(ev.kind as usize) else { continue };
        let id = b.instantiate_audio(name);
        b.start_if_not_playing(id);
    }
}

/// Menu part of the audio plugin.
pub struct MenuAudioPlugin;

impl Plugin for MenuAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayMenuUiSound>()
            .init_resource::<AppMode>()
            .add_systems(Startup, menu_audio_init_system.in_set(crate::plugin::AudioInit::Menu))
            .add_systems(Update, menu_audio.in_set(crate::plugin::AudioFrame::Menu));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::Call;
    use crate::test_util::{app_with, frame};

    fn run(mode: u32, kind: u8) -> Vec<Call> {
        let (mut app, rec) = app_with(MenuAudioPlugin);
        app.insert_resource(AppMode(mode));
        frame(&mut app, &rec, [PlayMenuUiSound { kind }])
    }

    #[test]
    fn named_kinds_play() {
        for (k, n) in MENU_KINDS.iter().enumerate() {
            let calls = run(0, k as u8);
            match n {
                Some(n) => assert_eq!(calls[1], Call::InstantiateAudio { name: (*n).into(), id: 1 }, "kind {k}"),
                None => assert_eq!(calls.len(), 1, "unknown kind {k} must do nothing"),
            }
        }
    }

    #[test]
    fn kind_13_out_of_range() {
        assert_eq!(run(0, 13).len(), 1);
    }

    #[test]
    fn mode_gate() {
        assert_eq!(run(0xa, 99)[0], Call::StopInstance { id: 0 });
        assert_eq!(run(0x2, 99)[0], Call::StopInstance { id: 0 }); // 2|8 == 0xa
        assert_eq!(run(0x1, 99)[0], Call::Start { id: 0 });
    }

    #[test]
    fn name_lengths_match_address_spacing() {
        // B §3d string addresses: Hover 0x142ac6e30, Ui2dHover e44, Select e57, SettingsTab e83,
        // NewSession ed1, StartSession eea, Back f05, Close f2b, LockedClick f3f.
        assert_eq!(MENU_KINDS[0].unwrap().len(), 0xe44 - 0xe30);
        assert_eq!(MENU_KINDS[1].unwrap().len(), 0xe57 - 0xe44);
        assert_eq!(MENU_KINDS[7].unwrap().len(), 0xeea - 0xed1);
        assert_eq!(MENU_KINDS[8].unwrap().len(), 0xf05 - 0xeea);
    }
}
