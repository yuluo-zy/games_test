//! B `decorator_audio.rs`: decorator (window/lantern/chimney/flag/clutter)
//! edit sounds — 16 loops driven by `PlayDecoratorSoundCmd`.
//!
//! O-level facts (chapter decorator_audio.rs.md, decomp-0x14145c6b0 /
//! 0x14145dc20, disasm-0x14145e9c0, rne-read): init creates 16 loops (decomp
//! names, incl. three `Lantern/Amb` and the `Cosmetics/Clutter_Rotate` /
//! `Cosmetics/MoveGround` pair). The system dispatches each event kind to a
//! **one-shot** (verified names: WindowPlace / WindowCombine /
//! WindowPlacePreview / WindowDelete / WindowRemove / WindowChangeSeed /
//! WindowHover / WindowSplit / Lantern/Delete / ChimneyBaseDisappear /
//! ChimneyDelete / Flags/ChangeSeed / Flags/Delete / Cosmetics/Clutter_Clone)
//! or to a **loop** slot (kinds 6/7/0xc/0xd/0x12/0x16/0x18/0x1c/0x1d/0x22/0x24),
//! collects distance records for kind 0xf, and frame-end drives the loops with
//! stop/start_if_not_playing pair toggling. The loop slot ↔ event-name
//! correspondence (native state +0x10/+0x38/+0x70/+0xb8/+0xf0/+0x128/+0x160/
//! +0x198/+0x1d0/+0x208/+0x240/+0x278/+0x2b0/+0x2d8/+0x310/+0x358) is 待对账.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::audio::roof_audio::instantiate_audio_w_position,
    utils::audio_utils::AudioLoop,
};
use crate::systems::audio::{PlayClutterSoundCmd, PlayDecoratorSoundCmd};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The 16 decorator loops (O: decomp-0x14145c6b0 string constants).
pub const DECORATOR_LOOP_EVENTS: [&str; 16] = [
    "event:/SFX/WindowDrag",
    "event:/SFX/DecoratorMoveWoodenFloorTEMP",
    "event:/SFX/Lantern/MoveWall",
    "event:/SFX/Lantern/MoveGround",
    "event:/SFX/Lantern/Rotate",
    "event:/SFX/Lantern/Amb",
    "event:/SFX/Lantern/Amb",
    "event:/SFX/Lantern/Amb",
    "event:/SFX/ChimneyChangeHeight",
    "event:/SFX/ChimneyDrag",
    "event:/SFX/Flags/Rotate",
    "event:/SFX/Flags/Drag",
    "event:/SFX/Flags/DragGround",
    "event:/SFX/Cosmetics/Clutter_Rotate",
    "event:/SFX/Cosmetics/MoveGround",
    "event:/SFX/Cosmetics/MoveBuilding",
];

/// DecoratorAudioState: the 16 loops + per-loop should-play flag.
#[derive(Resource)]
pub struct DecoratorAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 16],
    pub should_play: [bool; 16],
    pub positions: [Vec3; 16],
}

impl<E: EventInstance> DecoratorAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        DecoratorAudioState {
            loops: DECORATOR_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
            should_play: [false; 16],
            positions: [Vec3::ZERO; 16],
        }
    }
}

/// `decorator_audio_init_system` (0x14145c6b0).
pub fn decorator_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(DecoratorAudioState::<S::Event>::new(&core));
}

/// Decorator one-shot event by kind (O: disasm-0x14145dc20 string table;
/// each verified by the native string address + length. Kinds not listed are
/// either loop kinds (see [`kind_to_loop`]) or reserved).
pub fn decorator_one_shot(kind: u8) -> Option<&'static str> {
    Some(match kind {
        1 => "event:/SFX/WindowPlace",
        2 => "event:/SFX/WindowCombine",
        3 => "event:/SFX/WindowPlacePreview",
        4 => "event:/SFX/WindowDelete",
        5 => "event:/SFX/WindowRemove",
        8 => "event:/SFX/WindowChangeSeed",
        9 => "event:/SFX/WindowHover",
        10 => "event:/SFX/WindowSplit",
        // B 14145f174：只传世界坐标，不设置参数，也不在消费者重复限额。
        0x0e => "event:/SFX/Lantern/TurnOn",
        0x10 => "event:/SFX/Lantern/Delete",
        0x14 => "event:/SFX/ChimneyBaseDisappear",
        0x17 => "event:/SFX/ChimneyDelete",
        0x19 => "event:/SFX/Flags/ChangeSeed",
        0x1e => "event:/SFX/Flags/Delete",
        0x1f => "event:/SFX/Cosmetics/Clutter_Clone",
        0x21 => "event:/SFX/Cosmetics/Clutter_Pickup",
        0x25 => "event:/SFX/Cosmetics/Clutter_Detach",
        _ => return None,
    })
}

