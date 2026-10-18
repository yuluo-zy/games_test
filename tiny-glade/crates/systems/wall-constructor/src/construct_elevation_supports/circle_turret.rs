//! 圆形塔楼（construct_elevation_supports/circle_turret.rs）。
//!
//! `construct_turret`（0x14124cc30，1902 行 fresh，5523B）：圆形塔楼支撑构造。
//! 2026-10-17 复核：旧实现（每层 1.2 高、半径线性收 15%、TAU 均分）为
//! **近似体**（层数/收分率无证据），已移除。
//!
//! fresh 已确认的骨架：`circle_categorize` 返回 false 时由
//! `construct_circle_supports` 调用；内部为圆形逐层砖环 + 地形/墙高门控
//! （与 `construct_circle_pillars` 同族的 `wall+0x298` 半木构二路）。
//! 层数、收分率、每层计数公式未逐语句锚定 → **未实现**。
use glam::Vec3;

/// B `construct_turret`（0x14124cc30）。**未实现**：层数/收分率未定。
pub fn construct_turret(
    _center: Vec3,
    _radius: f32,
    _height: f32,
    _out: &mut Vec<[u8; 96]>,
) {
    todo!("未实现: construct_turret 0x14124cc30（1902 行 fresh：层数/收分率/逐层计数/半木构二路未逐语句）")
}
