//! populate_autoclutter_regions（B 主基线；0x1422d6ed0，size 44132）。
//!
//! 证据（clutter-placement-analysis.md + re-analyst RE-ANALYSIS-F3.md 机器码契约 +
//! wave2 全量 120KB 反编译 `evidence/decomp/populate_autoclutter_regions_1422d6ed0.c`）：
//! 1. puffin 范围；2. `HashMap::retain<remove_stale_regions>` 删过期区域
//!    （区域 epoch != state+0x90）；3. flush_candidates_pending_removal（2 处）；
//! 4. `DirtyTiles::new_clear` → `Subscriber::take<DirtyTiles>` → `extend` 合入发布 →
//!    0x200B 位集逐位 mark_dirty；
//! 5. 逐区域记录（stride 0x48，懒网格格内）分派（wave2 全量反编译）：
//!    - **kind switch**（记录+0x41，`switch(*(char*)(local_138-7))`）：case 0/2/3/4 各建
//!      9/25/25/37 槽的 {网格表指针, f32 权重} 缓冲（stride 0x10），case 1 → 独立处理器
//!      0x1422dd1d7，其余 → default；**万圣节（is_halloween2024 0x140b46860）→ 跳过
//!      kind switch**（local_180 恒 0）。每槽 `[entry+0x30] == -1`（Option None 哨兵）
//!      → 权重 0。
//!    - **mesh-type 跳表**（记录+0x47，DAT_14304382c）：0/1 = 加权随机挑网格
//!      （`init_rng_u64(键哈希)` + `fastrand::f32` + 权重/total 累和选中；落点
//!      `maxps(0.01,Δ)→floorf→xvy`，0.01 = 0x1429cd300）；2 = 跳过；3/4 = 固定候选槽
//!      +0x25/+0x26；7 = 计数累积 `[cand+0xad]++`；5/6/8/11/12/13/14/15/18/19 → 透传
//!      （code_r0x0001422d7d8e）；9/0xa/0x11/0x14/0x15/0x10/0x18 → 错误/panic 路径。
//!    - 过滤器（0x1422d8107–0x1422d8468）：`RaycastWorld::legacy_raycast`（掩码 0x30fb）
//!      → 结果 ==2 才走水面采样；4 角 bilinear（半径 130.0=0x142a7b658）求和
//!      clamp[0,1]（1.0=0x142925904）；**水位阈值 = 0.0（0x142960690）**；
//!      水面 > 0 → 落点 + 写候选/区域。
//!
//! U 登记（不伪造）：① 网格表 `local_100[n]` → 具体 ClutterMeshTy 资产清单 = U
//! （需 load_clutter_meshes 资产清单对照，本批登记不判等）；② kind 1 处理器
//! 0x1422dd1d7 body、default（kind 5–25）槽表 = U（反编译未完整还原）；③ 区域记录
//! stride 0x48 逐字节布局 = U（现以 AutoClutterRegion 语义承载）；④ kind 4 权重经
//! memmove 打包循环写出，部分权重寄存器复用歧义 = U。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_rock_mask;
use country_core::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use country_core::resources::world_raster_defs::{GardenRaster, PathRaster, WaterRaster};
use country_core::systems::collision::world::RaycastWorld;
use country_core::utils::dirty_tiles::DirtyTiles;
use glam::{Vec2, Vec3};
use tabloid::Subscriber;

use super::{
    AutoClutterCandidate, AutoClutterRegion, AutoClutterState, ClutterCandidateIndex,
    ClutterRegionId, GlobalId, LazyGridClutterCell,
};

/// 脏格订阅（build 里 insert_resource AutoClutterDirtySubscriptions）。
/// Subscriber 含 Cell 非 Sync，故以 Mutex 包裹（资源要求 Send+Sync）。
/// O（0x1422d6930）：13 个订阅句柄入 Vec——8× `RaycastWorld::subscribe_to_changes`、
/// `BushGrid` 与 `TerrainHeightsData` 各 1× `Publisher<DirtyTiles>::subscribe`、`PathRaster`/
/// `WaterRaster`/`GardenRaster` 各 1× `WorldRaster::subscribe`；填充见
/// [`autoclutter_subscribe_to_tabloids`]。
#[derive(Resource, Default)]
pub struct AutoClutterDirtySubscriptions {
    pub subscriber: std::sync::Mutex<Vec<Subscriber<DirtyTiles>>>,
}

