//! Original B firstperson_steps140a889b0/1509B. Eight canonical system inputs,
//! six ground materials, current raster buffers and the real mapped rock mask.
use crate::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    input::action_set::ActionSetStates,
    resources::{
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
        terrain_rock_mask::TerrainRockMaskSsbo,
        world_raster_defs::{GardenRaster, WaterRaster},
    },
    systems::{camera_rig::main_camera::MainCamera, paths::PathRastersResources},
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;

pub const FIRST_PERSON_PARAM: &str = "FirstPerson";
pub const GROUND_TYPE_PARAM: &str = "GroundType";
pub const WALK_LOOP_EVENT: &str = "event:/SFX/Player/Walking";
pub const GROUND_TYPE_DEFAULT: f32 = 2.0;
pub const GROUND_TYPE_SAMPLED: f32 = 4.0;
pub const GROUND_TYPE_NONE: f32 = 0.0;
pub const GROUND_SAMPLE_THRESHOLD: f32 = 0.1;
pub const GARDEN_SAMPLE_THRESHOLD: f32 = 0.4;

#[derive(Resource)]
pub struct FirstPersonViewAudioState<E: EventInstance> {
    pub walking_loop: AudioLoop<AudioInstance<E>>,
}

impl<E: EventInstance> FirstPersonViewAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        // Both native init callers pass {name, count3, delay50}. The original
        // instantiate_loop140a59243..6a iterates 0..count+1, then copies delay
        // into AudioLoop+30 and probes the first actual event's "loop" parameter.
        // Use the current shared AudioLoop owner; do not restore its internals.
        let entries: Vec<_> = (0..4)
            .map(|_| {
                let instance = core.instantiate_audio(WALK_LOOP_EVENT);
                if let AudioInstance::Err(error) = &instance {
                    // Native init unwraps instantiate_loop's collected Result.
                    // Previously created entries are released by their owner.
                    panic!("instantiate walking loop: {error}");
                }
                instance
            })
            .collect();
        let uses_loop_parameter = match entries[0].has_parameter("loop") {
            Ok(found) => found,
            Err(error) => {
                log::error!("Couldn't query walking loop parameter: {error:?}");
                false
            }
        };
        Self {
            walking_loop: AudioLoop::new(entries, 50, uses_loop_parameter, false),
        }
    }
}

pub fn firstperson_steps_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(FirstPersonViewAudioState::<S::Event>::new(&core));
}

/// Native call order is water0, paths1/2/3/4, then repeated path1 OR garden5.
/// The mapped bit lookup is lazy after all four paths fail their aggregate gate.
/// Comparison/order is explicit to retain original NaN and rounding behavior.
fn select_ground_material(mut sample: impl FnMut(usize) -> f32, rock: impl FnOnce() -> u32) -> f32 {
    if sample(0) > GROUND_SAMPLE_THRESHOLD {
        return 3.0;
    }
    let p0 = sample(1);
    let p1 = sample(2);
    let p2 = sample(3);
    let p3 = sample(4);
    let sum = ((p0 + p1) + p2) + p3;
    let sum = if 0.0 > sum { 0.0 } else { sum }; // MAXSS zero,sum
    let sum = if 1.0 < sum { 1.0 } else { sum }; // MINSS one,sum
    if sum > GROUND_SAMPLE_THRESHOLD {
        if sample(1) > GROUND_SAMPLE_THRESHOLD {
            1.0
        } else {
            5.0
        }
    } else if rock() != 0 {
        GROUND_TYPE_DEFAULT
    } else if sample(5) > GARDEN_SAMPLE_THRESHOLD {
        GROUND_TYPE_SAMPLED
    } else {
        GROUND_TYPE_NONE
    }
}

