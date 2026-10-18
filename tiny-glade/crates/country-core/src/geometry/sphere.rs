//! 二十面体细分球：细分复用 hexasphere，当前模块负责输入校验和网格属性。
use std::f32::consts::{PI, TAU};

use glam::{Vec2, Vec3};
use hexasphere::shapes::IcoSphere;

use crate::render::mesh::{ATTR_NORMAL, ATTR_POSITION, ATTR_UV, Mesh};

/// 控制顶点和索引规模，最大产生 64,002 个顶点。
pub const MAX_SUBDIVISIONS: usize = 79;

/// 球参数；radius 必须有限且非负。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub subdivisions: usize,
    pub radius: f32,
}

/// 参数错误在分配细分数据前返回。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SphereError {
    TooManyVertices {
        subdivisions: usize,
        number_of_resulting_points: usize,
    },
    InvalidRadius,
}

impl core::fmt::Display for SphereError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooManyVertices {
                subdivisions,
                number_of_resulting_points,
            } => write!(
                formatter,
                "Cannot create an icosphere of {subdivisions} subdivisions due to there being too many vertices being generated: {number_of_resulting_points}. (Limited to 65535 vertices or 79 subdivisions)"
            ),
            Self::InvalidRadius => formatter.write_str("球半径必须为有限非负数"),
        }
    }
}

impl std::error::Error for SphereError {}

/// 顶点数估计；超大输入饱和为 usize::MAX，不发生回绕。
pub fn icosphere_vertex_count(subdivisions: usize) -> usize {
    subdivisions
        .saturating_add(1)
        .saturating_pow(2)
        .saturating_mul(10)
        .saturating_add(2)
}

/// 单位球方向对应的 UV；夹取 y 的舍入误差，避免 acos 产生 NaN。
pub fn sphere_uv(direction: Vec3) -> [f32; 2] {
    let longitude = direction.z.atan2(direction.x) / -TAU + 0.5;
    let latitude = direction.y.clamp(-1.0, 1.0).acos() / PI;
    [longitude, latitude]
}

impl Sphere {
    pub fn new(subdivisions: usize, radius: f32) -> Self {
        Self {
            subdivisions,
            radius,
        }
    }

    /// 生成球体网格；错误参数不会触发细分和大块分配。
    pub fn to_mesh(&self) -> Result<Mesh, SphereError> {
        if self.subdivisions > MAX_SUBDIVISIONS {
            return Err(SphereError::TooManyVertices {
                subdivisions: self.subdivisions,
                number_of_resulting_points: icosphere_vertex_count(self.subdivisions),
            });
        }
        if !self.radius.is_finite() || self.radius < 0.0 {
            return Err(SphereError::InvalidRadius);
        }
        let generated = IcoSphere::new(self.subdivisions, |point| {
            sphere_uv(Vec3::new(point.x, point.y, point.z))
        });
        let unit_positions = generated.raw_points();
        let radius = self.radius;
        let positions: Vec<Vec3> = unit_positions
            .iter()
            .map(|position| {
                Vec3::new(
                    position.x * radius,
                    position.y * radius,
                    position.z * radius,
                )
            })
            .collect();
        let normals: Vec<Vec3> = unit_positions
            .iter()
            .map(|position| Vec3::new(position.x, position.y, position.z))
            .collect();
        let uvs: Vec<Vec2> = generated
            .raw_data()
            .iter()
            .map(|uv| Vec2::new(uv[0], uv[1]))
            .collect();
        let mut indices = Vec::with_capacity(generated.indices_per_main_triangle() * 20);
        for main_triangle in 0..20 {
            generated.get_indices(main_triangle, &mut indices);
        }

        let mut mesh = Mesh::new("Sphere");
        mesh.set_indices(indices);
        mesh.set_attribute(ATTR_POSITION, positions);
        mesh.set_attribute(ATTR_NORMAL, normals);
        mesh.set_attribute(ATTR_UV, uvs);
        Ok(mesh)
    }
}

impl TryFrom<Sphere> for Mesh {
    type Error = SphereError;

    fn try_from(sphere: Sphere) -> Result<Self, Self::Error> {
        sphere.to_mesh()
    }
}

#[cfg(test)]
#[path = "tests/sphere.rs"]
mod tests;
