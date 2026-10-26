//! B `tiny_glade::rollercoaster`：摄影乘坐装饰车链。
//!
//! 原入口：update@1403b7f20、create_cart_chain@1403be1a0、
//! CoasterAudioInstance::new@1403b6880。当前沿用既有RaycastWorld、
//! DecoratorVisualState、ComponentWrite和AudioCore架构。
//! 原指令夹具、桥接边界与尚待生产联合验收项见本轮toolbar-rollercoaster报告，
//! 不以局部数值差分替代完整业务验收。

use anyhow::Result;

/// Audio backend used by the coaster (country-core `AudioCore` in B).
pub trait CoasterAudio {
    type Instance;
    fn instantiate_audio(&mut self) -> Result<Self::Instance>;
    fn set_volume(instance: &mut Self::Instance, volume: f32);
    fn set_parameter(instance: &mut Self::Instance, name: &str, value: f32);
}

/// B `tiny_glade::rollercoaster::CoasterAudioInstance`.
pub struct CoasterAudioInstance<I> {
    pub instance: I,
    pub volume: f32,
}

impl<I> CoasterAudioInstance<I> {
    /// Minimum audible volume (`0.03` in the machine code).
    pub const MIN_VOLUME: f32 = 0.03;

    /// B `CoasterAudioInstance::new` (`0x1403b6880`).
    pub fn new<A: CoasterAudio<Instance = I>>(
        audio: &mut A,
        volume: f32,
        parameters: &[(&str, f32)],
    ) -> Option<Self> {
        if !(volume >= Self::MIN_VOLUME) {
            return None;
        }
        match audio.instantiate_audio() {
            Ok(mut instance) => {
                A::set_volume(&mut instance, volume);
                for (name, value) in parameters {
                    A::set_parameter(&mut instance, name, *value);
                }
                Some(Self { instance, volume })
            }
            Err(error) => {
                log::error!("failed to instantiate coaster audio: {error:#}");
                None
            }
        }
    }
}

/// 将现有 AudioCore 作为原构造器的注入后端；没有另外实现FMOD或伪造句柄。
struct NamedCoasterAudio<'a, S: country_core::resources::audio_resources::Studio> {
    core: &'a country_core::resources::audio_resources::AudioCore<S>,
    name: &'a str,
}
impl<S: country_core::resources::audio_resources::Studio> CoasterAudio
    for NamedCoasterAudio<'_, S>
{
    type Instance = country_core::resources::audio_resources::AudioInstance<S::Event>;
    fn instantiate_audio(&mut self) -> Result<Self::Instance> {
        let instance = self.core.instantiate_audio(self.name);
        if let country_core::resources::audio_resources::AudioInstance::Err(error) = &instance {
            Err(anyhow::anyhow!("{error}"))
        } else {
            Ok(instance)
        }
    }
    fn set_volume(instance: &mut Self::Instance, volume: f32) {
        let _ = instance.set_volume(volume);
    }
    fn set_parameter(instance: &mut Self::Instance, name: &str, value: f32) {
        let _ = instance.set_parameter(name, value);
    }
}

type CoasterSound<E> =
    CoasterAudioInstance<country_core::resources::audio_resources::AudioInstance<E>>;

/// 原每车0x128字节音频块的安全所有权表达：四个可选实例和三个可选实例池。
/// None池保留“任一实例构造失败则整池失败”的原collect::<Option<Vec<_>>>()语义。
struct CoasterAudioState<E: country_core::resources::audio_resources::EventInstance> {
    timberframe: Option<CoasterSound<E>>,
    height: Option<CoasterSound<E>>,
    overlap: Option<CoasterSound<E>>,
    water: Option<CoasterSound<E>>,
    chimney: Option<Vec<CoasterSound<E>>>,
    fence: Option<Vec<CoasterSound<E>>>,
    lantern: Option<Vec<CoasterSound<E>>>,
}
impl<E: country_core::resources::audio_resources::EventInstance> CoasterAudioState<E> {
    fn new<S: country_core::resources::audio_resources::Studio<Event = E>>(
        core: &country_core::resources::audio_resources::AudioCore<S>,
        index: usize,
        selected: usize,
    ) -> Self {
        let volume = (1.0 / ((index as f32 - selected as f32).abs() + 1.0)).powf(1.5);
        let make = |name: &str, parameters: &[(&str, f32)]| {
            CoasterAudioInstance::new(&mut NamedCoasterAudio { core, name }, volume, parameters)
        };
        let pool = |name: &str, count: usize| {
            (0..count)
                .map(|_| make(name, &[]))
                .collect::<Option<Vec<_>>>()
        };
        let mut timberframe = make("event:/SFX/TimberframeWidth", &[]);
        if let Some(sound) = &mut timberframe {
            let _ = sound.instance.start_if_not_playing();
        }
        let (chimney, fence, height, overlap) = if index == selected {
            let chimney = pool("event:/SFX/ChimneyPipePlace", 10);
            let fence = pool("event:/SFX/FenceRearrange", 10);
            let mut height = make(
                "event:/SFX/Building/BuildingChangeHeight",
                &[
                    ("loop", 1.0),
                    ("BuildingSize", 1.0),
                    ("BuildingHeight", 1.0),
                    ("editing_speed", 0.0),
                ],
            );
            if let Some(sound) = &mut height {
                let _ = sound.instance.set_volume(0.0);
                let _ = sound.instance.start_if_not_playing();
            }
            let mut overlap = make("event:/SFX/Building/BuildingOverlapMove", &[]);
            if let Some(sound) = &mut overlap {
                let _ = sound.instance.set_volume(sound.volume * 0.7);
                let _ = sound.instance.start_if_not_playing();
            }
            (chimney, fence, height, overlap)
        } else {
            (None, None, None, None)
        };
        let lantern = pool("event:/SFX/Lantern/PlaceGround", 6);
        let mut water = make("event:/SFX/Water/WaterDraw", &[("editing_speed", 1.0)]);
        if let Some(sound) = &mut water {
            let _ = sound.instance.set_volume(sound.volume * 2.0);
        }
        Self {
            timberframe,
            height,
            overlap,
            water,
            chimney,
            fence,
            lantern,
        }
    }
}

/// 原 `raycast_cart` 选中的首个持久视觉实体，以及原 Stair 目的地信息。
/// 不是合成车辆：实体必须来自 DecoratorVisualState 的同一装饰物条目。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoasterTarget {
    pub entity: bevy_ecs::entity::Entity,
    pub stair_nodes: [u32; 2],
    pub decorator_id: country_core::resources::walls::decorator::DecoratorId,
    pub decorator_type: u8,
}

/// 原 `raycast_cart` 的已解析碰撞结果消费者（内联于 update@1403baec0
/// 和 create_cart_chain@1403be8xx）。保持目的地断言、类型位集和只取首实体的规则。
/// 调用方负责原射线距离/掩码；本函数不重新筛选或寻找后面的“可用实体”。
fn coaster_target_from_hit(
    hit: &country_core::systems::collision::world::RaycastWorldResult,
    storage: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    visuals: &system_decorator::decorator_visual_state::DecoratorVisualState,
) -> Option<CoasterTarget> {
    use country_core::resources::walls::decorator::DecoratorId;
    if hit.is_miss() || hit.handle[0] as u8 != 4 || hit.handle[1] != 2 {
        return None;
    }
    let id = DecoratorId((hit.handle[0] >> 32) as u32);
    let info = storage.lookup_decorator_info(id)?;
    let stair = info.dst_as_stair();
    // 原先判断 0x38..0x3f/0x3e，再判 <=0x36 与位集；保留原写法，
    // 避免误把相邻枚举族或默认视觉当作可乘车辆。
    let family = info.ty.wrapping_sub(0x38);
    if (family < 8 && family != 6)
        || info.ty > 0x36
        || ((0x0040_0100_0180_0000_u64 >> (info.ty & 63)) & 1) == 0
    {
        return None;
    }
    let entity = *visuals.persistent_entities_of(id.0)?.first()?;
    // Rust Entity 自身保证 generation 非零；原实体切片不扫描后续项。
    if info.ty == 0x38 {
        return None;
    }
    Some(CoasterTarget {
        entity,
        stair_nodes: [stair.dst_wall as u32, (stair.dst_wall >> 32) as u32],
        decorator_id: id,
        decorator_type: info.ty,
    })
}

/// 原空闲 ride 分支 @1403bad37..bb0c5 的候选检测。检测结果仅代表目标资格；
/// 完整 rollercoaster_update 还必须处理确认键、轨道构造和骑乘退出后才能注册。
/// 这里使用现有共享 RaycastWorld，保留原 max_toi=4、mask=0x101。
fn find_rollercoaster_prompt_target(
    mode: country_core::systems::mode_manager::AppMode,
    camera: &country_core::systems::camera_rig::MainCamera,
    ride_in_progress: bool,
    collisions: &country_core::systems::collision::world::RaycastWorld,
    heights: &country_core::resources::terrain_height::TerrainHeightsData,
    storage: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    visuals: &system_decorator::decorator_visual_state::DecoratorVisualState,
) -> Option<CoasterTarget> {
    if !coaster_prompt_context_is_eligible(mode, camera.mode_byte(), ride_in_progress) {
        return None;
    }
    let hit = collisions.raycast_w_terrain_with_max_toi(
        heights,
        camera.eye_position(),
        camera.eye_forward(),
        4.0,
        0x101,
    );
    if hit.toi > 4.0 {
        return None;
    }
    coaster_target_from_hit(&hit, storage, visuals)
}

fn coaster_prompt_context_is_eligible(
    mode: country_core::systems::mode_manager::AppMode,
    camera_mode: u8,
    ride_in_progress: bool,
) -> bool {
    mode == country_core::systems::mode_manager::AppMode::Photomode
        && camera_mode == 3
        && !ride_in_progress
}

