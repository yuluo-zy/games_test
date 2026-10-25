//! add_tree_vegetation_setdressing（B 主基线；0x1422d3800，size 2566）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译 + 逐指令反汇编重建
//! （`evidence/decomp/0x1422d3800_*.c`、`evidence/disasm6_0.txt`；常量实读
//! `0x142a7c88c=0.9`、`0x142925984=0.1`、`0x1429cd128=1.2`、`0x142a16cb8=0.8`、
//! `0x1429ddce8=-0.05`、`0x1429eb6f8=-0.1`、`0x142a7fa50=-0.8`、`0x1429259b0=1.3`、
//! `0x142925970=0.4`）。
//!
//! 控制流（O）：
//! - 每树：`PlacementDataSsbo::position` ×2 取位；`is_dragging` / `is_started` /
//!   `is_corrupted` 多重门控：
//!   `bVar7 = !is_dragging && 树记录 kind(+0x2c)==2`。
//! - 半径 = `树 scale(+0xc) + 0.9`，且 `is_started || is_corrupted` 时再 +0.4。
//! - `Circle2d::new(center.xz, radius)` + `init_rng(树 id)`。
//! - 内层 **12 次**：`u = rem_euclid(i/12 + f32×0.1, 1)` → `get_pos_at_u`；
//!   两次 `f32` 抖动（bVar7 或 started/corrupted 时 ×1，否则 ×0.5）加到位；
//!   4 角 bilinear（130.0）采样，`0.0 < sample` 才放。
//! - 入水后两次 `f32` 归一化方向；键 `calculate_hash<tuple<TreeStorageIndex,usize,&str>>`
//!   （树 id、i、"tree" len 4）。
//! - 偏移/缩放（O）：
//!   - started/corrupted 或 bVar7 → y = i/12 × -0.05 + base，factor 0.8；
//!   - 否则默认 y = -0.8、factor 0.8；键偶 → y = i/12 × -0.1 + base，factor 1.3。
//!   base 每轮更新。
//! - `f32` 抽取 → 缩放 `factor × (r×1.2 + (1-r)×0.8)`；写输出候选 stride 0x68。
//!
//! U 登记：base（+0x38 宿主表）与树存储（PlacementDataSsbo）为宿主查询边界，本 crate
//! 以 [`TreeEntry`] 承载；`is_corrupted` 资源以 [`IsCorrupted`] 承载；输出候选偏移为
//! 类型合同 U（同 lantern）。

use bevy_ecs::prelude::*;
use country_core::resources::world_raster::WorldRaster;
use fastrand::Rng;
use glam::{Vec2, Vec3};

use utils::geometry::circle::Circle2d;

use super::{AutoClutterSpawn, AutoClutterState, SpawnShared};

/// 树条目（宿主 `PlacementDataSsbo::position` + 树记录 kind/scale）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TreeEntry {
    /// `TreeStorageIndex`。
    pub id: u64,
    pub position: Vec3,
    /// 树记录 +0xc（scale；半径 = scale + 0.9）。
    pub scale: f32,
    /// 树记录 +0x2c（bVar7 判据 ==2）。
    pub kind: i32,
}

/// 树位置表（宿主填充；TreeStorageIndex 键）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TreePositions {
    pub entries: Vec<TreeEntry>,
}

/// 树放置 RNG（fastrand；init_rng 语义）。
#[derive(Resource, Debug)]
pub struct TreeRng(pub std::sync::Mutex<fastrand::Rng>);

impl Default for TreeRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(fastrand::Rng::with_seed(0x606f)))
    }
}

/// 拖拽状态（O 0x14092e6a0 is_dragging；宿主输入系统写）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct IsDragging(pub bool);

/// 存档损坏状态（O `BackupSaveFileKind::is_corrupted`；宿主写）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct IsCorrupted(pub bool);

/// 内层迭代数（O：i/12.0）。
pub const TREE_ITERATIONS: usize = 12;

/// add_tree_setdressing（0x1422d3800）：每树 12 个圆周 jitter 落位，入水才放（type 5）。
pub fn add_tree_setdressing(
    mut state: ResMut<AutoClutterState>,
    trees: Res<TreePositions>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    mut rng: ResMut<TreeRng>,
    started: Res<crate::autoclutter::add_lantern_water_setdresssing::IsStarted>,
    dragging: Res<IsDragging>,
    corrupted: Res<IsCorrupted>,
) {
    let w = (*water).as_ref();
    let r = &mut rng.0.lock().unwrap();
    let started_or_corrupt = started.0 || corrupted.0;
    for entry in &trees.entries {
        // O：bVar7 = !is_dragging && 树记录 kind==2。
        let bvar7 = !dragging.0 && entry.kind == 2;
        // O：半径 = scale + 0.9（0x142a7c88c），started/corrupted 再 +0.4。
        let mut radius = entry.scale + 0.9;
        if started_or_corrupt {
            radius += 0.4;
        }
        let circle = Circle2d::new(Vec2::new(entry.position.x, entry.position.z), radius);
        let mut tree_rng: Rng = utils::rng::rng_from_seed_u64(utils::rng::init_rng_u64(entry.id));
        // O：base 为宿主三张表选中值；本处以树位置 y 承载（U）。
        let mut base_y = entry.position.y;
        for i in 0..TREE_ITERATIONS {
            let base = i as f32 / TREE_ITERATIONS as f32;
            let u = (base + tree_rng.f32() * 0.1).rem_euclid(1.0);
            let point = circle.get_pos_at_u(u);
            // O：两次 f32 抖动；bVar7 或 started/corrupted 时 ×1，否则 ×0.5。
            let jitter_scale = if bvar7 || started_or_corrupt { 1.0 } else { 0.5 };
            let jitter = Vec2::new(tree_rng.f32(), tree_rng.f32()) * jitter_scale;
            let point = point + jitter;
            let s = w.bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0);
            if !(s > 0.0) {
                continue;
            }
            // O：两次 f32 → 1/SQRT 归一化方向。
            let nx = tree_rng.f32();
            let nz = tree_rng.f32();
            let inv = 1.0 / (nx * nx + nz * nz).sqrt();
            let _dir = Vec2::new(nx * inv, nz * inv);
            let key = hash_tree_key(entry.id, i);
            // O：偏移/缩放三路（started/corrupted 或 bVar7 → -0.05/0.8；
            //    否则默认 -0.8/0.8，键偶 → -0.1/1.3）。
            let (y, factor) = if started_or_corrupt || bvar7 {
                (base_y + base * -0.05, 0.8f32)
            } else if key & 1 == 0 {
                (base_y + base * -0.1, 1.3f32)
            } else {
                (-0.8, 0.8f32)
            };
            base_y = y;
            let rr = tree_rng.f32();
            let scale = factor * (rr * 1.2 + (1.0 - rr) * 0.8);
            state.spawn_candidates.push(AutoClutterSpawn {
                kind: 5,
                shared: SpawnShared::Prefab("tree".to_string()),
                f18: 2,
                key,
                marker: 5,
                position: Vec3::new(point.x, y, point.y),
                host_id: entry.id,
                f48: key,
                f50: 0,
                f54: 0,
                f58: 0,
                f5c: 0,
                scale,
            });
        }
    }
}

/// 放置键（`calculate_hash<tuple<TreeStorageIndex,usize,&str>>` 0x141cf7600 语义）。
pub fn hash_tree_key(tree_index: u64, i: usize) -> u64 {
    utils::calculate_hash(&(tree_index, i, "tree"))
}
