//! building_audio_init_system (0x1417d21a0) / building_audio (0x1417d2ad0), A §4, B §2.
use crate::backend::{AudioBackendRes, LoopId};
use crate::events::PlayBuildingAudio;
use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;
use std::collections::HashMap;

/// Init order of the 7 loops (B §2 offsets: Plaster @+0x118 is 6th, OverlapMove @+0x150 is 7th).
/// (name, doc len where given).
pub const BUILDING_LOOP_NAMES: [&str; 7] = [
    "event:/SFX/Building/BuildingResize",          // 0x22
    "event:/SFX/Building/BuildingRotateRectangle",
    "event:/SFX/Building/BuildingRotateCircle",
    "event:/SFX/Building/BuildingChangeHeight",
    "event:/SFX/Building/BuildingMove",
    "event:/SFX/PlasterChangeLoop",
    "event:/SFX/Building/BuildingOverlapMove",
];

/// Event-path kinds 6/7/8 -> (name, len); table index = kind-6 (B §2). Kinds outside 6..=8 are skipped.
pub const BUILDING_EVENT_KINDS: [(&str, usize); 3] = [
    ("event:/SFX/Shapes/DetachFromGround", 0x22),
    ("event:/SFX/Shapes/AttachToGround", 0x20),
    ("event:/SFX/Shapes/PeakReached", 0x1d),
];

/// Tail "default path" kind -> loop index into `BUILDING_LOOP_NAMES` (B §2 table; 6/7/8 unsupported).
/// UNKNOWN(evidence: B §5): what drives the tail path, so no behavior is built on it.
pub const BUILDING_TAIL_KIND_LOOP: [(u8, usize); 7] = [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 6), (9, 5)];

/// Parameters of the tail path, B §2 (names only).
pub const BUILDING_TAIL_PARAMS: [&str; 7] =
    ["editing_speed", "BuildingSize", "ifWindow", "BuildingHeight", "isHalfTimbered", "IsOnGround", "loop"];

#[derive(Resource, Debug)]
pub struct BuildingAudioState {
    pub loops: [LoopId; 7],
}

/// `building_audio_init_system` (Startup).
pub fn building_audio_init_system(mut commands: Commands, mut backend: ResMut<AudioBackendRes>) {
    let mut loops = [0; 7];
    for (i, n) in BUILDING_LOOP_NAMES.iter().enumerate() {
        loops[i] = backend.instantiate_loop(n, crate::wall::LOOP_DESC_COUNT, crate::wall::LOOP_DESC_MAX);
    }
    commands.insert_resource(BuildingAudioState { loops });
}

/// One entry of the 0x2a8 building table.
/// `size_param` is the already-normalised `BuildingSize`.
/// UNKNOWN(evidence: A §4b/B §2): the area `inverse_lerp` range (0x140c99520/0x140c97580),
/// so the table holds the normalised value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BuildingEntry {
    pub size_param: f32,
}

/// The hashbrown building table queried by `PlayBuildingAudio::id` (owned by the game; a
/// plain map resource here).
#[derive(Resource, Clone, Debug, Default)]
pub struct BuildingTable(pub HashMap<u64, BuildingEntry>);

/// `building_audio` event path.
/// UNKNOWN(evidence: B §2): behavior when the id is missing; we skip BuildingSize then.
pub fn building_audio(
    mut backend: ResMut<AudioBackendRes>,
    mut events: EventReader<PlayBuildingAudio>,
    table: Res<BuildingTable>,
) {
    let b = backend.get();
    for ev in events.read() {
        let Some(idx) = ev.kind.checked_sub(6).filter(|i| *i < 3) else { continue };
        let (name, _) = BUILDING_EVENT_KINDS[idx as usize];
        let inst = b.instantiate_audio(name);
        b.set_position(inst, ev.position);
        b.set_parameter(inst, "editing_speed", ev.editing_speed);
        if let Some(e) = table.0.get(&ev.id) {
            b.set_parameter(inst, "BuildingSize", e.size_param);
        }
        b.start_if_not_playing(inst);
    }
}

