//! 音频缝（critters 音频消费侧）。
//!
//! 证据：`AudioInstance::start_if_not_playing`（0x140a57300）、`set_position`
//! （0x140a57da0）、`set_volume`（0x140a575a0）、`AudioLoop::current`（0x140a593f0）、
//! `set_parameter`（0x140a57670）、`stop`（0x140a59430）、`AudioCore::instantiate_audio`
//! （0x140a55f40）——均为 country-core `resources::audio_resources` 的 FMOD 实例方法。
//! country-core 的 `AudioInstance<S>` 对 Studio 后端泛型；本模块提供 trait 缝，
//! 宿主装配层用真实 `AudioInstance<S>` 实现（blanket impl 映射各原生调用）。

use bevy_ecs::resource::Resource;
use glam::Vec3;

/// critters 消费的音频方法集（每方法 = 一条已证原生调用）。
pub trait CritterAudioSink: Send + Sync {
    /// `AudioInstance::start`（0x140a57240）：每次调用都转发给活动实例。
    fn start(&mut self);
    /// `start_if_not_playing`（0x140a57300）：位掩码 0xb 跳过 {Playing,Sustaining,Starting}。
    fn start_if_not_playing(&mut self);
    /// `stop`（0x140a59430/0x140a57470）：STOP_IMMEDIATE。
    fn stop(&mut self);
    /// `set_position`（0x140a57da0）：forward=(0,0,-1), up=(0,1,0), velocity=0。
    fn set_position(&mut self, position: Vec3);
    /// `set_volume`（0x140a575a0）。
    fn set_volume(&mut self, volume: f32);
    /// `set_parameter`（0x140a57670）。
    fn set_parameter(&mut self, name: &str, value: f32);
}

/// 惰性实例句柄（Disabled = 未实例化；Active 持有宿主实例）。
pub enum AudioHandle {
    Disabled,
    Active(Box<dyn CritterAudioSink>),
}

impl std::fmt::Debug for AudioHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioHandle::Disabled => f.write_str("AudioHandle::Disabled"),
            AudioHandle::Active(_) => f.write_str("AudioHandle::Active"),
        }
    }
}

impl Default for AudioHandle {
    fn default() -> Self {
        AudioHandle::Disabled
    }
}

impl AudioHandle {
    pub fn start(&mut self) {
        if let AudioHandle::Active(sink) = self {
            sink.start();
        }
    }
    pub fn start_if_not_playing(&mut self) {
        if let AudioHandle::Active(sink) = self {
            sink.start_if_not_playing();
        }
    }
    pub fn stop(&mut self) {
        if let AudioHandle::Active(sink) = self {
            sink.stop();
        }
    }
    pub fn set_position(&mut self, position: Vec3) {
        if let AudioHandle::Active(sink) = self {
            sink.set_position(position);
        }
    }
    pub fn set_volume(&mut self, volume: f32) {
        if let AudioHandle::Active(sink) = self {
            sink.set_volume(volume);
        }
    }
    pub fn set_parameter(&mut self, name: &str, value: f32) {
        if let AudioHandle::Active(sink) = self {
            sink.set_parameter(name, value);
        }
    }
    pub fn is_active(&self) -> bool {
        matches!(self, AudioHandle::Active(_))
    }
}

/// 鸭音频（Duck::new 装载三事件 event:/SFX/Ducks/{Quack,TakeOff,Land}，O core 章节）。
#[derive(Debug)]
pub struct DuckAudio {
    pub quack: AudioHandle,
    pub takeoff: AudioHandle,
    pub land: AudioHandle,
    /// B Duck+a8：Flap 四实例环，独立于三个单次事件。
    pub flap: AudioHandle,
}

impl Default for DuckAudio {
    fn default() -> Self {
        Self {
            quack: AudioHandle::Disabled,
            takeoff: AudioHandle::Disabled,
            land: AudioHandle::Disabled,
            flap: AudioHandle::Disabled,
        }
    }
}

