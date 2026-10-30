//! B `ui_move_shapes.rs`：预制建筑移动及与自由墙共用的真实移动手柄。
//!
//! 原函数边界由 B PDB module 519 裁定：主系统 14097a060/12822 B，
//! 手柄 14097d280/2618 B、闭包 14097dcc0/1488 B 和 14097e290/709 B。
//! 旧反编译跨过这些边界，不能按其错误的 void 签名或邻接代码恢复调用。

use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::{Quat, Vec2, Vec3};

use crate::{
    components::DashedLineLength,
    input::action_set::ActionSetStates,
    resources::{
        critters::sheep::CmdCheckForSheepInsideBuilding,
        cursor_terrain_raycast::CursorTerrainRaycast,
        events::OnShapeMoved,
        glade_border::{BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        history::{
            EditType, History,
            wall_edit::{
                CompoundEditBuilder, MorphPremadeShapeEdit, WallHistoryEdit, WallValueEdit,
            },
        },
        mouse_terrain_raycast::CameraCursorRay,
        roofs::Roof,
        shape_intersections::ShapeIntersections,
        terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
        time::GameTime,
        tree_storage::{HideTreesinAreaCmd, TreeHideArea},
        walls::{
            InnerWalls, WallId, WallState,
            decorator_archivist::{DecoratorArchivalResources, LiveDecoratorArchivist},
            decorator_storage::DecoratorStorage,
            public_wall_state::PublicWallState,
            public_walls::PublicWalls,
        },
        window::AudioCursorMoved,
    },
    systems::{
        audio::{PlayBuildingAudio, PlayWallUiSound, RequestWallMaterialAudio},
        backend::input::mod_key::Modifiers,
        backend::input::translate_input::DemoModeEnabled,
        collision::world::RaycastWorld,
        mode_manager::{AppMode, EditSubject},
        ui::{
            resources::{
                ui_state::{UiState, UiWidgetContext},
                widget_params::{WidgetGeometry, WidgetParams},
            },
            signifiers::{SignifierEntityBuilder, SignifierFactory, UiSignifierStream},
        },
        ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter,
        ui_move_freehand_walls::{anchor_xz, archive_freehand, wall_anchor},
        wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume,
        wall::{
            shapes::shape_placement::{RoofRidgeSnap, snap_to_roof_ridge},
            valid_enclosure::{Shape, ShapeParameters},
        },
    },
    utils::{
        components::transform::Transform,
        debug_viz::{DebugLine, DebugSphere},
    },
};

/// 14097d2c4..14097d322：小建筑的手柄缩到一半，三米以上恢复全尺寸。
fn move_gizmo_scale(length: f32) -> f32 {
    let value = (length + -0.5) / 2.5;
    // MAXSS/MINSS 相等或无序时取第二操作数；保留 NaN 与负零。
    let lower = if value < 0.0 { 0.0 } else { value };
    let t = if lower > 1.0 { 1.0 } else { lower };
    let square = t * t;
    let smooth = (3.0 - (t + t)) * square;
    (1.0 - smooth) * 0.5 + smooth
}

/// B 14097d280：创建拾取盒、地面虚线、实体与描边，返回同一个真实 Widget。
///
/// `ground_y` 来自调用者的真实地形采样，不能拿零或光标的屏幕坐标替代。
/// 参数次序对应原八实参；返回值对应 RAX/RDX 的 UiWidgetContext。
pub fn move_gizmo_and_trigger_vol<'ui>(
    position: Vec3,
    orientation: Quat,
    actions: &ActionSetStates,
    ui: &'ui UiState,
    wall: WallId,
    state: &PublicWallState,
    ground_y: f32,
    stream: &mut UiSignifierStream,
) -> UiWidgetContext<'ui> {
    let scale = move_gizmo_scale(state.simplified_curve_2d().length);
    let dimension = 1.5 * scale;
    let volume = box_trigger_volume(
        dimension,
        // 原XMM2保持0.5；只有XMM1/XMM3随建筑长度缩放。
        0.5,
        dimension,
        &Transform {
            rotation: orientation,
            translation: position,
            scale: Vec3::ONE,
        },
    );
    let mut params = WidgetParams::default()
        .mesh_volume(volume)
        .occlude_by_world(false);
    // 原 ActionSetStates +0x10 为 camera_rotate_mode；旋转镜头时清掉四个 UI 声音。
    let sounds_enabled = actions.digital[2].phase as u32 >= 3;
    for (slot, tag) in params.sounds.slots.iter_mut().zip([1u32, 2, 3, 3]) {
        slot[..4].copy_from_slice(&(if sounds_enabled { tag } else { 0 }).to_le_bytes());
    }
    let context = ui.add(&format!("move_shape{wall}"), params);

    if state.authority.elevation() > 0.0 {
        let start = position;
        let end = Vec3::new(position.x, ground_y, position.z);
        stream.write_factory(&SignifierFactory::from_closure(
            &format!("move_shape_line{wall:?}{position:?}{end}{position}"),
            Arc::new(move |meshes, shaders, commands| {
                let entity = SignifierEntityBuilder::from_points(vec![end, start])
                    .animation_duration(0.0)
                    .custom_shader("dashed_line")
                    .build(meshes, shaders, commands);
                let delta = end - start;
                let length = (delta.z * delta.z + delta.y * delta.y + delta.x * delta.x).sqrt();
                commands.entity(entity).insert((
                    crate::render::pass_builder::draw_list::DrawTopology::Alt(1.0),
                    DashedLineLength(length),
                ));
                vec![entity]
            }),
        ));
    }

    // 原放大条件不读 intersection tag，与 UiWidgetContext::is_active 不等价。
    let active = ui.interaction();
    let enlarge = active.native_tag != 0
        && active.widget == context.id
        && if active.native_tag == 1 {
            active.native_phase != 0
        } else {
            active.native_phase != 2
        };
    let display_scale = scale * if enlarge { 1.2 } else { 1.0 };
    for (pass, mesh) in [(0, "move_shape"), (1, "move_shape_outline")] {
        let rotation = orientation;
        stream.write_factory(&SignifierFactory::from_closure(
            &format!("move_shape{wall:?}{position:?}{mesh}{display_scale}{rotation:?}"),
            Arc::new(move |meshes, shaders, commands| {
                vec![
                    SignifierEntityBuilder::from_mesh_name(mesh)
                        .rotation(rotation)
                        .position(position)
                        .scale(display_scale * 1.5)
                        .animation_duration(0.0)
                        .set_ui_pass(pass)
                        .build(meshes, shaders, commands),
                ]
            }),
        ));
    }
    context
}

/// 原 Local 的向量、抓取差、初始28B形状、抓取平面高度与吸附边沿。
#[derive(Default)]
pub struct MoveShapeCache {
    pub affected: Vec<(u32, f32, f32)>,
    pub cursor_offset: Vec2,
    pub before: Option<ShapeParameters>,
    pub grab_height: f32,
    pub snapped: bool,
}

fn with_center(shape: Shape, center: Vec2) -> Shape {
    match shape {
        Shape::Circle(mut circle) => {
            circle.center = center;
            Shape::Circle(circle)
        }
        Shape::Rectangle(mut rectangle) => {
            rectangle.center = center;
            Shape::Rectangle(rectangle)
        }
    }
}

fn boundary(shape: Shape) -> BorderShape {
    match shape {
        Shape::Circle(circle) => BorderShape::Circle(circle),
        Shape::Rectangle(rectangle) => BorderShape::Rectangle(rectangle),
    }
}

fn numeric_max(a: f32, b: f32) -> f32 {
    if b.is_nan() || a > b { a } else { b }
}

fn numeric_min_first(a: f32, b: f32) -> f32 {
    // 原97bf9d..bfb7：第一距离NaN时选第二，否则相等保留第一。
    if a.is_nan() || b < a { b } else { a }
}

fn smooth_position(current: Vec2, target: Vec2, delta: f32) -> Vec2 {
    // 原140c90570向量路径共享一个expf，不分别计算两个分量的因子。
    let factor = ::utils::time_continuous_lerp_factor(delta, 100.0);
    (target - current) * factor + current
}

