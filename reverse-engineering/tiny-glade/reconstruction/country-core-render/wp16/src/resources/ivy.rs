//! ivy.rs — 常春藤生长状态族（WP16 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/ivy/{ivy_storage,ivy_leaf_storage,
//! ivy_direction_proposer,wall_coord,mod}.rs`。PDB 命名实体：
//! `Ivy` / `IvyPoint` / `IvyStorage` / `IvyLeaf` / `IvyLeafRng` / `IvyLeafStorage` /
//! `IvyDirectionProposer` / `IvySpawnNoise` / `WallCoord`。
//!
//! 标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。
//! 消费方：`systems::ivy`（grower/spawner/pruner/leaf 等），
//! `render_loop::draw_passes::draw_shaders::instanced_ivy`（分支绘制）。

#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

use bevy_ecs::resource::Resource;
use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

/// 常春藤折线点（O：`Ivy::add_point` 0x140869460 写 0x28 字节 stride）：
/// `{ point_id: u64 @0x0, position: Vec3 @0x8, direction: Vec3 @0x14,
///    reserved: u32 @0x20 }`（36 有效 + 4 填充 = 40）。
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
}

/// 叶（O：`resources::ivy::ivy_leaf_storage` 实体；leaf_spawner 生成）。
/// 布局 U（typed gap；位置/朝向/尺寸）。
#[derive(Clone, Copy, Debug, Default)]
pub struct IvyLeaf(pub Gap);

/// 叶随机上下文（O：实体；叶位置采样用）。
#[derive(Clone, Copy, Debug, Default)]
pub struct IvyLeafRng(pub Gap);

/// 叶存储资源（O：`Res<IvyLeafStorage>`；`add`/`all_leaves`/`all_leaves_mut`/
/// `delete_all_leaves`/`delete_affected_leaves`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct IvyLeafStorage {
    /// 全部叶（O：`all_leaves` 内联读全量；容器类型 U）。
    pub leaves: Vec<IvyLeaf>,
}

impl IvyLeafStorage {
    /// `add`（O 0x1408fe280）。
    pub fn add(&mut self, leaf: IvyLeaf) {
        self.leaves.push(leaf);
    }
    pub fn all_leaves(&self) -> &[IvyLeaf] {
        &self.leaves
    }
    pub fn all_leaves_mut(&mut self) -> &mut [IvyLeaf] {
        &mut self.leaves
    }
    /// `delete_all_leaves`（O：pruner 内联）。
    pub fn delete_all_leaves(&mut self) {
        self.leaves.clear();
    }
    /// `delete_affected_leaves`（O：pruner/pruner_cmd 内联）。
    pub fn delete_affected_leaves(&mut self, _affected: &[u64]) {
        self.leaves.clear();
    }
}

/// 常春藤方向提议器（O：`Res<IvyDirectionProposer>`；grower 采样生长方向）。
///
/// 布局 O（`impl$0::default` 0x1412343b0）：两个 `bracket_noise::FastNoise`
/// （seeded + fractal octaves/gain + seed）+ 参数（0x3f800000=1.0、0x40000000=2.0、
/// 0x3fa66666=1.3，O 常量）。
#[derive(Resource)]
pub struct IvyDirectionProposer {
    /// 场 A（O：bracket_noise FastNoise）。
    pub noise_a: crate::reused::noise::FastNoise,
    /// 场 B（O）。
    pub noise_b: crate::reused::noise::FastNoise,
    /// 种子/参数（O 常量见 default；typed gap）。
    pub _params: Gap,
}

impl Default for IvyDirectionProposer {
    fn default() -> Self {
        IvyDirectionProposer {
            noise_a: crate::reused::noise::FastNoise::seeded(0),
            noise_b: crate::reused::noise::FastNoise::seeded(0),
            _params: Gap,
        }
    }
}

impl IvyDirectionProposer {
    /// `new`（O：`insert_ivy_resources` 内联 `IvySpawnNoise::new`；这里合并到 default）。
    pub fn new() -> Self {
        Self::default()
    }

    /// `get_direction`（O 0x141234590）：3×noise3d + 3×uniform_f32 混合，
    /// 得到 `θ = ((v*80 - (1-v)*80) * DEG2RAD * 0.5)`（v 归一化到 [0,1]），
    /// 返回 `(cos θ, sin θ)`。I：噪声采样的具体坐标映射。
    pub fn get_direction(&mut self, _pos: Vec3) -> Vec2 {
        // O：θ 范围对称 [-40°, +40°]（80° × 0.5）；实际 noise 采样 U。
        let theta = 0.0f32; // U
        Vec2::new(theta.cos(), theta.sin())
    }
}

/// 常春藤生成噪声参数（O：PDB 实体；`insert_ivy_resources` 内联）。
#[derive(Clone, Copy, Debug, Default)]
pub struct IvySpawnNoise(pub Gap);

/// 墙坐标（O：`resources::ivy::wall_coord::WallCoord`）。
///
/// 布局 I：`{ u: f32, v: f32 }` —— 沿墙曲线参数 u 与高度 v。
/// `into_world_space`（O 0x140b199e0）：读 `PublicWallState` 曲线总长（+0x30）、
/// 曲线点表（+8 ptr/+0x10 len，每点 Vec2），以 u/总长 采样 `Curve::get_coord_at_u`
/// 得墙点，沿墙面法线偏移 `0.63 * 0.5`（O 常量），按侧（bool）取 ±。
#[derive(Clone, Copy, Debug, Default)]
pub struct WallCoord {
    /// 沿墙参数（I）。
    pub u: f32,
    /// 高度（I）。
    pub v: f32,
}

impl WallCoord {
    /// `into_world_space`（O 0x140b199e0）：`(u, v, 墙数据, 墙侧)` → 世界位置。
    /// 精确曲线几何 U（依赖墙公共状态曲线）。
    pub fn into_world_space(
        &self,
        _wall: &crate::systems::collision::world::WallCurveSource,
        _side: bool,
    ) -> Vec3 {
        Vec3::ZERO
    }
}
