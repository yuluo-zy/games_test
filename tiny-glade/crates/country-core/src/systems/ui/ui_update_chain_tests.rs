// Candidate integration test: actual registry + actual camera/mouse picker +
// real digital resource -> ui_update -> canonical history start/finish. No
// replace_interaction or seeded clicked/used bit exists anywhere in this test.
#[cfg(test)]
mod real_input_chain {
    use super::super::ui_update;
    use super::*;
    use crate::{
        input::action_set::{ActionSetStates, DigitalActionPhase, DigitalActionState},
        resources::{
            cursor_terrain_raycast::CursorTerrainRaycast,
            events::EscapeMenuJustOpened,
            events::ShutdownGameCmd,
            history::{History, MaybePendingSnapshotId},
            mouse_capture::OsCursorVisibilityCmd,
            mouse_terrain_raycast::CursorPositionSS,
            read_mouse_depth_value::MouseDepthValue,
            window::{WindowFocusEvent, WindowSize},
        },
        systems::{
            brush_preview::OverwriteBrushPreview,
            camera_rig::MainCamera,
            history::recording::{
                history_finish_recording_system, history_start_recording, register_recording_events,
            },
            ui::{
                hover_intersection::hover_intersection,
                resources::{
                    ui_state::{UiState, WidgetId},
                    widget_params::{PlayUiSound, WidgetGeometry, WidgetParams},
                },
            },
        },
    };
    use bevy_ecs::prelude::*;
    use glam::{Mat4, Vec2};
    use std::sync::Arc;
    fn register_target(world: &World) {
        let mut p = WidgetParams::default()
            .geometry(WidgetGeometry::ScreenAabb {
                min: Vec2::splat(40.0),
                max: Vec2::splat(60.0),
                distance: 0.5,
            })
            .occlude_by_world(false);
        for slot in 0..4 {
            p.sounds.slots[slot][..4].copy_from_slice(&(100 + slot as u32).to_le_bytes());
            p.sounds.slots[slot][4..8].copy_from_slice(&(10 + slot as u32).to_le_bytes());
        }
        world.resource::<UiState>().add_with_id(WidgetId(91), p);
    }
    fn setup() -> (World, Schedule) {
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let mut world = World::new();
        world.insert_resource(UiState::at_startup());
        let mut camera = MainCamera::new(1.0);
        for offset in [0x40, 0x80, 0xc0] {
            camera.write_matrix(offset, &Mat4::IDENTITY);
        }
        world.insert_resource(camera);
        world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(CursorPositionSS(Vec2::splat(50.0)));
        world.insert_resource(MouseDepthValue::default());
        world.insert_resource(WindowSize::new(100, 100));
        world.insert_resource(ActionSetStates::default());
        world.init_resource::<Events<WindowFocusEvent>>();
        world.init_resource::<Events<EscapeMenuJustOpened>>();
        world.init_resource::<Events<ShutdownGameCmd>>();
        world.init_resource::<Events<OverwriteBrushPreview>>();
        world.init_resource::<Events<OsCursorVisibilityCmd>>();
        world.init_resource::<Events<PlayUiSound>>();
        register_recording_events(&mut world);
        world.insert_resource(History::<MaybePendingSnapshotId>::empty_glade());
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                hover_intersection,
                ui_update,
                history_start_recording::<MaybePendingSnapshotId>,
                history_finish_recording_system::<MaybePendingSnapshotId>,
            )
                .chain_ignore_deferred(),
        );
        (world, schedule)
    }
    fn frame(world: &mut World, schedule: &mut Schedule, phase: DigitalActionPhase) {
        register_target(world);
        world.resource_mut::<ActionSetStates>().digital[0] = DigitalActionState {
            phase,
            timer_seconds: 0.,
        };
        schedule.run(world);
    }
    #[test]
    fn picked_mouse_press_hold_release_reaches_real_history_lifecycle() {
        let (mut world, mut schedule) = setup();
        world.insert_resource(crate::systems::mode_manager::AppMode::Wall(
            crate::systems::mode_manager::WallShape::Freeform,
        ));
        frame(&mut world, &mut schedule, DigitalActionPhase::Idle);
        assert_eq!(world.resource::<UiState>().hover_byte(), 1);
        assert!(world.resource::<UiState>().registry_snapshot().is_empty());
        frame(&mut world, &mut schedule, DigitalActionPhase::JustPressed);
        assert_eq!(world.resource::<UiState>().click_byte(), 3);
        assert!(world
            .resource::<History<MaybePendingSnapshotId>>()
            .is_recording());
        let captured = world.resource::<UiState>().active_params().unwrap();
        // No registration during a continuing frame: actual picker returns no hit,
        // captured strong Arc is retained and original drag stays phase4.
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::Held;
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiState>().click_byte(), 4);
        assert!(Arc::ptr_eq(
            &captured,
            &world.resource::<UiState>().active_params().unwrap()
        ));
        frame(&mut world, &mut schedule, DigitalActionPhase::JustReleased);
        assert_eq!(world.resource::<UiState>().click_byte(), 1);
        assert!(!world
            .resource::<History<MaybePendingSnapshotId>>()
            .is_recording());
        let sounds: Vec<_> = world
            .resource_mut::<Events<PlayUiSound>>()
            .drain()
            .collect();
        assert_eq!(
            sounds.iter().map(|s| s.kind).collect::<Vec<_>>(),
            [100, 101, 103]
        );
        assert_eq!(sounds[1].native_slot()[4..8], 11u32.to_le_bytes());
    }
    #[test]
    fn native_default_sound_slots_reach_real_hover_press_and_release() {
        let (mut world, mut schedule) = setup();
        world.insert_resource(crate::systems::mode_manager::AppMode::Wall(
            crate::systems::mode_manager::WallShape::Freeform,
        ));
        for phase in [DigitalActionPhase::Idle, DigitalActionPhase::JustPressed,
            DigitalActionPhase::Held, DigitalActionPhase::JustReleased] {
            let params = WidgetParams::default()
                .geometry(WidgetGeometry::ScreenAabb {
                    min: Vec2::splat(40.0), max: Vec2::splat(60.0), distance: 0.5,
                })
                .occlude_by_world(false);
            world.resource::<UiState>().add_with_id(WidgetId(91), params);
            world.resource_mut::<ActionSetStates>().digital[0] = DigitalActionState {
                phase, timer_seconds: 0.0,
            };
            schedule.run(&mut world);
        }
        // 原构造器的声音通过实际picker和捕获生命周期发出；无需手工覆盖声音槽。
        assert_eq!(world.resource_mut::<Events<PlayUiSound>>().drain()
            .map(|sound| sound.kind).collect::<Vec<_>>(), [1, 2, 3]);
    }

    #[test]
    fn actual_pointer_outside_on_release_cancels_captured_widget() {
        let (mut world, mut schedule) = setup();
        world.insert_resource(crate::systems::mode_manager::AppMode::Wall(
            crate::systems::mode_manager::WallShape::Freeform,
        ));
        frame(&mut world, &mut schedule, DigitalActionPhase::JustPressed);
        world.resource_mut::<CursorPositionSS>().0 = Vec2::splat(90.0);
        frame(&mut world, &mut schedule, DigitalActionPhase::JustReleased);
        assert_eq!(world.resource::<UiState>().click_byte(), 0);
        assert!(!world
            .resource::<History<MaybePendingSnapshotId>>()
            .is_recording());
        assert_eq!(
            world
                .resource_mut::<Events<PlayUiSound>>()
                .drain()
                .map(|s| s.kind)
                .collect::<Vec<_>>(),
            [100, 101, 102]
        );
    }
    #[test]
    fn menu_and_last_focus_payload_use_original_forced_end_rules() {
        for end in 0..3 {
            let (mut world, mut schedule) = setup();
            world.insert_resource(crate::systems::mode_manager::AppMode::Wall(
                crate::systems::mode_manager::WallShape::Freeform,
            ));
            frame(&mut world, &mut schedule, DigitalActionPhase::JustPressed);
            match end {
                0 => {
                    world.send_event(EscapeMenuJustOpened);
                }
                1 => {
                    world.send_event(ShutdownGameCmd);
                }
                _ => {
                    world.send_event(WindowFocusEvent(1));
                    world.send_event(WindowFocusEvent(0));
                }
            }
            frame(&mut world, &mut schedule, DigitalActionPhase::Held);
            assert_eq!(world.resource::<UiState>().click_byte(), 1);
            assert!(!world
                .resource::<History<MaybePendingSnapshotId>>()
                .is_recording());
        }
    }
}