/// 原圆形谷地的滑边算法：矩形需同时裁定面对光标的一条边的两个端点。
fn constrain_to_border(border: &GladeBorder, current: Shape, target: Shape, delta: f32) -> Shape {
    match border.0 {
        BorderShape::Rectangle(_) => border
            .search_lerped_shape_that_fits_into_glade(
                ShapeParameters::from_shape(current),
                ShapeParameters::from_shape(target),
            )
            .get_shape()
            .expect("原边界插值返回预制形状"),
        BorderShape::Circle(circle) => {
            let offset = target.center() - circle.center;
            let inverse = 1.0 / (offset.y * offset.y + offset.x * offset.x).sqrt();
            let direction = offset * inverse;
            let destination = match current {
                Shape::Circle(shape) => direction * (circle.radius - shape.radius) + circle.center,
                Shape::Rectangle(mut shape) => {
                    // 原先将矩形中心移到谷地中心，选择最近边，再对两端点沿同一单位方向求圆交。
                    shape.center = circle.center;
                    let edge = shape.get_closest_edge_id(target.center());
                    let (first, second) = shape.get_edge_pts(edge);
                    let far = |origin| {
                        circle
                            .intersect_ray(::utils::geometry::Ray2 { origin, direction })
                            .map_or(0.0, |(_, far)| far)
                    };
                    let distance = numeric_min_first(far(first), far(second));
                    direction * distance + circle.center
                }
            };
            with_center(
                target,
                smooth_position(current.center(), destination, delta),
            )
        }
    }
}

fn add_history<S: Send + Sync + 'static>(history: &mut History<S>, edit: WallHistoryEdit) {
    let mut edits = CompoundEditBuilder::default();
    edits.new_edit_sequence();
    edits.next(edit);
    history
        .add(EditType::Wall(Arc::new(edits.build())))
        .unwrap_or_else(|error| panic!("移动建筑历史写入失败: {error:?}"));
}

fn shape_curve(shape: Shape) -> crate::resources::roofs::geometry::surface::Curve2 {
    let curve = match shape {
        // 原97c237明确传0.5，不用通用Shape::as_curve2的0.2。
        Shape::Circle(circle) => circle.as_curve2(0.5),
        Shape::Rectangle(rectangle) => rectangle.as_curve2(),
    };
    crate::resources::roofs::geometry::surface::Curve2 {
        points: curve
            .points
            .into_iter()
            .map(|point| point.to_array())
            .collect(),
        points_u: curve.points_u,
        length: curve.length,
    }
}

fn moved_wall_state(previous: &WallState, shape: Shape, demo: bool) -> WallState {
    use crate::resources::{
        history::wall_edit::WallAuthoritySnapshot,
        walls::publication_source::RawWallPublicationSource,
    };
    let source = previous.source();
    let mut metadata = previous.snapshot().metadata;
    metadata.source_69 = if demo { 0 } else { 5 };
    // 原只构造新曲线并拷贝标量；不要先clone旧曲线，再立刻用新曲线覆盖并释放。
    WallState::from_snapshot(
        WallAuthoritySnapshot {
            source: RawWallPublicationSource {
                curve: shape_curve(shape),
                curve_kind_tag: shape.tag(),
                terrain_floor: source.terrain_floor,
                shape: ShapeParameters::from_shape(shape).to_snapshot(),
                style: source.style,
                extra: source.extra,
                special: source.special,
                observed_path_scalar: source.observed_path_scalar,
                clutter_version: source.clutter_version,
                observed_source_flag_64: source.observed_source_flag_64,
            },
            metadata,
        },
        previous.freehand_ty(),
    )
}

fn show_snap_line(stream: &mut UiSignifierStream, a: Vec3, b: Vec3) {
    DebugLine::new(vec![a, b])
        .line_width(3.0)
        .dashed()
        .show(stream);
}

