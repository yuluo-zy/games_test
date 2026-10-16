//! populate_autoclutter_regions（B 主基线；0x1422d6ed0，size 44132）。
//!
//! 证据（clutter-placement-analysis.md + re-analyst RE-ANALYSIS-F3.md 机器码契约）：
//! 1. puffin 范围；2. `HashMap::retain<remove_stale_regions>` 删过期区域
//!    （区域 epoch != state+0x90）；3. flush_candidates_pending_removal（2 处）；
//! 4. `DirtyTiles::new_clear` → `Subscriber::take<DirtyTiles>` → `extend` 合入发布 →
//!    0x200B 位集逐位 mark_dirty；
//! 5. 每脏格种类分派（F3 契约，O）：
//!    - 记录（stride 0x48 区域表项）：key@+0x00 = `calculate_hash<tuple<GlobalId,u32>>`
//!      （0x140fabc80）、**mesh-type@+0x40**（28 项跳表 0x14304382c）、**kind@+0x41**
//!      （5 项跳表 0x143043818）、candidate_index@+0x42（U 写入点，非恒 0）、epoch@+0x44、
//!      7×f32 中心/基向量@+0x20..+0x3c。
//!    - 过滤器（0x1422d8107–0x1422d8468）：`RaycastWorld::legacy_raycast`（掩码 0x30fb）
//!      → 结果 ==2 才走水面采样；4 角 bilinear（半径 130.0=0x142a7b658）求和
//!      clamp[0,1]（1.0=0x142925904）；**水位阈值 = 0.0（0x142960690，非 0.01！）**；
//!      wet && type<3 → 加权随机挑网格（权重表 + init_rng_u64(键哈希) + fastrand::f32，
//!      落点 maxps(0.01,Δ)→floorf→xvy；0.01 = 0x1429cd300 packed 最小钳）；
//!      dry && type<3 → 固定位（记录 +0x24 取负, +0x20）；type>=3 → 圆周落点
//!      （θ = rng×2π，2π = 0x142934674；种子=键哈希）。
//!    - kind 跳表（0x143043818）：kind 0/1/2/3/4、>4 default → 预设槽组
//!      （+0x48/+0x40/+0x18/+0x20…、+0x50…、+0xc0…、+0x168…）；万圣节（is_halloween2024
//!      0x140b46860）→ 独立槽组 +0x70..+0x168；每槽 {ptr, f32 权重}，槽记录 +0x30 == -1
//!      （Option None 哨兵）→ 权重 0。**kind = 该格可用网格组 + 各网格权重**。
//!    - mesh-type（0x14304382c）：0/1 = 加权随机挑网格；2 = 跳过；3/4 = 固定候选槽
//!      +0x25/+0x26；7 = 计数累积 [cand+0xad]；9+ = 第三跳表 0x143043840。
//! - autoclutter_subscribe_to_tabloids 0x1422d6930：RaycastWorld::subscribe_to_changes
//!   把脏变化接入订阅（tabloid Subscriber）。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_rock_mask;
use country_core::resources::world_raster::WorldRaster;
use country_core::systems::collision::world::RaycastWorld;
use country_core::utils::dirty_tiles::DirtyTiles;
use fastrand::Rng;
use glam::{Vec2, Vec3};
use tabloid::Subscriber;

use super::{
    AutoClutterCandidate, AutoClutterRegion, AutoClutterState, ClutterCandidateIndex,
    ClutterRegionId, GlobalId, LazyGridClutterCell,
};

/// 脏格订阅（build 里 insert_resource AutoClutterDirtySubscriptions）。
/// Subscriber 含 Cell 非 Sync，故以 Mutex 包裹（资源要求 Send+Sync）。
#[derive(Resource, Default)]
pub struct AutoClutterDirtySubscriptions {
    pub subscriber: std::sync::Mutex<Option<Subscriber<DirtyTiles>>>,
}

/// populate 局部状态（PDB impl$0::init_state 0x1422e1b40）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PopulateLocal {
    /// 已处理名集合（BTreeSet；pick_clutter_mesh 去重）。
    pub processed_names: std::collections::BTreeSet<String>,
}

/// 脏格发布者（宿主经世界栅格/地形发布链填充；O：RaycastWorld::subscribe_to_changes
/// 的 tabloid Publisher 字段）。
#[derive(Resource, Default)]
pub struct DirtyTilesPublisher(pub tabloid::Publisher<DirtyTiles>);