#[cfg(test)]
mod original_coaster_target_tests {
    use super::*;
    use bevy_ecs::{event::Events, schedule::IntoScheduleConfigs};
    #[test]
    fn original_sheep_terrain_consumer_keeps_all_state_payloads_and_velocity() {
        use std::cell::Cell;
        use system_critters::{
            components::CritterLocomotion, sheep::sheep_terrain::apply_terrain_follow,
        };
        let cases: Vec<(Vec<u32>, Vec<u32>)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-coaster-sheep-terrain.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 4399);
        for (index, (input, expected)) in cases.into_iter().enumerate() {
            if expected[5] != 0 {
                assert_eq!(input[0], 3);
                continue;
            }
            let f = |i: usize| f32::from_bits(input[i]);
            let calls = Cell::new(0);
            let (y, v, state) = apply_terrain_follow(
                CritterLocomotion(input[0], f(3)),
                f(1),
                glam::Vec3::new(0., f(2), 0.),
                f(4),
                f(5),
                |_, _| {
                    calls.set(calls.get() + 1);
                    f(6)
                },
            );
            assert_eq!(
                vec![
                    y.to_bits(),
                    v.to_bits(),
                    state.0,
                    state.1.to_bits(),
                    calls.get(),
                    0
                ],
                expected,
                "case {index}"
            );
        }
    }

    #[test]
    #[should_panic(expected = "flying sheep cannot follow terrain")]
    fn original_sheep_flying_terrain_branch_keeps_panic() {
        system_critters::sheep::sheep_terrain::apply_terrain_follow(
            system_critters::components::CritterLocomotion::new(3),
            0.,
            glam::Vec3::ZERO,
            0.,
            0.016,
            |_, _| 0.,
        );
    }

    #[test]
    fn original_sheep_unstuck_keeps_hit_height_winter_and_lazy_samples() {
        use std::cell::Cell;
        use system_critters::{
            components::{CritterLocomotion, SheepMoveState},
            sheep::sheep_unstuck::{SheepGroundRecord, apply_unstuck},
        };
        let cases: Vec<(Vec<u32>, Vec<u32>)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-coaster-sheep-unstuck.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 6982);
        for (index, (input, expected)) in cases.into_iter().enumerate() {
            let f = |i: usize| f32::from_bits(input[i]);
            let mut loc = CritterLocomotion(input[0], f(1));
            let mut ground = SheepGroundRecord {
                flag: input[2] as u8,
                h: f(3),
            };
            let mut movement = if input[4] == 0 {
                SheepMoveState::Idle { timeout_s: f(5) }
            } else {
                SheepMoveState::Walking {
                    timeout_s: f(5),
                    goal_change_timeout: 1.,
                    force_reach_goal: false,
                }
            };
            let calls = Cell::new(0);
            apply_unstuck(
                &mut loc,
                &mut ground,
                &mut movement,
                f(6),
                (input[7] != 0).then(|| f(8)),
                input[9] != 0,
                f(10),
                input[11] != 0,
                f(13),
                || {
                    calls.set(calls.get() + 1);
                    f(12)
                },
            );
            let timeout = match movement {
                SheepMoveState::Idle { timeout_s } | SheepMoveState::Walking { timeout_s, .. } => {
                    timeout_s
                }
            };
            let sampled = u32::from(input[7] == 0);
            assert_eq!(
                vec![
                    loc.0,
                    loc.1.to_bits(),
                    u32::from(ground.flag),
                    ground.h.to_bits(),
                    timeout.to_bits(),
                    sampled,
                    sampled,
                    calls.get()
                ],
                expected,
                "case {index}"
            );
        }
    }

    #[test]
    fn actual_sheep_unstuck_retains_session_warmup_and_real_collision_height() {
        use country_core::{
            asset_libraries::ssbo_library::AssetSsboLibrary,
            render::test_device::TestDevice,
            resources::{
                session::{
                    ActiveSession, NewSessionStartedCmd, SessionId, SessionLoadedCmd, SessionMeta,
                },
                terrain_rock_mask::TerrainRockMaskSsbo,
                world_raster::DirtyTilePublisher,
                world_raster_defs::WaterRaster,
            },
            systems::{
                collision::world::{RaycastGeoSource, RaycastMeshData, upsert_collider},
                ui::signifiers::signifier_create::UiSignifierStream,
            },
        };
        use glam::{Quat, Vec3};
        use system_critters::{
            components::{CritterLocomotion, SheepMoveState},
            sheep::sheep_unstuck::{SheepGroundRecord, sheep_unstuck},
        };
        let mut app = bevy_app::App::new();
        let buffers = AssetSsboLibrary::new();
        buffers.add_new::<TerrainRockMaskSsbo>(&TestDevice(702));
        app.insert_resource(buffers);
        app.insert_resource(GameTime {
            delta: 0.016,
            ..Default::default()
        });
        app.insert_resource(TerrainHeightsData::<
            country_core::resources::terrain_height::TerrainHeightSsbo,
        >::new());
        app.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        app.insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()));
        app.init_resource::<UiSignifierStream>();
        app.add_event::<NewSessionStartedCmd>();
        app.add_event::<SessionLoadedCmd>();
        let mut raycast = RaycastWorld::default();
        let mut collider_instances = std::collections::HashMap::new();
        upsert_collider(
            &mut raycast,
            &mut collider_instances,
            77,
            RaycastGeoSource::from_slot(0),
            std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(0.5), Vec3::ZERO)),
            Vec3::ZERO,
            Quat::IDENTITY,
        );
        raycast.flush_colliders();
        let hit = raycast.legacy_spherecast(Vec3::new(0., 1.5, 0.), Vec3::new(-0., -1., -0.), 1.);
        assert!(hit.is_hit());
        let expected_height = hit.position.y;
        app.insert_resource(raycast);
        let entity = app
            .world_mut()
            .spawn((
                Transform {
                    translation: Vec3::new(0., 2., 0.),
                    ..Default::default()
                },
                CritterLocomotion::new(2),
                SheepGroundRecord { flag: 1, h: 12. },
                Sheep {
                    behavior_state: 0,
                    goal_timer: 0.,
                    pet_phase: 0,
                    pet_progress: 0.,
                    pet_target: 0,
                    move_state: SheepMoveState::Idle { timeout_s: 3. },
                    amplitude: 0.,
                    control_flag: 0,
                },
            ))
            .id();
        app.add_systems(bevy_app::Update, sheep_unstuck);
        app.update();
        assert_eq!(app.world().get::<CritterLocomotion>(entity).unwrap().0, 2);
        app.update();
        assert_eq!(
            *app.world().get::<CritterLocomotion>(entity).unwrap(),
            CritterLocomotion(1, expected_height)
        );
        assert_eq!(
            app.world().get::<SheepGroundRecord>(entity).unwrap().flag,
            0
        );
        assert!(matches!(
            app.world().get::<Sheep>(entity).unwrap().move_state,
            SheepMoveState::Idle { timeout_s: 0. }
        ));
        app.world_mut()
            .entity_mut(entity)
            .insert(CritterLocomotion::new(2));
        app.world_mut()
            .resource_mut::<Events<NewSessionStartedCmd>>()
            .send(NewSessionStartedCmd);
        app.update();
        assert_eq!(app.world().get::<CritterLocomotion>(entity).unwrap().0, 2);
        // 下一帧新到达Loaded，避免把Bevy事件两帧保留期误当成reader提前消费。
        app.world_mut()
            .resource_mut::<Events<SessionLoadedCmd>>()
            .send(SessionLoadedCmd);
        for _ in 0..2 {
            app.update();
            assert_eq!(app.world().get::<CritterLocomotion>(entity).unwrap().0, 2);
        }
        app.update();
        assert_eq!(app.world().get::<CritterLocomotion>(entity).unwrap().0, 1);
    }
    use country_core::{
        resources::walls::{
            decorator::{DecoratorId, DecoratorInfo},
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        },
        systems::{collision::world::RaycastWorldResult, mode_manager::AppMode},
    };

    #[test]
    fn recovered_ride_enters_updates_real_entities_and_restores_on_cancel() {
        exercise_recovered_ride(false);
    }

    #[test]
    fn recovered_ride_exits_after_real_level_unload_already_despawned_entities() {
        exercise_recovered_ride(true);
    }

    fn exercise_recovered_ride(unload_first: bool) {
        // `PROMPT_ROLLERCOASTER` is a process-wide flag written by every `rollercoaster_update`;
        // the two ride tests must not interleave their updates around its assertions.
        static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _serial = SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        use country_core::{
            render::nani::{
                gpu_world::{GpuWorldSubsetKind, InstanceDataMarker},
                instance_data::VelocityOnlyInstanceData,
            },
            resources::{
                audio_resources::DisabledStudio,
                clutter_meshes::{ClutterMesh, ClutterMeshes},
                stairs::stairs_assembly_params::{
                    AdjustedStairEdge, AlignKind, PlayermadeStairsAssemblyParams, StairNodeAnchor,
                },
            },
            systems::{
                camera_rig::main_camera::CameraMode,
                collision::world::{RaycastGeoSource, RaycastMeshData, upsert_collider},
            },
        };
        use glam::{Quat, Vec2, Vec3};
        let mut app = bevy_app::App::new();
        // 这组测试执行真实消费者，设备仅提供CPU映射页；不声称GPU/窗口验收。
        let buffers = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        buffers.add_new::<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>(
            &country_core::render::test_device::TestDevice(701),
        );
        app.insert_resource(buffers);
        app.insert_resource(
            country_core::resources::world_raster_defs::WaterRaster::new(
                country_core::resources::world_raster::DirtyTilePublisher::unique(),
            ),
        );
        app.insert_resource(
            country_core::resources::world_raster_defs::GardenRaster::new(
                country_core::resources::world_raster::DirtyTilePublisher::unique(),
            ),
        );
        app.init_resource::<country_core::resources::walls::public_walls::PublicWalls>();
        app.init_resource::<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>();
        let audio = AudioCore::<DisabledStudio> {
            studio: None,
            banks: Default::default(),
        };
        app.insert_resource(TriggerAudioState::new(&audio));
        app.insert_resource(audio);
        let mut camera = MainCamera::new(1.5);
        camera.set_mode_byte(CameraMode::WALK);
        camera.set_camera_matrices(glam::Mat4::from_translation(Vec3::new(0.0, 0.25, 0.0)));
        app.insert_resource(camera);
        app.insert_resource(country_core::input::action_set::ActionSetStates::default());
        app.insert_resource(VermeerState {
            freeze_camera: false,
            ..Default::default()
        });
        app.insert_resource(GameTime {
            delta: 0.02,
            ..Default::default()
        });
        app.insert_resource(TerrainHeightsData::<
            country_core::resources::terrain_height::TerrainHeightSsbo,
        >::new());
        app.insert_resource(AppMode::Photomode);
        app.insert_resource(Modifiers::default());
        app.add_event::<PlaySheepPet>();
        app.add_event::<PlaySheepReactSound>();
        let mut stairs = StairsState::default();
        stairs.add_edge(11, 12);
        app.insert_resource(stairs);
        let mut requests = PlayermadeStairsAssemblyRequests::default();
        let points = vec![
            Vec3::new(-3.0, 0.0, -2.0),
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(6.0, 0.0, -2.0),
            Vec3::new(12.0, 0.0, -2.0),
        ];
        let curve = ::utils::curve::Curve2::try_new_from_points(
            points.iter().map(|p| Vec2::new(p.x, p.z)).collect(),
            false,
        )
        .unwrap();
        requests.requests.push(PlayermadeStairsAssemblyParams {
            curve,
            heights: vec![0.0; 4],
            widths: vec![1.0; 4],
            tangents: vec![Vec2::X; 4],
            auxiliary_samples: vec![0.0; 4],
            interpolated_curve: Some(
                ::utils::curve::Curve3::try_new_from_points(points, false).unwrap(),
            ),
            nodes: [(StairNodeAnchor::Terrain { height: 0.0 }, 0); 2],
            terrain_contacts: Default::default(),
            segment: country_core::resources::stairs::stairs_state::StairSegment::new(
                DecoratorId(11),
                DecoratorId(12),
            ),
            sub_segments: Default::default(),
            dirty: false,
        });
        requests.alignments.add_edge(
            DecoratorId(11),
            DecoratorId(12),
            AlignKind {
                tag: 1,
                ..Default::default()
            },
        );
        requests.alignments.add_edge(
            DecoratorId(12),
            DecoratorId(11),
            AlignKind {
                tag: 1,
                ..Default::default()
            },
        );
        requests.adjusted_edges.add_edge(
            DecoratorId(11),
            DecoratorId(12),
            AdjustedStairEdge {
                position: Vec2::ZERO,
                outgoing_dir: Vec2::NEG_X,
            },
        );
        app.insert_resource(requests);
        let fixed = std::array::from_fn(|_| {
            let mut mesh = ClutterMesh::omitted_in_demo("fixture/cart", 0, None, false, false);
            mesh.bounds =
                ::utils::geometry::rectangle::Rectangle3d::new(Vec3::ZERO, Vec3::ONE, Vec2::X);
            mesh
        });
        app.insert_resource(ClutterMeshes::from_parts(fixed, Vec::new()));
        let mut storage = DecoratorStorage::default();
        let mut visuals = system_decorator::decorator_visual_state::DecoratorVisualState::default();
        let mut collisions = RaycastWorld::new();
        let mut colliders = std::collections::HashMap::new();
        let mut originals = Vec::new();
        for (index, x) in [0.0, 2.0].into_iter().enumerate() {
            let id = DecoratorId(index as u32 + 7);
            let transform = Transform {
                translation: Vec3::new(x, 0.0, -2.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            };
            let entity = app
                .world_mut()
                .spawn((
                    transform,
                    InstanceKey {
                        id: index as u32,
                        index: index as u32,
                        kind: GpuWorldSubsetKind::VelocityOnly,
                    },
                ))
                .id();
            originals.push((entity, transform));
            visuals.persistent.add_decorator_visual_entity(id.0, entity);
            let mut info = DecoratorInfo::default();
            info.dst_wall = (12_u64 << 32) | 11;
            info.u20 = 4 << 16;
            info.ty = 23;
            storage
                .decorator_to_storage
                .insert(id, DecoratorStorageAddr::Stair);
            storage
                .get_or_insert_default(DecoratorStorageAddr::Stair)
                .insert(id, info);
            upsert_collider(
                &mut collisions,
                &mut colliders,
                id,
                RaycastGeoSource::decorator_interaction(id, DecoratorStorageAddr::Stair, 0),
                std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                transform.translation,
                transform.rotation,
            );
        }
        collisions.flush_colliders();
        app.insert_resource(collisions);
        app.insert_resource(storage);
        app.insert_resource(visuals);
        let umbrella = app.world_mut().spawn(SheepAccessory::default()).id();
        let floatie = app.world_mut().spawn(SheepAccessory::default()).id();
        let sheep_transform = Transform {
            translation: Vec3::new(50.0, 10.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(0.65),
        };
        let sheep = app
            .world_mut()
            .spawn((
                sheep_transform,
                system_critters::components::CritterLocomotion(1, -3.0),
                system_critters::components::Velocity(Vec3::new(1.0, 0.25, 0.0)),
                Sheep {
                    behavior_state: 0,
                    goal_timer: 0.0,
                    pet_phase: 0,
                    pet_progress: 0.0,
                    pet_target: 123,
                    move_state: country_core::resources::critters::sheep::SheepMoveState::Idle {
                        timeout_s: 1.0,
                    },
                    amplitude: 0.0,
                    control_flag: 0,
                },
                UmbrellaHolder {
                    spawned: Some(umbrella),
                    ..Default::default()
                },
                FloatieHolder {
                    spawned: Some(floatie),
                    ..Default::default()
                },
            ))
            .id();
        #[derive(bevy_ecs::resource::Resource, Default)]
        struct BeforeTerrain(Option<(Transform, system_critters::components::Velocity)>);
        app.init_resource::<BeforeTerrain>();
        // 原 set9 ride 位于 set36 的 terrain/move 之前。保留乘坐之后的原修改，
        // 不发明 Ride 标记、Without 过滤或改变系统排序来强行固定羊的位置。
        app.add_systems(
            bevy_app::Update,
            (
                rollercoaster_update::<DisabledStudio>,
                |query: Query<
                    (&Transform, &system_critters::components::Velocity),
                    With<Sheep>,
                >,
                 mut snapshot: ResMut<BeforeTerrain>| {
                    snapshot.0 = query.single().ok().map(|(t, v)| (*t, *v));
                },
                system_critters::sheep::sheep_terrain::sheep_walk_on_terrain,
                system_critters::shared::move_critters,
            )
                .chain_ignore_deferred(),
        );
        // Idle photomode frame with the walk camera on the real stair carts: the original ride
        // system itself raises `PROMPT_ROLLERCOASTER` (the flag the photomode tail turns into
        // `DisplayRollercoasterPrompt`) — and only while no ride is running.
        app.update();
        assert!(
            country_core::systems::main_camera::PROMPT_ROLLERCOASTER
                .load(std::sync::atomic::Ordering::Relaxed),
            "an eligible cart in photomode must raise the rollercoaster prompt"
        );
        app.world_mut()
            .resource_mut::<country_core::input::action_set::ActionSetStates>()
            .digital[19] = country_core::input::action_set::DigitalActionState {
            phase: country_core::input::action_set::DigitalActionPhase::JustPressed,
            timer_seconds: 0.0,
        };
        app.update();
        // The original raises the flag as soon as a target is found, before it checks the
        // confirm key, so the starting frame still carries the prompt.
        assert!(
            country_core::systems::main_camera::PROMPT_ROLLERCOASTER
                .load(std::sync::atomic::Ordering::Relaxed),
            "the frame that starts the ride still carries the prompt"
        );
        let temporary: Vec<Entity> = app
            .world_mut()
            .query_filtered::<Entity, With<InstanceDataMarker<VelocityOnlyInstanceData>>>()
            .iter(app.world())
            .collect();
        assert_eq!(
            temporary.len(),
            2,
            "both neighboring real carts must create velocity entities"
        );
        assert!(app.world().resource::<TriggerAudioState<country_core::resources::audio_resources::DisabledEvent>>().in_forest_state);
        assert!(
            app.world()
                .get::<bevy_ecs::entity_disabling::Disabled>(umbrella)
                .is_some()
        );
        assert!(
            app.world()
                .get::<bevy_ecs::entity_disabling::Disabled>(floatie)
                .is_some()
        );
        app.world_mut()
            .resource_mut::<country_core::input::action_set::ActionSetStates>()
            .digital[19] = Default::default();
        // 图形宿主负责VelocityOnly注册；此CPU验收以实际InstanceKey查询边界插入。
        for (index, entity) in temporary.iter().enumerate() {
            app.world_mut().entity_mut(*entity).insert(InstanceKey {
                id: 100 + index as u32,
                index: 100 + index as u32,
                kind: GpuWorldSubsetKind::VelocityOnly,
            });
        }
        app.update();
        // A running ride is "not idle": the original clears the prompt and finds no new target.
        assert!(
            !country_core::systems::main_camera::PROMPT_ROLLERCOASTER
                .load(std::sync::atomic::Ordering::Relaxed),
            "no prompt while a ride is in progress"
        );
        assert_ne!(
            *app.world().get::<Transform>(originals[0].0).unwrap(),
            originals[0].1
        );
        assert_ne!(
            *app.world().get::<Transform>(sheep).unwrap(),
            sheep_transform
        );
        let (before, old_velocity) = app.world().resource::<BeforeTerrain>().0.unwrap();
        let state = *app
            .world()
            .get::<system_critters::components::CritterLocomotion>(sheep)
            .unwrap();
        let (_, expected_v, expected_state) =
            system_critters::sheep::sheep_terrain::apply_terrain_follow(
                state,
                0.0,
                before.translation,
                old_velocity.0.y,
                0.02,
                |_, _| 0.0,
            );
        assert_eq!(
            *app.world()
                .get::<system_critters::components::CritterLocomotion>(sheep)
                .unwrap(),
            expected_state
        );
        let velocity = app
            .world()
            .get::<system_critters::components::Velocity>(sheep)
            .unwrap();
        assert_eq!(
            velocity.0.y.to_bits(),
            expected_v.to_bits(),
            "terrain writes the canonical Velocity"
        );
        let base_y = if before.translation.y > 0.0 {
            before.translation.y
        } else {
            0.0
        };
        let expected =
            Vec3::new(before.translation.x, base_y, before.translation.z) + velocity.0 * 0.02;
        assert_eq!(
            app.world().get::<Transform>(sheep).unwrap().translation,
            expected,
            "real post-ride move query must still advance the passenger without an invented exclusion"
        );
        assert!(app.world().resource::<MainCamera>().eye_position().y > 1.0);
        if unload_first {
            use bevy_ecs::system::RunSystemOnce;
            // 执行真实OnExit消费者，保留Local骑乘状态到随后Update，再以原mode门退出。
            app.world_mut()
                .run_system_once(country_core::startup::clearing::despawn_glade_entities)
                .unwrap();
            for entity in &temporary {
                assert!(app.world().get_entity(*entity).is_err());
            }
            *app.world_mut().resource_mut::<AppMode>() = AppMode::LoadGame;
        } else {
            app.world_mut()
                .resource_mut::<country_core::input::action_set::ActionSetStates>()
                .digital[18] = country_core::input::action_set::DigitalActionState {
                phase: country_core::input::action_set::DigitalActionPhase::JustPressed,
                timer_seconds: 0.0,
            };
        }
        app.update();
        for (entity, transform) in originals {
            if unload_first {
                assert!(app.world().get_entity(entity).is_err());
            } else {
                assert_eq!(*app.world().get::<Transform>(entity).unwrap(), transform);
            }
        }
        for entity in temporary {
            assert!(app.world().get_entity(entity).is_err());
        }
        assert!(
            app.world()
                .get::<bevy_ecs::entity_disabling::Disabled>(umbrella)
                .is_none()
        );
        assert!(
            app.world()
                .get::<bevy_ecs::entity_disabling::Disabled>(floatie)
                .is_none()
        );
        assert!(!app.world().resource::<TriggerAudioState<country_core::resources::audio_resources::DisabledEvent>>().in_forest_state);
    }

    #[test]
    fn original_neighbor_rays_preserve_side_order_and_general_quaternion_rotation() {
        type Case = (Vec<u32>, Vec<u32>, Vec<u32>, Vec<Vec<u32>>);
        let cases: Vec<Case> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-neighbor-rays.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 45);
        for (index, (q, p, extent, expected)) in cases.into_iter().enumerate() {
            let transform = country_core::utils::components::transform::Transform {
                rotation: glam::Quat::from_array(
                    <[u32; 4]>::try_from(q).unwrap().map(f32::from_bits),
                ),
                translation: glam::Vec3::from_array(
                    <[u32; 3]>::try_from(p).unwrap().map(f32::from_bits),
                ),
                scale: glam::Vec3::splat(37.0),
            };
            let extent =
                glam::Vec3::from_array(<[u32; 3]>::try_from(extent).unwrap().map(f32::from_bits));
            for (side, ((origin, direction), expected)) in coaster_neighbor_rays(transform, extent)
                .into_iter()
                .zip(expected)
                .enumerate()
            {
                let actual: Vec<u32> = origin
                    .to_array()
                    .into_iter()
                    .chain(direction.to_array())
                    .map(f32::to_bits)
                    .collect();
                assert_eq!(actual, expected, "case {index} side {side}");
            }
        }
    }

    #[test]
    fn original_cart_pose_preserves_lean_arc_and_mesh_orientation() {
        type Case = (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>);
        let cases: Vec<Case> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-pose.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 27);
        for (index, (look, lean, expected, visual)) in cases.into_iter().enumerate() {
            let look =
                glam::Quat::from_array(<[u32; 4]>::try_from(look).unwrap().map(f32::from_bits));
            let lean = glam::Vec3::new(f32::from_bits(lean[0]), f32::from_bits(lean[1]), 0.0);
            let (rotation, mesh) = coaster_cart_rotations(look, lean);
            assert_eq!(
                rotation.to_array().map(f32::to_bits).to_vec(),
                expected,
                "case {index}: lean pose"
            );
            assert_eq!(
                mesh.to_array().map(f32::to_bits).to_vec(),
                visual,
                "case {index}: mesh pose"
            );
        }
    }

    #[test]
    fn original_look_at_reuses_locked_dolly_without_direction_shortcuts() {
        let cases: Vec<(Vec<u32>, Vec<u32>)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-look-at.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 120);
        for (index, (direction, expected)) in cases.into_iter().enumerate() {
            let direction = glam::Vec3::from_array(
                <[u32; 3]>::try_from(direction).unwrap().map(f32::from_bits),
            );
            let q = coaster_look_at(direction);
            assert_eq!(
                q.to_array().map(f32::to_bits).to_vec(),
                expected,
                "case {index}"
            );
        }
    }

    #[test]
    fn original_speed_step_keeps_gravity_controls_drag_and_tail_slowdown() {
        type Case = (u32, u32, u32, u32, u8, u32, usize, u32, u32, u8);
        // The original calls the platform CRT `expf`; Rust's `f32::exp` calls the
        // same platform libm. Apple's libm differs from the Windows UCRT by 1 ULP
        // on 9 of these 5400 inputs (all in the speed column), so each platform is
        // compared against original-binary results produced with its own `expf`.
        #[cfg(windows)]
        const FIXTURE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-speed.windows-ucrt.ron"
        ));
        #[cfg(not(windows))]
        const FIXTURE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-speed.ron"
        ));
        let cases: Vec<Case> = ron::from_str(FIXTURE).unwrap();
        assert_eq!(cases.len(), 5400);
        for (index, (speed, tangent, drive, brake, flag, t, count, new_speed, new_t, bottom)) in
            cases.into_iter().enumerate()
        {
            let mut state = CoasterProgress {
                t: f32::from_bits(t),
                speed: f32::from_bits(speed),
                accumulator: 0.0,
            };
            let braking = state.step(
                f32::from_bits(tangent),
                f32::from_bits(drive),
                f32::from_bits(brake),
                flag,
                count,
                0.0,
                50.0,
            );
            assert_eq!(state.speed.to_bits(), new_speed, "case {index}: speed");
            assert_eq!(state.t.to_bits(), new_t, "case {index}: t");
            assert_eq!(braking, bottom != 0, "case {index}: bottom");
        }
    }

    #[test]
    fn original_cart_substep_preserves_all_inertia_channels_and_crossing() {
        type Input = (
            u32,
            u32,
            u32,
            u32,
            Vec<u32>,
            Vec<u32>,
            Vec<u32>,
            u32,
            Vec<u32>,
            Vec<u32>,
            Vec<u32>,
            u32,
            Vec<u32>,
        );
        type Case = (Input, (Vec<u32>, Vec<u32>, Vec<u32>, u32, u8));
        let mut rows: Vec<Case> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-cart-dynamics.ron"
        )))
        .unwrap();
        assert_eq!(rows.len(), 1728);
        let nonfinite: Vec<Case> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-cart-dynamics-nonfinite.ron"
        )))
        .unwrap();
        assert_eq!(nonfinite.len(), 120);
        rows.extend(nonfinite);
        let vec3 = |bits: Vec<u32>| {
            glam::Vec3::from_array(<[u32; 3]>::try_from(bits).unwrap().map(f32::from_bits))
        };
        let mut canonical_nan_sign_only = 0;
        let mut compare = |actual: Vec<u32>, expected: Vec<u32>, index: usize, channel: &str| {
            assert_eq!(actual.len(), expected.len());
            for (lane, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
                // 单独原指令实验已证：SSE invalid新生NaN为ffc00000，ARM为
                // 7fc00000。仅补集中这对canonical符号差异作为架构边界计数；
                // 既有NaN payload与所有有限/无穷值仍必须逐位一致。
                if index >= 1728
                    && actual != expected
                    && actual & 0x7fff_ffff == 0x7fc0_0000
                    && expected & 0x7fff_ffff == 0x7fc0_0000
                {
                    canonical_nan_sign_only += 1;
                } else {
                    assert_eq!(actual, expected, "case {index} {channel} lane {lane}");
                }
            }
        };
        for (
            index,
            (
                (
                    selected,
                    sheep,
                    old_speed,
                    speed,
                    old_tan,
                    tan,
                    look,
                    steer,
                    old_j,
                    old_bc,
                    old_a,
                    old_peak,
                    shape,
                ),
                (j, bc, a, peak, cross),
            ),
        ) in rows.into_iter().enumerate()
        {
            let mut state = CartMotion {
                world_inertia: vec3(old_j),
                lean: vec3(old_bc),
                local_inertia: vec3(old_a),
                peak: f32::from_bits(old_peak),
            };
            let crossed = state.step(
                selected != 0,
                (sheep != 0).then(|| vec3(shape)),
                f32::from_bits(old_speed),
                f32::from_bits(speed),
                vec3(old_tan),
                vec3(tan),
                glam::Quat::from_array(<[u32; 4]>::try_from(look).unwrap().map(f32::from_bits)),
                f32::from_bits(steer),
            );
            compare(
                state.world_inertia.to_array().map(f32::to_bits).to_vec(),
                j,
                index,
                "J",
            );
            compare(
                state.lean.to_array().map(f32::to_bits).to_vec(),
                bc,
                index,
                "BC",
            );
            compare(
                state.local_inertia.to_array().map(f32::to_bits).to_vec(),
                a,
                index,
                "A",
            );
            compare(vec![state.peak.to_bits()], vec![peak], index, "peak");
            assert_eq!(u8::from(crossed), cross, "case {index}: crossing");
        }
        eprintln!("nonfinite canonical NaN sign-only lanes: {canonical_nan_sign_only}");
    }

    #[test]
    fn original_resolved_target_gate_keeps_all_type_values_and_first_visual_only() {
        type Case = (u8, u8, bool, u32, u8, u64, bool, u8, u8, bool);
        let cases: Vec<Case> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-target.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 275);
        let mut compared = 0;
        for (mode, camera, active, miss, source, subtag, present, ty, visual, expected) in cases {
            // 原探针另覆盖generation=0的无效Entity字节；安全Rust Entity不能构造
            // 该值，故不把非法ABI字节伪造成当前权威资源。
            if visual == 2 {
                continue;
            }
            let mut storage = DecoratorStorage::default();
            let mut info = DecoratorInfo::default();
            info.dst_wall = (12_u64 << 32) | 11;
            info.u20 = 4 << 16;
            info.ty = ty;
            if present {
                storage
                    .decorator_to_storage
                    .insert(DecoratorId(7), DecoratorStorageAddr::Stair);
                storage
                    .get_or_insert_default(DecoratorStorageAddr::Stair)
                    .insert(DecoratorId(7), info);
            }
            let mut visuals =
                system_decorator::decorator_visual_state::DecoratorVisualState::default();
            let first = bevy_ecs::entity::Entity::from_raw(42);
            if visual == 1 {
                visuals.persistent.add_decorator_visual_entity(7, first);
                visuals
                    .persistent
                    .add_decorator_visual_entity(7, bevy_ecs::entity::Entity::from_raw(99));
            }
            let hit = RaycastWorldResult {
                handle: [(7_u64 << 32) | u64::from(source), subtag, 0],
                discriminant: miss,
                ..RaycastWorldResult::miss()
            };
            let mode = if mode == 19 {
                AppMode::Photomode
            } else {
                AppMode::default()
            };
            let actual = coaster_prompt_context_is_eligible(mode, camera, active)
                .then(|| coaster_target_from_hit(&hit, &storage, &visuals))
                .flatten();
            assert_eq!(
                actual.is_some(),
                expected,
                "camera={camera} active={active} ty={ty} source={source} subtag={subtag}"
            );
            if let Some(target) = actual {
                assert_eq!(
                    target,
                    CoasterTarget {
                        entity: first,
                        stair_nodes: [11, 12],
                        decorator_id: DecoratorId(7),
                        decorator_type: ty,
                    }
                );
            }
            compared += 1;
        }
        assert_eq!(compared, 274);
    }

    #[test]
    fn original_initializer_preserves_sampling_count_distance_order_and_first_minimum() {
        let cases: Vec<(usize, f32, (f32, f32, f32), u32)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-initial-t.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 120);
        for (count, max_t, (x, y, z), expected_bits) in cases {
            let spline = ::utils::natural_cubic::Spline3D {
                segments: vec![::utils::natural_cubic::SplineSegment3D {
                    start_t: 0.0,
                    coefficients: [
                        glam::Vec3::ZERO,
                        glam::Vec3::new(3., 0., 0.),
                        glam::Vec3::ZERO,
                        glam::Vec3::ZERO,
                    ],
                }],
                min_t: 0.0,
                max_t,
                periodic: false,
            };
            assert_eq!(((max_t * 5.0).ceil() as usize).max(2), count);
            assert_eq!(
                initial_coaster_parameter(&spline, glam::Vec3::new(x, y, z)).to_bits(),
                expected_bits,
                "max_t={max_t} position=({x},{y},{z})"
            );
        }
    }

    #[test]
    fn original_audio_constructor_keeps_selected_pools_call_order_and_low_volume_failure() {
        use country_core::resources::{
            audio_instance::PlaybackState,
            audio_resources::{AudioCore, DisabledVca, EventInstance, ListenerAttributes, Studio},
        };
        use std::{
            convert::Infallible,
            sync::{Arc, Mutex},
        };
        type Calls = Arc<Mutex<Vec<(u8, String, u32)>>>;
        struct Event(Calls);
        impl EventInstance for Event {
            type Error = Infallible;
            fn start(&mut self) -> std::result::Result<(), Self::Error> {
                self.0.lock().unwrap().push((3, String::new(), 0));
                Ok(())
            }
            fn stop(&mut self, _: i32) -> std::result::Result<(), Self::Error> {
                Ok(())
            }
            fn get_playback_state(&self) -> std::result::Result<PlaybackState, Self::Error> {
                Ok(PlaybackState::Stopped)
            }
            fn set_volume(&mut self, value: f32) -> std::result::Result<(), Self::Error> {
                self.0
                    .lock()
                    .unwrap()
                    .push((1, String::new(), value.to_bits()));
                Ok(())
            }
            fn set_parameter_by_name(
                &mut self,
                name: &str,
                value: f32,
            ) -> std::result::Result<(), Self::Error> {
                self.0
                    .lock()
                    .unwrap()
                    .push((2, name.into(), value.to_bits()));
                Ok(())
            }
            fn has_parameter(&self, _: &str) -> std::result::Result<bool, Self::Error> {
                Ok(false)
            }
            fn set_3d_attributes(
                &mut self,
                _: [f32; 3],
                _: [f32; 3],
                _: [f32; 3],
                _: [f32; 3],
            ) -> std::result::Result<(), Self::Error> {
                Ok(())
            }
            fn get_timeline_position(&self) -> std::result::Result<i32, Self::Error> {
                Ok(0)
            }
            fn pause(&mut self, _: bool) -> std::result::Result<(), Self::Error> {
                Ok(())
            }
            fn install_beat_callback(&mut self) -> std::result::Result<(), Self::Error> {
                Ok(())
            }
        }
        struct Backend(Calls);
        impl Studio for Backend {
            type Error = Infallible;
            type Event = Event;
            type Vca = DisabledVca;
            fn load_bank_file(&mut self, _: &str) -> std::result::Result<(), Self::Error> {
                Ok(())
            }
            fn load_sample_data(&mut self) -> std::result::Result<(), Self::Error> {
                Ok(())
            }
            fn get_vca(&self, _: &str) -> std::result::Result<Self::Vca, Self::Error> {
                panic!("coaster does not request VCA")
            }
            fn set_listener_attributes(
                &mut self,
                _: ListenerAttributes,
            ) -> std::result::Result<(), Self::Error> {
                Ok(())
            }
            fn update(&mut self) -> std::result::Result<(), Self::Error> {
                Ok(())
            }
            fn set_parameter_by_name(
                &self,
                _: &str,
                _: f32,
            ) -> std::result::Result<(), Self::Error> {
                Ok(())
            }
            fn create_event_instance(
                &self,
                name: &str,
            ) -> std::result::Result<Option<Self::Event>, Self::Error> {
                self.0.lock().unwrap().push((0, name.into(), 0));
                Ok(Some(Event(self.0.clone())))
            }
        }
        type CartAudio = ([bool; 4], [Option<usize>; 3]);
        type Case = (usize, usize, Vec<CartAudio>, Vec<(u8, String, u32)>);
        let cases: Vec<Case> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-audio-init.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 3);
        for (count, selected, expected_carts, expected_calls) in cases {
            let calls = Calls::default();
            let core = AudioCore {
                studio: Some(Backend(calls.clone())),
                banks: Default::default(),
            };
            let mut states = Vec::new();
            let mut carts = Vec::new();
            for index in 0..count {
                let audio = CoasterAudioState::new(&core, index, selected);
                carts.push((
                    [
                        audio.timberframe.is_some(),
                        audio.height.is_some(),
                        audio.overlap.is_some(),
                        audio.water.is_some(),
                    ],
                    [
                        audio.chimney.as_ref().map(Vec::len),
                        audio.fence.as_ref().map(Vec::len),
                        audio.lantern.as_ref().map(Vec::len),
                    ],
                ));
                states.push(audio);
            }
            assert_eq!(carts, expected_carts);
            let actual = calls.lock().unwrap();
            assert_eq!(actual.len(), expected_calls.len());
            for (index, (actual, expected)) in actual.iter().zip(&expected_calls).enumerate() {
                assert_eq!(
                    actual, expected,
                    "cart_count={count} selected={selected} call={index}"
                );
            }
        }
    }

    #[test]
    fn original_audio_frame_keeps_all_parameters_pools_and_water_threshold() {
        use country_core::resources::{
            audio_instance::PlaybackState,
            audio_resources::{AudioInstance, EventInstance},
        };
        use std::{
            convert::Infallible,
            sync::{Arc, Mutex},
        };
        type Call = (u8, usize, String, Vec<u32>);
        type Calls = Arc<Mutex<Vec<Call>>>;
        struct Event {
            id: usize,
            calls: Calls,
            playing: u8,
        }
        impl EventInstance for Event {
            type Error = Infallible;
            fn start(&mut self) -> std::result::Result<(), Self::Error> {
                self.calls
                    .lock()
                    .unwrap()
                    .push((3, self.id, String::new(), Vec::new()));
                Ok(())
            }
            fn stop(&mut self, _: i32) -> std::result::Result<(), Self::Error> {
                self.calls
                    .lock()
                    .unwrap()
                    .push((5, self.id, String::new(), Vec::new()));
                Ok(())
            }
            fn get_playback_state(&self) -> std::result::Result<PlaybackState, Self::Error> {
                Ok(
                    if self.id >= 30 && (self.playing >> (self.id - 30)) & 1 != 0 {
                        PlaybackState::Playing
                    } else {
                        PlaybackState::Stopped
                    },
                )
            }
            fn set_volume(&mut self, v: f32) -> std::result::Result<(), Self::Error> {
                self.calls
                    .lock()
                    .unwrap()
                    .push((1, self.id, String::new(), vec![v.to_bits()]));
                Ok(())
            }
            fn set_parameter_by_name(
                &mut self,
                n: &str,
                v: f32,
            ) -> std::result::Result<(), Self::Error> {
                self.calls
                    .lock()
                    .unwrap()
                    .push((2, self.id, n.into(), vec![v.to_bits()]));
                Ok(())
            }
            fn has_parameter(&self, _: &str) -> std::result::Result<bool, Self::Error> {
                Ok(true)
            }
            fn set_3d_attributes(
                &mut self,
                p: [f32; 3],
                _: [f32; 3],
                _: [f32; 3],
                _: [f32; 3],
            ) -> std::result::Result<(), Self::Error> {
                self.calls.lock().unwrap().push((
                    4,
                    self.id,
                    String::new(),
                    p.map(f32::to_bits).to_vec(),
                ));
                Ok(())
            }
            fn get_timeline_position(&self) -> std::result::Result<i32, Self::Error> {
                Ok(0)
            }
            fn pause(&mut self, _: bool) -> std::result::Result<(), Self::Error> {
                panic!("coaster frame does not pause")
            }
            fn install_beat_callback(&mut self) -> std::result::Result<(), Self::Error> {
                panic!("coaster frame does not install beats")
            }
        }
        type Input = (Vec<u32>, Vec<u32>, u32, bool, bool, u32, u8);
        type Output = (u32, u32, usize, Vec<Call>);
        let cases: Vec<(Input, Output)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-rollercoaster-audio-frame.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 216);
        for (
            index,
            (
                (ax, position, speed, cross, bottom, timer, playing),
                (peak, new_timer, wheel, expected),
            ),
        ) in cases.into_iter().enumerate()
        {
            let calls = Calls::default();
            let make = |id| CoasterAudioInstance {
                instance: AudioInstance::active(
                    Event {
                        id,
                        calls: calls.clone(),
                        playing,
                    },
                    "fixture/coaster",
                ),
                volume: 0.7,
            };
            let mut cart = CoasterCart {
                original_transform: Transform::identity(),
                audio: CoasterAudioState {
                    timberframe: Some(make(0)),
                    height: Some(make(1)),
                    overlap: Some(make(2)),
                    water: Some(make(3)),
                    chimney: Some((10..20).map(&make).collect()),
                    fence: Some((20..30).map(&make).collect()),
                    lantern: Some((30..36).map(&make).collect()),
                },
                wheel_index: 2,
                wheel_timer: f32::from_bits(timer),
                velocity_entity: Entity::from_raw(99),
                sheep: None,
                decorator_id: DecoratorId(7),
                motion: CartMotion {
                    local_inertia: glam::Vec3::new(f32::from_bits(ax[0]), 0.0, 0.0),
                    peak: 0.08,
                    ..Default::default()
                },
            };
            let position =
                glam::Vec3::from_array(<[u32; 3]>::try_from(position).unwrap().map(f32::from_bits));
            cart.update_audio(position, f32::from_bits(speed), 0.016, cross, bottom);
            assert_eq!(cart.motion.peak.to_bits(), peak, "case {index}: peak");
            assert_eq!(cart.wheel_timer.to_bits(), new_timer, "case {index}: timer");
            assert_eq!(cart.wheel_index, wheel, "case {index}: index");
            // 原探针is_playing记录的是包装器调用，当前AudioInstance复用后端时也
            // 会在start_if_not_playing内查询；设备可观察输出与池选择逐项比较。
            let expected: Vec<_> = expected.into_iter().filter(|call| call.0 != 6).collect();
            assert_eq!(
                *calls.lock().unwrap(),
                expected,
                "case {index}: ordered device outputs"
            );
        }
    }

    #[test]
    fn original_audio_ordered_threshold_rejects_nan_before_instantiation() {
        #[derive(Default)]
        struct Audio {
            calls: usize,
        }
        impl CoasterAudio for Audio {
            type Instance = Vec<(String, f32)>;
            fn instantiate_audio(&mut self) -> Result<Self::Instance> {
                self.calls += 1;
                Ok(Vec::new())
            }
            fn set_volume(instance: &mut Self::Instance, value: f32) {
                instance.push(("volume".into(), value));
            }
            fn set_parameter(instance: &mut Self::Instance, name: &str, value: f32) {
                instance.push((name.into(), value));
            }
        }
        let mut audio = Audio::default();
        for volume in [f32::NEG_INFINITY, -0., 0.029999, f32::NAN] {
            assert!(CoasterAudioInstance::new(&mut audio, volume, &[]).is_none());
        }
        assert_eq!(audio.calls, 0);
        let instance =
            CoasterAudioInstance::new(&mut audio, 0.03, &[("speed", 2.), ("wheels", 3.)]).unwrap();
        assert_eq!(audio.calls, 1);
        assert_eq!(
            instance.instance,
            [
                ("volume".into(), 0.03),
                ("speed".into(), 2.),
                ("wheels".into(), 3.)
            ]
        );
    }
}

