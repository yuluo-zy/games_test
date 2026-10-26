//! B `ui_focus_mode_shape_height`：墙／屋顶聚焦时的高度与抬升控件。
//! 原140a623e0，PDB9215字节含析构funclet；两个工厂为140a647e0/140a66a30。
//! 原证的第二个类型化系统接口才属本函数，不混用同文件的高度编辑候选。
use bevy_ecs::prelude::*;
use glam::{Mat4, Quat, Vec2, Vec3};
use std::sync::Arc;

use crate::{
    resources::{
        events::{FinishAdjWallHeightMode, InitAdjWallHeightMode},
        time::GameTime,
        walls::{WallId, public_walls::PublicWalls},
        window::WindowSize,
    },
    systems::{
        camera_rig::MainCamera,
        mode_manager::{AppMode, EditSubject},
        ui::{
            resources::{
                ui_state::UiState,
                widget_params::{WidgetGeometry, WidgetParams},
            },
            signifiers::{
                signifier_create::UiSignifierStream,
                signifier_factory::{SignifierEntityBuilder, SignifierFactory},
            },
        },
        ui_raise_shape::{HandleRaiseShapeUiInteraction, MAX_ELEVATION},
        wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi,
    },
    utils::debug_viz::DebugLine,
};

/// 原Local为16字节Option<Vec3>；切换到非墙／屋顶模式时清空，换墙不强制跳变。
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct LerpedGizmoPosition(pub Option<Vec3>);

/// 原minNum展开：相等保留旧值，旧值NaN时取新值；不改成有限值过滤。
fn minimum(previous: f32, next: f32) -> f32 {
    if previous.is_nan() || next < previous {
        next
    } else {
        previous
    }
}

/// 原626c8..6296a：屏幕最右侧轮廓点、最低地面与水平相机偏移。
/// 投影读取宿主已维护的view矩阵，不能由inverse_view再次求逆引入舍入差异。
fn target_position(
    points: &[[f32; 3]],
    maximum_y: f32,
    width: u32,
    projection: Mat4,
    view: Mat4,
    eye_right: Vec3,
) -> (Vec3, Vec3) {
    let first = points.first().expect("聚焦墙必须拥有发布轮廓点");
    let mut selected = Vec2::new(first[0], first[2]);
    let mut rightmost = 0.0f32;
    let mut floor = 10000.0f32;
    for point in points {
        // 原先做view的仿射变换，再把xyz按w=1投影，保留各乘加次序。
        let camera = ((view.y_axis * point[1] + view.x_axis * point[0]) + view.z_axis * point[2])
            + view.w_axis;
        let clip = ((projection.y_axis * camera.y + projection.x_axis * camera.x)
            + projection.z_axis * camera.z)
            + projection.w_axis;
        let x = ((clip.x / clip.w) * 0.5 + 0.5) * width as f32;
        if rightmost < x {
            selected = Vec2::new(point[0], point[2]);
            rightmost = x;
        }
        floor = minimum(floor, point[1]);
    }
    floor = minimum(floor, maximum_y + (-0.75));
    // 原先求倒长度，再逐分量相乘；零向量产生NaN，不擅自选择备用轴。
    let horizontal = Vec3::new(eye_right.x, 0.0, eye_right.z);
    let reciprocal = 1.0
        / ((horizontal.y * horizontal.y + horizontal.x * horizontal.x)
            + horizontal.z * horizontal.z)
            .sqrt();
    let offset = (horizontal * reciprocal) * 1.5;
    (Vec3::new(selected.x, floor, selected.y) + offset, offset)
}

impl LerpedGizmoPosition {
    fn update(&mut self, target: Vec3, delta: f32) -> Vec3 {
        let position = self.0.map_or(target, |previous| {
            // 原140c90600，expf依赖复用已恢复的同一时间连续因子。
            (target - previous) * utils::time_continuous_lerp_factor(delta, 20.0) + previous
        });
        self.0 = Some(position);
        position
    }
}

