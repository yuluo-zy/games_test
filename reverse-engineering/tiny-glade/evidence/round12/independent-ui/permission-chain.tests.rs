// Include at the bottom of the integrated country-core ui_update module.
// Independent acceptance candidate: unrun. No UiInteraction injection.
#[cfg(test)]
mod independent_permission_chain {
    use super::*;
    use crate::{
        input::action_set::{DigitalActionPhase, DigitalActionState},
        resources::{
            cursor_terrain_raycast::CursorTerrainRaycast,
            history::{History, MaybePendingSnapshotId},
            mouse_terrain_raycast::CursorPositionSS,
            read_mouse_depth_value::MouseDepthValue,
            window::WindowSize,
        },
        systems::{
            camera_rig::MainCamera,
            history::recording::{history_start_recording, register_recording_events},
            mode_manager::{AppMode, WallShape},
            ui::hover_intersection::hover_intersection,
        },
    };
    use glam::{Mat4, Vec2};

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
        world.insert_resource(AppMode::Wall(WallShape::Freeform));
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
            )
                .chain_ignore_deferred(),
        );
        (world, schedule)
    }

    fn press(world: &mut World, schedule: &mut Schedule, no_history: bool, cursor: f32) {
        let mut params = WidgetParams::default()
            .geometry(WidgetGeometry::ScreenAabb {
                min: Vec2::splat(40.0),
                max: Vec2::splat(60.0),
                distance: 0.5,
            })
            .occlude_by_world(false);
        if no_history {
            params = params.no_history_recording();
        }
        world
            .resource::<UiState>()
            .add_with_id(WidgetId(123), params);
        world.resource_mut::<CursorPositionSS>().0 = Vec2::splat(cursor);
        world.resource_mut::<ActionSetStates>().digital[0] = DigitalActionState {
            phase: DigitalActionPhase::JustPressed,
            timer_seconds: 0.0,
        };
        schedule.run(world);
    }

    #[test]
    fn real_picked_recording_permission_survives_cleared_registry() {
        for no_history in [false, true] {
            let (mut world, mut schedule) = setup();
            press(&mut world, &mut schedule, no_history, 50.0);
            let ui = world.resource::<UiState>();
            assert_eq!(ui.click_byte(), 3);
            assert!(ui.registry_snapshot().is_empty());
            let active = ui.active_params().expect("the real picker captured an Arc");
            assert_eq!(active.flags.no_history_recording(), no_history);
            assert_eq!(
                world
                    .resource::<History<MaybePendingSnapshotId>>()
                    .is_recording(),
                !no_history
            );
        }
    }

    #[test]
    fn real_picked_screen_boundary_does_not_start_history() {
        for cursor in [40.0, 60.0] {
            let (mut world, mut schedule) = setup();
            press(&mut world, &mut schedule, false, cursor);
            assert_eq!(world.resource::<UiState>().hover_byte(), 0);
            assert!(world.resource::<UiState>().active_params().is_none());
            assert!(
                !world
                    .resource::<History<MaybePendingSnapshotId>>()
                    .is_recording()
            );
        }
    }
}
