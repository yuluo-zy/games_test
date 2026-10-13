//! ivy_grower.rs — 常春藤生长系统（O 0x14094a910）。
//!
//! 源 `systems/ivy/ivy_grower.rs`。功能（O）：
//!  - 逐株常春藤：`ivy_grower` 沿墙生长 —— 采样 `IvyDirectionProposer::get_direction`
//!    生长方向、`get_growth_point_proposal` 提议新点、`check_for_wall_jump` 检测
//!    墙跳、`get_wall_double_sided_normal`（0x14094caa0）取双面法线、
//!    `closest_segment_index`（0x14094c800）与 `intersect_ivy_growth_w_wall_segment`
//!    （0x14094cc60）约束生长不与墙段碰撞。
//!  - `ivy_grower` 还读 `PublicWallState::max_ivy_grow_height` 限制高度、
//!    `WallCoord` 坐标换算，`Ivy::last_point`/`set_last_point_position`/`add_point`
//!    推进折线。

#![allow(dead_code)]

use glam::Vec3;

use crate::resources::ivy::Ivy;

/// `closest_segment_index`（O 0x14094c800）：找距离给定点最近且允许生长的墙段
/// 索引与最近点/距离（返回值结构 O：`{found, index, dist}`）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ClosestSegmentResult {
    pub found: bool,
    pub index: u32,
    pub distance: f32,
}

/// `closest_segment_index`（O 0x14094c800；用于 grower 的墙段约束）。
pub fn closest_segment_index(_point: Vec3, _segments: &[WallSegment]) -> ClosestSegmentResult {
    ClosestSegmentResult::default()
}

/// 墙段（I：grower 查询墙段；O 结构为两 Vec2 端 + 高度区间）。
#[derive(Clone, Copy, Debug, Default)]
pub struct WallSegment {
    pub a: Vec3,
    pub b: Vec3,
    pub min_height: f32,
    pub max_height: f32,
}

/// `intersect_ivy_growth_w_wall_segment`（O 0x14094cc60）：生长线是否穿墙段。
pub fn intersect_ivy_growth_w_wall_segment(_from: Vec3, _to: Vec3, _seg: &WallSegment) -> bool {
    false
}

/// `get_growth_point_proposal`（O：ivy_grower 内联）—— 建议下一生长点。
pub fn get_growth_point_proposal(_ivy: &Ivy, _dir: Vec3, _step: f32) -> Vec3 {
    Vec3::ZERO
}

/// `check_for_wall_jump`（O：ivy_grower 内联）—— 检测能否跳过相邻墙。
pub fn check_for_wall_jump(_ivy: &Ivy, _wall: &crate::systems::collision::world::WallCurveSource) -> bool {
    false
}

/// `ivy_grower`（O 0x14094a910）。
pub fn ivy_grower(_world: &mut bevy_ecs::world::World) {}

/// 生长统计（I：grower 内部增量；typed gap）。
pub struct GrowStats {
    pub grown: u32,
    pub stuck: u32,
}