/// 原 `assemble_coaster_track`（内联于1403bb2b1..bc190）。原图的邻接顺序
/// 决定岔路选择；只延伸首条未访问的无向边，不排序、不回溯，也不改成BFS。
fn assemble_coaster_track(
    initial_nodes: [u32; 2],
    stairs: &country_core::resources::stairs::stairs_state::StairsState,
    requests: &country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests,
) -> Result<::utils::natural_cubic::Spline3D> {
    let points = collect_coaster_track_points(initial_nodes, stairs, requests);
    let curve = ::utils::curve::Curve3::try_new_from_points(points, false)?;
    let curve = curve.try_resample(1.0, true)?.smooth(4)?;
    Ok(
        ::utils::natural_cubic::Spline3D::from_control_points_with_tangents(
            &curve.points,
            None,
            None,
        ),
    )
}

/// 原轨道图到控制点的完整业务段，和下游Curve算法分开形成可核验边界。
fn collect_coaster_track_points(
    initial_nodes: [u32; 2],
    stairs: &country_core::resources::stairs::stairs_state::StairsState,
    requests: &country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests,
) -> Vec<glam::Vec3> {
    use country_core::resources::stairs::stairs_state::{StairGraphEdge, StairSegment};
    use country_core::resources::walls::decorator::DecoratorId;
    let mut nodes = Vec::from(initial_nodes);
    let mut visited =
        std::collections::HashSet::from([StairGraphEdge::new(initial_nodes[0], initial_nodes[1])]);
    loop {
        let last = *nodes.last().unwrap();
        let next = stairs
            .graph()
            .neighbors(last)
            .find(|next| !visited.contains(&StairGraphEdge::new(last, *next)));
        let Some(next) = next else {
            break;
        };
        visited.insert(StairGraphEdge::new(last, next));
        nodes.push(next);
    }
    let mut points: Vec<glam::Vec3> = Vec::new();
    for edge in nodes.windows(2) {
        let [a, b] = [edge[0], edge[1]];
        let segment = StairSegment::new(DecoratorId(a), DecoratorId(b));
        let Some(request) = requests
            .requests
            .iter()
            .find(|request| request.segment == segment)
        else {
            continue;
        };
        let mut edge_points = request
            .interpolated_curve
            .as_ref()
            .map(|curve| curve.points.clone())
            .unwrap_or_default();
        if b == request.segment.nodes().0.0 {
            edge_points.reverse();
        }
        let start = requests
            .alignments
            .edge_weight(DecoratorId(a), DecoratorId(b))
            .unwrap();
        if start.tag == 0 && (start.flag != 0 || start.byte_2 != 0) && !edge_points.is_empty() {
            edge_points.remove(0);
        }
        let end = requests
            .alignments
            .edge_weight(DecoratorId(b), DecoratorId(a))
            .unwrap();
        if end.tag == 0 && (end.flag != 0 || end.byte_2 == 1) {
            edge_points.pop();
        }
        for point in edge_points {
            // 原逐分量 CMPNLE/UCOMISS 分支：差分无序也保留，不能改成有序cmpgt或欧氏距离。
            if points.last().is_none_or(|last| {
                (point - *last)
                    .abs()
                    .to_array()
                    .into_iter()
                    .any(|difference| !(difference <= 0.01))
            }) {
                points.push(point);
            }
        }
    }
    // 原 SIMD MAXPS 与标量 MAXSS 都以 -1.3 为第二操作数：NaN同样选择地板。
    for point in &mut points {
        if !(point.y > -1.3) {
            point.y = -1.3;
        }
    }
    points
}

