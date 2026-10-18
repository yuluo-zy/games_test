//! B terrain audio; original table0x142c63274 and six executed one-shot
//! probes correct the old swapped tag4/5 parameter interpretation. Tag4 LowPoint
//! sets smoothness+editing_speed; tag5 CreateChunk sets terrain_height from its
//! single raw/unclamped height payload. Reuse existing AudioCore/AudioInstance.
use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::audio::audio_update::EditingSpeed,
    systems::terrain::PlayTerrainSound,
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The four terrain edit loops (O: rne-read; slot order 0..3).
pub const TERRAIN_LOOP_EVENTS: [&str; 4] = [
    "event:/SFX/TerrainEditShape",
    "event:/SFX/TerrainEditHeight",
    "event:/SFX/Terrain Draw",
    "event:/SFX/TerrainMove",
];

/// One-shots (O: 0x142c62d58 / 0x142c62d90).
pub const TERRAIN_LOW_POINT_EVENT: &str = "event:/SFX/TerrainLowPoint";
pub const TERRAIN_CREATE_CHUNK_EVENT: &str = "event:/SFX/TerrainCreateChunk";

/// Instance parameters (O: 0x142c62d38 / 0x142c62d4a / 0x142c62943).
pub const TERRAIN_SMOOTHNESS_PARAM: &str = "terrain_smoothness";
pub const TERRAIN_HEIGHT_PARAM: &str = "terrain_height";
pub const TERRAIN_EDITING_SPEED_PARAM: &str = "editing_speed";

/// Frame-end drive order (O: decomp 0x1414622de..0x1414624f0).
pub const TERRAIN_DRIVE_ORDER: [usize; 4] = [2, 1, 0, 3];

/// TerrainAudioState: the four loops, per-slot should-play flags, the shared
/// last event position and per-slot editing-speed values.
#[derive(Resource)]
pub struct TerrainAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 4],
    pub should_play: [bool; 4],
    /// O: every event overwrites the shared frame position used by the
    /// frame-end `set_position`.
    pub position: Vec3,
    /// O: per-slot `editing_speed` values (native xmm7/xmm9/xmm8/xmm6).
    pub speed: [f32; 4],
}

impl<E: EventInstance> TerrainAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        TerrainAudioState {
            loops: TERRAIN_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
            should_play: [false; 4],
            position: Vec3::ZERO,
            speed: [0.0; 4],
        }
    }
}

/// `terrain_audio_init_system` (0x1414613e0).
pub fn terrain_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(TerrainAudioState::<S::Event>::new(&core));
}

/// Initialize the existing loop owner once and consume this frame's terrain
/// edits after their CPU producers. Chaining also commits the initial Commands
/// before the consumer first reads TerrainAudioState.
pub fn add_terrain_audio_systems<S: Studio + Send + Sync + 'static>(schedule: &mut Schedule) {
    use bevy_ecs::schedule::common_conditions::{not, resource_exists};
    schedule.add_systems(
        (
            terrain_audio_init_system::<S>
                .run_if(not(resource_exists::<TerrainAudioState<S::Event>>)),
            terrain_audio_system::<S>,
        )
            .chain()
            .after(crate::systems::terrain::TerrainCpuStage::Changed),
    );
}

/// Native `inverse_lerp` (0x140c904a0) — unclamped.
fn inverse_lerp(min: f32, max: f32, value: f32) -> f32 {
    (value - min) / (max - min)
}

fn clamp01(value: f32) -> f32 {
    // Native MAXSS/MINSS retain NaN and the input signed zero. The f32
    // min/max methods ignore NaN and canonicalize opposite signed zeros.
    value.clamp(0.0, 1.0)
}