/// 原142c631a4的56项类型表；0x20删除、0x23出现共用此映射。
/// 逐项保留原类型映射，不按装饰物编号近似选择声音。
const CLUTTER_SOUND_KIND_BY_SELECTOR: [u8; 56] = [
    7, 9, 10, 12, 13, 2, 3, 11, 4, 0, 19, 19, 6, 23, 24, 7,
    20, 25, 25, 23, 21, 22, 7, 5, 5, 1, 1, 7, 14, 31, 17, 17,
    17, 17, 30, 28, 29, 26, 26, 26, 5, 1, 29, 23, 32, 33, 34, 35,
    36, 37, 2, 38, 31, 31, 5, 1,
];

/// Map a `PlayDecoratorSoundCmd` loop kind to a loop index. The native
/// 0x14145dc20 jump table assigns each loop kind a state slot at
/// +0x10/+0x38/+0x70/+0xb8/+0xf0/+0x128/+0x160/+0x198/+0x1d0/+0x208/+0x240/
/// +0x278/+0x2b0/+0x2d8/+0x310/+0x358; the exact slot ↔ event-name
/// correspondence requires the full DecoratorAudioState layout and is 待对账.
/// The assignment below is the best-effort mapping kept from the pre-verification
/// code (window→0, lantern→2..7, chimney→8, flags→10..12, clutter→13).
fn kind_to_loop(kind: u8) -> Option<usize> {
    // O partial: loop kinds only (the one-shot kinds are handled first).
    match kind {
        0xc | 0xd => Some(8),
        6 => Some(1),
        7 => Some(2),
        0x12 => Some(4),
        0x16 => Some(13),
        0x18 => Some(10),
        0x1c | 0x1d => Some(12),
        0x22 => Some(14),
        0x24 => Some(9),
        _ => None,
    }
}

/// `decorator_audio_system` (0x14145dc20): dispatch each event to the verified
/// one-shot (kind→event) or loop (kind→loop slot), drive the flagged loops at
/// frame end (O: distance-sorted stop/start_if_not_playing toggling; the
/// distance collection for kind 0xf and the 3-nearest drive are 待对账).
pub fn decorator_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<DecoratorAudioState<S::Event>>,
    mut events: EventReader<PlayDecoratorSoundCmd>,
    core: Res<AudioCore<S>>,
    mut clutter: EventWriter<PlayClutterSoundCmd>,
) {
    for event in events.read() {
        let position = [event.position.x, event.position.y, event.position.z];
        if matches!(event.kind, 0x20 | 0x23) {
            // 原14145e220..2b5：删除先播放通用声，再把材质类型写入
            // 真PlayClutterSoundCmd队列；出现仅写队列。创建失败不可先发后续事件。
            if event.kind == 0x20 {
                let _ = instantiate_audio_w_position(&core,
                    "event:/SFX/Cosmetics/Clutter_Delete", position);
            }
            let kind = CLUTTER_SOUND_KIND_BY_SELECTOR[usize::from(event.selector)];
            // 原公共写入块14145e163先INC，再写context：出现1，删除2。
            clutter.write(PlayClutterSoundCmd { kind, position: event.position,
                context: u8::from(event.kind == 0x20) + 1 });
            continue;
        }
        if let Some(name) = decorator_one_shot(event.kind) {
            // O: one-shot instantiate → set_position → start_if_not_playing.
            let _ = instantiate_audio_w_position(&core, name, position);
            continue;
        }
        let Some(index) = kind_to_loop(event.kind) else {
            continue;
        };
        state.should_play[index] = true;
        state.positions[index] = event.position;
    }

    let should_play = std::mem::take(&mut state.should_play);
    let positions = std::mem::take(&mut state.positions);
    for (index, loop_player) in state.loops.iter_mut().enumerate() {
        if should_play[index] {
            let _ = loop_player.start_if_not_playing();
            let current = &mut loop_player.entries[loop_player.cursor];
            let pos = positions[index];
            let _ = current.set_position([pos.x, pos.y, pos.z]);
        } else {
            let _ = loop_player.stop_now();
        }
    }
    state.should_play = should_play;
    state.positions = positions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decorator_loop_count_and_first_names_are_original() {
        assert_eq!(DECORATOR_LOOP_EVENTS.len(), 16);
        assert_eq!(DECORATOR_LOOP_EVENTS[0], "event:/SFX/WindowDrag");
        assert_eq!(DECORATOR_LOOP_EVENTS[13], "event:/SFX/Cosmetics/Clutter_Rotate");
        assert_eq!(DECORATOR_LOOP_EVENTS[14], "event:/SFX/Cosmetics/MoveGround");
    }

    #[test]
    fn decorator_one_shots_match_the_dispatch_string_table() {
        assert_eq!(decorator_one_shot(1), Some("event:/SFX/WindowPlace"));
        assert_eq!(decorator_one_shot(2), Some("event:/SFX/WindowCombine"));
        assert_eq!(decorator_one_shot(4), Some("event:/SFX/WindowDelete"));
        assert_eq!(decorator_one_shot(8), Some("event:/SFX/WindowChangeSeed"));
        assert_eq!(decorator_one_shot(0x0e), Some("event:/SFX/Lantern/TurnOn"));
        assert_eq!(decorator_one_shot(0x17), Some("event:/SFX/ChimneyDelete"));
        assert_eq!(decorator_one_shot(0x19), Some("event:/SFX/Flags/ChangeSeed"));
        assert_eq!(decorator_one_shot(0x1e), Some("event:/SFX/Flags/Delete"));
    }
}

