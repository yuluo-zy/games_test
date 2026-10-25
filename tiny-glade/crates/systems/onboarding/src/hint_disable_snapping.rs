//! B `hint_disable_snapping`（0x14227e140）：真实形状吸附通知到提示／教程完成。
//! 原producer在有墙／屋顶候选时写false（正常吸附）或true（按修饰键关闭）；
//! consumer只取整批最后事件，键帽取权威DisableSnapping键，Steam查询为0x0f01。

use bevy_ecs::prelude::*;

/// 原1字节状态，启动由GameProfile.disable_snapping_completed初始化。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintDisableSnappingConditions {
    pub hint_done: bool,
}

pub fn hint_disable_snapping(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<HintDisableSnappingConditions>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemSnapping>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
) {
    // 原EventIterator::last同时消耗双缓冲全部通知；早期状态不盖过最新吸附结果。
    let Some(event) = events.read().last() else {
        return;
    };
    if event.0 {
        if !state.hint_done {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 3 });
        }
        state.hint_done = true;
    } else if !profile.disable_snapping_completed && !state.hint_done {
        if let country_core::systems::mode_manager::AppMode::Wall(shape) = *mode {
            let key = country_core::game_key_bindings::KeyBinding::DisableSnapping
                .get_keycode(&profile.key_bindings);
            let display = crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
                country_core::resources::steam::packed_set_plus_one(15),
            ))
            .alternate(key);
            request.write(
                crate::DisplayHintRequest::new(display, crate::HintType::DisableSnapping, 0)
                    .with_placement(crate::hint_orchestrator::wall_shape_placement(shape as u8)),
            );
        }
    }
}

#[cfg(test)]
mod native_regressions {
    use super::*;
    use crate::{
        BindingHintDisplay, DisplayHintRequest,
        tutorial_completion::{TutorialHintComplete, tutorial_completion},
    };
    use bevy_input::keyboard::{KeyCode, NativeKeyCode};
    use country_core::{
        game_profile::{GameProfile, SaveGameProfileCmd},
        resources::onboarding::NotifyHintSystemSnapping,
        systems::mode_manager::{AppMode, WallShape},
    };

