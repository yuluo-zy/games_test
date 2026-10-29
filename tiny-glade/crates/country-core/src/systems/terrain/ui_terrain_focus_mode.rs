//! 地形焦点编辑：B `1409a4fb0` 的坡度、高度和平移工具。
//!
//! 原 PDB 527:19776 长度 13202；三个四路拖拽跳表已按发行 RNE 核准。
//! 原先截断反编译误入相邻墙体函数，CircularF32 和窗口焦点事件均不属于本系统。
use super::{
    deferred_terrain_history::{PendingTerrainHistory, TerrainEdit},
    edit_terrain_stroke::{EditTerrainStrokeCmd, SetTerrainStrokePtsCmd},
    terrain_hover_idx::HoveredTerrainStrokeId,
};
use crate::{
    input::action_set::ActionSetStates,
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS},
        terrain_editing::{StrokeId, TerrainEditingData},
        walls::decorator_archivist::LiveDecoratorArchivist,
        window::WindowSize,
    },
    systems::{
        camera_rig::MainCamera,
        collision::world::RaycastWorld,
        mode_manager::{AppMode, EditSubject},
        render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd,
        ui::{
            resources::{
                ui_state::UiState,
                widget_params::{UiSounds, WidgetGeometry, WidgetParams},
            },
            signifiers::{SignifierEntityBuilder, SignifierFactory, UiSignifierStream},
        },
        wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume,
    },
    utils::{
        components::transform::Transform,
        project_cursor_onto_ray::{ProportionalScreenSpace, ProportionalScreenSpaceCoord},
    },
};
use bevy_ecs::prelude::*;
use glam::{Quat, Vec2, Vec3};
use std::sync::Arc;
use tg_terrain::edit::{Adjustment, EditCommand};
use utils::geometry::{Circle2d, Ray};

/// 原 Local 的拥有型表示。三个工具复用原缓存，不能按每帧输入重造历史起点。
#[derive(Default)]
pub struct EditTerrainStrokeCache {
    original_points: Option<Vec<[f32; 3]>>,
    original_first: Option<Vec3>,
    offset: Vec2,
    direction: Vec2,
    previous_height: f32,
    previous_falloff: f32,
}

/// 原 minps/maxps 加 unordered 掩码：忽略单侧 NaN，相等时保留累积值。
fn bounds_value(candidate: f32, previous: f32, maximum: bool) -> f32 {
    if previous.is_nan()
        || if maximum {
            candidate > previous
        } else {
            candidate < previous
        }
    {
        candidate
    } else {
        previous
    }
}

/// 原锚点是最靠近 XZ 包围盒中心的实际笔画点，不是包围盒中心。
fn focus_anchor(points: &[[f32; 3]]) -> Vec2 {
    let first = points[0];
    let mut minimum = Vec2::new(first[0], first[2]);
    let mut maximum = minimum;
    for point in points {
        minimum.x = bounds_value(point[0], minimum.x, false);
        minimum.y = bounds_value(point[2], minimum.y, false);
        maximum.x = bounds_value(point[0], maximum.x, true);
        maximum.y = bounds_value(point[2], maximum.y, true);
    }
    let center = (minimum + maximum) * 0.5;
    let mut closest = Vec2::new(first[0], first[2]);
    let mut distance = 1_000_000.0_f32;
    for point in points {
        let x = point[0] - center.x;
        let z = point[2] - center.y;
        let candidate = (z * z + x * x).sqrt();
        if distance > candidate {
            distance = candidate;
            closest = Vec2::new(point[0], point[2]);
        }
    }
    closest
}

fn length_original(vector: Vec3) -> f32 {
    (vector.z * vector.z + (vector.y * vector.y + vector.x * vector.x)).sqrt()
}

/// 原 maxss/minss 的第二操作数携带 NaN，不能用 f32::clamp 改写失败/NaN 行为。
fn original_clamp(value: f32, maximum: f32) -> f32 {
    let value = if 0.0 <= value {
        value
    } else if value.is_nan() {
        value
    } else {
        0.0
    };
    if value <= maximum {
        value
    } else if value.is_nan() {
        value
    } else {
        maximum
    }
}

fn falloff_parameter(cursor: Vec2, anchor: Vec2, direction: Vec2) -> f32 {
    let start = direction * 2.5 + anchor;
    let end = direction * 7.0 + anchor;
    let delta = end - start;
    let offset = cursor - start;
    (offset.y * delta.y + offset.x * delta.x) / (delta.y * delta.y + delta.x * delta.x).max(1e-20)
}

fn stage_parameter_history(
    cache: &EditTerrainStrokeCache,
    pending: &mut PendingTerrainHistory,
    id: StrokeId,
    height: f32,
    falloff: f32,
) {
    pending.stage(TerrainEdit::EditTerrainStroke {
        index: id,
        prev_height_ws: cache.previous_height,
        new_height_ws: height,
        prev_falloff: cache.previous_falloff,
        new_falloff: falloff,
    });
}

