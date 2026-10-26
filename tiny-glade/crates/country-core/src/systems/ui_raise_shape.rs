//! B ui_raise_shape：预制建筑的底边拾取、抬升拖拽、历史和反馈。
//! 原命令16B为WidgetId/WallId；不能用单位事件或外部位置代理替代。
use crate::resources::walls::public_walls::PublicWalls;
use crate::{
    resources::{
        critters::sheep::CmdCheckForSheepInsideBuilding,
        glade_border::MaxDemoElevationReached,
        history::{
            EditType, History,
            wall_edit::{CompoundEditBuilder, WallHistoryEdit, WallValueEdit},
        },
        mouse_terrain_raycast::CursorPositionSS,
        time::GameTime,
        walls::{
            InnerWalls, WallId,
            decorator_archivist::{DecoratorArchivalResources, LiveDecoratorArchivist},
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        },
        window::WindowSize,
    },
    systems::{
        audio::{PlayBuildingAudio, RequestWallMaterialAudio},
        camera_rig::MainCamera,
        mode_manager::{AppMode, EditSubject},
        ui::{
            resources::{
                ui_state::{UiState, WidgetId},
                widget_params::{WidgetGeometry, WidgetParams},
            },
            signifiers::{
                SignifierEntityBuilder, SignifierFactory, UiSignifierStream,
                signifier_factory::SignifierMesh,
            },
        },
    },
    utils::project_cursor_onto_ray::{ProportionalScreenSpace, ProportionalScreenSpaceCoord},
};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use std::sync::Arc;

/// B原只读静态量142ab44c8。
pub const MAX_ELEVATION: f32 = 60.0;

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandleRaiseShapeUiInteraction {
    pub widget: WidgetId,
    pub wall: WallId,
}

/// B Local：原起点、初始/当前/上一反馈高度和最低地面值。
#[derive(Clone, Copy, Debug, Default)]
pub struct RaiseShape {
    pub origin: Vec3,
    pub initial_elevation: f32,
    pub current_elevation: f32,
    pub last_feedback_elevation: f32,
    pub terrain_floor: f32,
}

fn target_elevation(state: &RaiseShape, projected: Vec3) -> f32 {
    // 原 MAXSS 在相等时选初始高度；保留 -0 与 NaN 的第二操作数语义。
    let base = if state.initial_elevation.is_nan() || state.terrain_floor > state.initial_elevation
    {
        state.terrain_floor
    } else {
        state.initial_elevation
    };
    (base + (projected.y - state.origin.y)).clamp(0., MAX_ELEVATION)
}

fn ground_floor(public: &PublicWalls, wall: WallId) -> f32 {
    let state = public.get(wall).expect("抬升的发布墙不存在");
    // PublicWalls::get 已执行原源曲线断言；不为派生轮廓另造约束。
    let mut floor = 100000.0f32;
    for point in state.geometry.outline.iter() {
        floor = if floor.is_nan() || point[1] < floor {
            point[1]
        } else {
            floor
        };
    }
    // 原末尾 MAXSS 的第二操作数固定为 +0，含 -0/NaN 时均选 +0。
    if floor > 0.0 { floor } else { 0.0 }
}

fn archive_raise(
    archivist: &mut LiveDecoratorArchivist,
    storage: &DecoratorStorage,
    resources: &DecoratorArchivalResources<'_>,
    wall: WallId,
) {
    // 原140955a68归档整墙及地面，并读取活碰撞体/网格；不能只克隆装饰物元数据。
    archivist.archive_current_decorators_in_storages_with_resources(
        &[
            DecoratorStorageAddr::Wall(wall),
            DecoratorStorageAddr::Terrain,
        ],
        storage,
        resources,
        "change_elevation",
    );
    archivist.decorator_cull_allow_wall(wall, storage);
    if let Some(terrain) = storage.get(DecoratorStorageAddr::Terrain) {
        archivist
            .culling
            .allowed_ids
            .extend(terrain.decorators().keys().copied());
    }
}

