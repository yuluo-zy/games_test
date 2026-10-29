//! B `systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs`:
//! rectangle orientation editing.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs.md`
//! (O, 0x140ad8000 / do_ui_edit_rectangle_rotate 0x140ad8be0). The rotation
//! angle uses the full eight-coefficient acos with its π branch. The original
//! Wip variant1, time interpolation, stepped border limit and final effects
//! are preserved through independent producer and consumer systems.

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use utils::geometry::intersect::Triangle;

/// 8 fast-acos coefficients (0x142ae0d44..0x142ae0d60).
const ACOS_COEFFS: [u32; 8] = [
    0xbaa57a2c, 0x3bda90c5, 0xbc8bfc66, 0x3cfd10f8, 0xbd4d8392, 0x3db63a9e, 0xbe5bbfca, 0x3fc90fda,
];
/// pi (0x40490fdb @0x1429edba4).

/// B 0x140ad9f10: skewed box trigger volume.
///
/// Two input records are 4 `Vec3` corners each (0x30 B; fresh disasm copies
/// `+0x0`, `+0x8`, `+0xc`, `+0x14`, `+0x18`, `+0x20`, `+0x24`, `+0x2c`
/// verbatim). The output is 0xf0 B: eight corner positions (arg3's four then
/// arg2's four) followed by 36 `u32` triangle indices, each offset by `base`.
/// Every index is emitted with `LEA r9d,[r9+n]`, i.e. 32-bit wrapping.
pub fn skewed_box_trigger_vol(
    loop_a: &[Vec3; 4],
    loop_b: &[Vec3; 4],
    base: u32,
) -> ([Vec3; 8], [u32; 36]) {
    let vertices = [
        loop_b[0], loop_b[1], loop_b[2], loop_b[3], loop_a[0], loop_a[1], loop_a[2], loop_a[3],
    ];
    let b = base;
    let indices = [
        b,
        b.wrapping_add(1),
        b.wrapping_add(2),
        b.wrapping_add(2),
        b.wrapping_add(3),
        b,
        b.wrapping_add(4),
        b.wrapping_add(5),
        b.wrapping_add(6),
        b.wrapping_add(6),
        b.wrapping_add(7),
        b.wrapping_add(4),
        b.wrapping_add(3),
        b.wrapping_add(2),
        b.wrapping_add(6),
        b.wrapping_add(6),
        b.wrapping_add(7),
        b.wrapping_add(3),
        b.wrapping_add(1),
        b,
        b.wrapping_add(4),
        b.wrapping_add(4),
        b.wrapping_add(5),
        b.wrapping_add(1),
        b.wrapping_add(2),
        b.wrapping_add(1),
        b.wrapping_add(5),
        b.wrapping_add(5),
        b.wrapping_add(6),
        b.wrapping_add(2),
        b,
        b.wrapping_add(3),
        b.wrapping_add(7),
        b.wrapping_add(7),
        b.wrapping_add(4),
        b,
    ];
    (vertices, indices)
}

/// 原 B 140ada0e0：按六个面的原顶点与绕序生成盒体的十二个真实三角形。
/// 维度可为负、零或非有限值；原入口不修正这些输入。
pub fn box_trigger_volume(
    width: f32,
    height: f32,
    depth: f32,
    transform: &crate::utils::components::transform::Transform,
) -> Vec<Triangle> {
    let x0 = width * -0.5;
    let x1 = width * 0.5;
    let y0 = height * -0.5;
    let y1 = height * 0.5;
    let z0 = depth * -0.5;
    let z1 = depth * 0.5;
    // 原表保留每个面的独立顶点；不能改为八点拓扑后重排三角形。
    let positions = [
        Vec3::new(x0, y0, z1),
        Vec3::new(x1, y0, z1),
        Vec3::new(x1, y1, z1),
        Vec3::new(x0, y1, z1),
        Vec3::new(x0, y1, z0),
        Vec3::new(x1, y1, z0),
        Vec3::new(x1, y0, z0),
        Vec3::new(x0, y0, z0),
        Vec3::new(x1, y0, z0),
        Vec3::new(x1, y1, z0),
        Vec3::new(x1, y1, z1),
        Vec3::new(x1, y0, z1),
        Vec3::new(x0, y0, z1),
        Vec3::new(x0, y1, z1),
        Vec3::new(x0, y1, z0),
        Vec3::new(x0, y0, z0),
        Vec3::new(x1, y1, z0),
        Vec3::new(x0, y1, z0),
        Vec3::new(x0, y1, z1),
        Vec3::new(x1, y1, z1),
        Vec3::new(x1, y0, z1),
        Vec3::new(x0, y0, z1),
        Vec3::new(x0, y0, z0),
        Vec3::new(x1, y0, z0),
    ];
    const INDICES: [usize; 36] = [
        0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17,
        18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
    ];
    box_trigger_triangles(&INDICES, &transform.compute_matrix(), &positions)
}