    fn world() -> World {
        let mut world = World::new();
        world.init_resource::<GameProfile>();
        world.insert_resource(AppMode::Wall(WallShape::Freeform));
        world.init_resource::<HintDisableSnappingConditions>();
        world.init_resource::<Events<NotifyHintSystemSnapping>>();
        world.init_resource::<Events<DisplayHintRequest>>();
        world.init_resource::<Events<TutorialHintComplete>>();
        world.init_resource::<Events<SaveGameProfileCmd>>();
        world
    }
    fn key(words: &[u32]) -> Option<KeyCode> {
        match words[0] {
            195 => None,
            1 => Some(KeyCode::Backquote),
            20 => Some(KeyCode::KeyA),
            0 => Some(KeyCode::Unidentified(match words[1] & 0xffff {
                0 => NativeKeyCode::Unidentified,
                1 => NativeKeyCode::Android(words[2]),
                2 => NativeKeyCode::MacOS((words[1] >> 16) as u16),
                3 => NativeKeyCode::Windows((words[1] >> 16) as u16),
                4 => NativeKeyCode::Xkb(words[2]),
                _ => unreachable!("原夹具平台键类型"),
            })),
            _ => unreachable!("原夹具命名键"),
        }
    }
    #[test]
    fn full_original_consumer_matches_all_modes_last_events_buffers_and_typed_keys() {
        type Row = (
            u8,
            u8,
            bool,
            bool,
            Vec<bool>,
            usize,
            Vec<u32>,
            (bool, usize, usize, u8, u16, Vec<u32>),
        );
        let rows: Vec<Row> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/hint-disable-snapping.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 4896);
        let mut world = world();
        let mut schedule = Schedule::default();
        schedule.add_systems(hint_disable_snapping);
        for (
            mode,
            shape,
            profile_done,
            local_done,
            events,
            split,
            input_key,
            (done, completion_count, request_count, placement, query, output_key),
        ) in rows
        {
            world.insert_resource(match mode {
                0 => AppMode::default(),
                6 => AppMode::Wall(match shape {
                    0 => WallShape::Freeform,
                    1 => WallShape::Circle,
                    2 => WallShape::Rectangle,
                    _ => unreachable!(),
                }),
                7 => AppMode::Path,
                10 => AppMode::Terrain,
                _ => unreachable!(),
            });
            world
                .resource_mut::<GameProfile>()
                .disable_snapping_completed = profile_done;
            world.resource_mut::<GameProfile>().key_bindings.bindings[24] = key(&input_key);
            world
                .resource_mut::<HintDisableSnappingConditions>()
                .hint_done = local_done;
            for event in &events[..split] {
                world.send_event(NotifyHintSystemSnapping(*event));
            }
            world
                .resource_mut::<Events<NotifyHintSystemSnapping>>()
                .update();
            for event in &events[split..] {
                world.send_event(NotifyHintSystemSnapping(*event));
            }
            schedule.run(&mut world);
            assert_eq!(
                world.resource::<HintDisableSnappingConditions>().hint_done,
                done
            );
            assert_eq!(
                world.resource::<Events<TutorialHintComplete>>().len(),
                completion_count
            );
            let requests = world.resource::<Events<DisplayHintRequest>>();
            assert_eq!(requests.len(), request_count);
            if let Some(request) = requests.get_cursor().read(requests).next() {
                assert_eq!(request.id, 3);
                assert_eq!(request.flags, 0);
                assert_eq!(request.placement.native_tag(), placement as u32);
                assert_eq!(
                    request.display,
                    BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(query))
                        .alternate(key(&output_key))
                );
            }
            world.resource_mut::<Events<DisplayHintRequest>>().clear();
            world.resource_mut::<Events<TutorialHintComplete>>().clear();
            schedule.run(&mut world);
            assert!(
                world.resource::<Events<DisplayHintRequest>>().is_empty(),
                "原last已消费整批"
            );
            assert!(world.resource::<Events<TutorialHintComplete>>().is_empty());
        }
    }
    #[test]
    fn actual_shape_authority_producer_drives_hint_then_completion_and_profile_save() {
        use country_core::{
            resources::{
                cursor_terrain_raycast::CursorTerrainRaycast,
                mouse_terrain_raycast::CameraCursorRay,
                roofs::geometry::surface::Curve2,
                walls::{
                    InnerWalls, WallId, WallState, public_walls::PublicWalls,
                    publication_source::TerrainHeightGrid512,
                },
            },
            systems::{
                audio::PlayWallUiSound,
                backend::input::mod_key::Modifiers,
                ui::signifiers::signifier_create::UiSignifierStream,
                wall::{
                    shapes::shape_placement::{
                        DontRecalculateShapePlacement, ShapePlacement, SnapMode,
                        shape_placement_params,
                    },
                    wall_state::wall_spatial_hash::{WallSpatialHash, WallSpatialSegment},
                },
            },
        };
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(
            WallId(42),
            WallState::new(
                Curve2::try_new(vec![[0., 0.], [8., 0.]], false).unwrap(),
                3.,
                0,
                0,
                false,
            ),
        );
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.,
            maximum_y: 0.,
            sampling_fix: false,
        };
        let mut public = PublicWalls::default();
        public.consume(&mut inner, &terrain);
        let mut spatial = WallSpatialHash::default();
        spatial.add_segment(WallSpatialSegment {
            endpoints: [[0., 0.], [8., 0.]],
            source_u: [0., 1.],
            relative_heights: [0.; 2],
            terrain_heights: [0.; 2],
            wall: WallId(42),
        });
        let mut world = world();
        world.insert_resource(AppMode::Wall(WallShape::Circle));
        world.insert_resource(public);
        world.insert_resource(spatial);
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: glam::Vec3::new(2., 0., 1.),
            ..Default::default()
        });
        world.init_resource::<ShapePlacement>();
        world.init_resource::<SnapMode>();
        world.init_resource::<Modifiers>();
        world.init_resource::<CameraCursorRay>();
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<Events<PlayWallUiSound>>();
        world.init_resource::<Events<DontRecalculateShapePlacement>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                shape_placement_params,
                hint_disable_snapping,
                tutorial_completion,
            )
                .chain(),
        );
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<ShapePlacement>().wall_id(),
            Some(WallId(42))
        );
        assert_eq!(
            world.resource::<Events<DisplayHintRequest>>().len(),
            1,
            "真实墙候选产生正常吸附提示"
        );
        assert!(world.resource::<Events<SaveGameProfileCmd>>().is_empty());
        world.resource_mut::<Modifiers>().flags = 1;
        schedule.run(&mut world);
        assert!(
            world.resource::<GameProfile>().disable_snapping_completed,
            "原producer true不是手工注入通知"
        );
        assert_eq!(world.resource::<Events<SaveGameProfileCmd>>().len(), 1);
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<SaveGameProfileCmd>>().len(), 1);
        world
            .resource_mut::<WallSpatialHash>()
            .remove_wall_id(WallId(42));
        let count = world.resource::<Events<NotifyHintSystemSnapping>>().len();
        world.resource_mut::<Modifiers>().flags = 0;
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<Events<NotifyHintSystemSnapping>>().len(),
            count,
            "无候选原路径不发吸附通知"
        );
        let saved = ron::to_string(world.resource::<GameProfile>()).unwrap();
        assert!(
            ron::from_str::<GameProfile>(&saved)
                .unwrap()
                .disable_snapping_completed
        );
    }

    #[test]
    fn true_completion_saves_once_and_startup_uses_reloaded_authority() {
        use bevy_ecs::system::RunSystemOnce;
        let mut world = world();
        let mut schedule = Schedule::default();
        schedule.add_systems((hint_disable_snapping, tutorial_completion).chain());
        world.send_event(NotifyHintSystemSnapping(false));
        schedule.run(&mut world);
        assert!(!world.resource::<GameProfile>().disable_snapping_completed);
        assert!(world.resource::<Events<SaveGameProfileCmd>>().is_empty());
        world.send_event(NotifyHintSystemSnapping(true));
        schedule.run(&mut world);
        assert!(world.resource::<GameProfile>().disable_snapping_completed);
        assert_eq!(world.resource::<Events<SaveGameProfileCmd>>().len(), 1);
        world.send_event(NotifyHintSystemSnapping(true));
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<Events<SaveGameProfileCmd>>().len(),
            1,
            "重复完成不重复保存"
        );
        let serialized = ron::to_string(world.resource::<GameProfile>()).unwrap();
        world.insert_resource(ron::from_str::<GameProfile>(&serialized).unwrap());
        world
            .run_system_once(crate::hints_startup::hints_startup)
            .unwrap();
        assert!(world.resource::<HintDisableSnappingConditions>().hint_done);
        world.resource_mut::<Events<DisplayHintRequest>>().clear();
        world.send_event(NotifyHintSystemSnapping(false));
        schedule.run(&mut world);
        assert!(world.resource::<Events<DisplayHintRequest>>().is_empty());
    }
}
