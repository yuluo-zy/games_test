//! 木地板边条（curved_halftimber/wooden_floor_borders.rs）。
//!
//! `wooden_floor_borders`（0x141244310，1124 行 fresh:
//! `decomp-0x141244310.txt`）：真实系统由 `PublicWalls` 屋顶关系定位墙，
//! `WallHoleStorage` finalized holes、`WallStyle::is_halftimbered` 过滤、
//! `CurvedHalfTimberInstanceData::new_curved_xz/new_straight`（矩形
//! `as_curve2`/`circular_slice` + `fmodf(方向/周期,1.0)`）与 spawn 组成。
//!
//! **未实现（U）**：`CurvedHalfTimberInstanceData` 在 country-core 未落库
//! （`CurvedHalfTimberDrawables` 为 `Gap`）；边框宽高常量与实例字段语义 U。
//! 旧“相邻点中点直推 64B”的近似体已移除（不伪造输出）。
use glam::Vec3;

/// B 0x141244310。**未实现**：见模块文档。
pub fn wooden_floor_borders(_points: &[Vec3], _floor_y: f32, _out: &mut Vec<[u8; 64]>) {
    todo!(
        "未实现: wooden_floor_borders 0x141244310（CurvedHalfTimberInstanceData/new_curved_xz/new_straight 未落库；边框常量 U）"
    )
}
