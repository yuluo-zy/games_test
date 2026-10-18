//! add_preplaced_autoclutter（B 主基线；0x141ce5900，size 1566）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译重建
//! （`evidence/decomp/0x141ce5900_*.c`）。
//!
//! 控制流（O）：
//! - 两个 `Wait` 字节门控（0x141ce5e22 / 0x141ce5e68）：每帧检查记录表是否有新增；
//!   仅当无新增且门控未置位时，置位并进入放置；随后把门控置 1（一次性）。
//! - 记录表：`data@+8, len@+0x10`，stride **0x1c**；每条 `Vec3::xvy/x0y` swizzle 后
//!   写区域记录，键 `calculate_hash<tuple<&str,usize>>`（名字 + 下标）。
//! - 每记录 `AutoClutterState::add_region`；区域判别=3、候选索引初值 0xffff。
//!
//! U 登记：记录表字段名/来源（preplaced_clutter 存档数据）为宿主装配边界；本 crate 以
//! [`PreplacedEntry`] 承载位置/朝向/名字。精确 0x1c 记录布局与 swizzle 目标（位置/朝向）
//! 登记为 I/U。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::{AutoClutterRegion, AutoClutterState, ClutterCandidateIndex, ClutterRegionId};

/// 预放置自动杂物条目（宿主从存档/场景数据填充）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PreplacedAutoClutter {
    pub entries: Vec<PreplacedEntry>,
    /// 一次性门控（O：`Wait` 字节置 1 后不再放置）。
    pub placed: bool,
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
/// calculate_hash<tuple<&str,usize>> 0x141cf70d0）。
pub fn add_preplaced_autoclutter(
    mut state: ResMut<AutoClutterState>,
    mut preplaced: ResMut<PreplacedAutoClutter>,
) {
    // O：一次性门控（Wait 字节）。
    if preplaced.placed {
        return;
    }
    if preplaced.entries.is_empty() {
        return;
    }
    for (i, entry) in preplaced.entries.iter().enumerate() {
        // O：键 tuple<&str, usize>（名字 + 下标；不含 host）。
        let key = hash_preplaced_key(entry.name, i);
        // O：记录经 xvy/x0y swizzle；本处以 position + 旋转构造基向量（swizzle 目标 I/U）。
        let region = AutoClutterRegion {
            kind: 3,
            key: ClutterRegionId(key),
            center: entry.position,
            basis: [
                Vec2::new(entry.rotation.cos(), entry.rotation.sin()),
                Vec2::new(-entry.rotation.sin(), entry.rotation.cos()),
            ],
            f34: 0,
            b38: 0,
            b39: 0,
            // O：候选索引初值 0xffff。
            candidate_index: ClutterCandidateIndex::NONE,
            epoch: state.epoch,
        };
        state.add_region(ClutterRegionId(key), &region);
    }
    // O：门控置 1（一次性）。
    preplaced.placed = true;
}

/// 预放置键（`calculate_hash<tuple<&str,usize>>` 0x141cf70d0 语义）。
pub fn hash_preplaced_key(name: &str, i: usize) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut h);
    i.hash(&mut h);
    h.finish()
}
