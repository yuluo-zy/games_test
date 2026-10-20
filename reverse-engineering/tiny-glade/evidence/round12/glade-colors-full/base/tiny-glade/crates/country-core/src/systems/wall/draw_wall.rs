//! B freehand wall creation and continued sampling, 0x140a7cba0. Reuses the
//! current InnerWalls, InProgressCurve, camera, clock and command consumers.
//! Ordinary continuation reuses the actual saved authority and both curve
//! directions. Snapped topology retains an explicit unresolved diagnostic.
use bevy_ecs::{prelude::*, system::SystemParam};
use glam::{Vec2, Vec3};
use utils::curve::Curve2 as InputCurve;

use crate::{
    game_config::GameConfig,
    resources::{
        color_id::{SetHalftimberColorIdCmd, SetWallColorIdCmd},
        cursor_terrain_raycast::CursorTerrainRaycast,
        drawing_input_smoother::DrawingInputSmoother,
        glade_color_customization::GladeColorCustomization,
        in_progress_curve::InProgressCurve,
        roofs::geometry::surface::Curve2,
        time::GameTime,
        walls::{InnerWalls, WallId, WallState, in_progress_wall::WallDrawingMetadata},
        window::WindowSize,
    },
    systems::{
        audio::RequestWallMaterialAudio,
        camera_rig::main_camera::MainCamera,
        wall::{
            adjust_wall_height::AdjustWallHeightCmd, finish_wall::ContinueWallOriginalState,
            height_command::AdjustWallHeightRequest, shapes::to_authority_curve,
            snapping::SnappingMode,
        },
    },
};

/// Original reader clears all pending unit events, sampling the current ray
/// once. No pointer coordinates are invented in this event.
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct RecordMouseInputIntoWall;

#[derive(Resource, Clone, Debug, Default)]
pub struct CurveNursery {
    pub points: Vec<Vec2>,
    /// Original +0x18, supplied by the actual wall/fence UI start.
    pub kind: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WallDrawingIssue {
    SnappedTopology {
        wall: WallId,
        mode: u8,
        point: Vec3,
    },
    ContinuationProjectionOutside {
        wall: WallId,
        coordinate: f32,
    },
    MissingDrawingMetadata {
        wall: WallId,
        point: Vec3,
    },
    RejectedRawAppend {
        wall: WallId,
        point: Vec3,
    },
    FinalCurveRejected {
        wall: WallId,
    },
    TooManyFinalPoints {
        wall: WallId,
        count: usize,
    },
    UnsupportedFinish {
        wall: WallId,
        commit: bool,
        snapped_mode: u8,
        continued: bool,
    },
    MissingFinishMetadata {
        wall: WallId,
        commit: bool,
    },
}

/// Failure observations keep the original input and leave live authority and
/// pending curve state available for recovery. They are not success events.
#[derive(Resource, Debug, Default)]
pub struct WallDrawingIssues(pub Vec<WallDrawingIssue>);

#[derive(SystemParam)]
pub struct WallDrawCommands<'w> {
    wall_color: EventWriter<'w, SetWallColorIdCmd>,
    fence_color: EventWriter<'w, SetHalftimberColorIdCmd>,
    height: EventWriter<'w, AdjustWallHeightCmd>,
    material_audio: EventWriter<'w, RequestWallMaterialAudio>,
}

/// B no-new-point path: round(60 / round(1 / max(delta, 1e-5))), at least one.
/// The existing curve method performs the native wrapping add and 1..=50 clamp.
pub fn stationary_point_weight(delta: f32) -> usize {
    (60.0 / (1.0 / delta.max(0.00001)).round()).round().max(1.0) as usize
}

