//! ivy_storage.rs — 常春藤折线存储（WP16 逻辑重述；拆分到骨架子路径）。
//!
//! O：`IvyPoint` 40B（`add_point` 0x140869460 写 0x28 步长）、`Ivy`（VecDeque<IvyPoint>
//! + `next_point_id` @+0x58）、`IvyStorage`（HashMap<usize,Ivy> + next_ivy_id +
//! PerchanceContext RNG）。函数：add_point 0x140869460、last_point 0x1408694f0、
//! last_segment_point_ids 0x140869540、spawn_ivy 0x1408696f0。

use std::collections::{HashMap, VecDeque};

use bevy_ecs::resource::Resource;
use glam::{Vec2, Vec3};

use crate::typed_gap::Gap;

/// 常春藤折线点（O：`{ point_id: u64 @0x0, position: Vec3 @0x8, direction: Vec3 @0x14,
/// reserved: u32 @0x20 }` = 40B）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IvyPoint {
    pub point_id: u64,
    pub position: Vec3,
    pub direction: Vec3,
    pub reserved: u32,
}

/// 单株常春藤（O：VecDeque<IvyPoint> 折线 + next_point_id）。
#[derive(Clone, Debug, Default)]
pub struct Ivy {
    pub points: VecDeque<IvyPoint>,
    pub next_point_id: u64,
    /// 其他字段（U；0x20..0x58 区间）。
    pub _gap: Gap,
}

impl Ivy {
    /// `add_point`（O 0x140869460）。
    pub fn add_point(&mut self, position: Vec3, direction: Vec3) -> u64 {
        let id = self.next_point_id + 1;
        self.next_point_id = id;
        self.points.push_back(IvyPoint {
            point_id: id,
            position,
            direction,
            reserved: 0,
        });
        id
    }
    /// `last_point`（O 0x1408694f0：末点；空 unwrap 失败）。
    pub fn last_point(&self) -> Option<&IvyPoint> {
        self.points.back()
    }
    /// `last_segment_point_ids`（O 0x140869540：末两段端点 id，len<2 expect 失败）。
    pub fn last_segment_point_ids(&self) -> (u64, u64) {
        let n = self.points.len();
        assert!(n >= 2, "last_segment_point_ids");
        (self.points[n - 2].point_id, self.points[n - 1].point_id)
    }
    /// `set_last_point_position`（O：grower 内联）。
    pub fn set_last_point_position(&mut self, position: Vec3) {
        if let Some(p) = self.points.back_mut() {
            p.position = position;
        }
    }
    /// `remove_last_point`（O：pruner 内联）。
    pub fn remove_last_point(&mut self) -> Option<IvyPoint> {
        self.points.pop_back()
    }
    /// `remove_root`（O：pruner 内联）。
    pub fn remove_root(&mut self) -> Option<IvyPoint> {
        self.points.pop_front()
    }
    /// `point_count`（O）。
    pub fn point_count(&self) -> usize {
        self.points.len()
    }
    /// `get_point_id`（O 名，内联）。
    pub fn get_point_id(&self, index: usize) -> Option<u64> {
        self.points.get(index).map(|p| p.point_id)
    }
}

/// `IvyStorage`（O：HashMap<usize,Ivy> + next_ivy_id + PerchanceContext RNG）。
#[derive(Resource, Clone, Debug, Default)]
pub struct IvyStorage {
    pub ivies: HashMap<usize, Ivy>,
    pub next_ivy_id: usize,
    /// RNG（O：PerchanceContext `usize_less_than`/`uniform_f32`；perchance 0.4.0 复用边界）。
    pub rng: Gap,
}

impl IvyStorage {
    /// `new`（O：insert_ivy_resources 内联 0x1409440c0）。
    pub fn new() -> Self {
        Self::default()
    }
    /// `spawn_ivy`（O 0x1408696f0）：新 ivy id，插入空 Ivy。
    pub fn spawn_ivy(&mut self) -> usize {
        let id = self.next_ivy_id + 1;
        self.next_ivy_id = id;
        self.ivies.insert(id, Ivy::default());
        id
    }
    pub fn iter(&self) -> impl Iterator<Item = (&usize, &Ivy)> {
        self.ivies.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&usize, &mut Ivy)> {
        self.ivies.iter_mut()
    }
    pub fn roots(&self) -> impl Iterator<Item = &Ivy> {
        self.ivies.values()
    }
    pub fn get(&self, id: usize) -> Option<&Ivy> {
        self.ivies.get(&id)
    }
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Ivy> {
        self.ivies.get_mut(&id)
    }
    pub fn remove(&mut self, id: usize) -> Option<Ivy> {
        self.ivies.remove(&id)
    }
    /// `total_segment_count`（O：闭包求和段数）。
    pub fn total_segment_count(&self) -> usize {
        self.ivies
            .values()
            .map(|ivy| ivy.points.len().saturating_sub(1))
            .sum()
    }
}
