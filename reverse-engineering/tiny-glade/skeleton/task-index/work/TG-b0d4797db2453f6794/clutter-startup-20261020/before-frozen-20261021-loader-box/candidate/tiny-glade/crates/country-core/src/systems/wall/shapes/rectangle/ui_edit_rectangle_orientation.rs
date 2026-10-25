//! B `systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs`:
//! rectangle orientation editing.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs.md`
//! (O, 0x140ad8000 / do_ui_edit_rectangle_rotate 0x140ad8be0). The rotation
//! angle uses the fast 8-coefficient acos polynomial; the sign comes from the
//! cross product. The result updates the UiState80 (variant 2) and emits an
//! EditRectangleCmd plus FinishEditRectangleCmd.

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::get_rectangle_orientation;
use super::finish_edit_rectangle::RectangleEditUiState;
use crate::resources::walls::InnerWalls;
use utils::geometry::intersect::Triangle;

/// 8 fast-acos coefficients (0x142ae0d44..0x142ae0d60).
const ACOS_COEFFS: [u32; 8] = [
    0x3bda90c5, 0x3cd90a90, 0x3c8bfc66, 0x3dfd10f8, 0xbd4d8392, 0x3db63a9e, 0xbe5bbfca, 0x3fc90fda,
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

/// Rotation input (UI producer 待接线).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct RectangleOrientationEditInput {
    pub wall: crate::resources::walls::WallId,
    pub from_axis: Vec2,
    pub cursor: Vec2,
    pub submitting: bool,
}

/// B fast acos: `sqrt(1 - |x|) * P(x)` with the observed coefficients.
fn fast_acos(x: f32) -> f32 {
    let x = x.abs();
    let poly = ACOS_COEFFS
        .iter()
        .fold(0.0f32, |acc, &coef| acc * x + f32::from_bits(coef));
    poly * (1.0 - x).sqrt()
}

/// B 0x140ad8be0: rotate the rectangle toward the cursor direction.
pub fn do_ui_edit_rectangle_rotate(
    wall: crate::resources::walls::WallId,
    from_axis: Vec2,
    cursor: Vec2,
    state: &mut RectangleEditUiState,
    preview: &mut super::super::PreviewRectangle,
    commands: &mut EventWriter<super::super::EditRectangleCmd>,
    finish: &mut EventWriter<super::super::FinishEditRectangleCmd>,
) {
    let to_axis = if cursor.length_squared() > 0.0 {
        cursor.normalize()
    } else {
        from_axis
    };
    let cos = (from_axis.dot(to_axis))
        / ((from_axis.length_squared() * to_axis.length_squared()).max(1e-12).sqrt());
    let angle = fast_acos(cos);
    let cross = from_axis.x * to_axis.y - from_axis.y * to_axis.x;
    let signed = if cross < 0.0 { -angle } else { angle };
    let orientation = get_rectangle_orientation(signed);

    preview.orientation = orientation;

    state.0.fill(0);
    state.0[..4].copy_from_slice(&2u32.to_le_bytes());
    state.0[0x08..0x10].copy_from_slice(&wall.0.to_le_bytes());
    state.0[0x18..0x1c].copy_from_slice(&signed.to_le_bytes());

    commands.write(super::super::EditRectangleCmd {
        wall,
        center: preview.center,
        dimensions: preview.dimensions,
        orientation,
    });
    finish.write(super::super::FinishEditRectangleCmd { wall });
}

/// B 0x140ad8000 (system shell).
pub fn ui_edit_rectangle_orientation(
    input: Res<RectangleOrientationEditInput>,
    inner: Res<InnerWalls>,
    mut state: ResMut<RectangleEditUiState>,
    mut preview: ResMut<super::super::PreviewRectangle>,
    mut commands: EventWriter<super::super::EditRectangleCmd>,
    mut finish: EventWriter<super::super::FinishEditRectangleCmd>,
) {
    if !input.submitting {
        return;
    }
    if !inner.contains(input.wall) {
        return;
    }
    do_ui_edit_rectangle_rotate(
        input.wall,
        input.from_axis,
        input.cursor,
        &mut state,
        &mut preview,
        &mut commands,
        &mut finish,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