fn arrow_rotation(right: Vec3, down: bool) -> Quat {
    let reciprocal = 1.0 / (right.x * right.x + right.z * right.z).sqrt();
    let direction = Vec2::new((-right.z) * reciprocal, right.x * reciprocal);
    let rotation = utils::xz_rotation_bad(direction);
    if down {
        // 原glam半周X旋转的常量，不能用不同精度的角度/三角函数重建。
        rotation * Quat::from_xyzw(1.0, 0.0, 0.0, f32::from_bits(0xb33b_bd2e))
    } else {
        rotation
    }
}

fn sphere(stream: &mut UiSignifierStream, name: String, position: Vec3, scale: f32) {
    stream.write_factory(&SignifierFactory::from_closure(
        &name,
        Arc::new(move |meshes, shaders, commands| {
            vec![
                SignifierEntityBuilder::from_mesh_name("temp_sphere")
                    .position(position)
                    .scale(scale)
                    .animation_duration(0.0)
                    .build(meshes, shaders, commands),
            ]
        }),
    ));
}

fn arrow(stream: &mut UiSignifierStream, name: String, position: Vec3, rotation: Quat) {
    stream.write_factory(&SignifierFactory::from_closure(
        &name,
        Arc::new(move |meshes, shaders, commands| {
            vec![
                SignifierEntityBuilder::from_mesh_name("brush_arrow_flat_single_up")
                    .position(position)
                    .rotation(rotation)
                    .animation_duration(0.0)
                    .build(meshes, shaders, commands),
            ]
        }),
    ));
}

