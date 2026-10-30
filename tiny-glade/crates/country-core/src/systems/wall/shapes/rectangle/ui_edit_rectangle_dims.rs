//! B140ad6250/140ad6e00：原UI producer→DO consumer的矩形尺寸拖拽链。
use super::{
    super::{EditRectangleCmd, FinishEditRectangleCmd},
    WipRectangle,
    ui_modes::edit_dims::UiRectangleDimsEdit,
};
use crate::{
    resources::{
        glade_border::{BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        mouse_terrain_raycast::CameraCursorRay,
        tree_storage::{HideTreesinAreaCmd, TreeHideArea},
        walls::{
            WallId,
            decorator_archivist::LiveDecoratorArchivist,
            public_walls::{PublicWalls, WallEntities},
            wall_trigger_volume::WallTriggerVolume,
        },
    },
    systems::{
        camera_rig::main_camera::MainCameraMoved,
        mode_manager::{AppMode, EditSubject},
        ui::{
            resources::{
                slider3d::{Slider3d, Slider3dStart},
                ui_state::{DragState, UiState},
            },
            signifiers::UiSignifierStream,
        },
        ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter,
        wall::valid_enclosure::{Shape, ShapeParameters},
    },
};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use utils::{curve::Curve2, geometry::Rectangle2d};

/// 原80字节活动记录：Curve2、WidgetId和世界原点；producer每帧替换该拥有型状态。
#[derive(Resource, Clone, Debug, Default)]
pub struct UiEditRectangleActiveWidgetResource(
    pub  Option<(
        Curve2,
        crate::systems::ui::resources::ui_state::WidgetId,
        Vec3,
    )>,
);

/// 原104字节业务载荷：完整intersection、墙、矩形、边及拖拽phase。
#[derive(Event, Clone, Copy, Debug)]
pub struct DoUiEditRectangleDims {
    pub intersection: [u8; 56],
    pub wall: WallId,
    pub rectangle: Rectangle2d,
    pub edge: usize,
    pub phase: DragState,
}
impl DoUiEditRectangleDims {
    pub fn point(&self) -> Vec3 {
        let f = |at| f32::from_le_bytes(self.intersection[at..at + 4].try_into().unwrap());
        Vec3::new(f(36), f(40), f(44))
    }
}

/// 原RayBoxIntersection的有序hit判定，同时检查出口，不能只检查near有限。
pub(super) fn candidate_hit(rectangle: Rectangle2d, ray: &CameraCursorRay) -> bool {
    let bounds = rectangle.aabb();
    let first =
        (bounds.min - Vec2::new(ray.origin.x, ray.origin.z)) / Vec2::new(ray.dir.x, ray.dir.z);
    let last =
        (bounds.max - Vec2::new(ray.origin.x, ray.origin.z)) / Vec2::new(ray.dir.x, ray.dir.z);
    let near = first.x.min(last.x).max(first.y.min(last.y));
    let far = first.x.max(last.x).min(first.y.max(last.y));
    near <= far && 0.0 <= far
}

/// 原11个SystemParam；producer只注册真实widget并运输当前交互，不直接修改墙。
pub fn ui_edit_rectangle_dims(
    mode: Res<AppMode>,
    mut stream: ResMut<UiSignifierStream>,
    ui: Res<UiState>,
    ray: Res<CameraCursorRay>,
    walls: Res<PublicWalls>,
    entities: Res<WallEntities>,
    triggers: Query<&WallTriggerVolume>,
    mut moved: EventReader<MainCameraMoved>,
    mut active: ResMut<UiEditRectangleActiveWidgetResource>,
    mut events: EventWriter<DoUiEditRectangleDims>,
    mut commands: Commands,
) {
    active.0 = None;
    if mode.native_tag() > 5 {
        return;
    }
    let camera_moved = moved.read().last().is_some();
    for (wall, state) in walls.all() {
        if state.shape().tag() != 1 {
            continue;
        }
        let selected = match *mode {
            AppMode::Editing(EditSubject::None) => false,
            AppMode::Editing(EditSubject::Wall(id) | EditSubject::Roof(id)) if id == wall.0 => true,
            _ => continue,
        };
        let rectangle = ShapeParameters::from_snapshot(state.shape()).as_rectangle();
        for edge in 0..4 {
            let mut widget = UiRectangleDimsEdit::new(wall, rectangle, edge, selected);
            let context = ui.widget_context(widget.widget_id());
            let dragged = context.dragged();
            if !selected && !candidate_hit(rectangle, &ray) && !context.is_active() {
                continue;
            }
            let trigger = if selected {
                None
            } else {
                let entity = entities
                    .wall_entity(wall)
                    .expect("矩形墙缺少WallEntities关联");
                Some(
                    triggers
                        .get(entity)
                        .expect("矩形墙实体缺少WallTriggerVolume"),
                )
            };
            let context = widget.widget(&ui, state, trigger, camera_moved);
            if camera_moved && !selected && dragged == 2 {
                continue;
            }
            widget.signifier(&context, &mut stream, &mut active);
            // 原1412dce20为deferred HighlightWallCmd，不是模式切换。
            if !selected && context.hovered() != 0 {
                commands.queue(move |world: &mut World| {
                    world.send_event(crate::systems::wall::wall_highlight::HighlightWallCmd {
                        wall,
                        ty: 2,
                    });
                });
            }
            let intersection = context.intersecting();
            if u32::from_le_bytes(intersection[..4].try_into().unwrap()) != 2 {
                events.write(DoUiEditRectangleDims {
                    intersection,
                    wall,
                    rectangle,
                    edge,
                    phase: DragState(context.dragged()),
                });
            }
        }
    }
}

/// 原12组参数；Started归档/建锚点，Held编辑，Finished发送unit完成信号。
#[allow(clippy::too_many_arguments)]
pub fn do_ui_edit_rectangle_dims(
    walls: Res<PublicWalls>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    mut wip: ResMut<WipRectangle>,
    mut edit: EventWriter<EditRectangleCmd>,
    mut finish: EventWriter<FinishEditRectangleCmd>,
    mut slider: Slider3d,
    border: Res<GladeBorder>,
    mut display: EventWriter<DisplayOutOfBorder>,
    mut feedback: EventWriter<FeedbackInputOutOfBorder>,
    mut trees: EventWriter<HideTreesinAreaCmd>,
    mut pins: EventWriter<ShowPinnedDecoClutter>,
    mut events: EventReader<DoUiEditRectangleDims>,
) {
    let Some(event) = events.read().last() else {
        return;
    };
    if u32::from_le_bytes(event.intersection[..4].try_into().unwrap()) == 2 {
        return;
    }
    let wall = walls.get(event.wall).expect("矩形尺寸编辑墙尚未发布");
    let point = event.point();
    let rectangle = event.rectangle;
    let edge = event.edge;
    let normal = rectangle.get_edge_normal(edge);
    if event.phase.is_started() {
        archivist.decorator_cull_allow_any();
        let (a, b) = rectangle.get_edge_pts(edge);
        let d = b - a;
        let denominator = (d.y * d.y + d.x * d.x).max(1e-20);
        let t = ((point.z - a.y) * d.y + (point.x - a.x) * d.x) / denominator;
        let foot = Vec2::new((1.0 - t) * a.x + t * b.x, (1.0 - t) * a.y + t * b.y);
        let distance = normal.y * (point.z - foot.y) + normal.x * (point.x - foot.x);
        wip.begin_dimensions(event.wall, edge, rectangle, point.y, normal * distance);
    } else if event.phase.0 == 4 {
        pins.write(ShowPinnedDecoClutter::rectangle(
            rectangle,
            wall.authority.arch_height(),
            wall.max_y_gable(),
            Some(event.wall),
        ));
        trees.write(HideTreesinAreaCmd {
            area: TreeHideArea::Rectangle(rectangle),
            height_limit: wall.authority.arch_height(),
            expansion: 1.5,
            finished: false,
        });
    } else if event.phase.is_finished() {
        finish.write(FinishEditRectangleCmd);
        trees.write(HideTreesinAreaCmd {
            area: TreeHideArea::Rectangle(rectangle),
            height_limit: wall.authority.arch_height(),
            expansion: 1.5,
            finished: true,
        });
    }
    if wip.tag() != 2 {
        return;
    }
    let dimension = usize::from(edge == 1 || edge == 3);
    let Some(change) = slider.update(
        event.phase,
        || Slider3dStart {
            origin: point,
            direction: Vec3::new(normal.x, 0.0, normal.y),
            value: rectangle.size[dimension],
        },
        [2.25, 15.0],
    ) else {
        return;
    };
    let delta = change.value - rectangle.size[dimension];
    let mut after = rectangle;
    after.expand_along_edge(edge, delta, 2.25, 15.0);
    if !border.is_shape_inside(BorderShape::Rectangle(after)) {
        display.write(DisplayOutOfBorder);
        feedback.write(FeedbackInputOutOfBorder);
        after = border
            .search_lerped_shape_that_fits_into_glade(
                ShapeParameters::from_shape(Shape::Rectangle(wip.current())),
                ShapeParameters::from_shape(Shape::Rectangle(after)),
            )
            .as_rectangle();
    }
    let difference = after.center - wip.current().center;
    if (difference.y * difference.y + difference.x * difference.x).sqrt() > 0.0 {
        let old = wip.before();
        let y = wip.scalar(4);
        wip.set_current(after);
        edit.write(EditRectangleCmd {
            wall: event.wall,
            center: after.center,
            dimensions: after.size,
            orientation: after.axis,
            position: Vec3::new(old.center.x, y, old.center.y),
            movement: delta.abs(),
            rotating: false,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resources::{
            critters::sheep::CmdCheckForSheepInsideBuilding,
            history::History,
            onboarding::NotifyHintSystemShapeEditWasPerformed,
            walls::{
                InnerWalls,
                decorator_storage::DecoratorStorage,
                public_walls::{WallPublicationDirty, WallSpatialHash},
                publication_source::TerrainHeightGrid512,
            },
        },
        systems::{
            audio::{PlayBuildingAudio, RequestWallMaterialAudio},
            camera_rig::MainCamera,
            ui::resources::ui_state::UiInteraction,
            wall::{
                shapes::rectangle::{
                    edit_rectangle::edit_rectangle,
                    finish_edit_rectangle::{OnRectangleEdited, finish_edit_rectangle},
                },
                wall_state::path_preparation::PathRasters512,
            },
        },
    };
    use std::sync::Arc;

    fn publish(world: &mut World, terrain: &TerrainHeightGrid512, paths: &PathRasters512) {
        world.resource_scope(|world, mut public: Mut<PublicWalls>| {
            public.consume_with_paths(
                &mut world.resource_mut::<InnerWalls>(),
                terrain,
                Some(paths),
                &WallPublicationDirty::default(),
                &WallSpatialHash::default(),
            );
            let ids: Vec<_> = public.wall_ids().collect();
            for wall in ids {
                public
                    .get_mut(wall)
                    .unwrap()
                    .update_gable_heights(None)
                    .unwrap();
            }
        });
    }

    fn replay_recorded_rectangle_both_directions(
        world: &mut World,
        wall: WallId,
        terrain: &TerrainHeightGrid512,
        paths: &PathRasters512,
    ) {
        use crate::resources::history::{EditType, wall_edit::WallHistoryEdit};
        use crate::resources::walls::publication_fields::WallId as ReplayWallId;
        use crate::systems::history::wall_replay_interpreter::WallReplayCommand;
        let edit = match &world.resource::<History>().pending().unwrap().edits[0] {
            EditType::Wall(compound) => compound.groups[0][0].clone(),
            other => panic!("矩形应产生Wall历史，实际{other:?}"),
        };
        let WallHistoryEdit::MorphPremadeShape(parameters) = &edit else {
            panic!("矩形应记录原MorphPremadeShape");
        };
        let mut sources = std::collections::HashMap::from([(
            ReplayWallId(wall.0),
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .clone()
                .into_snapshot()
                .0,
        )]);
        for undo in [true, false] {
            let mut command = WallReplayCommand {
                edit: edit.clone(),
                timed: false,
                duration: 0.0,
                elapsed: 0.0,
                undo,
                finished: false,
            };
            let result = command.advance(&mut sources, 0.25, terrain).unwrap();
            assert!(!result.updated_source_notifications.is_empty());
            let restored = &sources[&ReplayWallId(wall.0)].source;
            world
                .resource_mut::<InnerWalls>()
                .update_wall_state_curve(wall, restored.curve.clone());
            world
                .resource_mut::<InnerWalls>()
                .update_wall_state_enclosure(wall, restored.shape.clone());
            publish(world, terrain, paths);
            let expected = if undo {
                parameters.before
            } else {
                parameters.after
            };
            assert_eq!(
                world
                    .resource::<PublicWalls>()
                    .get(wall)
                    .unwrap()
                    .shape()
                    .bytes(),
                expected.bytes()
            );
        }
    }

    fn validation_world() -> (
        World,
        WallId,
        Rectangle2d,
        TerrainHeightGrid512,
        PathRasters512,
    ) {
        let rectangle = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(10.0, 4.0));
        let terrain = TerrainHeightGrid512 {
            encoded: Arc::from(vec![0; 512 * 512]),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        };
        let paths = PathRasters512::from_encoded(
            std::array::from_fn(|_| Arc::from(vec![0; 512 * 512])),
            false,
        )
        .unwrap();
        let mut world = World::new();
        world.init_resource::<InnerWalls>();
        world.init_resource::<PublicWalls>();
        world.init_resource::<WallEntities>();
        world.init_resource::<UiEditRectangleActiveWidgetResource>();
        world.init_resource::<WipRectangle>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(crate::resources::mouse_terrain_raycast::CursorPositionSS(
            Vec2::new(50., 50.),
        ));
        world.insert_resource(crate::resources::window::WindowSize::new(100, 100));
        let mut camera = MainCamera::default();
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &glam::Mat4::IDENTITY);
        world.insert_resource(camera);
        world.insert_resource(CameraCursorRay {
            origin: Vec3::new(0., 10., 0.),
            dir: Vec3::NEG_Y,
        });
        world.insert_resource(GladeBorder(BorderShape::Rectangle(
            Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(130.)),
        )));
        let mut time = bevy_time::Time::<()>::default();
        time.advance_by(std::time::Duration::from_secs_f32(0.25));
        world.insert_resource(time);
        let mut history: History = History::empty_glade();
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        macro_rules! event {($($ty:ty),*)=>{$(world.init_resource::<Events<$ty>>();)*};}
        event!(
            DoUiEditRectangleDims,
            EditRectangleCmd,
            FinishEditRectangleCmd,
            OnRectangleEdited,
            DisplayOutOfBorder,
            FeedbackInputOutOfBorder,
            HideTreesinAreaCmd,
            ShowPinnedDecoClutter,
            MainCameraMoved,
            RequestWallMaterialAudio,
            PlayBuildingAudio,
            NotifyHintSystemShapeEditWasPerformed,
            CmdCheckForSheepInsideBuilding
        );
        // 使用真实创建consumer建立权威墙/屋顶请求/创建历史，再跨帧进入编辑。
        use crate::resources::color_id::{
            HalftimberColorIds, SetGateColorIdCmd, SetHalftimberColorIdCmd, SetWallColorIdCmd,
            WallColorIds,
        };
        use crate::systems::{
            history::recording::HistoryRecordingFailed, mode_manager::RequestToolbarCategoryChange,
        };
        use bevy_ecs::system::RunSystemOnce;
        world.init_resource::<WallColorIds>();
        world.init_resource::<HalftimberColorIds>();
        world
            .init_resource::<crate::resources::glade_color_customization::GladeColorCustomization>(
            );
        event!(
            super::super::super::CreateRectangleCmd,
            crate::resources::roofs::commands::CreateRoofCmd,
            SetWallColorIdCmd,
            SetHalftimberColorIdCmd,
            SetGateColorIdCmd,
            RequestToolbarCategoryChange,
            HistoryRecordingFailed
        );
        world
            .resource_mut::<Events<super::super::super::CreateRectangleCmd>>()
            .send(super::super::super::CreateRectangleCmd {
                center: rectangle.center,
                dimensions: rectangle.size,
                orientation: rectangle.axis,
                previous_wall: None,
                height: 3.0,
                elevation: 0.0,
            });
        world
            .run_system_once(super::super::create_rectangle::create_rectangle)
            .unwrap();
        assert_eq!(
            world
                .resource::<Events<crate::resources::roofs::commands::CreateRoofCmd>>()
                .len(),
            1
        );
        assert!(
            world
                .resource::<Events<HistoryRecordingFailed>>()
                .is_empty()
        );
        let wall = world.resource::<InnerWalls>().wall_ids().next().unwrap();
        assert_eq!(
            world
                .resource::<Events<CmdCheckForSheepInsideBuilding>>()
                .len(),
            1
        );
        world
            .resource_mut::<Events<CmdCheckForSheepInsideBuilding>>()
            .clear();
        world.resource_mut::<Events<HideTreesinAreaCmd>>().clear();
        world
            .resource_mut::<History>()
            .finish_input_recording()
            .unwrap();
        world
            .resource_mut::<History>()
            .start_input_recording(1)
            .unwrap();
        publish(&mut world, &terrain, &paths);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        (world, wall, rectangle, terrain, paths)
    }
    #[test]
    fn actual_finish_without_geometry_change_restores_archive_and_preserves_inactive_bytes() {
        let (mut world, wall, rectangle, _, _) = validation_world();
        let mut wip = WipRectangle::default();
        wip.begin_dimensions(wall, 0, rectangle, 1.0, Vec2::new(0.3, -0.4));
        let before = wip.0;
        world.insert_resource(wip);
        world.resource_mut::<LiveDecoratorArchivist>().special1 = Some(Default::default());
        world
            .resource_mut::<LiveDecoratorArchivist>()
            .decorator_cull_allow_any();
        world
            .resource_mut::<Events<FinishEditRectangleCmd>>()
            .send(FinishEditRectangleCmd);
        let mut schedule = Schedule::default();
        schedule.add_systems(finish_edit_rectangle::<crate::resources::history::MaybePendingSnapshotId>);
        schedule.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(world.resource::<WipRectangle>().tag(), 0);
        assert_eq!(&world.resource::<WipRectangle>().0[4..], &before[4..]);
        assert!(
            world
                .resource::<LiveDecoratorArchivist>()
                .special1
                .is_none()
        );
        assert!(
            world
                .resource::<History>()
                .pending()
                .unwrap()
                .edits
                .is_empty()
        );
        assert_eq!(world.resource::<Events<OnRectangleEdited>>().len(), 1);
        assert!(
            world
                .resource::<Events<NotifyHintSystemShapeEditWasPerformed>>()
                .is_empty()
        );
        assert!(
            world
                .resource::<Events<CmdCheckForSheepInsideBuilding>>()
                .is_empty()
        );
    }
    #[test]
    fn actual_world_ui_drag_audio_history_and_idle_frames() {
        let (mut world, wall, rectangle, terrain, paths) = validation_world();
        let widget = UiRectangleDimsEdit::new(wall, rectangle, 0, true).widget_id();
        let mut ui = UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: 3,
            widget,
        });
        ui.set_intersection_tag(0);
        ui.set_intersection_point(Vec3::new(-5., 1., 0.));
        world.insert_resource(ui);
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ui_edit_rectangle_dims,
                do_ui_edit_rectangle_dims,
                edit_rectangle,
                finish_edit_rectangle::<crate::resources::history::MaybePendingSnapshotId>,
            )
                .chain(),
        );
        schedule.run(&mut world);
        assert_eq!(world.resource::<WipRectangle>().tag(), 2);
        assert_eq!(
            world.resource::<WipRectangle>().before().size,
            Vec2::new(10., 4.)
        );
        assert!(
            world
                .resource::<History>()
                .pending()
                .unwrap()
                .edits
                .is_empty()
        );
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 4,
                widget,
            });
        world
            .resource_mut::<crate::resources::mouse_terrain_raycast::CursorPositionSS>()
            .0 = Vec2::new(75., 50.);
        schedule.run(&mut world);
        let edited = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert_ne!(edited.size, rectangle.size, "真实屏幕投影必须改变尺寸");
        assert_eq!(world.resource::<Events<PlayBuildingAudio>>().len(), 1);
        assert_eq!(
            world.resource::<Events<RequestWallMaterialAudio>>().len(),
            1
        );
        assert_eq!(world.resource::<Events<ShowPinnedDecoClutter>>().len(), 1);
        publish(&mut world, &terrain, &paths);
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 1,
                widget,
            });
        schedule.run(&mut world);
        assert_eq!(world.resource::<WipRectangle>().tag(), 0);
        assert_eq!(
            world.resource::<History>().pending().unwrap().edits.len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<CmdCheckForSheepInsideBuilding>>()
                .len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<NotifyHintSystemShapeEditWasPerformed>>()
                .len(),
            1
        );
        replay_recorded_rectangle_both_directions(&mut world, wall, &terrain, &paths);
        let save =
            serde_json::to_string(world.resource::<InnerWalls>().get(wall).unwrap()).unwrap();
        let restored: crate::resources::walls::WallState = serde_json::from_str(&save).unwrap();
        assert_eq!(
            restored.get_enclosure_params().unwrap().bytes(),
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap()
                .bytes()
        );
        world
            .resource_mut::<History>()
            .finish_input_recording()
            .unwrap();
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 0,
                native_phase: 2,
                widget,
            });
        schedule.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<Events<OnRectangleEdited>>().len(),
            3,
            "空闲帧不重放活动快照"
        );
        world.resource_mut::<InnerWalls>().remove(wall);
        publish(&mut world, &terrain, &paths);
        schedule.run(&mut world);
        assert!(world.resource::<PublicWalls>().get(wall).is_none());
    }
    #[test]
    fn actual_rotation_ui_has_started_held_finished_and_history() {
        use super::super::ui_edit_rectangle_orientation::{
            DoUiEditRectangleRotate, do_ui_edit_rectangle_rotate, ui_edit_rectangle_orientation,
        };
        use super::super::ui_modes::edit_orientation::UiRectangleOrientationEdit;
        let (mut world, wall, rectangle, terrain, paths) = validation_world();
        world.init_resource::<Events<DoUiEditRectangleRotate>>();
        world.insert_resource(crate::resources::time::GameTime {
            delta: 0.25,
            ..Default::default()
        });
        let widget = UiRectangleOrientationEdit::new(wall, rectangle, 0, true).widget_id();
        let mut ui = UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: 3,
            widget,
        });
        ui.set_intersection_tag(0);
        ui.set_intersection_point(Vec3::new(-5., 1., -2.));
        world.insert_resource(ui);
        world.insert_resource(CameraCursorRay {
            origin: Vec3::new(-5., 10., -2.),
            dir: Vec3::NEG_Y,
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ui_edit_rectangle_orientation,
                do_ui_edit_rectangle_rotate,
                edit_rectangle,
                finish_edit_rectangle::<crate::resources::history::MaybePendingSnapshotId>,
            )
                .chain(),
        );
        schedule.run(&mut world);
        assert_eq!(world.resource::<WipRectangle>().tag(), 1);
        assert_eq!(world.resource::<WipRectangle>().before().axis, Vec2::X);
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 4,
                widget,
            });
        world.insert_resource(CameraCursorRay {
            origin: Vec3::new(2., 10., -5.),
            dir: Vec3::NEG_Y,
        });
        schedule.run(&mut world);
        let after = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert_ne!(after.axis, rectangle.axis, "原旋转DO必须修改权威墙朝向");
        assert_eq!(after.size, rectangle.size);
        assert_eq!(world.resource::<Events<PlayBuildingAudio>>().len(), 1);
        assert_eq!(world.resource::<Events<ShowPinnedDecoClutter>>().len(), 1);
        publish(&mut world, &terrain, &paths);
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 1,
                widget,
            });
        schedule.run(&mut world);
        assert_eq!(world.resource::<WipRectangle>().tag(), 0);
        assert_eq!(
            world.resource::<History>().pending().unwrap().edits.len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<CmdCheckForSheepInsideBuilding>>()
                .len(),
            1
        );
        replay_recorded_rectangle_both_directions(&mut world, wall, &terrain, &paths);
        world
            .resource_mut::<History>()
            .finish_input_recording()
            .unwrap();
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 0,
                native_phase: 2,
                widget,
            });
        schedule.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<OnRectangleEdited>>().len(), 3);
        assert_eq!(world.resource::<Events<PlayBuildingAudio>>().len(), 1);
    }
}