/// B 140952e70：普通编辑模式只为预制建筑注册底边拾取体。
/// prepared_path 是原公开 Curve3，不能以未经路径处理的地形轮廓替换。
pub fn ui_raise_shape(
    mode: Res<AppMode>,
    walls: Res<PublicWalls>,
    mut stream: ResMut<UiSignifierStream>,
    ui: Res<UiState>,
    time: Res<GameTime>,
    mut interactions: EventWriter<HandleRaiseShapeUiInteraction>,
) {
    if !matches!(*mode, AppMode::Editing(EditSubject::None)) {
        return;
    }
    let mut active = None;
    for (wall, state) in walls.all() {
        state.assert_valid();
        if !state.is_premade_shape() {
            continue;
        }
        let path = state
            .prepared_path
            .as_ref()
            .expect("抬升拾取前必须完成公开墙路径准备");
        let centers = path
            .positions
            .iter()
            .map(|point| Vec3::new(point[0], point[1] - 0.2, point[2]))
            .collect();
        let context = ui.add(
            &format!("raise_wall_elevation_{wall}"),
            WidgetParams::default().geometry(WidgetGeometry::SphereCloud {
                centers,
                radius: 0.5,
            }),
        );
        if context.is_active() {
            active = Some((wall, context.id));
        }
    }
    let Some((wall, widget)) = active else {
        return;
    };
    let state = walls.get(wall).expect("活动抬升墙必须存在");
    let path = state
        .prepared_path
        .as_ref()
        .expect("活动抬升墙路径必须存在");
    let points: Vec<Vec3> = path
        .positions
        .iter()
        .copied()
        .map(Vec3::from_array)
        .collect();
    let outline = points.clone();
    stream.write_factory(&SignifierFactory::from_closure(
        &format!("outline_wall_bottom_{wall}_{}", time.frame_count),
        Arc::new(move |meshes, shaders, commands| {
            let entity = SignifierEntityBuilder::new(SignifierMesh::Points(outline.clone()))
                .animation_duration(0.0)
                .build(meshes, shaders, commands);
            commands.entity(entity).insert(
                crate::render::pass_builder::draw_list::DrawTopology::Alt(3.0),
            );
            vec![entity]
        }),
    ));
    // 与原作相同，在轮廓提交后再次核对交点，不从屏幕坐标另造世界交点。
    if !ui.widget_context(widget).is_active() {
        return;
    }
    let intersection = ui.intersection_point();
    let curve = utils::curve::Curve3::from_cached_parts(points, path.points_u.clone(), path.length);
    let u = curve
        .get_approx_u_from_pos(Vec2::new(intersection.x, intersection.z))
        .clamp(0., 1.);
    let coordinate = curve.get_coord_at_u(u);
    let first = curve.points[coordinate.segment];
    let second = curve.points[coordinate.segment + 1];
    let position = first * (1.0 - coordinate.t) + second * coordinate.t;
    // 原先求三维单位切线，再求XZ法向的单位向量；两个除法阶段都不可省略。
    let normal = raise_arrow_normal(second - first);
    let rotation = utils::xz_rotation_bad(normal);
    stream.write_factory(&SignifierFactory::from_closure(
        &format!("arrow_raise_shape_{wall}_{}", time.frame_count),
        Arc::new(move |meshes, shaders, commands| {
            vec![
                SignifierEntityBuilder::from_mesh_name("brush_arrow_flat")
                    .position(position)
                    .rotation(rotation)
                    .animation_duration(0.0)
                    .build(meshes, shaders, commands),
            ]
        }),
    ));
    interactions.write(HandleRaiseShapeUiInteraction { widget, wall });
}

