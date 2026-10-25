//! 游戏自有采样工具（B 证据 VA：uniform_sample_disk 0x140c90700、uniform_sample_circle
//! 0x140c90770、uniform_sample_sphere 0x140c907d0、golden_spiral 0x140c90940）。
//! 依赖复用边界：数学复用 glam/std；这里是游戏自有策略（采样分布/螺旋序列），
//! 与 reconstruction/firefly.rs::sample_sphere_game 及 critters-extra 出生采样一致。

use glam::{Vec2, Vec3};
use perchance::PerchanceContext;

/// `utils::uniform_sample_disk`（0x140c90700）：单位圆盘均匀采样。
pub fn uniform_sample_disk(rng: &mut PerchanceContext, u: f32) -> Vec2 {
    let v = rng.uniform_f32();
    let r = u.sqrt();
    let a = v * std::f32::consts::TAU;
    Vec2::new(r * a.cos(), r * a.sin())
}

/// `utils::uniform_sample_circle`（0x140c90770）：单位圆周均匀采样。
pub fn uniform_sample_circle(rng: &mut PerchanceContext, u: f32) -> Vec2 {
    let _ = rng;
    let a = u * std::f32::consts::TAU;
    Vec2::new(a.cos(), a.sin())
}

/// `utils::uniform_sample_sphere`（0x140c907d0）：单位球面均匀采样。
/// 与 reconstruction/firefly.rs::sample_sphere_game 逐式一致。
pub fn uniform_sample_sphere(rng: &mut PerchanceContext) -> Vec3 {
    let u = rng.uniform_f32();
    let v = rng.uniform_f32();
    let z = 1.0 - (u + u);
    let radial_squared = 1.0 - z * z;
    let radial = if radial_squared <= 0.0 { 0.0 } else { radial_squared }.sqrt();
    let angle = v * std::f32::consts::TAU;
    Vec3::new(radial * angle.cos(), radial * angle.sin(), z)
}

/// `utils::golden_spiral`（0x140c90940）：黄金螺旋序列点（第 i 项；r 由调用方缩放）。
pub fn golden_spiral(i: f32) -> Vec2 {
    let a = i * std::f32::consts::PI * (3.0 - 5.0_f32.sqrt());
    Vec2::new(a.cos(), a.sin()) * (i / 2.0).sqrt()
}

/// `x0y`（utils::swizzlers）：Vec2 → 水平面 Vec3。
pub fn x0y(xy: Vec2) -> Vec3 {
    Vec3::new(xy.x, 0.0, xy.y)
}

/// `x0z`（utils::swizzlers）：丢 y。
pub fn x0z(xyz: Vec3) -> Vec3 {
    Vec3::new(xyz.x, 0.0, xyz.z)
}