/// 原 RollercoasterState::new@1403b6bc0..6d46 的初始参数搜索。
/// 固定约每单位5个采样，等距时保留第一个；不能替换成最近线段投影。
fn initial_coaster_parameter(
    spline: &::utils::natural_cubic::Spline3D,
    position: glam::Vec3,
) -> f32 {
    let count = ((spline.max_t * 5.0).ceil() as usize).max(2);
    let mut best_distance = f32::MAX;
    let mut best_t = 0.0;
    for index in 0..count {
        let t = (index as f32 / (count - 1) as f32) * spline.max_t;
        let delta = spline.position_at_t(t) - position;
        let distance = ((delta.y * delta.y + delta.x * delta.x) + delta.z * delta.z).sqrt();
        if distance < best_distance {
            best_distance = distance;
            best_t = t;
        }
    }
    best_t
}

/// 原每车运动状态。J在世界坐标累计，BC/A在look_at局部坐标累计；
/// BC的第三轴没有前两轴的摩擦，不能合并成同一弹簧近似。
#[derive(Clone, Copy, Debug, Default)]
struct CartMotion {
    world_inertia: glam::Vec3,
    lean: glam::Vec3,
    local_inertia: glam::Vec3,
    peak: f32,
}

/// 原ride进度中的标量动力学，不把可变速度简化成匀速弧长步进。
#[derive(Clone, Copy, Debug, Default)]
struct CoasterProgress {
    t: f32,
    speed: f32,
    accumulator: f32,
}