#[cfg(test)]
mod lantern_turn_on_tests {
    use super::*;
    use crate::resources::{
        audio_instance::PlaybackState,
        audio_resources::{ListenerAttributes, VcaHandle},
    };
    use std::sync::{Arc, Mutex};

    const TURN_ON: &str = "event:/SFX/Lantern/TurnOn";

    #[derive(Clone, Debug, PartialEq)]
    enum Call {
        Create,
        CreateNamed(String),
        Position { position: [u32; 3], forward: [u32; 3], up: [u32; 3], velocity: [u32; 3] },
        Playback,
        Start,
        Stop,
        Release,
        Volume,
        Parameter,
    }
    #[derive(Clone, Copy, Debug, Default)]
    enum Failure {
        #[default]
        None,
        Create,
        Position,
        Playback,
        Start,
        Release,
    }
    #[derive(Clone, Debug)]
    struct RecordingStudio {
        calls: Arc<Mutex<Vec<Call>>>,
        playback: PlaybackState,
        failure: Failure,
    }
    impl RecordingStudio {
        fn new(playback: PlaybackState, failure: Failure) -> Self {
            Self { calls: Arc::default(), playback, failure }
        }
        fn record(&self, call: Call) { self.calls.lock().unwrap().push(call); }
    }
    #[derive(Debug)]
    struct RecordingEvent { studio: RecordingStudio, is_turn_on: bool }
    impl RecordingEvent {
        fn record(&self, call: Call) { if self.is_turn_on { self.studio.record(call); } }
        fn fail(&self, expected: fn(Failure) -> bool) -> Result<(), String> {
            if self.is_turn_on && expected(self.studio.failure) { Err("recorded SDK failure".into()) } else { Ok(()) }
        }
    }
    impl EventInstance for RecordingEvent {
        type Error = String;
        fn start(&mut self) -> Result<(), String> {
            self.record(Call::Start); self.fail(|f| matches!(f, Failure::Start))
        }
        fn stop(&mut self, _: i32) -> Result<(), String> { self.record(Call::Stop); Ok(()) }
        fn get_playback_state(&self) -> Result<PlaybackState, String> {
            self.record(Call::Playback);
            self.fail(|f| matches!(f, Failure::Playback))?;
            Ok(self.studio.playback)
        }
        fn set_volume(&mut self, _: f32) -> Result<(), String> { self.record(Call::Volume); Ok(()) }
        fn set_parameter_by_name(&mut self, _: &str, _: f32) -> Result<(), String> {
            self.record(Call::Parameter); Ok(())
        }
        fn has_parameter(&self, _: &str) -> Result<bool, String> { Ok(true) }
        fn set_3d_attributes(&mut self, p: [f32; 3], f: [f32; 3], u: [f32; 3], v: [f32; 3]) -> Result<(), String> {
            self.record(Call::Position { position: p.map(f32::to_bits), forward: f.map(f32::to_bits), up: u.map(f32::to_bits), velocity: v.map(f32::to_bits) });
            self.fail(|f| matches!(f, Failure::Position))
        }
        fn get_timeline_position(&self) -> Result<i32, String> { Ok(0) }
        fn pause(&mut self, _: bool) -> Result<(), String> { Ok(()) }
        fn install_beat_callback(&mut self) -> Result<(), String> { Ok(()) }
        fn release(&mut self) -> Result<(), String> {
            self.record(Call::Release); self.fail(|f| matches!(f, Failure::Release))
        }
    }
    struct RecordingVca;
    impl VcaHandle for RecordingVca {
        type Error = String;
        fn set_volume(&self, _: f32) -> Result<(), String> { Ok(()) }
    }
    impl Studio for RecordingStudio {
        type Error = String;
        type Event = RecordingEvent;
        type Vca = RecordingVca;
        fn load_bank_file(&mut self, _: &str) -> Result<(), String> { Ok(()) }
        fn load_sample_data(&mut self) -> Result<(), String> { Ok(()) }
        fn get_vca(&self, _: &str) -> Result<RecordingVca, String> { Ok(RecordingVca) }
        fn set_listener_attributes(&mut self, _: ListenerAttributes) -> Result<(), String> { Ok(()) }
        fn update(&mut self) -> Result<(), String> { Ok(()) }
        fn set_parameter_by_name(&self, _: &str, _: f32) -> Result<(), String> { Ok(()) }
        fn create_event_instance(&self, name: &str) -> Result<Option<RecordingEvent>, String> {
            let is_turn_on = matches!(name, TURN_ON
                | "event:/SFX/Cosmetics/Clutter_Delete"
                | "event:/SFX/Cosmetics/Clutter_Detach"
                | "event:/SFX/Cosmetics/Clutter_Pickup");
            if is_turn_on {
                self.record(if name == TURN_ON { Call::Create } else { Call::CreateNamed(name.to_owned()) });
                if matches!(self.failure, Failure::Create) { return Err("recorded creation failure".into()); }
            }
            Ok(Some(RecordingEvent { studio: self.clone(), is_turn_on }))
        }
    }
    fn world(studio: Option<RecordingStudio>) -> (World, Schedule) {
        let core = AudioCore { studio, banks: Default::default() };
        let state = DecoratorAudioState::new(&core);
        let mut world = World::new();
        world.insert_resource(core);
        world.insert_resource(state);
        world.init_resource::<Events<PlayDecoratorSoundCmd>>();
        world.init_resource::<Events<PlayClutterSoundCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(decorator_audio_system::<RecordingStudio>);
        (world, schedule)
    }
    fn position_call(position: Vec3) -> Call {
        Call::Position {
            position: position.to_array().map(f32::to_bits),
            forward: [-0.0f32, -0.0, -1.0].map(f32::to_bits),
            up: [0.0f32, 1.0, 0.0].map(f32::to_bits),
            velocity: [0; 3],
        }
    }