/// 原B 14097a060：15组SystemParam，完整30个实际资源/事件合同。
/// 同组附带现宿主DemoModeEnabled，用它承接原本不占SystemParam的进程全局门禁。
pub fn ui_move_shapes<S: Send + Sync + 'static>(
    mode: Res<AppMode>,
    (public, mut inner, roofs): (Res<PublicWalls>, ResMut<InnerWalls>, Query<&Roof>),
    (ray, cursor, mut audio_cursor, collision): (
        Res<CameraCursorRay>,
        Res<CursorTerrainRaycast>,
        EventReader<AudioCursorMoved>,
        Res<RaycastWorld>,
    ),
    (mut storage, mut archivist): (ResMut<DecoratorStorage>, ResMut<LiveDecoratorArchivist>),
    (ui, mut stream): (Res<UiState>, ResMut<UiSignifierStream>),
    mut cache: Local<MoveShapeCache>,
    mut history: ResMut<History<S>>,
    actions: Res<ActionSetStates>,
    (mut material_audio, mut building_audio, mut wall_audio): (
        EventWriter<RequestWallMaterialAudio>,
        EventWriter<PlayBuildingAudio>,
        EventWriter<PlayWallUiSound>,
    ),
    (mut moved, mut sheep, mut trees, mut pins): (
        EventWriter<OnShapeMoved>,
        EventWriter<CmdCheckForSheepInsideBuilding>,
        EventWriter<HideTreesinAreaCmd>,
        EventWriter<ShowPinnedDecoClutter>,
    ),
    archival: DecoratorArchivalResources,
    (terrain, demo): (
        Res<TerrainHeightsData<TerrainHeightSsbo>>,
        Option<Res<DemoModeEnabled>>,
    ),
    (border, mut display, mut feedback, time): (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
        Res<GameTime>,
    ),
    modifiers: Res<Modifiers>,
    intersections: Res<ShapeIntersections>,
) {
    let wall = match *mode {
        AppMode::Editing(EditSubject::Wall(id) | EditSubject::Roof(id)) => WallId(id),
        _ => return,
    };
    let published = public.get(wall).expect("移动建筑的公开墙不存在");
    let parameters = ShapeParameters::from_snapshot(published.shape());
    let Some(shape) = parameters.get_shape() else {
        return;
    };
    let center = shape.center();
    let elevation = published.authority.elevation();
    let ground = terrain.sample_at_world_xz(center.x, center.y);
    let position = Vec3::new(center.x, numeric_max(elevation, ground), center.y);

    if published.authority.removed_supports() != 0 {
        // 原地面/悬空高度的较大者下方两米，单独注册恢复支撑的真实球体。
        let support_position = Vec3::new(center.x, numeric_max(elevation, ground) + -2.0, center.y);
        let widget = ui.add(
            &format!("add_support_{wall}"),
            WidgetParams::default()
                .geometry(WidgetGeometry::Sphere {
                    center: support_position,
                    radius: 0.4,
                })
                .occlude_by_world(false),
        );
        DebugSphere::new(support_position)
            .radius(if widget.is_active() { 0.4 } else { 0.3 })
            .solid()
            .show(&mut stream);
        if widget.clicked() {
            inner.update_supports(wall, 0);
            add_history(
                &mut history,
                WallHistoryEdit::ChangeSupports(WallValueEdit {
                    wall_index: wall.0,
                    before: true,
                    after: false,
                }),
            );
            wall_audio.write(PlayWallUiSound {
                kind: if published.authority.style() > 1 {
                    0x13
                } else {
                    0x11
                },
                selector: 0,
                value: 0.0,
                position: support_position,
            });
        }
    }
    let rotation = match shape {
        Shape::Circle(_) => Quat::IDENTITY,
        Shape::Rectangle(rectangle) => Quat::from_mat3(&rectangle.orientation_mat3()),
    };
    let widget = move_gizmo_and_trigger_vol(
        position,
        rotation,
        &actions,
        &ui,
        wall,
        published,
        ground,
        &mut stream,
    );
    let intersecting = intersections.0.keys().any(|pair| pair.contains(&wall));
    let mut is_inside = true;
    match widget.dragged() {
        2 => return,
        3 => {
            archive_freehand(&mut archivist, &storage, &archival, wall, "move_shape");
            cache.affected.clear();
            if let Some(decorators) = storage.wall_decorators_mut(wall) {
                for (&id, info) in decorators.decorators() {
                    if wall_anchor(info) {
                        let offset = anchor_xz(info) - center;
                        cache.affected.push((id, offset.x, offset.y));
                    }
                }
            }
            cache.before = Some(parameters);
            cache.grab_height = position.y;
            let factor = (position.y - ray.origin.y) / ray.dir.y;
            cache.cursor_offset = Vec2::new(
                factor * ray.dir.x + ray.origin.x,
                factor * ray.dir.z + ray.origin.z,
            ) - center;
        }
        4 => {
            let authority = inner.get(wall).expect("移动建筑的权威墙不存在");
            let cursor = if elevation != 0.0 {
                let factor = (cache.grab_height - ray.origin.y) / ray.dir.y;
                Vec2::new(
                    factor * ray.dir.x + ray.origin.x,
                    factor * ray.dir.z + ray.origin.z,
                )
            } else {
                let point = cursor.position();
                Vec2::new(point.x, point.z)
            };
            let mut destination = cursor - cache.cursor_offset;
            let mut snapped = false;
            if !modifiers.disable_snapping() {
                if let Some(roof) = public
                    .roof_entity(wall)
                    .and_then(|entity| roofs.get(entity).ok())
                {
                    let mut projected_height = terrain
                        .sample_at_world_xz(destination.x, destination.y)
                        + authority.elevation();
                    if roof.height_01() > 0.1 {
                        projected_height += roof.height();
                    }
                    let hit = collision.raycast_simple_excluding_wall_id(
                        Vec3::new(destination.x, -0.5 + projected_height, destination.y),
                        Vec3::new(-0.0, -1.0, -0.0),
                        0x202,
                        wall,
                    );
                    if matches!(hit.handle[0] as u8, 1 | 2) {
                        let target_wall = WallId(hit.handle[1]);
                        if let (Some(target), Some(roof)) = (
                            public.get(target_wall),
                            public
                                .roof_entity(target_wall)
                                .and_then(|entity| roofs.get(entity).ok()),
                        ) {
                            let top = roof.height() + roof.floor_y();
                            if target.authority.elevation() <= projected_height
                                && authority.elevation() <= top
                            {
                                let point = Vec3::new(hit.position.x, top, hit.position.z);
                                let snap = snap_to_roof_ridge(roof, point, 1.0);
                                let snap_point = match snap {
                                    RoofRidgeSnap::None => None,
                                    RoofRidgeSnap::Point(point) => Some(point),
                                    RoofRidgeSnap::Segment { a, b, t } => {
                                        show_snap_line(
                                            &mut stream,
                                            a + Vec3::new(0., 0.2, 0.),
                                            b + Vec3::new(0., 0.2, 0.),
                                        );
                                        Some(b * t + a * (1.0 - t))
                                    }
                                };
                                if let Some(point) = snap_point {
                                    destination = Vec2::new(point.x, point.z);
                                    show_snap_line(
                                        &mut stream,
                                        Vec3::new(point.x, numeric_max(ground, point.y), point.z),
                                        Vec3::new(point.x, position.y, point.z),
                                    );
                                    if !cache.snapped {
                                        wall_audio.write(PlayWallUiSound {
                                            kind: 3,
                                            selector: 0,
                                            value: 0.0,
                                            position,
                                        });
                                    }
                                    snapped = true;
                                }
                            }
                        }
                    }
                }
            }
            cache.snapped = snapped;
            let proposed = with_center(shape, destination);
            pins.write(ShowPinnedDecoClutter::for_shape(
                ShapeParameters::from_shape(proposed),
                elevation,
                published.max_y(),
                wall,
            ));
            let tree_area = |shape| match shape {
                Shape::Circle(circle) => TreeHideArea::Circle(circle),
                Shape::Rectangle(rectangle) => TreeHideArea::Rectangle(rectangle),
            };
            trees.write(HideTreesinAreaCmd {
                area: tree_area(proposed),
                height_limit: elevation,
                expansion: 1.5,
                finished: false,
            });
            is_inside = border.is_shape_inside(boundary(proposed));
            let next = if is_inside {
                proposed
            } else {
                display.write(DisplayOutOfBorder);
                feedback.write(FeedbackInputOutOfBorder);
                constrain_to_border(&border, shape, proposed, time.delta)
            };
            moved.write(OnShapeMoved {
                offsets: cache.affected.clone(),
                wall,
                center: next.center(),
            });
            // 原97c259..265及97c2ea读取进程全局；复用现输入/边界共用门禁。
            // 无宿主配置时是原B发布入口的false，与setup_glade_border相同。
            let next =
                moved_wall_state(authority, next, demo.as_deref().is_some_and(|demo| demo.0));
            // 原97b70b重新读权威height覆盖入口terrain，完整保存建筑高度及其他字段。
            inner.update_wall_state(wall, next);
        }
        _ => {
            moved.write(OnShapeMoved {
                offsets: cache.affected.clone(),
                wall,
                center,
            });
            sheep.write(CmdCheckForSheepInsideBuilding { wall });
            if let Some(before) = cache.before.take() {
                add_history(
                    &mut history,
                    WallHistoryEdit::MorphPremadeShape(MorphPremadeShapeEdit::new(
                        crate::resources::walls::publication_fields::WallId(wall.0),
                        before,
                        parameters,
                    )),
                );
            }
            trees.write(HideTreesinAreaCmd {
                area: match shape {
                    Shape::Circle(circle) => TreeHideArea::Circle(circle),
                    Shape::Rectangle(rectangle) => TreeHideArea::Rectangle(rectangle),
                },
                height_limit: elevation,
                expansion: 1.5,
                finished: true,
            });
        }
    }
    if !is_inside {
        return;
    }
    if audio_cursor.read().last().is_some() {
        material_audio.write(RequestWallMaterialAudio {
            id: wall.0,
            position,
            kind: 4,
            flag: 0,
        });
        let ratio = cursor.speed / 250.0;
        let ratio = if ratio < 0.0 { 0.0 } else { ratio };
        let speed = if ratio > 1.0 { 1.0 } else { ratio };
        building_audio.write(PlayBuildingAudio {
            id: wall.0,
            position,
            editing_speed: speed,
            kind: 4,
        });
        if intersecting && widget.dragged() == 4 {
            building_audio.write(PlayBuildingAudio {
                id: wall.0,
                position,
                editing_speed: speed,
                kind: 5,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gizmo_scale_matches_original_sse_bits() {
        // 原 RNE 14097d2c4..326 实际执行的146个输出，不由本实现生成。
        let cases: &[(u32, u32)] = &[
            (0x00000000, 0x3f000000),
            (0x80000000, 0x3f000000),
            (0x3f000000, 0x3f000000),
            (0x3f000001, 0x3f000000),
            (0x3effffff, 0x3f000000),
            (0x40400000, 0x3f800000),
            (0x403fffff, 0x3f800000),
            (0x40400001, 0x3f800000),
            (0x3f800000, 0x3f0d4fe0),
            (0x40000000, 0x3f52f1aa),
            (0x7f800000, 0x3f800000),
            (0xff800000, 0x3f000000),
            (0x7fc12345, 0x7fc12345),
            (0xffc12345, 0xffc12345),
            (0x7f812345, 0x7fc12345),
            (0xff812345, 0xffc12345),
            (0x7f7fffff, 0x3f800000),
            (0xff7fffff, 0x3f000000),
            (0x71d0708a, 0x3f800000),
            (0xcf2ea453, 0x3f000000),
            (0xd9696e0e, 0x3f000000),
            (0x8f87460c, 0x3f000000),
            (0x722be155, 0x3f800000),
            (0x45871ede, 0x3f800000),
            (0x5fac20bd, 0x3f800000),
            (0xa7109eb0, 0x3f000000),
            (0xc73e6bbc, 0x3f000000),
            (0x6a01cc81, 0x3f800000),
            (0x0f2e9787, 0x3f000000),
            (0xe184521d, 0x3f000000),
            (0xcf8ee744, 0x3f000000),
            (0x68eedf9a, 0x3f800000),
            (0x8b8ab6c8, 0x3f000000),
            (0xd967d6d8, 0x3f000000),
            (0x9de2215c, 0x3f000000),
            (0xd312c6dc, 0x3f000000),
            (0xa14e0727, 0x3f000000),
            (0x42ffc8cf, 0x3f800000),
            (0x39c5b709, 0x3f000000),
            (0xf6541954, 0x3f000000),
            (0x4036a401, 0x3f7ebcbe),
            (0x2d108cc6, 0x3f000000),
            (0x5097584d, 0x3f800000),
            (0x8eb5eb16, 0x3f000000),
            (0xb67e6f20, 0x3f000000),
            (0x898f4506, 0x3f000000),
            (0x6dd9747c, 0x3f800000),
            (0xcb96f3cd, 0x3f000000),
            (0xb880aa7b, 0x3f000000),
            (0x68b4b173, 0x3f800000),
            (0x1138a737, 0x3f000000),
            (0x389063e2, 0x3f000000),
            (0x25236f62, 0x3f000000),
            (0xa5afc978, 0x3f000000),
            (0x52dc4464, 0x3f800000),
            (0x1f3e5407, 0x3f000000),
            (0x7d620a4a, 0x3f800000),
            (0x6e722360, 0x3f800000),
            (0x2239c44e, 0x3f000000),
            (0xe730cbfc, 0x3f000000),
            (0x46878d93, 0x3f800000),
            (0x3e7181a0, 0x3f000000),
            (0xb13c6b9d, 0x3f000000),
            (0x75b82b84, 0x3f800000),
            (0x1d303249, 0x3f000000),
            (0x6cc02282, 0x3f800000),
            (0x2cde579c, 0x3f000000),
            (0x9bd45682, 0x3f000000),
            (0x4c25262c, 0x3f800000),
            (0xd7f6d76a, 0x3f000000),
            (0xf1855f22, 0x3f000000),
            (0x6c821f23, 0x3f800000),
            (0x1e4f9394, 0x3f000000),
            (0x6446b1bd, 0x3f800000),
            (0x976ef76a, 0x3f000000),
            (0xa84f58c6, 0x3f000000),
            (0x8c0af120, 0x3f000000),
            (0x8499e6ec, 0x3f000000),
            (0x9f647b02, 0x3f000000),
            (0x21581837, 0x3f000000),
            (0x53a985e0, 0x3f800000),
            (0xa543db15, 0x3f000000),
            (0x1e49ea48, 0x3f000000),
            (0xf23e1798, 0x3f000000),
            (0x916e96f9, 0x3f000000),
            (0xe86881a3, 0x3f000000),
            (0xda875d69, 0x3f000000),
            (0x42ecc77c, 0x3f800000),
            (0xe3b7f631, 0x3f000000),
            (0xc8006b1c, 0x3f000000),
            (0x92c0ffcb, 0x3f000000),
            (0x7edfd265, 0x3f800000),
            (0xb61379f5, 0x3f000000),
            (0x502b5725, 0x3f800000),
            (0x0f4a8bee, 0x3f000000),
            (0x9ce3190a, 0x3f000000),
            (0x24786efa, 0x3f000000),
            (0xb3780cab, 0x3f000000),
            (0x9e383b55, 0x3f000000),
            (0xb69db5d4, 0x3f000000),
            (0xef340edc, 0x3f000000),
            (0x63ce80d3, 0x3f800000),
            (0x361fe77b, 0x3f000000),
            (0x871373d4, 0x3f000000),
            (0x1e6ce84a, 0x3f000000),
            (0xafa81e7a, 0x3f000000),
            (0xa71928ab, 0x3f000000),
            (0x406541cb, 0x3f800000),
            (0xdff83a05, 0x3f000000),
            (0xfc425101, 0x3f000000),
            (0x6337f566, 0x3f800000),
            (0x8fa4bfad, 0x3f000000),
            (0xbe5df5b6, 0x3f000000),
            (0x2cf87cce, 0x3f000000),
            (0x116b16f8, 0x3f000000),
            (0x99055f47, 0x3f000000),
            (0xa041ccb9, 0x3f000000),
            (0xb6122fe7, 0x3f000000),
            (0x21518135, 0x3f000000),
            (0x47c1532f, 0x3f800000),
            (0x984680d5, 0x3f000000),
            (0x9311e49b, 0x3f000000),
            (0x3ddab18c, 0x3f000000),
            (0xcfd8f999, 0x3f000000),
            (0x15c88482, 0x3f000000),
            (0x4f6b5298, 0x3f800000),
            (0x8da4f781, 0x3f000000),
            (0x8060223d, 0x3f000000),
            (0x0b9e80be, 0x3f000000),
            (0x4166461c, 0x3f800000),
            (0x2f85bfd1, 0x3f000000),
            (0xb8f547b3, 0x3f000000),
            (0xf764a09d, 0x3f000000),
            (0x8042bd26, 0x3f000000),
            (0x88cfc386, 0x3f000000),
            (0xce0a1183, 0x3f000000),
            (0x6c458109, 0x3f800000),
            (0x9613da81, 0x3f000000),
            (0x6672b7ac, 0x3f800000),
            (0xe2b8e1ab, 0x3f000000),
            (0xfb012d2e, 0x3f000000),
            (0x1d5647a6, 0x3f000000),
            (0x5679d2f6, 0x3f800000),
            (0x8bee5ec1, 0x3f000000),
            (0x2b32f15a, 0x3f000000),
            (0xe9d95d6f, 0x3f000000),
        ];
        for &(length, expected) in cases {
            assert_eq!(
                move_gizmo_scale(f32::from_bits(length)).to_bits(),
                expected,
                "length={length:08x}"
            );
        }
    }

    #[test]
    fn picked_gizmo_moves_authority_commits_history_and_releases_capture() {
        use crate::{
            input::action_set::DigitalActionPhase,
            resources::{
                events::{EscapeMenuJustOpened, ShutdownGameCmd},
                history::MaybePendingSnapshotId,
                mouse_capture::OsCursorVisibilityCmd,
                mouse_terrain_raycast::CursorPositionSS,
                read_mouse_depth_value::MouseDepthValue,
                walls::{
                    public_walls::WallPublicationDirty, publication_source::TerrainHeightGrid512,
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
                ui::{
                    hover_intersection::hover_intersection,
                    resources::{ui_state::WidgetId, widget_params::PlayUiSound},
                    ui_update::ui_update,
                },
                wall::wall_state::{
                    path_preparation::PathRasters512, wall_spatial_hash::WallSpatialHash,
                },
            },
        };
        use bevy_ecs::system::RunSystemOnce;
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        for shape in [
            Shape::Circle(::utils::geometry::Circle2d::new(Vec2::ZERO, 2.0)),
            Shape::Rectangle(::utils::geometry::Rectangle2d::new_centered_axis_aligned(
                Vec2::new(4.0, 3.0),
            )),
        ] {
            let mut authority = WallState::new(shape_curve(shape), 7.125, 0, 0, false);
            authority.snapshot.source.shape = ShapeParameters::from_shape(shape).to_snapshot();
            authority.snapshot.source.terrain_floor = 0.0;
            let mut inner = InnerWalls::new();
            let wall = inner.new_wall(authority);
            let terrain_grid = TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(),
                minimum_y: 0.,
                maximum_y: 0.,
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
                    &terrain_grid,
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
            world.insert_resource(CameraCursorRay {
                origin: Vec3::new(0., 10., 0.),
                dir: -Vec3::Y,
            });
            world.insert_resource(CursorTerrainRaycast {
                tag: 1,
                pos: Vec3::ZERO,
                delta: Vec3::ZERO,
                speed: 125.,
            });
            world.init_resource::<RaycastWorld>();
            #[cfg(feature = "native-physics")]
            world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
            world.init_resource::<ActionSetStates>();
            world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
            world.insert_resource(GladeBorder(BorderShape::Rectangle(
                ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(130.)),
            )));
            world.init_resource::<Modifiers>();
            world.insert_resource(DemoModeEnabled(shape.tag() == 1));
            world.init_resource::<ShapeIntersections>();
            world.init_resource::<GameTime>();
            let mut camera = MainCamera::new(1.0);
            for offset in [0x40, 0x80, 0xc0] {
                camera.write_matrix(offset, &glam::Mat4::IDENTITY);
            }
            world.insert_resource(camera);
            world.insert_resource(CursorPositionSS(Vec2::splat(50.)));
            world.insert_resource(WindowSize::new(100, 100));
            world.init_resource::<MouseDepthValue>();
            world.insert_resource(History::<MaybePendingSnapshotId>::empty_glade());
            register_recording_events(&mut world);
            macro_rules! event { ($($ty:ty),* $(,)?) => { $(world.init_resource::<Events<$ty>>();)* }; }
            event!(
                AudioCursorMoved,
                RequestWallMaterialAudio,
                PlayBuildingAudio,
                PlayWallUiSound,
                OnShapeMoved,
                CmdCheckForSheepInsideBuilding,
                HideTreesinAreaCmd,
                ShowPinnedDecoClutter,
                DisplayOutOfBorder,
                FeedbackInputOutOfBorder,
                WindowFocusEvent,
                EscapeMenuJustOpened,
                ShutdownGameCmd,
                OverwriteBrushPreview,
                OsCursorVisibilityCmd,
                PlayUiSound
            );
            // 先让真正业务producer注册拾取盒；不手造UiInteraction或编辑事件。
            world
                .run_system_once(ui_move_shapes::<MaybePendingSnapshotId>)
                .unwrap();
            let widget = WidgetId::from_name(&format!("move_shape{wall}"));
            let params = world.resource::<UiState>().widget(widget).unwrap();
            assert!(
                matches!(params.geometry, Some(WidgetGeometry::TriangleMesh(ref triangles)) if triangles.len()==12)
            );
            let Some(WidgetGeometry::TriangleMesh(triangles)) = &params.geometry else {
                unreachable!()
            };
            for point in triangles.iter().flat_map(|triangle| triangle.points) {
                assert_eq!(
                    point.y.abs(),
                    0.25,
                    "原手柄拾取盒固定高度0.5，不能误跟随宽深缩放"
                );
                assert_eq!(point.x.abs(), 0.75);
                assert_eq!(point.z.abs(), 0.75);
            }
            let weak = Arc::downgrade(&params);
            drop(params);
            let mut schedule = Schedule::default();
            schedule.add_systems(
                (
                    hover_intersection,
                    ui_update,
                    history_start_recording::<MaybePendingSnapshotId>,
                    ui_move_shapes::<MaybePendingSnapshotId>,
                    history_finish_recording_system::<MaybePendingSnapshotId>,
                )
                    .chain(),
            );
            world.resource_mut::<ActionSetStates>().digital[0].phase =
                DigitalActionPhase::JustPressed;
            schedule.run(&mut world);
            assert_eq!(world.resource::<UiState>().interaction().widget, widget);
            assert_eq!(world.resource::<UiState>().click_byte(), 3);
            assert!(world.resource::<History>().is_recording());
            assert!(
                world
                    .resource::<LiveDecoratorArchivist>()
                    .special1
                    .is_some()
            );
            world.resource_mut::<CursorTerrainRaycast>().pos = Vec3::new(3., 0., 4.);
            world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::Held;
            schedule.run(&mut world);
            let moved_shape = ShapeParameters::from_snapshot(
                &world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .source()
                    .shape,
            );
            assert_eq!(moved_shape.center(), Vec2::new(3., 4.));
            assert_eq!(
                world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .height()
                    .to_bits(),
                7.125f32.to_bits()
            );
            assert_eq!(
                world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .snapshot()
                    .metadata
                    .source_69,
                if shape.tag() == 1 { 0 } else { 5 }
            );
            publish(&mut world);
            world.resource_mut::<ActionSetStates>().digital[0].phase =
                DigitalActionPhase::JustReleased;
            schedule.run(&mut world);
            assert!(!world.resource::<History>().is_recording());
            assert!(
                world
                    .resource::<LiveDecoratorArchivist>()
                    .special1
                    .is_none()
            );
            let edits = &world
                .resource::<History>()
                .ledger()
                .entries
                .last()
                .unwrap()
                .edits;
            assert_eq!(edits.len(), 1);
            let EditType::Wall(compound) = &edits[0] else {
                panic!("必须写真实墙复合历史")
            };
            let WallHistoryEdit::MorphPremadeShape(change) = &compound.groups[0][0] else {
                panic!("必须保存原预制形状变换")
            };
            assert_eq!(change.before, ShapeParameters::from_shape(shape));
            assert_eq!(change.after, moved_shape);
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
                    duration: 1.,
                    elapsed: 0.,
                    undo,
                    finished: false,
                };
                command.advance(&mut sources, 0.1, &terrain_grid).unwrap();
                let restored = ShapeParameters::from_snapshot(
                    &sources[&crate::resources::walls::publication_fields::WallId(wall.0)]
                        .source
                        .shape,
                );
                assert_eq!(restored, if undo { change.before } else { change.after });
            }
            assert_eq!(
                world
                    .resource::<Events<CmdCheckForSheepInsideBuilding>>()
                    .len(),
                1
            );
            let tree_events = world
                .resource_mut::<Events<HideTreesinAreaCmd>>()
                .drain()
                .collect::<Vec<_>>();
            assert_eq!(tree_events.len(), 2);
            assert!(!tree_events[0].finished && tree_events[1].finished);
            assert_eq!(world.resource::<Events<OnShapeMoved>>().len(), 2);
            // Drive Undo and Redo through the production history consumer and replay systems.
            world.insert_resource(
                crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain(
                    terrain_grid.clone(),
                ),
            );
            world.init_resource::<crate::systems::history::PendingHistoryCommands>();
            world.insert_resource(crate::resources::history::ReplayBuffer::new(1.0));
            world.init_resource::<Events<crate::systems::mode_manager::HistoryCommand>>();
            world
                .init_resource::<Events<crate::systems::history::history_process::DebugHistoryMsg>>(
                );
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
            // Timed Morph replay advances using the real GameTime delta. The
            // default clock is stopped, so supply a frame delta as the host does.
            world.resource_mut::<GameTime>().delta = 0.05;
            let mut replay = Schedule::default();
            replay.add_systems(
                (
                    crate::systems::history::history_process_system::<MaybePendingSnapshotId>,
                    crate::systems::history::replay_wall::replay_wall,
                    crate::systems::history::pop_empty_replay_bucket,
                )
                    .chain(),
            );
            world.send_event(crate::systems::mode_manager::HistoryCommand::UNDO);
            for _ in 0..400 {
                replay.run(&mut world);
            }
            assert!(world.resource::<crate::resources::history::ReplayBuffer>().is_empty());
            assert!(world.resource::<Events<crate::systems::history::HistoryReplayFailed>>().is_empty());
            let restored = world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap();
            assert!(
                (ShapeParameters::from_snapshot(restored).center() - shape.center()).length()
                    < 0.001
            );
            world.send_event(crate::systems::mode_manager::HistoryCommand::REDO);
            for _ in 0..400 {
                replay.run(&mut world);
            }
            assert!(world.resource::<crate::resources::history::ReplayBuffer>().is_empty());
            assert!(world.resource::<Events<crate::systems::history::HistoryReplayFailed>>().is_empty());
            let redone = world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap();
            assert!(
                (ShapeParameters::from_snapshot(redone).center() - moved_shape.center()).length()
                    < 0.001
            );
            // 在同一真实输入链点击恢复支撑；相机实际移到原球体所在的-2高度。
            world.resource_mut::<InnerWalls>().update_supports(wall, 1);
            publish(&mut world);
            world
                .resource_mut::<MainCamera>()
                .write_matrix(0xc0, &glam::Mat4::from_translation(Vec3::new(3., -2., 4.)));
            for phase in [
                DigitalActionPhase::Idle,
                DigitalActionPhase::JustPressed,
                DigitalActionPhase::JustReleased,
            ] {
                world.resource_mut::<ActionSetStates>().digital[0].phase = phase;
                schedule.run(&mut world);
            }
            assert_eq!(
                world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .removed_supports(),
                0
            );
            assert!(!world.resource::<History>().is_recording());
            let EditType::Wall(support) = &world
                .resource::<History>()
                .ledger()
                .entries
                .last()
                .unwrap()
                .edits[0]
            else {
                panic!("恢复支撑必须写真实墙历史")
            };
            assert!(
                matches!(&support.groups[0][0],WallHistoryEdit::ChangeSupports(change) if change.wall_index==wall.0 && change.before && !change.after)
            );
            let support_sounds: Vec<_> = world.resource_mut::<Events<PlayWallUiSound>>().drain().collect();
            assert_eq!(support_sounds.len(), 1);
            assert_eq!(support_sounds[0].kind, 0x11);
            #[cfg(feature = "native-physics")]
            if matches!(shape, Shape::Circle(_)) {
                let starting_shape = with_center(shape, Vec2::new(4.0, 0.0));
                let previous = world.resource::<InnerWalls>().get(wall).unwrap().clone();
                let starting = moved_wall_state(&previous, starting_shape, false);
                world
                    .resource_mut::<InnerWalls>()
                    .update_wall_state(wall, starting);
                publish(&mut world);
                use crate::systems::collision::world::{
                    ColliderMeshInstance, RaycastGeoSource, RaycastMeshData, upsert_collider,
                };
                use std::{collections::HashMap, sync::Arc};
                let other_circle = utils::geometry::Circle2d::new(glam::Vec2::new(6.5, 0.0), 2.0);
                let mut other = WallState::new(
                    crate::systems::wall::shapes::to_authority_curve(&other_circle.as_curve2(0.5)),
                    2.0,
                    0,
                    0,
                    false,
                );
                other.snapshot.source.shape =
                    ShapeParameters::from_shape(Shape::Circle(other_circle)).to_snapshot();
                other.snapshot.source.curve_kind_tag = 0;
                let other_wall = world
                    .resource_mut::<InnerWalls>()
                    .new_wall_from_curve(other);
                world.resource_scope(|world, mut public: Mut<PublicWalls>| {
                    let mut inner = world.resource_mut::<InnerWalls>();
                    public.consume_with_paths(
                        &mut inner,
                        &terrain_grid,
                        None,
                        &WallPublicationDirty::default(),
                        &WallSpatialHash::default(),
                    );
                });
                let mut roof_bytes = [0u8; 88];
                roof_bytes[0x4c..0x50].copy_from_slice(&5.0f32.to_le_bytes());
                roof_bytes[0x0c..0x10].copy_from_slice(&6.5f32.to_le_bytes());
                let other_roof = world.spawn(Roof::from_observed(roof_bytes)).id();
                // The downward snap ray starts at the selected roof's height
                // minus 0.5. A flat zero-height roof starts below the target
                // collider and cannot hit it; make the source roof pitched.
                let mut selected_roof_bytes = [0; 88];
                selected_roof_bytes[0x14..0x18].copy_from_slice(&2.0f32.to_le_bytes());
                selected_roof_bytes[0x30..0x34].copy_from_slice(&1.0f32.to_le_bytes());
                let selected_roof = world.spawn(Roof::from_observed(selected_roof_bytes)).id();
                assert!(world.get::<Roof>(selected_roof).unwrap().height() - 0.5 > 6.0);
                world
                    .resource_mut::<PublicWalls>()
                    .associate_roof(wall, selected_roof);
                world
                    .resource_mut::<PublicWalls>()
                    .associate_roof(other_wall, other_roof);
                world
                    .resource_mut::<TerrainHeightsData<TerrainHeightSsbo>>()
                    .set_height_range(0.0, 0.0);
                let mut instances: HashMap<u32, ColliderMeshInstance> = HashMap::new();
                {
                    let mut collision = world.resource_mut::<RaycastWorld>();
                    for (key, id, y) in [(1, wall, 10.0), (2, other_wall, 5.0)] {
                        let mut source = RaycastGeoSource::Slot1;
                        source.0[8..16].copy_from_slice(&id.0.to_le_bytes());
                        upsert_collider(
                            &mut collision,
                            &mut instances,
                            key,
                            source,
                            Arc::new(RaycastMeshData::new_cuboid_at(
                                glam::Vec3::splat(2.0),
                                glam::Vec3::ZERO,
                            )),
                            glam::Vec3::new(6.0, y, 0.0),
                            glam::Quat::IDENTITY,
                        );
                    }
                    collision.flush_colliders();
                }
                *world.resource_mut::<GladeBorder>() =
                    GladeBorder(crate::resources::glade_border::BorderShape::Circle(
                        utils::geometry::Circle2d::new(glam::Vec2::ZERO, 65.0),
                    ));
                world.insert_resource(UiState::at_startup());
                // The picker builds its own camera ray, independently of
                // CameraCursorRay. Aim that ray through the ground-level
                // gizmo (half-height 0.25), not one metre above it.
                let inverse = glam::Mat4::from_translation(glam::Vec3::new(4.0, 0.0, 10.0));
                let mut camera =
                    crate::systems::camera_rig::main_camera::MainCamera::new(1280.0 / 720.0);
                camera.write_matrix(0, &glam::Mat4::IDENTITY);
                camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
                camera.write_matrix(0x80, &inverse.inverse());
                camera.write_matrix(0xc0, &inverse);
                world.insert_resource(camera);
                world.resource_mut::<ActionSetStates>().digital[0].phase =
                    crate::input::action_set::DigitalActionPhase::Idle;
                world.resource_mut::<CameraCursorRay>().origin = glam::Vec3::new(4.0, 10.0, 0.0);
                world.resource_mut::<CursorTerrainRaycast>().pos = glam::Vec3::new(4.0, 0.0, 0.0);
                schedule.run(&mut world);
                schedule.run(&mut world);
                world.resource_mut::<ActionSetStates>().digital[0].phase =
                    crate::input::action_set::DigitalActionPhase::JustPressed;
                schedule.run(&mut world);
                assert_eq!(world.resource::<UiState>().interaction().widget, widget);
                assert_eq!(world.resource::<UiState>().click_byte(), 3);
                world.resource_mut::<ActionSetStates>().digital[0].phase =
                    crate::input::action_set::DigitalActionPhase::Held;
                world.resource_mut::<CameraCursorRay>().origin = glam::Vec3::new(6.0, 10.0, 0.0);
                world.resource_mut::<CursorTerrainRaycast>().pos = glam::Vec3::new(6.0, 0.0, 0.0);
                schedule.run(&mut world);
                let snapped = world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .get_enclosure_params()
                    .unwrap();
                assert_eq!(
                    ShapeParameters::from_snapshot(snapped).center(),
                    glam::Vec2::new(6.5, 0.0)
                );
                let sound: Vec<_> = world
                    .resource_mut::<Events<PlayWallUiSound>>()
                    .drain()
                    .collect();
                assert_eq!(sound.len(), 1);
                assert_eq!(sound[0].kind, 3);
                schedule.run(&mut world);
                assert!(
                    world.resource::<Events<PlayWallUiSound>>().is_empty(),
                    "continued snap must not repeat the transition sound"
                );
                world.resource_mut::<Modifiers>().flags = 1;
                schedule.run(&mut world);
                let unsnapped = world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .get_enclosure_params()
                    .unwrap();
                assert_eq!(
                    ShapeParameters::from_snapshot(unsnapped).center(),
                    glam::Vec2::new(6.0, 0.0)
                );
                world.resource_mut::<Modifiers>().flags = 0;
                world
                    .resource_mut::<PublicWalls>()
                    .get_mut(other_wall)
                    .unwrap()
                    .authority
                    .snapshot
                    .source
                    .terrain_floor = 30.0;
                schedule.run(&mut world);
                let separated = world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .get_enclosure_params()
                    .unwrap();
                assert_eq!(
                    ShapeParameters::from_snapshot(separated).center(),
                    glam::Vec2::new(6.0, 0.0)
                );
                world
                    .resource_mut::<PublicWalls>()
                    .get_mut(other_wall)
                    .unwrap()
                    .authority
                    .snapshot
                    .source
                    .terrain_floor = 0.0;
                let mut rectangle_roof = roof_bytes;
                rectangle_roof[8..12].copy_from_slice(&1u32.to_le_bytes());
                rectangle_roof[0x0c..0x10].copy_from_slice(&1.0f32.to_le_bytes());
                rectangle_roof[0x10..0x14].copy_from_slice(&0.0f32.to_le_bytes());
                rectangle_roof[0x14..0x18].copy_from_slice(&6.5f32.to_le_bytes());
                rectangle_roof[0x18..0x1c].copy_from_slice(&0.0f32.to_le_bytes());
                rectangle_roof[0x1c..0x20].copy_from_slice(&4.0f32.to_le_bytes());
                rectangle_roof[0x20..0x24].copy_from_slice(&2.0f32.to_le_bytes());
                rectangle_roof[0x34..0x38].copy_from_slice(&1.0f32.to_le_bytes());
                let roof = Roof::from_observed(rectangle_roof);
                let hit_top = roof.floor_y() + roof.height();
                let RoofRidgeSnap::Segment { a, b, t } =
                    snap_to_roof_ridge(&roof, glam::Vec3::new(6.0, hit_top, 0.0), 1.0)
                else {
                    panic!("fixture must have a ridge segment")
                };
                let expected = a * (1.0 - t) + b * t;
                *world.get_mut::<Roof>(other_roof).unwrap() = roof;
                // Isolate this frame so each producer's output can be checked.
                world.resource_mut::<UiSignifierStream>().take_entries();
                schedule.run(&mut world);
                let segment = world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .get_enclosure_params()
                    .unwrap();
                assert_eq!(
                    ShapeParameters::from_snapshot(segment).center(),
                    glam::Vec2::new(expected.x, expected.z)
                );
                assert_eq!(
                    world
                        .resource_mut::<Events<PlayWallUiSound>>()
                        .drain()
                        .count(),
                    1
                );
                let entries = world.resource_mut::<UiSignifierStream>().take_entries();
                let names: Vec<_> = entries
                    .iter()
                    .map(|entry| {
                        let crate::systems::ui::signifiers::signifier_create::UiSignifierEntry::Factory(factory) = entry else {
                            panic!("move and ridge signifiers must be factories");
                        };
                        factory.name().unwrap()
                    })
                    .collect();
                // B 14097d4c7 emits the vertical ground line only for a
                // positive elevation; this fixture's selected wall is grounded.
                assert_eq!(
                    world.resource::<PublicWalls>().get(wall).unwrap().authority.elevation(),
                    0.0
                );
                assert_eq!(names.len(), 4);
                assert_eq!(names.iter().filter(|name| name.contains("move_shape_outline")).count(), 1);
                assert_eq!(names.iter().filter(|name| name.starts_with("move_shape") && !name.contains("move_shape_outline") && !name.starts_with("move_shape_line")).count(), 1);
                assert_eq!(names.iter().filter(|name| name.starts_with("move_shape_line")).count(), 0);
                assert_eq!(names.iter().filter(|name| name.starts_with('[')).count(), 2, "ridge segment and vertical snap lines");
                world.get_mut::<Roof>(selected_roof).unwrap().observed_mut()[0x30..0x34]
                    .copy_from_slice(&f32::NAN.to_le_bytes());
                schedule.run(&mut world);
                let nan_roof = world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .get_enclosure_params()
                    .unwrap();
                assert_eq!(
                    ShapeParameters::from_snapshot(nan_roof).center(),
                    glam::Vec2::new(6.0, 0.0)
                );
                world.resource_mut::<ActionSetStates>().digital[0].phase =
                    DigitalActionPhase::JustReleased;
                schedule.run(&mut world);
            }
            if matches!(shape, Shape::Circle(_)) {
                // Exercise the rectangle/circular-border branch through the same
                // picker, press and held production systems, without roof snapping.
                let rect = utils::geometry::Rectangle2d::new(
                    glam::Vec2::ZERO,
                    glam::Vec2::splat(2.0),
                    glam::Vec2::X,
                );
                {
                    let mut inner = world.resource_mut::<InnerWalls>();
                    let mut state = inner.get(wall).unwrap().clone();
                    state.snapshot.source.curve =
                        crate::systems::wall::shapes::to_authority_curve(&rect.as_curve2());
                    state.snapshot.source.shape =
                        ShapeParameters::from_shape(Shape::Rectangle(rect)).to_snapshot();
                    state.snapshot.source.curve_kind_tag = 1;
                    inner.update_wall_state(wall, state);
                }
                world.resource_scope(|world, mut public: Mut<PublicWalls>| {
                    public.remove_roof(wall);
                    let mut inner = world.resource_mut::<InnerWalls>();
                    public.consume_with_paths(
                        &mut inner,
                        &terrain_grid,
                        None,
                        &WallPublicationDirty::default(),
                        &WallSpatialHash::default(),
                    );
                });
                *world.resource_mut::<GladeBorder>() =
                    GladeBorder(crate::resources::glade_border::BorderShape::Circle(
                        utils::geometry::Circle2d::new(glam::Vec2::ZERO, 6.0),
                    ));
                world.insert_resource(UiState::at_startup());
                // The grounded gizmo's trigger is only 0.5 high; point the
                // real picker through its center, as in the roof-snap fixture.
                let inverse = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 10.0));
                let mut camera =
                    crate::systems::camera_rig::main_camera::MainCamera::new(1280.0 / 720.0);
                camera.write_matrix(0, &glam::Mat4::IDENTITY);
                camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
                camera.write_matrix(0x80, &inverse.inverse());
                camera.write_matrix(0xc0, &inverse);
                world.insert_resource(camera);
                world.resource_mut::<ActionSetStates>().digital[0].phase =
                    crate::input::action_set::DigitalActionPhase::Idle;
                world.resource_mut::<CameraCursorRay>().origin = glam::Vec3::new(0.0, 10.0, 0.0);
                world.resource_mut::<CursorTerrainRaycast>().pos = glam::Vec3::ZERO;
                world.resource_mut::<GameTime>().delta = 1.0;
                schedule.run(&mut world);
                schedule.run(&mut world);
                world.resource_mut::<ActionSetStates>().digital[0].phase =
                    crate::input::action_set::DigitalActionPhase::JustPressed;
                schedule.run(&mut world);
                assert_eq!(world.resource::<UiState>().interaction().widget, widget);
                assert_eq!(world.resource::<UiState>().click_byte(), 3);
                world.resource_mut::<ActionSetStates>().digital[0].phase =
                    crate::input::action_set::DigitalActionPhase::Held;
                world.resource_mut::<CameraCursorRay>().origin = glam::Vec3::new(20.0, 10.0, 0.0);
                world.resource_mut::<CursorTerrainRaycast>().pos = glam::Vec3::new(20.0, 0.0, 0.0);
                schedule.run(&mut world);
                let moved = world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .get_enclosure_params()
                    .unwrap();
                // Right edge endpoints (1, ±1) meet the radius-6 circle at
                // x = sqrt(35); after one second the smoothing reaches this limit.
                let actual = ShapeParameters::from_snapshot(moved).as_rectangle().center;
                assert!((actual.x - (35.0f32.sqrt() - 1.0)).abs() < 0.0001);
                assert!(actual.y.abs() < 0.0001);
                world.resource_mut::<ActionSetStates>().digital[0].phase =
                    DigitalActionPhase::JustReleased;
                schedule.run(&mut world);
            }
            // 真实picker已消费原注册表；退出模式后清空最后一帧注册和捕获Arc。
            world.insert_resource(AppMode::Editing(EditSubject::None));
            world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::Idle;
            schedule.run(&mut world);
            world.resource_mut::<UiSignifierStream>().take_entries();
            drop(schedule);
            drop(world);
            assert!(weak.upgrade().is_none());
        }
    }

    #[test]
    fn elevated_gizmo_factories_spawn_real_meshes_line_passes_and_release() {
        use crate::{
            asset_libraries::{
                mesh_library::{AssetMeshLibrary, MeshAsset},
                shader_library::AssetShaderLibrary,
            },
            render::{
                mesh::Mesh, pass_builder::draw_list::DrawTopology, shader::RasterShaderProgram,
                test_device::TestDevice,
            },
            resources::walls::publication_source::TerrainHeightGrid512,
            systems::ui::signifiers::{
                animation::SignifierAnimation, signifier_create::UiSignifierEntry,
            },
        };
        use bevy_ecs::world::CommandQueue;
        let shape = Shape::Circle(::utils::geometry::Circle2d::new(Vec2::new(2., 3.), 2.));
        let mut authority = WallState::new(shape_curve(shape), 7., 0, 0, false);
        authority.snapshot.source.shape = ShapeParameters::from_shape(shape).to_snapshot();
        authority.snapshot.source.terrain_floor = 5.;
        let grid = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 2.,
            maximum_y: 2.,
            sampling_fix: false,
        };
        let public = PublicWallState::collect(&authority, &grid).unwrap();
        let ui = UiState::at_startup();
        let mut stream = UiSignifierStream::default();
        let position = Vec3::new(2., 7., 3.);
        let rotation = Quat::IDENTITY;
        let widget = move_gizmo_and_trigger_vol(
            position,
            rotation,
            &ActionSetStates::default(),
            &ui,
            WallId(41),
            &public,
            2.,
            &mut stream,
        );
        assert_eq!(widget.dragged(), 2);
        let factories = stream.take_entries();
        assert_eq!(factories.len(), 3);
        let UiSignifierEntry::Factory(vertical) = &factories[0] else {
            panic!("elevated ground line must be a factory")
        };
        assert!(vertical.name().unwrap().starts_with("move_shape_line"));
        let mut meshes = AssetMeshLibrary::new();
        for name in ["move_shape", "move_shape_outline"] {
            meshes.add(MeshAsset::new(Mesh::new(name), Some(name.to_owned())));
        }
        let shaders = AssetShaderLibrary::new();
        for name in ["vertex_color_shader", "dashed_line"] {
            shaders.add_raster(
                RasterShaderProgram::new_draw(name, Default::default(), &TestDevice(1)),
                false,
            );
        }
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let entities = factories
            .iter()
            .flat_map(|entry| {
                let UiSignifierEntry::Factory(factory) = entry else {
                    panic!("手柄不接受不可唤醒raw占位")
                };
                factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world))
            })
            .collect::<Vec<_>>();
        queue.apply(&mut world);
        assert_eq!(entities.len(), 3);
        let transient = world
            .get::<crate::asset_libraries::transient_mesh::TransientMesh>(entities[0])
            .unwrap()
            .mesh_handle();
        assert_eq!(world.get::<DashedLineLength>(entities[0]).unwrap().0, 5.);
        assert!(
            matches!(world.get::<DrawTopology>(entities[0]),Some(DrawTopology::Alt(width)) if *width==1.)
        );
        for entity in &entities[1..] {
            assert_eq!(
                world.get::<Transform>(*entity).unwrap().translation,
                position
            );
            let animation = world.get::<SignifierAnimation>(*entity).unwrap();
            assert_eq!(animation.value, 1.5);
            assert_eq!(animation.duration, 0.);
        }
        assert!(
            world
                .get::<crate::systems::render_loop::draw_passes::draw_lists::UiPass>(entities[1])
                .is_none()
        );
        assert!(
            world
                .get::<crate::systems::render_loop::draw_passes::draw_lists::UiPass>(entities[2])
                .is_some()
        );
        // 工厂消费者按既有record_id合同附加身份；实际生产由signifier_create负责。
        // 再接真正mark/反向动画/despawn调度验证卸载，不手工清空实体或跳过删除动画。
        let widget_id = widget.id;
        for (entity, entry) in entities.into_iter().zip(&factories) {
            let UiSignifierEntry::Factory(factory) = entry else {
                unreachable!()
            };
            world
                .entity_mut(entity)
                .insert(crate::systems::ui::signifiers::SignifierId(
                    crate::systems::ui::signifiers::signifier_create::record_id(
                        widget_id,
                        factory.name(),
                    ),
                    widget_id,
                ));
        }
        world.insert_resource(ui);
        world.insert_resource(GameTime {
            delta: 0.1,
            ..Default::default()
        });
        let mut cleanup = Schedule::default();
        cleanup.add_systems(
            (
                crate::systems::ui::signifiers::signifier_mark_for_delete::signfier_mark_for_delete,
                crate::systems::ui::signifiers::animation::signifier_animation_t,
                crate::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked,
            )
                .chain(),
        );
        cleanup.run(&mut world);
        assert_eq!(world.entities().len(), 0);
        assert_eq!(*meshes.transient_meshes().lock().unwrap(), vec![transient]);
        crate::asset_libraries::transient_mesh::delete_dropped_transient_meshes(
            &TestDevice(1),
            &mut meshes,
            &mut crate::asset_libraries::vao_library::AssetVAOLibrary::new(),
        );
        assert!(meshes.get(transient).is_none());
        assert!(meshes.transient_meshes().lock().unwrap().is_empty());
        drop(factories);
        drop(world);
    }
    #[test]
    fn move_gizmo_registers_native_box_and_camera_rotation_sound_switches() {
        use crate::input::action_set::DigitalActionPhase;
        use crate::resources::walls::WallState;
        use crate::resources::walls::public_wall_state::PublicWallState;
        use crate::resources::walls::publication_source::TerrainHeightGrid512;
        use crate::systems::ui::resources::widget_params::WidgetGeometry;
        use glam::{Quat, Vec3};

        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let anchor = Vec3::new(4.0, 2.0, -3.0);
        // These source lengths select the native low/middle/high smoothstep
        // values. Wall height is deliberately unrelated to all three values.
        for (length, half_width) in [(0.5, 0.375), (1.75, 0.5625), (3.0, 0.75)] {
            let curve = crate::resources::roofs::geometry::surface::Curve2::try_new(
                vec![[0.0, 0.0], [length, 0.0]],
                false,
            )
            .unwrap();
            let state = WallState::new(curve, 20.0, 0, 0, false);
            let shape = PublicWallState::collect(&state, &terrain).unwrap();
            for phase in [
                DigitalActionPhase::JustPressed,
                DigitalActionPhase::Held,
                DigitalActionPhase::Repeating,
                DigitalActionPhase::JustReleased,
                DigitalActionPhase::Idle,
            ] {
                let ui = UiState::at_startup();
                let mut actions = ActionSetStates::default();
                actions.digital[2].phase = phase;
                let mut stream = UiSignifierStream::default();
                let context = move_gizmo_and_trigger_vol(
                    anchor,
                    Quat::IDENTITY,
                    &actions,
                    &ui,
                    WallId(17),
                    &shape,
                    1.0,
                    &mut stream,
                );
                assert_eq!(
                    context.id,
                    crate::systems::ui::resources::ui_state::WidgetId::from_name(
                        "move_shapeWallId(17)"
                    )
                );
                let params = ui
                    .widget(context.id)
                    .expect("registration also occurs during camera rotation");
                assert!(!params.occlude_by_world);
                assert_eq!(params.flags.0, 1);
                let expected = if (phase as u8) < 3 {
                    [0, 0, 0, 0]
                } else {
                    [1, 2, 3, 3]
                };
                for (slot, kind) in params.sounds.slots.iter().zip(expected) {
                    assert_eq!(u32::from_le_bytes(slot[..4].try_into().unwrap()), kind);
                }
                let Some(WidgetGeometry::TriangleMesh(triangles)) = &params.geometry else {
                    panic!("move widget must use the original triangle mesh");
                };
                assert_eq!(triangles.len(), 12);
                let mut min = Vec3::splat(f32::INFINITY);
                let mut max = Vec3::splat(f32::NEG_INFINITY);
                for p in triangles.iter().flat_map(|t| t.points) {
                    min = min.min(p);
                    max = max.max(p);
                }
                assert_eq!(min, anchor - Vec3::new(half_width, 0.25, half_width));
                assert_eq!(max, anchor + Vec3::new(half_width, 0.25, half_width));
            }
        }
    }

    #[test]
    fn move_gizmo_factories_keep_native_names_hover_scale_and_activation_points() {
        use crate::asset_libraries::{
            mesh_library::{AssetMeshLibrary, MeshAsset},
            shader_library::AssetShaderLibrary,
        };
        use crate::render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice};
        use crate::resources::walls::{
            WallState, public_wall_state::PublicWallState, publication_source::TerrainHeightGrid512,
        };
        use crate::systems::ui::{
            resources::ui_state::{UiInteraction, WidgetId},
            signifiers::UiSignifierEntry,
        };
        use crate::utils::components::transform::Transform;
        use glam::{Quat, Vec3};

        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let curve = crate::resources::roofs::geometry::surface::Curve2::try_new(
            vec![[0.0, 0.0], [4.0, 0.0]],
            false,
        )
        .unwrap();
        let mut state = WallState::new(curve, 20.0, 0, 0, false);
        state.snapshot.source.terrain_floor = 2.5;
        let shape = PublicWallState::collect(&state, &terrain).unwrap();
        let meshes = AssetMeshLibrary::new();
        for name in ["move_shape", "move_shape_outline"] {
            meshes.add(MeshAsset::new(Mesh::new(name), Some(name.to_owned())));
        }
        let shaders = AssetShaderLibrary::new();
        for name in ["vertex_color_shader", "dashed_line"] {
            shaders.add_raster(
                RasterShaderProgram::new_draw(name, Default::default(), &TestDevice(1)),
                false,
            );
        }
        let anchor = Vec3::new(4.0, 2.0, -3.0);
        let start = Vec3::new(4.0, 1.0, -3.0);
        let rotation = Quat::from_rotation_y(0.5);
        let wall = WallId(17);
        let widget = WidgetId::from_name("move_shapeWallId(17)");
        for (tag, phase, matched, scale) in [
            (0, 1, true, 1.0),
            (1, 0, true, 1.0),
            (1, 1, true, 1.2),
            (2, 2, true, 1.0),
            (2, 0, true, 1.2),
            (2, 3, true, 1.2),
            (2, 3, false, 1.0),
        ] {
            let ui = UiState::new(UiInteraction {
                native_tag: tag,
                native_phase: phase,
                widget: if matched { widget } else { WidgetId(0) },
            });
            let mut stream = UiSignifierStream::default();
            move_gizmo_and_trigger_vol(
                anchor,
                rotation,
                &ActionSetStates::default(),
                &ui,
                wall,
                &shape,
                1.0,
                &mut stream,
            );
            let entries = stream.take_entries();
            assert_eq!(
                entries.len(),
                3,
                "positive elevation emits the two-point line before both axes"
            );
            let mut world = World::new();
            let mut queue = bevy_ecs::world::CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &world);
            for (i, entry) in entries.iter().enumerate() {
                let UiSignifierEntry::Factory(factory) = entry else {
                    panic!("real factory required");
                };
                let expected = if i == 0 {
                    format!("move_shape_line{wall:?}{anchor:?}{start}{anchor}")
                } else {
                    let mesh = ["move_shape", "move_shape_outline"][i - 1];
                    format!("move_shape{wall:?}{anchor:?}{mesh}{scale}{rotation:?}")
                };
                assert_eq!(factory.name(), Some(expected.as_str()));
                assert_eq!(factory.wake(&meshes, &shaders, &mut commands).len(), 1);
            }
            drop(commands);
            queue.apply(&mut world);
            // Native workers keep flag8=0 and duration=0: creation starts at
            // zero scale; the real animation stage establishes the target.
            for (transform, animation) in world
                .query::<(
                    &Transform,
                    &crate::systems::ui::signifiers::SignifierAnimation,
                )>()
                .iter(&world)
            {
                assert_eq!(transform.scale, Vec3::ZERO);
                assert_eq!(
                    (animation.flag_8, animation.flag_9, animation.duration),
                    (0, 1, 0.0)
                );
            }
            use bevy_ecs::system::RunSystemOnce;
            world.insert_resource(UiState::at_startup());
            world.insert_resource(GameTime::default());
            world
                .run_system_once(crate::systems::ui::signifiers::animation::signifier_animation_t)
                .unwrap();
            world
                .run_system_once(crate::systems::ui::signifiers::animation::signifier_animation)
                .unwrap();
            let transforms: Vec<_> = world.query::<&Transform>().iter(&world).copied().collect();
            assert_eq!(transforms.len(), 3);
            let axes: Vec<_> = transforms
                .iter()
                .filter(|t| t.translation == anchor)
                .collect();
            assert_eq!(axes.len(), 2);
            for axis in axes {
                assert_eq!(axis.rotation, rotation);
                assert_eq!(axis.scale, Vec3::splat(scale * 1.5));
            }
            let lengths: Vec<_> = world
                .query::<&crate::components::DashedLineLength>()
                .iter(&world)
                .map(|x| x.0)
                .collect();
            assert_eq!(lengths, vec![1.0]);
        }
    }
}
