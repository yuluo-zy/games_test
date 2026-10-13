//! wall_coord.rs — 墙坐标（WP16 逻辑重述）。
//!
//! O：`WallCoord`：`from_wall_u`（1 内联）+ `into_world_space`（0x140b199e0）：读
//! `PublicWallState` 曲线总长（+0x30）、曲线点表（+8/+0x10，Vec2/项），`u/总长` 采样
//! `Curve::get_coord_at_u` 得墙点，沿墙面法线偏移 `0.63*0.5`（O 常量），按侧 bool 取 ±。
//! `PublicWallState::max_ivy_grow_height` 内联引用（O 名）。

use glam::{Vec2, Vec3};

use crate::typed_gap::Gap;

/// `WallCoord`（O：墙 U 坐标 → 世界）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WallCoord {
    pub u: f32,
}

impl WallCoord {
    /// `from_wall_u`（O 名，1 内联）。
    pub fn from_wall_u(u: f32) -> Self {
        Self { u }
    }
    /// `into_world_space`（O 0x140b199e0）：沿曲线采样，法向偏移 0.63*0.5，按侧±。
    ///
    /// `public` = PublicWallState 曲线面（U 布局；整合 crate 提供）。side=true=右/+
    /// （I，偏移方向）。
    pub fn into_world_space(
        &self,
        public: &PublicWallCurve,
        side: bool,
    ) -> Vec3 {
        let t = self.u / public.total_length.max(f32::EPSILON);
        let pos = public.get_coord_at_u(t);
        let normal = public.normal_at_u(t);
        let offset = normal * (0.63 * 0.5) * if side { 1.0 } else { -1.0 };
        pos + offset
    }
}

/// `PublicWallState` 曲线面（O：总长 +0x30、点表 +8/+0x10；布局 U，typed gap）。
#[derive(Clone, Debug, Default)]
pub struct PublicWallCurve {
    pub total_length: f32,
    pub points: Vec<Vec2>,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl PublicWallCurve {
    /// `Curve::get_coord_at_u`（O 名；curve 采样）。
    pub fn get_coord_at_u(&self, u: f32) -> Vec3 {
        if self.points.is_empty() {
            return Vec3::ZERO;
        }
        let x = u * (self.points.len() as f32 - 1.0);
        let i = x.floor().clamp(0.0, self.points.len() as f32 - 2.0) as usize;
        let f = x - i as f32;
        let a = self.points[i];
        let b = self.points[(i + 1).min(self.points.len() - 1)];
        Vec3::new(a.x + (b.x - a.x) * f, 0.0, a.y + (b.y - a.y) * f)
    }
    /// 墙面法向（I：沿切线旋转）。
    pub fn normal_at_u(&self, u: f32) -> Vec3 {
        // I：曲线切向 (dx,0,dz) → 法向 (-dz,0,dx)。
        let t = self.get_coord_at_u((u + 0.001).min(1.0)) - self.get_coord_at_u(u);
        let len = t.truncate().length().max(f32::EPSILON);
        Vec3::new(-t.z / len, 0.0, t.x / len)
    }
}