/// Full original eight SystemParams in original order. GroundType is written
/// only in the active-Walk branch; inactive frames retain FMOD's last value.
#[allow(clippy::too_many_arguments)]
pub fn firstperson_steps<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    camera: Res<MainCamera>,
    buffers: Res<AssetSsboLibrary>,
    paths: PathRastersResources,
    mut state: ResMut<FirstPersonViewAudioState<S::Event>>,
    water: Res<WaterRaster>,
    garden: Res<GardenRaster>,
    actions: Res<ActionSetStates>,
) {
    let bytes = camera.bytes();
    let walking = bytes[0x4d0] == 3;
    let _ = core.set_parameter(FIRST_PERSON_PARAM, if walking { 1.0 } else { 0.0 });
    let pan = actions.analog[2];
    // Original sqrt/add lane order, not a host movement approximation.
    let moving = actions.digital[3].pressed() || (pan.y * pan.y + pan.x * pan.x).sqrt() > 0.0;
    if walking && moving {
        let material = if bytes[0x328] == 11 {
            // Native resolves and asserts this typed mapping BEFORE sampling
            // even when the water branch subsequently short-circuits.
            let mask = buffers
                .get::<TerrainRockMaskSsbo>()
                .expect("firstperson_steps requires the canonical mapped TerrainRockMaskSsbo");
            let paths = paths.as_float_grid();
            let water = water.0.as_ref();
            let garden = garden.0.as_ref();
            let eye = camera.eye_position();
            select_ground_material(
                |index| {
                    let raster = match index {
                        0 => water,
                        1..=4 => paths[index - 1],
                        5 => garden,
                        _ => unreachable!(),
                    };
                    raster.bilinear_sample_at_world_xz(eye.x, eye.z, 130.0, 130.0)
                },
                || TerrainRockMaskSsbo::sample_mapped_at_world_xz(mask.mapped, eye.x, eye.z),
            )
        } else {
            GROUND_TYPE_DEFAULT
        };
        let _ = core.set_parameter(GROUND_TYPE_PARAM, material);
        if bytes[0x344] == 0 {
            let _ = state.walking_loop.start_if_not_playing();
            return;
        }
    }
    let _ = state.walking_loop.stop_now();
}

#[cfg(test)]
mod original_tests {
    use super::*;
    use crate::{
        input::action_set::{DigitalActionPhase, DigitalActionState},
        resources::{
            audio_instance::PlaybackState,
            audio_resources::{ListenerAttributes, VcaHandle},
            world_raster::{DirtyTilePublisher, WorldRaster},
            world_raster_defs::{
                Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, PathRaster,
            },
        },
    };
    use bevy_ecs::system::RunSystemOnce;
    use std::{
        collections::HashMap,
        sync::{
            Arc, Mutex,
            atomic::{AtomicUsize, Ordering},
        },
    };

