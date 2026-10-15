//! `cube.rs` — 轴对齐盒网格（B `geometry::cube`，`From<Box> for Mesh`，0x140a05530，3643 B）。
//!
//! 位置/法线/UV/索引取自汇编里的栈上常量写入，由符号模拟器逐条还原（不是手抄），再与索引表
//! `0x142ac5cc0..0x142ac5d50`（36 个 u32）核对。证据级别 O。纠正此前文档几处：
//! - 每个顶点是 **位置 12 B + 法线 12 B + UV 8 B = 32 B**（文档写成 `位置、UV、颜色` 的 0x18 B）；
//! - 输入 `Box` 的 6 个 f32 按 **`[x0, x1, y0, y1, z0, z1]`** 排布（三个区间交错），不是 `min.xyz`/`max.xyz`；
//! - 共 **24 个顶点**（6 面 × 4，每面独立法线），不是 8 角；
//! - 属性按 `Vertex_Position`、`Vertex_Normal`、`Vertex_UV`、`Vertex_Color` 的顺序设置（颜色 24 个白色，来自常量
//!   0x142925930 = `(1,1,1,1)`）；索引直接写入结构而不是 `set_indices`；网格标签是 `"Box"`（串池 0x142ac6550）。
//!
//! 表中每个顶点的 8 个数：`[x 选 x0/x1, y 选 y0/y1, z 选 z0/z1, 法线 x, y, z, u, v]`。
use glam::{Vec2, Vec3};

use crate::render::mesh::{ATTR_COLOR, ATTR_NORMAL, ATTR_POSITION, ATTR_UV, Mesh};

/// 轴对齐盒（O：内存布局 `[x0, x1, y0, y1, z0, z1]`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubeBox {
    pub x: [f32; 2],
    pub y: [f32; 2],
    pub z: [f32; 2],
}

/// 24 个顶点（O）。
const VERTICES: [[i8; 8]; 24] = [
    [0, 0, 1, 0, 0, 1, 0, 0],
    [1, 0, 1, 0, 0, 1, 1, 0],
    [1, 1, 1, 0, 0, 1, 1, 1],
    [0, 1, 1, 0, 0, 1, 0, 1],
    [0, 1, 0, 0, 0, -1, 1, 0],
    [1, 1, 0, 0, 0, -1, 0, 0],
    [1, 0, 0, 0, 0, -1, 0, 1],
    [0, 0, 0, 0, 0, -1, 1, 1],
    [1, 0, 0, 1, 0, 0, 0, 0],
    [1, 1, 0, 1, 0, 0, 1, 0],
    [1, 1, 1, 1, 0, 0, 1, 1],
    [1, 0, 1, 1, 0, 0, 0, 1],
    [0, 0, 1, -1, 0, 0, 1, 0],
    [0, 1, 1, -1, 0, 0, 0, 0],
    [0, 1, 0, -1, 0, 0, 0, 1],
    [0, 0, 0, -1, 0, 0, 1, 1],
    [1, 1, 0, 0, 1, 0, 1, 0],
    [0, 1, 0, 0, 1, 0, 0, 0],
    [0, 1, 1, 0, 1, 0, 0, 1],
    [1, 1, 1, 0, 1, 0, 1, 1],
    [1, 0, 1, 0, -1, 0, 0, 0],
    [0, 0, 1, 0, -1, 0, 1, 0],
    [0, 0, 0, 0, -1, 0, 1, 1],
    [1, 0, 0, 0, -1, 0, 0, 1],
];

/// 36 个索引（O：0x142ac5cc0 起的 9×16 B）。
const INDICES: [u32; 36] = [
    0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17, 18,
    18, 19, 16, 20, 21, 22, 22, 23, 20,
];