impl CoasterProgress {
    fn step(
        &mut self,
        tangent_y: f32,
        drive: f32,
        brake: f32,
        raw_flag: u8,
        cart_count: usize,
        min_t: f32,
        max_t: f32,
    ) -> bool {
        let dt = 0.001;
        let continuous_lerp = |a: f32, b: f32, rate: f32| (b - a) * (1.0 - (-dt * rate).exp()) + a;
        let raw = self.speed + (tangent_y * -dt) * 10.0;
        self.speed = raw.max(2.0);
        let rate = f32::from(raw_flag) + drive;
        if rate > 0.0 {
            self.speed = continuous_lerp(self.speed, 15.0, rate);
        }
        self.speed *= ((brake + brake + 0.01) * -dt).exp();
        self.speed = self.speed.clamp((1.0 - brake) + (1.0 - brake), 50.0);
        let tail = self.t + (cart_count - 1) as f32 * 2.5;
        let start = max_t - 8.0;
        let u = ((tail - start) / (max_t - start)).clamp(0.0, 1.0);
        let smooth = (3.0 - (u + u)) * (u * u);
        let target = (2.0 - self.speed) * smooth + self.speed;
        self.speed = continuous_lerp(self.speed, target, (self.speed - 2.0).max(1.0) * 0.5);
        self.t = (self.t + self.speed * dt).clamp(min_t, max_t);
        raw < 2.0 && tangent_y > 0.2
    }
}

