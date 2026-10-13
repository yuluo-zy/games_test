//! wall_coord.rs — 墙坐标（WP16 权威；WP21 补充 `PublicWallCurve` 采样面）。
//!
//! O：`WallCoord`：`from_wall_u`（1 内联）+ `into_world_space`（0x140b199e0）：读
//! `PublicWallState` 曲线总长（+0x30）、曲线点表（+8/+0x10，Vec2/项），`u/总长` 采样
//! `Curve::get_coord_at_u` 得墙点，沿墙面法线偏移 `0.63*0.5`（O 常量），按侧 bool 取 ±。
//! `PublicWallState::max_ivy_grow_height` 内联引用（O 名）。

use glam::{Vec2, Vec3};

use crate::typed_gap::Gap;

/// 墙坐标（O：`resources::ivy::wall_coord::WallCoord`）。
///
/// 布局 I：`{ u: f32, v: f32 }` —— 沿墙曲线参数 u 与高度 v。
/// `into_world_space`（O 0x140b199e0）：读 `PublicWallState` 曲线总长（+0x30）、
/// 曲线点表（+8 ptr/+0x10 len，每点 Vec2），以 u/总长 采样 `Curve::get_coord_at_u`
/// 得墙点，沿墙面法线偏移 `0.63 * 0.5`（O 常量），按侧（bool）取 ±。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WallCoord {
    /// 沿墙参数（I）。
    pub u: f32,
    /// 高度（I）。
    pub v: f32,
}

impl WallCoord {
    /// `from_wall_u`（WP21；O 名，1 内联）。
    pub fn from_wall_u(u: f32) -> Self {
        Self { u, v: 0.0 }
    }
    /// `into_world_space`（O 0x140b199e0）：`(u, v, 墙数据, 墙侧)` → 世界位置。
    /// 精确曲线几何 U（依赖墙公共状态曲线）。
    pub fn into_world_space(
        &self,
        _wall: &crate::systems::collision::world::WallCurveSource,
        _side: bool,
    ) -> Vec3 {
        Vec3::ZERO
    }
}

/// `PublicWallState` 曲线面（WP21；O：总长 +0x30、点表 +8/+0x10；布局 U，typed gap）。
/// 供 `WallCoord` 曲线采样；整合 crate 的墙公共状态未直接映射，此处独立承载。
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
    /// 墙面法向（WP21；I：沿切线旋转）。
    pub fn normal_at_u(&self, u: f32) -> Vec3 {
        // I：曲线切向 (dx,0,dz) → 法向 (-dz,0,dx)。
        let t = self.get_coord_at_u((u + 0.001).min(1.0)) - self.get_coord_at_u(u);
        let len = t.truncate().length().max(f32::EPSILON);
        Vec3::new(-t.z / len, 0.0, t.x / len)
    }
}