/// populate 局部状态（PDB impl$0::init_state 0x1422e1b40）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PopulateLocal {
    /// 已处理名集合（BTreeSet；pick_clutter_mesh 去重）。
    pub processed_names: std::collections::BTreeSet<String>,
}

/// `autoclutter_subscribe_to_tabloids`（Startup，0x1422d6930）。
///
/// 机器码证据：FunctionSystem 的 `validate_param_unsafe`（0x141ced3b0；错误串自 0x142f32706 起）依序给出
/// 七个 `ResMut` 参数：`AutoClutterDirtySubscriptions`、`RaycastWorld`、`BushGrid`、`PathRaster`、
/// `WaterRaster`、`GardenRaster`、`TerrainHeightsData<TerrainHeightSsbo>`。函数体按 O 顺序把 13 个
/// `Subscriber<DirtyTiles>` 句柄 push 入第一个资源的 Vec：
/// 8× `RaycastWorld::subscribe_to_changes(world, slot)`（0x1408cdc20，槽序 0,1,3,4,5,6,7,2）→
/// `BushGrid` 内联发布者（`*grid + 0x18`）`Publisher<DirtyTiles>::subscribe` →
/// `PathRaster`/`WaterRaster`/`GardenRaster` 各经 `TypedStorageBuffer::buffer()` 取内层
/// `WorldRaster<512>` 再 `WorldRaster::subscribe`（0x140a158e0）→
/// `TerrainHeightsData` 内联发布者（`*data + 0x18`）`Publisher<DirtyTiles>::subscribe`。
pub fn autoclutter_subscribe_to_tabloids(
    mut sub: ResMut<AutoClutterDirtySubscriptions>,
    mut raycast: ResMut<RaycastWorld>,
    mut bush: ResMut<BushGrid>,
    mut path: ResMut<PathRaster>,
    mut water: ResMut<WaterRaster>,
    mut garden: ResMut<GardenRaster>,
    mut terrain: ResMut<TerrainHeightsData<TerrainHeightSsbo>>,
) {
    let mut guard = sub.subscriber.lock().unwrap();
    for slot in [0u8, 1, 3, 4, 5, 6, 7, 2] {
        guard.push(raycast.subscribe_to_changes(slot));
    }
    guard.push(bush.publisher.subscribe());
    guard.push(path.subscribe());
    guard.push(water.subscribe());
    guard.push(garden.subscribe());
    guard.push(terrain.subscribe());
}

