//! 原 B 14165d460：圆形半径的拾取、屏幕投影、边界限制、装饰归档和历史事务。

use ::utils::geometry::Circle2d;
use bevy_ecs::prelude::*;

use super::super::to_authority_curve;
use crate::resources::walls::InnerWalls;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};

/// 原半径变化历史阈值（0x3c23d70a @0x142960674）。
const CHANGE_THRESHOLD: f32 = 0.01;
/// 原编辑入口与完整圆周闭包的采样步长142925870。
const CIRCLE_SAMPLE_STEP: f32 = 0.5;

/// 原 B `AnimatedCircleSignifierResource`：末尾 `Option<bool>` 为 None 时不续租实体。
/// 位置和法线是编辑手柄的世界坐标，不能从所有 PublicWalls 再次猜测。
#[repr(C)]
#[derive(Resource, Clone, Copy, Debug)]
pub struct AnimatedCircleSignifierResource {
    pub widget: crate::systems::ui::resources::ui_state::WidgetId,
    pub wall: crate::resources::walls::WallId,
    pub circle: Circle2d,
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub dragging: Option<bool>,
}
impl Default for AnimatedCircleSignifierResource {
    fn default() -> Self {
        Self {
            widget: crate::systems::ui::resources::ui_state::WidgetId(0),
            wall: crate::resources::walls::WallId(0),
            circle: Circle2d::new(glam::Vec2::ZERO, 0.0),
            position: glam::Vec3::ZERO,
            normal: glam::Vec3::ZERO,
            dragging: None,
        }
    }
}

fn circle_snapshot(circle: Circle2d) -> WallShapeSnapshot {
    WallShapeSnapshot::from_observed(ShapeParameters::from_shape(Shape::Circle(circle)).to_record())
}

fn apply_circle_radius(inner: &mut InnerWalls, wall: crate::resources::walls::WallId, circle: Circle2d) -> bool {
    inner.update_wall_state_curve(wall, to_authority_curve(&circle.as_curve2(CIRCLE_SAMPLE_STEP)));
    inner.update_wall_state_enclosure(wall, circle_snapshot(circle))
}

/// B 141660880/141660b30：完整圆周的拥有型生成工厂，采样高度属于闭包载荷。
pub fn show_full_circle_signifier(
    stream: &mut UiSignifierStream,
    wall: crate::resources::walls::WallId,
    circle: Circle2d,
    height: f32,
    width: f32,
) {
    use crate::systems::ui::signifiers::signifier_factory::SignifierMesh;
    use crate::systems::ui::signifiers::{SignifierEntityBuilder, SignifierFactory};
    let factory = SignifierFactory::from_closure(
        &format!("circle{wall:?}_edit_{circle:?}{height}_{width}"),
        std::sync::Arc::new(move |meshes, shaders, commands| {
            let entity =
                SignifierEntityBuilder::new(SignifierMesh::Generated(Box::new(move || {
                    circle.as_curve3(0.5, height).points
                })))
                .animation_duration(0.0)
                .build(meshes, shaders, commands);
            commands.entity(entity).insert(
                crate::render::pass_builder::draw_list::DrawTopology::Alt(width),
            );
            vec![entity]
        }),
    );
    stream.write_factory(&factory);
}

/// 首次拖拽时记录圆和屏幕投影误差；局部状态跨帧存在，不能逐帧覆盖before。
#[derive(Default)]
pub struct CircleGrabParams {
    before: Option<Circle2d>,
    projection_offset: f32,
}
#[derive(Default)]
pub struct CircleSignifierParams {
    normal: glam::Vec2,
    height: f32,
}

fn constrained_radius(raw: f32) -> f32 {
    let minimum = f32::from_bits(0x3f66_6667);
    // MAXSS/MINSS返回右侧无序输入；保留NaN及边界的原位模式。
    let lower = if minimum > raw { minimum } else { raw };
    if 8.0 < lower { 8.0 } else { lower }
}
fn radius_change_requires_history(before: f32, after: f32) -> bool {
    // 原UCOMISS/JBE把无序比较交给还原分支，不能写成 <= 后取反。
    (before - after).abs() > CHANGE_THRESHOLD
}