/// 原输入处于旋转相机模式时静音；四声音槽的 kind 与首槽内层 tag 均保留。
fn terrain_widget(geometry: WidgetGeometry, sounds_enabled: bool) -> WidgetParams {
    let mut sounds = UiSounds::empty();
    for (slot, tag) in sounds.slots.iter_mut().zip([1u32, 2, 3, 3]) {
        slot[..4].copy_from_slice(&(tag * u32::from(sounds_enabled)).to_le_bytes());
    }
    sounds.slots[0][8..12].copy_from_slice(&2u32.to_le_bytes());
    WidgetParams::default()
        .geometry(geometry)
        .sounds(sounds)
        .occlude_by_world(false)
}

fn projected_height(cursor: Vec2, window: &WindowSize, camera: &MainCamera, anchor: Vec3) -> Vec3 {
    let screen = ProportionalScreenSpace::from_window_size([window.width, window.height]);
    let coord = screen.transform_screen_space(cursor.x, cursor.y);
    ProportionalScreenSpaceCoord(coord).project_onto_ray(
        &screen,
        &camera.read_matrix(0x80),
        &camera.read_matrix(0),
        Ray {
            origin: Vec3::new(anchor.x, 0.0, anchor.z),
            direction: Vec3::Y,
        },
        [0.0, f32::MAX],
    )
}

