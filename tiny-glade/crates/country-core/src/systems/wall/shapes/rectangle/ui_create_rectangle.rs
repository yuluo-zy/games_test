//! B 14155d410：矩形工具的原画布状态机、相机/吸附朝向、尺寸、创建和反馈。
use super::super::{
    CreateRectangleCmd, DontRecalculateShapePlacement, PreviewRectangle, ShapePlacement,
};
use crate::{
    input::action_set::ActionSetStates,
    resources::{
        events::ConstructTimberframeCmd,
        glade_border::{
            BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
            MaxDemoElevationReached,
        },
        mouse_capture::OsCursorCaptureCmd,
        mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS},
        time::GameTime,
        tree_storage::{HideTreesinAreaCmd, TreeHideArea},
        window::AudioCursorMoved,
    },
    systems::{
        audio::PlayTimberframeSound,
        camera_rig::MainCamera,
        mode_manager::{AppMode, WallShape},
        ui::{
            resources::{
                ui_state::UiState,
                widget_params::{WidgetGeometry, WidgetParams},
            },
            signifiers::{
                SignifierEntityBuilder, SignifierFactory, UiSignifierStream,
                signifier_factory::SignifierMesh,
            },
        },
        wall::valid_enclosure::{Shape, ShapeParameters},
    },
};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use std::sync::Arc;
use utils::geometry::Rectangle2d;

/// B原共享16B拖拽资源：累计的是输入动作的屏幕位移，不是地形世界位移。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ShapesDragMouseMoved {
    pub distance: f32,
    pub start_position: Vec2,
    pub has_moved: bool,
}

fn rectangle_display(rectangle: &Rectangle2d) -> String {
    format!(
        "Rectangle2d {{ orientation: {:?}, width: {:?}, length: {:?}, center: {:?} }}",
        rectangle.axis, rectangle.size.x, rectangle.size.y, rectangle.center
    )
}

/// B 14155e460/14155e6b0：拥有原生成闭包与地形贴合实体，禁止保存栈上env地址。
pub fn rectangle_signifier(rectangle: Rectangle2d, dashed: bool) -> SignifierFactory {
    SignifierFactory::from_closure(
        &format!(
            "cursor_rectangle_{}{}",
            rectangle_display(&rectangle),
            dashed
        ),
        Arc::new(move |meshes, shaders, commands| {
            let entity =
                SignifierEntityBuilder::new(SignifierMesh::Generated(Box::new(move || {
                    rectangle_signifier_curve(&rectangle).points
                })))
                .custom_shader(if dashed {
                    "dashed_line"
                } else {
                    "vertex_color_shader"
                })
                .animation_duration(0.0)
                .build(meshes, shaders, commands);
            commands.entity(entity).insert((
                crate::render::pass_builder::draw_list::DrawTopology::Alt(3.0),
                crate::components::DashedLineLength(rectangle.perimeter()),
                crate::components::MeshFollowTerrain,
            ));
            vec![entity]
        }),
    )
}
/// B141559a10，原曲线每段0.5间距重采样，保留3D语义及原零Y。
pub fn rectangle_signifier_curve(rectangle: &Rectangle2d) -> utils::curve::Curve3 {
    rectangle
        .as_curve2()
        .crashy_resample_remap_per_segment(0.5, |_segment, _t, point| {
            utils::swizzlers::xvy(point, 0.0)
        })
}
fn shape(preview: &PreviewRectangle) -> Rectangle2d {
    Rectangle2d {
        axis: preview.orientation,
        center: preview.center,
        size: preview.dimensions,
    }
}
fn size_value(rectangle: Rectangle2d) -> f32 {
    utils::scalar::inverse_lerp(5.0625, 225.0, rectangle.area())
}
fn original_dimensions(preview: &PreviewRectangle, plane_y: f32, ray: &CameraCursorRay) -> Vec2 {
    let point = ray.intersect_with_xz_plane(plane_y);
    let dx = point.x - preview.center.x;
    let dz = point.z - preview.center.y;
    let width = (preview.orientation.y * dz + dx * preview.orientation.x).abs();
    let length = (preview.orientation.x * dz - dx * preview.orientation.y).abs();
    // 原maxNum/minNum会将NaN尺寸取下限，而不是用NaN画不存在的矩形。
    Vec2::new(
        (width + width).max(2.25).min(15.0),
        (length + length).max(2.25).min(15.0),
    )
}