impl DuckAudio {
    /// B Duck::new 0x140f85bb0：严格保留创建、参数及失败前缀释放顺序。
    pub fn from_audio_core<S: country_core::resources::audio_resources::Studio>(
        core: &country_core::resources::audio_resources::AudioCore<S>,
        pitch: f32,
    ) -> Self {
        use country_core::resources::audio_resources::AudioLoopDesc;
        let mut quack = duck_audio_instance(core, "event:/SFX/Ducks/Quack");
        // 原方法失败时全局去重告警后继续；依赖 canonical set_parameter 的报告策略。
        let _ = quack.set_parameter("Pitch", pitch);
        let takeoff = duck_audio_instance(core, "event:/SFX/Ducks/TakeOff");
        let land = duck_audio_instance(core, "event:/SFX/Ducks/Land");
        let flap = core.instantiate_loop(&AudioLoopDesc {
            name: "event:/SFX/Ducks/Flap".to_owned(),
            count: 3,
            threshold_ms: 50,
        });
        Self {
            quack: AudioHandle::Active(Box::new(quack)),
            takeoff: AudioHandle::Active(Box::new(takeoff)),
            land: AudioHandle::Active(Box::new(land)),
            flap: AudioHandle::Active(Box::new(DuckFlapLoop(flap))),
        }
    }

    /// 原移动消费者只管理独立 Flap 环，不启动或停止 Quack/TakeOff/Land。
    pub fn start_loops(&mut self, position: Vec3, landing_t: f32, is_taking_off: bool) {
        self.flap.start_if_not_playing();
        self.flap.set_position(position);
        self.flap.set_parameter("landing_t", landing_t);
        self.flap
            .set_parameter("is_taking_off", if is_taking_off { 1.0 } else { 0.0 });
    }
    pub fn stop_loops(&mut self) {
        self.flap.stop();
    }
}

fn duck_audio_instance<S: country_core::resources::audio_resources::Studio>(
    core: &country_core::resources::audio_resources::AudioCore<S>,
    name: &str,
) -> country_core::resources::audio_resources::AudioInstance<S::Event> {
    use country_core::resources::audio_resources::AudioInstance;
    let instance = core.instantiate_audio(name);
    if let AudioInstance::Err(error) = &instance {
        panic!("called `Result::unwrap()` on an `Err` value: {error}");
    }
    instance
}

/// 仅擦除后端事件类型；实例轮换、实时时钟、延迟停止及 loop 参数均复用 core。
struct DuckFlapLoop<E: country_core::resources::audio_resources::EventInstance>(
    country_core::utils::audio_utils::AudioLoop<
        country_core::resources::audio_resources::AudioInstance<E>,
    >,
);

impl<E: country_core::resources::audio_resources::EventInstance + Send + Sync> CritterAudioSink
    for DuckFlapLoop<E>
{
    fn start(&mut self) {
        let _ = self.0.current_mut().start();
    }
    fn start_if_not_playing(&mut self) {
        let _ = self.0.start_if_not_playing();
    }
    fn stop(&mut self) {
        let _ = self.0.stop_now();
    }
    fn set_position(&mut self, position: Vec3) {
        let _ = self.0.current_mut().set_position(position.to_array());
    }
    fn set_volume(&mut self, volume: f32) {
        let _ = self.0.current_mut().set_volume(volume);
    }
    fn set_parameter(&mut self, name: &str, value: f32) {
        let _ = self.0.current_mut().set_parameter(name, value);
    }
}

/// 音频核心缝（`AudioCore::instantiate_audio` 0x140a55f40 语义；宿主 FMOD 装配）。
#[derive(Resource, Default)]
pub struct CritterAudioCore {
    /// 实例化回调（宿主把 `AudioCore<S>::instantiate_audio` 接入）。
    pub instantiate: Option<fn(&str) -> AudioHandle>,
    duck_audio_factory: Option<std::sync::Arc<dyn Fn(f32) -> DuckAudio + Send + Sync>>,
}

impl std::fmt::Debug for CritterAudioCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CritterAudioCore")
            .field("instantiate", &self.instantiate.is_some())
            .field("duck_audio_factory", &self.duck_audio_factory.is_some())
            .finish()
    }
}