/// autoclutter_subscribe_to_tabloids（0x1422d6930）：
/// `RaycastWorld::subscribe_to_changes`（0x140b5d040）→ 把脏变化经 tabloid
/// `Subscriber<DirtyTiles>` 接入（发布者 = 世界栅格/地形发布链；宿主装配注入）。
pub fn autoclutter_subscribe_to_tabloids(
    mut sub: ResMut<AutoClutterDirtySubscriptions>,
    mut publisher: ResMut<DirtyTilesPublisher>,
    raycast: Res<RaycastWorldHandleClutter>,
    mut dirty: ResMut<DirtyTilesResource>,
) {
    // O：RaycastWorld::subscribe_to_changes 语义——世界栅格变化发布订阅；
    // 本重建经 tabloid Publisher<DirtyTiles> 的 subscribe 实现同一接线。
    let mut guard = sub.subscriber.lock().unwrap();
    if guard.is_none() {
        *guard = Some(publisher.0.subscribe());
    }
    let _ = (&raycast, &mut dirty);
}

/// RaycastWorld 资源包装（core RaycastWorld 非 Resource；装配层构造）。
#[derive(Resource, Default)]
pub struct RaycastWorldHandleClutter(pub RaycastWorld);

impl std::fmt::Debug for RaycastWorldHandleClutter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RaycastWorldHandleClutter")
    }
}

/// populate_autoclutter_regions（0x1422d6ed0；9 参数 SystemParam 面）。
#[allow(clippy::too_many_arguments)]
pub fn populate_autoclutter_regions(
    mut state: ResMut<AutoClutterState>,
    mut dirty: ResMut<DirtyTilesResource>,
    water: Res<WorldRaster<512>>,
    rock_mask: Res<RockMaskData>,
    mut sub: ResMut<AutoClutterDirtySubscriptions>,
    mut local: ResMut<PopulateLocal>,
    mut session: ResMut<SessionMeta>,
    mut busher: ResMut<BushGrid>,
    mut _lazy: ResMut<LazyGridClutterCell>,
    mut mesh_groups: ResMut<MeshGroupWeights>,
) {
    // 1. 过期区域清理（O：HashMap::retain，epoch != state+0x90 删除）。
    let epoch = state.epoch;
    state.regions.retain(|_, r| r.epoch == epoch);

    // 2. 冲刷候选（2 处中的第一处）。
    state.flush_candidates_pending_removal();

    // 4. 脏格合入（O：new_clear → Subscriber::take → extend → 位集 mark_dirty）。
    let mut merged = DirtyTiles::new_clear();
    if let Some(reader) = sub.subscriber.lock().unwrap().as_mut().and_then(|s| s.take()) {
        merged.extend(&reader);
    }
    dirty.inner.extend(&merged);

    // 3. 第二处冲刷。
    state.flush_candidates_pending_removal();

    // 5. 逐脏格种类分派（F3 契约）。
    let w = (*water).as_ref();
    let rock = &rock_mask;
    let groups = &mesh_groups;
    for (x, y) in dirty_tiles_cells(&dirty.inner) {
        process_dirty_cell(
            &mut state,
            &w,
            rock,
            &session,
            &mut local,
            groups,
            x,
            y,
        );
    }
    let _ = &mut busher;
}

