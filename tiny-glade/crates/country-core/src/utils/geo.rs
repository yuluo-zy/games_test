//! 折线网格构建和闭合曲线起点调整。曲线采样/切片复用共享 Curve2。

use crate::render::mesh::{ATTR_COLOR, ATTR_POSITION, Mesh};
use glam::{Vec2, Vec3};
use utils::curve::Curve2;

/// 合并临近端点的世界空间距离。
pub const ENDPOINT_EPSILON: f32 = 0.1;

fn fill_polyline_attributes(mesh: &mut Mesh, points: Vec<Vec3>) {
    let vertex_count = u32::try_from(points.len()).expect("折线顶点数量超过 u32 索引范围");
    mesh.set_attribute(ATTR_POSITION, points);
    mesh.set_attribute(ATTR_COLOR, vec![Vec3::ONE; vertex_count as usize]);
    mesh.set_indices((0..vertex_count).collect());
}

/// 创建白色折线网格；点集必须非空，绘制拓扑由消费方指定。
pub fn mesh_from_pts(label: &'static str, points: &[Vec3]) -> Mesh {
    assert!(!points.is_empty(), "折线网格至少需要一个顶点");
    let mut mesh = Mesh::new(label);
    fill_polyline_attributes(&mut mesh, points.to_vec());
    mesh
}

/// 替换位置、颜色和索引。其他自定义属性由消费方自行更新。
pub fn update_curve_mesh(mesh: Option<&mut Mesh>, points: impl IntoIterator<Item = Vec3>) {
    let mesh = mesh.expect("更新折线时目标网格不存在");
    fill_polyline_attributes(mesh, points.into_iter().collect());
}

/// 二维 X/Y 作为世界 X/Z，世界高度为零。
pub fn x0y(point: Vec2) -> Vec3 {
    x_h_y(point, 0.0)
}

/// 二维 X/Y 作为世界 X/Z，并指定世界高度。
pub fn x_h_y(point: Vec2, height: f32) -> Vec3 {
    Vec3::new(point.x, height, point.y)
}

/// 把闭合曲线的起点移到 split，返回“后半段 + 前半段”。
/// 无效分割、非闭合曲线或过短分段返回 None，不对用户编辑数据 unwrap。
pub fn rewire_enclosed_curve(curve: &Curve2, split: f32) -> Option<Curve2> {
    if !split.is_finite() || split <= 0.0 || split >= 1.0 {
        return None;
    }
    let first = *curve.points.first()?;
    let last = *curve.points.last()?;
    if first.distance(last) >= ENDPOINT_EPSILON {
        return None;
    }
    let mut head = curve.try_slice(0.0, split).ok()?;
    let mut tail = curve.try_slice(split, 1.0).ok()?;
    if head.length < ENDPOINT_EPSILON
        || tail.length < ENDPOINT_EPSILON
        || head.points.len() < 2
        || tail.points.len() < 2
    {
        return None;
    }
    let head_length = head.points.len();
    if head.points[head_length - 1].distance(head.points[head_length - 2]) < ENDPOINT_EPSILON {
        head.points[head_length - 2] = head.points[head_length - 1];
        head.points.pop();
    }
    let tail_start = if tail.points[0].distance(tail.points[1]) < ENDPOINT_EPSILON {
        tail.points[1] = tail.points[0];
        1
    } else {
        0
    };
    // 去掉原闭合接点的重复末点，复用后半段的 Vec 分配。
    let tail_end = tail.points.len() - 1;
    if tail_start > tail_end {
        return None;
    }
    tail.points.copy_within(tail_start..tail_end, 0);
    tail.points.truncate(tail_end - tail_start);
    tail.points.extend_from_slice(&head.points);
    Curve2::try_new_from_points(tail.points, false).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_from_pts_has_white_colours_and_sequential_indices() {
        let pts = [Vec3::X, Vec3::Y, Vec3::Z];
        let mesh = mesh_from_pts("pts", &pts);
        assert_eq!(mesh.label, "pts");
        assert_eq!(mesh.indices, [0, 1, 2]);
        assert_eq!(mesh.positions_without_indices(), &pts);
        assert!(mesh.has_attribute(ATTR_COLOR));
    }

    #[test]
    fn vec2_mappings_put_y_into_the_z_slot() {
        assert_eq!(x0y(Vec2::new(1.0, 2.0)), Vec3::new(1.0, 0.0, 2.0));
        assert_eq!(x_h_y(Vec2::new(1.0, 2.0), 5.0), Vec3::new(1.0, 5.0, 2.0));
    }

    #[test]
    #[should_panic(expected = "折线网格至少需要一个顶点")]
    fn mesh_from_pts_rejects_empty_input() {
        mesh_from_pts("x", &[]);
    }

    #[test]
    fn update_curve_mesh_replaces_existing_contents() {
        let mut mesh = mesh_from_pts("m", &[Vec3::ZERO; 5]);
        update_curve_mesh(Some(&mut mesh), [Vec3::X, Vec3::Y]);
        assert_eq!(mesh.indices, [0, 1]);
        assert_eq!(mesh.positions_without_indices(), &[Vec3::X, Vec3::Y]);
    }

    #[test]
    #[should_panic(expected = "更新折线时目标网格不存在")]
    fn update_curve_mesh_panics_without_a_mesh() {
        update_curve_mesh(None, [Vec3::X]);
    }

    fn square() -> Curve2 {
        Curve2::try_new_from_points(
            vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(4.0, 0.0),
                Vec2::new(4.0, 4.0),
                Vec2::new(0.0, 4.0),
                Vec2::new(0.0, 0.0),
            ],
            false,
        )
        .unwrap()
    }

    #[test]
    fn rewire_rotates_a_closed_loop_to_start_at_the_split() {
        let curve = square();
        let out = rewire_enclosed_curve(&curve, 0.25).unwrap();
        // 在 u=0.25（点 (4,0)）处切开：后段在前，且闭合重复点被去掉。
        assert_eq!(out.points.first(), Some(&Vec2::new(4.0, 0.0)));
        assert_eq!(out.points.last(), Some(&Vec2::new(4.0, 0.0)));
        assert!((out.length - curve.length).abs() < 1e-4);
    }

    #[test]
    fn rewire_refuses_sentinel_and_tiny_pieces() {
        let curve = square();
        assert!(rewire_enclosed_curve(&curve, 0.0).is_none());
        assert!(rewire_enclosed_curve(&curve, 1.0).is_none());
        // 切出的前段只有 0.016 × 16 = 0.256 > 0.1 不算短，0.005 × 16 = 0.08 < 0.1 才算。
        assert!(rewire_enclosed_curve(&curve, 0.005).is_none());
    }

    #[test]
    fn rewire_rejects_invalid_splits_and_open_curves() {
        assert!(rewire_enclosed_curve(&square(), f32::NAN).is_none());
        assert!(rewire_enclosed_curve(&square(), -0.5).is_none());
        let open =
            Curve2::try_new_from_points(vec![Vec2::ZERO, Vec2::X, Vec2::ONE], false).unwrap();
        assert!(rewire_enclosed_curve(&open, 0.5).is_none());
    }
}