/// Building part of the audio plugin.
pub struct BuildingAudioPlugin;

impl Plugin for BuildingAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayBuildingAudio>()
            .init_resource::<BuildingTable>()
            .add_systems(Startup, building_audio_init_system.in_set(crate::plugin::AudioInit::Building))
            .add_systems(Update, building_audio.in_set(crate::plugin::AudioFrame::Building));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{Call, RecordingBackend};
    use crate::test_util::{app_with, build, frame};
    use glam::Vec3;

    fn setup(entry: Option<f32>) -> (App, RecordingBackend) {
        let (mut app, rec) = app_with(BuildingAudioPlugin);
        if let Some(size_param) = entry {
            app.world_mut().resource_mut::<BuildingTable>().0.insert(9, BuildingEntry { size_param });
        }
        (app, rec)
    }

    fn ev(kind: u8) -> PlayBuildingAudio {
        PlayBuildingAudio { id: 9, position: Vec3::X, editing_speed: 0.4, kind }
    }

    #[test]
    fn init_seven_loops() {
        let (mut app, rec) = build(BuildingAudioPlugin);
        app.update();
        assert_eq!(app.world().resource::<BuildingAudioState>().loops.len(), 7);
        assert_eq!(rec.calls().len(), 7);
        assert_eq!(BUILDING_LOOP_NAMES[0].len(), 0x22);
    }

    #[test]
    fn event_name_lengths() {
        for (n, l) in BUILDING_EVENT_KINDS {
            assert_eq!(n.len(), l);
        }
    }

    macro_rules! kind_test {
        ($($t:ident: $kind:expr => $name:expr),*) => {$(
            #[test]
            fn $t() {
                let (mut app, rec) = setup(Some(0.25));
                let calls = frame(&mut app, &rec, [ev($kind)]);
                assert_eq!(calls, vec![
                    Call::InstantiateAudio { name: $name.into(), id: 0 },
                    Call::SetPosition { id: 0, pos: Vec3::X },
                    Call::SetParameter { id: 0, name: "editing_speed".into(), value: 0.4 },
                    Call::SetParameter { id: 0, name: "BuildingSize".into(), value: 0.25 },
                    Call::Start { id: 0 },
                ]);
            }
        )*};
    }
    kind_test!(kind6_detach: 6 => "event:/SFX/Shapes/DetachFromGround",
               kind7_attach: 7 => "event:/SFX/Shapes/AttachToGround",
               kind8_peak: 8 => "event:/SFX/Shapes/PeakReached");

    #[test]
    fn other_kinds_skipped() {
        let (mut app, rec) = setup(None);
        let evs: Vec<_> = [0u8, 1, 2, 3, 4, 5, 9, 200].iter().map(|k| ev(*k)).collect();
        assert!(frame(&mut app, &rec, evs).is_empty());
    }

    #[test]
    fn missing_entry_omits_size() {
        let (mut app, rec) = setup(None);
        assert_eq!(frame(&mut app, &rec, [ev(6)]).len(), 4);
    }

    #[test]
    fn tail_table_is_consistent() {
        assert_eq!(BUILDING_TAIL_KIND_LOOP.len(), 7);
        assert_eq!(BUILDING_TAIL_PARAMS.len(), 7);
        // kind 9 -> PlasterChangeLoop, kind 5 -> OverlapMove
        assert_eq!(BUILDING_LOOP_NAMES[5], "event:/SFX/PlasterChangeLoop");
        assert_eq!(BUILDING_TAIL_KIND_LOOP[6], (9, 5));
        assert_eq!(BUILDING_LOOP_NAMES[BUILDING_TAIL_KIND_LOOP[5].1], "event:/SFX/Building/BuildingOverlapMove");
    }
}
