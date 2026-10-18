//! B endpoint continuation (0x140aadd30) and suppression (0x140aadbb0).
//! CPU input uses the existing spatial widget registry, authority/publication
//! and InProgressCurve owner. The signifier renderer closure is independent.
use crate::{
    resources::{time::GameTime, window::CursorMovedScreenSpace},
    systems::backend::input::mod_key::Modifiers,
};
use bevy_ecs::{prelude::*, system::SystemParam};
use glam::{Vec2, Vec3};
use utils::curve::Curve2 as InputCurve;

use crate::{
    resources::{
        color_id::{HalftimberColorIds, WallColorIds},
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        in_progress_curve::InProgressCurve,
        walls::{
            InnerWalls, WallId, decorator_archivist::LiveDecoratorArchivist,
            decorator_storage::DecoratorStorage, in_progress_wall::WallDrawingMetadata,
            public_walls::PublicWalls,
        },
    },
    systems::{
        camera_rig::main_camera::MainCameraMoved,
        mode_manager::{AppMode, WallShape},
        ui::resources::{
            ui_state::UiState,
            widget_params::{WidgetParams, WidgetVolumeShape},
        },
        wall::{
            draw_wall::RecordMouseInputIntoWall,
            finish_wall::{ContinueWallOriginalState, FinishWall},
        },
    },
};

#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SuppressContinueWall {
    pub elapsed: f32,
    pub distance: f32,
    pub suppressed: bool,
}

pub fn suppress_continue_wall(
    modifiers: Res<Modifiers>,
    mut state: ResMut<SuppressContinueWall>,
    time: Res<GameTime>,
    mut motion: EventReader<CursorMovedScreenSpace>,
) {
    if state.suppressed {
        state.elapsed += time.delta;
        // Original clears every pending event, then measures only the last
        // payload. Events remain unread while suppression is inactive.
        if let Some(motion) = motion.read().last() {
            let point = motion.0;
            state.distance += (point.y * point.y + point.x * point.x).sqrt();
        }
        if state.elapsed >= 1.0 || state.distance >= 50.0 {
            state.suppressed = false;
            state.elapsed = 0.0;
            state.distance = 0.0;
        }
    }
    // This check is after the thresholds: holding the modifier re-arms the
    // flag in the same frame, without changing either accumulator here.
    if modifiers.disable_snapping() {
        state.suppressed = true;
    }
}

/// The original one-byte state: idle0, endpoint selected1, dragging2, ended3.
/// The selected wall/end are transient locals, not a second live-wall owner.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct WallContinueState {
    pub state: u8,
}

#[derive(SystemParam)]
pub struct ContinueWallUiEffects<'w> {
    record: EventWriter<'w, RecordMouseInputIntoWall>,
    finish: EventWriter<'w, FinishWall>,
    outside: EventWriter<'w, DisplayOutOfBorder>,
    feedback: EventWriter<'w, FeedbackInputOutOfBorder>,
}

#[derive(SystemParam)]
pub struct ContinueWallColors<'w> {
    wall: Res<'w, WallColorIds>,
    fence: Res<'w, HalftimberColorIds>,
}

/// Stable native format pieces and actual WallId Display specialization.
/// Both endpoint hit volumes use this same name for capture across frames.
pub fn continuation_widget_name(wall: WallId, extend_start: bool) -> String {
    format!("continue_wall_{extend_start}_{wall}")
}

