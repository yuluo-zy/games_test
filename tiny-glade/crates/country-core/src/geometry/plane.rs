//! XZ 平面的正方形网格，法线朝 +Y，UV 覆盖单位正方形。
use glam::{Vec2, Vec3};

use crate::render::mesh::{ATTR_COLOR, ATTR_NORMAL, ATTR_POSITION, ATTR_UV, Mesh};

/// 两个朝 +Y 的三角形。
const INDICES: [u32; 6] = [0, 2, 1, 0, 3, 2];

/// size 为完整边长；负值按绝对值处理，零长度保留退化几何。
pub fn plane_mesh(size: f32) -> Mesh {
    assert!(size.is_finite(), "平面边长必须为有限数值");
    let half_size = size.abs() * 0.5;
    let positions = vec![
        Vec3::new(half_size, 0.0, -half_size),
        Vec3::new(half_size, 0.0, half_size),
        Vec3::new(-half_size, 0.0, half_size),
        Vec3::new(-half_size, 0.0, -half_size),
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
#[path = "tests/plane.rs"]
mod tests;