fn raise_arrow_normal(tangent: Vec3) -> Vec2 {
    // 140954074..1409540d3：原代码是先求倒长度再乘，不能换为逐分量除法。
    let inverse =
        1.0 / (tangent.z * tangent.z + (tangent.y * tangent.y + tangent.x * tangent.x)).sqrt();
    let tx = tangent.x * inverse;
    let tz = tangent.z * inverse;
    let inverse_xz = 1.0 / (tx * tx + tz * tz).sqrt();
    Vec2::new((-tz) * inverse_xz, tx * inverse_xz)
}

fn feedback_kind(previous: f32, current: f32) -> Option<u8> {
    if previous == 0. && current > 0. {
        Some(6)
    } else if previous > 0. && current == 0. {
        Some(7)
    } else if previous < MAX_ELEVATION && current == MAX_ELEVATION {
        Some(8)
    } else {
        None
    }
}

/// B 140955650..1409564c5，后续两个funclet仅负责Rust析构。
/// 事件按原顺序消费，None不制造完成；Finished不跳过相等历史。
#[allow(clippy::too_many_arguments)]
pub fn impl_raise_shape(
    ui: Res<UiState>,
    mut events: EventReader<HandleRaiseShapeUiInteraction>,
    (mut material, mut building): (
        EventWriter<RequestWallMaterialAudio>,
        EventWriter<PlayBuildingAudio>,
    ),
    (storage, mut archivist, mut history): (
        Res<DecoratorStorage>,
        ResMut<LiveDecoratorArchivist>,
        ResMut<History>,
    ),
    mut state: Local<RaiseShape>,
    public: Res<PublicWalls>,
    mut inner: ResMut<InnerWalls>,
    time: Res<GameTime>,
    archival: DecoratorArchivalResources,
    cursor: Res<CursorPositionSS>,
    camera: Res<MainCamera>,
    window: Res<WindowSize>,
    mut sheep: EventWriter<CmdCheckForSheepInsideBuilding>,
    mut maximum: EventWriter<MaxDemoElevationReached>,
) {
    for event in events.read() {
        let context = ui.widget_context(event.widget);
        let phase = context.dragged();
        match phase {
            2 => {}
            3 => {
                archive_raise(&mut archivist, &storage, &archival, event.wall);
                assert!(context.is_active(), "原抬升开始必须有有效世界交点");
                state.origin = ui.intersection_point();
                let initial = inner
                    .get(event.wall)
                    .expect("原抬升的权威墙不存在")
                    .elevation();
                state.initial_elevation = initial;
                state.current_elevation = initial;
                state.last_feedback_elevation = initial;
                state.terrain_floor = ground_floor(&public, event.wall);
            }
            4 => {
                let screen = ProportionalScreenSpace::from_window_size(window.as_dims());
                let coordinate = ProportionalScreenSpaceCoord(
                    screen.transform_screen_space(cursor.0.x, cursor.0.y),
                );
                let projected = coordinate.project_onto_ray(
                    &screen,
                    &camera.read_matrix(0x80),
                    &camera.read_matrix(0),
                    utils::geometry::Ray {
                        origin: state.origin,
                        direction: Vec3::Y,
                    },
                    [-f32::MAX, f32::MAX],
                );
                let target = target_elevation(&state, projected);
                if target >= MAX_ELEVATION {
                    maximum.write(MaxDemoElevationReached(
                        public
                            .get_shape_params(event.wall)
                            .expect("原最高抬升提示只接受预制形状"),
                    ));
                }
                inner.update_elevation(event.wall, target);
                state.current_elevation = target;
            }
            _ => {
                let after = inner
                    .get(event.wall)
                    .expect("原抬升历史的权威墙不存在")
                    .elevation();
                let mut builder = CompoundEditBuilder::default();
                builder.new_edit_sequence();
                builder.next(WallHistoryEdit::ChangeElevation(WallValueEdit {
                    wall_index: event.wall.0,
                    before: state.initial_elevation,
                    after,
                }));
                history
                    .add(EditType::Wall(Arc::new(builder.build())))
                    .unwrap_or_else(|error| panic!("原抬升历史写入失败: {error:?}"));
                sheep.write(CmdCheckForSheepInsideBuilding { wall: event.wall });
            }
        }
        let interaction = ui.interaction();
        if interaction.native_tag < 2
            || interaction.widget != event.widget
            || interaction.native_phase == 2
        {
            continue;
        }
        let current = state.current_elevation;
        let previous = state.last_feedback_elevation;
        if current == previous {
            continue;
        }
        let position = Vec3::new(state.origin.x, current, state.origin.z);
        let speed = ((current - previous).abs() / time.delta / 200.).clamp(0., 1.);
        material.write(RequestWallMaterialAudio {
            id: event.wall.0,
            position,
            kind: 0,
            flag: 0,
        });
        if let Some(kind) = feedback_kind(previous, current) {
            building.write(PlayBuildingAudio {
                id: event.wall.0,
                position,
                editing_speed: speed,
                kind,
            });
        }
        state.last_feedback_elevation = current;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// 通过实际公开墙生产、UiState注册及工厂唤醒验证，不用手工向消费者塞事件。
    #[test]
    fn actual_editing_widget_emits_arrow_outline_and_raise_command() {
        use crate::{
            asset_libraries::{
                mesh_library::{AssetMeshLibrary, MeshAsset},
                shader_library::AssetShaderLibrary,
            },
            render::{
                mesh::Mesh, pass_builder::draw_list::DrawTopology, shader::RasterShaderProgram,
                test_device::TestDevice,
            },
            resources::{
                roofs::geometry::surface::Curve2,
                walls::{
                    WallState, public_walls::WallPublicationDirty,
                    publication_fields::WallShapeSnapshot,
                    publication_source::TerrainHeightGrid512,
                },
            },
            systems::{
                ui::{
                    resources::ui_state::UiInteraction,
                    signifiers::{
                        animation::SignifierAnimation, signifier_create::UiSignifierEntry,
                    },
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
        use bevy_ecs::{system::RunSystemOnce, world::CommandQueue};
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
            minimum_y: 2.0,
            maximum_y: 2.0,
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
        let original_points = public
            .get(wall)
            .unwrap()
            .prepared_path
            .as_ref()
            .unwrap()
            .positions
            .clone();
        let widget = WidgetId::from_name(&format!("raise_wall_elevation_{wall}"));
        let mut ui = UiState::new(UiInteraction {
            native_tag: 1,
            native_phase: 0,
            widget,
        });
        ui.set_intersection_tag(0);
        ui.set_intersection_point(Vec3::new(2., 2., 0.));
        let mut world = World::new();
        world.insert_resource(public);
        world.insert_resource(ui);
        world.insert_resource(AppMode::Editing(EditSubject::None));
        world.insert_resource(GameTime {
            frame_count: 77,
            ..Default::default()
        });
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<Events<HandleRaiseShapeUiInteraction>>();
        world.run_system_once(ui_raise_shape).unwrap();
        let parameters = world.resource::<UiState>().widget(widget).unwrap();
        let WidgetGeometry::SphereCloud { centers, radius } = parameters.geometry.as_ref().unwrap()
        else {
            panic!("原拾取体必须为球云")
        };
        assert_eq!(*radius, 0.5);
        assert_eq!(centers.len(), original_points.len());
        for (center, original) in centers.iter().zip(original_points) {
            assert_eq!(
                *center,
                Vec3::new(original[0], original[1] - 0.2, original[2])
            );
        }
        let event = world
            .resource_mut::<Events<HandleRaiseShapeUiInteraction>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(event, [HandleRaiseShapeUiInteraction { widget, wall }]);
        let factories = world.resource_mut::<UiSignifierStream>().take_entries();
        let names = factories
            .iter()
            .map(|entry| match entry {
                UiSignifierEntry::Factory(f) => f.name().unwrap(),
                _ => panic!("不能使用raw记录替代真实工厂"),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [
                format!("outline_wall_bottom_{wall}_77"),
                format!("arrow_raise_shape_{wall}_77")
            ]
        );
        let meshes = AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(
            Mesh::new("arrow"),
            Some("brush_arrow_flat".to_owned()),
        ));
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
        let entities = factories
            .iter()
            .flat_map(|entry| match entry {
                UiSignifierEntry::Factory(f) => {
                    f.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world))
                }
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        queue.apply(&mut world);
        assert_eq!(entities.len(), 2);
        assert!(
            matches!(world.get::<DrawTopology>(entities[0]),Some(DrawTopology::Alt(width)) if *width==3.0)
        );
        for entity in entities {
            assert_eq!(
                world.get::<SignifierAnimation>(entity).unwrap().duration,
                0.0
            );
            assert!(world.get::<Transform>(entity).unwrap().rotation.is_finite());
        }
        // 保留同一Schedule的Local状态，走真实producer→拖拽→权威墙→历史，而非直接注入Raise事件。
        let initial_elevation = inner.get(wall).unwrap().elevation();
        world.insert_resource(inner);
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        let mut history =
            History::<crate::resources::history::MaybePendingSnapshotId>::empty_glade();
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        world.insert_resource(CursorPositionSS(Vec2::new(500., -30000.)));
        world.insert_resource(WindowSize::new(1000, 1000));
        let mut camera = MainCamera::default();
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &glam::Mat4::IDENTITY);
        world.insert_resource(camera);
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        world.init_resource::<Events<PlayBuildingAudio>>();
        world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
        world.init_resource::<Events<MaxDemoElevationReached>>();
        let mut schedule = Schedule::default();
        schedule.add_systems((ui_raise_shape, impl_raise_shape).chain());
        for phase in [3, 4, 0] {
            let mut ui = UiState::new(UiInteraction {
                native_tag: 2,
                native_phase: phase,
                widget,
            });
            ui.set_intersection_tag(0);
            ui.set_intersection_point(Vec3::new(2., 2., 0.));
            world.insert_resource(ui);
            schedule.run(&mut world);
            world.resource_mut::<UiSignifierStream>().take_entries();
        }
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .elevation(),
            MAX_ELEVATION
        );
        let hints = world
            .resource_mut::<Events<MaxDemoElevationReached>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(hints.len(), 1);
        assert_eq!(hints[0].0.to_record(), shape.to_record());
        assert_eq!(
            world
                .resource::<Events<CmdCheckForSheepInsideBuilding>>()
                .len(),
            1
        );
        let audio = world
            .resource_mut::<Events<PlayBuildingAudio>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(audio.len(), 1);
        assert_eq!(audio[0].kind, if initial_elevation == 0.0 { 6 } else { 8 });
        assert_eq!(
            world.resource::<Events<RequestWallMaterialAudio>>().len(),
            1
        );
        let history = world.resource::<History>();
        let edits = &history.pending().unwrap().edits;
        assert_eq!(edits.len(), 1);
        let EditType::Wall(compound) = &edits[0] else {
            panic!("必须记录墙体复合历史")
        };
        assert_eq!(compound.groups.len(), 1);
        let WallHistoryEdit::ChangeElevation(change) = &compound.groups[0][0] else {
            panic!("必须记录高度历史")
        };
        assert_eq!(change.wall_index, wall.0);
        assert_eq!(change.before, initial_elevation);
        assert_eq!(change.after, 60.0);
        assert!(
            world
                .resource::<LiveDecoratorArchivist>()
                .special1
                .is_some()
        );
        // 同一真实世界切到聚焦入口，再走独立持久Local的Raise消费者。
        world
            .resource_mut::<Events<HandleRaiseShapeUiInteraction>>()
            .clear();
        world
            .resource_mut::<InnerWalls>()
            .update_elevation(wall, 0.0);
        world
            .resource_mut::<History>()
            .start_input_recording(1)
            .unwrap();
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        world.init_resource::<Events<crate::resources::events::FinishAdjWallHeightMode>>();
        world.init_resource::<Events<crate::resources::events::InitAdjWallHeightMode>>();
        world.init_resource::<Events<crate::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>>();
        let focus_widget = WidgetId::from_name(&format!("edit_elevation_{wall}"));
        let mut focus = Schedule::default();
        focus.add_systems(
            (
                crate::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height,
                impl_raise_shape,
            )
                .chain(),
        );
        for phase in [3, 4, 0] {
            let mut ui = UiState::new(UiInteraction {
                native_tag: 2,
                native_phase: phase,
                widget: focus_widget,
            });
            ui.set_intersection_tag(0);
            ui.set_intersection_point(Vec3::new(2., 2., 0.));
            world.insert_resource(ui);
            focus.run(&mut world);
            world.resource_mut::<UiSignifierStream>().take_entries();
        }
        assert!(matches!(
            world
                .resource::<UiState>()
                .widget(focus_widget)
                .unwrap()
                .geometry,
            Some(WidgetGeometry::Sphere { .. })
        ));
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .elevation(),
            60.0
        );
        assert_eq!(world.resource::<Events<MaxDemoElevationReached>>().len(), 1);
        assert_eq!(
            world
                .resource::<Events<CmdCheckForSheepInsideBuilding>>()
                .len(),
            2
        );
        let focused_edits = &world.resource::<History>().pending().unwrap().edits;
        assert_eq!(focused_edits.len(), 1);
        let EditType::Wall(compound) = &focused_edits[0] else {
            panic!("聚焦抬升须记录墙历史")
        };
        let WallHistoryEdit::ChangeElevation(change) = &compound.groups[0][0] else {
            panic!("聚焦抬升须记录高度历史")
        };
        assert_eq!((change.before, change.after), (0.0, 60.0));
        // 最后一帧的真实事件已被consumer读取，清空存储后检查模式门禁。
        world
            .resource_mut::<Events<HandleRaiseShapeUiInteraction>>()
            .clear();
        // 摄影/工具/选中建筑各自拥有入口，本入口不制造额外交互或提示。
        world.insert_resource(AppMode::Photomode);
        world.resource::<UiState>().clear_widgets();
        world.run_system_once(ui_raise_shape).unwrap();
        assert!(world.resource::<UiState>().registry_snapshot().is_empty());
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        assert!(
            world
                .resource::<Events<HandleRaiseShapeUiInteraction>>()
                .is_empty()
        );
    }

    #[test]
    fn original_target_and_feedback_instruction_fixtures() {
        let rows: Vec<([u32; 4], u32)> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-raise-target.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 10000);
        for (input, expected) in rows {
            let [original, projected, initial, floor] = input.map(f32::from_bits);
            let state = RaiseShape {
                origin: Vec3::new(0., original, 0.),
                initial_elevation: initial,
                terrain_floor: floor,
                ..Default::default()
            };
            let actual = target_elevation(&state, Vec3::new(0., projected, 0.));
            if f32::from_bits(expected).is_nan() {
                assert!(actual.is_nan(), "{input:?}");
            } else {
                assert_eq!(actual.to_bits(), expected, "{input:?}");
            }
        }
        let rows: Vec<([u32; 3], [u32; 2])> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-raise-normal.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 1000);
        for (input, expected) in rows {
            let actual = raise_arrow_normal(Vec3::from_array(input.map(f32::from_bits))).to_array();
            for (actual, expected) in actual.into_iter().zip(expected) {
                if f32::from_bits(expected).is_nan() {
                    assert!(actual.is_nan(), "{input:?}");
                } else {
                    assert_eq!(actual.to_bits(), expected, "{input:?}");
                }
            }
        }
        let rows: Vec<([u32; 2], u8)> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-raise-feedback.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 100);
        for ([previous, current], expected) in rows {
            assert_eq!(
                feedback_kind(f32::from_bits(previous), f32::from_bits(current)),
                if expected == 255 {
                    None
                } else {
                    Some(expected)
                }
            );
        }
    }
}