/// B原16组参数；所有事件都送到原权威消费者，生产者不直接造墙。
#[allow(clippy::too_many_arguments)]
pub fn ui_create_rectangle(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    (placement, mut preview): (Res<ShapePlacement>, ResMut<PreviewRectangle>),
    camera: Res<MainCamera>,
    mut create: EventWriter<CreateRectangleCmd>,
    mut stream: ResMut<UiSignifierStream>,
    mut drag: ResMut<ShapesDragMouseMoved>,
    actions: Res<ActionSetStates>,
    mut capture: EventWriter<OsCursorCaptureCmd>,
    cursor: Res<CursorPositionSS>,
    mut freeze: EventWriter<DontRecalculateShapePlacement>,
    (mut moved, ray): (EventReader<AudioCursorMoved>, Res<CameraCursorRay>),
    (mut construction, mut trees, mut audio): (
        EventWriter<ConstructTimberframeCmd>,
        EventWriter<HideTreesinAreaCmd>,
        EventWriter<PlayTimberframeSound>,
    ),
    time: Res<GameTime>,
    (border, mut display, mut feedback): (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
    ),
    mut maximum: EventWriter<MaxDemoElevationReached>,
) {
    if !matches!(*mode, AppMode::Wall(WallShape::Rectangle)) {
        preview.last_audio_position = None;
        return;
    }
    let context = ui.add(
        "wall_rectangle_canvas",
        WidgetParams::default()
            .geometry(WidgetGeometry::TerrainCanvas {
                distance: f32::MAX,
                layer: 0,
            })
            .occlude_by_world(false),
    );
    if placement.elevation() >= 60.0 {
        maximum.write(MaxDemoElevationReached(ShapeParameters::from_shape(
            Shape::Rectangle(shape(&preview)),
        )));
    }
    let phase = context.dragged();
    if phase == 3 {
        drag.distance = 0.0;
        drag.start_position = cursor.0;
        drag.has_moved = false;
        freeze.write(DontRecalculateShapePlacement);
    }
    if phase == 4 {
        let delta = actions.analog[0];
        drag.distance += (delta.y * delta.y + delta.x * delta.x).sqrt();
        if drag.distance > 4.0 {
            drag.has_moved = true;
        }
        freeze.write(DontRecalculateShapePlacement);
    }
    if phase < 2 {
        capture.write(OsCursorCaptureCmd::SetPosition(drag.start_position));
    }
    let mut permitted = true;
    let mut dimensions_delta = Vec2::ZERO;
    match phase {
        2 | 3 => {
            preview.orientation = if placement.flat_roof().is_none() {
                let right = camera.camera_matrices().eye_right();
                let inv = 1.0 / (right.z * right.z + right.x * right.x).sqrt();
                Vec2::new(inv * right.x, inv * right.z)
            } else {
                let normal = placement.normal();
                let inv = 1.0 / (normal.x * normal.x + normal.y * normal.y).sqrt();
                Vec2::new(inv * (-normal.y), inv * normal.x)
            };
            let point = placement.position();
            preview.center = Vec2::new(point.x, point.z);
            preview.placement_y = point.y;
            stream.write_factory(&rectangle_signifier(shape(&preview), true));
        }
        4 => {
            if drag.has_moved {
                let dimensions = original_dimensions(&preview, placement.position().y, &ray);
                dimensions_delta = (dimensions - preview.dimensions).abs();
                preview.dimensions = dimensions;
            }
            stream.write_factory(&rectangle_signifier(shape(&preview), !drag.has_moved));
        }
        _ => {
            if !drag.has_moved {
                let rectangle = shape(&preview);
                if !border.is_shape_inside(BorderShape::Rectangle(rectangle)) {
                    feedback.write(FeedbackInputOutOfBorder);
                    permitted = false;
                } else {
                    create.write(CreateRectangleCmd {
                        center: rectangle.center,
                        dimensions: rectangle.size,
                        orientation: rectangle.axis,
                        previous_wall: placement
                            .wall_id()
                            .filter(|_| placement.flat_roof().is_some()),
                        height: placement.computed_height(),
                        elevation: placement.elevation(),
                    });
                    audio.write(PlayTimberframeSound {
                        kind: 7,
                        flag: 0,
                        speed_source: 0.0,
                        position: placement.position(),
                        value: size_value(rectangle),
                    });
                }
            }
        }
    }
    let rectangle = shape(&preview);
    if !border.is_shape_inside(BorderShape::Rectangle(rectangle)) {
        display.write(DisplayOutOfBorder);
    }
    if permitted {
        let expanded = Rectangle2d {
            size: rectangle.size + Vec2::splat(0.56),
            ..rectangle
        };
        let roof = placement.observed()[0x2c];
        construction.write(ConstructTimberframeCmd::from_shape(
            ShapeParameters::from_shape(Shape::Rectangle(expanded)),
            if roof == 2 { 0 } else { roof },
        ));
        trees.write(HideTreesinAreaCmd {
            area: TreeHideArea::Rectangle(expanded),
            height_limit: placement.elevation(),
            expansion: 4.0,
            finished: false,
        });
    }
    let cursor_moved = moved.read().count() != 0;
    let value = size_value(rectangle);
    match phase {
        2 if cursor_moved => {
            let position = Vec3::new(preview.center.x, placement.elevation(), preview.center.y);
            let previous = preview.last_audio_position.unwrap_or(position);
            let delta = position - previous;
            let speed =
                (delta.z * delta.z + (delta.y * delta.y + delta.x * delta.x)).sqrt() / time.delta;
            audio.write(PlayTimberframeSound {
                kind: 3,
                flag: u8::from(placement.elevation() == 0.0),
                speed_source: speed,
                position: placement.position(),
                value,
            });
            preview.last_audio_position = Some(position);
        }
        3 => {
            audio.write(PlayTimberframeSound {
                kind: 2,
                flag: 0,
                speed_source: 0.0,
                position: placement.position(),
                value,
            });
        }
        4 => {
            if dimensions_delta.x > 0.0 {
                audio.write(PlayTimberframeSound {
                    kind: 1,
                    flag: 0,
                    speed_source: dimensions_delta.x,
                    position: placement.position(),
                    value,
                });
            }
            if dimensions_delta.y > 0.0 {
                audio.write(PlayTimberframeSound {
                    kind: 0,
                    flag: 0,
                    speed_source: dimensions_delta.y,
                    position: placement.position(),
                    value,
                });
            }
        }
        _ => {}
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use crate::systems::ui::resources::ui_state::{UiInteraction, WidgetId};
    /// 只初始化原资源与真实事件，不提供代理动作或直接生成创建命令。
    pub(crate) fn validation_world(mode: AppMode) -> World {
        let mut world = World::new();
        super::super::super::register_shape_resources(&mut world);
        world.insert_resource(mode);
        world.init_resource::<ActionSetStates>();
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 0,
            native_phase: 2,
            widget: WidgetId(0),
        }));
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(CursorPositionSS(Vec2::new(12.0, 34.0)));
        world.insert_resource(CameraCursorRay {
            origin: Vec3::new(10., 10., 6.),
            dir: -Vec3::Y,
        });
        world.insert_resource(MainCamera::default());
        world.insert_resource(GameTime {
            delta: 0.016,
            ..Default::default()
        });
        world.insert_resource(GladeBorder(BorderShape::Circle(
            utils::geometry::Circle2d::new(Vec2::ZERO, 130.0),
        )));
        world.init_resource::<Events<AudioCursorMoved>>();
        world.init_resource::<Events<ConstructTimberframeCmd>>();
        world.init_resource::<Events<HideTreesinAreaCmd>>();
        world.init_resource::<Events<PlayTimberframeSound>>();
        world.init_resource::<Events<OsCursorCaptureCmd>>();
        world.init_resource::<Events<DontRecalculateShapePlacement>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<MaxDemoElevationReached>>();
        let mut raw = *ShapePlacement::default().observed();
        raw[8..12].copy_from_slice(&1u32.to_le_bytes());
        raw[0x18..0x1c].copy_from_slice(&0f32.to_le_bytes());
        raw[0x1c..0x20].copy_from_slice(&(-1f32).to_le_bytes());
        raw[0x20..0x28].copy_from_slice(&77u64.to_le_bytes());
        raw[0x2c] = 0;
        for (i, v) in [3f32, 2., 4.].into_iter().enumerate() {
            raw[0x30 + i * 4..0x34 + i * 4].copy_from_slice(&v.to_le_bytes());
        }
        raw[0x3c..0x40].copy_from_slice(&5f32.to_le_bytes());
        world.insert_resource(ShapePlacement::from_observed(raw));
        world
    }
    pub(crate) fn interact(world: &mut World, name: &str, phase: u8) {
        let mut ui = UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: phase,
            widget: WidgetId::from_name(name),
        });
        ui.set_intersection_tag(0);
        world.insert_resource(ui);
    }
    pub(crate) fn wake_preview_factories(world: &mut World) {
        use crate::{
            asset_libraries::{
                mesh_library::AssetMeshLibrary,
                shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary},
                transient_mesh::{TransientMesh, delete_dropped_transient_meshes},
                vao_library::AssetVAOLibrary,
            },
            render::{shader::RasterShaderProgram, test_device::TestDevice},
            systems::ui::signifiers::{
                animation::{SignifierAnimation, signifier_animation_t},
                signifier_create::{SignifierId, signifier_create},
                signifier_mark_for_delete::{despawn_marked, signfier_mark_for_delete},
            },
        };
        use bevy_ecs::system::RunSystemOnce;
        assert!(world.resource::<UiSignifierStream>().committed_len() > 0);
        let meshes = AssetMeshLibrary::new();
        let shaders = AssetShaderLibrary::new();
        for name in ["vertex_color_shader", "dashed_line"] {
            shaders.add_raster(
                RasterShaderProgram::new_draw(name, Default::default(), &TestDevice(1)),
                false,
            );
        }
        world.insert_resource(meshes);
        world.insert_resource(SharedAssetShaderLibrary(Arc::new(shaders)));
        let widget = world.resource::<UiState>().active_widget();
        let parameters = world.resource::<UiState>().widget(widget).unwrap();
        world.resource::<UiState>().clear_widgets();
        world.resource_mut::<UiState>().replace_interaction_full(
            UiInteraction {
                native_tag: 2,
                native_phase: 2,
                widget,
            },
            Some(parameters),
            [0; 56],
        );
        // 真正由生产consumer建立SignifierId和活动工厂所有权，而不是测试手动spawn。
        world.run_system_once(signifier_create).unwrap();
        let entities = world
            .query_filtered::<Entity, With<SignifierId>>()
            .iter(world)
            .collect::<Vec<_>>();
        assert!(!entities.is_empty());
        let mut generated = Vec::new();
        for &entity in &entities {
            assert!(
                world
                    .get::<crate::components::MeshFollowTerrain>(entity)
                    .is_some()
            );
            assert!(
                world
                    .get::<crate::components::DashedLineLength>(entity)
                    .unwrap()
                    .0
                    > 0.0
            );
            assert!(
                matches!(world.get::<crate::render::pass_builder::draw_list::DrawTopology>(entity),Some(crate::render::pass_builder::draw_list::DrawTopology::Alt(width)) if *width==3.0)
            );
            assert_eq!(
                world.get::<SignifierAnimation>(entity).unwrap().duration,
                0.0
            );
            generated.push(world.get::<TransientMesh>(entity).unwrap().mesh_handle());
        }
        let owner = Arc::downgrade(&world.resource::<UiState>().active_params().unwrap());
        world.insert_resource(UiState::at_startup());
        assert!(owner.upgrade().is_none());
        world.run_system_once(signfier_mark_for_delete).unwrap();
        world.run_system_once(signifier_animation_t).unwrap();
        world.run_system_once(despawn_marked).unwrap();
        assert!(
            entities
                .into_iter()
                .all(|entity| world.get_entity(entity).is_err())
        );
        delete_dropped_transient_meshes(
            &TestDevice(1),
            &mut world.resource_mut::<AssetMeshLibrary>(),
            &mut AssetVAOLibrary::new(),
        );
        assert!(
            generated
                .into_iter()
                .all(|handle| world.resource::<AssetMeshLibrary>().get(handle).is_none())
        );
    }
    #[test]
    fn original_canvas_resize_stamp_border_and_factory_chain() {
        let mut world = validation_world(AppMode::Wall(WallShape::Rectangle));
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_create_rectangle);
        // 原None/Started固定center；Held围绕center调完整尺寸，不把center移成两点中点。
        schedule.run(&mut world);
        let preview = *world.resource::<PreviewRectangle>();
        assert_eq!(
            (
                preview.center,
                preview.orientation,
                preview.dimensions,
                preview.placement_y
            ),
            (Vec2::new(3., 4.), Vec2::X, Vec2::splat(6.0), 2.0)
        );
        interact(&mut world, "wall_rectangle_canvas", 3);
        schedule.run(&mut world);
        world.resource_mut::<ActionSetStates>().analog[0] = Vec2::new(3., 4.);
        interact(&mut world, "wall_rectangle_canvas", 4);
        schedule.run(&mut world);
        assert_eq!(world.resource::<ShapesDragMouseMoved>().distance, 5.0);
        assert!(world.resource::<ShapesDragMouseMoved>().has_moved);
        assert_eq!(
            world.resource::<PreviewRectangle>().dimensions,
            Vec2::new(14., 4.)
        );
        interact(&mut world, "wall_rectangle_canvas", 0);
        schedule.run(&mut world);
        assert!(world.resource::<Events<CreateRectangleCmd>>().is_empty());
        interact(&mut world, "wall_rectangle_canvas", 3);
        schedule.run(&mut world);
        interact(&mut world, "wall_rectangle_canvas", 1);
        schedule.run(&mut world);
        let create = world
            .resource_mut::<Events<CreateRectangleCmd>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(create.len(), 1);
        let request = create[0];
        assert_eq!(
            (request.center, request.dimensions, request.orientation),
            (Vec2::new(3., 4.), Vec2::new(14., 4.), Vec2::X)
        );
        assert_eq!(
            (request.previous_wall, request.height, request.elevation),
            (Some(crate::resources::walls::WallId(77)), 0.0, 5.0)
        );
        let sounds = world
            .resource_mut::<Events<PlayTimberframeSound>>()
            .drain()
            .collect::<Vec<_>>();
        assert!(
            sounds
                .iter()
                .any(|event| event.kind == 1 && event.speed_source == 8.0)
        );
        assert!(
            sounds
                .iter()
                .any(|event| event.kind == 0 && event.speed_source == 2.0)
        );
        assert!(sounds.iter().any(|event| event.kind == 7));
        assert_eq!(
            world
                .resource::<Events<DontRecalculateShapePlacement>>()
                .len(),
            3
        );
        let trees = world
            .resource_mut::<Events<HideTreesinAreaCmd>>()
            .drain()
            .last()
            .unwrap();
        let TreeHideArea::Rectangle(area) = trees.area else {
            panic!("树请求必须保留矩形")
        };
        assert_eq!(area.size, Vec2::new(14.56, 4.56));
        assert_eq!(
            (trees.height_limit, trees.expansion, trees.finished),
            (5.0, 4.0, false)
        );
        wake_preview_factories(&mut world);
        // 在边界外点击时只反馈与显示，不发送创建命令。
        let mut raw = *world.resource::<ShapePlacement>().observed();
        raw[0x30..0x34].copy_from_slice(&1000f32.to_le_bytes());
        world.insert_resource(ShapePlacement::from_observed(raw));
        interact(&mut world, "wall_rectangle_canvas", 3);
        schedule.run(&mut world);
        interact(&mut world, "wall_rectangle_canvas", 0);
        schedule.run(&mut world);
        assert!(world.resource::<Events<CreateRectangleCmd>>().is_empty());
        assert_eq!(
            world.resource::<Events<FeedbackInputOutOfBorder>>().len(),
            1
        );
        assert!(!world.resource::<Events<DisplayOutOfBorder>>().is_empty());
    }

    #[test]
    fn original_geometry_instruction_fixture() {
        type Row = ([u32; 2], u32, [u32; 3], [u32; 3], [u32; 2], [u32; 2]);
        let rows: Vec<Row> = ron::from_str(include_str!(
            "../../../../../../../tests/fixtures/native/toolbar-create-rectangle-geometry.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 1440);
        for (center, plane, origin, direction, axis, expected) in rows {
            let preview = PreviewRectangle {
                center: Vec2::from_array(center.map(f32::from_bits)),
                orientation: Vec2::from_array(axis.map(f32::from_bits)),
                ..Default::default()
            };
            let actual = original_dimensions(
                &preview,
                f32::from_bits(plane),
                &CameraCursorRay {
                    origin: Vec3::from_array(origin.map(f32::from_bits)),
                    dir: Vec3::from_array(direction.map(f32::from_bits)),
                },
            );
            for (actual, expected) in actual.to_array().into_iter().zip(expected) {
                if f32::from_bits(expected).is_nan() {
                    assert!(actual.is_nan());
                } else {
                    assert_eq!(
                        actual.to_bits(),
                        expected,
                        "{center:?}/{plane}/{origin:?}/{direction:?}/{axis:?}"
                    );
                }
            }
        }
    }
    #[test]
    fn rectangle_signifier_name_uses_native_display_format() {
        let rectangle = Rectangle2d {
            axis: Vec2::X,
            center: Vec2::new(1.0, 2.0),
            size: Vec2::new(3.0, 4.0),
        };
        let factory = rectangle_signifier(rectangle, true);
        // Display impl 0x141559ce0 = debug_struct_field4_finish, field order
        // orientation/width/length/center; payload = "cursor_rectangle_" + it + bool.
        assert_eq!(
            factory.payload(),
            b"cursor_rectangle_Rectangle2d { orientation: Vec2(1.0, 0.0), width: 3.0, length: 4.0, center: Vec2(1.0, 2.0) }true"
        );
        assert_eq!(factory.payload().len(), factory.len());
    }

    #[test]
    fn rectangle_signifier_curve_resamples_to_flat_3d() {
        let rectangle = Rectangle2d {
            axis: Vec2::X,
            center: Vec2::ZERO,
            size: Vec2::new(4.0, 2.0),
        };
        let curve = rectangle_signifier_curve(&rectangle);
        // as_curve2 的 4 角 + 0.5 间距重采样；xvy 使所有 y 保持 0。
        assert!(curve.points.len() >= 5);
        assert!(curve.points.iter().all(|point| point.y == 0.0));
    }
}
