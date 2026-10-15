//! `sphere.rs` — 二十面体球网格（B `geometry::sphere`，`TryFrom<Sphere> for Mesh`，0x140a06b00）。
//!
//! 复用原依赖 `hexasphere 12.0.0`（原版 lock 里同一份，校验和一致），游戏侧的部分由汇编还原：
//! - 入参是 `subdivisions: usize`（`rdx`）和 `radius: f32`（`xmm2`）；`subdivisions >= 0x50` 直接失败，
//!   顶点数按 `(n + 1)² * 10 + 2` 计算（`inc; imul; lea*5; lea*2+2`，O）。错误文案三段
//!   （0x142ac66c6/0x142ac66e4/0x142ac6728）与 bevy 的 `IcosphereError::TooManyVertices` 一致：
//!   `Cannot create an icosphere of {n} subdivisions due to there being too many vertices being generated: {points}.
//!   (Limited to 65535 vertices or 79 subdivisions)`；
//! - 网格标签 `"Sphere"`（0x142ac66c0，6 B）；
//! - `Subdivided::new(n, uv)`：UV 闭包在 0x141306330（hexasphere 单态里对全部点做 map）：
//!   `v = acos(p.y) / π`，`u = atan2(p.z, p.x) / (-2π) + 0.5`，写出 `[u, v]`（低 32 位 u，高 32 位 v，O）；
//!   `acos`/`atan2` 调的是 CRT 的 `acosf`/`atan2f`（0x142923de0 / 0x142923e20），这里用 Rust 的 `f32` 实现，
//!   最后一位可能与 Windows CRT 相差 1 ulp（未能在本机验证）；
//! - 位置 = `raw_points * radius`（逐分量乘），法线 = `raw_points`（`Vec3A → [f32; 3]`，0x1413096c0），
//!   UV = 闭包产生的 `raw_data`；索引 = 对 20 个主三角形依次 `get_indices`（0x141797940 循环 `0..0x14`），
//!   `Vec<u32>` 直接写入网格结构；
//! - 只设置 `Vertex_Position`、`Vertex_Normal`、`Vertex_UV` 三个属性，**没有颜色**（与 cube/plane 不同，O）。
//!
//! 取舍：原版的 `Err` 载荷是 `String`（`rsi+8` 处 `format!` 产物），这里用 [`SphereError`] 包同一段文案。
use std::f32::consts::{PI, TAU};

use glam::{Vec2, Vec3};
use hexasphere::shapes::IcoSphere;

use crate::render::mesh::{ATTR_NORMAL, ATTR_POSITION, ATTR_UV, Mesh};

/// `subdivisions` 的上限（含）：79（O：`cmp rdx, 0x50; jb ok`）。
pub const MAX_SUBDIVISIONS: usize = 79;

/// 球参数（`Sphere { subdivisions, radius }`；字段顺序取自 `try_from` 的寄存器传参）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub subdivisions: usize,
    pub radius: f32,
}

/// 顶点数过多的错误（原版以 `String` 返回）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SphereError {
    pub subdivisions: usize,
    pub number_of_resulting_points: usize,
}

impl core::fmt::Display for SphereError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Cannot create an icosphere of {} subdivisions due to there being too many vertices being generated: {}. (Limited to 65535 vertices or 79 subdivisions)",
            self.subdivisions, self.number_of_resulting_points
        )
    }
}

impl std::error::Error for SphereError {}

/// 二十面体球的顶点数（O：`(n + 1)² * 10 + 2`，环绕乘法）。
pub fn icosphere_vertex_count(subdivisions: usize) -> usize {
    let n1 = subdivisions.wrapping_add(1);
    n1.wrapping_mul(n1).wrapping_mul(10).wrapping_add(2)
}

/// 单点 UV（0x141306330 的循环体）。
pub fn sphere_uv(point: Vec3) -> [f32; 2] {
    let v = point.y.acos() / PI;
    let u = point.z.atan2(point.x) / -TAU + 0.5;
    [u, v]
}

impl Sphere {
    pub fn new(subdivisions: usize, radius: f32) -> Self {
        Self {
            subdivisions,
            radius,
        }
    }