/// 原glam通用四元数旋转公式，保留非单位q和SSE加法顺序。
/// 不使用要求单位q的v+2*cross简式，也不归一化已求得的姿态。
fn coaster_rotate(q: glam::Quat, v: glam::Vec3) -> glam::Vec3 {
    let diag = q.w * q.w - ((q.y * q.y + q.x * q.x) + q.z * q.z);
    let dot2 = ((q.y * v.y + q.x * v.x) + q.z * v.z) * 2.0;
    let double_w = q.w + q.w;
    glam::Vec3::new(
        (v.x * diag + q.x * dot2) + (q.y * v.z - q.z * v.y) * double_w,
        (v.y * diag + q.y * dot2) + (q.z * v.x - q.x * v.z) * double_w,
        (v.z * diag + q.z * dot2) + (q.x * v.y - q.y * v.x) * double_w,
    )
}

fn coaster_cart_rotations(look: glam::Quat, lean: glam::Vec3) -> (glam::Quat, glam::Quat) {
    let swing = glam::Vec3::new(lean.x + 0.0, lean.y + 2.0, 0.0);
    let direction = coaster_normalize(swing);
    let cosine = (1.0 * direction.y + 0.0 * direction.x) + 0.0 * direction.z;
    let epsilon = f32::from_bits(0x3f7f_fffc);
    let arc = if cosine > epsilon || cosine < -epsilon {
        // 奇点分支继续复用原锁定glam；车摆量[-1,1]使实际方向Y始终为正。
        glam::Quat::from_rotation_arc(glam::Vec3::Y, direction)
    } else {
        let q = [
            1.0 * direction.z - 0.0 * direction.y,
            0.0 * direction.x - 0.0 * direction.z,
            0.0 * direction.y - 1.0 * direction.x,
            1.0 + cosine,
        ];
        // 同锁glam的ARM normalize采用倒数乘，原SSE这里是DIVPS；只在
        // 这个有原指令差分的业务边界保留除法和dot4 lane顺序。
        let length = ((q[3] * q[3] + q[1] * q[1]) + (q[2] * q[2] + q[0] * q[0])).sqrt();
        glam::Quat::from_array(q.map(|value| value / length))
    };
    let rotation = look * arc;
    let visual = rotation
        * glam::Quat::from_xyzw(
            0.0,
            -core::f32::consts::FRAC_1_SQRT_2,
            0.0,
            core::f32::consts::FRAC_1_SQRT_2,
        );
    (rotation, visual)
}

/// 原先算倒数再乘各lane；逐lane除长度会改变f32舍入。
fn coaster_normalize(value: glam::Vec3) -> glam::Vec3 {
    value * (1.0 / ((value.y * value.y + value.x * value.x) + value.z * value.z).sqrt())
}

impl CartMotion {
    /// 原1403b8d68..9260、94b0..95d9的一个1ms子步；返回摆动过零事件。
    /// 切线必须保持Spline原长度，只有look_at输入方向另作归一化。
    fn step(
        &mut self,
        selected: bool,
        sheep_scale: Option<glam::Vec3>,
        old_speed: f32,
        speed: f32,
        old_tangent: glam::Vec3,
        tangent: glam::Vec3,
        look: glam::Quat,
        steer: f32,
    ) -> bool {
        let dt = f32::from_bits(0x3a83_126f);
        let slow_decay = f32::from_bits(0x3f7e_fa61);
        let delta_velocity = old_tangent * old_speed - tangent * speed;
        self.world_inertia += (delta_velocity * 28.0) * dt;
        if selected {
            self.world_inertia += ((coaster_rotate(look, glam::Vec3::X) * steer) * dt) * 6.0;
        }
        self.world_inertia *= slow_decay;
        let was_swinging = self.lean.x.abs() >= 0.00001 && self.peak > 0.025;
        let mass = sheep_scale.map_or(120.0, |scale| {
            ((((scale.x + scale.x) * scale.y) * scale.z) / f32::from_bits(0x3e8c_9ba5)) * 120.0
        });
        let local = coaster_rotate(look.conjugate(), (delta_velocity * mass) * dt);
        let response = |value: f32| (1.0 - value.abs()).max(0.0).min(1.0);
        if selected {
            let signed_zero = f32::from_bits(steer.to_bits() & 0x8000_0000);
            let control = [steer, signed_zero, signed_zero];
            for (value, input) in self.lean.as_mut().iter_mut().zip(control) {
                *value += response(*value) * ((input * dt) * 5.0);
            }
        }
        for (value, acceleration) in self.lean.as_mut().iter_mut().zip(local.to_array()) {
            *value = (*value + response(*value) * acceleration)
                .max(-1.0)
                .min(1.0);
        }
        for value in &mut self.lean.as_mut()[..2] {
            *value -= value.abs().min(0.002).copysign(*value);
        }
        let swing = self.lean.x.abs();
        // 原maxss相等时保留旧peak（包括-0），其后cmpunord修正旧NaN。
        self.peak = if swing > self.peak || self.peak.is_nan() {
            swing
        } else {
            self.peak
        } * slow_decay;
        let crossed = was_swinging && self.lean.x.abs() < 0.00001;
        let local = coaster_rotate(look.conjugate(), (delta_velocity * 32.0) * dt);
        self.local_inertia = (self.local_inertia + local.map(|value| value.max(-1.0).min(1.0)))
            * f32::from_bits(0x3f5a_2618);
        crossed
    }
}

/// 原create_cart_chain中查找前后车的两条射线。保留未乘对象scale的mesh范围，
/// 与原四元数乘法/旋转的通用公式；第0条和第1条分别决定push_back/push_front。
fn coaster_neighbor_rays(
    transform: country_core::utils::components::transform::Transform,
    extent: glam::Vec3,
) -> [(glam::Vec3, glam::Vec3); 2] {
    let q = transform.rotation;
    let s = core::f32::consts::FRAC_1_SQRT_2;
    let rotation = glam::Quat::from_xyzw(
        (q.y * 0.0 * 1.0 + q.z * s * -1.0) + (q.w * 0.0 + q.x * s * 1.0),
        (q.y * s * 1.0 + q.z * 0.0 * 1.0) + (q.w * s + q.x * 0.0 * -1.0),
        (q.y * 0.0 * -1.0 + q.z * s * 1.0) + (q.w * 0.0 + q.x * s * 1.0),
        (q.y * s * -1.0 + q.z * 0.0 * -1.0) + (q.w * s + q.x * 0.0 * -1.0),
    );
    [false, true].map(|front| {
        let sign = if front { -1.0 } else { 1.0 };
        let offset = sign * ((extent.x * 0.5) * 1.1);
        let zero = offset * -0.0;
        let position = glam::Vec3::new(
            extent.y * 0.0 * 0.5 + zero,
            extent.y * 0.5 + zero,
            extent.y * 0.0 * 0.5 - offset,
        );
        let direction = -sign;
        let signed_zero = f32::from_bits(direction.to_bits() & 0x8000_0000);
        (
            coaster_rotate(rotation, position) + transform.translation,
            coaster_rotate(
                rotation,
                glam::Vec3::new(signed_zero, signed_zero, direction),
            ),
        )
    })
}