impl CritterAudioCore {
    /// 宿主传入同一音频引擎的共享 core；不会创建第二个 Studio。
    /// 无音频模式也传入真实的 AudioCore<DisabledStudio>，执行原 Disabled 构造分支。
    pub fn from_audio_core<S>(core: country_core::resources::audio_resources::AudioCore<S>) -> Self
    where
        S: country_core::resources::audio_resources::Studio + Send + Sync + 'static,
    {
        Self {
            instantiate: None,
            duck_audio_factory: Some(std::sync::Arc::new(move |pitch| {
                DuckAudio::from_audio_core(&core, pitch)
            })),
        }
    }

    /// 缺失宿主装配是错误，不能用全 Disabled 结果冒充已构造的鸭子。
    pub fn instantiate_duck(&self, pitch: f32) -> DuckAudio {
        self.duck_audio_factory
            .as_ref()
            .expect("Duck audio factory must be installed from the actual AudioCore")(pitch)
    }
    pub fn instantiate(&self, name: &str) -> AudioHandle {
        match self.instantiate {
            Some(f) => f(name),
            None => AudioHandle::Disabled,
        }
    }
}

/// blanket impl：把 country-core 的 `AudioInstance<S>` 映射到本缝
/// （各方法经 core 方法转发到已证原生调用）。
impl<S: country_core::resources::audio_resources::EventInstance + Send + Sync> CritterAudioSink
    for country_core::resources::audio_resources::AudioInstance<S>
{
    fn start(&mut self) {
        let _ = country_core::resources::audio_resources::AudioInstance::start(self);
    }
    fn start_if_not_playing(&mut self) {
        let _ = country_core::resources::audio_resources::AudioInstance::start_if_not_playing(self);
    }
    fn stop(&mut self) {
        let _ = country_core::resources::audio_resources::AudioInstance::stop(self);
    }
    fn set_position(&mut self, position: Vec3) {
        let _ = country_core::resources::audio_resources::AudioInstance::set_position(
            self,
            [position.x, position.y, position.z],
        );
    }
    fn set_volume(&mut self, volume: f32) {
        let _ = country_core::resources::audio_resources::AudioInstance::set_volume(self, volume);
    }
    fn set_parameter(&mut self, name: &str, value: f32) {
        let _ = country_core::resources::audio_resources::AudioInstance::set_parameter(
            self, name, value,
        );
    }
}