/// 原 13 组 SystemParam。安全 ResMut 承载指示符流，不维持输入镜像或伪事件。
#[allow(clippy::too_many_arguments)]
pub fn ui_terrain_focus_mode(
    mode: Res<AppMode>,
    mut terrain: ResMut<TerrainEditingData>,
    ui: Res<UiState>,
    mut stream: ResMut<UiSignifierStream>,
    mut cache: Local<EditTerrainStrokeCache>,
    ray: Res<CameraCursorRay>,
    mut pending: ResMut<PendingTerrainHistory>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    mut points: EventWriter<SetTerrainStrokePtsCmd>,
    (border, mut display_border, mut feedback_border, mut highlights): (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
        EventWriter<AddTerrainHighlightCmd>,
    ),
    actions: Res<ActionSetStates>,
    mut edits: EventWriter<EditTerrainStrokeCmd>,
    (window, cursor, camera, world, hovered, terrain_cursor): (
        Res<WindowSize>,
        Res<CursorPositionSS>,
        Res<MainCamera>,
        Res<RaycastWorld>,
        Res<HoveredTerrainStrokeId>,
        Res<CursorTerrainRaycast>,
    ),
) {
    let AppMode::Editing(EditSubject::Terrain(value)) = *mode else {
        return;
    };
    // 原 ResMut 在 Mode5 即触发 changed tick，空 map/未找到选中 ID 也不能跳过。
    terrain.set_changed();
    let id = StrokeId(value);
    let native_id = native_stroke_name(value);
    let mut active = false;
    if let Some(stroke) = terrain.strokes.get(&id) {
        let anchor_xz = focus_anchor(&stroke.points);
        let anchor = Vec3::new(anchor_xz.x, stroke.height_ws, anchor_xz.y);
        let cursor_xz3 = ray.intersect_with_xz_plane(stroke.height_ws);
        let cursor_xz = Vec2::new(cursor_xz3.x, cursor_xz3.z);
        let radius = stroke.falloff * 7.0 + (1.0 - stroke.falloff) * 2.5;
        let sounds_enabled = actions.digital[2].phase as u32 > 2;
        let circle = Circle2d::new(anchor_xz, radius);
        let widget = ui.add(
            &format!("edit_falloff_terrain_focused_{native_id}"),
            terrain_widget(
                WidgetGeometry::SphereCloud {
                    centers: circle.as_points3(0.5, stroke.height_ws).collect(),
                    radius: 0.5,
                },
                sounds_enabled,
            ),
        );
        active |= widget.is_active();
        let width = if widget.is_active() { 5.0 } else { 2.0 };
        write_terrain_circle(&mut stream, value, anchor, radius, width);
        let phase = widget.dragged();
        match phase {
            2 if widget.is_active() => {
                let hit = ui.intersection_point();
                let delta = Vec2::new(hit.x - anchor_xz.x, hit.z - anchor_xz.y);
                let inverse = 1.0 / (delta.y * delta.y + delta.x * delta.x).sqrt();
                cache.direction = delta * inverse;
            }
            3 => {
                cache.offset.x =
                    falloff_parameter(cursor_xz, anchor_xz, cache.direction) - stroke.falloff;
                cache.previous_falloff = stroke.falloff;
                cache.previous_height = stroke.height_ws;
            }
            4 => {
                let falloff = original_clamp(
                    falloff_parameter(cursor_xz, anchor_xz, cache.direction) - cache.offset.x,
                    1.0,
                );
                if falloff != stroke.falloff {
                    edits.write(EditTerrainStrokeCmd(EditCommand {
                        id,
                        falloff: Adjustment::Absolute(falloff),
                        height: Adjustment::Keep,
                        position: anchor.to_array(),
                    }));
                }
            }
            0 | 1 => {
                stage_parameter_history(&cache, &mut pending, id, stroke.height_ws, stroke.falloff)
            }
            _ => {}
        }
        if widget.is_active() {
            let rotation = utils::xz_rotation_bad(cache.direction);
            let tip_xz = cache.direction * radius + anchor_xz;
            let tip = Vec3::new(tip_xz.x, stroke.height_ws, tip_xz.y)
                + rotate_original(rotation, Vec3::new(0.0, 0.0, 0.2));
            write_terrain_falloff_arrow(&mut stream, value, circle, rotation, tip);
        }
        let height_center = Vec3::new(anchor.x + 0.0, (anchor.y + 1.0) + 0.5, anchor.z + 0.0);
        let height_mesh =
            box_trigger_volume(0.75, 1.0, 0.5, &Transform::from_translation(height_center));
        let widget = ui.add(
            &format!("edit_height_terrain_{native_id}"),
            terrain_widget(WidgetGeometry::TriangleMesh(height_mesh), sounds_enabled),
        );
        active |= widget.is_active();
        let scale = if widget.hovered() != 0 || widget.dragged() != 2 {
            1.2
        } else {
            1.0
        };
        write_terrain_height_arrow(&mut stream, value, anchor, scale);
        match widget.dragged() {
            3 => {
                let projected = projected_height(cursor.0, &window, &camera, anchor);
                cache.offset = Vec2::new(0.0, length_original(projected - anchor));
                cache.previous_falloff = stroke.falloff;
                cache.previous_height = stroke.height_ws;
                archivist.decorator_cull_allow_any();
            }
            4 => {
                let projected = projected_height(cursor.0, &window, &camera, anchor);
                let ground = Vec3::new(anchor.x, 0.0, anchor.z);
                let height =
                    original_clamp(length_original(projected - ground) - cache.offset.y, 20.0);
                if height != stroke.height_ws {
                    edits.write(EditTerrainStrokeCmd(EditCommand {
                        id,
                        falloff: Adjustment::Keep,
                        height: Adjustment::Absolute(height),
                        position: anchor.to_array(),
                    }));
                }
            }
            0 | 1 => {
                stage_parameter_history(&cache, &mut pending, id, stroke.height_ws, stroke.falloff)
            }
            _ => {}
        }
        let move_mesh = box_trigger_volume(1.5, 0.5, 1.5, &Transform::from_translation(anchor));
        let widget = ui.add(
            &format!("move_terrain{native_id}"),
            terrain_widget(WidgetGeometry::TriangleMesh(move_mesh), sounds_enabled),
        );
        active |= widget.is_active();
        let scale = if widget.hovered() != 0 || widget.dragged() != 2 {
            1.2
        } else {
            1.0
        };
        write_terrain_move_arrows(&mut stream, value, anchor, scale);
        match widget.dragged() {
            3 => {
                archivist.decorator_cull_allow_any();
                cache.original_points = Some(stroke.points.clone());
                let first = Vec3::from_array(stroke.points[0]);
                cache.original_first = Some(first);
                cache.offset = cursor_xz - Vec2::new(first.x, first.z);
            }
            4 => {
                let original = cache
                    .original_points
                    .as_ref()
                    .expect("原地形平移必须先捕获点集");
                let first = original.first().expect("原地形平移的捕获点集不能为空");
                let target = cursor_xz - cache.offset;
                let delta = target - Vec2::new(first[0], first[2]);
                // 原 map closure 对 Y 也执行 +0；保留有符号零行为。
                let moved = original
                    .iter()
                    .map(|point| [point[0] + delta.x, point[1] + 0.0, point[2] + delta.y])
                    .collect();
                if border.is_shape_inside(BorderShape::Circle(Circle2d::new(cursor_xz, 2.0))) {
                    points.write(SetTerrainStrokePtsCmd {
                        points: moved,
                        position: anchor,
                        id,
                    });
                } else {
                    display_border.write(DisplayOutOfBorder);
                    feedback_border.write(FeedbackInputOutOfBorder);
                }
            }
            0 | 1 => {
                if let Some(original) = cache.original_points.take() {
                    let current = stroke.points[0];
                    let first = cache.original_first.expect("原地形平移历史必须保留原首点");
                    pending.stage(TerrainEdit::MoveTerrainStroke {
                        id,
                        original_stroke_pts: original,
                        move_vec: [current[0] - first.x, current[2] - first.z],
                    });
                }
            }
            _ => {}
        }
    }
    if active {
        return;
    }
    let hit = world.raycast_simple(ray.origin, ray.dir, 0x3f43);
    let visible_distance = if hit.is_miss() {
        f32::MAX
    } else {
        length_original(hit.position - ray.origin)
    };
    let cursor_position = terrain_cursor.position();
    if length_original(cursor_position - ray.origin) < visible_distance
        && hovered.tag == 1
        && hovered.id as u32 != value
    {
        highlights.write(AddTerrainHighlightCmd(hovered.id as i32));
    }
}