/// 原CoasterCart的权威所有权，保留装饰物原Transform和真实速度实例实体。
/// 不另造可见车厢；骑乘时更新同一DecoratorVisualState内的持久实体。
struct CoasterCart<E: country_core::resources::audio_resources::EventInstance> {
    original_transform: country_core::utils::components::transform::Transform,
    audio: CoasterAudioState<E>,
    wheel_index: usize,
    wheel_timer: f32,
    velocity_entity: bevy_ecs::entity::Entity,
    sheep: Option<bevy_ecs::entity::Entity>,
    decorator_id: country_core::resources::walls::decorator::DecoratorId,
    motion: CartMotion,
}

impl<E: country_core::resources::audio_resources::EventInstance> CoasterCart<E> {
    /// 原1403b9f20..ba53f的逐车帧尾音频。底部撞击与摆动过零是整个子步
    /// 批次的OR，不可只消费最后子步；池按原顺序选择首个未播放实例。
    fn update_audio(
        &mut self,
        position: glam::Vec3,
        speed: f32,
        dt: f32,
        crossed: bool,
        touched_bottom: bool,
    ) {
        let pos = position.to_array();
        for sound in [
            &mut self.audio.height,
            &mut self.audio.overlap,
            &mut self.audio.timberframe,
            &mut self.audio.water,
        ]
        .into_iter()
        .flatten()
        {
            let _ = sound.instance.set_position(pos);
        }
        if let Some(pool) = &mut self.audio.lantern {
            for sound in pool {
                let _ = sound.instance.set_position(pos);
            }
        }
        if let Some(sound) = &mut self.audio.height {
            let _ = sound
                .instance
                .set_volume((speed * 0.02).max(0.0).min(1.0) * sound.volume);
            let _ = sound
                .instance
                .set_parameter("editing_speed", (speed * 0.05).max(0.0).min(1.0));
        }
        if crossed {
            if let Some(pool) = &mut self.audio.lantern {
                if let Some(sound) = pool
                    .iter_mut()
                    .find(|sound| !sound.instance.is_playing().unwrap_or(false))
                {
                    let _ = sound
                        .instance
                        .set_volume((self.motion.peak * 12.0).max(0.5).min(4.0) * sound.volume);
                    self.motion.peak = 0.0;
                    let _ = sound.instance.start_if_not_playing();
                }
            }
        }
        if let Some(sound) = &mut self.audio.water {
            if position.y < -0.3 {
                let _ = sound.instance.start_if_not_playing();
            } else {
                let _ = sound.instance.stop();
            }
        }
        if touched_bottom && self.wheel_timer <= 0.0 {
            if let Some(pool) = &mut self.audio.chimney {
                let sound = &mut pool[self.wheel_index];
                let _ = sound.instance.set_volume(sound.volume * 0.25);
                let _ = sound.instance.start_if_not_playing();
                let _ = sound.instance.set_position(pos);
            }
            if let Some(pool) = &mut self.audio.fence {
                let sound = &mut pool[self.wheel_index];
                let _ = sound.instance.set_volume(sound.volume * 0.25);
                let _ = sound.instance.start_if_not_playing();
                let _ = sound.instance.set_position(pos);
            }
            self.wheel_index += 1;
            self.wheel_timer = 0.25;
            if let Some(pool) = &self.audio.chimney {
                self.wheel_index %= pool.len();
            }
        } else {
            self.wheel_timer -= dt;
        }
        if let Some(sound) = &mut self.audio.timberframe {
            let _ = sound.instance.set_parameter("editing_speed", 1.0);
            let _ = sound.instance.set_volume(
                (self.motion.local_inertia.x.abs().min(2.0) * 0.4) * 0.5 * sound.volume,
            );
        }
    }

    /// 原退出显式停止四个持续实例；实例池沿已有AudioInstance Drop释放。
    fn stop_audio(&mut self) {
        for sound in [
            &mut self.audio.timberframe,
            &mut self.audio.height,
            &mut self.audio.overlap,
            &mut self.audio.water,
        ]
        .into_iter()
        .flatten()
        {
            let _ = sound.instance.stop();
        }
    }
}

impl<E: country_core::resources::audio_resources::EventInstance> CoasterAudioState<E> {
    /// 原create_cart_chain先创建无音频Cart，State::new再按选中索引初始化。
    fn empty() -> Self {
        Self {
            timberframe: None,
            height: None,
            overlap: None,
            water: None,
            chimney: None,
            fence: None,
            lantern: None,
        }
    }
}

/// 原1403be1a0完整邻车遍历与VelocityOnly实体生产。transform_of必须来自
/// 原同一已激活Nani实体查询（含Without<Sheep>），查询失败由原unwrap路径报错。
/// 栈采用LIFO，去重只检查已经处理的双端队列，不能提前去重pending改变原顺序。
fn create_cart_chain<E: country_core::resources::audio_resources::EventInstance>(
    initial: CoasterTarget,
    commands: &mut bevy_ecs::system::Commands,
    mut transform_of: impl FnMut(
        bevy_ecs::entity::Entity,
    ) -> country_core::utils::components::transform::Transform,
    meshes: &country_core::resources::clutter_meshes::ClutterMeshes,
    collisions: &country_core::systems::collision::world::RaycastWorld,
    heights: &country_core::resources::terrain_height::TerrainHeightsData,
    storage: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    visuals: &system_decorator::decorator_visual_state::DecoratorVisualState,
) -> Vec<CoasterCart<E>> {
    use country_core::{
        components::drawable::{NaniBundle, NaniInstanceDesc, NaniInstanceMeshDesc, PrevTransform},
        render::nani::{
            gpu_world::{GpuWorldSubsetKind, InstanceDataMarker},
            instance_data::VelocityOnlyInstanceData,
        },
        resources::clutter_meshes::{ClutterMeshTy, clutter_mesh_ty_data},
    };
    let mut carts = std::collections::VecDeque::new();
    let mut pending = vec![(initial, true)];
    while let Some((target, front)) = pending.pop() {
        let original_transform = transform_of(target.entity);
        let ty = ClutterMeshTy::from_native_tag(target.decorator_type).unwrap();
        let mesh = clutter_mesh_ty_data(ty, meshes).mesh;
        let velocity_entity = commands
            .spawn((
                NaniBundle {
                    desc: NaniInstanceDesc {
                        mesh: NaniInstanceMeshDesc::Named(mesh.name_hash),
                        subset_kind: GpuWorldSubsetKind::VelocityOnly,
                    },
                    transform: original_transform,
                    marker: InstanceDataMarker::<VelocityOnlyInstanceData>::default(),
                },
                PrevTransform::from(original_transform),
            ))
            .id();
        let cart = CoasterCart {
            original_transform,
            audio: CoasterAudioState::empty(),
            wheel_index: 0,
            wheel_timer: 0.0,
            velocity_entity,
            sheep: None,
            decorator_id: target.decorator_id,
            motion: CartMotion::default(),
        };
        if front {
            carts.push_front(cart);
        } else {
            carts.push_back(cart);
        }
        for (side, (origin, direction)) in
            coaster_neighbor_rays(original_transform, mesh.bounds.extent())
                .into_iter()
                .enumerate()
        {
            let hit =
                collisions.raycast_w_terrain_with_max_toi(heights, origin, direction, 2.0, 0x101);
            if hit.toi > 2.0 {
                continue;
            }
            if let Some(next) = coaster_target_from_hit(&hit, storage, visuals) {
                if !carts
                    .iter()
                    .any(|cart| cart.decorator_id == next.decorator_id)
                {
                    pending.push((next, side != 0));
                }
            }
        }
    }
    carts.into_iter().collect()
}

/// 原Local<Option<RollercoasterState>>载荷。没有成功的空/default骑乘状态。
pub struct RollercoasterState<E: country_core::resources::audio_resources::EventInstance> {
    carts: Vec<CoasterCart<E>>,
    spline: ::utils::natural_cubic::Spline3D,
    selected: usize,
    progress: CoasterProgress,
}
impl<E: country_core::resources::audio_resources::EventInstance> RollercoasterState<E> {
    fn new<S: country_core::resources::audio_resources::Studio<Event = E>>(
        mut carts: Vec<CoasterCart<E>>,
        spline: ::utils::natural_cubic::Spline3D,
        selected: usize,
        audio: &country_core::resources::audio_resources::AudioCore<S>,
    ) -> Self {
        // 原入口以首车而非选中车搜索初始t，空链是断言失败。
        let first = carts
            .first()
            .expect("rollercoaster cart chain is empty")
            .original_transform;
        assert!(selected < carts.len());
        let t = initial_coaster_parameter(&spline, first.translation);
        for (index, cart) in carts.iter_mut().enumerate() {
            cart.audio = CoasterAudioState::new(audio, index, selected);
            cart.wheel_index = 0;
            cart.wheel_timer = 0.0;
        }
        Self {
            carts,
            spline,
            selected,
            progress: CoasterProgress {
                t,
                speed: 0.0,
                accumulator: 0.0,
            },
        }
    }
}

fn coaster_look_at(direction: glam::Vec3) -> glam::Quat {
    glam::Quat::from_array(dolly::util::look_at::<
        dolly::handedness::RightHanded,
        _,
        [f32; 4],
    >(direction.to_array()))
}

use bevy_ecs::prelude::{Commands, Entity, EventWriter, Local, Query, Res, ResMut, With, Without};
use country_core::{
    render::{extract::ComponentWrite, nani::gpu_world::InstanceKey},
    resources::{
        audio_resources::{AudioCore, EventInstance, Studio},
        clutter_meshes::ClutterMeshes,
        critters::sheep::Sheep,
        stairs::{
            stairs_assembly_params::PlayermadeStairsAssemblyRequests, stairs_state::StairsState,
        },
        terrain_height::TerrainHeightsData,
        time::GameTime,
        walls::decorator_storage::DecoratorStorage,
    },
    systems::{
        audio::{PlaySheepReactSound, trigger_audio::TriggerAudioState},
        backend::input::mod_key::Modifiers,
        camera_rig::{MainCamera, walk::teleport_rig},
        collision::world::RaycastWorld,
        mode_manager::AppMode,
        render_loop::vermeer::VermeerState,
    },
    utils::components::transform::Transform,
};
use system_critters::sheep::{
    sheep_behavior::SheepAudio,
    sheep_floatie::{FloatieHolder, SheepAccessory},
    sheep_pet_animation::PlaySheepPet,
    sheep_umbrella::UmbrellaHolder,
};

type CoasterContext<'w> = (
    Res<'w, RaycastWorld>,
    Res<'w, TerrainHeightsData>,
    Res<'w, GameTime>,
    Res<'w, StairsState>,
    Res<'w, PlayermadeStairsAssemblyRequests>,
    Res<'w, DecoratorStorage>,
    Res<'w, system_decorator::decorator_visual_state::DecoratorVisualState>,
    Res<'w, ClutterMeshes>,
    Res<'w, AppMode>,
    Res<'w, Modifiers>,
);