    #[derive(Default)]
    struct Log {
        parameters: Mutex<Vec<(String, u32)>>,
        creates: AtomicUsize,
        starts: AtomicUsize,
        releases: AtomicUsize,
        has_loop: bool,
        fail_at: Option<usize>,
    }
    struct Event(Arc<Log>);
    impl EventInstance for Event {
        type Error = &'static str;
        fn start(&mut self) -> Result<(), Self::Error> {
            self.0.starts.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        fn stop(&mut self, _: i32) -> Result<(), Self::Error> {
            Ok(())
        }
        fn get_playback_state(&self) -> Result<PlaybackState, Self::Error> {
            Ok(PlaybackState::Stopped)
        }
        fn set_volume(&mut self, _: f32) -> Result<(), Self::Error> {
            Ok(())
        }
        fn set_parameter_by_name(&mut self, _: &str, _: f32) -> Result<(), Self::Error> {
            Ok(())
        }
        fn has_parameter(&self, name: &str) -> Result<bool, Self::Error> {
            assert_eq!(name, "loop");
            Ok(self.0.has_loop)
        }
        fn set_3d_attributes(
            &mut self,
            _: [f32; 3],
            _: [f32; 3],
            _: [f32; 3],
            _: [f32; 3],
        ) -> Result<(), Self::Error> {
            Ok(())
        }
        fn get_timeline_position(&self) -> Result<i32, Self::Error> {
            Ok(0)
        }
        fn pause(&mut self, _: bool) -> Result<(), Self::Error> {
            Ok(())
        }
        fn install_beat_callback(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn release(&mut self) -> Result<(), Self::Error> {
            self.0.releases.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }
    struct Vca;
    impl VcaHandle for Vca {
        type Error = &'static str;
        fn set_volume(&self, _: f32) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    struct Backend(Arc<Log>);
    impl Studio for Backend {
        type Error = &'static str;
        type Event = Event;
        type Vca = Vca;
        fn load_bank_file(&mut self, _: &str) -> Result<(), Self::Error> {
            Ok(())
        }
        fn load_sample_data(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn get_vca(&self, _: &str) -> Result<Vca, Self::Error> {
            Ok(Vca)
        }
        fn set_listener_attributes(&mut self, _: ListenerAttributes) -> Result<(), Self::Error> {
            Ok(())
        }
        fn update(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn set_parameter_by_name(&self, name: &str, value: f32) -> Result<(), Self::Error> {
            self.0
                .parameters
                .lock()
                .unwrap()
                .push((name.to_owned(), value.to_bits()));
            Ok(())
        }
        fn create_event_instance(&self, name: &str) -> Result<Option<Event>, Self::Error> {
            assert_eq!(name, WALK_LOOP_EVENT);
            let index = self.0.creates.fetch_add(1, Ordering::SeqCst);
            if self.0.fail_at == Some(index) {
                return Err("native fixture creation failure");
            }
            Ok(Some(Event(self.0.clone())))
        }
    }
    fn core(log: &Arc<Log>, studio: bool) -> AudioCore<Backend> {
        AudioCore {
            studio: studio.then(|| Backend(log.clone())),
            banks: HashMap::new(),
        }
    }
    fn raster(value: u8, selector: u8) -> WorldRaster<512> {
        let mut raster = WorldRaster::new(DirtyTilePublisher::unique());
        raster.composite_mut().fill(value);
        raster.scratch_mut().fill(255 - value);
        if selector != 0 {
            raster.copy_to_staging(selector, &mut vec![0; 512 * 512]);
        }
        raster
    }
    fn fixtures() -> serde_json::Value {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/original-firstperson-b.json"
        )))
        .unwrap()
    }
    fn setting<T: serde::de::DeserializeOwned>(
        inputs: &serde_json::Value,
        name: &str,
        default: T,
    ) -> T {
        inputs
            .get(name)
            .map(|v| serde_json::from_value(v.clone()).unwrap())
            .unwrap_or(default)
    }
    fn expected_parameters(result: &serde_json::Value) -> Vec<(String, u32)> {
        result["calls"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|entry| {
                Some((
                    entry.get("parameter")?.as_str()?.to_owned(),
                    entry["value_bits"].as_u64()? as u32,
                ))
            })
            .collect()
    }
    #[test]
    fn complete_original_203_cases_drive_actual_eight_parameter_ecs_system() {
        for case in fixtures().as_array().unwrap() {
            let inputs = &case["inputs"];
            let mode = setting(inputs, "mode", 3u8);
            let phase = setting(inputs, "phase", 1u32);
            let pan = setting(inputs, "pan_bits", [0u32; 2]);
            let channels = setting(inputs, "channels", [0u8; 6]);
            let selector = setting(inputs, "selector", 0u8);
            let log = Arc::new(Log::default());
            let backend = core(&log, setting(inputs, "studio", true));
            let audio = FirstPersonViewAudioState::new(&backend);
            let mut camera = MainCamera::new(1.0);
            camera.set_mode_byte(crate::systems::camera_rig::main_camera::CameraMode(mode));
            camera.write_u64(0x328, setting(inputs, "source", 11u64));
            camera.write_u64(0x340, setting(inputs, "modifier", 0u64) << 32);
            let mut world = World::new();
            world.insert_resource(backend);
            world.insert_resource(audio);
            world.insert_resource(camera);
            let mut actions = ActionSetStates::default();
            actions.digital[3] = DigitalActionState {
                phase: match phase {
                    0 => DigitalActionPhase::JustPressed,
                    1 => DigitalActionPhase::Held,
                    2 => DigitalActionPhase::Repeating,
                    3 => DigitalActionPhase::JustReleased,
                    4 => DigitalActionPhase::Idle,
                    _ => panic!("native digital phase"),
                },
                timer_seconds: 0.0,
            };
            actions.analog[2] = glam::Vec2::new(f32::from_bits(pan[0]), f32::from_bits(pan[1]));
            world.insert_resource(actions);
            let mut library = AssetSsboLibrary::new();
            library
                .add_new::<TerrainRockMaskSsbo>(&crate::render::test_device::TestDevice::default());
            library
                .get_mut::<TerrainRockMaskSsbo>()
                .unwrap()
                .mapped
                .fill(TerrainRockMaskSsbo(if setting(inputs, "rock", false) {
                    u32::MAX
                } else {
                    0
                }));
            world.insert_resource(library);
            world.insert_resource(WaterRaster(raster(channels[0], selector)));
            world.insert_resource(PathRaster(raster(channels[1], selector)));
            world.insert_resource(Cobblestone1Raster(raster(channels[2], selector)));
            world.insert_resource(Cobblestone2Raster(raster(channels[3], selector)));
            world.insert_resource(Cobblestone3Raster(raster(channels[4], selector)));
            world.insert_resource(GardenRaster(raster(channels[5], selector)));
            world.run_system_once(firstperson_steps::<Backend>).unwrap();
            assert_eq!(
                *log.parameters.lock().unwrap(),
                expected_parameters(&case["result"]),
                "{}",
                case["name"]
            );
            let expected_start = case["result"]["calls"]
                .as_array()
                .unwrap()
                .iter()
                .any(|entry| entry.get("loop").and_then(|v| v.as_str()) == Some("start"));
            let loop_owner = &world
                .resource::<FirstPersonViewAudioState<Event>>()
                .walking_loop;
            assert_eq!(loop_owner.started, expected_start, "{}", case["name"]);
            assert_eq!(
                loop_owner.start.is_some(),
                !expected_start,
                "{}",
                case["name"]
            );
        }
    }
    #[test]
    fn native_sample_order_and_repeated_first_path_are_preserved() {
        for case in fixtures().as_array().unwrap().iter().filter(|case| {
            case["name"].as_str().unwrap().contains("False") || case["name"] == "scratch-selected"
        }) {
            let channels = setting(&case["inputs"], "channels", [0u8; 6]);
            let selector = setting(&case["inputs"], "selector", 0u8);
            let mut order = Vec::new();
            let value = select_ground_material(
                |index| {
                    order.push(index);
                    f32::from(if selector == 0 {
                        channels[index]
                    } else {
                        255 - channels[index]
                    }) / 255.0
                },
                || u32::from(setting(&case["inputs"], "rock", false)),
            );
            let expected = expected_parameters(&case["result"])
                .into_iter()
                .find(|(name, _)| name == GROUND_TYPE_PARAM)
                .unwrap()
                .1;
            assert_eq!(value.to_bits(), expected, "{}", case["name"]);
            let expected_order: Vec<usize> =
                serde_json::from_value(case["result"]["sample_order"].clone()).unwrap();
            assert_eq!(order, expected_order, "{}", case["name"]);
        }
    }
    #[test]
    fn walking_init_has_four_instances_fifty_ms_and_actual_parameter_probe() {
        for has_loop in [false, true] {
            let log = Arc::new(Log {
                has_loop,
                ..Default::default()
            });
            let state = FirstPersonViewAudioState::new(&core(&log, true));
            assert_eq!(log.creates.load(Ordering::SeqCst), 4);
            assert_eq!(state.walking_loop.entries.len(), 4);
            assert_eq!(state.walking_loop.threshold_ms, 50);
            assert_eq!(state.walking_loop.uses_loop_parameter, has_loop);
            assert!(!state.walking_loop.stops_explicitly);
        }
    }
    #[test]
    fn creation_failure_drops_preceding_instances_and_does_not_build_a_dummy_loop() {
        let log = Arc::new(Log {
            fail_at: Some(2),
            ..Default::default()
        });
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            FirstPersonViewAudioState::new(&core(&log, true))
        }));
        assert!(outcome.is_err());
        assert_eq!(log.creates.load(Ordering::SeqCst), 3);
        assert_eq!(log.releases.load(Ordering::SeqCst), 2);
    }
}