/// B 14165d460的真实交互链。参数按原13组保留，帧时钟和历史均取权威owner。
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn ui_edit_circle<S: Send + Sync + 'static>(
    mode: Res<crate::systems::mode_manager::AppMode>,
    mut inner: ResMut<InnerWalls>,
    (public, entities, triggers): (
        Res<crate::resources::walls::public_walls::PublicWalls>,
        Res<crate::resources::walls::public_walls::WallEntities>,
        Query<&crate::resources::walls::wall_trigger_volume::WallTriggerVolume>,
    ),
    archival: crate::resources::walls::decorator_archivist::DecoratorArchivalResources,
    border: Res<crate::resources::glade_border::GladeBorder>,
    (ui, mut stream): (
        Res<crate::systems::ui::resources::ui_state::UiState>,
        ResMut<UiSignifierStream>,
    ),
    (mut grab, mut signifier, mut animated): (
        Local<CircleGrabParams>,
        Local<CircleSignifierParams>,
        ResMut<AnimatedCircleSignifierResource>,
    ),
    (cursor, window, camera): (
        Res<crate::resources::mouse_terrain_raycast::CursorPositionSS>,
        Res<crate::resources::window::WindowSize>,
        Res<crate::systems::camera_rig::MainCamera>,
    ),
    (mut archivist, mut storage): (
        ResMut<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
        ResMut<crate::resources::walls::decorator_storage::DecoratorStorage>,
    ),
    (time, wall_time): (Res<crate::resources::time::GameTime>, Res<bevy_time::Time>),
    mut camera_moved: EventReader<crate::systems::camera_rig::main_camera::MainCameraMoved>,
    (
        mut display,
        mut feedback,
        mut highlight,
        mut hints,
        mut sheep,
        mut audio,
        mut material,
        mut trees,
        mut pins,
    ): (
        EventWriter<crate::resources::glade_border::DisplayOutOfBorder>,
        EventWriter<crate::resources::glade_border::FeedbackInputOutOfBorder>,
        EventWriter<crate::systems::wall::wall_highlight::HighlightWallCmd>,
        EventWriter<crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,
        EventWriter<crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,
        EventWriter<crate::systems::audio::PlayBuildingAudio>,
        EventWriter<crate::systems::audio::RequestWallMaterialAudio>,
        EventWriter<crate::resources::tree_storage::HideTreesinAreaCmd>,
        EventWriter<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>,
    ),
    mut history: ResMut<crate::resources::history::History<S>>,
) {
    use crate::{
        resources::{
            glade_border::{BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder},
            tree_storage::{HideTreesinAreaCmd, TreeHideArea},
            walls::{
                WallId, decorator::DecoratorDst, decorator_storage::DecoratorStorageAddr,
                publication_fields::WallId as RecordedWallId,
                wall_trigger_volume::WallTriggerVolumeParams,
            },
        },
        systems::{
            mode_manager::{AppMode, EditSubject},
            ui::resources::widget_params::{WidgetGeometry, WidgetParams},
            wall::wall_highlight::HighlightWallCmd,
        },
        utils::project_cursor_onto_ray::{ProportionalScreenSpace, ProportionalScreenSpaceCoord},
    };
    use glam::{Vec2, Vec3};
    let camera_changed = camera_moved.read().count() != 0;
    animated.dragging = None;
    let selected = match *mode {
        AppMode::Editing(EditSubject::None) => None,
        AppMode::Editing(EditSubject::Wall(wall) | EditSubject::Roof(wall)) => Some(WallId(wall)),
        _ => return,
    };
    let mut active = None;
    let mut register =
        |wall: WallId, state: &crate::resources::walls::public_wall_state::PublicWallState| {
            let Some(circle) = state.is_circle() else {
                return false;
            };
            let height = state.max_y() - state.geometry.metrics.constrained_extra * 0.5;
            let geometry = if selected.is_some() {
                WidgetGeometry::SphereCloud {
                    centers: circle.as_curve3(0.5, height).points,
                    radius: 0.5,
                }
            } else {
                let Some(entity) = entities.wall_entity(wall) else {
                    return false;
                };
                let volume = triggers
                    .get(entity)
                    .expect("圆形墙实体缺少WallTriggerVolume");
                if volume.is_valid() {
                    WidgetGeometry::TriangleMesh(
                        volume.get_tris(WallTriggerVolumeParams::edit_shape_default()),
                    )
                } else {
                    WidgetGeometry::Empty
                }
            };
            let mut sounds = crate::systems::ui::resources::widget_params::UiSounds::empty();
            if selected.is_none() || !camera_changed {
                for (slot, kind) in sounds.slots.iter_mut().zip([1u32, 2, 3, 3]) {
                    slot[..4].copy_from_slice(&kind.to_le_bytes());
                }
            }
            // 相机事件只清四个声音tag；原flags仍保留record_history，不能误关历史。
            let params = WidgetParams::default()
                .geometry(geometry)
                .occlude_by_world(selected.is_none())
                .sounds(sounds);
            let name = if selected.is_some() {
                format!("edit_circle_wall_focused_{}", wall.0)
            } else {
                format!("edit_circle_wall_{}", wall.0)
            };
            let context = ui.add(&name, params);
            if context.is_active() {
                let point = ui.intersection_point();
                let relative = Vec2::new(point.x, point.z) - circle.center;
                let inverse = 1.0 / (relative.y * relative.y + relative.x * relative.x).sqrt();
                let normal = relative * inverse;
                let y = if selected.is_some() { height } else { point.y };
                if selected.is_none() {
                    highlight.write(HighlightWallCmd { wall, ty: 0 });
                }
                active = Some((
                    wall,
                    circle,
                    normal,
                    y,
                    context.id,
                    context.dragged(),
                    selected.is_some(),
                ));
                return true;
            } else if selected.is_some() {
                show_full_circle_signifier(&mut stream, wall, circle, height, 3.0);
            }
            false
        };
    if let Some(wall) = selected {
        register(wall, public.get(wall).expect("选中圆编辑的公共墙不存在"));
    } else {
        for (wall, state) in public.all() {
            if register(wall, state) {
                break;
            }
        }
    }
    let Some((wall, circle, normal, y, widget, phase, focused)) = active else {
        return;
    };
    let public_state = public.get(wall).expect("活动圆的公共墙必须存在");
    let project = |normal: Vec2, y: f32| {
        let screen = ProportionalScreenSpace::from_window_size(window.as_dims());
        let coord =
            ProportionalScreenSpaceCoord(screen.transform_screen_space(cursor.0.x, cursor.0.y));
        let origin = Vec3::new(circle.center.x, y, circle.center.y);
        let point = coord.project_onto_ray(
            &screen,
            &camera.read_matrix(0x80),
            &camera.read_matrix(0),
            utils::geometry::Ray {
                origin,
                direction: Vec3::new(normal.x, 0.0, normal.y),
            },
            [0.0, f32::MAX],
        );
        let delta = point - origin;
        (delta.z * delta.z + delta.y * delta.y + delta.x * delta.x).sqrt()
    };
    match phase {
        2 | 3 => {
            signifier.normal = normal;
            signifier.height = y;
            grab.before = Some(circle);
            if phase == 3 {
                grab.projection_offset = project(normal, y) - circle.radius;
                archivist.archive_current_decorators_in_storages_with_resources(
                    &[
                        DecoratorStorageAddr::Wall(wall),
                        DecoratorStorageAddr::Terrain,
                    ],
                    &storage,
                    &archival,
                    "ui_edit_circle",
                );
                archivist.decorator_cull_allow_wall(wall, &storage);
                archivist.decorator_cull_allow();
            }
        }
        4 => {
            let raw = project(signifier.normal, signifier.height) - grab.projection_offset;
            let wanted = constrained_radius(raw);
            trees.write(HideTreesinAreaCmd {
                area: TreeHideArea::Circle(circle),
                height_limit: public_state.authority.elevation(),
                expansion: 1.5,
                finished: false,
            });
            pins.write(crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter::for_circle(circle, public_state.authority.elevation(), public_state.max_y(), Some(wall)));
            if wanted != circle.radius {
                let mut next = Circle2d {
                    radius: utils::time_continuous_lerp(circle.radius, wanted, time.delta, 10.0),
                    ..circle
                };
                if !border.is_shape_inside(BorderShape::Circle(next)) {
                    display.write(DisplayOutOfBorder);
                    feedback.write(FeedbackInputOutOfBorder);
                    next.radius = match border.0 {
                        BorderShape::Circle(boundary) => {
                            let delta = boundary.center - circle.center;
                            boundary.radius - (delta.y * delta.y + delta.x * delta.x).sqrt()
                        }
                        BorderShape::Rectangle(_) => {
                            border
                                .search_lerped_shape_that_fits_into_glade(
                                    ShapeParameters::from_shape(Shape::Circle(circle)),
                                    ShapeParameters::from_shape(Shape::Circle(next)),
                                )
                                .as_circle()
                                .radius
                        }
                    };
                }
                if time.delta * 0.5 < (next.radius - circle.radius).abs() {
                    apply_circle_radius(&mut inner, wall, next);
                    let position = Vec3::new(circle.center.x, signifier.height, circle.center.y);
                    material.write(crate::systems::audio::RequestWallMaterialAudio {
                        id: wall.0,
                        position,
                        kind: 4,
                        flag: 0,
                    });
                    audio.write(crate::systems::audio::PlayBuildingAudio {
                        id: wall.0,
                        position,
                        editing_speed: utils::scalar::inverse_lerp(
                            0.0,
                            20.0,
                            (next.radius - circle.radius).abs() / wall_time.delta_secs(),
                        )
                        .clamp(0.0, 1.0),
                        kind: 0,
                    });
                }
            }
        }
        _ => {
            let before = grab.before.expect("圆编辑释放前缺少抓取快照");
            if !radius_change_requires_history(before.radius, circle.radius) {
                archivist.extract_and_restore_to_storage(&mut storage);
            } else {
                let current = inner.get(wall).expect("圆编辑完成时权威墙不存在");
                let after = ShapeParameters::from_snapshot(
                    current
                        .get_enclosure_params()
                        .expect("圆编辑完成时enclosure不存在"),
                );
                if let Some(decorators) = storage.wall_decorators_mut(wall) {
                    for (&id, info) in &mut decorators.decorators {
                        let mut destination = info.destination();
                        let wall_destination = match &mut destination {
                            DecoratorDst::Wall(dst) | DecoratorDst::WallCompound(dst) => dst,
                            DecoratorDst::Roof(_) => continue,
                            _ => panic!("圆墙存储出现非墙/屋顶装饰"),
                        };
                        let point = after.as_circle().closest_point_on_perimeter(Vec2::new(
                            wall_destination.position.x,
                            wall_destination.position.z,
                        ));
                        wall_destination.position.x = point.x;
                        wall_destination.position.z = point.y;
                        info.set_destination(destination);
                        decorators.modified_ids.insert(id);
                    }
                }
                let edit = crate::resources::history::wall_edit::WallCompoundHistoryEdit {
                    groups: vec![vec![
                        crate::resources::history::wall_edit::WallHistoryEdit::MorphPremadeShape(
                            crate::resources::history::wall_edit::MorphPremadeShapeEdit::new(
                                RecordedWallId(wall.0),
                                ShapeParameters::from_shape(Shape::Circle(before)),
                                after,
                            ),
                        ),
                    ]],
                };
                history
                    .add(crate::resources::history::EditType::Wall(
                        std::sync::Arc::new(edit),
                    ))
                    .expect("圆编辑完成时历史事务不可写");
                hints.write(crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed);
                sheep.write(
                    crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding { wall },
                );
            }
            trees.write(HideTreesinAreaCmd {
                area: TreeHideArea::Circle(circle),
                height_limit: public_state.authority.elevation(),
                expansion: 1.5,
                finished: true,
            });
        }
    }
    if focused {
        use crate::systems::ui::signifiers::{SignifierEntityBuilder, SignifierFactory};
        let expanded = Circle2d {
            radius: circle.radius + 0.5,
            ..circle
        };
        show_full_circle_signifier(&mut stream, wall, expanded, signifier.height, 5.0);
        let rotation = utils::xz_rotation_bad(signifier.normal);
        let point = signifier.normal * expanded.radius + circle.center;
        let position =
            Vec3::new(point.x, signifier.height, point.y) + rotation * Vec3::new(0.0, 0.0, 0.2);
        let factory = SignifierFactory::from_closure(
            &format!("circle{wall:?}_edit_{expanded:?}_arrow{position:?}"),
            std::sync::Arc::new(move |meshes, shaders, commands| {
                vec![
                    SignifierEntityBuilder::from_mesh_name("brush_arrow_flat_single_foward")
                        .rotation(rotation)
                        .position(position)
                        .scale(1.5)
                        .animation_duration(0.0)
                        .build(meshes, shaders, commands),
                ]
            }),
        );
        stream.write_factory(&factory);
    } else {
        animated.widget = widget;
        animated.wall = wall;
        animated.circle = circle;
        animated.position = Vec3::new(
            circle.radius * signifier.normal.x + circle.center.x,
            signifier.height,
            circle.radius * signifier.normal.y + circle.center.y,
        );
        animated.normal = Vec3::new(signifier.normal.x, 0.0, signifier.normal.y);
        animated.dragging = Some(phase != 2);
    }
    if matches!(phase, 3 | 4) {
        let offset = if focused { 0.5 } else { 0.0 };
        let point = |radius: f32| {
            let p = signifier.normal * (radius + offset) + circle.center;
            Vec3::new(p.x, signifier.height, p.y)
        };
        crate::utils::debug_viz::DebugLine::new(vec![
            point(f32::from_bits(0x3f66_6667)),
            point(8.0),
        ])
        .line_width(2.0)
        .show(&mut stream);
    }
}

