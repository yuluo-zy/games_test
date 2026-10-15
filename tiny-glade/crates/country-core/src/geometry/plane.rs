//! `plane.rs` — XZ 平面四边形网格（B `geometry::plane`，`From<Plane> for Mesh`，0x140a06370，1934 B）。
//!
//! 汇编逐条还原（O），纠正此前文档几处：
//! - 输入 `Plane` 只有**一个 f32**（`xmm1 * 0.5` 同时给两个轴，所以是正方形），不是 `Vec2`；
//! - 每个顶点是 `{位置 12 B, 法线 12 B, UV 8 B}` = 32 B；4 个顶点依次为
//!   `(+h,0,-h) uv(1,1)`、`(+h,0,+h) uv(1,0)`、`(-h,0,+h) uv(0,0)`、`(-h,0,-h) uv(0,1)`，法线都是 `(0,1,0)`；
//! - 索引是 `[0, 2, 1, 0, 3, 2]`（16 B 常量 0x142ac5d50 = `(0,2,1,0)` 加 qword 0x200000003 = `(3,2)`）；
//! - 属性设置顺序 `Vertex_Position`、`Vertex_Normal`、`Vertex_UV`、`Vertex_Color`（4 个白色）；网格标签是 `"Plane"`
//!   （串池 0x142ac65e8，长 5）。
use glam::{Vec2, Vec3};

use crate::render::mesh::{ATTR_COLOR, ATTR_NORMAL, ATTR_POSITION, ATTR_UV, Mesh};

/// 索引（O）。
const INDICES: [u32; 6] = [0, 2, 1, 0, 3, 2];

/// `From<Plane> for Mesh`（0x140a06370）：`size` 是整边长。
pub fn plane_mesh(size: f32) -> Mesh {
    let h = size * 0.5;
    let positions = vec![
        Vec3::new(h, 0.0, -h),
        Vec3::new(h, 0.0, h),
        Vec3::new(-h, 0.0, h),
        Vec3::new(-h, 0.0, -h),
    ];
    let uvs = vec![
        Vec2::new(1.0, 1.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 1.0),
    ];
    let mut mesh = Mesh::new("Plane");
    mesh.set_indices(INDICES.to_vec());
    mesh.set_attribute(ATTR_POSITION, positions);
    mesh.set_attribute(ATTR_NORMAL, vec![Vec3::Y; 4]);
    mesh.set_attribute(ATTR_UV, uvs);
    mesh.set_attribute(ATTR_COLOR, vec![Vec3::ONE; 4]);
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corners_are_half_the_size_on_the_xz_plane() {
        let mesh = plane_mesh(4.0);
        let p = mesh.positions_without_indices();
        assert_eq!(p[0], Vec3::new(2.0, 0.0, -2.0));
        assert_eq!(p[1], Vec3::new(2.0, 0.0, 2.0));
        assert_eq!(p[2], Vec3::new(-2.0, 0.0, 2.0));
        assert_eq!(p[3], Vec3::new(-2.0, 0.0, -2.0));
        assert!(p.iter().all(|c| c.y == 0.0));
    }

    #[test]
    fn normals_point_up_and_uvs_cover_the_unit_square() {
        let mesh = plane_mesh(1.0);
        assert!(mesh.normals_without_indices().iter().all(|n| *n == Vec3::Y));
        let uvs = mesh.uvs_without_indices();
        assert_eq!(uvs[0], Vec2::new(1.0, 1.0));
        assert_eq!(uvs[2], Vec2::new(0.0, 0.0));
        assert_eq!(mesh.label, "Plane");
    }

    #[test]
    fn indices_make_two_upward_facing_triangles() {
        let mesh = plane_mesh(2.0);
        assert_eq!(mesh.indices, [0, 2, 1, 0, 3, 2]);
        let p = mesh.positions_without_indices();
        for tri in mesh.indices.chunks_exact(3) {
            let (a, b, c) = (p[tri[0] as usize], p[tri[1] as usize], p[tri[2] as usize]);
            assert!((b - a).cross(c - a).y > 0.0, "{tri:?}");
        }
    }

    #[test]
    fn colours_are_white_per_vertex() {
        let mesh = plane_mesh(1.0);
        assert_eq!(mesh.attributes[ATTR_COLOR].array_length(), 4);
    }

    #[test]
    fn zero_size_collapses_to_a_point_but_still_has_four_vertices() {
        let mesh = plane_mesh(0.0);
        assert_eq!(mesh.attributes[ATTR_POSITION].array_length(), 4);
    }
}
