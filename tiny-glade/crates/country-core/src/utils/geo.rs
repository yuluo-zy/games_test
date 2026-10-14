//! `country_core::utils::geo` — 曲线/点集到网格的几何辅助。
//!
//! | 函数 | 地址 | 级别 |
//! |---|---|---|
//! | `mesh_from_pts` | 0x140a6c120 | O |
//! | `update_curve_mesh`（4 个单态化，同一函数体） | 0x14157eac0 / 0x14157f130 / 0x14157eda0 / 0x14157ef70 | O |
//! | `rewire_enclosed_curve` | 0x140a6ba50 | O |
//!
//! 常量（EXE 字节）：白色 `0x14292e6b0` = `[1.0, 1.0]` 加上栈上 `0x3f800000`，即 `Vec3::ONE`；
//! `0x142925904` 低 4 字节 = 1.0；`0x142925984` 低 4 字节 = `0x3dcccccd` = 0.1（端点去重阈值）。

use glam::{Vec2, Vec3};
use utils::curve::Curve2;

use crate::render::mesh::{ATTR_COLOR, ATTR_POSITION, Mesh};

/// 端点去重阈值。O：`0x142925984` = `0x3dcccccd`。
pub const ENDPOINT_EPSILON: f32 = 0.1;

fn fill_polyline_attributes(mesh: &mut Mesh, points: Vec<Vec3>) {
    let count = points.len();
    mesh.set_attribute(ATTR_POSITION, points);
    mesh.set_attribute(ATTR_COLOR, vec![Vec3::ONE; count]);
    mesh.set_indices((0..count as u32).collect());
}

/// O 0x140a6c120：`(label, pts)` → 带顺序索引的折线网格。
///
/// 先 `assert!(pts_len > 0)`；`Vertex_Position` = 点的拷贝，`Vertex_Color` = 全白，
/// 索引 `0..n`；`Mesh` 的 `label` 为入参。
pub fn mesh_from_pts(label: &'static str, pts: &[Vec3]) -> Mesh {
    assert!(!pts.is_empty(), "assertion failed: pts_len > 0");
    let mut mesh = Mesh::new(label);
    fill_polyline_attributes(&mut mesh, pts.to_vec());
    mesh
}

/// O 0x14157f130（及三个同体实例）：取到资产里的网格后整体改写位置/颜色/索引。
///
/// `mesh` 是 `Assets<Mesh>` 按句柄查询的结果；查不到 → panic `MEOW####`（`0x142c77ab9`，8 字节）。
/// 点列先于属性写入收集；空点列由 `set_attribute` 的非空断言 panic。
pub fn update_curve_mesh(mesh: Option<&mut Mesh>, points: impl IntoIterator<Item = Vec3>) {
    let Some(mesh) = mesh else {
        panic!("MEOW####");
    };
    fill_polyline_attributes(mesh, points.into_iter().collect());
}

/// O 0x140a6ba50：以 `u = split` 把曲线一分为二，调换两半的先后顺序后重新建曲线。
///
/// * `split == 0.0` 或 `split == 1.0` → `None`；
/// * 取 `[0, split]` 与 `[split, 1]` 两段，任一段长度 < 0.1 → `None`；
/// * 前段末两点距离 < 0.1 时丢弃末点（倒数第二点改写为末点）；
///   后段首两点距离 < 0.1 时丢弃首点（保留原首点位置）；
///   原曲线首尾点距离 < 0.1 时再丢弃后段的最后一个点；
/// * 结果点列 = 后段 + 前段，`try_new_from_points(.., false)`，失败直接 unwrap panic。
pub fn rewire_enclosed_curve(curve: &Curve2, split: f32) -> Option<Curve2> {
    if split == 0.0 || split == 1.0 {
        return None;
    }
    let mut head = curve.crashy_slice(0.0, split);
    let mut tail = curve.crashy_slice(split, 1.0);
    if head.length < ENDPOINT_EPSILON || tail.length < ENDPOINT_EPSILON {
        return None;
    }

    let n = head.points.len();
    assert!(n >= 2, "index out of bounds");
    let last = head.points[n - 1];
    let mut head_len = n;
    if head.points[n - 1].distance(head.points[n - 2]) < ENDPOINT_EPSILON {
        head.points[n - 2] = last;
        head_len = n - 1;
    }

    assert!(tail.points.len() >= 2, "index out of bounds");
    let mut tail_len = tail.points.len();
    if tail.points[0].distance(tail.points[1]) < ENDPOINT_EPSILON {
        tail.points[1] = tail.points[0];
        tail.points.remove(0);
        tail_len -= 1;
    }

    let first = *curve.points.first().expect("empty curve");
    let last_point = *curve.points.last().expect("empty curve");
    if first.distance(last_point) < ENDPOINT_EPSILON {
        tail_len -= 1;
    }

    let mut points: Vec<Vec2> = tail.points[..tail_len].to_vec();
    points.extend_from_slice(&head.points[..head_len]);
    Some(Curve2::try_new_from_points(points, false).unwrap())
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
    #[should_panic(expected = "assertion failed: pts_len > 0")]
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
    #[should_panic(expected = "MEOW####")]
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
}