    /// `TryFrom<Sphere> for Mesh`。
    pub fn to_mesh(&self) -> Result<Mesh, SphereError> {
        if self.subdivisions > MAX_SUBDIVISIONS {
            return Err(SphereError {
                subdivisions: self.subdivisions,
                number_of_resulting_points: icosphere_vertex_count(self.subdivisions),
            });
        }
        let generated = IcoSphere::new(self.subdivisions, |p| sphere_uv(Vec3::new(p.x, p.y, p.z)));
        let raw = generated.raw_points();
        let radius = self.radius;
        let positions: Vec<Vec3> = raw
            .iter()
            .map(|p| Vec3::new(p.x * radius, p.y * radius, p.z * radius))
            .collect();
        let normals: Vec<Vec3> = raw.iter().map(|p| Vec3::new(p.x, p.y, p.z)).collect();
        let uvs: Vec<Vec2> = generated
            .raw_data()
            .iter()
            .map(|uv| Vec2::new(uv[0], uv[1]))
            .collect();
        let mut indices = Vec::with_capacity(generated.indices_per_main_triangle() * 20);
        for triangle in 0..20 {
            generated.get_indices(triangle, &mut indices);
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
mod tests {
    use super::*;

    #[test]
    fn vertex_counts_follow_the_icosphere_formula() {
        assert_eq!(icosphere_vertex_count(0), 12);
        assert_eq!(icosphere_vertex_count(1), 42);
        assert_eq!(icosphere_vertex_count(79), 64_002);
        assert_eq!(icosphere_vertex_count(80), 65_612);
    }

    #[test]
    fn limit_is_79_and_message_matches_the_recovered_strings() {
        assert!(Sphere::new(79, 1.0).to_mesh().is_ok());
        let err = Sphere::new(80, 1.0).to_mesh().unwrap_err();
        assert_eq!(
            err.to_string(),
            "Cannot create an icosphere of 80 subdivisions due to there being too many vertices being generated: 65612. (Limited to 65535 vertices or 79 subdivisions)"
        );
    }

    #[test]
    fn subdivision_zero_is_the_plain_icosahedron() {
        let mesh = Sphere::new(0, 2.0).to_mesh().unwrap();
        assert_eq!(mesh.label, "Sphere");
        assert_eq!(mesh.indices.len(), 60);
        for name in [ATTR_POSITION, ATTR_NORMAL, ATTR_UV] {
            assert_eq!(mesh.attributes[name].array_length(), 12, "{name}");
        }
        assert_eq!(mesh.attributes.len(), 3, "no colour attribute");
    }

    #[test]
    fn index_count_is_60_per_n_plus_one_squared() {
        for n in [1usize, 2, 5] {
            let mesh = Sphere::new(n, 1.0).to_mesh().unwrap();
            assert_eq!(mesh.indices.len(), 60 * (n + 1) * (n + 1));
            assert_eq!(
                mesh.attributes[ATTR_POSITION].array_length(),
                icosphere_vertex_count(n)
            );
            assert!(
                mesh.indices
                    .iter()
                    .all(|&i| (i as usize) < icosphere_vertex_count(n))
            );
        }
    }

    #[test]
    fn positions_are_normals_scaled_by_the_radius() {
        let mesh = Sphere::new(2, 3.5).to_mesh().unwrap();
        let crate::render::mesh::VertexAttributeValues::Float3(p) = &mesh.attributes[ATTR_POSITION]
        else {
            panic!("positions are Float3")
        };
        let crate::render::mesh::VertexAttributeValues::Float3(n) = &mesh.attributes[ATTR_NORMAL]
        else {
            panic!("normals are Float3")
        };
        for (p, n) in p.iter().zip(n) {
            assert_eq!(*p, *n * 3.5);
            assert!((n.length() - 1.0).abs() < 1e-5);
        }
    }

    #[test]
    fn uv_formula_matches_the_closure() {
        assert_eq!(sphere_uv(Vec3::Y), [0.5, 0.0]);
        assert_eq!(sphere_uv(Vec3::NEG_Y), [0.5, 1.0]);
        let [u, v] = sphere_uv(Vec3::X);
        assert_eq!((u, v), (0.5, 0.5));
        let [u, _] = sphere_uv(Vec3::Z);
        assert!((u - 0.25).abs() < 1e-6);
    }

    #[test]
    fn try_from_matches_to_mesh() {
        let a = Mesh::try_from(Sphere::new(1, 1.0)).unwrap();
        let b = Sphere::new(1, 1.0).to_mesh().unwrap();
        assert_eq!(a.indices, b.indices);
    }
}
