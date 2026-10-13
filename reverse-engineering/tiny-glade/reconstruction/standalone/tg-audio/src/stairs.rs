//! playermade_stairs_audio: init 0x140f878d0, system 0x140f880a0 (A §5, B §3a).
use crate::backend::{AudioBackendRes, LoopId};
use crate::events::PlayStairsAudioCmd;
use crate::wall::{LOOP_DESC_COUNT, LOOP_DESC_MAX};
use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;

/// 6 loops in init order (A §5a).
pub const STAIRS_LOOP_NAMES: [&str; 6] = [
    "event:/SFX/Stairs/StairMove",
    "event:/SFX/Stairs/StairChangeHeight",
    "event:/SFX/Stairs/StairChangeWidth",
    "event:/SFX/Stairs/LadderMove",
    "event:/SFX/Stairs/LadderChangeHeight",
    "event:/SFX/Stairs/LadderChangeWidth",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StairsAction {
    Loop(usize),
    /// name, doc length
    OneShot(&'static str, usize),
    /// kind 16: jump to loop top, no sound.
    Ignore,
}

use StairsAction::*;
/// kind 0..=18 (B §3a table). The `event:/SFX/Stairs/` prefix on one-shots is inferred
/// from the doc string lengths (all ten match exactly; see tests).
pub const STAIRS_KINDS: [(u8, u64, StairsAction); 19] = [
    (0, 0x140f88580, Loop(0)),
    (1, 0x140f88395, Loop(1)),
    (2, 0x140f8837e, Loop(2)),
    (3, 0x140f88354, OneShot("event:/SFX/Stairs/StairPlace", 0x1c)),
    (4, 0x140f8839e, OneShot("event:/SFX/Stairs/StairDelete", 0x1d)),
    (5, 0x140f883ba, OneShot("event:/SFX/Stairs/StairsMerge", 0x1d)),
    (6, 0x140f88387, OneShot("event:/SFX/Stairs/StairsMergeConfirm", 0x24)),
    (7, 0x140f88579, Loop(3)),
    (8, 0x140f88370, OneShot("event:/SFX/Stairs/LadderAppear", 0x1e)),
    (9, 0x140f88573, Loop(4)),
    (10, 0x140f8856d, Loop(5)),
    (11, 0x140f88346, OneShot("event:/SFX/Stairs/LadderPlace", 0x1d)),
    (12, 0x140f88362, OneShot("event:/SFX/Stairs/StepsAppear", 0x1d)),
    (13, 0x140f883ac, OneShot("event:/SFX/Stairs/WalkwayAppear", 0x1f)),
    (14, 0x140f88324, OneShot("event:/SFX/Stairs/StairSupportAppear", 0x24)),
    (15, 0x140f88335, OneShot("event:/SFX/Stairs/StairPillarAppear", 0x23)),
    (16, 0x140f8825e, Ignore),
    (17, 0x140f88324, OneShot("event:/SFX/Stairs/StairSupportAppear", 0x24)),
    (18, 0x140f88324, OneShot("event:/SFX/Stairs/StairSupportAppear", 0x24)),
];

/// Parameter names present in the string table 0x142bad818 (B §3a). UNKNOWN(evidence: A §5b/B §3a):
/// which event/kind sets which and the value sources (xmm11 etc.), so none are emitted.
pub const STAIRS_PARAM_NAMES: [&str; 11] = [
    "stair_width", "size", "editing_speed", "extend_amount", "h_offset", "avg_arch_supports",
    "height_above_ground", "in_water", "stair_edge_length", "is_stub_preview", "move_all",
];

#[derive(Resource, Debug)]
pub struct StairsAudioState {
    pub loops: [LoopId; 6],
}

/// `stairs_audio_init_system` (Startup).
pub fn stairs_audio_init_system(mut commands: Commands, mut backend: ResMut<AudioBackendRes>) {
    let mut loops = [0; 6];
    for (i, n) in STAIRS_LOOP_NAMES.iter().enumerate() {
        loops[i] = backend.instantiate_loop(n, LOOP_DESC_COUNT, LOOP_DESC_MAX);
    }
    commands.insert_resource(StairsAudioState { loops });
}

/// `stairs_audio_system`: stop all 6 loops at frame start, then dispatch events.
pub fn stairs_audio_system(
    mut backend: ResMut<AudioBackendRes>,
    state: Res<StairsAudioState>,
    mut events: EventReader<PlayStairsAudioCmd>,
) {
    let b = backend.get();
    for l in state.loops {
        b.loop_stop(l);
    }
    for ev in events.read() {
        let Some((_, _, action)) = STAIRS_KINDS.get(ev.kind as usize) else { continue };
        match *action {
            Loop(i) => b.loop_start_if_not_playing(state.loops[i]),
            OneShot(name, _) => {
                let id = b.instantiate_audio(name);
                b.set_position(id, ev.position);
                b.start_if_not_playing(id);
            }
            Ignore => {}
        }
    }
}

/// Stairs part of the audio plugin.
pub struct StairsAudioPlugin;

impl Plugin for StairsAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayStairsAudioCmd>()
            .add_systems(Startup, stairs_audio_init_system.in_set(crate::plugin::AudioInit::Stairs))
            .add_systems(Update, stairs_audio_system.in_set(crate::plugin::AudioFrame::Stairs));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::Call;
    use crate::test_util::{app_with, build, frame};
    use glam::Vec3;

    fn run(kind: u8) -> Vec<Call> {
        let (mut app, rec) = app_with(StairsAudioPlugin);
        let ev = PlayStairsAudioCmd { kind, position: Vec3::new(4.0, 5.0, 6.0) };
        frame(&mut app, &rec, [ev])
    }

    #[test]
    fn init_six_loops() {
        let (mut app, rec) = build(StairsAudioPlugin);
        app.update();
        let inits = rec.calls().iter().filter(|c| matches!(c, Call::InstantiateLoop { .. })).count();
        assert_eq!(inits, 6);
        assert_eq!(app.world().resource::<StairsAudioState>().loops, [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn table_indexed_and_lengths_match() {
        for (i, (k, _, a)) in STAIRS_KINDS.iter().enumerate() {
            assert_eq!(*k as usize, i);
            if let OneShot(n, l) = a {
                assert_eq!(n.len(), *l, "kind {k}");
            }
        }
    }

    macro_rules! stairs_kind_test {
        ($($t:ident: $kind:expr),*) => {$(
            #[test]
            fn $t() {
                let calls = run($kind);
                assert_eq!(calls.iter().filter(|c| matches!(c, Call::LoopStop { .. })).count(), 6);
                match STAIRS_KINDS[$kind].2 {
                    Loop(i) => assert!(calls.contains(&Call::LoopStart { l: i as u32 })),
                    OneShot(n, _) => {
                        assert_eq!(calls[6], Call::InstantiateAudio { name: n.into(), id: 0 });
                        assert_eq!(calls[7], Call::SetPosition { id: 0, pos: Vec3::new(4.0, 5.0, 6.0) });
                        assert_eq!(calls[8], Call::Start { id: 0 });
                    }
                    Ignore => assert_eq!(calls.len(), 6),
                }
            }
        )*};
    }
    stairs_kind_test!(s0: 0, s1: 1, s2: 2, s3: 3, s4: 4, s5: 5, s6: 6, s7: 7, s8: 8, s9: 9, s10: 10,
        s11: 11, s12: 12, s13: 13, s14: 14, s15: 15, s16: 16, s17: 17, s18: 18);

    #[test]
    fn out_of_range_kind_skipped() {
        assert_eq!(run(19).len(), 6);
    }
}