#[cfg(test)]
mod interaction_tests {
    use super::*;
    use crate::{
        input::action_set::{ActionSetStates, DigitalActionPhase},
        resources::{
            cursor_terrain_raycast::CursorTerrainRaycast,
            events::{EscapeMenuJustOpened, ShutdownGameCmd},
            history::{EditType, History, MaybePendingSnapshotId, wall_edit::WallHistoryEdit},
            mouse_capture::OsCursorVisibilityCmd,
            mouse_terrain_raycast::CursorPositionSS,
            read_mouse_depth_value::MouseDepthValue,
            walls::{
                WallId, WallState,
                public_walls::{PublicWalls, WallPublicationDirty},
                publication_source::TerrainHeightGrid512,
            },
            window::{WindowFocusEvent, WindowSize},
        },
        systems::{
            brush_preview::OverwriteBrushPreview,
            camera_rig::MainCamera,
            history::{
                recording::{
                    history_finish_recording_system, history_start_recording,
                    register_recording_events,
                },
                wall_replay_interpreter::WallReplayCommand,
            },
            mode_manager::{AppMode, EditSubject},
            ui::{
                hover_intersection::hover_intersection,
                resources::{
                    ui_state::{UiState, WidgetId},
                    widget_params::PlayUiSound,
                },
                ui_update::ui_update,
            },
            wall::wall_state::{
                path_preparation::PathRasters512, wall_spatial_hash::WallSpatialHash,
            },
        },
    };
    use bevy_ecs::system::RunSystemOnce;
    use glam::{Mat4, Vec2, Vec3};
    #[test]
    fn circle_radius_limits_and_history_branch_match_original_instructions() {
        let evidence: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../reports/module-recovery-2026-10-23/m01-circle-radius-policy-native.json"
        )))
        .unwrap();
        assert_eq!(
            evidence["binary_sha256"],
            "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
        );
        assert_eq!(evidence["limit_rows"].as_array().unwrap().len(), 13);
        for row in evidence["limit_rows"].as_array().unwrap() {
            let bits = row["input"].as_u64().unwrap() as u32;
            assert_eq!(
                constrained_radius(f32::from_bits(bits)).to_bits(),
                row["result"].as_u64().unwrap() as u32
            );
        }
        assert_eq!(evidence["release_rows"].as_array().unwrap().len(), 30);
        for row in evidence["release_rows"].as_array().unwrap() {
            assert_eq!(
                radius_change_requires_history(
                    f32::from_bits(row["before"].as_u64().unwrap() as u32),
                    f32::from_bits(row["after"].as_u64().unwrap() as u32),
                ),
                row["record"].as_bool().unwrap()
            );
        }
    }
    fn circle_world() -> (
        World,
        WallId,
        Circle2d,
        TerrainHeightGrid512,
        PathRasters512,
    ) {
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let circle = Circle2d::new(Vec2::ZERO, 2.0);
        let mut authority =
            WallState::new(to_authority_curve(&circle.as_curve2(0.5)), 7.0, 0, 0, false);
        authority.snapshot.source.shape = circle_snapshot(circle);
        authority.snapshot.source.terrain_floor = 0.0;
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(authority);
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        };
        let paths = PathRasters512::from_encoded(
            std::array::from_fn(|_| vec![0u8; 512 * 512].into()),
            false,
        )
        .unwrap();
        let publish = |world: &mut World| {
            let mut inner = world.remove_resource::<InnerWalls>().unwrap();
            world.resource_mut::<PublicWalls>().consume_with_paths(
                &mut inner,
                &terrain,
                Some(&paths),
                &WallPublicationDirty::default(),
                &WallSpatialHash::default(),
            );
            world.insert_resource(inner);
        };
        let mut world = World::new();
        world.insert_resource(inner);
        world.init_resource::<PublicWalls>();
        publish(&mut world);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<AnimatedCircleSignifierResource>();
        world.init_resource::<crate::resources::walls::public_walls::WallEntities>();
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.init_resource::<ActionSetStates>();
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::ZERO,
            delta: Vec3::ZERO,
            speed: 0.0,
        });
        world.insert_resource(crate::resources::glade_border::GladeBorder(
            crate::resources::glade_border::BorderShape::Rectangle(
                utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(130.0)),
            ),
        ));
        world.init_resource::<crate::resources::time::GameTime>();
        world
            .resource_mut::<crate::resources::time::GameTime>()
            .delta = 0.1;
        let mut clock = bevy_time::Time::<()>::default();
        clock.advance_by(std::time::Duration::from_millis(100));
        world.insert_resource(clock);
        let view = Mat4::look_at_rh(Vec3::new(0.0, 10.0, 0.0), Vec3::ZERO, Vec3::Z);
        let projection = Mat4::orthographic_rh_gl(-5.0, 5.0, -5.0, 5.0, 0.1, 20.0);
        let mut camera = MainCamera::new(1.0);
        for (offset, matrix) in [
            (0, projection),
            (0x40, projection.inverse()),
            (0x80, view),
            (0xc0, view.inverse()),
        ] {
            camera.write_matrix(offset, &matrix);
        }
        world.insert_resource(camera);
        world.insert_resource(CursorPositionSS(Vec2::new(30.0, 50.0)));
        world.insert_resource(WindowSize::new(100, 100));
        world.init_resource::<MouseDepthValue>();
        world.insert_resource(History::<MaybePendingSnapshotId>::empty_glade());
        register_recording_events(&mut world);
        macro_rules! event { ($($ty:ty),* $(,)?) => { $(world.init_resource::<Events<$ty>>();)* }; }
        event!(
            WindowFocusEvent,
            EscapeMenuJustOpened,
            ShutdownGameCmd,
            OverwriteBrushPreview,
            OsCursorVisibilityCmd,
            PlayUiSound,
            crate::systems::camera_rig::main_camera::MainCameraMoved,
            crate::resources::glade_border::DisplayOutOfBorder,
            crate::resources::glade_border::FeedbackInputOutOfBorder,
            crate::systems::wall::wall_highlight::HighlightWallCmd,
            crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed,
            crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding,
            crate::systems::audio::PlayBuildingAudio,
            crate::systems::audio::RequestWallMaterialAudio,
            crate::resources::tree_storage::HideTreesinAreaCmd,
            crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter,
        );
        (world, wall, circle, terrain, paths)
    }
    fn publish_circle(world: &mut World, terrain: &TerrainHeightGrid512, paths: &PathRasters512) {
        let mut inner = world.remove_resource::<InnerWalls>().unwrap();
        world.resource_mut::<PublicWalls>().consume_with_paths(
            &mut inner,
            terrain,
            Some(paths),
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        world.insert_resource(inner);
    }
    // 对应production帧前缀mark_deco_storage_mutable；finish会在每个拖动帧重新封存。
    fn begin_edit_frame(
        mut storage: ResMut<crate::resources::walls::decorator_storage::DecoratorStorage>,
    ) {
        storage.mark_mutable();
    }
    #[test]
    fn picked_radius_drag_keeps_initial_snapshot_publishes_commits_replays_and_releases_owner() {
        let (mut world, wall, circle, terrain, paths) = circle_world();
        // 用真实业务注册SphereCloud，再经过真实picker和ui_update产生3/4/释放相位。
        world.run_system_once(ui_edit_circle::<MaybePendingSnapshotId>).unwrap();
        let widget = WidgetId::from_name(&format!("edit_circle_wall_focused_{}", wall.0));
        let owner = world.resource::<UiState>().widget(widget).unwrap();
        assert!(matches!(
            owner.geometry,
            Some(crate::systems::ui::resources::widget_params::WidgetGeometry::SphereCloud { .. })
        ));
        let weak = std::sync::Arc::downgrade(&owner);
        drop(owner);
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                begin_edit_frame,
                hover_intersection,
                ui_update,
                history_start_recording::<MaybePendingSnapshotId>,
                ui_edit_circle::<MaybePendingSnapshotId>,
                history_finish_recording_system::<MaybePendingSnapshotId>,
            )
                .chain(),
        );
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::JustPressed;
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiState>().active_widget(), widget);
        assert_eq!(world.resource::<UiState>().click_byte(), 3);
        for cursor in [20.0, 10.0] {
            world.resource_mut::<CursorPositionSS>().0.x = cursor;
            world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::Held;
            schedule.run(&mut world);
            publish_circle(&mut world, &terrain, &paths);
        }
        let after = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        );
        assert!(after.as_circle().radius > circle.radius);
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::JustReleased;
        schedule.run(&mut world);
        assert!(!world.resource::<History>().is_recording());
        let edits = &world
            .resource::<History>()
            .ledger()
            .entries
            .last()
            .unwrap()
            .edits;
        let EditType::Wall(compound) = &edits[0] else {
            panic!("圆拖拽必须写墙复合历史");
        };
        let WallHistoryEdit::MorphPremadeShape(change) = &compound.groups[0][0] else {
            panic!("圆拖拽必须保留原MorphPremadeShape");
        };
        assert_eq!(
            change.before,
            ShapeParameters::from_shape(Shape::Circle(circle))
        );
        assert_eq!(change.after, after);
        let edit = compound.groups[0][0].clone();
        let mut sources = std::collections::HashMap::from([(
            crate::resources::walls::publication_fields::WallId(wall.0),
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .snapshot()
                .clone(),
        )]);
        for undo in [true, false] {
            let mut command = WallReplayCommand {
                edit: edit.clone(),
                timed: false,
                duration: 1.0,
                elapsed: 0.0,
                undo,
                finished: false,
            };
            command.advance(&mut sources, 0.1, &terrain).unwrap();
            assert_eq!(
                ShapeParameters::from_snapshot(
                    &sources[&crate::resources::walls::publication_fields::WallId(wall.0)]
                        .source
                        .shape
                ),
                if undo { change.before } else { change.after }
            );
        }
        world.insert_resource(AppMode::Photomode);
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::Idle;
        schedule.run(&mut world);
        world.resource_mut::<UiSignifierStream>().take_entries();
        drop(schedule);
        drop(world);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn circle_rotation_real_picker_updates_roof_shared_event_history_and_return_audio() {
        use super::super::ui_rotate_circle::ui_rotate_circle;
        use crate::resources::mouse_terrain_raycast::{CameraCursorRay, camera_mouse_ray};
        use crate::resources::{events::OnCircleRotated, roofs::commands::RecalculateRoofCmd};
        let (mut world, wall, mut circle, terrain, paths) = circle_world();
        circle.rotation = 6.1;
        assert!(super::super::ui_rotate_circle::rotate_circle_wall(
            &mut world.resource_mut::<InnerWalls>(),
            wall,
            circle.rotation,
        ));
        publish_circle(&mut world, &terrain, &paths);
        world.init_resource::<CameraCursorRay>();
        world.init_resource::<crate::input::action_set::MainActionSet>();
        world.init_resource::<Events<crate::resources::window::CursorMovedScreenSpace>>();
        world.init_resource::<Events<crate::resources::window::AudioCursorMoved>>();
        world.init_resource::<Events<OnCircleRotated>>();
        world.init_resource::<Events<RecalculateRoofCmd>>();
        let roof = world.spawn_empty().id();
        world
            .resource_mut::<PublicWalls>()
            .associate_roof(wall, roof);
        world.run_system_once(ui_rotate_circle::<MaybePendingSnapshotId>).unwrap();
        let widget = WidgetId::from_name(&format!("rotate_circle_{}", wall.0));
        let owner = world.resource::<UiState>().widget(widget).unwrap();
        let crate::systems::ui::resources::widget_params::WidgetGeometry::TriangleMesh(triangles) =
            owner.geometry.as_ref().unwrap()
        else {
            panic!("原旋转手柄必须是真实三角盒");
        };
        assert_eq!(triangles.len(), 12);
        // 从实际三角形求屏幕拾取点，不预设轴向或手柄朝向。
        let center = triangles.iter().flat_map(|t| t.points).sum::<Vec3>() / 36.0;
        let clip = world.resource::<MainCamera>().read_matrix(0)
            * world.resource::<MainCamera>().read_matrix(0x80)
            * center.extend(1.0);
        world.resource_mut::<CursorPositionSS>().0 =
            Vec2::new(clip.x / clip.w * 0.5 + 0.5, 0.5 - clip.y / clip.w * 0.5) * 100.0;
        let weak = std::sync::Arc::downgrade(&owner);
        drop(owner);
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                begin_edit_frame,
                camera_mouse_ray,
                hover_intersection,
                ui_update,
                history_start_recording::<MaybePendingSnapshotId>,
                ui_rotate_circle::<MaybePendingSnapshotId>,
                history_finish_recording_system::<MaybePendingSnapshotId>,
            )
                .chain(),
        );
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::JustPressed;
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiState>().active_widget(), widget);
        assert_eq!(world.resource::<UiState>().click_byte(), 3);
        for y in [60.0, 70.0] {
            world.resource_mut::<CursorPositionSS>().0.y = y;
            world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::Held;
            schedule.run(&mut world);
            publish_circle(&mut world, &terrain, &paths);
        }
        let after = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        );
        assert_ne!(after.as_circle().rotation, circle.rotation);
        assert!(
            after.as_circle().rotation < circle.rotation,
            "真实拖动跨越2π后应回到规范角域"
        );
        let roof_events: Vec<_> = world
            .resource_mut::<Events<RecalculateRoofCmd>>()
            .drain()
            .collect();
        assert_eq!(roof_events.len(), 2);
        assert!(
            roof_events
                .iter()
                .all(|e| e.entity == roof && e.wall == wall && e.kind == 2)
        );
        let rotated: Vec<_> = world
            .resource_mut::<Events<OnCircleRotated>>()
            .drain()
            .collect();
        assert_eq!(rotated, vec![OnCircleRotated(wall); 2]);
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::JustReleased;
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<OnCircleRotated>>().len(), 1);
        assert!(!world.resource::<History>().is_recording());
        let EditType::Wall(compound) = &world
            .resource::<History>()
            .ledger()
            .entries
            .last()
            .unwrap()
            .edits[0]
        else {
            panic!("旋转历史必须是墙复合事务");
        };
        let WallHistoryEdit::MorphPremadeShape(change) = &compound.groups[0][0] else {
            panic!("旋转历史必须保留MorphPremadeShape");
        };
        assert_eq!(
            change.before,
            ShapeParameters::from_shape(Shape::Circle(circle))
        );
        assert_eq!(change.after, after);
        let edit = compound.groups[0][0].clone();
        let mut sources = std::collections::HashMap::from([(
            crate::resources::walls::publication_fields::WallId(wall.0),
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .snapshot()
                .clone(),
        )]);
        for undo in [true, false] {
            let mut command = WallReplayCommand {
                edit: edit.clone(),
                timed: false,
                duration: 1.0,
                elapsed: 0.0,
                undo,
                finished: false,
            };
            command.advance(&mut sources, 0.1, &terrain).unwrap();
            assert_eq!(
                ShapeParameters::from_snapshot(
                    &sources[&crate::resources::walls::publication_fields::WallId(wall.0)]
                        .source
                        .shape
                ),
                if undo { change.before } else { change.after }
            );
        }
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::Idle;
        for _ in 0..32 {
            schedule.run(&mut world);
        }
        let audio: Vec<_> = world
            .resource_mut::<Events<PlayUiSound>>()
            .drain()
            .collect();
        assert!(
            audio.iter().any(|e| e.kind == 34),
            "回位途中必须连续发速度音"
        );
        assert_eq!(
            audio.iter().filter(|e| e.kind == 35).count(),
            1,
            "回位终点只播放一次"
        );
        world.insert_resource(AppMode::Photomode);
        schedule.run(&mut world);
        world.resource_mut::<UiSignifierStream>().take_entries();
        drop(schedule);
        drop(world);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn circle_unchanged_drag_restores_wall_and_terrain_archives_and_culling() {
        use crate::resources::walls::{
            decorator::{DecoratorId, DecoratorInfo},
            decorator_archivist::LiveDecoratorArchivist,
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        };
        let (mut world, wall, _, _, _) = circle_world();
        let wall_info = DecoratorInfo {
            u40: 6 << 8,
            u38: 123,
            ..Default::default()
        };
        let terrain_info = DecoratorInfo {
            u20: 3 << 16,
            u40: 6 << 8,
            u38: 456,
            ..Default::default()
        };
        for (address, id, info) in [
            (DecoratorStorageAddr::Wall(wall), 10, wall_info),
            (DecoratorStorageAddr::Terrain, 11, terrain_info),
        ] {
            world
                .resource_mut::<DecoratorStorage>()
                .get_or_insert_default(address)
                .insert(DecoratorId(id), info);
            world
                .resource_mut::<DecoratorStorage>()
                .decorator_to_storage
                .insert(DecoratorId(id), address);
        }
        let count_before = world.resource::<History>().ledger().entries.len();
        world.run_system_once(ui_edit_circle::<MaybePendingSnapshotId>).unwrap();
        fn same_frame_decorator_update(
            ui: Res<UiState>,
            mode: Res<AppMode>,
            mut storage: ResMut<DecoratorStorage>,
        ) {
            if ui.click_byte() != 3 {
                return;
            }
            let AppMode::Editing(EditSubject::Wall(wall)) = *mode else {
                unreachable!()
            };
            for (address, id) in [
                (DecoratorStorageAddr::Wall(WallId(wall)), 10),
                (DecoratorStorageAddr::Terrain, 11),
            ] {
                storage
                    .get_mut(address)
                    .unwrap()
                    .get_mut(DecoratorId(id))
                    .unwrap()
                    .u38 = 999;
            }
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                begin_edit_frame,
                hover_intersection,
                ui_update,
                history_start_recording::<MaybePendingSnapshotId>,
                ui_edit_circle::<MaybePendingSnapshotId>,
                same_frame_decorator_update,
                history_finish_recording_system::<MaybePendingSnapshotId>,
            )
                .chain(),
        );
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::JustPressed;
        schedule.run(&mut world);
        let archive = world.resource::<LiveDecoratorArchivist>();
        assert_eq!(archive.walls.len(), 1);
        assert!(archive.special1.is_some());
        assert!(archive.culling.allowed_ids.contains(&10));
        assert!(archive.culling.allowed_ids.contains(&11));
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::JustReleased;
        schedule.run(&mut world);
        let storage = world.resource::<DecoratorStorage>();
        assert_eq!(
            *storage
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .get(DecoratorId(10))
                .unwrap(),
            wall_info
        );
        assert_eq!(
            *storage
                .get(DecoratorStorageAddr::Terrain)
                .unwrap()
                .get(DecoratorId(11))
                .unwrap(),
            terrain_info
        );
        let archive = world.resource::<LiveDecoratorArchivist>();
        assert!(archive.walls.is_empty());
        assert!(archive.special1.is_none());
        assert!(archive.culling.allowed_ids.is_empty());
        assert!(!archive.culling.allow_any);
        assert_eq!(
            world.resource::<History>().ledger().entries.len(),
            count_before
        );
        assert!(!world.resource::<History>().is_recording());
    }

    #[test]
    fn circle_unfocused_original_wall_triangles_feed_hover_arrow_resource_and_highlight() {
        use crate::resources::walls::{
            public_walls::WallEntities,
            wall_trigger_volume::{WallTriggerVolume, WallTriggerVolumeParams},
        };
        let (mut world, wall, circle, _, _) = circle_world();
        let mut trigger = WallTriggerVolume::default();
        trigger
            .recompute(world.resource::<PublicWalls>().get(wall).unwrap())
            .unwrap();
        let triangles = trigger.get_tris(WallTriggerVolumeParams::edit_shape_default());
        let point = triangles
            .iter()
            .map(|t| t.points.into_iter().sum::<Vec3>() / 3.0)
            .max_by(|a, b| (a.x + a.z).partial_cmp(&(b.x + b.z)).unwrap())
            .unwrap();
        let entity = world.spawn(trigger).id();
        world.resource_mut::<WallEntities>().0.insert(wall, entity);
        world.insert_resource(AppMode::Editing(EditSubject::None));
        let view = Mat4::look_at_rh(Vec3::new(8.0, 10.0, 8.0), Vec3::new(0.0, 3.0, 0.0), Vec3::Y);
        let projection = Mat4::orthographic_rh_gl(-6.0, 6.0, -6.0, 6.0, 0.1, 30.0);
        for (offset, matrix) in [
            (0, projection),
            (0x40, projection.inverse()),
            (0x80, view),
            (0xc0, view.inverse()),
        ] {
            world
                .resource_mut::<MainCamera>()
                .write_matrix(offset, &matrix);
        }
        let clip = projection * view * point.extend(1.0);
        world.resource_mut::<CursorPositionSS>().0 =
            Vec2::new(clip.x / clip.w * 0.5 + 0.5, 0.5 - clip.y / clip.w * 0.5) * 100.0;
        world.resource_mut::<MouseDepthValue>().view_depth = 1000.0;
        world.run_system_once(ui_edit_circle::<MaybePendingSnapshotId>).unwrap();
        let widget = WidgetId::from_name(&format!("edit_circle_wall_{}", wall.0));
        let params = world.resource::<UiState>().widget(widget).unwrap();
        assert!(params.occlude_by_world);
        assert!(
            matches!(&params.geometry, Some(crate::systems::ui::resources::widget_params::WidgetGeometry::TriangleMesh(v)) if *v==triangles)
        );
        drop(params);
        let mut schedule = Schedule::default();
        schedule.add_systems((hover_intersection, ui_update, ui_edit_circle::<MaybePendingSnapshotId>).chain());
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiState>().active_widget(), widget);
        let animated = world.resource::<AnimatedCircleSignifierResource>();
        assert_eq!(animated.dragging, Some(false));
        assert_eq!(animated.wall, wall);
        assert_eq!(animated.circle, circle);
        assert!((animated.normal.length() - 1.0).abs() < 1e-6);
        assert!(
            (animated.position.y - world.resource::<UiState>().intersection_point().y).abs() < 1e-5
        );
        assert_eq!(
            world
                .resource::<Events<crate::systems::wall::wall_highlight::HighlightWallCmd>>()
                .len(),
            1
        );
        world.insert_resource(AppMode::Photomode);
        world.run_system_once(ui_edit_circle::<MaybePendingSnapshotId>).unwrap();
        assert!(
            world
                .resource::<AnimatedCircleSignifierResource>()
                .dragging
                .is_none()
        );
    }
}

