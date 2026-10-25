//! ivy_storage.rs — 常春藤折线存储（WP16 权威；WP21 逻辑重述）。
//!
//! O：`IvyPoint` 40B（`add_point` 0x140869460 写 0x28 步长）、`Ivy`（VecDeque<IvyPoint>
//! + `next_point_id` @+0x58）、`IvyStorage`（HashMap<usize,Ivy> + next_ivy_id +
//! PerchanceContext RNG）。函数：add_point 0x140869460、last_point 0x1408694f0、
//! last_segment_point_ids 0x140869540、spawn_ivy 0x1408696f0。

use std::collections::{HashMap, VecDeque};

use bevy_ecs::resource::Resource;
use glam::Vec3;

use crate::typed_gap::Gap;

/// 常春藤折线点（O：`Ivy::add_point` 0x140869460 写 0x28 字节 stride）：
/// `{ point_id: u64 @0x0, position: Vec3 @0x8, direction: Vec3 @0x14,
///    reserved: u32 @0x20 }`（36 有效 + 4 填充 = 40）。
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IvyPoint {
    /// 唯一点 id（O：`add_point` 从 `next_point_id` 递增写入）。
    pub point_id: u64,
    /// 世界位置（O：3 个连续 f32 @0x8）。
    pub position: Vec3,
    /// 生长方向（O：3 个连续 f32 @0x14）。
    pub direction: Vec3,
    /// 保留（O：写入 0 @0x20；typed gap）。
    pub reserved: u32,
}

/// 单株常春藤（一株折线植物）。
///
/// 布局 O（`add_point` 0x140869460）：`points: VecDeque<IvyPoint>`（B 构建观察到
/// cap@0x0 / data-ptr@0x8 / head@0x10 / len@0x18），`next_point_id: u64 @0x58`。
/// 使用 std `VecDeque` 承载折线语义；内部布局属被复用 std 容器实现，U。
#[derive(Clone, Debug, Default)]
pub struct Ivy {
    /// 折线点（O：VecDeque<IvyPoint>；`add_point` push_back）。
    pub points: VecDeque<IvyPoint>,
    /// 下一个点 id（O：`add_point` 自增并作为新点 id）。
    pub next_point_id: u64,
    /// 其他字段（U；0x20..0x58 区间，typed gap）。
    pub _gap: Gap,
}

impl Ivy {
    /// `add_point`（O 0x140869460）：push_back 一个新点，id=`next_point_id+1`，
    /// 位置/方向由调用方给出，`reserved=0`。
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

    /// `last_point`（O 0x1408694f0）：末点；空 → unwrap 失败（O panic）。
    pub fn last_point(&self) -> Option<&IvyPoint> {
        self.points.back()
    }

    /// `set_last_point_position`（O：grower 内联）：改末点位置。
    pub fn set_last_point_position(&mut self, position: Vec3) {
        if let Some(p) = self.points.back_mut() {
            p.position = position;
        }
    }

    /// `remove_last_point`（O：pruner 内联）：弹末点。
    pub fn remove_last_point(&mut self) -> Option<IvyPoint> {
        self.points.pop_back()
    }

    /// `remove_root`（O：pruner 内联）：弹首点（根）。
    pub fn remove_root(&mut self) -> Option<IvyPoint> {
        self.points.pop_front()
    }

    /// `point_count`（O：grower/pruner/branch 绘制内联）。
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    /// `get_point_id`（O：`last_segment_point_ids`/leaf 系统内联；闭包按 id 查找）。
    pub fn get_point_id(&self, point_id: u64) -> Option<&IvyPoint> {
        self.points.iter().find(|p| p.point_id == point_id)
    }

    /// `last_segment_point_ids`（O 0x140869540）：`(倒数第二点 id, 末点 id)`；
    /// 点少于 2 → expect 失败（O）。leaf_spawner 沿末段放叶。
    pub fn last_segment_point_ids(&self) -> (Option<u64>, Option<u64>) {
        let n = self.points.len();
        if n < 2 {
            return (None, None);
        }
        let mut it = self.points.iter().rev();
        let last = it.next().map(|p| p.point_id);
        let second = it.next().map(|p| p.point_id);
        (second, last)
    }
}

/// 常春藤存储资源（O：`Res<IvyStorage>`；grower/spawner/pruner 共享）。
///
/// 布局 O（`spawn_ivy` 0x1408696f0）：`HashMap<usize, Ivy>` + 自增 ivy id
/// （+0x40 计数器）+ `PerchanceContext` RNG（spawn 随机根位置）。
#[derive(Resource)]
pub struct IvyStorage {
    /// 全部常春藤，按 ivy id（O：`hashbrown HashMap<usize, Ivy>`）。
    pub ivies: HashMap<u64, Ivy>,
    /// 下一个 ivy id（O：`spawn_ivy` 结束后 +0x40 自增）。
    pub next_ivy_id: u64,
    /// 随机上下文（O：`PerchanceContext`；spawn/grow 采样）。
    pub rng: crate::reused::rng::PerchanceContext,
    /// 其他字段（U）。
    pub _gap: Gap,
}

impl Default for IvyStorage {
    fn default() -> Self {
        IvyStorage {
            ivies: HashMap::new(),
            next_ivy_id: 0,
            rng: crate::reused::rng::PerchanceContext(crate::Gap),
            _gap: Gap,
        }
    }
}

impl IvyStorage {
    /// `new`（O：`insert_ivy_resources` 内联 `IvyStorage::new`）。
    pub fn new() -> Self {
        Self::default()
    }

    /// `spawn_ivy`（O 0x1408696f0）：用 RNG 采一个随机根点，建一株新常春藤
    /// （首点 push），按新 id 入表，`next_ivy_id += 1`。
    /// 参数 I：`(root_position_hint, root_direction)` —— 调用方给出可选起点。
    pub fn spawn_ivy(&mut self, position: Vec3, direction: Vec3) -> u64 {
        let id = self.next_ivy_id;
        let mut ivy = Ivy::default();
        ivy.add_point(position, direction);
        self.ivies.insert(id, ivy);
        self.next_ivy_id += 1;
        id
    }

    /// `iter` / `iter_mut`（O：grower / send_to_gpu_segments / spawner 内联）。
    pub fn iter(&self) -> impl Iterator<Item = &Ivy> {
        self.ivies.values()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Ivy> {
        self.ivies.values_mut()
    }

    /// `get` / `remove`（O：leaf_spawner / pruner 内联）。
    pub fn get(&self, id: u64) -> Option<&Ivy> {
        self.ivies.get(&id)
    }
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Ivy> {
        self.ivies.get_mut(&id)
    }
    pub fn remove(&mut self, id: &u64) -> Option<Ivy> {
        self.ivies.remove(id)
    }

    /// `roots`（O：spawner 内联）—— 所有根的（近似）世界位置。
    pub fn roots(&self) -> Vec<Vec3> {
        self.ivies
            .values()
            .filter_map(|ivy| ivy.points.front().map(|p| p.position))
            .collect()
    }

    /// `total_segment_count`（WP21：闭包求和段数；O 名）。
    pub fn total_segment_count(&self) -> usize {
        self.ivies
            .values()
            .map(|ivy| ivy.points.len().saturating_sub(1))
            .sum()
    }
}