/// populate_autoclutter_regions（0x1422d6ed0；9 参数 SystemParam 面）。
#[allow(clippy::too_many_arguments)]
pub fn populate_autoclutter_regions(
    mut state: ResMut<AutoClutterState>,
    mut dirty: ResMut<DirtyTilesResource>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
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
    {
        let mut guard = sub.subscriber.lock().unwrap();
        if let Some(first) = guard.first_mut() {
            if let Some(reader) = first.take() {
                merged.extend(&reader);
            }
        }
    }
    dirty.inner.extend(&merged);

    // 3. 第二处冲刷。
    state.flush_candidates_pending_removal();

    // 5. 逐脏格种类分派（F3 契约）。
    let w = (*water).as_ref();
    let rock = &rock_mask;
    let groups = &mesh_groups;
    for (x, y) in dirty_tiles_cells(&dirty.inner) {
        process_dirty_cell(&mut state, &w, rock, &session, &mut local, groups, x, y);
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

    // mesh-type / kind（O：kind switch → KIND_MESH_SLOTS 槽表；mesh-type 跳表选网格。
    // 区域记录 kind@+0x41 / mesh-type@+0x47 为存储值；本每格重建（非逐记录）以可用槽组
    // 近似派生 kind（万圣节 → 4，否则 0），mesh-type 取 0（加权随机分支）——U：逐记录
    // 派生未取证）。
    let kind: u8 = if session.is_halloween2024 { 4 } else { 0 };
    let mesh_type: u8 = 0;
    let chosen_mesh = select_mesh(mesh_type, kind, key);

    // 中心/基向量（O）：
    // - type<3 && wet → 加权随机挑网格 + 位面落点（maxps(0.01,Δ) → floorf → xvy）；
    // - type<3 && dry → 固定位（记录 +0x24 取负, +0x20）；
    // - type>=3 → 圆周落点（θ = rng×2π，种子 = 键哈希）。
    let center = if mesh_type < 3 {
        if wet && chosen_mesh.is_some() {
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
        forced: if mesh_type >= 9 { 6 } else { 0 },
        f34: mesh_type as u32,
        f38: 0,
        f40: 0,
        f48: 0,
        f50: 0,
        f54: 0,
        f5c: 0,
        f60: 1.0f32.to_bits(),
        region_key: key.0,
        ..AutoClutterCandidate::default()
    });
    let region = AutoClutterRegion {
        kind: kind as u32,
        key,
        center,
        basis,
        f34: 0,
        b38: 0,
        b39: 0,
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
    utils::calculate_hash(&((x as i64), (y as u32)))
}

/// kind → 网格槽表（O：kind 跳表 0x143043818 各 case 写块，wave2 全量反编译逐槽提取；
/// 槽 = {网格表 `local_100` 下标, f32 权重}。网格表下标 → 具体 ClutterMeshTy 资产 =
/// U（①登记，需 load_clutter_meshes 资产清单对照））。
///
/// 各槽权重源码形如 `[entry+0x30] == -1 → 权重=0`（Option None 哨兵）。case 0/2/3 的
/// 权重经写块直接提取（O）；case 4 经 memmove 打包循环（0x18..0x278 stride 0x20）写出，
/// 槽序确定、部分权重寄存器复用歧义（④登记，取提取值）。
const KIND_MESH_SLOTS: &[(u8, &[(u8, f32)])] = &[
    // case '\0'：9 槽（+0x00..+0x40；槽 8（mesh 10）权重 = `(float)local_108 * fVar53`，
    // local_108 写前经 `CONCAT44(...,local_108*1.5)` 更新（行 768）→ scale×1.5 链，
    // scale=1.0（mesh-type≠1）→ 1.5）。
    (
        0,
        &[
            (9, 1.2),
            (8, 1.2),
            (3, 2.5),
            (4, 1.0),
            (5, 1.0),
            (6, 1.0),
            (7, 1.0),
            (10, 1.5),
            (11, 1.0),
        ],
    ),
    // case '\x02'：25 槽（+0x00..+0xc0；槽 0 权重 = scale*1.5，槽 14 = fVar61+fVar61=2.0）。
    (
        2,
        &[
            (10, 1.5),
            (25, 0.75),
            (24, 0.5),
            (23, 1.0),
            (21, 0.75),
            (20, 0.75),
            (19, 0.5),
            (17, 0.75),
            (16, 0.75),
            (22, 0.75),
            (14, 1.0),
            (15, 1.0),
            (9, 1.2),
            (8, 1.2),
            (3, 2.0),
            (18, 1.5),
            (4, 1.0),
            (5, 1.0),
            (6, 1.0),
            (7, 1.0),
            (11, 1.0),
            (28, 0.5),
            (29, 0.5),
            (26, 0.5),
            (27, 0.5),
        ],
    ),
    // case '\x03'：25 槽（+0x00..+0xc0；末槽权重 = scale*1.0）。
    (
        3,
        &[
            (25, 1.0),
            (24, 1.0),
            (23, 1.0),
            (21, 1.0),
            (20, 1.0),
            (19, 1.0),
            (17, 0.75),
            (16, 0.75),
            (22, 0.5),
            (14, 0.75),
            (15, 0.75),
            (18, 0.75),
            (28, 1.0),
            (29, 1.0),
            (26, 1.2),
            (27, 1.0),
            (9, 1.0),
            (8, 1.0),
            (3, 1.0),
            (4, 1.0),
            (5, 1.0),
            (6, 1.0),
            (7, 1.0),
            (10, 1.0),
            (11, 1.0),
        ],
    ),
    // case '\x04'：37 槽（+0x00..+0x238；memmove 打包循环写出，权重为提取值/1.0 主）。
    (
        4,
        &[
            (45, 1.0),
            (56, 1.0),
            (57, 1.0),
            (39, 1.0),
            (40, 1.0),
            (50, 1.0),
            (48, 1.0),
            (49, 1.0),
            (51, 1.0),
            (52, 1.0),
            (46, 1.0),
            (25, 1.0),
            (24, 1.0),
            (23, 1.0),
            (21, 1.0),
            (20, 1.0),
            (19, 1.0),
            (17, 1.0),
            (16, 1.0),
            (22, 1.0),
            (13, 1.0),
            (14, 1.0),
            (15, 1.0),
            (18, 1.0),
            (28, 1.0),
            (29, 1.0),
            (26, 1.0),
            (27, 1.0),
            (9, 1.0),
            (8, 1.0),
            (3, 1.0),
            (4, 1.0),
            (5, 1.0),
            (6, 1.0),
            (7, 1.0),
            (10, 1.0),
            (11, 1.0),
        ],
    ),
];

/// kind → 网格槽表（None = case 1 独立处理器 / default kind 5–25，body U（②登记））。
pub fn mesh_slots_for_kind(kind: u8) -> Option<&'static [(u8, f32)]> {
    KIND_MESH_SLOTS
        .iter()
        .find(|(k, _)| *k == kind)
        .map(|(_, slots)| *slots)
}

/// mesh-type 跳表分派（O：DAT_14304382c）：
/// - 0/1：加权随机挑网格（`init_rng_u64(键哈希)` + `fastrand::f32` + 权重/total 累和选中）；
/// - 2：跳过；3/4：固定候选槽 +0x25/+0x26；
/// - 5/6/8/11/12/13/14/15/18/19：透传（code_r0x0001422d7d8e，交下游）；
/// - 7：计数累积 [cand+0xad]++；9/0xa/0x11/0x14/0x15/0x10/0x18：错误/panic 路径（原为
///   `halt_baddata`/`slice_error_fail`，本重建以 None 跳过并登记）。
///
/// 返回选中槽的网格表下标（加权随机时含权重）。网格表下标 → ClutterMeshTy = U（①登记）。
///
/// 加权选中（O：行 2042-2065）：`do { acc += w/total; idx++ } while (acc <= r)`——**acc>r
/// 严格**时选中（等号继续累加）；count==0 兜底 uVar25=0、耗尽（r>=total）均走
/// `if (count <= idx) panic_bounds_check`（原 B 越界 panic，非静默末槽兜底）。
pub fn select_mesh(mesh_type: u8, kind: u8, key: ClutterRegionId) -> Option<u8> {
    match mesh_type {
        0 | 1 => {
            let slots = mesh_slots_for_kind(kind)?;
            let total: f32 = slots.iter().map(|s| s.1).sum();
            // O：`init_rng_u64(键哈希)` → `fastrand::Rng::f32()`。
            let mut rng = utils::rng::rng_from_seed_u64(utils::rng::init_rng_u64(key.0));
            let r = rng.f32();
            let mut acc = 0.0f32;
            let mut idx = 0usize;
            loop {
                // O：`if (count==0) { uVar25=0; break; }` 兜底首槽（随后越界 panic）。
                if idx >= slots.len() {
                    break;
                }
                acc += slots[idx].1 / total;
                idx += 1;
                // O：`while (acc <= r)` 继续；acc>r 停 → 选中首个 acc>r 的槽。
                if acc > r {
                    break;
                }
            }
            // O：`if (local_160 <= uVar25) panic_bounds_check`（count==0 / r>=total 耗尽）。
            assert!(
                idx < slots.len(),
                "populate_autoclutter_regions: weighted mesh pick bounds check failed"
            );
            Some(slots[idx].0)
        }
        2 => None, // O：跳过（case 2 break）。
        3 => Some(0x25),
        4 => Some(0x26),
        _ => None, // 透传/错误路径 U（本重建不虚构网格选择）。
    }
}

/// 网格组权重表（O：kind 跳表 0x143043818 预设槽组 + 权重；
/// 槽记录 +0x30 == -1（Option None 哨兵）→ 权重 0）。
/// 本重建以 KIND_MESH_SLOTS 静态表承载（见上），资源字段保留为装配层接口（兼容）。
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
    /// 网格组选择（mesh-type/kind；本重建以 KIND_MESH_SLOTS + select_mesh 为准——
    /// 原 F3 近似 select_group 弃用，保留签名防装配层依赖）。
    #[deprecated(note = "wave2 以 KIND_MESH_SLOTS + select_mesh 取代；此近似勿用于判等")]
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

/// GPU 灌木网格：build 里 insert_resource 的唯一 `BushGrid`（read_bush 域）。此前这里有一个
/// 并行占位类型（`Vec<[f32;2]>`），ClutterPlugin 插入的是 read_bush 版，导致本系统的参数
/// 校验永远失败；统一为同一个类型。
pub use super::read_bush_positions_from_gpu::BushGrid;

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

#[cfg(test)]
mod subscribe_tests {
    use super::*;
    use country_core::resources::world_raster::DirtyTilePublisher;
    use country_core::resources::terrain_height::GRID_SIDE;

    fn world() -> World {
        let mut world = World::new();
        world.insert_resource(AutoClutterDirtySubscriptions::default());
        world.insert_resource(RaycastWorld::new());
        world.insert_resource(BushGrid::default());
        world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        world
    }

    fn pending(world: &World) -> Vec<bool> {
        let subs = world.resource::<AutoClutterDirtySubscriptions>();
        let mut guard = subs.subscriber.lock().unwrap();
        guard.iter_mut().map(|subscriber| subscriber.take().is_some()).collect()
    }

    /// 真实 World 上：七个参数资源全部存在 → 参数校验通过，且按机器码顺序订阅 13 个句柄；
    /// 之后向各源发布，只有对应序号的句柄收到（8 = BushGrid、9/10/11 = Path/Water/Garden 栅格、
    /// 12 = TerrainHeightsData）。
    #[test]
    fn subscribes_thirteen_handles_in_machine_order_and_validates_on_a_real_world() {
        let mut world = world();
        let mut schedule = Schedule::default();
        schedule.add_systems(autoclutter_subscribe_to_tabloids);
        schedule.run(&mut world);
        assert_eq!(pending(&world).len(), 13);
        assert!(pending(&world).iter().all(|got| !got));

        let mut tiles = DirtyTiles::new_clear();
        tiles.set_cell(1, 2);
        world.resource_mut::<BushGrid>().publisher.publish(tiles).unwrap();
        let got = pending(&world);
        assert_eq!(got.iter().position(|g| *g), Some(8));
        assert_eq!(got.iter().filter(|g| **g).count(), 1);

        world.resource_mut::<PathRaster>().copy_to_staging(0, &mut []);
        assert_eq!(pending(&world).iter().position(|g| *g), Some(9));
        world.resource_mut::<WaterRaster>().copy_to_staging(0, &mut []);
        assert_eq!(pending(&world).iter().position(|g| *g), Some(10));
        world.resource_mut::<GardenRaster>().copy_to_staging(0, &mut []);
        assert_eq!(pending(&world).iter().position(|g| *g), Some(11));

        let mut gpu = vec![0u16; GRID_SIDE * GRID_SIDE];
        gpu[0] = 7;
        world.resource_mut::<TerrainHeightsData<TerrainHeightSsbo>>().copy_from_gpu(&gpu);
        let got = pending(&world);
        assert_eq!(got.iter().position(|g| *g), Some(12));
        assert_eq!(got.iter().filter(|g| **g).count(), 1);
    }

    /// 缺任一资源 → 系统参数校验失败，不产生任何订阅。
    #[test]
    fn missing_resource_fails_validation_without_subscribing() {
        use bevy_ecs::system::{RunSystemError, RunSystemOnce};
        let mut world = world();
        world.remove_resource::<WaterRaster>();
        let result = world.run_system_once(autoclutter_subscribe_to_tabloids);
        assert!(matches!(result, Err(RunSystemError::InvalidParams { .. })), "{result:?}");
        assert_eq!(pending(&world).len(), 0);
    }
}