/// 原生产者：真正注册两枚sphere控件，再由UiWidgetContext裁决拖拽。
/// 不直接修改高度、档案或历史，副作用交给原Adjust／Raise消费者。
#[allow(clippy::too_many_arguments)]
pub fn ui_focus_mode_shape_height(
    mode: Res<AppMode>,
    walls: Res<PublicWalls>,
    window: Res<WindowSize>,
    camera: Res<MainCamera>,
    ui: Res<UiState>,
    mut finish: EventWriter<FinishAdjWallHeightMode>,
    mut adjust: EventWriter<AdjustWallHeightUi>,
    mut init: EventWriter<InitAdjWallHeightMode>,
    mut raise: EventWriter<HandleRaiseShapeUiInteraction>,
    mut stream: ResMut<UiSignifierStream>,
    time: Res<GameTime>,
    mut position: Local<LerpedGizmoPosition>,
) {
    let wall = match *mode {
        AppMode::Editing(EditSubject::Wall(id) | EditSubject::Roof(id)) => WallId(id),
        _ => {
            (*position).0 = None;
            return;
        }
    };
    let record = walls.get(wall).expect("聚焦的发布墙必须存在");
    // 原先检查轮廓首点，再读取必须就绪的max_y缓存，保留失败先后次序。
    record
        .geometry
        .outline
        .first()
        .expect("聚焦墙必须拥有发布轮廓点");
    let maximum_y = record.max_y();
    let right = camera.camera_matrices().eye_right();
    let (target, offset) = target_position(
        &record.geometry.outline,
        maximum_y,
        window.width,
        camera.read_matrix(0),
        camera.read_matrix(0x80),
        right,
    );
    let floor = position.update(target, time.delta);
    let top = Vec3::new(floor.x, maximum_y, floor.z);
    DebugLine::new(vec![floor, top])
        .line_width(4.0)
        .show(&mut stream);

    let height_widget = ui.add(
        &format!("edit_height_{wall}"),
        WidgetParams::default()
            .geometry(WidgetGeometry::Sphere {
                center: top,
                radius: 0.5,
            })
            .occlude_by_world(false),
    );
    if height_widget.is_active() {
        if record.geometry.metrics.constrained_extra > 0.7 {
            arrow(
                &mut stream,
                format!("height_arrow_down_{wall}_{}", time.frame_count),
                (offset * 0.75 + top) + Vec3::new(0.0, -0.05, 0.0),
                arrow_rotation(right, true),
            );
        }
        if record.geometry.metrics.constrained_extra < 14.0 {
            arrow(
                &mut stream,
                format!("height_arrow_up_{wall}_{}", time.frame_count),
                (offset * 0.75 + top) + Vec3::new(0.0, 0.05, 0.0),
                arrow_rotation(right, false),
            );
        }
    }
    sphere(
        &mut stream,
        format!("height_gizmo_{wall}{}", time.frame_count),
        top,
        if height_widget.is_active() { 0.5 } else { 0.35 },
    );
    match height_widget.dragged() {
        2 => {}
        3 => {
            adjust.write(AdjustWallHeightUi(wall));
            init.write(InitAdjWallHeightMode);
        }
        4 => {
            adjust.write(AdjustWallHeightUi(wall));
        }
        _ => {
            finish.write(FinishAdjWallHeightMode(wall));
        }
    }

    let elevation_widget = ui.add(
        &format!("edit_elevation_{wall}"),
        WidgetParams::default()
            .geometry(WidgetGeometry::Sphere {
                center: floor,
                radius: 0.5,
            })
            .occlude_by_world(false),
    );
    if elevation_widget.is_active() {
        if record.authority.elevation() > 0.0 {
            arrow(
                &mut stream,
                format!("elevation_arrow_down_{wall}_{}", time.frame_count),
                (offset * 0.75 + floor) + Vec3::new(0.0, -0.05, 0.0),
                arrow_rotation(right, true),
            );
        }
        if record.authority.elevation() < MAX_ELEVATION {
            arrow(
                &mut stream,
                format!("elevation_arrow_up_{wall}_{}", time.frame_count),
                (offset * 0.75 + floor) + Vec3::new(0.0, 0.05, 0.0),
                arrow_rotation(right, false),
            );
        }
    }
    sphere(
        &mut stream,
        format!("elevation_gizmo_{wall}{}", time.frame_count),
        floor,
        if elevation_widget.is_active() {
            0.5
        } else {
            0.35
        },
    );
    raise.write(HandleRaiseShapeUiInteraction {
        widget: elevation_widget.id,
        wall,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset_libraries::{
            mesh_library::{AssetMeshLibrary, MeshAsset},
            shader_library::AssetShaderLibrary,
        },
        render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
        resources::{
            roofs::geometry::surface::Curve2,
            walls::{
                InnerWalls, WallState, public_walls::WallPublicationDirty,
                publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512,
            },
        },
        systems::{
            ui::{
                resources::ui_state::{UiInteraction, WidgetId},
                signifiers::{animation::SignifierAnimation, signifier_create::UiSignifierEntry},
            },
            wall::{
                valid_enclosure::{Shape, ShapeParameters},
                wall_state::{
                    path_preparation::PathRasters512, wall_spatial_hash::WallSpatialHash,
                },
            },
        },
        utils::components::transform::Transform,
    };
    use bevy_ecs::world::CommandQueue;

    #[derive(serde::Deserialize)]
    struct NativeTarget {
        points: Vec<[u32; 3]>,
        maximum_y: u32,
        width: u32,
        projection: [u32; 16],
        view: [u32; 16],
        eye_right: [u32; 3],
        target: [u32; 3],
        offset: [u32; 3],
    }

    /// 原指令真执行512例；有限结果逐位相等，NaN只比较分类。
    #[test]
    fn target_matches_original_bounded_instructions() {
        let cases: Vec<NativeTarget> = serde_json::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-focus-height-target.json"
        ))
        .unwrap();
        assert_eq!(cases.len(), 512);
        for (index, case) in cases.into_iter().enumerate() {
            let points = case
                .points
                .into_iter()
                .map(|p| p.map(f32::from_bits))
                .collect::<Vec<_>>();
            let (target, offset) = target_position(
                &points,
                f32::from_bits(case.maximum_y),
                case.width,
                Mat4::from_cols_array(&case.projection.map(f32::from_bits)),
                Mat4::from_cols_array(&case.view.map(f32::from_bits)),
                Vec3::from_array(case.eye_right.map(f32::from_bits)),
            );
            for (field, actual, expected) in [
                ("target", target, case.target),
                ("offset", offset, case.offset),
            ] {
                for (axis, (actual, expected)) in
                    actual.to_array().into_iter().zip(expected).enumerate()
                {
                    if f32::from_bits(expected).is_nan() {
                        assert!(actual.is_nan(), "case {index} {field}[{axis}]");
                    } else {
                        assert_eq!(actual.to_bits(), expected, "case {index} {field}[{axis}]");
                    }
                }
            }
        }
    }

    fn world() -> (World, WallId, Schedule) {
        let shape = ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d {
            center: Vec2::ZERO,
            radius: 2.0,
            rotation: 0.0,
        }));
        let points = shape
            .as_curve2()
            .points
            .iter()
            .map(|p| p.to_array())
            .collect();
        let mut authority =
            WallState::new(Curve2::try_new(points, false).unwrap(), 4.0, 0, 0, false);
        authority.snapshot.source.shape = WallShapeSnapshot::from_observed(shape.to_record());
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(authority);
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 2.,
            maximum_y: 2.,
            sampling_fix: false,
        };
        let paths = PathRasters512::from_encoded(
            std::array::from_fn(|_| vec![0u8; 512 * 512].into()),
            false,
        )
        .unwrap();
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &terrain,
            Some(&paths),
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        let mut camera = MainCamera::default();
        camera.write_matrix(0, &Mat4::IDENTITY);
        camera.write_matrix(0x80, &Mat4::IDENTITY);
        camera.set_camera_matrices(Mat4::IDENTITY);
        let mut world = World::new();
        world.insert_resource(public);
        world.insert_resource(camera);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        world.insert_resource(WindowSize::new(1000, 1000));
        world.insert_resource(GameTime {
            delta: 0.016,
            frame_count: 77,
            ..Default::default()
        });
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<Events<FinishAdjWallHeightMode>>();
        world.init_resource::<Events<AdjustWallHeightUi>>();
        world.init_resource::<Events<InitAdjWallHeightMode>>();
        world.init_resource::<Events<HandleRaiseShapeUiInteraction>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_focus_mode_shape_height);
        (world, wall, schedule)
    }

    fn interact(world: &mut World, widget: WidgetId, phase: u8) {
        let mut ui = UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: phase,
            widget,
        });
        ui.set_intersection_tag(0);
        world.insert_resource(ui);
    }

    fn factories(world: &mut World) -> Vec<SignifierFactory> {
        world
            .resource_mut::<UiSignifierStream>()
            .take_entries()
            .into_iter()
            .map(|e| match e {
                UiSignifierEntry::Factory(f) => f,
                _ => panic!("不能用raw占位替代真实工厂"),
            })
            .collect()
    }

    #[test]
    fn actual_widget_phases_emit_wall_id_and_real_raise_owner() {
        let (mut world, wall, mut schedule) = world();
        let height = WidgetId::from_name(&format!("edit_height_{wall}"));
        let elevation = WidgetId::from_name(&format!("edit_elevation_{wall}"));
        for phase in [2, 3, 4, 0, 1, 5, 255] {
            interact(&mut world, height, phase);
            schedule.run(&mut world);
            let finish = world
                .resource_mut::<Events<FinishAdjWallHeightMode>>()
                .drain()
                .map(|e| e.0)
                .collect::<Vec<_>>();
            let adjust = world
                .resource_mut::<Events<AdjustWallHeightUi>>()
                .drain()
                .map(|e| e.0)
                .collect::<Vec<_>>();
            let init = world
                .resource_mut::<Events<InitAdjWallHeightMode>>()
                .drain()
                .count();
            assert_eq!(
                finish,
                if matches!(phase, 0 | 1 | 5 | 255) {
                    vec![wall]
                } else {
                    vec![]
                },
                "phase {phase}"
            );
            assert_eq!(
                adjust,
                if matches!(phase, 3 | 4) {
                    vec![wall]
                } else {
                    vec![]
                },
                "phase {phase}"
            );
            assert_eq!(init, usize::from(phase == 3));
            let raised = world
                .resource_mut::<Events<HandleRaiseShapeUiInteraction>>()
                .drain()
                .collect::<Vec<_>>();
            assert_eq!(
                raised,
                [HandleRaiseShapeUiInteraction {
                    widget: elevation,
                    wall
                }]
            );
            for widget in [height, elevation] {
                let params = world.resource::<UiState>().widget(widget).unwrap();
                assert!(matches!(
                    params.geometry,
                    Some(WidgetGeometry::Sphere { radius: 0.5, .. })
                ));
                assert!(!params.occlude_by_world);
            }
            let entries = factories(&mut world);
            assert_eq!(entries.len(), 5);
            assert_eq!(
                entries[1].name(),
                Some(format!("height_arrow_down_{wall}_77").as_str())
            );
            assert_eq!(
                entries[2].name(),
                Some(format!("height_arrow_up_{wall}_77").as_str())
            );
            assert_eq!(
                entries[3].name(),
                Some(format!("height_gizmo_{wall}77").as_str())
            );
            assert_eq!(
                entries[4].name(),
                Some(format!("elevation_gizmo_{wall}77").as_str())
            );
        }
    }

    #[test]
    fn actual_factories_keep_transforms_meshes_and_immediate_animation() {
        let (mut world, wall, mut schedule) = world();
        interact(
            &mut world,
            WidgetId::from_name(&format!("edit_elevation_{wall}")),
            2,
        );
        schedule.run(&mut world);
        let entries = factories(&mut world);
        // 初始绝对抬升为0，只存在向上箭头；DebugLine、两sphere均是真实工厂。
        assert_eq!(entries.len(), 4);
        assert_eq!(
            entries[2].name(),
            Some(format!("elevation_arrow_up_{wall}_77").as_str())
        );
        let meshes = AssetMeshLibrary::new();
        for name in ["temp_sphere", "brush_arrow_flat_single_up"] {
            meshes.add(MeshAsset::new(Mesh::new(name), Some(name.to_owned())));
        }
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(1),
            ),
            false,
        );
        let mut queue = CommandQueue::default();
        let entities = entries
            .iter()
            .flat_map(|f| f.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world)))
            .collect::<Vec<_>>();
        queue.apply(&mut world);
        assert_eq!(entities.len(), 4);
        let floor = match world
            .resource::<UiState>()
            .widget(WidgetId::from_name(&format!("edit_elevation_{wall}")))
            .unwrap()
            .geometry
            .as_ref()
            .unwrap()
        {
            WidgetGeometry::Sphere { center, .. } => *center,
            _ => unreachable!(),
        };
        let top = match world
            .resource::<UiState>()
            .widget(WidgetId::from_name(&format!("edit_height_{wall}")))
            .unwrap()
            .geometry
            .as_ref()
            .unwrap()
        {
            WidgetGeometry::Sphere { center, .. } => *center,
            _ => unreachable!(),
        };
        for (index, entity) in entities.iter().copied().enumerate() {
            let transform = world.get::<Transform>(entity).unwrap();
            assert!(transform.translation.is_finite());
            if index > 0 {
                let expected = if index == 1 {
                    top
                } else if index == 2 {
                    floor + Vec3::new(1.125, 0.05, 0.)
                } else {
                    floor
                };
                assert_eq!(transform.translation, expected);
            }
            let mut animation = world.get_mut::<SignifierAnimation>(entity).unwrap();
            assert_eq!(animation.duration, 0.);
            animation.advance(0., false);
            if index > 0 {
                assert_eq!(
                    animation.value,
                    if index == 1 {
                        0.35
                    } else if index == 3 {
                        0.5
                    } else {
                        1.
                    }
                );
            }
        }
        assert_eq!(
            *world
                .get::<crate::asset_libraries::Handle<Mesh>>(entities[1])
                .unwrap(),
            meshes.get_handle_by_name("temp_sphere")
        );
        assert_eq!(
            *world
                .get::<crate::asset_libraries::Handle<Mesh>>(entities[2])
                .unwrap(),
            meshes.get_handle_by_name("brush_arrow_flat_single_up")
        );
    }
    #[derive(serde::Deserialize)]
    struct NativeRotation {
        right: [u32; 3],
        up: [u32; 4],
        down: [u32; 4],
    }
    #[test]
    fn arrows_match_original_normalization_and_quaternion_product() {
        let cases: Vec<NativeRotation> = serde_json::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-focus-height-rotation.json"
        ))
        .unwrap();
        assert_eq!(cases.len(), 512);
        for (index, case) in cases.into_iter().enumerate() {
            for (down, expected) in [(false, case.up), (true, case.down)] {
                let actual = arrow_rotation(Vec3::from_array(case.right.map(f32::from_bits)), down)
                    .to_array();
                for (axis, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
                    if f32::from_bits(expected).is_nan() {
                        assert!(actual.is_nan(), "case {index} down {down} axis {axis}");
                    } else {
                        assert_eq!(
                            actual.to_bits(),
                            expected,
                            "case {index} down {down} axis {axis}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn original_thresholds_and_same_schedule_mode_exit_reentry() {
        let (mut world, wall, mut schedule) = world();
        let height = WidgetId::from_name(&format!("edit_height_{wall}"));
        for (value, down, up) in [
            (0.7, false, true),
            (f32::from_bits(0.7f32.to_bits() + 1), true, true),
            (14., true, false),
            (f32::NAN, false, false),
        ] {
            world
                .resource_mut::<PublicWalls>()
                .get_mut(wall)
                .unwrap()
                .geometry
                .metrics
                .constrained_extra = value;
            interact(&mut world, height, 2);
            schedule.run(&mut world);
            let entries = factories(&mut world);
            let names = entries.iter().filter_map(|f| f.name()).collect::<Vec<_>>();
            assert_eq!(
                names.iter().any(|n| n.starts_with("height_arrow_down_")),
                down
            );
            assert_eq!(names.iter().any(|n| n.starts_with("height_arrow_up_")), up);
        }
        let elevation = WidgetId::from_name(&format!("edit_elevation_{wall}"));
        for (value, down, up) in [
            (0., false, true),
            (60., true, false),
            (f32::from_bits(60f32.to_bits() - 1), true, true),
            (f32::NAN, false, false),
        ] {
            world
                .resource_mut::<PublicWalls>()
                .get_mut(wall)
                .unwrap()
                .authority
                .snapshot
                .source
                .terrain_floor = value;
            interact(&mut world, elevation, 2);
            schedule.run(&mut world);
            let entries = factories(&mut world);
            let names = entries.iter().filter_map(|f| f.name()).collect::<Vec<_>>();
            assert_eq!(
                names.iter().any(|n| n.starts_with("elevation_arrow_down_")),
                down
            );
            assert_eq!(
                names.iter().any(|n| n.starts_with("elevation_arrow_up_")),
                up
            );
        }
        // 同一Schedule保留Local：移动发布轮廓后，Wall→Roof仍渐进跟随。
        let floor = |world: &World| match world
            .resource::<UiState>()
            .widget(WidgetId::from_name(&format!("edit_elevation_{wall}")))
            .unwrap()
            .geometry
            .as_ref()
            .unwrap()
        {
            WidgetGeometry::Sphere { center, .. } => *center,
            _ => unreachable!(),
        };
        let before = floor(&world);
        {
            let mut walls = world.resource_mut::<PublicWalls>();
            let record = walls.get_mut(wall).unwrap();
            let points = Arc::make_mut(&mut record.geometry.outline);
            for point in points {
                point[0] += 100.;
            }
        }
        world.insert_resource(AppMode::Editing(EditSubject::Roof(wall.0)));
        schedule.run(&mut world);
        let moving = floor(&world);
        assert!(moving.x > before.x && moving.x < before.x + 100.);
        factories(&mut world);
        world.insert_resource(AppMode::default());
        world.resource::<UiState>().clear_widgets();
        schedule.run(&mut world);
        assert!(
            world
                .resource_mut::<UiSignifierStream>()
                .take_entries()
                .is_empty()
        );
        assert!(world.resource::<UiState>().registry_snapshot().is_empty());
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        schedule.run(&mut world);
        assert_eq!(floor(&world), before + Vec3::new(100., 0., 0.));
    }

    /// 真正消费工厂、捕获活动参数并走回收，不手工despawn掩盖卸载问题。
    #[test]
    fn actual_signifier_consumer_releases_entities_and_transient_mesh() {
        use crate::{
            asset_libraries::{
                shader_library::SharedAssetShaderLibrary,
                transient_mesh::{TransientMesh, delete_dropped_transient_meshes},
                vao_library::AssetVAOLibrary,
            },
            systems::ui::signifiers::{
                animation::signifier_animation_t,
                signifier_create::{SignifierId, signifier_create},
                signifier_mark_for_delete::{despawn_marked, signfier_mark_for_delete},
            },
        };
        use bevy_ecs::system::RunSystemOnce;
        let (mut world, wall, mut schedule) = world();
        let widget = WidgetId::from_name(&format!("edit_height_{wall}"));
        interact(&mut world, widget, 2);
        schedule.run(&mut world);
        let meshes = AssetMeshLibrary::new();
        for name in ["temp_sphere", "brush_arrow_flat_single_up"] {
            meshes.add(MeshAsset::new(Mesh::new(name), Some(name.to_owned())));
        }
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(1),
            ),
            false,
        );
        world.insert_resource(meshes);
        world.insert_resource(SharedAssetShaderLibrary(Arc::new(shaders)));
        let params = world.resource::<UiState>().widget(widget).unwrap();
        world.resource::<UiState>().clear_widgets();
        world.resource_mut::<UiState>().replace_interaction_full(
            UiInteraction {
                native_tag: 2,
                native_phase: 2,
                widget,
            },
            Some(params),
            [0; 56],
        );
        world.run_system_once(signifier_create).unwrap();
        assert!(
            world
                .resource_mut::<UiSignifierStream>()
                .take_entries()
                .is_empty()
        );
        let entities = world
            .query_filtered::<Entity, With<SignifierId>>()
            .iter(&world)
            .collect::<Vec<_>>();
        assert_eq!(entities.len(), 5);
        let generated = entities
            .iter()
            .find_map(|&entity| {
                world
                    .get::<TransientMesh>(entity)
                    .map(|receipt| receipt.mesh_handle())
            })
            .unwrap();
        assert!(
            world
                .resource::<AssetMeshLibrary>()
                .get(generated)
                .is_some()
        );
        // 捕获owner归零，原mark→反向瞬时动画→despawn释放动态DebugLine网格。
        let owner = Arc::downgrade(&world.resource::<UiState>().active_params().unwrap());
        world.insert_resource(UiState::at_startup());
        assert!(owner.upgrade().is_none());
        world.run_system_once(signfier_mark_for_delete).unwrap();
        world.run_system_once(signifier_animation_t).unwrap();
        world.run_system_once(despawn_marked).unwrap();
        assert!(entities.into_iter().all(|e| world.get_entity(e).is_err()));
        let mut vaos = AssetVAOLibrary::new();
        delete_dropped_transient_meshes(
            &TestDevice(1),
            &mut world.resource_mut::<AssetMeshLibrary>(),
            &mut vaos,
        );
        assert!(
            world
                .resource::<AssetMeshLibrary>()
                .get(generated)
                .is_none()
        );
        assert!(
            world
                .resource::<AssetMeshLibrary>()
                .get(
                    world
                        .resource::<AssetMeshLibrary>()
                        .get_handle_by_name("temp_sphere")
                )
                .is_some()
        );
    }
    #[derive(serde::Deserialize)]
    struct NativeEvents {
        hover: u8,
        matching: bool,
        phase: u8,
        finish: usize,
        adjust: usize,
        init: usize,
    }
    #[test]
    fn all_original_drag_dispatch_cases_match_actual_widget_events() {
        let cases: Vec<NativeEvents> = serde_json::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-focus-height-events.json"
        ))
        .unwrap();
        assert_eq!(cases.len(), 2048);
        let (mut world, wall, mut schedule) = world();
        let widget = WidgetId::from_name(&format!("edit_height_{wall}"));
        for case in cases {
            let id = if case.matching {
                widget
            } else {
                WidgetId(widget.0.wrapping_add(1))
            };
            world.insert_resource(UiState::new(UiInteraction {
                native_tag: case.hover,
                native_phase: case.phase,
                widget: id,
            }));
            schedule.run(&mut world);
            let finish = world
                .resource_mut::<Events<FinishAdjWallHeightMode>>()
                .drain()
                .map(|e| e.0)
                .collect::<Vec<_>>();
            let adjust = world
                .resource_mut::<Events<AdjustWallHeightUi>>()
                .drain()
                .map(|e| e.0)
                .collect::<Vec<_>>();
            assert_eq!(
                finish,
                vec![wall; case.finish],
                "hover {} match {} phase {}",
                case.hover,
                case.matching,
                case.phase
            );
            assert_eq!(
                adjust,
                vec![wall; case.adjust],
                "hover {} match {} phase {}",
                case.hover,
                case.matching,
                case.phase
            );
            assert_eq!(
                world
                    .resource_mut::<Events<InitAdjWallHeightMode>>()
                    .drain()
                    .count(),
                case.init
            );
            world
                .resource_mut::<Events<HandleRaiseShapeUiInteraction>>()
                .clear();
            factories(&mut world);
        }
    }
}
