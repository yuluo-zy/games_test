//! add_autoclutter_around_gates（B 主基线；0x1422cc390，源码 21–68）。
//!
//! 证据：遍历门（+0xf8/+0x100 数据/长度、+0x108 过滤标志双分支）；每门经 WallId + tick
//! 索引（+0x110/+0x118）取墙条目（stride 0x38）；墙点数 > 2 才放置；取墙首/末点
//! （stride 0xc=Vec3）。**2 个放置**：prefab `gate_right`（0x143042260，len 10）与
//! `gate_left`（len 9，.data 相邻）；位置 = 墙端中点 + 法向/方向偏移（1/SQRT 归一化、
//! x0y swizzle），旋转用 2.0,2.0 四元数常量。键：`calculate_hash<tuple<&str,u32,WallId>>`
//! （0x141cf7440）→ `AutoClutterState::add_region`（每门左右各 1 区域候选）。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};


use super::{AutoClutterRegion, AutoClutterState, ClutterCandidateIndex, ClutterRegionId};
use country_core::resources::walls::WallId;

/// 门放置宿主记录（墙端 + 门宽；宿主装配时填充）。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct GateHost {
    pub wall: WallId,
    pub left: Vec3,
    pub right: Vec3,
}

/// add_autoclutter_around_gates（0x1422cc390）。
/// 每门：取墙首/末点（>2 点才放置）；左右各 1 区域候选。
pub fn add_autoclutter_around_gates(
    mut state: ResMut<AutoClutterState>,
    mut query: Query<&GateHost>,
) {
    for gate in &mut query {
        let (a, b) = (gate.left, gate.right);
        if a.distance_squared(b) < 1e-6 {
            continue;
        }
        // 中点 + 法向偏移（O：1/SQRT 归一化 + x0y swizzle）。
        let mid = (a + b) * 0.5;
        let dir = (b - a).normalize_or_zero();
        let normal = Vec3::new(-dir.z, 0.0, dir.x);
        for (name, side) in [("gate_left", -1.0f32), ("gate_right", 1.0f32)] {
            let pos = mid + normal * side * 0.5;
            let key = hash_gate_key(name, gate.wall.get_inner());
            let region = AutoClutterRegion {
                key: ClutterRegionId(key),
                center: pos,
                basis: [Vec2::new(normal.x, normal.z), Vec2::new(dir.x, dir.z)],
                candidate_index: ClutterCandidateIndex(0),
                epoch: state.epoch,
            };
            state.add_region(ClutterRegionId(key), &region);
        }
    }
}

/// 门放置键（`calculate_hash<tuple<&str,u32,WallId>>` 0x141cf7440 语义）。
pub fn hash_gate_key(name: &str, wall: u64) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut h);
    (name.len() as u32).hash(&mut h);
    wall.hash(&mut h);
    h.finish()
}