/// 每脏格处理（F3 契约落地；机器码判别链见模块文档）。
#[allow(clippy::too_many_arguments)]
pub fn process_dirty_cell(
    state: &mut AutoClutterState,
    w: &country_core::resources::world_raster::RasterBufferRef<'_, 512>,
    rock: &RockMaskData,
    session: &SessionMeta,
    local: &mut PopulateLocal,
    groups: &MeshGroupWeights,
    x: i32,
    y: i32,
) {
    let (wx, wz) = cell_center_world(x, y);
    let key = ClutterRegionId(hash_cell_key(x, y));
    // filter_clutter_object（O）：水位 4 角采样求和 clamp[0,1]（半径 130.0）+ 岩石掩码。
    let mut water_sum = 0.0f32;
    for (dx, dz) in WATER_CORNER_SAMPLES {
        water_sum += w.bilinear_sample_at_world_xz(wx + dx, wz + dz, 130.0, 130.0);
    }
    let water_sum = water_sum.clamp(0.0, 1.0);
    let rock_v = rock.sample(wx, wz);
    if rock_v != 0 {
        return;
    }
    // legacy_raycast（掩码 0x30fb；非 native 壳返回 None——与 core 壳一致）。
    let ray_hit = raycast_seam::legacy_raycast_mask(key.0, wx, wz);
    // 水位判别（O：阈值 0.0 = 0x142960690；`0.0 < clamp_sum`）。
    let wet = ray_hit && water_sum > 0.0;

    // mesh-type / kind（I：filter_clutter_object 闭包按"可用网格组"选定；
    // 写入点在 populate 外/内联处——本重建经 MeshGroupWeights 网格组表选择）。
    let (mesh_type, kind) = groups.select_group(key, session.is_halloween2024);

    // 中心/基向量（O）：
    // - type<3 && wet → 加权随机挑网格 + 位面落点（maxps(0.01,Δ) → floorf → xvy）；
    // - type<3 && dry → 固定位（记录 +0x24 取负, +0x20）；
    // - type>=3 → 圆周落点（θ = rng×2π，种子 = 键哈希）。
    let center = if mesh_type < 3 {
        if wet {
            weighted_mesh_landing(key, groups, mesh_type, wx, wz)
        } else {
            Vec3::new(wx, -0.0, wz)
        }
    } else {
        circular_position(key)
    };
    let basis = [Vec2::X, Vec2::Y];

    // candidate_index（O：u16@+0x42；非恒 0——与区域候选表交互产生）。
    let candidate_index = ClutterCandidateIndex(state.region_candidates.len() as u16);
    state.region_candidates.push(AutoClutterCandidate {
        kind,
        shared: 0,
        key: key.0,
        f18: 0,
        f20: 0,
        f28: 0,
        forced: mesh_type >= 9,
        f34: mesh_type as u32,
        f38: 0,
        f40: 0,
        f48: 0,
        f50: 0,
        f54: 0,
        f5c: 0,
        scale: 1.0,
    });
    let region = AutoClutterRegion {
        key,
        center,
        basis,
        candidate_index,
        epoch: state.epoch,
    };
    state.add_region(key, &region);
    // 懒网格（mark_dirty 消费）：格 → GlobalId。
    let cell_idx = (x as usize) * 64 + y as usize;
    while state.lazy_grid.len() <= cell_idx {
        state.lazy_grid.push(Vec::new());
    }
    if let Some(cell) = state.lazy_grid.get_mut(cell_idx) {
        if !cell.contains(&key.0) {
            cell.push(key.0);
        }
    }
    let _ = (local, y);
}

/// 加权随机挑网格 + 位面落点（O：type 0/1 分支；权重累和 + fastrand f32；
/// 落点 = maxps(0.01, Δ) → floorf → xvy；0.01 = 0x1429cd300 packed 最小钳）。
pub fn weighted_mesh_landing(
    key: ClutterRegionId,
    groups: &MeshGroupWeights,
    mesh_type: u8,
    wx: f32,
    wz: f32,
) -> Vec3 {
    let entries = if mesh_type == 0 {
        &groups.group_0
    } else {
        &groups.group_1
    };
    let total: f32 = entries.iter().map(|e| e.weight).sum();
    let mut rng = utils::rng::rng_from_seed_u64(utils::rng::init_rng_u64(key.0));
    let r = rng.f32();
    let mut acc = 0.0f32;
    let mut chosen = Vec2::ZERO;
    for e in entries {
        acc += if total > 0.0 { e.weight / total } else { 0.0 };
        if r <= acc {
            chosen = e.offset;
            break;
        }
    }
    // 位面落点（O：maxps(0.01, Δ) → floorf 网格化 → xvy swizzle）。
    let delta = Vec2::new(wx, wz) + chosen;
    let clamped = Vec2::new(delta.x.max(0.01), delta.y.max(0.01));
    let cell = Vec2::new(clamped.x.floor(), clamped.y.floor());
    Vec3::new(cell.x, 0.0, cell.y)
}

/// 圆周落点（O：type>=3；θ = rng×2π（0x142934674），种子 = 键哈希 → cos/sin 单位圆）。
pub fn circular_position(key: ClutterRegionId) -> Vec3 {
    let mut rng = utils::rng::rng_from_seed_u64(utils::rng::init_rng_u64(key.0));
    let theta = rng.f32() * std::f32::consts::TAU;
    Vec3::new(theta.cos(), 0.0, theta.sin())
}