pub fn draw_wall(
    mut requests: EventReader<RecordMouseInputIntoWall>,
    mut walls: ResMut<InnerWalls>,
    mut progress: ResMut<InProgressCurve>,
    mut nursery: ResMut<CurveNursery>,
    snap: Res<SnappingMode>,
    cursor: Res<CursorTerrainRaycast>,
    continuation: Res<ContinueWallOriginalState>,
    time: Res<GameTime>,
    camera: Res<MainCamera>,
    window: Res<WindowSize>,
    settings: Res<GladeColorCustomization>,
    config: Res<GameConfig>,
    mut commands: WallDrawCommands,
    mut issues: ResMut<WallDrawingIssues>,
) {
    if requests.is_empty() {
        return;
    }
    requests.clear();
    let point = if cursor.tag & 1 != 0 {
        cursor.pos
    } else {
        Vec3::ZERO
    };
    let Some(wall) = progress.wall else {
        // Native checks the accumulated length before sampling this frame.
        // A second accepted point therefore creates a wall on the next input.
        if nursery.points.len() > 1 {
            let input = InputCurve::try_new_from_points(std::mem::take(&mut nursery.points), false)
                .expect("original nursery curve must be valid");
            let kind = u8::from(nursery.kind != 0);
            let height = if kind == 0 { 3.9 } else { 1.2 };
            let state =
                WallState::new(to_authority_curve(&input), height, 0, kind, config.dev.demo);
            let wall = walls.new_wall_from_curve(state);
            let freehand_ty = if kind == 0 { settings.freehand_ty } else { 0 };
            let color_id = if kind == 0 {
                settings.freeform_wall.0
            } else {
                settings.fence.0
            };
            *progress = InProgressCurve::from_curve(input);
            progress.wall = Some(wall);
            progress.current_wall = wall;
            progress.drawing = Some(WallDrawingMetadata {
                first_unsnapped_cursor: None,
                desired_height: height,
                color_id,
                kind,
                freehand_ty,
            });
            walls.update_freehand_ty(wall, freehand_ty);
        } else {
            let mut smoother = DrawingInputSmoother::new(nursery.points.last().copied());
            nursery
                .points
                .extend_from_slice(smoother.process(point, &time, &camera, &window).points());
        }
        return;
    };
    if snap.mode > 1 {
        issues.0.push(WallDrawingIssue::SnappedTopology {
            wall,
            mode: snap.mode,
            point,
        });
        return;
    }
    let Some(metadata) = progress.drawing.as_mut() else {
        issues
            .0
            .push(WallDrawingIssue::MissingDrawingMetadata { wall, point });
        return;
    };
    metadata
        .first_unsnapped_cursor
        .get_or_insert(Vec2::new(point.x, point.z));
    let desired_height = metadata.desired_height;
    let color_id = metadata.color_id;
    let kind = metadata.kind;
    let freehand_ty = metadata.freehand_ty;
    let mut smoother = DrawingInputSmoother::new(progress.raw_input_points.last().copied());
    let accepted = smoother.process(point, &time, &camera, &window);
    let added = if accepted.is_empty() {
        progress.add_weight_to_the_last_point(stationary_point_weight(time.delta));
        false
    } else {
        let added = progress.try_add_raw_input_points(accepted.points());
        if !added {
            issues
                .0
                .push(WallDrawingIssue::RejectedRawAppend { wall, point });
        }
        added
    };
    if kind == 0 {
        commands
            .wall_color
            .write(SetWallColorIdCmd::new(wall, color_id, 0));
    } else {
        commands
            .fence_color
            .write(SetHalftimberColorIdCmd::new(wall, color_id, 0));
    }
    walls.update_freehand_ty(wall, freehand_ty);
    let old_height = walls.get_unwrapped(wall).height();
    let alpha = time.delta * 15.0;
    let height = alpha * desired_height + (1.0 - alpha) * old_height;
    if (height - old_height).abs() > 0.01 {
        commands
            .height
            .write(AdjustWallHeightCmd(AdjustWallHeightRequest {
                anchor: Some(point.to_array()),
                mode: 1,
                value: height,
                wall: crate::resources::walls::publication_fields::WallId(wall.0),
                observed_audio_flag: 0,
            }));
    }
    if !added || !progress.is_raw_input_curve_valid() {
        return;
    }
    let Some(curve) = progress.compute_final_curve() else {
        issues.0.push(WallDrawingIssue::FinalCurveRejected { wall });
        return;
    };
    let curve =
        match continued_output_curve(curve, progress.flags == 1, continuation.previous.as_ref()) {
            Ok(curve) => curve,
            Err(coordinate) => {
                issues
                    .0
                    .push(WallDrawingIssue::ContinuationProjectionOutside { wall, coordinate });
                return;
            }
        };
    if curve.points.len() > 999 {
        issues.0.push(WallDrawingIssue::TooManyFinalPoints {
            wall,
            count: curve.points.len(),
        });
        return;
    }
    walls.update_wall_state_curve(wall, curve);
    commands.material_audio.write(RequestWallMaterialAudio {
        id: wall.0,
        position: point,
        kind: 2,
        flag: 0,
    });
}

