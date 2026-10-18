//! 轴对齐盒网格：每面独立顶点，保留硬边法线与完整 UV。
use glam::{Vec2, Vec3};

use crate::render::mesh::{ATTR_COLOR, ATTR_NORMAL, ATTR_POSITION, ATTR_UV, Mesh};

/// 三轴端点区间；生成网格时自动排序，使反向输入仍得到外向面。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubeBox {
    pub x: [f32; 2],
    pub y: [f32; 2],
    pub z: [f32; 2],
}

/// 每个面有自己的四个端点、法线和 UV，避免用混合数字表编码不同属性。
struct CubeFace {
    corners: [[usize; 3]; 4],
    normal: Vec3,
    uvs: [[f32; 2]; 4],
}

const FACE_UVS: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
const MIRRORED_FACE_UVS: [[f32; 2]; 4] = [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
const FACES: [CubeFace; 6] = [
    CubeFace {
        normal: Vec3::Z,
        corners: [[0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]],
        uvs: FACE_UVS,
    },
    CubeFace {
        normal: Vec3::NEG_Z,
        corners: [[0, 1, 0], [1, 1, 0], [1, 0, 0], [0, 0, 0]],
        uvs: MIRRORED_FACE_UVS,
    },
    CubeFace {
        normal: Vec3::X,
        corners: [[1, 0, 0], [1, 1, 0], [1, 1, 1], [1, 0, 1]],
        uvs: FACE_UVS,
    },
    CubeFace {
        normal: Vec3::NEG_X,
        corners: [[0, 0, 1], [0, 1, 1], [0, 1, 0], [0, 0, 0]],
        uvs: MIRRORED_FACE_UVS,
    },
    CubeFace {
        normal: Vec3::Y,
        corners: [[1, 1, 0], [0, 1, 0], [0, 1, 1], [1, 1, 1]],
        uvs: MIRRORED_FACE_UVS,
    },
    CubeFace {
        normal: Vec3::NEG_Y,
        corners: [[1, 0, 1], [0, 0, 1], [0, 0, 0], [1, 0, 0]],
        uvs: FACE_UVS,
    },
];

/// 每面两个三角形，按外向逆时针排列。
const INDICES: [u32; 36] = [
    0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17, 18,
    18, 19, 16, 20, 21, 22, 22, 23, 20,
];

/// 生成六面盒体。非有限端点属于调用错误，零厚度轴允许生成退化面。
pub fn cube_mesh(cube: &CubeBox) -> Mesh {
    assert!(
        cube.x
            .into_iter()
            .chain(cube.y)
            .chain(cube.z)
            .all(f32::is_finite),
        "盒体端点必须为有限数值"
    );
    let x_bounds = [cube.x[0].min(cube.x[1]), cube.x[0].max(cube.x[1])];
    let y_bounds = [cube.y[0].min(cube.y[1]), cube.y[0].max(cube.y[1])];
    let z_bounds = [cube.z[0].min(cube.z[1]), cube.z[0].max(cube.z[1])];
    let vertex_count = FACES.len() * 4;
    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);
    for face in FACES {
        for ([x_index, y_index, z_index], uv) in face.corners.into_iter().zip(face.uvs) {
            positions.push(Vec3::new(
                x_bounds[x_index],
                y_bounds[y_index],
                z_bounds[z_index],
            ));
            normals.push(face.normal);
            uvs.push(Vec2::from_array(uv));
        }
    }
    let mut mesh = Mesh::new("Box");
    mesh.set_indices(INDICES.to_vec());
    mesh.set_attribute(ATTR_POSITION, positions);
    mesh.set_attribute(ATTR_NORMAL, normals);
    mesh.set_attribute(ATTR_UV, uvs);
    mesh.set_attribute(ATTR_COLOR, vec![Vec3::ONE; vertex_count]);
    mesh
}

#[cfg(test)]
#[path = "tests/cube.rs"]
mod tests;
