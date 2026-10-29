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
            speed: delta.abs(),
            rotation: false,
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
        schedule.add_systems(finish_edit_rectangle);
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
                finish_edit_rectangle,
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
                finish_edit_rectangle,
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
