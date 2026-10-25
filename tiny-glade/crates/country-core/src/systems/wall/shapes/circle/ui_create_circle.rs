//! B 14165bed0：圆形工具真实画布、预览、创建、边界、树遮挡及音频。
//! 交互来自 UiState 的原 wall_circle_canvas，不再用世界光标位移代理输入动作。
use super::super::rectangle::ui_create_rectangle::ShapesDragMouseMoved;
use super::super::{CreateCircleCmd, DontRecalculateShapePlacement, PreviewCircle, ShapePlacement};
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
use utils::geometry::Circle2d;

fn circle_display(circle: &Circle2d) -> String {
    format!(
        "Circle2d {{ center: {:?}, radius: {:?}, rotation: {:?} }}",
        circle.center, circle.radius, circle.rotation
    )
}

/// B 14165cd40/14165cf80：真实拥有闭包，创建可渲染及可卸载的地形贴合实体。
pub fn circle_signifier(circle: Circle2d, dashed: bool) -> SignifierFactory {
    SignifierFactory::from_closure(
        &format!("cursor_circle_{}{}", circle_display(&circle), dashed),
        Arc::new(move |meshes, shaders, commands| {
            let entity =
                SignifierEntityBuilder::new(SignifierMesh::Generated(Box::new(move || {
                    circle_signifier_curve(&circle).points
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
                crate::components::DashedLineLength(circle.perimeter()),
                crate::components::MeshFollowTerrain,
            ));
            vec![entity]
        }),
    )
}
/// B 14165b330：完整圆周以原0.2步长生成，实际Y由MeshFollowTerrain负责。
pub fn circle_signifier_curve(circle: &Circle2d) -> utils::curve::Curve3XZ {
    circle.as_curve3(0.2, 0.0)
}
fn shape(preview: &PreviewCircle) -> Circle2d {
    Circle2d {
        center: preview.center,
        radius: preview.radius,
        rotation: preview.rotation,
    }
}
fn size_value(circle: Circle2d) -> f32 {
    utils::scalar::inverse_lerp(
        f32::from_bits(0x4022dc35),
        f32::from_bits(0x43490fdb),
        circle.area(),
    )
}
fn original_radius(preview: &PreviewCircle, plane_y: f32, ray: &CameraCursorRay) -> f32 {
    let point = ray.intersect_with_xz_plane(plane_y);
    let dx = preview.center.x - point.x;
    let dz = preview.center.y - point.z;
    let distance = (dz * dz + dx * dx).sqrt();
    // 原MAXSS/MINSS保留NaN第二操作数；不能照错误反编译改成有序比较兜底。
    distance.clamp(f32::from_bits(0x3f666667), 8.0)
}

/// B原12组参数；拆分存储资源使用安全借用，不删减任何业务输出。
#[allow(clippy::too_many_arguments)]
pub fn ui_create_circle(
    mode: Res<AppMode>,
    actions: Res<ActionSetStates>,
    cursor: Res<CursorPositionSS>,
    ray: Res<CameraCursorRay>,
    time: Res<GameTime>,
    (placement, mut preview): (Res<ShapePlacement>, ResMut<PreviewCircle>),
    (ui, mut stream): (Res<UiState>, ResMut<UiSignifierStream>),
    mut drag: ResMut<ShapesDragMouseMoved>,
    mut moved: EventReader<AudioCursorMoved>,
    (mut construction, mut trees, mut audio, mut create, mut capture, mut freeze): (
        EventWriter<ConstructTimberframeCmd>,
        EventWriter<HideTreesinAreaCmd>,
        EventWriter<PlayTimberframeSound>,
        EventWriter<CreateCircleCmd>,
        EventWriter<OsCursorCaptureCmd>,
        EventWriter<DontRecalculateShapePlacement>,
    ),
    (border, mut display, mut feedback): (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
    ),
    mut maximum: EventWriter<MaxDemoElevationReached>,
) {
    if !matches!(*mode, AppMode::Wall(WallShape::Circle)) {
        preview.last_audio_position = None;
        return;
    }
    let context = ui.add(
        "wall_circle_canvas",
        WidgetParams::default()
            .geometry(WidgetGeometry::TerrainCanvas {
                distance: f32::MAX,
                layer: 0,
            })
            .occlude_by_world(false),
    );
    if placement.elevation() >= 60.0 {
        maximum.write(MaxDemoElevationReached(ShapeParameters::from_shape(
            Shape::Circle(shape(&preview)),
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
    let mut radius_delta = 0.0;
    match phase {
        2 | 3 => {
            let point = placement.position();
            preview.center = Vec2::new(point.x, point.z);
            preview.placement_y = point.y;
            stream.write_factory(&circle_signifier(shape(&preview), true));
        }
        4 => {
            if drag.has_moved {
                let radius = original_radius(&preview, placement.position().y, &ray);
                radius_delta = (radius - preview.radius).abs();
                preview.radius = radius;
            }
            stream.write_factory(&circle_signifier(shape(&preview), !drag.has_moved));
        }
        _ => {
            if !drag.has_moved {
                let circle = shape(&preview);
                if !border.is_shape_inside(BorderShape::Circle(circle)) {
                    feedback.write(FeedbackInputOutOfBorder);
                    permitted = false;
                } else {
                    create.write(CreateCircleCmd {
                        center: circle.center,
                        radius: circle.radius,
                        rotation: circle.rotation,
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
                        value: size_value(circle),
                    });
                }
            }
        }
    }
    let circle = shape(&preview);
    if !border.is_shape_inside(BorderShape::Circle(circle)) {
        display.write(DisplayOutOfBorder);
    }
    if permitted {
        let expanded = Circle2d {
            radius: circle.radius + 0.28,
            ..circle
        };
        construction.write(ConstructTimberframeCmd::from_shape(
            ShapeParameters::from_shape(Shape::Circle(expanded)),
            if placement.observed()[0x2c] == 2 {
                0
            } else {
                placement.observed()[0x2c]
            },
        ));
        trees.write(HideTreesinAreaCmd {
            area: TreeHideArea::Circle(expanded),
            height_limit: placement.elevation(),
            expansion: 4.0,
            finished: false,
        });
    }
    let cursor_moved = moved.read().count() != 0;
    let value = size_value(circle);
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
        4 if cursor_moved => {
            audio.write(PlayTimberframeSound {
                kind: 0,
                flag: 0,
                speed_source: radius_delta,
                position: placement.position(),
                value,
            });
        }
        _ => {}
    }
}

#[cfg(test)]
mod signifier_tests {
    use super::*;

    use super::super::super::rectangle::ui_create_rectangle::tests::{
        interact, validation_world, wake_preview_factories,
    };
    #[test]
    fn original_canvas_radius_stamp_border_and_factory_chain() {
        let mut world = validation_world(AppMode::Wall(WallShape::Circle));
        world.resource_mut::<CameraCursorRay>().origin = Vec3::new(10., 10., 4.);
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_create_circle);
        schedule.run(&mut world);
        assert_eq!(
            (
                world.resource::<PreviewCircle>().center,
                world.resource::<PreviewCircle>().radius,
                world.resource::<PreviewCircle>().placement_y
            ),
            (Vec2::new(3., 4.), 3.0, 2.0)
        );
        interact(&mut world, "wall_circle_canvas", 3);
        schedule.run(&mut world);
        world.resource_mut::<ActionSetStates>().analog[0] = Vec2::new(3., 4.);
        world
            .resource_mut::<Events<AudioCursorMoved>>()
            .send(AudioCursorMoved);
        interact(&mut world, "wall_circle_canvas", 4);
        schedule.run(&mut world);
        assert_eq!(world.resource::<ShapesDragMouseMoved>().distance, 5.0);
        assert_eq!(world.resource::<PreviewCircle>().radius, 7.0);
        interact(&mut world, "wall_circle_canvas", 0);
        schedule.run(&mut world);
        assert!(world.resource::<Events<CreateCircleCmd>>().is_empty());
        interact(&mut world, "wall_circle_canvas", 3);
        schedule.run(&mut world);
        interact(&mut world, "wall_circle_canvas", 1);
        schedule.run(&mut world);
        let create = world
            .resource_mut::<Events<CreateCircleCmd>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(create.len(), 1);
        let request = create[0];
        assert_eq!(
            (request.center, request.radius, request.rotation),
            (Vec2::new(3., 4.), 7.0, 0.0)
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
                .any(|event| event.kind == 0 && event.speed_source == 4.0)
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
        let TreeHideArea::Circle(area) = trees.area else {
            panic!("树请求必须保留圆")
        };
        assert_eq!(area.radius, 7.28);
        assert_eq!(
            (trees.height_limit, trees.expansion, trees.finished),
            (5.0, 4.0, false)
        );
        wake_preview_factories(&mut world);
        let mut raw = *world.resource::<ShapePlacement>().observed();
        raw[0x30..0x34].copy_from_slice(&1000f32.to_le_bytes());
        world.insert_resource(ShapePlacement::from_observed(raw));
        interact(&mut world, "wall_circle_canvas", 3);
        schedule.run(&mut world);
        interact(&mut world, "wall_circle_canvas", 0);
        schedule.run(&mut world);
        assert!(world.resource::<Events<CreateCircleCmd>>().is_empty());
        assert_eq!(
            world.resource::<Events<FeedbackInputOutOfBorder>>().len(),
            1
        );
        assert!(!world.resource::<Events<DisplayOutOfBorder>>().is_empty());
    }
    #[test]
    fn original_geometry_instruction_fixture() {
        type Row = ([u32; 2], u32, [u32; 3], [u32; 3], u32);
        let rows: Vec<Row> = ron::from_str(include_str!(
            "../../../../../../../tests/fixtures/native/toolbar-create-circle-geometry.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 240);
        for (center, plane, origin, direction, expected) in rows {
            let preview = PreviewCircle {
                center: Vec2::from_array(center.map(f32::from_bits)),
                ..Default::default()
            };
            let actual = original_radius(
                &preview,
                f32::from_bits(plane),
                &CameraCursorRay {
                    origin: Vec3::from_array(origin.map(f32::from_bits)),
                    dir: Vec3::from_array(direction.map(f32::from_bits)),
                },
            );
            if f32::from_bits(expected).is_nan() {
                assert!(actual.is_nan());
            } else {
                assert_eq!(
                    actual.to_bits(),
                    expected,
                    "{center:?}/{plane}/{origin:?}/{direction:?}"
                );
            }
        }
    }
    #[test]
    fn circle_signifier_name_uses_native_display_format() {
        let circle = Circle2d {
            center: Vec2::new(1.0, 2.0),
            radius: 3.0,
            rotation: 0.5,
        };
        let factory = circle_signifier(circle, false);
        // Display impl 0x14165bd40 = debug_struct_field3_finish (center/radius/rotation).
        assert_eq!(
            factory.payload(),
            b"cursor_circle_Circle2d { center: Vec2(1.0, 2.0), radius: 3.0, rotation: 0.5 }false"
        );
    }

    #[test]
    fn circle_signifier_curve_is_flat_at_as_curve3_step() {
        let circle = Circle2d {
            center: Vec2::new(1.0, 2.0),
            radius: 3.0,
            rotation: 0.0,
        };
        let curve = circle_signifier_curve(&circle);
        assert!(!curve.points.is_empty());
        assert!(curve.points.iter().all(|point| point.y == 0.0));
    }
}