#[cfg(test)]
mod native_widget_tests {
    use super::*;
    use super::super::finish_edit_rectangle::RectangleEditUiState;
    use crate::resources::walls::public_wall_state::PublicWallState;
    use crate::resources::walls::wall_trigger_volume::WallTriggerVolume;
    use crate::resources::walls::{WallState, publication_source::TerrainHeightGrid512};
    use crate::systems::ui::resources::widget_params::WidgetGeometry;
    use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    use utils::geometry::Rectangle2d;

    #[test]
    fn widget_id_matches_original_hash_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-dims-widget-hash-fixture.json"))).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let wall = u64::from_str_radix(case["wall"].as_str().unwrap(), 16).unwrap();
            let edge = case["edge"].as_u64().unwrap() as usize;
            let expected = u64::from_str_radix(case["widget_id"].as_str().unwrap(), 16).unwrap();
            assert_eq!(
                UiRectangleDimsEdit::new(crate::resources::walls::WallId(wall), Rectangle2d::new(Vec2::ZERO, Vec2::ONE, Vec2::X), edge, true).widget_id().0,
                expected
            );
        }
    }

    #[test]
    fn selected_and_unselected_geometry_uses_published_wall_and_trigger() {
        let rect = Rectangle2d::new(Vec2::ZERO, Vec2::splat(4.0), Vec2::X);
        let mut authority = WallState::new(
            crate::systems::wall::shapes::to_authority_curve(&rect.as_curve2()),
            4.0,
            0,
            0,
            false,
        );
        authority.snapshot.source.shape =
            ShapeParameters::from_shape(Shape::Rectangle(rect)).to_snapshot();
        authority.snapshot.source.curve_kind_tag = 1;
        let grid = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let mut wall = PublicWallState::collect(&authority, &grid).unwrap();
        let paths =
            crate::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
                std::array::from_fn(|_| std::sync::Arc::from(vec![0u8; 512 * 512])),
                true,
            )
            .unwrap();
        wall.prepare_paths(&paths).unwrap();
        let mut trigger = WallTriggerVolume::default();
        trigger.recompute(&wall).unwrap();
        use crate::resources::walls::{
            InnerWalls,
            public_walls::{PublicWalls, WallEntities, WallPublicationDirty, WallSpatialHash},
        };
        use crate::systems::mode_manager::{AppMode, EditSubject};
        let mut inner = InnerWalls::new();
        let id = inner.new_wall_from_curve(authority);
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &grid,
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        public.get_mut(id).unwrap().prepare_paths(&paths).unwrap();
        public.get_mut(id).unwrap().update_gable_heights(None).unwrap();
        let mut world = World::new();
        world.init_resource::<Events<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>>();
        world.insert_resource(inner);
        world.insert_resource(public);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(id.0)));
        world.init_resource::<WallEntities>();
        world.insert_resource(crate::systems::ui::resources::ui_state::UiState::at_startup());
        world
            .init_resource::<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>();
        world.insert_resource(crate::resources::mouse_terrain_raycast::CameraCursorRay {
            origin: glam::Vec3::new(0.0, 1.0, 10.0),
            dir: -glam::Vec3::Z,
        });
        world.init_resource::<UiEditRectangleActiveWidgetResource>();
        world.init_resource::<Events<DoUiEditRectangleDims>>();
        world.init_resource::<Events<crate::systems::camera_rig::main_camera::MainCameraMoved>>();
        let mut producer = Schedule::default();
        producer.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        producer.add_systems(ui_edit_rectangle_dims);
        producer.run(&mut world);
        let widget = UiRectangleDimsEdit::new(id, rect, 0, true).widget_id();
        let params = world
            .resource::<crate::systems::ui::resources::ui_state::UiState>()
            .widget(widget)
            .unwrap();
        let Some(WidgetGeometry::TriangleMesh(mesh)) = &params.geometry else {
            panic!("selected edge box")
        };
        let center = mesh
            .iter()
            .flat_map(|t| t.points)
            .fold(glam::Vec3::ZERO, |sum, p| sum + p)
            / (mesh.len() * 3) as f32;
        let inverse = glam::Mat4::from_translation(center + glam::Vec3::Z * 10.0);
        let mut camera = crate::systems::camera_rig::main_camera::MainCamera::new(1280.0 / 720.0);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &inverse.inverse());
        camera.write_matrix(0xc0, &inverse);
        world.insert_resource(camera);
        world.insert_resource(crate::resources::window::WindowSize::new(1280, 720));
        world.insert_resource(crate::resources::mouse_terrain_raycast::CursorPositionSS(
            Vec2::new(640.0, 360.0),
        ));
        world.insert_resource(crate::resources::read_mouse_depth_value::MouseDepthValue {
            view_depth: 100.0,
            ..Default::default()
        });
        world.init_resource::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
        world.init_resource::<crate::input::action_set::ActionSetStates>();
        world.init_resource::<Events<crate::resources::window::WindowFocusEvent>>();
        world.init_resource::<Events<crate::resources::events::EscapeMenuJustOpened>>();
        world.init_resource::<Events<crate::resources::events::ShutdownGameCmd>>();
        world.init_resource::<Events<crate::systems::brush_preview::OverwriteBrushPreview>>();
        world.init_resource::<Events<crate::resources::mouse_capture::OsCursorVisibilityCmd>>();
        world.init_resource::<Events<crate::systems::ui::resources::widget_params::PlayUiSound>>();
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let mut input = Schedule::default();
        input.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        input.add_systems(
            (
                crate::systems::ui::hover_intersection::hover_intersection,
                crate::systems::ui::ui_update::ui_update,
            )
                .chain_ignore_deferred(),
        );
        input.run(&mut world);
        producer.run(&mut world);
        let events: Vec<_> = world
            .resource_mut::<Events<DoUiEditRectangleDims>>()
            .drain()
            .collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].wall, id);
        assert_eq!(events[0].edge, 0);
        assert_eq!(events[0].phase.0, 2);
        // Feed the actual picker intersection into the transaction system.
        world.init_resource::<RectangleEditUiState>();
        world.init_resource::<crate::resources::walls::decorator_storage::DecoratorStorage>();
        world
            .init_resource::<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>(
            );
        world.insert_resource(crate::resources::glade_border::GladeBorder(
            crate::resources::glade_border::BorderShape::Rectangle(
                Rectangle2d::new_centered_axis_aligned(Vec2::splat(100.0)),
            ),
        ));
        world.init_resource::<Events<EditRectangleCmd>>();
        world.init_resource::<Events<super::super::super::FinishEditRectangleCmd>>();
        world.init_resource::<Events<crate::resources::tree_storage::HideTreesinAreaCmd>>();
        world.init_resource::<Events<crate::resources::glade_border::DisplayOutOfBorder>>();
        world.init_resource::<Events<crate::resources::glade_border::FeedbackInputOutOfBorder>>();
        let mut transaction = Schedule::default();
        transaction.add_systems(do_ui_edit_rectangle_dims);
        world.insert_resource(
            crate::resources::history::History::<()>::after_observed_future_clear(),
        );
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::JustPressed;
        input.run(&mut world);
        let mut history_start = Schedule::default();
        history_start
            .add_systems(crate::systems::history::recording::history_start_recording::<()>);
        history_start.run(&mut world);
        assert!(
            world
                .resource::<crate::resources::history::History<()>>()
                .is_recording()
        );
        producer.run(&mut world);
        transaction.run(&mut world);
        assert_eq!(
            u32::from_le_bytes(
                world.resource::<RectangleEditUiState>().0[..4]
                    .try_into()
                    .unwrap()
            ),
            2
        );
        assert_eq!(
            world.resource::<RectangleEditUiState>().before(),
            rect
        );
        world
            .resource_mut::<crate::resources::mouse_terrain_raycast::CursorPositionSS>()
            .0
            .x += 100.0;
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::Held;
        input.run(&mut world);
        producer.run(&mut world);
        transaction.run(&mut world);
        assert!(
            world
                .resource_mut::<Events<crate::resources::tree_storage::HideTreesinAreaCmd>>()
                .drain()
                .any(|event| !event.finished)
        );
        assert!(!world.resource::<Events<EditRectangleCmd>>().is_empty());
        world.init_resource::<Events<crate::systems::audio::RequestWallMaterialAudio>>();
        world.init_resource::<Events<crate::systems::audio::PlayBuildingAudio>>();
        world.init_resource::<bevy_time::Time>();
        let mut apply = Schedule::default();
        apply.add_systems(super::super::edit_rectangle::edit_rectangle);
        apply.run(&mut world);
        let changed = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(id)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert_ne!(changed, rect);
        world.resource_mut::<Events<EditRectangleCmd>>().clear();
        world.init_resource::<Events<super::super::finish_edit_rectangle::OnRectangleEdited>>();
        world.init_resource::<Events<crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>>();
        world.init_resource::<Events<crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding>>();
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::JustReleased;
        input.run(&mut world);
        producer.run(&mut world);
        transaction.run(&mut world);
        let mut finish_schedule = Schedule::default();
        finish_schedule
            .add_systems(super::super::finish_edit_rectangle::finish_edit_rectangle::<()>);
        finish_schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<crate::resources::history::History<()>>()
                .pending()
                .unwrap()
                .edits
                .len(),
            1
        );
        assert!(
            world
                .resource_mut::<Events<crate::resources::tree_storage::HideTreesinAreaCmd>>()
                .drain()
                .any(|event| event.finished)
        );
        world.init_resource::<crate::resources::stairs::stairs_state::StairsState>();
        world.init_resource::<crate::resources::stairs::stairs_removed_supports::StairsRemovedSupports>();
        world
            .init_resource::<Events<crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>>();
        world.init_resource::<Events<crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>>();
        world
            .init_resource::<Events<crate::systems::history::recording::FinishInputRecordingCmd>>();
        world.init_resource::<Events<crate::systems::backend::input::input_record_replay::InputReplayCommand>>();
        let mut history_finish = Schedule::default();
        history_finish
            .add_systems(crate::systems::history::recording::history_finish_recording_system::<()>);
        history_finish.run(&mut world);
        assert!(
            !world
                .resource::<crate::resources::history::History<()>>()
                .is_recording()
        );
        world.insert_resource(
            crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain(
                grid.clone(),
            ),
        );
        world.init_resource::<crate::systems::history::PendingHistoryCommands>();
        world.init_resource::<crate::resources::time::GameTime>();
        world
            .resource_mut::<crate::resources::time::GameTime>()
            .delta = 1.0;
        world.insert_resource(crate::resources::history::ReplayBuffer::new(1.0));
        world.init_resource::<Events<crate::systems::mode_manager::HistoryCommand>>();
        world.init_resource::<Events<crate::systems::history::history_process::DebugHistoryMsg>>();
        world.init_resource::<Events<crate::systems::history::history_process::NewReplayCommandEvent>>();
        world.init_resource::<Events<crate::systems::audio::PlayReplayAudio>>();
        world.init_resource::<Events<crate::systems::timelapse_replay::InterruptTimelapse>>();
        world.init_resource::<Events<crate::systems::history::HistoryReplayFailed>>();
        world
            .init_resource::<Events<crate::systems::history::replay_wall::WallHistoryReplayAudio>>(
            );
        world
            .init_resource::<Events<crate::systems::history::replay_wall::WallHistoryRestoreColor>>(
            );
        world.init_resource::<Events<crate::systems::history::ReplayBucketFinished>>();
        world
            .init_resource::<Events<crate::systems::wall::adjust_wall_height::AdjustWallHeightCmd>>(
            );
        world.init_resource::<crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>();
        let mut replay = Schedule::default();
        replay.add_systems(
            (
                crate::systems::history::history_process_system::<()>,
                crate::systems::history::replay_wall::replay_wall,
                crate::systems::history::pop_empty_replay_bucket,
            )
                .chain(),
        );
        world.send_event(crate::systems::mode_manager::HistoryCommand::UNDO);
        for _ in 0..400 {
            replay.run(&mut world);
        }
        let restored = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(id)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert!(
            (restored.center - rect.center).length() < 0.001
                && (restored.size - rect.size).length() < 0.001
        );
        world.send_event(crate::systems::mode_manager::HistoryCommand::REDO);
        for _ in 0..400 {
            replay.run(&mut world);
        }
        let redone = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(id)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert!(
            (redone.center - changed.center).length() < 0.001
                && (redone.size - changed.size).length() < 0.001
        );
        let before = world.resource::<RectangleEditUiState>().0;
        let mut missing = events[0].clone();
        missing.wall = crate::resources::walls::WallId(u64::MAX);
        missing.phase = crate::systems::ui::resources::ui_state::DragState(3);
        world.send_event(missing);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| transaction.run(&mut world)))
                .is_err()
        );
        assert_eq!(world.resource::<RectangleEditUiState>().0, before);
        assert_eq!(
            world
                .resource_mut::<Events<crate::resources::tree_storage::HideTreesinAreaCmd>>()
                .drain()
                .count(),
            0
        );
        assert_eq!(
            world
                .resource_mut::<Events<EditRectangleCmd>>()
                .drain()
                .count(),
            0
        );
        let entity = world.spawn(trigger).id();
        world.resource_mut::<WallEntities>().0.insert(id, entity);
        *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::None);
        world.insert_resource(crate::systems::ui::resources::ui_state::UiState::at_startup());
        world.init_resource::<Events<crate::systems::wall::wall_highlight::HighlightWallCmd>>();
        producer.run(&mut world);
        let params = world
            .resource::<crate::systems::ui::resources::ui_state::UiState>()
            .widget(widget)
            .unwrap();
        let Some(WidgetGeometry::TriangleMesh(mesh)) = &params.geometry else {
            panic!("ordinary edge mesh")
        };
        let center = mesh
            .iter()
            .flat_map(|t| t.points)
            .fold(glam::Vec3::ZERO, |sum, p| sum + p)
            / (mesh.len() * 3) as f32;
        let inverse = glam::Mat4::from_translation(center + glam::Vec3::Z * 10.0);
        let mut camera = crate::systems::camera_rig::main_camera::MainCamera::new(1280.0 / 720.0);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &inverse.inverse());
        camera.write_matrix(0xc0, &inverse);
        world.insert_resource(camera);
        input.run(&mut world);
        producer.run(&mut world);
        let highlighted: Vec<_> = world
            .resource_mut::<Events<crate::systems::wall::wall_highlight::HighlightWallCmd>>()
            .drain()
            .collect();
        assert_eq!(highlighted.len(), 1);
        assert_eq!((highlighted[0].wall, highlighted[0].ty), (id, 2));
        assert!(
            world
                .resource::<UiEditRectangleActiveWidgetResource>()
                .0
                .is_some()
        );
    }
}
