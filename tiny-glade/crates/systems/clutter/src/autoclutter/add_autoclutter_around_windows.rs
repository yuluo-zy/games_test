//! add_autoclutter_around_windows（B 主基线；0x1422cefe0，size 3606）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译重建
//! （`evidence/decomp/0x1422cefe0_*.c`；字符串实读 0x1430428e8 =
//! `decorator_right`(15) / `decorator_left`(14)、0x143042920 = `trap_door`(9)）。
//!
//! 控制流（O）：
//! - 遍历被移除/变化的窗装饰物：`PublicWalls::get(wall)` 且墙 style 字节
//!   `*(uint*)(wall+600) < 2`（非半木）才处理；`DecoratorType::spawns_clutter`
//!   （0x140833170：`(t&0x3e)==0x38`）门控。
//! - 分派（子类型字节 b）：`k = if 2 < b-2 {0} else {b-1}`：
//!   - `k==0`：沿装饰物曲线 `extend_right_until` 取段两端点，**2 次** `add_region`，
//!     名字 `decorator_right` / `decorator_left`，键
//!     `calculate_hash<tuple<&str,usize,u32>>`；
//!   - `k==1` 且 `DerivedDecoratorInfo::subtype()` ∈ {10,5}：`CachedDecoratorTransforms::transform`
//!     取四元数 → **2 次** `add_region`（i=0..2），名字 `trap_door`，键
//!     `calculate_hash<tuple<&str,i32,usize,u32>>`；位置含 ±1.5 偏移。
//!
//! U 登记：窗口曲线/四元数来自 `Decorators`/`CachedDecoratorTransforms` 宿主查询；本 crate
//! 以 [`WindowHost`] 承载曲线与朝向。精确位置算术（4.72/0.28/0.5/1.5 项）为 I。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorType;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use glam::{Vec2, Vec3};

use super::{AutoClutterRegion, AutoClutterState, ClutterCandidateIndex, ClutterRegionId};

/// 窗放置宿主记录（宿主装配时填充）。
#[derive(Component, Clone, Debug, PartialEq)]
pub struct WindowHost {
    /// 窗装饰物 id（DecoratorStorage 键）。
    pub decorator_id: u64,
    /// 装饰物类型字节（`DecoratorType::spawns_clutter` 判据）。
    pub decorator_type: u8,
    /// 子类型字节（`DecoratorInfo` 分派源）。
    pub subtype: u8,
    /// 墙 id（哈希载荷 u32 的来源之一；O +0x30）。
    pub u32_field: u32,
    /// 窗曲线（世界空间点；extend_right_until 消费）。
    pub curve: Vec<Vec3>,
    /// 窗朝向（xvy swizzle 语义）。
    pub forward: Vec2,
}

/// 窗放置名（O，字符串实读）。
pub const DECORATOR_RIGHT: &str = "decorator_right";
pub const DECORATOR_LEFT: &str = "decorator_left";
pub const TRAP_DOOR: &str = "trap_door";

/// add_autoclutter_around_windows（0x1422cefe0）。
pub fn add_autoclutter_around_windows(
    mut state: ResMut<AutoClutterState>,
    decorators: Res<DecoratorStorage>,
    mut query: Query<&WindowHost>,
) {
    let _ = &decorators;
    for window in &mut query {
        // O：DecoratorType::spawns_clutter 门控。
        if !DecoratorType(window.decorator_type).spawns_clutter() {
            continue;
        }
        let Some((a, b)) = window_ends(&window.curve) else {
            continue;
        };
        // O：子类型分派 k = if 2 < b-2 {0} else {b-1}。
        let subtype_b = window.subtype;
        let k = if 2 < subtype_b.wrapping_sub(2) {
            0
        } else {
            subtype_b.wrapping_sub(1)
        };
        let fwd = Vec3::new(window.forward.x, 0.0, window.forward.y);
        let dir = (b - a).normalize_or_zero();
        let normal = Vec3::new(-dir.z, 0.0, dir.x);
        if k == 0 {
            // O：窗口分支：decorator_right / decorator_left 各 1 区域。
            for (name, offset) in [(DECORATOR_RIGHT, -0.4f32), (DECORATOR_LEFT, 0.4f32)] {
                let pos = (a + b) * 0.5 + normal * offset + fwd * 0.2;
                // O：键 tuple<&str, usize, u32>（名字、曲线长、u32）。
                let key = hash_window_key(name, window.curve.len(), window.u32_field);
                let region = AutoClutterRegion {
                    kind: 3,
                    key: ClutterRegionId(key),
                    center: pos,
                    basis: [Vec2::new(normal.x, normal.z), Vec2::new(fwd.x, fwd.z)],
                    f34: 0,
                    b38: 0,
                    b39: 0,
                    candidate_index: ClutterCandidateIndex::NONE,
                    epoch: state.epoch,
                };
                state.add_region(ClutterRegionId(key), &region);
            }
        } else if k == 1 && matches!(window.subtype, 5 | 10) {
            // O：trap_door 分支：i=0..2，±1.5 偏移；键 tuple<&str,i32,usize,u32>。
            for i in 0..2usize {
                let side = if i == 0 { -1.0f32 } else { 1.0 };
                let pos = (a + b) * 0.5 + normal * side * 1.5;
                let key = hash_trap_door_key(window.subtype as i32, i, window.u32_field);
                let region = AutoClutterRegion {
                    kind: 3,
                    key: ClutterRegionId(key),
                    center: pos,
                    basis: [Vec2::new(normal.x, normal.z), Vec2::new(fwd.x, fwd.z)],
                    f34: 0,
                    b38: 0,
                    b39: 0,
                    candidate_index: ClutterCandidateIndex::NONE,
                    epoch: state.epoch,
                };
                state.add_region(ClutterRegionId(key), &region);
            }
        }
    }
}

/// 窗曲线端点（extend_right_until 语义的简化：取曲线首/末点；I）。
pub fn window_ends(curve: &[Vec3]) -> Option<(Vec3, Vec3)> {
    match (curve.first(), curve.last()) {
        (Some(&a), Some(&b)) => Some((a, b)),
        _ => None,
    }
}

/// 窗口键（`calculate_hash<tuple<&str,usize,u32>>` 语义）。
pub fn hash_window_key(name: &str, curve_len: usize, u32_field: u32) -> u64 {
    utils::calculate_hash(&(name, curve_len, u32_field))
}

/// trap_door 键（`calculate_hash<tuple<&str,i32,usize,u32>>` 语义）。
pub fn hash_trap_door_key(subtype: i32, i: usize, u32_field: u32) -> u64 {
    utils::calculate_hash(&(TRAP_DOOR, subtype, i, u32_field))
}