    #[test]
    fn original_clutter_conversion_dispatch_preserves_selector_position_and_queue() {
        mod original {
            include!(concat!(env!("CARGO_MANIFEST_DIR"),
                "/../../reports/module-recovery-2026-10-23/m01-clutter-conversion-audio-native-v2.rs"));
            pub(super) fn cases() -> &'static [(u8,u8,[u32;3],Option<(u8,u8)>,Option<&'static str>)] {
                ORIGINAL_CONVERSION_AUDIO
            }
        }
        for &(kind, selector, position, forward, name) in original::cases() {
            let studio = RecordingStudio::new(PlaybackState::Stopped, Failure::None);
            let calls = studio.calls.clone();
            let (mut world, mut schedule) = world(Some(studio));
            let position = Vec3::from_array(position.map(f32::from_bits));
            let mut command = PlayDecoratorSoundCmd::at(kind, position);
            command.selector = selector;
            world.send_event(command);
            schedule.run(&mut world);
            let forwarded: Vec<_> = world.resource_mut::<Events<PlayClutterSoundCmd>>().drain().collect();
            match forward {
                Some((expected_kind, expected_context)) => {
                    assert_eq!(forwarded.len(), 1);
                    assert_eq!(forwarded[0].kind, expected_kind);
                    assert_eq!(forwarded[0].context, expected_context);
                    assert_eq!(forwarded[0].position.to_array().map(f32::to_bits), position.to_array().map(f32::to_bits));
                }
                None => assert!(forwarded.is_empty()),
            }
            let expected_calls = match name {
                Some(name) => vec![Call::CreateNamed(name.to_owned()), position_call(position),
                    Call::Playback, Call::Start, Call::Release],
                None => vec![],
            };
            assert_eq!(*calls.lock().unwrap(), expected_calls, "kind={kind} selector={selector}");
            schedule.run(&mut world);
            assert!(world.resource::<Events<PlayClutterSoundCmd>>().is_empty());
            assert_eq!(*calls.lock().unwrap(), expected_calls, "下一帧不能重放");
        }
        assert_eq!(original::cases().len(), 236);
    }

    #[test]
    fn failed_delete_sound_does_not_emit_material_event_before_native_fatal_boundary() {
        let studio = RecordingStudio::new(PlaybackState::Stopped, Failure::Create);
        let (mut world, mut schedule) = world(Some(studio));
        world.send_event(PlayDecoratorSoundCmd::at(0x20, Vec3::ZERO));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
        assert!(result.is_err());
        assert!(world.resource::<Events<PlayClutterSoundCmd>>().is_empty());
    }
    #[test]
    fn real_consumer_preserves_world_position_and_one_shot_ownership_across_frames() {
        let studio = RecordingStudio::new(PlaybackState::Stopped, Failure::None);
        let calls = studio.calls.clone();
        let (mut world, mut schedule) = world(Some(studio));
        let positions = [Vec3::new(-0.0, 27.5, -3.25), Vec3::new(1234.5, -9.5, 0.0)];
        for p in positions { world.send_event(PlayDecoratorSoundCmd::at(14, p)); }
        schedule.run(&mut world);
        let expected: Vec<_> = positions.into_iter().flat_map(|p| [Call::Create, position_call(p), Call::Playback, Call::Start, Call::Release]).collect();
        assert_eq!(*calls.lock().unwrap(), expected);
        schedule.run(&mut world);
        assert_eq!(*calls.lock().unwrap(), expected, "下一帧不能重放已消费的点亮事件");
        world.send_event(PlayDecoratorSoundCmd::at(14, positions[0]));
        schedule.run(&mut world);
        assert_eq!(calls.lock().unwrap().len(), expected.len() + 5);
    }
    #[test]
    fn consumer_does_not_repeat_the_two_producer_passes_limit_or_deduplicate_positions() {
        let studio = RecordingStudio::new(PlaybackState::Stopped, Failure::None);
        let calls = studio.calls.clone();
        let (mut world, mut schedule) = world(Some(studio));
        // 原生产者各 pass 至多四条；音频消费者自身没有八条上限，也没有位置去重。
        for _ in 0..12 { world.send_event(PlayDecoratorSoundCmd::at(14, Vec3::ONE)); }
        schedule.run(&mut world);
        assert_eq!(calls.lock().unwrap().iter().filter(|c| **c == Call::Create).count(), 12);
        assert_eq!(calls.lock().unwrap().iter().filter(|c| **c == Call::Release).count(), 12);
    }
    #[test]
    fn turn_on_respects_all_five_original_playback_states() {
        for playback in [PlaybackState::Playing, PlaybackState::Sustaining, PlaybackState::Stopped, PlaybackState::Starting, PlaybackState::Stopping] {
            let studio = RecordingStudio::new(playback, Failure::None);
            let calls = studio.calls.clone();
            let (mut world, mut schedule) = world(Some(studio));
            world.send_event(PlayDecoratorSoundCmd::at(14, Vec3::ZERO));
            schedule.run(&mut world);
            let mut expected = vec![Call::Create, position_call(Vec3::ZERO), Call::Playback];
            if matches!(playback, PlaybackState::Stopped | PlaybackState::Stopping) { expected.push(Call::Start); }
            expected.push(Call::Release);
            assert_eq!(*calls.lock().unwrap(), expected, "{playback:?}");
        }
    }
    #[test]
    fn original_fatal_and_recoverable_sdk_failures_keep_their_order() {
        for failure in [Failure::Create, Failure::Position, Failure::Playback, Failure::Start, Failure::Release] {
            let studio = RecordingStudio::new(PlaybackState::Stopped, failure);
            let calls = studio.calls.clone();
            let (mut world, mut schedule) = world(Some(studio));
            world.send_event(PlayDecoratorSoundCmd::at(14, Vec3::ONE));
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
            let fatal = matches!(failure, Failure::Create | Failure::Playback | Failure::Start | Failure::Release);
            assert_eq!(result.is_err(), fatal, "{failure:?}");
            let expected = match failure {
                Failure::Create => vec![Call::Create],
                Failure::Playback => vec![Call::Create, position_call(Vec3::ONE), Call::Playback, Call::Release],
                _ => vec![Call::Create, position_call(Vec3::ONE), Call::Playback, Call::Start, Call::Release],
            };
            assert_eq!(*calls.lock().unwrap(), expected, "{failure:?}");
        }
    }
    #[test]
    fn disconnected_studio_consumes_turn_on_without_creating_a_backend() {
        let (mut world, mut schedule) = world(None);
        world.send_event(PlayDecoratorSoundCmd::at(14, Vec3::ONE));
        schedule.run(&mut world);
        schedule.run(&mut world);
        assert!(!world.resource::<AudioCore<RecordingStudio>>().is_connected());
    }

    #[test]
    fn original_instruction_requests_and_failure_gates_match_the_canonical_helper() {
        let evidence: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../reports/module-recovery-2026-10-23/m01-lantern-audio-native.json"
        ))).unwrap();
        assert_eq!(evidence["binary_sha256"], "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03");
        for row in evidence["lifecycle"].as_array().unwrap() {
            let playback = match row["playback"].as_u64().unwrap() {
                0 => PlaybackState::Playing,
                1 => PlaybackState::Sustaining,
                2 => PlaybackState::Stopped,
                3 => PlaybackState::Starting,
                4 => PlaybackState::Stopping,
                _ => unreachable!(),
            };
            let failure = match row["failure"].as_str() {
                None => Failure::None,
                Some("create") => Failure::Create,
                Some("playback") => Failure::Playback,
                Some("start") => Failure::Start,
                Some("release") => Failure::Release,
                _ => unreachable!(),
            };
            let p: [f32; 3] = std::array::from_fn(|i| f32::from_bits(row["input_position_bits"][i].as_u64().unwrap() as u32));
            let studio = RecordingStudio::new(playback, failure);
            let calls = studio.calls.clone();
            let core = AudioCore { studio: Some(studio), banks: Default::default() };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                drop(instantiate_audio_w_position(&core, TURN_ON, p));
            }));
            assert_eq!(result.is_err(), !row["panic"].is_null(), "{row}");
            let expected: Vec<_> = row["calls"].as_array().unwrap().iter().map(|call| {
                match call["call"].as_str().unwrap() {
                    "create" => { assert_eq!(call["name"], TURN_ON); Call::Create },
                    "set_3d_attributes" => {
                        let bits = |field: &str| std::array::from_fn(|i| call[field][i].as_u64().unwrap() as u32);
                        Call::Position { position: bits("position_bits"), forward: bits("forward_bits"), up: bits("up_bits"), velocity: bits("velocity_bits") }
                    },
                    "playback" => Call::Playback,
                    "start" => Call::Start,
                    "release" => Call::Release,
                    _ => panic!("unexpected native request: {call}"),
                }
            }).collect();
            let observed = calls.lock().unwrap();
            // 指令探针在 unwrap 入口终止，不伪造 Windows unwind；Rust 真实展开再释放一次句柄。
            assert_eq!(&observed[..expected.len()], expected, "{row}");
            let suffix = &observed[expected.len()..];
            if result.is_err() && matches!(failure, Failure::Playback | Failure::Start) {
                assert_eq!(suffix, [Call::Release]);
            } else {
                assert!(suffix.is_empty(), "{row}");
            }
        }
    }
}