/// `From<Box> for Mesh`（0x140a05530）。
pub fn cube_mesh(cube: &CubeBox) -> Mesh {
    let mut positions = Vec::with_capacity(24);
    let mut normals = Vec::with_capacity(24);
    let mut uvs = Vec::with_capacity(24);
    for v in VERTICES {
        positions.push(Vec3::new(
            cube.x[v[0] as usize],
            cube.y[v[1] as usize],
            cube.z[v[2] as usize],
        ));
        normals.push(Vec3::new(f32::from(v[3]), f32::from(v[4]), f32::from(v[5])));
        uvs.push(Vec2::new(f32::from(v[6]), f32::from(v[7])));
    }
    let mut mesh = Mesh::new("Box");
    mesh.set_indices(INDICES.to_vec());
    mesh.set_attribute(ATTR_POSITION, positions);
    mesh.set_attribute(ATTR_NORMAL, normals);
    mesh.set_attribute(ATTR_UV, uvs);
    mesh.set_attribute(ATTR_COLOR, vec![Vec3::ONE; 24]);
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit() -> CubeBox {
        CubeBox {
            x: [-1.0, 1.0],
            y: [-2.0, 2.0],
            z: [-3.0, 3.0],
        }
    }

    #[test]
    fn has_24_vertices_36_indices_and_four_attributes() {
        let mesh = cube_mesh(&unit());
        assert_eq!(mesh.label, "Box");
        assert_eq!(mesh.indices.len(), 36);
        for name in [ATTR_POSITION, ATTR_NORMAL, ATTR_UV, ATTR_COLOR] {
            assert_eq!(mesh.attributes[name].array_length(), 24, "{name}");
        }
        assert!(mesh.indices.iter().all(|i| *i < 24));
    }

    #[test]
    fn every_face_has_one_axis_aligned_unit_normal_and_matching_plane() {
        let mesh = cube_mesh(&unit());
        let positions = mesh.positions_without_indices();
        let normals = mesh.normals_without_indices();
        let bounds = [[-1.0, 1.0], [-2.0, 2.0], [-3.0, 3.0]];
        for face in 0..6 {
            let n = normals[face * 4];
            assert!(normals[face * 4..face * 4 + 4].iter().all(|m| *m == n));
            let axis = (0..3).find(|a| n[*a] != 0.0).unwrap();
            assert_eq!(n[axis].abs(), 1.0);
            let plane = if n[axis] > 0.0 {
                bounds[axis][1]
            } else {
                bounds[axis][0]
            };
            assert!(
                positions[face * 4..face * 4 + 4]
                    .iter()
                    .all(|p| p[axis] == plane)
            );
        }
    }

    #[test]
    fn faces_are_two_triangles_over_their_own_four_vertices() {
        let mesh = cube_mesh(&unit());
        for face in 0..6u32 {
            let tri = &mesh.indices[face as usize * 6..face as usize * 6 + 6];
            assert!(tri.iter().all(|i| *i / 4 == face));
            assert_eq!(tri[0], tri[5]);
            assert_eq!(tri[2], tri[3]);
        }
    }

    #[test]
    fn winding_follows_the_stored_normals() {
        let mesh = cube_mesh(&unit());
        let p = mesh.positions_without_indices();
        let n = mesh.normals_without_indices();
        for tri in mesh.indices.chunks_exact(3) {
            let (a, b, c) = (p[tri[0] as usize], p[tri[1] as usize], p[tri[2] as usize]);
            let geometric = (b - a).cross(c - a);
            assert!(geometric.dot(n[tri[0] as usize]) > 0.0, "{tri:?}");
        }
    }

    #[test]
    fn uvs_cover_the_unit_square_on_every_face() {
        let mesh = cube_mesh(&unit());
        let uvs = mesh.uvs_without_indices();
        for face in 0..6 {
            let mut face_uvs: Vec<[i32; 2]> = uvs[face * 4..face * 4 + 4]
                .iter()
                .map(|u| [u.x as i32, u.y as i32])
                .collect();
            face_uvs.sort();
            assert_eq!(face_uvs, [[0, 0], [0, 1], [1, 0], [1, 1]]);
        }
    }

    #[test]
    fn box_corners_come_from_the_selected_ranges() {
        let c = CubeBox {
            x: [10.0, 20.0],
            y: [30.0, 40.0],
            z: [50.0, 60.0],
        };
        let mesh = cube_mesh(&c);
        let p = mesh.positions_without_indices();
        assert_eq!(p[0], Vec3::new(10.0, 30.0, 60.0));
        assert_eq!(p[1], Vec3::new(20.0, 30.0, 60.0));
    }
}