/// 原内部映射闭包保留分量乘加顺序，第三点的最后一次加法操作数相反。
fn box_trigger_triangles(
    indices: &[usize],
    transform: &glam::Mat4,
    positions: &[Vec3; 24],
) -> Vec<Triangle> {
    let x_axis = transform.x_axis.truncate();
    let y_axis = transform.y_axis.truncate();
    let z_axis = transform.z_axis.truncate();
    let translation = transform.w_axis.truncate();
    indices
        .chunks(3)
        .map(|chunk| {
            let point = |slot: usize| {
                let local = positions[chunk[slot]];
                let xy = y_axis * local.y + x_axis * local.x;
                let xyz = z_axis * local.z + xy;
                if slot == 2 {
                    translation + xyz
                } else {
                    xyz + translation
                }
            };
            Triangle::new(point(0), point(1), point(2))
        })
        .collect()
}

/// B原16B Local：一次拖拽期间锁定最近相机角点，结束后才允许重新选择。
#[derive(Clone, Copy, Debug)]
pub struct ClosestCornerCache {
    pub corner: usize,
    pub recalculate: bool,
}
impl Default for ClosestCornerCache {
    fn default() -> Self {
        Self {
            corner: 0,
            recalculate: true,
        }
    }
}

/// B140ad8000原十组参数；真实widget交互写完整DO事件。
#[allow(clippy::too_many_arguments)]
pub fn ui_edit_rectangle_orientation(
    mode: Res<crate::systems::mode_manager::AppMode>,
    walls: Res<crate::resources::walls::public_walls::PublicWalls>,
    mut stream: ResMut<crate::systems::ui::signifiers::UiSignifierStream>,
    ui: Res<crate::systems::ui::resources::ui_state::UiState>,
    camera: Res<crate::systems::camera_rig::MainCamera>,
    ray: Res<crate::resources::mouse_terrain_raycast::CameraCursorRay>,
    mut moved: EventReader<crate::systems::camera_rig::main_camera::MainCameraMoved>,
    mut cache: Local<ClosestCornerCache>,
    mut events: EventWriter<DoUiEditRectangleRotate>,
    mut commands: Commands,
) {
    use super::ui_modes::edit_orientation::UiRectangleOrientationEdit;
    use crate::systems::{
        mode_manager::{AppMode, EditSubject},
        ui::resources::ui_state::DragState,
        wall::valid_enclosure::ShapeParameters,
    };
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
        if selected && cache.recalculate {
            let eye = camera.eye_position();
            let eye = Vec2::new(eye.x, eye.z);
            let distance = |corner| {
                let d = rectangle.get_point(corner) - eye;
                (d.y * d.y + d.x * d.x).sqrt()
            };
            let first = distance(0);
            let second = distance(1);
            let mut corner = usize::from(second < first);
            let mut closest = if first <= second { first } else { second };
            for next in 2..4 {
                let d = distance(next);
                if d < closest {
                    closest = d;
                    corner = next;
                }
            }
            cache.corner = corner;
        }
        for corner in 0..4 {
            if selected && cache.corner != corner {
                continue;
            }
            let mut widget = UiRectangleOrientationEdit::new(wall, rectangle, corner, selected);
            let previous = ui.widget_context(widget.widget_id());
            if !selected
                && !super::ui_edit_rectangle_dims::candidate_hit(rectangle, &ray)
                && !previous.is_active()
            {
                continue;
            }
            let context = widget.widget(&ui, state, camera_moved);
            if camera_moved && !selected && context.dragged() == 2 {
                continue;
            }
            widget.signifier(&context, &mut stream);
            if !selected && context.hovered() == 1 {
                commands.queue(move |world: &mut World| {
                    world.send_event(crate::systems::wall::wall_highlight::HighlightWallCmd {
                        wall,
                        ty: 2,
                    });
                });
            }
            cache.recalculate = ui.hover_byte() < 2 || context.dragged() == 2;
            let intersection = context.intersecting();
            if u32::from_le_bytes(intersection[..4].try_into().unwrap()) != 2 {
                events.write(DoUiEditRectangleRotate {
                    intersection,
                    wall,
                    rectangle,
                    corner,
                    phase: DragState(context.dragged()),
                });
                // B140ad898c..8997写出唯一交互后直接结束；继续枚举会破坏角点锁存。
                return;
            }
        }
    }
}