pub fn ui_continue_wall(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    walls: Res<InnerWalls>,
    public: Res<PublicWalls>,
    mut progress: ResMut<InProgressCurve>,
    mut continuation: ResMut<ContinueWallOriginalState>,
    mut state: ResMut<WallContinueState>,
    cursor: Res<CursorTerrainRaycast>,
    suppress: Res<SuppressContinueWall>,
    colors: ContinueWallColors,
    mut camera_moved: EventReader<MainCameraMoved>,
    border: Res<GladeBorder>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    storage: Res<DecoratorStorage>,
    mut effects: ContinueWallUiEffects,
) {
    let fence = match *mode {
        AppMode::Wall(WallShape::Freeform) => false,
        AppMode::Fences => true,
        _ => return,
    };
    if state.state == 3 {
        state.state = 0;
    }
    if state.state == 0 {
        // Suppression must not consume camera events. Active dragging bypasses
        // both gates, so a camera event remains for the next idle invocation.
        if suppress.suppressed {
            return;
        }
        if !camera_moved.is_empty() {
            camera_moved.clear();
            return;
        }
    }
    let mut selected = None;
    for (wall, published) in public.all() {
        // Original iterator subsequently performs the validated get as well.
        published.assert_valid();
        let authority = walls.get(wall);
        if published.is_being_bulldozed() || authority.is_none() {
            if state.state != 0 {
                effects.finish.write(FinishWall(false));
                if state.state.wrapping_sub(1) < 2 {
                    state.state = 3;
                }
            }
            continue;
        }
        // The map probe above checks live existence only. Shape +260 and
        // source byte +296 below belong to the published row, not live source.
        if published.shape().tag() != 2 {
            continue;
        }
        let kind = published.authority.source_flag_65();
        // B preserves raw non-0/1 values; only exactly 0/1 restrict the mode.
        if (kind == 1 && !fence) || (kind == 0 && fence) {
            continue;
        }
        let outline = &published.geometry.outline;
        let first = outline.first().expect("wall outline first point");
        let last = outline.last().expect("wall outline last point");
        for (point, extend_start) in [(first, true), (last, false)] {
            // MAXSS source zero wins for NaN and negative/signed-zero inputs.
            let y = if point[1] > 0.0 { point[1] } else { 0.0 };
            let point = Vec3::new(point[0], y, point[2]);
            let name = continuation_widget_name(wall, extend_start);
            let context = ui.add(
                &name,
                // Sphere variant: tag0, radius1 and actual published position.
                WidgetParams::new(WidgetVolumeShape::sphere(point, 1.0), [0; 2]),
            );
            match context.dragged().wrapping_sub(2).min(3) {
                0 => {}
                1 => {
                    let position = if cursor.tag & 1 != 0 {
                        Vec2::new(cursor.pos.x, cursor.pos.z)
                    } else {
                        Vec2::ZERO
                    };
                    if !border.is_pos_inside(position) {
                        effects.outside.write(DisplayOutOfBorder);
                        effects.feedback.write(FeedbackInputOutOfBorder);
                    }
                    // Unlike fresh drawing, outside still selects/captures the
                    // endpoint. No sampling event is emitted on this phase.
                    state.state = 1;
                    selected = Some((wall, extend_start));
                }
                2 => {
                    if state.state.wrapping_sub(1) < 2 {
                        if state.state == 1 {
                            if let Some(terrain) = storage.terrain.as_ref() {
                                archivist.archive_storage_decorators(1, terrain, None);
                                archivist
                                    .culling
                                    .allowed_ids
                                    .extend(terrain.decorators().keys().copied());
                            } else {
                                // Original archive_current records an empty before bucket too.
                                archivist.archive_storage_decorators(1, &Default::default(), None);
                            }
                        }
                        let position = if cursor.tag & 1 != 0 {
                            Vec2::new(cursor.pos.x, cursor.pos.z)
                        } else {
                            Vec2::ZERO
                        };
                        if border.is_pos_inside(position) {
                            effects.record.write(RecordMouseInputIntoWall);
                        } else {
                            effects.outside.write(DisplayOutOfBorder);
                            effects.feedback.write(FeedbackInputOutOfBorder);
                        }
                        state.state = 2;
                    } else {
                        state.state = 0;
                    }
                }
                _ => {
                    effects.finish.write(FinishWall(false));
                    if state.state.wrapping_sub(1) < 2 {
                        state.state = 3;
                    }
                }
            }
        }
    }
    if let Some((wall, extend_start)) = selected {
        let source = walls.get_unwrapped(wall);
        // Preserve all source and metadata, not just curve/height. Replacement
        // occurs before color indexing and before constructing the new input.
        continuation.previous = Some(source.clone());
        let kind = source.source_flag_65();
        let color_id = if kind == 0 {
            colors.wall.0[wall.0 as usize]
        } else {
            colors.fence.0[wall.0 as usize]
        };
        let source_curve = &source.source().curve;
        let mut curve = InputCurve::crashy_new_from_points(
            source_curve
                .points
                .iter()
                .copied()
                .map(Vec2::from_array)
                .collect(),
            false,
        );
        // An unreversed continuation copies the actual stored parameters.
        curve.points_u.clone_from(&source_curve.points_u);
        curve.length = source_curve.length;
        if extend_start {
            // Same original leaf 0x14146c190: reverse then rebuild parameters.
            curve.reverse();
        }
        *progress = InProgressCurve::from_curve(curve);
        progress.wall = Some(wall);
        progress.current_wall = wall;
        progress.flags = u8::from(extend_start);
        progress.drawing = Some(WallDrawingMetadata {
            first_unsnapped_cursor: None,
            desired_height: source.height(),
            color_id,
            kind,
            freehand_ty: source.freehand_ty(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    #[test]
    fn last_movement_only_and_modifier_rearm_keep_original_event_cursor_order() {
        let mut world = World::new();
        world.insert_resource(Modifiers { flags: 1 });
        world.init_resource::<SuppressContinueWall>();
        world.insert_resource(GameTime {
            delta: 0.2,
            ..Default::default()
        });
        world.init_resource::<Events<CursorMovedScreenSpace>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(suppress_continue_wall);
        // Inactive suppression does not consume this event.
        world
            .resource_mut::<Events<CursorMovedScreenSpace>>()
            .send(CursorMovedScreenSpace(Vec2::new(50.0, 0.0)));
        schedule.run(&mut world);
        let state = world.resource::<SuppressContinueWall>();
        assert!(state.suppressed);
        assert_eq!((state.elapsed, state.distance), (0.0, 0.0));
        world.resource_mut::<Modifiers>().flags = 0;
        schedule.run(&mut world);
        let state = world.resource::<SuppressContinueWall>();
        assert!(!state.suppressed);
        assert_eq!((state.elapsed, state.distance), (0.0, 0.0));
        world.resource_mut::<SuppressContinueWall>().suppressed = true;
        let mut motion = world.resource_mut::<Events<CursorMovedScreenSpace>>();
        motion.send(CursorMovedScreenSpace(Vec2::new(30.0, 40.0)));
        motion.send(CursorMovedScreenSpace(Vec2::new(3.0, 4.0)));
        schedule.run(&mut world);
        let state = world.resource::<SuppressContinueWall>();
        assert!(state.suppressed);
        assert_eq!((state.elapsed, state.distance), (0.2, 5.0));
        world.resource_mut::<SuppressContinueWall>().elapsed = 0.9;
        world.resource_mut::<Modifiers>().flags = 1;
        schedule.run(&mut world);
        let state = world.resource::<SuppressContinueWall>();
        assert!(state.suppressed);
        assert_eq!((state.elapsed, state.distance), (0.0, 0.0));
    }
    fn endpoint_world(fence: bool) -> (World, WallId) {
        use crate::resources::{
            history::History,
            roofs::geometry::surface::Curve2,
            walls::{WallState, publication_source::TerrainHeightGrid512},
        };
        let mut world = World::new();
        world.insert_resource(if fence {
            AppMode::Fences
        } else {
            AppMode::Wall(WallShape::Freeform)
        });
        world.insert_resource(UiState::at_startup());
        world.insert_resource(GladeBorder(
            crate::resources::glade_border::BorderShape::Circle(utils::geometry::Circle2d::new(
                Vec2::ZERO,
                10.0,
            )),
        ));
        let mut walls = InnerWalls::new();
        let source = WallState::new(
            Curve2::try_new(vec![[0.0, 0.0], [1.0, 0.0]], false).unwrap(),
            4.0,
            2,
            u8::from(fence),
            false,
        );
        let wall = walls.new_wall_from_curve(source);
        walls.update_freehand_ty(wall, 1);
        let mut public = PublicWalls::default();
        public.consume(
            &mut walls,
            &TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(),
                minimum_y: -2.0,
                maximum_y: -2.0,
                sampling_fix: false,
            },
        );
        world.insert_resource(walls);
        world.insert_resource(public);
        let mut colors = WallColorIds::new();
        colors.0[wall.0 as usize] = 23;
        world.insert_resource(colors);
        let mut colors = HalftimberColorIds::new();
        colors.0[wall.0 as usize] = 42;
        world.insert_resource(colors);
        world.init_resource::<InProgressCurve>();
        world.init_resource::<ContinueWallOriginalState>();
        world.init_resource::<WallContinueState>();
        world.init_resource::<SuppressContinueWall>();
        world.init_resource::<CursorTerrainRaycast>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<Events<MainCameraMoved>>();
        world.init_resource::<Events<RecordMouseInputIntoWall>>();
        world.init_resource::<Events<FinishWall>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(5).unwrap();
        world.insert_resource(history);
        (world, wall)
    }

    fn endpoint_frame(
        world: &mut World,
        schedule: &mut Schedule,
        wall: WallId,
        extend_start: bool,
        phase: u8,
        point: Vec3,
    ) {
        use crate::systems::ui::resources::ui_state::{UiInteraction, WidgetId};
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: phase,
                widget: WidgetId::from_name(&continuation_widget_name(wall, extend_start)),
            });
        world
            .resource_mut::<CursorTerrainRaycast>()
            .update(Some(point));
        schedule.run(world);
    }

    #[test]
    fn endpoint_select_captures_actual_authority_colors_and_both_directions() {
        for fence in [false, true] {
            for extend_start in [false, true] {
                let (mut world, wall) = endpoint_world(fence);
                let before = world.resource::<InnerWalls>().get_unwrapped(wall).clone();
                let mut schedule = Schedule::default();
                schedule.add_systems(ui_continue_wall);
                // Outside still captures/selects, but does not record a point.
                endpoint_frame(
                    &mut world,
                    &mut schedule,
                    wall,
                    extend_start,
                    3,
                    Vec3::new(20.0, 0.0, 0.0),
                );
                assert_eq!(world.resource::<WallContinueState>().state, 1);
                let previous = world
                    .resource::<ContinueWallOriginalState>()
                    .previous
                    .as_ref()
                    .unwrap();
                assert_eq!(previous.source().curve.points, before.source().curve.points);
                assert_eq!(previous.height(), 4.0);
                assert_eq!(previous.style(), 2);
                assert_eq!(previous.freehand_ty(), 1);
                let progress = world.resource::<InProgressCurve>();
                assert_eq!(
                    (progress.wall, progress.flags),
                    (Some(wall), u8::from(extend_start))
                );
                assert_eq!(
                    progress.raw_input_points,
                    if extend_start {
                        vec![Vec2::new(1.0, 0.0), Vec2::ZERO]
                    } else {
                        vec![Vec2::ZERO, Vec2::new(1.0, 0.0)]
                    }
                );
                let metadata = progress.drawing.as_ref().unwrap();
                assert_eq!(
                    (
                        metadata.desired_height,
                        metadata.color_id,
                        metadata.kind,
                        metadata.freehand_ty
                    ),
                    (4.0, if fence { 42 } else { 23 }, u8::from(fence), 1)
                );
                assert!(metadata.first_unsnapped_cursor.is_none());
                assert!(
                    world
                        .resource::<Events<RecordMouseInputIntoWall>>()
                        .is_empty()
                );
                assert_eq!(world.resource::<Events<DisplayOutOfBorder>>().len(), 1);
                assert_eq!(
                    world.resource::<Events<FeedbackInputOutOfBorder>>().len(),
                    1
                );
                let ui = world.resource::<UiState>();
                for endpoint in [false, true] {
                    use crate::systems::ui::resources::ui_state::WidgetId;
                    let params = ui
                        .widget(WidgetId::from_name(&continuation_widget_name(
                            wall, endpoint,
                        )))
                        .unwrap();
                    assert_eq!(params.volume.tag, 0);
                    assert_eq!(params.volume.size.x, 1.0);
                    assert_eq!(params.volume.center.y.to_bits(), 0.0f32.to_bits());
                    assert!(params.occlude_by_world);
                    assert!(!params.flags.no_history_recording());
                }
            }
        }
    }

    #[test]
    fn active_drag_uses_border_gate_but_keeps_state_two_and_bypasses_idle_gates() {
        let (mut world, wall) = endpoint_world(false);
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_continue_wall);
        endpoint_frame(&mut world, &mut schedule, wall, false, 3, Vec3::X);
        world.resource_mut::<SuppressContinueWall>().suppressed = true;
        world
            .resource_mut::<Events<MainCameraMoved>>()
            .send(MainCameraMoved(Vec3::X));
        endpoint_frame(
            &mut world,
            &mut schedule,
            wall,
            false,
            4,
            Vec3::new(20.0, 0.0, 0.0),
        );
        assert_eq!(world.resource::<WallContinueState>().state, 2);
        assert!(
            world
                .resource::<Events<RecordMouseInputIntoWall>>()
                .is_empty()
        );
        assert_eq!(
            world.resource::<Events<FeedbackInputOutOfBorder>>().len(),
            1
        );
        endpoint_frame(
            &mut world,
            &mut schedule,
            wall,
            false,
            4,
            Vec3::new(2.0, 0.0, 0.0),
        );
        assert_eq!(
            world.resource::<Events<RecordMouseInputIntoWall>>().len(),
            1
        );
        endpoint_frame(
            &mut world,
            &mut schedule,
            wall,
            false,
            1,
            Vec3::new(2.0, 0.0, 0.0),
        );
        assert_eq!(world.resource::<WallContinueState>().state, 3);
        let events = world.resource::<Events<FinishWall>>();
        assert_eq!(
            events
                .get_cursor()
                .read(events)
                .map(|event| event.0)
                .collect::<Vec<_>>(),
            [false]
        );
    }

    #[test]
    fn idle_suppression_leaves_camera_unread_then_camera_blocks_only_one_invocation() {
        let (mut world, wall) = endpoint_world(false);
        world.resource_mut::<WallContinueState>().state = 3;
        world.resource_mut::<SuppressContinueWall>().suppressed = true;
        world
            .resource_mut::<Events<MainCameraMoved>>()
            .send(MainCameraMoved(Vec3::X));
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_continue_wall);
        endpoint_frame(&mut world, &mut schedule, wall, false, 3, Vec3::X);
        assert_eq!(world.resource::<WallContinueState>().state, 0);
        assert!(world.resource::<InProgressCurve>().wall.is_none());
        world.resource_mut::<SuppressContinueWall>().suppressed = false;
        endpoint_frame(&mut world, &mut schedule, wall, false, 3, Vec3::X);
        assert!(world.resource::<InProgressCurve>().wall.is_none());
        endpoint_frame(&mut world, &mut schedule, wall, false, 3, Vec3::X);
        assert_eq!(world.resource::<InProgressCurve>().wall, Some(wall));
    }

    #[test]
    fn missing_live_authority_emits_false_finish_without_fabricating_a_source() {
        let (mut world, wall) = endpoint_world(false);
        world.resource_mut::<WallContinueState>().state = 2;
        world.resource_mut::<InnerWalls>().remove(wall);
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_continue_wall);
        schedule.run(&mut world);
        assert_eq!(world.resource::<WallContinueState>().state, 3);
        assert!(
            world
                .resource::<ContinueWallOriginalState>()
                .previous
                .is_none()
        );
        let events = world.resource::<Events<FinishWall>>();
        assert_eq!(
            events
                .get_cursor()
                .read(events)
                .map(|event| event.0)
                .collect::<Vec<_>>(),
            [false]
        );
    }

    #[test]
    fn endpoint_gate_uses_published_shape_while_begin_captures_current_live_source() {
        use crate::resources::walls::publication_fields::WallShapeSnapshot;
        let (mut world, wall) = endpoint_world(false);
        let mut changed_shape = [0; 28];
        changed_shape[..4].copy_from_slice(&3u32.to_le_bytes());
        world
            .resource_mut::<InnerWalls>()
            .update_wall_state_enclosure(wall, WallShapeSnapshot::from_observed(changed_shape));
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_continue_wall);
        // Public shape remains2 even though current source has changed to3.
        endpoint_frame(&mut world, &mut schedule, wall, false, 3, Vec3::X);
        assert_eq!(world.resource::<InProgressCurve>().wall, Some(wall));
        assert_eq!(
            world
                .resource::<ContinueWallOriginalState>()
                .previous
                .as_ref()
                .unwrap()
                .source()
                .shape
                .tag(),
            3
        );
    }

    #[test]
    fn actual_endpoint_ui_draw_reverse_finish_and_deferred_history_share_one_owner() {
        use crate::{
            game_config::GameConfig,
            resources::{
                color_id::{SetHalftimberColorIdCmd, SetWallColorIdCmd},
                glade_color_customization::GladeColorCustomization,
                history::{EditType, History, wall_edit::WallHistoryEdit},
                onboarding::NotifyHintSystemToolWasJustUsed,
                window::WindowSize,
            },
            systems::{
                audio::RequestWallMaterialAudio,
                camera_rig::main_camera::MainCamera,
                history::recording::HistoryRecordingFailed,
                wall::{
                    adjust_wall_height::AdjustWallHeightCmd,
                    deferred_wall_history::{PendingWallHistory, history_record_wall_changed},
                    draw_wall::{CurveNursery, WallDrawingIssues, draw_wall},
                    finish_wall::finish_wall,
                    history_recording::{RecordWallHistory, record_wall_history},
                    snapping::SnappingMode,
                },
            },
        };
        for fence in [false, true] {
            for extend_start in [false, true] {
                let (mut world, wall) = endpoint_world(fence);
                world.init_resource::<CurveNursery>();
                world.init_resource::<SnappingMode>();
                world.init_resource::<PendingWallHistory>();
                world.init_resource::<WallDrawingIssues>();
                world.init_resource::<GameConfig>();
                world.init_resource::<GladeColorCustomization>();
                world.insert_resource(GameTime {
                    ticks_60: 1,
                    delta: 1.0 / 60.0,
                    ..Default::default()
                });
                world.insert_resource(WindowSize::new(1920, 1080));
                let mut camera = MainCamera::default();
                camera.write_f32(0x14, 1.0);
                camera.write_f32(0xb8, -10.0);
                world.insert_resource(camera);
                world.init_resource::<Events<RecordWallHistory>>();
                world.init_resource::<Events<HistoryRecordingFailed>>();
                world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
                world.init_resource::<Events<SetWallColorIdCmd>>();
                world.init_resource::<Events<SetHalftimberColorIdCmd>>();
                world.init_resource::<Events<AdjustWallHeightCmd>>();
                world.init_resource::<Events<RequestWallMaterialAudio>>();
                let mut schedule = Schedule::default();
                schedule.add_systems(
                    (
                        ui_continue_wall,
                        draw_wall,
                        finish_wall,
                        history_record_wall_changed,
                        record_wall_history::<()>,
                    )
                        .chain(),
                );
                let start = if extend_start { Vec3::ZERO } else { Vec3::X };
                let next = if extend_start {
                    -Vec3::X
                } else {
                    2.0 * Vec3::X
                };
                endpoint_frame(&mut world, &mut schedule, wall, extend_start, 3, start);
                assert!(
                    world
                        .resource::<Events<RequestWallMaterialAudio>>()
                        .is_empty()
                );
                endpoint_frame(&mut world, &mut schedule, wall, extend_start, 4, start);
                endpoint_frame(&mut world, &mut schedule, wall, extend_start, 4, next);
                let live = world.resource::<InnerWalls>().get_unwrapped(wall);
                assert_eq!(
                    live.source().curve.points.first().copied(),
                    Some(if extend_start {
                        [-1.0, 0.0]
                    } else {
                        [0.0, 0.0]
                    })
                );
                assert_eq!(
                    live.source().curve.points.last().copied(),
                    Some(if extend_start { [1.0, 0.0] } else { [2.0, 0.0] })
                );
                assert_eq!(
                    world.resource::<Events<RequestWallMaterialAudio>>().len(),
                    1
                );
                // Captured desired height remains distinct from live authority.
                world
                    .resource_mut::<InnerWalls>()
                    .update_wall_state_height(wall, 2.0);
                endpoint_frame(&mut world, &mut schedule, wall, extend_start, 1, next);
                assert!(
                    world
                        .resource::<ContinueWallOriginalState>()
                        .previous
                        .is_none()
                );
                assert!(world.resource::<InProgressCurve>().wall.is_none());
                assert_eq!(
                    world.resource::<InnerWalls>().get_unwrapped(wall).height(),
                    2.0
                );
                assert!(world.resource::<WallDrawingIssues>().0.is_empty());
                assert!(
                    world
                        .resource::<Events<HistoryRecordingFailed>>()
                        .is_empty()
                );
                let edits = &world.resource::<History>().pending().unwrap().edits;
                assert_eq!(edits.len(), 1);
                let EditType::Wall(compound) = &edits[0] else {
                    panic!("wall edit");
                };
                assert_eq!((compound.groups.len(), compound.groups[0].len()), (1, 1));
                let WallHistoryEdit::StateChange {
                    previous,
                    next,
                    observed_operation,
                    ..
                } = &compound.groups[0][0]
                else {
                    panic!("real captured expansion");
                };
                assert_eq!(previous.source.curve.points, [[0.0, 0.0], [1.0, 0.0]]);
                assert_eq!((previous.source.extra, next.source.extra), (4.0, 4.0));
                let lo = f32::from_le_bytes(observed_operation[4..8].try_into().unwrap());
                let hi = f32::from_le_bytes(observed_operation[8..12].try_into().unwrap());
                assert_eq!((lo, hi), if extend_start { (0.5, 1.0) } else { (0.0, 0.5) });
            }
        }
    }
    #[test]
    fn empty_terrain_before_is_archived_once_and_survives_later_bucket_creation() {
        use crate::resources::walls::{
            decorator::{DecoratorId, DecoratorInfo},
            decorator_storage::Decorators,
        };
        let (mut world, wall) = endpoint_world(false);
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_continue_wall);
        endpoint_frame(&mut world, &mut schedule, wall, false, 3, Vec3::X);
        assert!(
            world
                .resource::<LiveDecoratorArchivist>()
                .special1
                .is_none()
        );
        endpoint_frame(&mut world, &mut schedule, wall, false, 4, Vec3::X);
        assert!(world.resource::<DecoratorStorage>().terrain.is_none());
        let archive = world.resource::<LiveDecoratorArchivist>();
        let before = archive
            .special1
            .as_ref()
            .expect("empty terrain before must be Some");
        assert!(before.items.is_empty() && before.removed.is_empty() && before.added.is_empty());
        assert!(archive.culling.allowed_ids.is_empty());
        let mut terrain = Decorators::default();
        terrain.insert(DecoratorId(17), DecoratorInfo::default());
        world.resource_mut::<DecoratorStorage>().terrain = Some(terrain);
        world.resource_mut::<WallContinueState>().state = 1;
        endpoint_frame(&mut world, &mut schedule, wall, false, 4, Vec3::X);
        let archive = world.resource::<LiveDecoratorArchivist>();
        let before = archive.special1.as_ref().unwrap();
        assert!(before.items.is_empty() && before.removed.is_empty() && before.added.is_empty());
        // Only cull permission reads today's real bucket on the second call.
        assert!(archive.culling.allowed_ids.contains(&17));
    }
}