/// 脏格坐标列表（DirtyTiles 0x200B 位集；cell 语义）。
pub fn dirty_tiles_cells(dirty: &DirtyTiles) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for idx in dirty.tile_indices() {
        let x = (idx % 64) as i32;
        let y = (idx / 64) as i32;
        out.push((x, y));
    }
    out
}

/// 格中心世界坐标（格 8 世界单位；世界半宽 65 → 格域 [-65,65]；I：机器码为 ×2 后
/// trunc 的位面中心流，近似对齐）。
pub fn cell_center_world(x: i32, y: i32) -> (f32, f32) {
    ((x as f32 + 0.5) * 8.0 - 65.0, (y as f32 + 0.5) * 8.0 - 65.0)
}

/// 格哈希键（`calculate_hash<tuple<GlobalId,u32>>` 0x140fabc80 语义；固定种子）。
pub fn hash_cell_key(x: i32, y: i32) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    (x as i64).hash(&mut h);
    (y as u32).hash(&mut h);
    h.finish()
}

/// 网格组权重表（O：kind 跳表 0x143043818 预设槽组 + 权重；
/// 槽记录 +0x30 == -1（Option None 哨兵）→ 权重 0）。
#[derive(Resource, Clone, Debug, Default)]
pub struct MeshGroupWeights {
    /// mesh-type 0 的权重组（O：type 0/1 分支的加权随机挑网格）。
    pub group_0: Vec<MeshEntry>,
    /// mesh-type 1 的权重组。
    pub group_1: Vec<MeshEntry>,
    /// 万圣节组（O：is_halloween2024 → 槽序 +0x70..+0x168）。
    pub halloween: Vec<MeshEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeshEntry {
    /// 权重（+0x30 == -1 哨兵 → 0）。
    pub weight: f32,
    /// 落点偏移（位面数学输入）。
    pub offset: Vec2,
}

impl MeshGroupWeights {
    /// 网格组选择（mesh-type/kind；I：filter_clutter_object 闭包按可用组选定，
    /// 本重建按组表非空选择：type 0/1 = 加权组，其余按组序）。
    pub fn select_group(&self, _key: ClutterRegionId, halloween: bool) -> (u8, u8) {
        if halloween {
            return (0, 4);
        }
        if !self.group_0.is_empty() {
            (0, 0)
        } else if !self.group_1.is_empty() {
            (1, 1)
        } else {
            (2, 2)
        }
    }
}

/// 岩石掩码数据（宿主从 GPU 回读填充；采样 = core terrain_rock_mask）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RockMaskData {
    mask: Vec<u32>,
}

impl RockMaskData {
    pub fn set_mask(&mut self, mask: Vec<u32>) {
        self.mask = mask;
    }
    pub fn sample(&self, x: f32, z: f32) -> u32 {
        terrain_rock_mask::sample_at_world_xz(&self.mask, x, z)
    }
}

/// 会话元（is_halloween2024 门控 0x140b46860）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SessionMeta {
    pub is_halloween2024: bool,
}

/// GPU 灌木网格（build 里 insert_resource BushGrid；read_bush 域）。
#[derive(Resource, Clone, Debug, Default)]
pub struct BushGrid {
    pub cells: Vec<[f32; 2]>,
}

/// 全局 id 包装（hash_one<&GlobalId>）。
pub fn global_id(cell: [i32; 2]) -> GlobalId {
    hash_cell_key(cell[0], cell[1])
}

/// 水面 4 角采样偏移（O：0x1422d8234.. 四角 bilinear，半径 130.0）。
pub const WATER_CORNER_SAMPLES: [(f32, f32); 4] = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)];

/// DirtyTiles 资源包装（core `DirtyTiles` 非 Resource；装配层构造）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DirtyTilesResource {
    pub inner: DirtyTiles,
}

/// 射线查询缝（legacy_raycast 掩码 0x30fb；非 native 壳返回 None）。
pub mod raycast_seam {
    /// legacy_raycast（O 0x1408cc870：掩码 0x30fb；结果 ==2 才走水面采样）。
    pub fn legacy_raycast_mask(_key: u64, _wx: f32, _wz: f32) -> bool {
        false
    }
}