/// 原 glam 四元数乘向量多项式；不归一化、不简化为 yaw 假设。
fn rotate_original(rotation: Quat, vector: Vec3) -> Vec3 {
    let [x, y, z, w] = rotation.to_array();
    let squared = z * z + (y * y + x * x);
    let dot = (vector.z * z + (vector.y * y + vector.x * x)) * 2.0;
    let double_w = w + w;
    let scale = w * w - squared;
    Vec3::new(
        double_w * (y * vector.z - z * vector.y) + (dot * x + scale * vector.x),
        double_w * (z * vector.x - x * vector.z) + (dot * y + scale * vector.y),
        double_w * (x * vector.y - y * vector.x) + (dot * z + scale * vector.z),
    )
}

/// 原 TerrainStrokeId 的 Debug 名也是 widget/factory 身份的一部分。
fn native_stroke_name(id: u32) -> String {
    format!("TerrainStrokeId({id})")
}

fn write_terrain_circle(
    stream: &mut UiSignifierStream,
    id: u32,
    anchor: Vec3,
    radius: f32,
    width: f32,
) {
    let circle = Circle2d::new(Vec2::new(anchor.x, anchor.z), radius);
    let name = format!(
        "falloff{}_edit_{width:?}{anchor}{radius}",
        native_stroke_name(id)
    );
    stream.write_factory(&SignifierFactory::from_closure(
        &name,
        Arc::new(move |meshes, shaders, commands| {
            let generated =
                crate::systems::ui::signifiers::signifier_factory::SignifierMesh::Generated(
                    Box::new(move || circle.as_points3(0.5, anchor.y).collect()),
                );
            let entity = SignifierEntityBuilder::new(generated)
                .custom_shader("vertex_color_shader")
                .animation_duration(0.0)
                .build(meshes, shaders, commands);
            commands.entity(entity).insert(
                crate::render::pass_builder::draw_list::DrawTopology::Alt(width),
            );
            vec![entity]
        }),
    ));
}

fn write_terrain_falloff_arrow(
    stream: &mut UiSignifierStream,
    id: u32,
    circle: Circle2d,
    rotation: Quat,
    position: Vec3,
) {
    let name = format!(
        "falloff{}_edit_{circle:?}{position:?}_arrow",
        native_stroke_name(id)
    );
    stream.write_factory(&SignifierFactory::from_closure(
        &name,
        Arc::new(move |meshes, shaders, commands| {
            vec![
                SignifierEntityBuilder::from_mesh_name("brush_arrow_flat_single_foward")
                    .custom_shader("vertex_color_shader")
                    .rotation(rotation)
                    .position(position)
                    .scale(0.75)
                    .animation_duration(0.0)
                    .build(meshes, shaders, commands),
            ]
        }),
    ));
}

fn write_terrain_height_arrow(stream: &mut UiSignifierStream, id: u32, anchor: Vec3, scale: f32) {
    let rotation = Quat::IDENTITY;
    let position = Vec3::new(anchor.x + 0.0, anchor.y + 1.0, anchor.z + 0.0);
    let name = format!(
        "height_{}_arrow_{rotation:?}_{anchor:?}{scale}",
        native_stroke_name(id)
    );
    stream.write_factory(&SignifierFactory::from_closure(
        &name,
        Arc::new(move |meshes, shaders, commands| {
            vec![
                SignifierEntityBuilder::from_mesh_name("brush_arrow_flat_single_up")
                    .custom_shader("vertex_color_shader")
                    .rotation(rotation)
                    .position(position)
                    .scale(scale * 1.5)
                    .animation_duration(0.0)
                    .build(meshes, shaders, commands),
            ]
        }),
    ));
}