/// 0x140a7e89b..93a: restore the authority direction first, then test the
/// previous source's last endpoint on that output. Preserve stored parameters
/// unless the original reverse leaf itself recomputes them.
fn continued_output_curve(
    curve: Curve2,
    reverse: bool,
    previous: Option<&WallState>,
) -> Result<Curve2, f32> {
    if !reverse && previous.is_none() {
        return Ok(curve);
    }
    let mut input = InputCurve::crashy_new_from_points(
        curve.points.iter().copied().map(Vec2::from_array).collect(),
        false,
    );
    input.points_u.clone_from(&curve.points_u);
    input.length = curve.length;
    if reverse {
        input.reverse();
    }
    if let Some(previous) = previous {
        let endpoint = *previous
            .source()
            .curve
            .points
            .last()
            .expect("previous wall curve last point");
        let coordinate = input.get_approx_u_from_pos(Vec2::from_array(endpoint));
        if !(coordinate >= 0.0 && coordinate <= 1.0) {
            return Err(coordinate);
        }
    }
    Ok(Curve2 {
        points: input
            .points
            .into_iter()
            .map(|point| point.to_array())
            .collect(),
        points_u: input.points_u,
        length: input.length,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resources::{
            glade_border::{
                BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
            },
            history::{EditType, History, wall_edit::WallHistoryEdit},
            onboarding::NotifyHintSystemToolWasJustUsed,
            tree_storage::HideTreesNextToWallCmd,
            walls::{
                decorator_archivist::LiveDecoratorArchivist, decorator_storage::DecoratorStorage,
            },
        },
        systems::{
            audio::WallBendAudioParamReset,
            history::{history_process::DebugHistoryMsg, recording::HistoryRecordingFailed},
            mode_manager::{AppMode, WallShape},
            ui::resources::ui_state::{UiInteraction, UiState, WidgetId},
            wall::{
                finish_wall::{FinishWall, finish_wall},
                history_recording::{RecordWallHistory, record_wall_history},
                ui_continue_wall::SuppressContinueWall,
                ui_draw_wall::{WallDrawState, ui_draw_wall},
            },
        },
    };

    #[test]
    fn continued_direction_rebuilds_parameters_and_last_previous_endpoint_gate_is_ordered() {
        let previous = WallState::new(
            Curve2::try_new(vec![[0.0, 0.0], [1.0, 0.0]], false).unwrap(),
            4.0,
            0,
            0,
            false,
        );
        let mut reverse =
            Curve2::try_new(vec![[1.0, 0.0], [0.0, 0.0], [-2.0, 0.0]], false).unwrap();
        // Original reverse rebuilds the cache; it does not map 1-old_u.
        reverse.points_u = vec![0.0, 0.125, 1.0];
        let output = continued_output_curve(reverse, true, Some(&previous)).unwrap();
        assert_eq!(output.points, [[-2.0, 0.0], [0.0, 0.0], [1.0, 0.0]]);
        assert_eq!(output.points_u, [0.0, 2.0 / 3.0, 1.0]);
        let mut outside = Curve2::try_new(vec![[0.0, 0.0], [2.0, 0.0]], false).unwrap();
        outside.points_u = vec![2.0, 4.0];
        assert!(matches!(
            continued_output_curve(outside, false, Some(&previous)),
            Err(3.0)
        ));
    }

    #[test]
    fn original_roundf_stationary_weight_non_integral_ratios() {
        // Both original calls resolve through IAT143193198 to CRT roundf.
        // delta .04 gives round(60 / round(25)) = round(2.4) = 2.
        for (delta, expected) in [
            (0.0, 1),
            (-1.0, 1),
            (1.0 / 60.0, 1),
            (0.03, 2),
            (0.04, 2),
            (1.0 / 14.0, 4),
            (0.06, 4),
            (0.1, 6),
        ] {
            assert_eq!(stationary_point_weight(delta), expected);
        }
    }

    fn world(fence: bool) -> World {
        let mut world = World::new();
        world.insert_resource(if fence {
            AppMode::Fences
        } else {
            AppMode::Wall(WallShape::Freeform)
        });
        world.insert_resource(UiState::at_startup());
        world.insert_resource(GladeBorder(BorderShape::Circle(
            utils::geometry::Circle2d::new(Vec2::ZERO, 100.0),
        )));
        world.init_resource::<CursorTerrainRaycast>();
        world.init_resource::<WallDrawState>();
        world.init_resource::<CurveNursery>();
        world.init_resource::<InnerWalls>();
        world.init_resource::<InProgressCurve>();
        world.init_resource::<ContinueWallOriginalState>();
        world.init_resource::<SuppressContinueWall>();
        world.init_resource::<SnappingMode>();
        world.init_resource::<WallDrawingIssues>();
        world.init_resource::<crate::systems::wall::deferred_wall_history::PendingWallHistory>();
        world.init_resource::<crate::resources::color_id::WallColorIds>();
        world.init_resource::<crate::resources::color_id::HalftimberColorIds>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<GameConfig>();
        let mut settings = GladeColorCustomization::default();
        settings.freeform_wall.0 = 23;
        settings.fence.0 = 42;
        settings.freehand_ty = 1;
        world.insert_resource(settings);
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
        world.init_resource::<Events<RecordMouseInputIntoWall>>();
        world.init_resource::<Events<FinishWall>>();
        world.init_resource::<Events<RecordWallHistory>>();
        world.init_resource::<Events<HistoryRecordingFailed>>();
        world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
        world.init_resource::<Events<DebugHistoryMsg>>();
        world.init_resource::<Events<WallBendAudioParamReset>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<HideTreesNextToWallCmd>>();
        world.init_resource::<Events<SetWallColorIdCmd>>();
        world.init_resource::<Events<SetHalftimberColorIdCmd>>();
        world.init_resource::<Events<AdjustWallHeightCmd>>();
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(5).unwrap();
        world.insert_resource(history);
        world
    }

    fn ui_frame(world: &mut World, schedule: &mut Schedule, phase: u8, point: Vec3) {
        let name = if matches!(*world.resource::<AppMode>(), AppMode::Fences) {
            "fences"
        } else {
            "wall"
        };
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: phase,
                widget: WidgetId::clearing_canvas(name),
            });
        world
            .resource_mut::<CursorTerrainRaycast>()
            .update(Some(point));
        schedule.run(world);
    }

    fn schedule() -> Schedule {
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ui_draw_wall,
                draw_wall,
                finish_wall,
                record_wall_history::<()>,
            )
                .chain(),
        );
        schedule
    }

    #[test]
    fn real_world_wall_and_fence_input_curve_authority_and_two_group_history() {
        for fence in [false, true] {
            let mut world = world(fence);
            let mut schedule = schedule();
            ui_frame(&mut world, &mut schedule, 3, Vec3::ZERO);
            assert_eq!(world.resource::<CurveNursery>().points.len(), 1);
            assert!(world.resource::<InProgressCurve>().wall.is_none());
            ui_frame(&mut world, &mut schedule, 4, Vec3::new(0.1, 0.0, 0.0));
            assert_eq!(world.resource::<CurveNursery>().points.len(), 2);
            assert!(world.resource::<InProgressCurve>().wall.is_none());
            ui_frame(&mut world, &mut schedule, 4, Vec3::new(0.2, 0.0, 0.0));
            let wall = world.resource::<InProgressCurve>().wall.unwrap();
            assert_eq!(wall, WallId(1));
            assert_eq!(
                world.resource::<InProgressCurve>().raw_input_points,
                [Vec2::ZERO, Vec2::new(0.1, 0.0)]
            );
            assert!(world.resource::<CurveNursery>().points.is_empty());
            let authority = world.resource::<InnerWalls>().get_unwrapped(wall);
            assert_eq!(authority.height(), if fence { 1.2 } else { 3.9 });
            assert_eq!(authority.source_flag_65(), u8::from(fence));
            assert_eq!(authority.freehand_ty(), if fence { 0 } else { 1 });
            assert!(
                world
                    .resource::<InnerWalls>()
                    .changes()
                    .created
                    .contains(&wall)
            );
            ui_frame(&mut world, &mut schedule, 4, Vec3::new(0.3, 0.0, 0.1));
            let before_wait = world
                .resource::<InProgressCurve>()
                .point_weights()
                .last()
                .copied()
                .unwrap();
            assert!(
                world
                    .resource::<InnerWalls>()
                    .get_unwrapped(wall)
                    .source()
                    .curve
                    .length
                    >= 0.2
            );
            assert_eq!(
                world.resource::<Events<RequestWallMaterialAudio>>().len(),
                1
            );
            let audio = world.resource::<Events<RequestWallMaterialAudio>>();
            let request = *audio.get_cursor().read(audio).last().unwrap();
            assert_eq!((request.id, request.kind, request.flag), (wall.0, 2, 0));
            assert_eq!(request.position, Vec3::new(0.3, 0.0, 0.1));
            ui_frame(&mut world, &mut schedule, 4, Vec3::new(0.3, 0.0, 0.1));
            assert_eq!(
                world
                    .resource::<InProgressCurve>()
                    .point_weights()
                    .last()
                    .copied(),
                Some(before_wait + 1)
            );
            assert_eq!(
                world.resource::<Events<RequestWallMaterialAudio>>().len(),
                1
            );
            // The live source can differ from the requested draw height. Only
            // the captured history clone receives metadata.desired_height.
            world
                .resource_mut::<InnerWalls>()
                .update_wall_state_height(wall, 2.0);
            ui_frame(&mut world, &mut schedule, 1, Vec3::new(0.3, 0.0, 0.1));
            assert!(world.resource::<InProgressCurve>().wall.is_none());
            assert_eq!(
                world.resource::<InnerWalls>().get_unwrapped(wall).height(),
                2.0
            );
            assert!(world.resource::<SuppressContinueWall>().suppressed);
            assert!(world.resource::<WallDrawingIssues>().0.is_empty());
            assert!(
                world
                    .resource::<Events<HistoryRecordingFailed>>()
                    .is_empty()
            );
            let pending = world.resource::<History<()>>().pending().unwrap();
            assert_eq!(pending.edits.len(), 1);
            let EditType::Wall(record) = &pending.edits[0] else {
                panic!("wall history domain");
            };
            assert_eq!(record.groups.len(), 2);
            let WallHistoryEdit::Create {
                wall: recorded,
                color_id,
                restored_height,
                ..
            } = &record.groups[0][0]
            else {
                panic!("create group");
            };
            assert_eq!(recorded.0, wall.0);
            assert_eq!(*color_id, if fence { 42 } else { 23 });
            assert_eq!(*restored_height, if fence { 1.2 } else { 3.9 });
            let WallHistoryEdit::StateChange { next, .. } = &record.groups[1][0] else {
                panic!("growth group");
            };
            assert_eq!(next.source.extra, *restored_height);
            assert_eq!(
                world
                    .resource::<Events<NotifyHintSystemToolWasJustUsed>>()
                    .len(),
                1
            );
        }
    }

    #[test]
    fn too_short_finished_wall_is_removed_without_recording() {
        let mut world = world(false);
        let mut schedule = schedule();
        for (phase, x) in [(3, 0.0), (4, 0.1), (4, 0.2)] {
            ui_frame(&mut world, &mut schedule, phase, Vec3::new(x, 0.0, 0.0));
        }
        let wall = world.resource::<InProgressCurve>().wall.unwrap();
        ui_frame(&mut world, &mut schedule, 1, Vec3::new(0.2, 0.0, 0.0));
        assert!(!world.resource::<InnerWalls>().contains(wall));
        assert!(
            world
                .resource::<InnerWalls>()
                .changes()
                .removed
                .contains(&wall)
        );
        assert!(world.resource::<InProgressCurve>().wall.is_none());
        assert!(
            world
                .resource::<History<()>>()
                .pending()
                .unwrap()
                .edits
                .is_empty()
        );
        assert!(
            world
                .resource::<Events<NotifyHintSystemToolWasJustUsed>>()
                .is_empty()
        );
    }

    #[test]
    fn rejected_append_preserves_authority_and_reports_input_without_success_audio() {
        let mut world = world(false);
        let input = InputCurve::crashy_new_from_points(vec![Vec2::ZERO, Vec2::X], false);
        let wall = world.resource_mut::<InnerWalls>().new_wall_w_params(
            to_authority_curve(&input),
            3.9,
            0,
            0,
            false,
        );
        let mut progress = InProgressCurve::from_curve(input);
        progress.wall = Some(wall);
        progress.current_wall = wall;
        progress.drawing = Some(WallDrawingMetadata {
            first_unsnapped_cursor: None,
            desired_height: 3.9,
            color_id: 23,
            kind: 0,
            freehand_ty: 1,
        });
        progress.raw_input_points.push(Vec2::X);
        world.insert_resource(progress);
        world
            .resource_mut::<CursorTerrainRaycast>()
            .update(Some(Vec3::new(2.0, 0.0, 0.0)));
        world
            .resource_mut::<Events<RecordMouseInputIntoWall>>()
            .send(RecordMouseInputIntoWall);
        let mut schedule = Schedule::default();
        schedule.add_systems(draw_wall);
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get_unwrapped(wall)
                .source()
                .curve
                .points,
            [[0.0, 0.0], [1.0, 0.0]]
        );
        assert_eq!(
            world.resource::<InProgressCurve>().raw_input_points.len(),
            4
        );
        assert!(
            matches!(world.resource::<WallDrawingIssues>().0.as_slice(), [WallDrawingIssue::RejectedRawAppend { wall: id, point }] if *id == wall && *point == Vec3::new(2.0,0.0,0.0))
        );
        assert!(
            world
                .resource::<Events<RequestWallMaterialAudio>>()
                .is_empty()
        );
    }
    #[test]
    fn unsupported_snap_keeps_the_live_curve_and_exact_last_finish_request() {
        let mut world = world(false);
        let input = InputCurve::crashy_new_from_points(vec![Vec2::ZERO, Vec2::X], false);
        let wall = world.resource_mut::<InnerWalls>().new_wall_w_params(
            to_authority_curve(&input),
            3.9,
            0,
            0,
            false,
        );
        let mut progress = InProgressCurve::from_curve(input);
        progress.wall = Some(wall);
        progress.current_wall = wall;
        progress.drawing = Some(WallDrawingMetadata {
            first_unsnapped_cursor: None,
            desired_height: 3.9,
            color_id: 23,
            kind: 0,
            freehand_ty: 1,
        });
        world.insert_resource(progress);
        world.resource_mut::<SnappingMode>().mode = 2;
        world
            .resource_mut::<CursorTerrainRaycast>()
            .update(Some(Vec3::new(4.0, 2.0, 3.0)));
        world
            .resource_mut::<Events<RecordMouseInputIntoWall>>()
            .send(RecordMouseInputIntoWall);
        let mut draw = Schedule::default();
        draw.add_systems(draw_wall);
        draw.run(&mut world);
        assert_eq!(
            world.resource::<InProgressCurve>().raw_input_points,
            [Vec2::ZERO, Vec2::X]
        );
        assert!(
            matches!(world.resource::<WallDrawingIssues>().0.as_slice(), [WallDrawingIssue::SnappedTopology { wall: id, mode: 2, point }] if *id == wall && *point == Vec3::new(4.0,2.0,3.0))
        );
        world.resource_mut::<SnappingMode>().mode = 2;
        world
            .resource_mut::<Events<FinishWall>>()
            .send(FinishWall(false));
        world
            .resource_mut::<Events<FinishWall>>()
            .send(FinishWall(true));
        let mut finish = Schedule::default();
        finish.add_systems(finish_wall);
        finish.run(&mut world);
        assert_eq!(world.resource::<InProgressCurve>().wall, Some(wall));
        assert!(world.resource::<InnerWalls>().contains(wall));
        assert!(
            matches!(world.resource::<WallDrawingIssues>().0.last(), Some(WallDrawingIssue::UnsupportedFinish { wall: id, commit: true, snapped_mode: 2, continued: false }) if *id == wall)
        );
        assert!(world.resource::<Events<RecordWallHistory>>().is_empty());
    }
    #[test]
    fn first_wall_and_fence_ui_operation_commits_through_actual_history_lifecycle() {
        use crate::systems::{
            backend::input::{
                input_record_replay::{
                    InputRecordReplayState, InputReplayCommand, control_input_replay,
                },
                register_input_events,
            },
            history::{
                HistoryCpuStage,
                recording::{add_recording_systems, register_recording_events},
            },
            wall::registration::{add_wall_drawing_cpu_systems, register_wall_drawing_resources},
        };
        for fence in [false, true] {
            let mut world = world(fence);
            // Unlike the candidate-only pending test, start from the actual
            // empty history and absent canvas registry for the first input.
            world.insert_resource(History::<()>::empty_glade());
            register_wall_drawing_resources(&mut world);
            register_recording_events(&mut world);
            register_input_events(&mut world);
            world.init_resource::<crate::resources::toolbar::Toolbar>();
            world.init_resource::<crate::resources::mouse_capture::CursorCapturedPosition>();
            let name = if fence { "fences" } else { "wall" };
            let widget = WidgetId::clearing_canvas(name);
            assert!(
                world
                    .resource::<UiState>()
                    .clearing_canvas_params(widget)
                    .is_none()
            );
            assert!(!world.resource::<History<()>>().is_recording());
            let mut schedule = Schedule::default();
            schedule.add_systems(control_input_replay.before(ui_draw_wall));
            add_wall_drawing_cpu_systems::<()>(&mut schedule);
            schedule.add_systems(record_wall_history::<()>.in_set(HistoryCpuStage::Record));
            add_recording_systems::<()>(&mut schedule);
            ui_frame(&mut world, &mut schedule, 3, Vec3::ZERO);
            assert!(
                world
                    .resource::<UiState>()
                    .clearing_canvas_params(widget)
                    .is_some()
            );
            assert!(world.resource::<History<()>>().is_recording());
            ui_frame(&mut world, &mut schedule, 4, Vec3::new(0.1, 0., 0.));
            ui_frame(&mut world, &mut schedule, 4, Vec3::new(0.2, 0., 0.));
            let wall = world.resource::<InProgressCurve>().wall.unwrap();
            ui_frame(&mut world, &mut schedule, 4, Vec3::new(0.3, 0., 0.1));
            ui_frame(&mut world, &mut schedule, 1, Vec3::new(0.3, 0., 0.1));
            assert!(world.resource::<InnerWalls>().get(wall).is_some());
            assert!(world.resource::<InProgressCurve>().wall.is_none());
            let history = world.resource::<History<()>>();
            assert!(!history.is_recording());
            assert_eq!(history.ledger().cursor, 0);
            assert_eq!(history.ledger().entries.len(), 1);
            assert!(matches!(
                &history.ledger().entries[0].edits[..],
                [EditType::Wall(_)]
            ));
            assert!(world.resource::<WallDrawingIssues>().0.is_empty());
            let events = world.resource::<Events<InputReplayCommand>>();
            let mut cursor = events.get_cursor();
            assert_eq!(
                cursor.read(events).cloned().collect::<Vec<_>>(),
                [InputReplayCommand::Sync { record: true }]
            );
            assert_eq!(
                world
                    .resource::<InputRecordReplayState>()
                    .stream
                    .sync_points()
                    .len(),
                0
            );
            ui_frame(&mut world, &mut schedule, 3, Vec3::new(0.4, 0., 0.1));
            assert!(world.resource::<History<()>>().is_recording());
            assert_eq!(world.resource::<History<()>>().ledger().entries.len(), 1);
            assert_eq!(
                world
                    .resource::<InputRecordReplayState>()
                    .stream
                    .sync_points()
                    .len(),
                1
            );
        }
    }
}