/// 原rollercoaster_update@1403b7f20的同一生产链。原两个互斥Transform查询
/// 保留在同一系统内，写入经当前ComponentWrite记录给渲染抽取，不另建影子场景。
pub fn rollercoaster_update<S>(
    mut ride: Local<Option<RollercoasterState<S::Event>>>,
    mut camera: ResMut<MainCamera>,
    mut trigger_audio: ResMut<TriggerAudioState<S::Event>>,
    audio: Res<AudioCore<S>>,
    actions: Res<country_core::input::action_set::ActionSetStates>,
    vermeer: Res<VermeerState>,
    context: CoasterContext,
    mut transforms: Query<ComponentWrite<Transform>, (With<InstanceKey>, Without<Sheep>)>,
    mut sheep: Query<(Entity, ComponentWrite<Transform>, &Sheep)>,
    sheep_audio: Query<&SheepAudio>,
    holders: Query<(&UmbrellaHolder, &FloatieHolder)>,
    disabled_accessories: Query<
        Entity,
        (
            With<SheepAccessory>,
            With<bevy_ecs::entity_disabling::Disabled>,
        ),
    >,
    mut commands: Commands,
    mut pet: EventWriter<PlaySheepPet>,
    mut react: EventWriter<PlaySheepReactSound>,
) where
    S: Studio + Send + Sync + 'static,
    S::Event: EventInstance + Send + Sync + 'static,
{
    use country_core::systems::main_camera::{
        PROMPT_ROLLERCOASTER, set_rollercoaster_walk_override, write_rollercoaster_camera_pose,
    };
    use std::sync::atomic::Ordering;
    let (collisions, heights, time, stairs, requests, storage, visuals, meshes, mode, modifiers) =
        context;
    let dt = if vermeer.should_freeze_camera() {
        0.0
    } else {
        time.delta
    };
    if let Some(state) = ride.as_mut() {
        set_rollercoaster_walk_override(&mut camera);
        let axis = actions.analog[2] / (dt * 30.0);
        let bounded = |v: f32| {
            if v.is_finite() {
                v.max(-1.0).min(1.0)
            } else {
                0.0
            }
        };
        let steer = bounded(axis.x);
        let vertical = bounded(-axis.y);
        let drive = vertical.max(0.0);
        let brake = (-vertical).max(0.0);
        let mut crossed = vec![false; state.carts.len()];
        let mut touched_bottom = false;
        state.progress.accumulator += dt;
        for _ in 0..1000 {
            // 原NaN不会提前退出，1000上限必须保留。
            if state.progress.accumulator < 0.001 {
                break;
            }
            state.progress.accumulator += -0.001;
            let old_t = state.progress.t;
            let old_speed = state.progress.speed;
            let tangent = state
                .spline
                .tangent_at_t(old_t + state.selected as f32 * 2.5);
            touched_bottom |= state.progress.step(
                tangent.y,
                drive,
                brake,
                modifiers.flags as u8,
                state.carts.len(),
                state.spline.min_t,
                state.spline.max_t,
            );
            for (index, cart) in state.carts.iter_mut().enumerate() {
                let t = state.progress.t + index as f32 * 2.5;
                let position = state.spline.position_at_t(t);
                let old_tangent = state.spline.tangent_at_t(old_t + index as f32 * 2.5);
                let tangent = state.spline.tangent_at_t(t);
                let direction = coaster_normalize(tangent);
                let look = coaster_look_at(direction);
                let mut passenger = cart.sheep.and_then(|entity| sheep.get_mut(entity).ok());
                crossed[index] |= cart.motion.step(
                    index == state.selected,
                    passenger.as_ref().map(|(_, transform, _)| transform.scale),
                    old_speed,
                    state.progress.speed,
                    old_tangent,
                    tangent,
                    look,
                    steer,
                );
                if let Some((entity, _, component)) = passenger.as_ref() {
                    let j = cart.motion.world_inertia;
                    if ((j.y * j.y + j.x * j.x) + j.z * j.z).sqrt() > 0.16
                        && fastrand::f32() > 0.99930024
                    {
                        pet.write(PlaySheepPet {
                            target: component.pet_target,
                        });
                        let pitch = sheep_audio
                            .get(*entity)
                            .map(|sound| {
                                (fastrand::f32() * 0.4 + sound.startup_pitch + 0.3).min(1.2)
                            })
                            .unwrap_or(1.0);
                        react.write(PlaySheepReactSound {
                            position,
                            pitch,
                            kind: 0,
                        });
                    }
                }
                let (rotation, visual_rotation) = coaster_cart_rotations(look, cart.motion.lean);
                for entity in visuals
                    .persistent_entities_of(cart.decorator_id.0)
                    .into_iter()
                    .flatten()
                    .copied()
                    .chain(core::iter::once(cart.velocity_entity))
                {
                    if let Ok(mut transform) = transforms.get_mut(entity) {
                        transform.translation = position;
                        transform.rotation = visual_rotation;
                    }
                }
                if let Some((_, ref mut transform, _)) = passenger {
                    transform.translation =
                        position + coaster_rotate(rotation, glam::Vec3::new(0.128, 0.697, 0.068));
                    transform.rotation = rotation
                        * glam::Quat::from_xyzw(0.0, 0.26673144, 0.0, 0.96377087)
                        * glam::Quat::from_xyzw(0.25708055, 0.0, 0.0, 0.96638995);
                }
            }
        }
        let selected = &state.carts[state.selected];
        let t = state.progress.t + state.selected as f32 * 2.5;
        let position = state.spline.position_at_t(t);
        let tangent = state.spline.tangent_at_t(t);
        let look = coaster_look_at(coaster_normalize(tangent));
        let walk_rotation = {
            let rigs = camera
                .startup_rigs
                .as_ref()
                .expect("MainCamera Dolly rigs absent")
                .lock()
                .unwrap();
            let q = rigs.walk.final_transform.rotation;
            glam::Quat::from_xyzw(q.v.x, q.v.y, q.v.z, q.s)
        };
        let position = position
            + coaster_rotate(look, glam::Vec3::new(0.0, 1.875, 0.5))
            + selected.motion.world_inertia.map(|v| v.max(-1.0).min(1.0)) * 0.4;
        write_rollercoaster_camera_pose(&mut camera, position, look * walk_rotation);
        for (index, cart) in state.carts.iter_mut().enumerate() {
            cart.update_audio(
                state
                    .spline
                    .position_at_t(state.progress.t + index as f32 * 2.5),
                state.progress.speed,
                dt,
                crossed[index],
                touched_bottom,
            );
        }
        let finished = actions.digital[18].just_pressed()
            || actions.digital[19].just_pressed()
            || *mode != AppMode::Photomode
            || camera.mode_byte() != 3
            || !(state.progress.t + (state.carts.len() - 1) as f32 * 2.5 < state.spline.max_t);
        if finished {
            let mut state = ride.take().unwrap();
            for cart in &mut state.carts {
                for entity in visuals
                    .persistent_entities_of(cart.decorator_id.0)
                    .into_iter()
                    .flatten()
                {
                    if let Ok(mut transform) = transforms.get_mut(*entity) {
                        *transform = cart.original_transform;
                    }
                }
                commands.entity(cart.velocity_entity).despawn();
                cart.stop_audio();
            }
            let position = camera.eye_position();
            let direction = camera.eye_forward() * glam::Vec3::new(1.0, 0.0, 1.0);
            let direction = coaster_normalize(direction);
            teleport_rig(
                &mut camera.startup_rigs.as_ref().unwrap().lock().unwrap().walk,
                position,
                direction,
            );
            trigger_audio.in_forest_state = false;
            trigger_audio.wind_volume = 0.0;
            for entity in &disabled_accessories {
                commands
                    .entity(entity)
                    .remove::<bevy_ecs::entity_disabling::Disabled>();
            }
        }
    }
    // 原清理之后无条件清提示，再按同帧空闲状态检测；不要在摄影消费者清理。
    PROMPT_ROLLERCOASTER.store(false, Ordering::Relaxed);
    let target = find_rollercoaster_prompt_target(
        *mode,
        &camera,
        ride.is_some(),
        &collisions,
        &heights,
        &storage,
        &visuals,
    );
    if let Some(target) = target {
        PROMPT_ROLLERCOASTER.store(true, Ordering::Relaxed);
        if actions.digital[19].just_pressed() {
            let transform = *transforms
                .get_mut(target.entity)
                .expect("rollercoaster target Transform absent");
            use country_core::resources::walls::decorator::DecoratorId;
            let [a, b] = target.stair_nodes.map(DecoratorId);
            let edge = requests
                .adjusted_edges
                .edge_weight(a, b)
                .expect("rollercoaster adjusted stair edge absent");
            let forward = coaster_rotate(transform.rotation, glam::Vec3::NEG_X);
            let nodes =
                if !(forward.x * edge.outgoing_dir.x + forward.z * edge.outgoing_dir.y > 0.0) {
                    [target.stair_nodes[1], target.stair_nodes[0]]
                } else {
                    target.stair_nodes
                };
            match assemble_coaster_track(nodes, &stairs, &requests) {
                Ok(spline) => {
                    let mut carts = create_cart_chain::<S::Event>(
                        target,
                        &mut commands,
                        |entity| {
                            *transforms
                                .get_mut(entity)
                                .expect("rollercoaster cart Transform absent")
                        },
                        &meshes,
                        &collisions,
                        &heights,
                        &storage,
                        &visuals,
                    );
                    let selected = carts
                        .iter()
                        .position(|cart| cart.decorator_id == target.decorator_id)
                        .expect("selected rollercoaster cart missing from chain");
                    let mut passengers: Vec<Entity> =
                        sheep.iter_mut().map(|(entity, _, _)| entity).collect();
                    fastrand::shuffle(&mut passengers);
                    for (index, cart) in carts.iter_mut().enumerate() {
                        if index != selected {
                            if let Some(entity) = passengers.pop() {
                                if let Ok((umbrella, floatie)) = holders.get(entity) {
                                    for accessory in
                                        [umbrella.spawned, floatie.spawned].into_iter().flatten()
                                    {
                                        commands
                                            .entity(accessory)
                                            .insert(bevy_ecs::entity_disabling::Disabled);
                                    }
                                }
                                cart.sheep = Some(entity);
                            }
                        }
                    }
                    trigger_audio.in_forest_state = true;
                    teleport_rig(
                        &mut camera.startup_rigs.as_ref().unwrap().lock().unwrap().walk,
                        glam::Vec3::ZERO,
                        glam::Vec3::NEG_Z,
                    );
                    *ride = Some(RollercoasterState::new(carts, spline, selected, &audio));
                }
                Err(error) => log::error!("Rollercoaster error: {error:#}"),
            }
        }
    }
}

/// 原build@1403b67a0：Update注册到GameSystemSet(9)，Local的None由Bevy初始化。
/// 具体宿主必须在真实相机之后、摄影提示消费之前调用此注册入口。
pub fn build(app: &mut bevy_app::App) {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    #[cfg(not(test))]
    type S = crate::gameplay_startup::GameplayAudioStudio;
    #[cfg(test)]
    type S = country_core::resources::audio_resources::DisabledStudio;
    app.add_systems(
        bevy_app::Update,
        rollercoaster_update::<S>
            .in_set(country_core::systems::GameSystemSet(9))
            .after(country_core::systems::main_camera::main_camera_update),
    );
}