/// 原旋转事件完整载荷，和尺寸事件保持各自独立reader。
#[derive(Event, Clone, Copy, Debug)]
pub struct DoUiEditRectangleRotate {
    pub intersection: [u8; 56],
    pub wall: crate::resources::walls::WallId,
    pub rectangle: utils::geometry::Rectangle2d,
    pub corner: usize,
    pub phase: crate::systems::ui::resources::ui_state::DragState,
}

/// 原glam acos多项式必须有负值π分支，不能将abs(cos)多项式直接当acos。
pub fn original_rectangle_signed_angle(a: Vec2, b: Vec2) -> f32 {
    let an = 1.0 / (a.y * a.y + a.x * a.x).sqrt();
    let bn = 1.0 / (b.y * b.y + b.x * b.x).sqrt();
    let a = Vec2::new(a.x * an, a.y * an);
    let b = Vec2::new(b.x * bn, b.y * bn);
    let cos = (b.x * a.x + b.y * a.y) / ((b.y * b.y + b.x * b.x) * (a.y * a.y + a.x * a.x)).sqrt();
    let x = cos.abs();
    let root = 1.0 - x;
    let root = if root.is_nan() || root >= 0.0 {
        root
    } else {
        0.0
    };
    let mut polynomial = f32::from_bits(ACOS_COEFFS[0]);
    for coefficient in &ACOS_COEFFS[1..] {
        polynomial = polynomial * x + f32::from_bits(*coefficient);
    }
    let angle = polynomial * root.sqrt();
    let angle = if cos < 0.0 {
        f32::from_bits(0x40490fdb) - angle
    } else {
        angle
    };
    let cross = b.y * a.x - a.y * b.x;
    let sign = if cross.is_nan() {
        f32::from_bits(0x7fc00000)
    } else {
        f32::from_bits((cross.to_bits() & 0x80000000) | 0x3f800000)
    };
    sign * angle
}
fn original_rotate_axis(axis: Vec2, angle: f32) -> Vec2 {
    let sin = angle.sin();
    let cos = angle.cos();
    Vec2::new(axis.y * (-sin) + axis.x * cos, axis.y * cos + axis.x * sin)
}