fn write_terrain_move_arrows(stream: &mut UiSignifierStream, id: u32, position: Vec3, scale: f32) {
    let rotation = Quat::IDENTITY;
    for (mesh, pass) in [("move_shape", 0), ("move_shape_outline", 1)] {
        let name = format!(
            "move_shape{}{position:?}{mesh}{scale}{rotation:?}",
            native_stroke_name(id)
        );
        stream.write_factory(&SignifierFactory::from_closure(
            &name,
            Arc::new(move |meshes, shaders, commands| {
                vec![
                    SignifierEntityBuilder::from_mesh_name(mesh)
                        .custom_shader("vertex_color_shader")
                        .rotation(rotation)
                        .position(position)
                        .scale(scale * 1.5)
                        .set_ui_pass(pass)
                        .animation_duration(0.0)
                        .build(meshes, shaders, commands),
                ]
            }),
        ));
    }
}

#[cfg(test)]
mod original_focus_tests {
    use super::*;
    use crate::{
        resources::{
            history::{EditType, History},
            time::GameTime,
        },
        systems::{
            terrain::{
                delayed_terrain_changed_msg::ScheduledTerrainReadback, register_terrain_events,
            },
            ui::resources::ui_state::{UiInteraction, WidgetId},
        },
    };
    use bevy_ecs::system::RunSystemOnce;