/// `terrain_audio_system` (0x1414618c0).
pub fn terrain_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<TerrainAudioState<S::Event>>,
    mut events: EventReader<PlayTerrainSound>,
    core: Res<AudioCore<S>>,
    speed: Res<EditingSpeed>,
) {
    let editing_speed = speed.0;
    for sound in events.read() {
        let tag = sound.native_tag();
        let (position, value0, value1, value2) = match sound {
            PlayTerrainSound::StrokeChanged => (None, 0.0, 0.0, 0.0),
            PlayTerrainSound::Falloff {
                position,
                falloff,
                magnitude,
            } => (Some(*position), *falloff, *magnitude, 0.0),
            PlayTerrainSound::Height {
                position,
                falloff,
                height,
                magnitude,
            } => (Some(*position), *falloff, *height, *magnitude),
            PlayTerrainSound::Points {
                position,
                falloff,
                height,
                magnitude,
            } => (Some(*position), *falloff, *height, *magnitude),
            PlayTerrainSound::Removed {
                position,
                falloff,
                magnitude,
            } => (Some(*position), *falloff, *magnitude, 0.0),
            PlayTerrainSound::CreateChunk { position, height } => {
                (Some(*position), *height, 0.0, 0.0)
            }
        };

        let set_param =
            |state: &mut TerrainAudioState<S::Event>, slot: usize, name: &str, value: f32| {
                let cursor = state.loops[slot].cursor;
                let _ = state.loops[slot].entries[cursor].set_parameter(name, value);
            };

        match tag {
            // 0 — Terrain Draw loop: only the flag (no position/parameters).
            0 => {
                state.should_play[2] = true;
            }
            // 1 — TerrainEditShape (slot 0).
            1 => {
                state.should_play[0] = true;
                state.position = position.unwrap_or(Vec3::ZERO);
                state.speed[0] = clamp01(inverse_lerp(0.0, 5.0, value1 / editing_speed));
                set_param(&mut state, 0, TERRAIN_SMOOTHNESS_PARAM, value0);
            }
            // 2 — TerrainEditHeight (slot 1).
            2 => {
                state.should_play[1] = true;
                state.position = position.unwrap_or(Vec3::ZERO);
                state.speed[1] = clamp01(inverse_lerp(0.0, 5.0, value2 / editing_speed));
                set_param(&mut state, 1, TERRAIN_SMOOTHNESS_PARAM, value0);
                set_param(
                    &mut state,
                    1,
                    TERRAIN_HEIGHT_PARAM,
                    inverse_lerp(0.0, 20.0, value1),
                );
            }
            // 3 — TerrainMove (slot 3).
            3 => {
                state.should_play[3] = true;
                state.position = position.unwrap_or(Vec3::ZERO);
                state.speed[3] = clamp01(inverse_lerp(0.0, 5.0, value2 / editing_speed));
                set_param(&mut state, 3, TERRAIN_SMOOTHNESS_PARAM, value0);
                set_param(
                    &mut state,
                    3,
                    TERRAIN_HEIGHT_PARAM,
                    inverse_lerp(0.0, 20.0, value1),
                );
            }
            // 4 — original0x141461d4d ->0x141461f10.
            4 => {
                let position = position.unwrap_or(Vec3::ZERO);
                let mut instance = core.instantiate_audio(TERRAIN_LOW_POINT_EVENT);
                let _ = instance.set_position(position.to_array());
                let _ = instance.start_if_not_playing();
                let _ = instance.set_parameter(TERRAIN_SMOOTHNESS_PARAM, value0);
                let _ = instance.set_parameter(
                    TERRAIN_EDITING_SPEED_PARAM,
                    clamp01(inverse_lerp(0.0, 5.0, value1 / editing_speed)),
                );
            }
            // 5 — original0x141461d90 ->0x141461e16.
            5 => {
                let position = position.unwrap_or(Vec3::ZERO);
                let mut instance = core.instantiate_audio(TERRAIN_CREATE_CHUNK_EVENT);
                let _ = instance.set_position(position.to_array());
                let _ = instance.start_if_not_playing();
                let _ =
                    instance.set_parameter(TERRAIN_HEIGHT_PARAM, inverse_lerp(0.0, 20.0, value0));
            }
            _ => unreachable!("typed terrain audio tag"),
        }
    }

    // O: frame-end drive order [2, 1, 0, 3].
    for slot in TERRAIN_DRIVE_ORDER {
        let play = state.should_play[slot];
        let speed = state.speed[slot];
        let position = state.position;
        let loop_player = &mut state.loops[slot];
        if play {
            let _ = loop_player.start_if_not_playing();
            let cursor = loop_player.cursor;
            let _ = loop_player.entries[cursor].set_parameter(TERRAIN_EDITING_SPEED_PARAM, speed);
            let _ = loop_player.entries[cursor].set_position([position.x, position.y, position.z]);
        } else {
            let _ = loop_player.stop(std::time::Duration::ZERO);
        }
        state.should_play[slot] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terrain_loop_names_are_original() {
        assert_eq!(TERRAIN_LOOP_EVENTS[0], "event:/SFX/TerrainEditShape");
        assert_eq!(TERRAIN_LOOP_EVENTS[1], "event:/SFX/TerrainEditHeight");
        assert_eq!(TERRAIN_LOOP_EVENTS[2], "event:/SFX/Terrain Draw");
        assert_eq!(TERRAIN_LOOP_EVENTS[3], "event:/SFX/TerrainMove");
    }

    #[test]
    fn terrain_drive_order_and_names_match_the_decomp() {
        assert_eq!(TERRAIN_DRIVE_ORDER, [2, 1, 0, 3]);
        assert_eq!(TERRAIN_SMOOTHNESS_PARAM, "terrain_smoothness");
        assert_eq!(TERRAIN_HEIGHT_PARAM, "terrain_height");
        assert_eq!(TERRAIN_LOW_POINT_EVENT, "event:/SFX/TerrainLowPoint");
        assert_eq!(TERRAIN_CREATE_CHUNK_EVENT, "event:/SFX/TerrainCreateChunk");
    }
}

#[cfg(test)]
mod original_one_shot_consumer_tests {
    use super::*;
    use crate::resources::{
        audio_instance::PlaybackState,
        audio_resources::{ListenerAttributes, VcaHandle},
    };
    use std::sync::{Arc, Mutex};
    #[derive(Clone, Debug, PartialEq)]
    enum Call {
        Create(String),
        Position(String, [u32; 3]),
        Start(String),
        Parameter(String, String, u32),
    }
    type Log = Arc<Mutex<Vec<Call>>>;
    #[derive(Debug)]
    struct RecordingEvent {
        name: String,
        log: Log,
    }
    impl EventInstance for RecordingEvent {
        type Error = String;
        fn start(&mut self) -> Result<(), String> {
            self.log
                .lock()
                .unwrap()
                .push(Call::Start(self.name.clone()));
            Ok(())
        }
        fn stop(&mut self, _: i32) -> Result<(), String> {
            Ok(())
        }
        fn get_playback_state(&self) -> Result<PlaybackState, String> {
            Ok(PlaybackState::Stopped)
        }
        fn set_volume(&mut self, _: f32) -> Result<(), String> {
            Ok(())
        }
        fn set_parameter_by_name(&mut self, n: &str, v: f32) -> Result<(), String> {
            self.log.lock().unwrap().push(Call::Parameter(
                self.name.clone(),
                n.into(),
                v.to_bits(),
            ));
            Ok(())
        }
        fn has_parameter(&self, _: &str) -> Result<bool, String> {
            Ok(true)
        }
        fn set_3d_attributes(
            &mut self,
            p: [f32; 3],
            _: [f32; 3],
            _: [f32; 3],
            _: [f32; 3],
        ) -> Result<(), String> {
            self.log
                .lock()
                .unwrap()
                .push(Call::Position(self.name.clone(), p.map(f32::to_bits)));
            Ok(())
        }
        fn get_timeline_position(&self) -> Result<i32, String> {
            Ok(0)
        }
        fn pause(&mut self, _: bool) -> Result<(), String> {
            Ok(())
        }
        fn install_beat_callback(&mut self) -> Result<(), String> {
            Ok(())
        }
    }
    struct RecordingVca;
    impl VcaHandle for RecordingVca {
        type Error = String;
        fn set_volume(&self, _: f32) -> Result<(), String> {
            Ok(())
        }
    }
    struct RecordingStudio(Log);
    impl Studio for RecordingStudio {
        type Error = String;
        type Event = RecordingEvent;
        type Vca = RecordingVca;
        fn load_bank_file(&mut self, _: &str) -> Result<(), String> {
            Ok(())
        }
        fn load_sample_data(&mut self) -> Result<(), String> {
            Ok(())
        }
        fn get_vca(&self, _: &str) -> Result<RecordingVca, String> {
            Ok(RecordingVca)
        }
        fn set_listener_attributes(&mut self, _: ListenerAttributes) -> Result<(), String> {
            Ok(())
        }
        fn update(&mut self) -> Result<(), String> {
            Ok(())
        }
        fn set_parameter_by_name(&self, _: &str, _: f32) -> Result<(), String> {
            Ok(())
        }
        fn create_event_instance(&self, n: &str) -> Result<Option<RecordingEvent>, String> {
            self.0.lock().unwrap().push(Call::Create(n.into()));
            Ok(Some(RecordingEvent {
                name: n.into(),
                log: self.0.clone(),
            }))
        }
    }
    fn observe(sound: PlayTerrainSound, speed: f32, name: &str) -> Vec<Call> {
        let log = Log::default();
        let core = AudioCore {
            studio: Some(RecordingStudio(log.clone())),
            banks: Default::default(),
        };
        let state = TerrainAudioState::new(&core);
        log.lock().unwrap().clear();
        let mut world = World::new();
        world.insert_resource(core);
        world.insert_resource(state);
        world.insert_resource(EditingSpeed(speed));
        world.init_resource::<Events<PlayTerrainSound>>();
        world.send_event(sound);
        let mut schedule = Schedule::default();
        schedule.add_systems(terrain_audio_system::<RecordingStudio>);
        schedule.run(&mut world);
        let out = log
            .lock()
            .unwrap()
            .iter()
            .filter(|call| match call {
                Call::Create(n)
                | Call::Position(n, _)
                | Call::Start(n)
                | Call::Parameter(n, _, _) => n == name,
            })
            .cloned()
            .collect();
        out
    }

    #[test]
    fn registered_audio_keeps_its_loop_owner_and_consumes_this_frame_producers() {
        fn produce(mut sound: EventWriter<PlayTerrainSound>) {
            sound.write(PlayTerrainSound::CreateChunk {
                position: Vec3::new(1.0, 2.0, 3.0),
                height: 26.5,
            });
        }
        let log = Log::default();
        let mut world = World::new();
        world.insert_resource(AudioCore {
            studio: Some(RecordingStudio(log.clone())),
            banks: Default::default(),
        });
        world.insert_resource(EditingSpeed(1.0));
        world.init_resource::<Events<PlayTerrainSound>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(produce.in_set(crate::systems::terrain::TerrainCpuStage::Changed));
        add_terrain_audio_systems::<RecordingStudio>(&mut schedule);
        schedule.run(&mut world);
        assert!(log.lock().unwrap().iter().any(|call| {
            matches!(call, Call::Parameter(name, parameter, bits)
                if name == TERRAIN_CREATE_CHUNK_EVENT
                    && parameter == TERRAIN_HEIGHT_PARAM
                    && *bits == (26.5_f32 / 20.0).to_bits())
        }));
        world
            .resource_mut::<TerrainAudioState<RecordingEvent>>()
            .speed[0] = 7.25;
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<TerrainAudioState<RecordingEvent>>().speed[0],
            7.25
        );
        for name in TERRAIN_LOOP_EVENTS {
            assert_eq!(
                log.lock()
                    .unwrap()
                    .iter()
                    .filter(|call| { matches!(call, Call::Create(created) if created == name) })
                    .count(),
                crate::resources::audio_resources::AudioLoopDesc::new(name).count,
                "loop owner must survive subsequent frames: {name}"
            );
        }
    }
    #[test]
    fn low_point_speed_preserves_native_nan_and_signed_zero() {
        // Original B tag4 branch replay: one-shot-boundaries.json. NaN sign
        // and payload from arithmetic depend on the host CPU, so compare
        // NaN semantically; finite and signed-zero results compare by bits.
        for (magnitude, editing_speed, expected) in [
            (0.0, 0.0, None),
            (f32::NAN, 1.0, None),
            (0.1, 0.0, Some(1.0_f32.to_bits())),
            (-0.1, 0.0, Some(0.0_f32.to_bits())),
            (-0.0, 1.0, Some((-0.0_f32).to_bits())),
        ] {
            let calls = observe(
                PlayTerrainSound::Removed {
                    position: Vec3::new(3.0, -1.0, 4.0),
                    falloff: 0.4,
                    magnitude,
                },
                editing_speed,
                TERRAIN_LOW_POINT_EVENT,
            );
            let bits = calls
                .iter()
                .find_map(|call| match call {
                    Call::Parameter(name, parameter, bits)
                        if name == TERRAIN_LOW_POINT_EVENT
                            && parameter == TERRAIN_EDITING_SPEED_PARAM =>
                    {
                        Some(*bits)
                    }
                    _ => None,
                })
                .expect("LowPoint must forward its editing_speed parameter");
            match expected {
                Some(expected) => assert_eq!(bits, expected),
                None => assert!(f32::from_bits(bits).is_nan()),
            }
        }
    }

    #[test]
    fn original_one_shot_0() {
        let position = Vec3::new(3., -1., 4.);
        let event = PlayTerrainSound::Removed {
            position,
            falloff: f32::from_bits(1053609165),
            magnitude: f32::from_bits(1075838976),
        };
        let observed = observe(
            event,
            f32::from_bits(1065353216),
            "event:/SFX/TerrainLowPoint",
        );
        assert_eq!(
            observed,
            vec![
                Call::Create("event:/SFX/TerrainLowPoint".into()),
                Call::Position(
                    "event:/SFX/TerrainLowPoint".into(),
                    [1077936128, 3212836864, 1082130432]
                ),
                Call::Start("event:/SFX/TerrainLowPoint".into()),
                Call::Parameter(
                    "event:/SFX/TerrainLowPoint".into(),
                    "terrain_smoothness".into(),
                    1053609165
                ),
                Call::Parameter(
                    "event:/SFX/TerrainLowPoint".into(),
                    "editing_speed".into(),
                    1056964608
                ),
            ]
        );
    }
    #[test]
    fn original_one_shot_1() {
        let position = Vec3::new(3., -1., 4.);
        let event = PlayTerrainSound::Removed {
            position,
            falloff: f32::from_bits(1104412672),
            magnitude: f32::from_bits(1082130432),
        };
        let observed = observe(
            event,
            f32::from_bits(1073741824),
            "event:/SFX/TerrainLowPoint",
        );
        assert_eq!(
            observed,
            vec![
                Call::Create("event:/SFX/TerrainLowPoint".into()),
                Call::Position(
                    "event:/SFX/TerrainLowPoint".into(),
                    [1077936128, 3212836864, 1082130432]
                ),
                Call::Start("event:/SFX/TerrainLowPoint".into()),
                Call::Parameter(
                    "event:/SFX/TerrainLowPoint".into(),
                    "terrain_smoothness".into(),
                    1104412672
                ),
                Call::Parameter(
                    "event:/SFX/TerrainLowPoint".into(),
                    "editing_speed".into(),
                    1053609165
                ),
            ]
        );
    }
    #[test]
    fn original_one_shot_2() {
        let position = Vec3::new(3., -1., 4.);
        let event = PlayTerrainSound::Removed {
            position,
            falloff: f32::from_bits(3212836864),
            magnitude: f32::from_bits(1036831949),
        };
        let observed = observe(event, f32::from_bits(0), "event:/SFX/TerrainLowPoint");
        assert_eq!(
            observed,
            vec![
                Call::Create("event:/SFX/TerrainLowPoint".into()),
                Call::Position(
                    "event:/SFX/TerrainLowPoint".into(),
                    [1077936128, 3212836864, 1082130432]
                ),
                Call::Start("event:/SFX/TerrainLowPoint".into()),
                Call::Parameter(
                    "event:/SFX/TerrainLowPoint".into(),
                    "terrain_smoothness".into(),
                    3212836864
                ),
                Call::Parameter(
                    "event:/SFX/TerrainLowPoint".into(),
                    "editing_speed".into(),
                    1065353216
                ),
            ]
        );
    }
    #[test]
    fn original_one_shot_3() {
        let position = Vec3::new(3., -1., 4.);
        let event = PlayTerrainSound::CreateChunk {
            position,
            height: f32::from_bits(1053609165),
        };
        let observed = observe(
            event,
            f32::from_bits(1065353216),
            "event:/SFX/TerrainCreateChunk",
        );
        assert_eq!(
            observed,
            vec![
                Call::Create("event:/SFX/TerrainCreateChunk".into()),
                Call::Position(
                    "event:/SFX/TerrainCreateChunk".into(),
                    [1077936128, 3212836864, 1082130432]
                ),
                Call::Start("event:/SFX/TerrainCreateChunk".into()),
                Call::Parameter(
                    "event:/SFX/TerrainCreateChunk".into(),
                    "terrain_height".into(),
                    1017370378
                ),
            ]
        );
    }
    #[test]
    fn original_one_shot_4() {
        let position = Vec3::new(3., -1., 4.);
        let event = PlayTerrainSound::CreateChunk {
            position,
            height: f32::from_bits(1104412672),
        };
        let observed = observe(
            event,
            f32::from_bits(1073741824),
            "event:/SFX/TerrainCreateChunk",
        );
        assert_eq!(
            observed,
            vec![
                Call::Create("event:/SFX/TerrainCreateChunk".into()),
                Call::Position(
                    "event:/SFX/TerrainCreateChunk".into(),
                    [1077936128, 3212836864, 1082130432]
                ),
                Call::Start("event:/SFX/TerrainCreateChunk".into()),
                Call::Parameter(
                    "event:/SFX/TerrainCreateChunk".into(),
                    "terrain_height".into(),
                    1068079514
                ),
            ]
        );
    }
    #[test]
    fn original_one_shot_5() {
        let position = Vec3::new(3., -1., 4.);
        let event = PlayTerrainSound::CreateChunk {
            position,
            height: f32::from_bits(3212836864),
        };
        let observed = observe(event, f32::from_bits(0), "event:/SFX/TerrainCreateChunk");
        assert_eq!(
            observed,
            vec![
                Call::Create("event:/SFX/TerrainCreateChunk".into()),
                Call::Position(
                    "event:/SFX/TerrainCreateChunk".into(),
                    [1077936128, 3212836864, 1082130432]
                ),
                Call::Start("event:/SFX/TerrainCreateChunk".into()),
                Call::Parameter(
                    "event:/SFX/TerrainCreateChunk".into(),
                    "terrain_height".into(),
                    3175926989
                ),
            ]
        );
    }
}