/// B140ad8be0完整held分支由原ASM恢复，旧反编译在jumptable错误后遗漏了此分支。
#[allow(clippy::too_many_arguments)]
pub fn do_ui_edit_rectangle_rotate(
    walls: Res<crate::resources::walls::public_walls::PublicWalls>,
    mut wip: ResMut<super::WipRectangle>,
    mut edit: EventWriter<super::super::EditRectangleCmd>,
    mut finish: EventWriter<super::super::FinishEditRectangleCmd>,
    time: Res<crate::resources::time::GameTime>,
    ray: Res<crate::resources::mouse_terrain_raycast::CameraCursorRay>,
    border: Res<crate::resources::glade_border::GladeBorder>,
    mut display: EventWriter<crate::resources::glade_border::DisplayOutOfBorder>,
    mut feedback: EventWriter<crate::resources::glade_border::FeedbackInputOutOfBorder>,
    mut trees: EventWriter<crate::resources::tree_storage::HideTreesinAreaCmd>,
    mut pins: EventWriter<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>,
    mut events: EventReader<DoUiEditRectangleRotate>,
) {
    use super::super::utils::shape_signifier_lerp::{ShapeVal, lerp_shape};
    use crate::resources::{
        glade_border::{BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder},
        tree_storage::{HideTreesinAreaCmd, TreeHideArea},
    };
    let Some(event) = events.read().last() else {
        return;
    };
    if u32::from_le_bytes(event.intersection[..4].try_into().unwrap()) == 2 {
        return;
    }
    let wall = walls.get(event.wall).expect("矩形旋转墙尚未发布");
    if event.phase.0 == 2 {
        return;
    }
    if event.phase.is_started() {
        let y = f32::from_le_bytes(event.intersection[40..44].try_into().unwrap());
        let point = ray.intersect_with_xz_plane(y);
        let rectangle = event.rectangle;
        let angle = original_rectangle_signed_angle(
            rectangle.get_point(event.corner) - rectangle.center,
            Vec2::new(point.x, point.z) - rectangle.center,
        );
        wip.begin_rotation(event.wall, event.corner, rectangle, y, angle);
        return;
    }
    if event.phase.is_finished() {
        finish.write(super::super::FinishEditRectangleCmd);
        trees.write(HideTreesinAreaCmd {
            area: TreeHideArea::Rectangle(event.rectangle),
            height_limit: wall.authority.arch_height(),
            expansion: 1.5,
            finished: true,
        });
        return;
    }
    assert_eq!(wip.tag(), 1, "矩形旋转held缺少Started状态");
    let before = wip.current();
    let point = ray.intersect_with_xz_plane(wip.scalar(4));
    let corner = u64::from_le_bytes(wip.0[24..32].try_into().unwrap()) as usize;
    let angle = original_rectangle_signed_angle(
        before.get_point(corner) - before.center,
        Vec2::new(point.x, point.z) - before.center,
    ) - wip.scalar(8);
    let angle = lerp_shape(ShapeVal::Circle(0.0), ShapeVal::Circle(angle), time.delta).get_radius();
    let mut after = before;
    after.axis = original_rotate_axis(before.axis, angle);
    if !border.is_shape_inside(BorderShape::Rectangle(after)) {
        display.write(DisplayOutOfBorder);
        feedback.write(FeedbackInputOutOfBorder);
        // 原版按正向0.1弧度步进，每次先保存前一有效值；负角越界直接保持before。
        let mut step = 0.0;
        let mut candidate = before;
        loop {
            after = candidate;
            if !(angle > step) {
                break;
            }
            step += 0.1;
            candidate.axis = original_rotate_axis(before.axis, step);
            if !border.is_shape_inside(BorderShape::Rectangle(candidate)) {
                break;
            }
        }
    }
    pins.write(
        crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter::rectangle(
            after,
            wall.authority.arch_height(),
            wall.max_y_gable(),
            Some(event.wall),
        ),
    );
    trees.write(HideTreesinAreaCmd {
        area: TreeHideArea::Rectangle(after),
        height_limit: wall.authority.arch_height(),
        expansion: 1.5,
        finished: false,
    });
    let d = after.axis - before.axis;
    if (d.y * d.y + d.x * d.x).sqrt() > 0.0 {
        let previous_corner = before.get_point(corner);
        let current_corner = after.get_point(corner);
        let d = current_corner - previous_corner;
        wip.set_current(after);
        edit.write(super::super::EditRectangleCmd {
            wall: event.wall,
            center: after.center,
            dimensions: after.size,
            orientation: after.axis,
            position: Vec3::new(after.center.x, wip.scalar(4), after.center.y),
            speed: (d.y * d.y + d.x * d.x).sqrt(),
            rotation: true,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_rectangle_signed_angle_native_bits() {
        let cases: &[([u32; 2], [u32; 2], u32)] = include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../reports/module-recovery-2026-10-23/m01-rectangle-angle-native-cases.rs"
        ));
        for (index, (a, b, expected)) in cases.iter().copied().enumerate() {
            let angle = original_rectangle_signed_angle(
                Vec2::from_array(a.map(f32::from_bits)),
                Vec2::from_array(b.map(f32::from_bits)),
            );
            assert_eq!(angle.to_bits(), expected, "原旋转角片段差异case {index}");
        }
    }

    #[test]
    fn skewed_box_volume_keeps_corner_order_and_fixed_index_pattern() {
        let a = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];
        let b = [
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(0.0, 1.0, 1.0),
        ];
        let (vertices, indices) = skewed_box_trigger_vol(&a, &b, 10);
        // arg3 (loop_b) first, then arg2 (loop_a).
        assert_eq!(vertices[0], b[0]);
        assert_eq!(vertices[3], b[3]);
        assert_eq!(vertices[4], a[0]);
        assert_eq!(vertices[7], a[3]);
        // First and last triangles.
        assert_eq!(&indices[..3], &[10, 11, 12]);
        assert_eq!(&indices[33..36], &[17, 14, 10]);
        // 12 triangles over 8 vertices, all within [base, base+8).
        assert_eq!(indices.len(), 36);
        assert!(indices.iter().all(|&i| (10..18).contains(&i)));
        // 32-bit wrapping like the native LEA.
        let (_, wrapped) = skewed_box_trigger_vol(&a, &b, u32::MAX);
        assert_eq!(wrapped[1], 0);
    }

    #[test]
    fn box_trigger_volume_transforms_triangle_positions() {
        let mut positions = [Vec3::ZERO; 24];
        positions[0] = Vec3::new(1.0, 2.0, 3.0);
        positions[1] = Vec3::new(4.0, 5.0, 6.0);
        positions[2] = Vec3::new(7.0, 8.0, 9.0);
        let shifted = glam::Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
        let triangles = box_trigger_triangles(&[0, 1, 2], &shifted, &positions);
        assert_eq!(triangles.len(), 1);
        assert_eq!(triangles[0].points[0], Vec3::new(11.0, 2.0, 3.0));
        assert_eq!(triangles[0].points[2], Vec3::new(17.0, 8.0, 9.0));
        // Two chunks -> two triangles.
        let two = box_trigger_triangles(&[0, 1, 2, 2, 1, 0], &glam::Mat4::IDENTITY, &positions);
        assert_eq!(two.len(), 2);
        assert_eq!(two[1].points[0], Vec3::new(7.0, 8.0, 9.0));
    }

    #[test]
    #[should_panic]
    fn box_trigger_volume_rejects_out_of_range_vertex() {
        let positions = [Vec3::ZERO; 24];
        let _ = box_trigger_triangles(&[0, 1, 24], &glam::Mat4::IDENTITY, &positions);
    }
    #[test]
    fn box_trigger_volume_has_all_native_faces_and_srt() {
        use crate::utils::components::transform::Transform;
        // 原24点/36索引独立标量lift在尺寸2/4/6下的十二个三角形。
        let expected = [
            [[-1., -2., 3.], [1., -2., 3.], [1., 2., 3.]],
            [[1., 2., 3.], [-1., 2., 3.], [-1., -2., 3.]],
            [[-1., 2., -3.], [1., 2., -3.], [1., -2., -3.]],
            [[1., -2., -3.], [-1., -2., -3.], [-1., 2., -3.]],
            [[1., -2., -3.], [1., 2., -3.], [1., 2., 3.]],
            [[1., 2., 3.], [1., -2., 3.], [1., -2., -3.]],
            [[-1., -2., 3.], [-1., 2., 3.], [-1., 2., -3.]],
            [[-1., 2., -3.], [-1., -2., -3.], [-1., -2., 3.]],
            [[1., 2., -3.], [-1., 2., -3.], [-1., 2., 3.]],
            [[-1., 2., 3.], [1., 2., 3.], [1., 2., -3.]],
            [[1., -2., 3.], [-1., -2., 3.], [-1., -2., -3.]],
            [[-1., -2., -3.], [1., -2., -3.], [1., -2., 3.]],
        ];
        let triangles = box_trigger_volume(2., 4., 6., &Transform::identity());
        assert_eq!(triangles.len(), 12);
        for (actual, points) in triangles.iter().zip(expected) {
            assert_eq!(actual.points, points.map(Vec3::from_array));
        }
        let transform = Transform {
            rotation: glam::Quat::from_xyzw(0., 1., 0., 0.),
            translation: Vec3::new(10., 20., 30.),
            scale: Vec3::new(2., 3., 4.),
        };
        let triangles = box_trigger_volume(2., 4., 6., &transform);
        for (actual, points) in triangles.iter().zip(expected) {
            assert_eq!(
                actual.points,
                points.map(|[x, y, z]| Vec3::new(10. - 2. * x, 20. + 3. * y, 30. - 4. * z))
            );
        }
    }

    #[test]
    fn box_trigger_volume_keeps_reflection_degeneracy_and_nonfinite_lanes() {
        use crate::utils::components::transform::Transform;
        let identity = Transform::identity();
        let positive = box_trigger_volume(2., 4., 6., &identity);
        let reflected = box_trigger_volume(-2., 4., 6., &identity);
        for (actual, original) in reflected.iter().zip(&positive) {
            assert_eq!(
                actual.points,
                original.points.map(|p| Vec3::new(-p.x, p.y, p.z))
            );
        }
        let flat = box_trigger_volume(0., 4., 6., &identity);
        assert_eq!(flat.len(), 12);
        assert!(flat.iter().flat_map(|t| t.points).all(|p| p.x == 0.));
        let nonfinite = box_trigger_volume(f32::INFINITY, 4., 6., &identity);
        assert_eq!(nonfinite.len(), 12);
        assert!(
            nonfinite
                .iter()
                .flat_map(|t| t.points)
                .all(|p| !p.is_finite())
        );
    }

    #[test]
    #[should_panic]
    fn box_trigger_triangle_mapper_keeps_short_chunk_failure() {
        let _ = box_trigger_triangles(&[0, 1], &glam::Mat4::IDENTITY, &[Vec3::ZERO; 24]);
    }
}
