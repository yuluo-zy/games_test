//! add_autoclutter_around_gates（B 主基线；0x1422cc390，size 2225）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译 + 逐指令反汇编重建
//! （`evidence/decomp/0x1422cc390_*.c`、`evidence/disasm6_1.txt`）。
//!
//! 控制流（O）：
//! - 宿主记录 stride **0x38**：`+0x08` 曲线数据、`+0x10` 曲线点数、`+0x18` WallId、
//!   `+0x20` 方向 Vec2、`+0x28` 方向 Vec2、`+0x30` u32、`+0x34` u8。
//! - 曲线点数 `> 2` 才放置（O：`2 < hiVar`）；首点用于 `gate_right`、末点用于
//!   `gate_left`（各 +2.0 y）。
//! - 每放置：方向 `dir = 宿主 +0x20/+0x28`；`normal = (-dy, dx)/|dir|`（0x140c9cdc0
//!   x0y swizzle 前）；位置 = 端点 + normal × side × 0.5（side：right=-1, left=+1）。
//! - 键 `calculate_hash<tuple<&str,u32,WallId>>`（名字、+0x30 u32、+0x18 WallId）。
//! - 区域记录：kind=3、候选索引 **0xffff**、`+0x34` 字节；每门 2 个 `add_region`。
//!
//! U 登记：位置组合的精确 swizzle/0.5 项（编译器将 midpoint/normal 展开为多条 SSE）
//! 为 I；区域基向量（二进制两个 2.0 常量）与 `+0x34` 字节语义登记为 I/U。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::{AutoClutterRegion, AutoClutterState, ClutterCandidateIndex, ClutterRegionId};
use country_core::resources::walls::WallId;

/// 门放置宿主记录（stride 0x38；宿主装配时填充）。
#[derive(Component, Clone, Debug, PartialEq)]
pub struct GateHost {
    /// +0x18：WallId。
    pub wall: WallId,
    /// +0x08/+0x10：曲线（世界空间点；首/末点消费）。
    pub curve: Vec<Vec3>,
    /// +0x20：方向向量（gate_right）。
    pub dir_right: Vec2,
    /// +0x28：方向向量（gate_left）。
    pub dir_left: Vec2,
    /// +0x30：u32（哈希载荷）。
    pub u32_field: u32,
    /// +0x34：u8（区域载荷）。
    pub flag: u8,
}

/// 门放置名（O：`gate_right` len 10 / `gate_left` len 9，左右顺序固定）。
pub const GATE_RIGHT: &str = "gate_right";
pub const GATE_LEFT: &str = "gate_left";

/// add_autoclutter_around_gates（0x1422cc390）。
/// 每门：曲线 >2 点才放置；首/末点各 1 区域候选。
pub fn add_autoclutter_around_gates(
    mut state: ResMut<AutoClutterState>,
    mut query: Query<&GateHost>,
) {
    for gate in &mut query {
        // O：曲线点数 > 2 才处理。
        if gate.curve.len() <= 2 {
            continue;
        }
        let first = gate.curve[0];
        let last = gate.curve[gate.curve.len() - 1];
        for (i, (name, dir, point, side)) in [
            (GATE_RIGHT, gate.dir_right, first + Vec3::Y * 2.0, -1.0f32),
            (GATE_LEFT, gate.dir_left, last + Vec3::Y * 2.0, 1.0f32),
        ]
        .into_iter()
        .enumerate()
        {
            // O：normal = (-dy, dx)/|d|。
            let len = (dir.x * dir.x + dir.y * dir.y).sqrt();
            if len == 0.0 {
                continue;
            }
            let normal = Vec2::new(-dir.y / len, dir.x / len);
            // O：位置 = 端点 + normal × side × 0.5（0.5 项为 I）。
            let pos = point + Vec3::new(normal.x, 0.0, normal.y) * side * 0.5;
            // O：键 tuple<&str,u32,WallId>。
            let key = hash_gate_key(name, gate.u32_field, gate.wall.get_inner());
            let region = AutoClutterRegion {
                kind: 3,
                key: ClutterRegionId(key),
                center: pos,
                basis: [normal, dir],
                f34: 0,
                b38: gate.flag,
                b39: 0,
                // O：候选索引初值 0xffff。
                candidate_index: ClutterCandidateIndex::NONE,
                epoch: state.epoch,
            };
            state.add_region(ClutterRegionId(key), &region);
            let _ = i;
        }
    }
}

/// 门放置键（`calculate_hash<tuple<&str,u32,WallId>>` 0x141cf7440 语义）。
pub fn hash_gate_key(name: &str, u32_field: u32, wall: u64) -> u64 {
    utils::calculate_hash(&(name, u32_field, WallId(wall)))
}
