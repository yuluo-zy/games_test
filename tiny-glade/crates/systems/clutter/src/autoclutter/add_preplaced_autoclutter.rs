//! add_preplaced_autoclutter（B 主基线；0x141ce5900，源码 29–）。
//!
//! 证据（skeleton PDB 依赖注解）：AutoClutterState::add_region（0x140ee3fd0）、
//! calculate_hash（0x141cf70d0）、Circle2d::new（0x140c9cdc0）/new_rotated（0x140c9cde0）、
//! puffin 范围。语义：预放置自动杂物资源（PreplacedAutoClutter）逐条目 add_region，
//! 使预放置杂物进入自动散布链。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};


use super::{AutoClutterRegion, AutoClutterState, ClutterCandidateIndex, ClutterRegionId};

/// 预放置自动杂物条目（宿主从存档/场景数据填充）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PreplacedAutoClutter {
    pub entries: Vec<PreplacedEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreplacedEntry {
    /// 放置名（"gate_left"/"lantern"…；键输入）。
    pub name: &'static str,
    /// 位置。
    pub position: Vec3,
    /// 朝向（旋转弧度）。
    pub rotation: f32,
    /// 宿主 id（墙/装饰物）。
    pub host: u64,
}

/// add_preplaced_autoclutter（0x141ce5900）：预放置条目 → add_region（键
/// calculate_hash<tuple<&str,usize,u64>> 0x141cf70d0）。
pub fn add_preplaced_autoclutter(
    mut state: ResMut<AutoClutterState>,
    preplaced: Res<PreplacedAutoClutter>,
) {
    for (i, entry) in preplaced.entries.iter().enumerate() {
        let key = hash_preplaced_key(entry.name, i, entry.host);
        let region = AutoClutterRegion {
            key: ClutterRegionId(key),
            center: entry.position,
            basis: [
                Vec2::new(entry.rotation.cos(), entry.rotation.sin()),
                Vec2::new(-entry.rotation.sin(), entry.rotation.cos()),
            ],
            candidate_index: ClutterCandidateIndex(i as u16),
            epoch: state.epoch,
        };
        state.add_region(ClutterRegionId(key), &region);
    }
}

/// 预放置键（`calculate_hash` 0x141cf70d0 语义）。
pub fn hash_preplaced_key(name: &str, i: usize, host: u64) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut h);
    i.hash(&mut h);
    host.hash(&mut h);
    h.finish()
}