#[cfg(test)]
mod duck_constructor_tests {
    use super::*;
    use country_core::resources::audio_instance::PlaybackState;
    use country_core::resources::audio_resources::{
        AudioCore, DisabledStudio, EventInstance, ListenerAttributes, Studio, VcaHandle,
    };
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    #[derive(Default)]
    struct Trace {
        calls: Vec<String>,
        creates: usize,
        fail_at: Option<usize>,
        fail_pitch: bool,
        releases: Vec<usize>,
    }
    struct Event {
        id: usize,
        trace: Arc<Mutex<Trace>>,
    }
    impl EventInstance for Event {
        type Error = &'static str;
        fn start(&mut self) -> Result<(), Self::Error> {
            self.trace
                .lock()
                .unwrap()
                .calls
                .push(format!("start:{}", self.id));
            Ok(())
        }
        fn stop(&mut self, _: i32) -> Result<(), Self::Error> {
            self.trace
                .lock()
                .unwrap()
                .calls
                .push(format!("stop:{}", self.id));
            Ok(())
        }
        fn get_playback_state(&self) -> Result<PlaybackState, Self::Error> {
            Ok(PlaybackState::Stopped)
        }
        fn set_volume(&mut self, value: f32) -> Result<(), Self::Error> {
            self.trace.lock().unwrap().calls.push(format!(
                "volume:{}:{:08x}",
                self.id,
                value.to_bits()
            ));
            Ok(())
        }
        fn set_parameter_by_name(&mut self, name: &str, value: f32) -> Result<(), Self::Error> {
            let mut trace = self.trace.lock().unwrap();
            trace.calls.push(format!(
                "parameter:{}:{name}:{:08x}",
                self.id,
                value.to_bits()
            ));
            if name == "Pitch" && trace.fail_pitch {
                Err("pitch failure")
            } else {
                Ok(())
            }
        }
        fn has_parameter(&self, name: &str) -> Result<bool, Self::Error> {
            self.trace
                .lock()
                .unwrap()
                .calls
                .push(format!("probe:{}:{name}", self.id));
            Ok(true)
        }
        fn set_3d_attributes(
            &mut self,
            position: [f32; 3],
            forward: [f32; 3],
            up: [f32; 3],
            velocity: [f32; 3],
        ) -> Result<(), Self::Error> {
            assert_eq!(forward, [0.0, 0.0, -1.0]);
            assert_eq!(up, [0.0, 1.0, 0.0]);
            assert_eq!(velocity, [0.0; 3]);
            self.trace
                .lock()
                .unwrap()
                .calls
                .push(format!("position:{}:{position:?}", self.id));
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
            self.trace.lock().unwrap().releases.push(self.id);
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
    struct RecordingStudio(Arc<Mutex<Trace>>);
    impl Studio for RecordingStudio {
        type Error = &'static str;
        type Event = Event;
        type Vca = Vca;
        fn load_bank_file(&mut self, _: &str) -> Result<(), Self::Error> {
            Ok(())
        }
        fn load_sample_data(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn get_vca(&self, _: &str) -> Result<Self::Vca, Self::Error> {
            Ok(Vca)
        }
        fn set_listener_attributes(&mut self, _: ListenerAttributes) -> Result<(), Self::Error> {
            Ok(())
        }
        fn update(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn set_parameter_by_name(&self, _: &str, _: f32) -> Result<(), Self::Error> {
            Ok(())
        }
        fn create_event_instance(&self, name: &str) -> Result<Option<Self::Event>, Self::Error> {
            let mut trace = self.0.lock().unwrap();
            let id = trace.creates;
            trace.creates += 1;
            trace.calls.push(format!("create:{id}:{name}"));
            if trace.fail_at == Some(id) {
                return Err("creation failure");
            }
            Ok(Some(Event {
                id,
                trace: self.0.clone(),
            }))
        }
    }
    fn core(trace: Trace) -> (AudioCore<RecordingStudio>, Arc<Mutex<Trace>>) {
        let trace = Arc::new(Mutex::new(trace));
        (
            AudioCore {
                studio: Some(RecordingStudio(trace.clone())),
                banks: HashMap::new(),
            },
            trace,
        )
    }

    #[test]
    fn constructor_keeps_all_seven_instances_and_the_raw_pitch_in_native_order() {
        let (core, trace) = core(Trace::default());
        let pitch = f32::from_bits(0x3e8abcde);
        let audio = DuckAudio::from_audio_core(&core, pitch);
        assert_eq!(
            trace.lock().unwrap().calls,
            [
                "create:0:event:/SFX/Ducks/Quack",
                "parameter:0:Pitch:3e8abcde",
                "create:1:event:/SFX/Ducks/TakeOff",
                "create:2:event:/SFX/Ducks/Land",
                "create:3:event:/SFX/Ducks/Flap",
                "create:4:event:/SFX/Ducks/Flap",
                "create:5:event:/SFX/Ducks/Flap",
                "create:6:event:/SFX/Ducks/Flap",
                "probe:3:loop",
            ]
        );
        assert!(trace.lock().unwrap().releases.is_empty());
        drop(audio);
        assert_eq!(trace.lock().unwrap().releases, [0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn every_creation_failure_panics_and_releases_only_the_successful_prefix() {
        for fail_at in 0..7 {
            let (core, trace) = core(Trace {
                fail_at: Some(fail_at),
                ..Default::default()
            });
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    DuckAudio::from_audio_core(&core, 0.25)
                }))
                .is_err()
            );
            let trace = trace.lock().unwrap();
            assert_eq!(trace.creates, fail_at + 1);
            let mut releases = trace.releases.clone();
            releases.sort_unstable();
            assert_eq!(releases, (0..fail_at).collect::<Vec<_>>());
            assert!(!trace.calls.iter().any(|call| call.starts_with("probe:")));
        }
    }

    #[test]
    fn pitch_failure_does_not_skip_takeoff_land_or_the_flap_ring() {
        let (core, trace) = core(Trace {
            fail_pitch: true,
            ..Default::default()
        });
        let audio = DuckAudio::from_audio_core(&core, 0.25);
        assert_eq!(trace.lock().unwrap().creates, 7);
        assert!(
            trace
                .lock()
                .unwrap()
                .calls
                .iter()
                .any(|call| call == "probe:3:loop")
        );
        drop(audio);
        assert_eq!(trace.lock().unwrap().releases.len(), 7);
    }

    #[test]
    fn flap_uses_the_real_ring_and_never_starts_the_three_one_shots() {
        let (core, trace) = core(Trace::default());
        let mut audio = DuckAudio::from_audio_core(&core, 0.25);
        trace.lock().unwrap().calls.clear();
        audio.start_loops(Vec3::new(1.0, 2.0, 3.0), 0.25, true);
        audio.start_loops(Vec3::new(4.0, 5.0, 6.0), 0.5, false);
        audio.stop_loops(); // 原环首次停止仅记录时钟，不立即停止或轮换实例。
        assert_eq!(
            trace.lock().unwrap().calls,
            [
                "parameter:3:loop:3f800000",
                "start:3",
                "position:3:[1.0, 2.0, 3.0]",
                "parameter:3:landing_t:3e800000",
                "parameter:3:is_taking_off:3f800000",
                "position:3:[4.0, 5.0, 6.0]",
                "parameter:3:landing_t:3f000000",
                "parameter:3:is_taking_off:00000000",
            ]
        );
    }

    #[test]
    fn a_real_disabled_core_still_constructs_and_owns_the_disabled_audio_chain() {
        let factory = CritterAudioCore::from_audio_core(AudioCore::<DisabledStudio> {
            studio: None,
            banks: HashMap::new(),
        });
        let mut audio = factory.instantiate_duck(0.25);
        audio.quack.start_if_not_playing();
        audio.start_loops(Vec3::ZERO, 0.5, false);
        audio.stop_loops();
    }

    #[test]
    #[should_panic(expected = "Duck audio factory must be installed")]
    fn an_uninstalled_factory_is_not_silently_treated_as_disabled_audio() {
        let _ = CritterAudioCore::default().instantiate_duck(0.25);
    }

    #[test]
    fn real_planner_factory_and_building_consumer_share_the_same_ducks_and_audio() {
        use crate::components::Duck;
        use crate::duck::{
            CritterRng, DevHotkeys, DuckTrajectoryPlanner, Trajectory, TrajectoryPlan,
        };
        use bevy_ecs::prelude::*;
        use country_core::{
            resources::{
                critters::sheep::CmdCheckForSheepInsideBuilding,
                roofs::geometry::surface::Curve2,
                time::GameTime,
                walls::{
                    InnerWalls, WallState, public_walls::PublicWalls,
                    publication_source::TerrainHeightGrid512,
                },
            },
            systems::wall::valid_enclosure::{Shape, ShapeParameters},
        };
        let (audio, trace) = core(Trace::default());
        let mut world = World::new();
        world.insert_resource(CritterAudioCore::from_audio_core(audio));
        world.insert_resource(GameTime::default());
        world.insert_resource(CritterRng::default());
        world.insert_resource(DevHotkeys::default());
        world.insert_resource(DuckTrajectoryPlanner {
            plans: vec![TrajectoryPlan {
                trajectories: vec![
                    Trajectory {
                        points: vec![Vec3::ZERO, Vec3::X],
                        done: true,
                    },
                    Trajectory {
                        points: vec![Vec3::ZERO, Vec3::Z],
                        done: true,
                    },
                    Trajectory {
                        points: Vec::new(),
                        done: true,
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });
        let (original_capacity, original_pointer) = {
            let planner = world.resource::<DuckTrajectoryPlanner>();
            (
                planner.plans[0].trajectories.capacity(),
                planner.plans[0].trajectories.as_ptr() as usize,
            )
        };
        let mut oracle = perchance::PerchanceContext::new(crate::duck::DUCK_PLANNER_RNG_SEED_B);
        let expected: [(u32, u32); 3] = std::array::from_fn(|_| {
            let pitch = oracle.uniform_f32().to_bits();
            let progress = (oracle.uniform_f32() * f32::from_bits(0xbe99_999a)).to_bits();
            (pitch, progress)
        });
        let expected_next = oracle.uniform_f32().to_bits();
        let mut spawn = Schedule::default();
        spawn.add_systems(crate::duck::plan_duck_trajectories);
        spawn.run(&mut world);
        assert_eq!(world.query::<&Duck>().iter(&world).count(), 3);
        assert_eq!(
            world
                .resource_mut::<DuckTrajectoryPlanner>()
                .rng
                .uniform_f32()
                .to_bits(),
            expected_next
        );
        for duck in world.query::<&Duck>().iter(&world) {
            let index = match duck.trajectory.first() {
                Some(point) if *point == Vec3::X => 0,
                Some(point) if *point == Vec3::Z => 1,
                None => 2,
                other => panic!("unexpected reversed trajectory: {other:?}"),
            };
            assert_eq!(duck.progress.to_bits(), expected[index].1);
            assert_eq!([duck.f_f0, duck.f_f4, duck.f_f8], [0.0; 3]);
            assert_eq!(duck.dwell[0], 0.0);
        }
        let pitch_calls: Vec<_> = trace
            .lock()
            .unwrap()
            .calls
            .iter()
            .filter(|call| call.contains(":Pitch:"))
            .cloned()
            .collect();
        assert_eq!(
            pitch_calls,
            [
                format!("parameter:0:Pitch:{:08x}", expected[0].0),
                format!("parameter:7:Pitch:{:08x}", expected[1].0),
                format!("parameter:14:Pitch:{:08x}", expected[2].0),
            ]
        );
        // 第二帧不能重复出生、抽样或创建音频；drain 保留外层缓冲及容量。
        spawn.run(&mut world);
        assert_eq!(world.query::<&Duck>().iter(&world).count(), 3);
        assert_eq!(trace.lock().unwrap().creates, 21);
        assert_eq!(
            world
                .resource_mut::<DuckTrajectoryPlanner>()
                .rng
                .uniform_f32()
                .to_bits(),
            oracle.uniform_f32().to_bits()
        );
        let mut animal_oracle = perchance::PerchanceContext::new(crate::CRITTER_RNG_SEED_B);
        assert_eq!(world.resource_mut::<CritterRng>().0.uniform_f32().to_bits(),
            animal_oracle.uniform_f32().to_bits(),
            "duck trajectory sampling consumed the independent animal-goal RNG");
        let planner = world.resource::<DuckTrajectoryPlanner>();
        assert!(planner.plans[0].trajectories.is_empty());
        assert_eq!(planner.plans[0].trajectories.capacity(), original_capacity);
        assert_eq!(
            planner.plans[0].trajectories.as_ptr() as usize,
            original_pointer
        );

        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut snapshot = WallState::new(curve, 4.0, 0, 0, false).snapshot().clone();
        snapshot.source.shape =
            ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d {
                center: glam::Vec2::ZERO,
                radius: 3.0,
                rotation: 0.0,
            }))
            .to_snapshot();
        let mut inner = InnerWalls::default();
        let wall = inner.new_wall(WallState::from_snapshot(snapshot, 0));
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(),
                minimum_y: 0.0,
                maximum_y: 0.0,
                sampling_fix: false,
            }, None, &country_core::resources::walls::public_walls::WallPublicationDirty::default(), &country_core::resources::walls::public_walls::WallSpatialHash::default());
        world.insert_resource(public);
        world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
        world
            .resource_mut::<Events<CmdCheckForSheepInsideBuilding>>()
            .send(CmdCheckForSheepInsideBuilding { wall });
        let mut react = Schedule::default();
        react.add_systems(
            crate::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings,
        );
        react.run(&mut world);
        let starts: Vec<_> = trace
            .lock()
            .unwrap()
            .calls
            .iter()
            .filter(|call| call.starts_with("start:"))
            .cloned()
            .collect();
        assert_eq!(starts.len(), 3);
        assert!(starts.contains(&"start:0".to_owned()));
        assert!(starts.contains(&"start:7".to_owned()));
        assert!(starts.contains(&"start:14".to_owned()));
    }
}