#[cfg(test)]
mod local_merge_tests {
    use super::*;
use bevy_ecs::prelude::*;

use glam::Vec2;

use crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding;
use crate::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder};
use crate::resources::history::History;
use crate::resources::mouse_terrain_raycast::CursorPositionSS;
use crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed;
use crate::resources::time::GameTime;
use crate::resources::tree_storage::HideTreesinAreaCmd;
use crate::resources::walls::decorator_archivist::{
    DecoratorArchivalResources, LiveDecoratorArchivist,
};
use crate::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use crate::resources::walls::public_walls::{PublicWalls, WallEntities};
use crate::resources::walls::wall_trigger_volume::WallTriggerVolume;
use crate::resources::walls::{InnerWalls, WallId, WallState};
use crate::resources::window::WindowSize;
use crate::systems::audio::{PlayBuildingAudio, RequestWallMaterialAudio};
use crate::systems::camera_rig::main_camera::{MainCamera, MainCameraMoved};
use crate::systems::mode_manager::{AppMode, EditSubject};
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter;
use crate::systems::wall::wall_highlight::HighlightWallCmd;



    #[test]
    fn real_circle_system_registers_normal_and_focused_geometry_without_injected_edit_events() {
        use crate::resources::walls::public_walls::{WallPublicationDirty, WallSpatialHash};
        use crate::resources::walls::{
            publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512,
        };
        use crate::systems::collision::world::RaycastWorld;
        use crate::systems::ui::resources::{ui_state::WidgetId, widget_params::WidgetGeometry};
        use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let circle = ::utils::geometry::Circle2d::new_rotated(Vec2::new(2.0, 3.0), 2.0, 0.4);
        let mut source = WallState::new(
            crate::systems::wall::shapes::to_authority_curve(&circle.as_curve2(0.2)),
            4.0,
            0,
            0,
            false,
        );
        source.snapshot.source.shape = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Circle(circle)).to_record(),
        );
        source.snapshot.source.curve_kind_tag = 0;
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall_from_curve(source);
        let grid = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &grid,
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        let published = public.get(wall).unwrap();
        let expected_y = published.max_y() - 0.5 * published.geometry.metrics.constrained_extra;
        let mut world = World::new();
        world.insert_resource(AppMode::Editing(EditSubject::None));
        world.insert_resource(inner);
        world.insert_resource(public);
        world.init_resource::<WallEntities>();
        world.init_resource::<RaycastWorld>();
        world.insert_resource(GladeBorder(
            crate::resources::glade_border::BorderShape::Circle(::utils::geometry::Circle2d::new(
                Vec2::ZERO,
                65.0,
            )),
        ));
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<AnimatedCircleSignifierResource>();
        world.insert_resource(CursorPositionSS(Vec2::ZERO));
        world.insert_resource(WindowSize::new(1280, 720));
        world.insert_resource(MainCamera::new(1280.0 / 720.0));
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<GameTime>();
        world.insert_resource(bevy_time::Time::<()>::default());
        world.init_resource::<Events<MainCameraMoved>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<HighlightWallCmd>>();
        world.init_resource::<Events<NotifyHintSystemShapeEditWasPerformed>>();
        world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
        world.init_resource::<Events<PlayBuildingAudio>>();
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        world.init_resource::<Events<HideTreesinAreaCmd>>();
        world.init_resource::<Events<ShowPinnedDecoClutter>>();
        world.insert_resource(History::<()>::after_observed_future_clear());
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        let mut schedule = Schedule::default();
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        schedule.add_systems(ui_edit_circle::<()>);
        for step in 0..3 {
            world.resource::<UiState>().clear_widgets();
            if step != 0 {
                *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::Wall(wall.0));
            }
            if step == 2 {
                world
                    .resource_mut::<Events<MainCameraMoved>>()
                    .send(MainCameraMoved);
            }
            schedule.run(&mut world);
            let name = if step == 0 {
                format!("edit_circle_wall_{wall}")
            } else {
                format!("edit_circle_wall_focused_{wall}")
            };
            let widget = world
                .resource::<UiState>()
                .widget(WidgetId::from_name(&name))
                .expect("circle widget must be registered from production resources");
            if step == 0 {
                assert!(
                    matches!(widget.geometry, Some(WidgetGeometry::Empty)),
                    "missing valid trigger produces Empty, not a fabricated sphere"
                );
                assert!(widget.occlude_by_world);
            } else {
                let Some(WidgetGeometry::SphereCloud { centers, radius }) = &widget.geometry else {
                    panic!("focused circle uses perimeter spheres");
                };
                assert_eq!(*radius, 0.5);
                assert!(!centers.is_empty());
                for p in centers {
                    assert_eq!(p.y, expected_y);
                    assert!((Vec2::new(p.x, p.z).distance(circle.center) - 2.5).abs() < 0.0001);
                }
                assert!(!widget.occlude_by_world);
            }
            let expected = if step == 2 { [0u32; 4] } else { [1, 2, 3, 3] };
            for (slot, kind) in widget.sounds.slots.iter().zip(expected) {
                assert_eq!(u32::from_le_bytes(slot[..4].try_into().unwrap()), kind);
            }
            assert!(world.resource::<Events<HighlightWallCmd>>().is_empty());
            assert!(world.resource::<History<()>>().pending().is_none());
        }
        let previews = world.resource_mut::<UiSignifierStream>().take_entries();
        assert_eq!(previews.len(), 2);
        let crate::systems::ui::signifiers::UiSignifierEntry::Factory(factory) = &previews[0]
        else {
            panic!("full circle factory")
        };
        assert_eq!(
            factory.name(),
            Some(
                format!(
                    "circle{wall:?}_edit_{:?}_{expected_y}3",
                    ::utils::geometry::Circle2d {
                        radius: circle.radius + 0.5,
                        ..circle
                    }
                )
                .as_str()
            )
        );
        assert_eq!(factory.marker, 0x142c824f8);
        let meshes = crate::asset_libraries::mesh_library::AssetMeshLibrary::default();
        let shaders = crate::asset_libraries::shader_library::AssetShaderLibrary::new();
        shaders.add_raster(
            crate::render::shader::RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &crate::render::test_device::TestDevice(1),
            ),
            false,
        );
        let mut spawned = World::new();
        let mut queue = bevy_ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &spawned);
        let entity = factory.wake(&meshes, &shaders, &mut commands)[0];
        drop(commands);
        queue.apply(&mut spawned);
        let animation = spawned
            .get::<crate::systems::ui::signifiers::animation::SignifierAnimation>(entity)
            .unwrap();
        assert_eq!(
            (animation.flag_8, animation.flag_9, animation.duration),
            (0, 1, 0.0)
        );
        assert_eq!(
            *spawned
                .get::<crate::render::pass_builder::draw_list::DrawTopology>(entity)
                .unwrap(),
            crate::render::pass_builder::draw_list::DrawTopology::Alt(3.0)
        );

        // Prepare the actual published wall from four supplied raster buffers,
        // then recompute its real trigger. No triangle or UiInteraction is seeded.
        let paths =
            crate::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
                std::array::from_fn(|_| std::sync::Arc::from(vec![0u8; 512 * 512])),
                true,
            )
            .unwrap();
        let mut trigger = WallTriggerVolume::default();
        {
            let mut published = world.resource_mut::<PublicWalls>();
            let state = published.get_mut(wall).unwrap();
            state.prepare_paths(&paths).unwrap();
            trigger.recompute(state).unwrap();
        }
        assert!(trigger.is_valid());
        assert!(!trigger.edit_shape_tris().is_empty());
        let entity = world.spawn(trigger).id();
        world.resource_mut::<WallEntities>().0.insert(wall, entity);
        *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::None);
        world.insert_resource(UiState::at_startup());
        let inverse_view = glam::Mat4::from_translation(glam::Vec3::new(3.0, 2.0, 10.0));
        let mut camera = MainCamera::new(1280.0 / 720.0);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &inverse_view.inverse());
        camera.write_matrix(0xc0, &inverse_view);
        world.insert_resource(camera);
        world.insert_resource(CursorPositionSS(Vec2::new(640.0, 360.0)));
        world.init_resource::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
        world.insert_resource(crate::resources::read_mouse_depth_value::MouseDepthValue {
            view_depth: 100.0,
            ..Default::default()
        });
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
        let mut input_schedule = Schedule::default();
        input_schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        input_schedule.add_systems(
            (
                crate::systems::ui::hover_intersection::hover_intersection,
                crate::systems::ui::ui_update::ui_update,
            )
                .chain_ignore_deferred(),
        );
        schedule.run(&mut world);
        let selected = WidgetId::from_name(&format!("edit_circle_wall_{wall}"));
        let widget = world.resource::<UiState>().widget(selected).unwrap();
        assert!(
            matches!(&widget.geometry, Some(WidgetGeometry::TriangleMesh(tris)) if !tris.is_empty())
        );
        input_schedule.run(&mut world);
        assert_eq!(world.resource::<UiState>().active_widget(), selected);
        assert_eq!(world.resource::<UiState>().hover_byte(), 1);
        schedule.run(&mut world);
        let animated_hover = *world.resource::<AnimatedCircleSignifierResource>();
        assert_eq!(animated_hover.dragging, Some(false));
        assert_eq!(animated_hover.circle.radius, circle.radius);
        assert_eq!(animated_hover.widget, selected);
        assert_eq!(animated_hover.wall, wall);
        world.init_resource::<crate::asset_libraries::mesh_library::AssetMeshLibrary>();
        let mut consumers = Schedule::default();
        consumers.add_systems((
            super::super::signifiers::edit_arrow::signifier_edit_circle_arrow,
            super::super::signifiers::edit_line::signifier_edit_circle_line,
        ));
        consumers.run(&mut world);
        assert_eq!(
            world
                .resource_mut::<UiSignifierStream>()
                .take_entries()
                .len(),
            2
        );
        let highlighted: Vec<_> = world
            .resource_mut::<Events<HighlightWallCmd>>()
            .drain()
            .collect();
        assert_eq!(highlighted.len(), 1);
        assert_eq!((highlighted[0].wall, highlighted[0].ty), (wall, 2));
        assert!(world.resource::<History<()>>().pending().is_none());

        // Drive the start phase through the actual input owner, then verify the
        // wall and terrain snapshots and the wall culling allow-list.
        use crate::input::action_set::DigitalActionPhase;
        use crate::resources::walls::decorator::{DecoratorId, DecoratorInfo};
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            storage
                .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
                .insert(DecoratorId(11), DecoratorInfo::default());
            let mut roof = DecoratorInfo::default();
            roof.u20 = 2 << 16;
            storage
                .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
                .insert(DecoratorId(13), roof);
            storage
                .get_or_insert_default(DecoratorStorageAddr::Terrain)
                .insert(DecoratorId(12), DecoratorInfo::default());
            storage
                .decorator_to_storage
                .insert(DecoratorId(11), DecoratorStorageAddr::Wall(wall));
            storage
                .decorator_to_storage
                .insert(DecoratorId(12), DecoratorStorageAddr::Terrain);
            storage
                .decorator_to_storage
                .insert(DecoratorId(13), DecoratorStorageAddr::Wall(wall));
            storage
                .get_mut(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .modified_ids
                .clear();
        }
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = DigitalActionPhase::JustPressed;
        input_schedule.run(&mut world);
        let mut start = Schedule::default();
        start.add_systems(crate::systems::history::recording::history_start_recording::<()>);
        start.run(&mut world);
        assert!(world.resource::<History<()>>().is_recording());
        assert_eq!(
            world
                .resource::<UiState>()
                .widget_context(selected)
                .dragged(),
            3
        );
        schedule.run(&mut world);
        assert_eq!(world.resource::<AnimatedCircleSignifierResource>().dragging, Some(true));
        let archived = world.resource::<LiveDecoratorArchivist>();
        assert!(
            archived
                .extract_before_after(DecoratorStorageAddr::Wall(wall))
                .is_some()
        );
        assert!(
            archived
                .extract_before_after(DecoratorStorageAddr::Terrain)
                .is_some()
        );
        assert!(archived.is_decorator_cull_allowed(11));
        assert!(archived.is_decorator_cull_allowed(12));
        assert!(
            world
                .resource::<DecoratorStorage>()
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .decorators()
                .contains_key(&11)
        );
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = DigitalActionPhase::Held;
        world.resource_mut::<GameTime>().delta = 0.1;
        world.resource_mut::<CursorPositionSS>().0.x = 1100.0;
        input_schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<UiState>()
                .widget_context(selected)
                .dragged(),
            4
        );
        schedule.run(&mut world);
        let edited = world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .get_enclosure_params()
            .unwrap();
        let edited_circle = ShapeParameters::from_snapshot(edited).as_circle();
        assert_ne!(edited_circle.radius.to_bits(), circle.radius.to_bits());
        assert_eq!(
            world
                .resource_mut::<Events<RequestWallMaterialAudio>>()
                .drain()
                .count(),
            1
        );
        // Publish the authoritative held update before releasing, as the real
        // frame chain does; release must record the old and new shapes.
        world.resource_scope(|world, mut public: Mut<PublicWalls>| {
            let mut inner = world.resource_mut::<InnerWalls>();
            public.consume_with_paths(
                &mut inner,
                &grid,
                None,
                &WallPublicationDirty::default(),
                &WallSpatialHash::default(),
            );
        });
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = DigitalActionPhase::JustReleased;
        input_schedule.run(&mut world);
        assert!(
            world
                .resource::<UiState>()
                .widget_context(selected)
                .dragged()
                < 2
        );
        schedule.run(&mut world);
        let pending = world.resource::<History<()>>();
        let edits = &pending.pending().unwrap().edits;
        assert!(
            matches!(&edits[0], crate::resources::history::EditType::Wall(edit) if matches!(&edit.groups[0][0], crate::resources::history::wall_edit::WallHistoryEdit::MorphPremadeShape(_)))
        );
        let info = world
            .resource::<DecoratorStorage>()
            .get(DecoratorStorageAddr::Wall(wall))
            .unwrap()
            .decorators()[&11];
        assert!(
            world
                .resource::<DecoratorStorage>()
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .modified_ids
                .contains(&13)
        );
        let pos = Vec2::new(
            f32::from_bits((info.dst_payload >> 32) as u32),
            f32::from_bits((info.u18 >> 32) as u32),
        );
        assert!((pos.distance(edited_circle.center) - edited_circle.radius).abs() < 0.0001);
        assert_eq!(
            world
                .resource_mut::<Events<CmdCheckForSheepInsideBuilding>>()
                .drain()
                .count(),
            1
        );
        assert_eq!(
            world
                .resource_mut::<Events<NotifyHintSystemShapeEditWasPerformed>>()
                .drain()
                .count(),
            1
        );
        assert!(
            world
                .resource_mut::<Events<HideTreesinAreaCmd>>()
                .drain()
                .any(|event| event.finished)
        );
        // Complete the canonical recording and replay owners, including the
        // nonempty decorator diff. An edit pending in History is not a commit.
        crate::systems::history::register_history_events(&mut world);
        world.init_resource::<crate::resources::walls::decorator::DecoratorIdGen>();
        world.init_resource::<crate::resources::walls::decorator::DeferredDecoratorOpEvents>();
        let mut finish = Schedule::default();
        finish
            .add_systems(crate::systems::history::recording::history_finish_recording_system::<()>);
        finish.run(&mut world);
        let history = world.resource::<History<()>>();
        assert!(!history.is_recording());
        assert_eq!(history.ledger().entries.len(), 1);
        assert!(
            history.ledger().entries[0]
                .auxiliary
                .decorator_diff
                .as_ref()
                .is_some_and(|diff| !diff.is_empty())
        );
        world.insert_resource(
            crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain(
                grid,
            ),
        );
        world.insert_resource(crate::resources::history::ReplayBuffer::new(1.0));
        world.resource_mut::<GameTime>().delta = 1.0;
        let mut replay = Schedule::default();
        #[cfg(feature = "native-physics")]
        replay.add_systems(
            (
                crate::systems::history::history_process_system::<()>,
                crate::systems::history::replay_wall::replay_wall,
                crate::systems::history::replay_decorator::replay_decorator,
                crate::systems::history::replay_pop_bucket_system,
            )
                .chain(),
        );
        #[cfg(not(feature = "native-physics"))]
        replay.add_systems(
            (
                crate::systems::history::history_process_system::<()>,
                crate::systems::history::replay_wall::replay_wall,
                crate::systems::history::replay_decorator::replay_decorator,
                crate::systems::history::replay_pop_bucket_cpu_system,
            )
                .chain(),
        );
        for (command, expected_circle, expected_decorator) in [
            (
                crate::systems::mode_manager::HistoryCommand::UNDO,
                circle,
                DecoratorInfo::default(),
            ),
            (
                crate::systems::mode_manager::HistoryCommand::REDO,
                edited_circle,
                info,
            ),
        ] {
            world.send_event(command);
            for _ in 0..400 {
                world.resource_mut::<DecoratorStorage>().mark_mutable();
                replay.run(&mut world);
            }
            let shape = ShapeParameters::from_snapshot(
                world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .get_enclosure_params()
                    .unwrap(),
            )
            .as_circle();
            assert_eq!(shape.center, expected_circle.center);
            assert_eq!(shape.radius.to_bits(), expected_circle.radius.to_bits());
            assert_eq!(shape.rotation.to_bits(), expected_circle.rotation.to_bits());
            let restored = world
                .resource::<DecoratorStorage>()
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .decorators()[&11];
            assert_eq!(restored, expected_decorator);
            assert!(
                world
                    .resource::<crate::resources::history::ReplayBuffer>()
                    .buckets
                    .is_empty()
            );
            assert!(
                world
                    .resource::<Events<crate::systems::history::HistoryReplayFailed>>()
                    .is_empty()
            );
        }
        // A release with an uninitialized Local is an original panic, not a no-op.
        world.resource_mut::<UiState>().replace_interaction(
            crate::systems::ui::resources::ui_state::UiInteraction {
                native_tag: 2,
                native_phase: 1,
                widget: selected,
            },
        );
        let mut fresh_schedule = Schedule::default();
        fresh_schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        fresh_schedule.add_systems(ui_edit_circle::<()>);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || fresh_schedule.run(&mut world)
            ))
            .is_err()
        );
    }

    #[test]
    fn locals_and_resource_defaults() {
        let grab = CircleGrabParams::default();
        let _ = &grab;
        let signifier = CircleSignifierParams::default();
        let _ = &signifier;
        // AnimatedCircleSignifierResource default tag = 2 (Freehand) per registration.
        assert_eq!(AnimatedCircleSignifierResource::default().dragging, None);
        assert_eq!(std::mem::size_of::<AnimatedCircleSignifierResource>(), 64);
        assert_eq!(std::mem::offset_of!(AnimatedCircleSignifierResource, widget), 0x00);
        assert_eq!(std::mem::offset_of!(AnimatedCircleSignifierResource, wall), 0x08);
        assert_eq!(std::mem::offset_of!(AnimatedCircleSignifierResource, circle), 0x10);
        assert_eq!(std::mem::offset_of!(AnimatedCircleSignifierResource, position), 0x20);
        assert_eq!(std::mem::offset_of!(AnimatedCircleSignifierResource, normal), 0x2c);
        assert_eq!(std::mem::offset_of!(AnimatedCircleSignifierResource, dragging), 0x38);
    }

    #[test]
    fn s0_numeric_helpers_clamp_and_threshold() {
        // S2 clamp bounds 0.9 / 8.0.
        assert_eq!(constrained_radius(0.1).to_bits(), 0x3f66_6667);
        assert_eq!(constrained_radius(100.0), 8.0);
        assert_eq!(constrained_radius(4.0), 4.0);
        // S5 threshold 0.01 (unordered follows the native restore branch).
        assert!(!radius_change_requires_history(2.0, 2.005));
        assert!(radius_change_requires_history(2.0, 2.02));
        assert!(!radius_change_requires_history(f32::NAN, 2.0));
    }

    #[test]
    fn circle_clamp_matches_executed_original_sse_fixtures_by_bits() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-analysis-20261028-clamp-fixture.json"
        ))).unwrap();
        assert_eq!(fixture["executed"], true);
        assert_eq!(
            fixture["original_B_sha256"],
            "cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323"
        );
        let cases = fixture["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 18);
        for case in cases {
            let input = u32::from_str_radix(case["input_bits"].as_str().unwrap(), 16).unwrap();
            let expected = u32::from_str_radix(case["output_bits"].as_str().unwrap(), 16).unwrap();
            assert_eq!(
                constrained_radius(f32::from_bits(input)).to_bits(),
                expected,
                "original SSE input {input:08x}"
            );
        }
    }

    #[test]
    fn s4_apply_circle_updates_curve_and_enclosure_radius() {
        use crate::resources::roofs::geometry::surface::Curve2;
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let wall = inner.new_wall_from_curve(WallState::new(curve, 2.0, 0, 0, false));
        let circle = ::utils::geometry::Circle2d::new_rotated(Vec2::new(1.0, 1.0), 3.0, 0.4);
        let applied = apply_circle_radius(&mut inner, wall, circle);
        assert!(applied, "apply_circle_radius should update the wall curve");
        let params = inner
            .get(wall)
            .unwrap()
            .get_enclosure_params()
            .expect("circle enclosure params");
        let radius = crate::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(params)
            .as_circle()
            .radius;
        assert!((radius - 3.0).abs() < 1e-3, "enclosure radius {radius}");
        assert_eq!(
            crate::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(params)
                .as_circle()
                .rotation,
            0.4
        );
        for radius in [f32::from_bits(0x3f66_6667), 3.0, 8.0] {
            let input = ::utils::geometry::Circle2d::new_rotated(Vec2::new(1.0, 1.0), radius, 0.4);
            assert!(apply_circle_radius(&mut inner, wall, input));
            let expected = input.as_curve2(0.5);
            let actual = &inner.get(wall).unwrap().snapshot.source.curve.points;
            assert_eq!(actual.len(), expected.points.len());
            for (a, b) in actual.iter().zip(&expected.points) {
                assert_eq!(a[0].to_bits(), b.x.to_bits());
                assert_eq!(a[1].to_bits(), b.y.to_bits());
            }
        }
    }
}