    fn fixture() -> World {
        let mut world = World::new();
        register_terrain_events(&mut world);
        let mut data = tg_terrain::stroke::TerrainEditingData::default();
        for (id, positions) in [
            (7, vec![[1., 1., 2.], [4., 1., -1.]]),
            (99, vec![[80., 0., 80.]]),
        ] {
            data.strokes.insert(
                StrokeId(id),
                tg_terrain::stroke::TerrainStroke {
                    points: positions,
                    falloff: 0.4,
                    index: StrokeId(id),
                    height_ws: 2.5,
                },
            );
        }
        world.insert_resource(TerrainEditingData(data));
        world.insert_resource(AppMode::Editing(EditSubject::Terrain(7)));
        world.insert_resource(UiState::at_startup());
        world.insert_resource(UiSignifierStream::default());
        world.insert_resource(CameraCursorRay {
            origin: Vec3::new(4., 10., 6.),
            dir: -Vec3::Y,
        });
        world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
            Vec2::ZERO,
            100.,
        ))));
        world.insert_resource(ActionSetStates::default());
        world.insert_resource(WindowSize::new(100, 100));
        world.insert_resource(CursorPositionSS(Vec2::new(50., -50.)));
        let mut camera = MainCamera::default();
        camera.write_matrix(0x80, &glam::Mat4::IDENTITY);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        world.insert_resource(camera);
        world.insert_resource(RaycastWorld::default());
        world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(GameTime {
            delta: 0.1,
            ..Default::default()
        });
        world.insert_resource(ScheduledTerrainReadback::default());
        world
    }

    fn interact(world: &mut World, name: &str, phase: u8, position: Vec3) {
        let mut ui = world.resource_mut::<UiState>();
        ui.replace_interaction(UiInteraction {
            native_tag: 2,
            native_phase: phase,
            widget: WidgetId::from_name(name),
        });
        ui.set_intersection_tag(0);
        ui.set_intersection_point(position);
        ui.clear_widgets();
    }

    #[test]
    fn original_focus_numeric_blocks_match_704_native_cases_exactly() {
        let oracle: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../reports/module-recovery-2026-10-23/m02-focus-numeric-native.json"
        ))
        .unwrap();
        let values = |row: &serde_json::Value| -> Vec<f32> {
            row.as_array()
                .unwrap()
                .iter()
                .map(|word| f32::from_bits(word.as_u64().unwrap() as u32))
                .collect()
        };
        let anchors = oracle["anchors"].as_array().unwrap();
        assert_eq!(anchors.len(), 192);
        for (index, row) in anchors.iter().enumerate() {
            let points: Vec<[f32; 3]> = row["points"]
                .as_array()
                .unwrap()
                .iter()
                .map(|point| values(point).try_into().unwrap())
                .collect();
            let expected = values(&row["anchor"]);
            assert_eq!(
                focus_anchor(&points).to_array().map(f32::to_bits),
                [expected[0].to_bits(), expected[1].to_bits()],
                "anchor {index}"
            );
            let ray = values(&row["ray"]);
            let height = f32::from_bits(row["height"].as_u64().unwrap() as u32);
            let ray = CameraCursorRay {
                origin: Vec3::new(ray[0], ray[1], ray[2]),
                dir: Vec3::new(ray[3], ray[4], ray[5]),
            };
            let actual = ray.intersect_with_xz_plane(height);
            let expected = values(&row["cursor"]);
            assert_eq!(
                [actual.x.to_bits(), actual.z.to_bits()],
                [expected[0].to_bits(), expected[1].to_bits()],
                "cursor {index}"
            );
        }
        for (index, row) in oracle["rotations"].as_array().unwrap().iter().enumerate() {
            let q = Quat::from_array(values(&row["quaternion"]).try_into().unwrap());
            let position = Vec3::from_array(values(&row["position"]).try_into().unwrap());
            let actual = position + rotate_original(q, Vec3::new(0.0, 0.0, 0.2));
            let expected = values(&row["result"]);
            assert_eq!(
                actual.to_array().map(f32::to_bits),
                [
                    expected[0].to_bits(),
                    expected[1].to_bits(),
                    expected[2].to_bits()
                ],
                "rotation {index}"
            );
        }
        let falloff = oracle["falloff"].as_array().unwrap();
        assert_eq!(falloff.len(), 256);
        for (index, row) in falloff.iter().enumerate() {
            let v = |name| Vec2::from_array(values(&row[name]).try_into().unwrap());
            let offset = f32::from_bits(row["offset"].as_u64().unwrap() as u32);
            let actual = original_clamp(
                falloff_parameter(v("cursor"), v("anchor"), v("direction")) - offset,
                1.,
            );
            assert_eq!(
                actual.to_bits(),
                row["result"].as_u64().unwrap() as u32,
                "falloff {index}"
            );
        }
    }

    #[test]
    fn original_focus_selects_explicit_id_and_publishes_real_widgets_and_factories() {
        let mut world = fixture();
        world.run_system_once(ui_terrain_focus_mode).unwrap();
        assert_eq!(world.resource::<UiState>().registry_snapshot().len(), 3);
        let widget = world
            .resource::<UiState>()
            .widget(WidgetId::from_name(
                "edit_falloff_terrain_focused_TerrainStrokeId(7)",
            ))
            .unwrap();
        let WidgetGeometry::SphereCloud { centers, radius } = widget.geometry.as_ref().unwrap()
        else {
            panic!("必须是原球云")
        };
        assert_eq!(*radius, 0.5);
        assert!(centers.iter().all(|point| point.y == 2.5 && point.x < 10.));
        assert!(!widget.occlude_by_world);
        assert_eq!(widget.sounds.slots[0][..4], 1u32.to_le_bytes());
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        assert_eq!(entries.len(), 4, "普通状态为圆线、向上箭头和两层移动网格");
        assert!(entries.iter().all(|entry| matches!(
            entry,
            crate::systems::ui::signifiers::UiSignifierEntry::Factory(_)
        )));
        *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::Terrain(700));
        world.resource::<UiState>().clear_widgets();
        world.run_system_once(ui_terrain_focus_mode).unwrap();
        assert!(
            world.resource::<UiState>().registry_snapshot().is_empty(),
            "不存在的ID不能退到最后一个笔画"
        );
    }

    #[test]
    fn original_focus_falloff_and_height_complete_stages_update_real_consumer_and_history() {
        for height_tool in [false, true] {
            let mut world = fixture();
            let mut schedule = Schedule::default();
            schedule.add_systems(
                (
                    ui_terrain_focus_mode,
                    super::super::edit_terrain_stroke::edit_terrain_stroke,
                )
                    .chain(),
            );
            let falloff_name = "edit_falloff_terrain_focused_TerrainStrokeId(7)";
            let name = if height_tool {
                "edit_height_terrain_TerrainStrokeId(7)"
            } else {
                falloff_name
            };
            // 原坡度方向只在 None 且实际交点属于该球云时捕获。
            interact(&mut world, falloff_name, 2, Vec3::new(2., 2.5, 2.));
            world.resource_mut::<CameraCursorRay>().origin = Vec3::new(5.3, 10., 2.);
            schedule.run(&mut world);
            interact(&mut world, name, 3, Vec3::new(1., 2.5, 2.));
            schedule.run(&mut world);
            interact(&mut world, name, 4, Vec3::new(1., 2.5, 2.));
            world.resource_mut::<CameraCursorRay>().origin = Vec3::new(20., 10., 2.);
            world.resource_mut::<CursorPositionSS>().0.y = -2000.;
            schedule.run(&mut world);
            let stroke = &world.resource::<TerrainEditingData>().strokes[&StrokeId(7)];
            assert_eq!(
                if height_tool {
                    stroke.height_ws
                } else {
                    stroke.falloff
                },
                if height_tool { 20. } else { 1. }
            );
            if height_tool {
                world.resource_mut::<CursorPositionSS>().0.y = 500.;
                interact(&mut world, name, 4, Vec3::ONE);
                schedule.run(&mut world);
                assert_eq!(
                    world.resource::<TerrainEditingData>().strokes[&StrokeId(7)].height_ws,
                    0.
                );
            }
            interact(&mut world, name, 1, Vec3::ONE);
            schedule.run(&mut world);
            let edit = world
                .resource::<PendingTerrainHistory>()
                .edit
                .clone()
                .unwrap();
            assert_eq!(
                edit,
                TerrainEdit::EditTerrainStroke {
                    index: StrokeId(7),
                    prev_height_ws: 2.5,
                    new_height_ws: if height_tool { 0. } else { 2.5 },
                    prev_falloff: 0.4,
                    new_falloff: if height_tool { 0.4 } else { 1. }
                }
            );
            assert!(
                world
                    .resource::<Events<super::super::RecalculateHeightMapCmd>>()
                    .len()
                    > 0
            );
            assert!(
                world
                    .resource::<Events<super::super::PlayTerrainSound>>()
                    .len()
                    > 0
            );
        }
    }

    #[test]
    fn original_focus_real_factories_keep_render_components_and_release_transient_mesh() {
        use crate::{
            asset_libraries::{
                mesh_library::{AssetMeshLibrary, MeshAsset},
                shader_library::AssetShaderLibrary,
                transient_mesh::{TransientMesh, drain_meshes_to_delete},
            },
            render::{
                mesh::Mesh, pass_builder::draw_list::DrawTopology, shader::RasterShaderProgram,
                test_device::TestDevice,
            },
            systems::ui::signifiers::{UiSignifierEntry, animation::SignifierAnimation},
        };
        use bevy_ecs::world::CommandQueue;
        let mut world = fixture();
        interact(
            &mut world,
            "edit_falloff_terrain_focused_TerrainStrokeId(7)",
            2,
            Vec3::new(2., 2.5, 2.),
        );
        world.run_system_once(ui_terrain_focus_mode).unwrap();
        let factories = world.resource_mut::<UiSignifierStream>().take_entries();
        assert_eq!(factories.len(), 5);
        let mut meshes = AssetMeshLibrary::new();
        for name in [
            "brush_arrow_flat_single_foward",
            "brush_arrow_flat_single_up",
            "move_shape",
            "move_shape_outline",
        ] {
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
        let entities: Vec<_> = factories
            .iter()
            .flat_map(|entry| {
                let UiSignifierEntry::Factory(factory) = entry else {
                    panic!("不能用Raw替代工厂")
                };
                factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world))
            })
            .collect();
        queue.apply(&mut world);
        assert_eq!(entities.len(), 5);
        assert_eq!(
            world.get::<DrawTopology>(entities[0]),
            Some(&DrawTopology::Alt(5.0))
        );
        for (entity, scale) in entities.iter().zip([1., 0.75, 1.5, 1.5, 1.5]) {
            let animation = world.get::<SignifierAnimation>(*entity).unwrap();
            assert_eq!(animation.duration.to_bits(), 0);
            assert_eq!(animation.value, scale);
            assert!(
                world
                    .get::<Transform>(*entity)
                    .unwrap()
                    .rotation
                    .is_finite()
            );
        }
        let temporary = world
            .get::<TransientMesh>(entities[0])
            .unwrap()
            .mesh_handle();
        assert!(meshes.get(temporary).is_some());
        for entity in entities {
            assert!(world.despawn(entity));
        }
        assert_eq!(
            meshes.transient_meshes().lock().unwrap().as_slice(),
            &[temporary]
        );
        let registry = Arc::clone(meshes.transient_meshes());
        drain_meshes_to_delete(&registry, |handle| meshes.remove(handle));
        assert!(meshes.get(temporary).is_none());
        assert!(registry.lock().unwrap().is_empty());
    }

    #[test]
    fn original_focus_missing_capture_and_pending_history_preserve_failure_order() {
        use std::panic::{AssertUnwindSafe, catch_unwind};
        let mut empty = fixture();
        empty
            .resource_mut::<TerrainEditingData>()
            .strokes
            .get_mut(&StrokeId(7))
            .unwrap()
            .points
            .clear();
        assert!(
            catch_unwind(AssertUnwindSafe(
                || empty.run_system_once(ui_terrain_focus_mode)
            ))
            .is_err()
        );
        assert_eq!(empty.resource::<UiSignifierStream>().committed_len(), 0);
        let mut missing = fixture();
        interact(&mut missing, "move_terrainTerrainStrokeId(7)", 4, Vec3::ONE);
        assert!(
            catch_unwind(AssertUnwindSafe(
                || missing.run_system_once(ui_terrain_focus_mode)
            ))
            .is_err()
        );
        assert_eq!(
            missing.resource::<Events<SetTerrainStrokePtsCmd>>().len(),
            0
        );
        let mut occupied = fixture();
        occupied.resource_mut::<PendingTerrainHistory>().edit =
            Some(TerrainEdit::NewTerrainStroke {
                index: StrokeId(99),
                height_ws: 1.,
                stroke_pts: vec![[0.; 3]],
            });
        interact(
            &mut occupied,
            "edit_height_terrain_TerrainStrokeId(7)",
            1,
            Vec3::ONE,
        );
        assert!(
            catch_unwind(AssertUnwindSafe(
                || occupied.run_system_once(ui_terrain_focus_mode)
            ))
            .is_err()
        );
        assert!(
            matches!(
                occupied.resource::<PendingTerrainHistory>().edit.as_ref(),
                Some(TerrainEdit::EditTerrainStroke {
                    index: StrokeId(7),
                    ..
                })
            ),
            "原replace先提交新值再因旧pending非空断言失败"
        );
    }

    #[test]
    fn original_focus_hover_writes_canonical_highlight_and_active_widget_takes_precedence() {
        let mut world = fixture();
        world.insert_resource(AppMode::Editing(EditSubject::Terrain(700)));
        *world.resource_mut::<HoveredTerrainStrokeId>() = HoveredTerrainStrokeId { tag: 1, id: 99 };
        *world.resource_mut::<CursorTerrainRaycast>() = CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::new(4., 0., 6.),
            ..Default::default()
        };
        world.run_system_once(ui_terrain_focus_mode).unwrap();
        let events = world
            .resource_mut::<Events<AddTerrainHighlightCmd>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0, 99);
        *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::Terrain(7));
        interact(&mut world, "move_terrainTerrainStrokeId(7)", 2, Vec3::ONE);
        world.run_system_once(ui_terrain_focus_mode).unwrap();
        assert_eq!(
            world.resource::<Events<AddTerrainHighlightCmd>>().len(),
            0,
            "真实活动widget抢占地形hover高亮"
        );
    }

    #[test]
    fn original_focus_move_cross_frame_boundary_history_and_replay_use_actual_owners() {
        let mut world = fixture();
        let original = world.resource::<TerrainEditingData>().strokes[&StrokeId(7)]
            .points
            .clone();
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ui_terrain_focus_mode,
                super::super::edit_terrain_stroke::edit_terrain_stroke,
            )
                .chain(),
        );
        let name = "move_terrainTerrainStrokeId(7)";
        interact(&mut world, name, 3, Vec3::new(1., 2.5, 2.));
        schedule.run(&mut world);
        assert!(world.resource::<LiveDecoratorArchivist>().culling.allow_any);
        world.resource_mut::<CameraCursorRay>().origin = Vec3::new(7., 10., 9.);
        interact(&mut world, name, 4, Vec3::new(1., 2.5, 2.));
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<TerrainEditingData>().strokes[&StrokeId(7)].points,
            vec![[4., 1., 5.], [7., 1., 2.]]
        );
        assert_eq!(
            world.resource::<TerrainEditingData>().strokes[&StrokeId(99)].points,
            vec![[80., 0., 80.]]
        );
        assert_eq!(world.resource::<Events<SetTerrainStrokePtsCmd>>().len(), 1);
        world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
            Vec2::ZERO,
            1.,
        ))));
        world.resource_mut::<CameraCursorRay>().origin = Vec3::new(20., 10., 20.);
        interact(&mut world, name, 4, Vec3::ONE);
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<Events<SetTerrainStrokePtsCmd>>().len(),
            1,
            "越界不提交点集"
        );
        assert_eq!(world.resource::<Events<DisplayOutOfBorder>>().len(), 1);
        assert_eq!(
            world.resource::<Events<FeedbackInputOutOfBorder>>().len(),
            1
        );
        interact(&mut world, name, 1, Vec3::ONE);
        schedule.run(&mut world);
        let edit = world
            .resource::<PendingTerrainHistory>()
            .edit
            .clone()
            .unwrap();
        assert_eq!(
            edit,
            TerrainEdit::MoveTerrainStroke {
                id: StrokeId(7),
                original_stroke_pts: original.clone(),
                move_vec: [3., 3.]
            }
        );
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(5).unwrap();
        world.insert_resource(history);
        world
            .run_system_once(super::super::deferred_terrain_history::history_record_terrain::<()>)
            .unwrap();
        let recording = world.resource::<History<()>>().pending().unwrap();
        assert_eq!(recording.edits.len(), 1);
        assert!(matches!(&recording.edits[0], EditType::Terrain(payload) if **payload == edit));
        assert!(world.resource::<PendingTerrainHistory>().is_empty());
        *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::None);
        for undo in [true, false] {
            let effects = crate::resources::history::terrain_edit::apply(
                &edit,
                &mut world.resource_mut::<TerrainEditingData>(),
                1.,
                undo,
            );
            for points in effects.point_sets {
                world.send_event(points);
            }
            schedule.run(&mut world);
            let expected = if undo {
                original.clone()
            } else {
                vec![[4., 1., 5.], [7., 1., 2.]]
            };
            assert_eq!(
                world.resource::<TerrainEditingData>().strokes[&StrokeId(7)].points,
                expected
            );
        }
    }
}
