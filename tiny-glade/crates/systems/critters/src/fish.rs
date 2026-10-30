//! fish 系统（B 主基线，反汇编级证据）。
//!
//! 证据（critters-extra-analysis.md §2 + fresh decomp 140f90cd0.c）：
//! - fish 0x140f90cd0（7533B，源码 92–334）：出生上限 200、每帧最多 20（0x14）；
//!   候选点 hash 采样 FishSpawnLocations 网格；`bilinear_sample_at_world_xz` `> 0.5` 才生；
//!   出生 = spawn(Fish, NaniBundle) + 写 Boid{pos,vel}。初始速度
//!   `vel = (rng.f32()-0.5, rng.f32()-0.5, rng.f32()-0.5)`（0x140f90cd0 出生写点）；
//!   出生位 x/z 微抖动 `(rng.f32()-0.5)*0.5`，y 为 `-0.8-0.25`。
//! - 每帧 Boid::frame_update<fish closure_env$0>（boids 参数：sep_r 0.8、align 0.0585、
//!   (min,max) = is_corrupted 选 (0.9,4.0) 或 (3.0,8.0)）；viable = 水面 `>= 0.5`
//!   （frame_update 内做 random_vel_change + 移动/反弹，调用方无需再调 apply_fish_motion）。
//! - 逐鱼：水面 `<= 0.5` → 进移除列表（BoidFlock::remove 清理）；`> 0.5` → 完整 boids 更新。
//! - 墙推开：4×`bilinear_sample_at_world_xz`（PathRastersResources 四栅格，同坐标）求和 clamp[0,1]
//!   `< 0.1` → WallSpatialHash 逐墙段，垂距 `< 1.12` → `time_continuous_lerp_vec3` 推开。
//! - 实体转向：byte==2 的 Fish，2D 距离 < 2.0 → 速度朝目标向 lerp（rate 5.0 档）。
//!
//! `FishBoids` 资源保留原版两段并行 Vec：0x18 Boid 与 0x38 aux。
//! aux+0x10 保存 Fish 实体 ID，群内索引驱动更新与倒序 swap_remove。
//! 墙推开门已恢复为四路径栅格采样；墙段投影和推开算法仍需原侧差分。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use country_core::systems::paths::PathRastersResources;
use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
use fastrand::Rng;
use glam::Vec2;
use glam::Vec3;

use crate::boids::{frame_update, time_continuous_lerp_vec3, Boid, BoidState, Neighbor};
use crate::components::{Fish, FishBoidAux, FishBoids, FishSpawnLocations};

/// fish（0x140f90cd0）。
#[allow(clippy::too_many_arguments)]
pub fn fish(
    time: Res<GameTime>,
    mut spawn_locations: ResMut<FishSpawnLocations>,
    mut flock: ResMut<FishBoids>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    path_rasters: PathRastersResources,
    walls: Res<WallSpatialHash>,
    rng: Res<FishRng>,
    mut commands: Commands,
) {
    let dt = time.delta;
    let w = (*water).as_ref();
    let path_grids = path_rasters.as_float_grid();
    let r = &mut rng.0.lock().unwrap();
    let (min_speed, max_speed) = min_max_speed();
    // 出生谓词：`BackupSaveFileKind::is_corrupted()` 选速度档；本重建缺省低档
    // （B 发行 pred 值 U，见 min_max_speed）。

    // 出生（L166–330）：数量 < 200 时逐帧补，每帧最多 20。
    let live = flock.boids.len();
    if live < MAX_FISH {
        let mut spawned = 0;
        while spawned < FISH_SPAWN_PER_FRAME && live + spawned < MAX_FISH {
            let candidate_index = pick_spawn_candidate(spawn_locations.candidates.len(), r);
            let candidate = spawn_locations.candidates[candidate_index];
            let in_water =
                w.bilinear_sample_at_world_xz(candidate.x, candidate.z, 130.0, 130.0) > 0.5;
            if !in_water {
                spawn_locations.used[candidate_index] = false;
                spawned += 1;
                continue;
            }
            if spawn_locations.used[candidate_index] {
                spawned += 1;
                continue;
            }
            spawn_locations.used[candidate_index] = true;
            // 出生位：候选 x/z 各取一次 f32 抖动，y 注入 -0.8 后加 -0.25。
            // 候选的 used 字节在出生前置 1；已用候选直接跳过。
            // 原指令 0x140f91054–0x140f910f0 只有两次 RNG：x/z；
            // x0y@0x140c9cde0 注入 -0.8，SIMD 第二 lane 再加 -0.25。
            let jx = (r.f32() - 0.5) * 0.5;
            let jy = f32::from_bits(0xbf4c_cccd) - 0.25;
            let jz = (r.f32() - 0.5) * 0.5;
            let boid = Boid {
                pos: Vec3::new(candidate.x + jx, candidate.y + jy, candidate.z + jz),
                vel: Vec3::new(r.f32() - 0.5, r.f32() - 0.5, r.f32() - 0.5),
            };
            // 0x140f911a8: Fish 的 0x30 B 初始记录在 +0x20 写零状态。
            let entity = commands.spawn(Fish { state: 0 }).id();
            flock.boids.push(boid);
            flock.states.push(FishBoidAux { state: BoidState::default(), entity });
            spawned += 1;
        }
    }

    // 每帧 Boid 更新；邻居表依资源顺序，包含本帧新增的 Boid。
    let all: Vec<Neighbor> = flock.boids
        .iter()
        .map(|b| Neighbor {
            pos: b.pos,
            vel: b.vel,
        })
        .collect();
    let mut despawn: Vec<usize> = Vec::new();
    // 原循环界是 min(boids.len, states.len)，即使资源暂时不同步也不越界。
    for self_idx in 0..flock.boids.len().min(flock.states.len()) {
        let mut boid = flock.boids[self_idx];
        // 水面 `<= 0.5` → 移除（O L567：`if (0.5 < sample) break` 处理，否则进移除列表）。
        let in_water =
            w.bilinear_sample_at_world_xz(boid.pos.x, boid.pos.z, 130.0, 130.0) > 0.5;
        if !in_water {
            despawn.push(self_idx);
            continue;
        }

        let mut state = flock.states[self_idx].state;
        // viable = 水面（frame_update 内做随机转向 + 移动/反弹；`accel_max` 由
        // frame_update 依 viable 前瞻与 goal 判定，鱼实例无 goal → 1.0）。
        let viable = |p: Vec3| w.bilinear_sample_at_world_xz(p.x, p.z, 130.0, 130.0) >= 0.5;
        frame_update(
            &mut boid,
            &mut state,
            &all,
            self_idx,
            0.8,
            f32::from_bits(0x3d6f_9db2),
            min_speed,
            max_speed,
            dt,
            r,
            &viable,
        );
        flock.states[self_idx].state = state;

        // 四张路径栅格在同一世界坐标采样，顺序为 path / cobblestone1 / 2 / 3。
        let mut path_sum = 0.0f32;
        for grid in &path_grids {
            path_sum += grid.bilinear_sample_at_world_xz(boid.pos.x, boid.pos.z, 130.0, 130.0);
        }
        if path_sum.clamp(0.0, 1.0) < 0.1 {
            repel_from_walls(&walls, &mut boid, dt);
        }
        flock.boids[self_idx] = boid;

    }
    // BoidFlock::remove<fish> (原 C): 倒序取移除索引，两个 Vec 同步 swap_remove。
    for index in despawn.into_iter().rev() {
        let entity = flock.states.swap_remove(index).entity;
        flock.boids.swap_remove(index);
        if let Ok(mut survivor) = commands.get_entity(entity) {
            survivor.despawn();
        }
    }
}

/// 原版鱼实体状态 2 的近距离避让（fish.decompiled.c L771–812）。
/// 原查询按实体次序依次处理所有状态 2、水平距离 <2 的目标；不排序、不选最近者。
/// `targets` 保留原实体迭代次序。生产侧 Fish 的 0x30 记录位置写面尚未恢复，
/// 故当前只提供可复用的独立算法，不用伪造 Boid 邻居作为实体查询。
pub fn repel_from_fish_targets(boid: &mut Boid, targets: &[(u8, Vec3)], dt: f32) {
    for &(state, position) in targets {
        if state != 2 {
            continue;
        }
        let delta = Vec2::new(boid.pos.x - position.x, boid.pos.z - position.z);
        let distance = (delta.y * delta.y + delta.x * delta.x).sqrt();
        if distance < 2.0 {
            let inv_distance = 1.0 / distance;
            let target = Vec3::new(delta.x * inv_distance * 5.0, 0.0, delta.y * inv_distance * 5.0);
            boid.vel = time_continuous_lerp_vec3(boid.vel, target, 5.0, dt);
        }
    }
}

/// 出生候选（O：fastrand hash 采样 FishSpawnLocations 网格）。
/// 原侧 `fastrand::Rng::usize(..len)` 使用乘法高位与拒绝重抽，不能用整域抽样后取余。
pub fn pick_spawn_candidate(candidate_count: usize, rng: &mut Rng) -> usize {
    rng.usize(..candidate_count)
}

/// 速度档（`BackupSaveFileKind::is_corrupted()` 谓词选 (0.9,4.0) 或 (3.0,8.0)；
/// 本重建由宿主注入档位，缺省低档）。
pub fn min_max_speed() -> (f32, f32) {
    (0.9, 4.0)
}

/// 出生上限 / 每帧上限（O：200 / 0x14=20）。
pub const MAX_FISH: usize = 200;
pub const FISH_SPAWN_PER_FRAME: usize = 20;

/// 墙避让（B fish.decompiled.c L641–693；原指令 0x140f91912–0x140f9192f）。
/// 触发：调用方四路径栅格同点求和 clamp[0,1] < 0.1。
pub fn repel_from_walls(walls: &WallSpatialHash, boid: &mut Boid, dt: f32) {
    let pos = boid.pos;
    let min = [pos.x - 4.0, pos.z - 4.0];
    let max = [pos.x + 4.0, pos.z + 4.0];
    for (_endpoint, wall_id) in walls.query_with_aabb_clamped(min, max) {
        for seg in walls.segments_for_wall(wall_id) {
            // fish.decompiled.c L641–646: 仅两端 +0x20/+0x24 高度都严格低于 0.2 的墙段。
            if !(seg.terrain_heights[0] < 0.2 && seg.terrain_heights[1] < 0.2) {
                continue;
            }
            let a = Vec2::new(seg.endpoints[0][0], seg.endpoints[0][1]);
            let b = Vec2::new(seg.endpoints[1][0], seg.endpoints[1][1]);
            let p = Vec2::new(pos.x, pos.z);
            let ab = b - a;
            let len2 = ab.length_squared();
            let t = ((p - a).dot(ab) / len2.max(1e-20)).clamp(0.0, 1.0);
            let closest = a + ab * t;
            let delta = p - closest;
            let dist = (delta.y * delta.y + delta.x * delta.x).sqrt();
            if dist < f32::from_bits(0x3f8f_5c29) {
                let inv_distance = 1.0 / dist;
                let speed = ((boid.vel.z * boid.vel.z + boid.vel.x * boid.vel.x)
                    + boid.vel.y * boid.vel.y).sqrt();
                let target = Vec3::new(delta.x * inv_distance * speed, 0.0, delta.y * inv_distance * speed);
                boid.vel = time_continuous_lerp_vec3(boid.vel, target, 10.0, dt);
                return;
            }
        }
    }
}

/// despawn_fish（0x140f90c10）——**资源重置**，非实体清理（fresh 反编译 2026-10-17）：
/// - `ResMut<FishBoids>`：先 drop 内部两段 Vec 存储（`__rust_dealloc` ×2），随后把
///   资源布局重写为空（`ptr=0,cap=4/8,len=0` 段——RawVec 空表示），即清空 boid 群；
/// - `FishSpawnLocations::new()`：把 `ResMut<FishSpawnLocations>` 重写为新构造的
///   25×25 网格（drop 旧存储 + 就地覆盖）。
/// B 侧注册于 `OnExit<AppLoadingState>`（lib.rs ⑦）。crate 无 bevy_state 生命周期，
/// 由宿主在退出加载态时调用；B 侧不清理实体，此处同样不 despawn。
pub fn despawn_fish(mut boids: ResMut<FishBoids>, mut locations: ResMut<FishSpawnLocations>) {
    // `*plVar1 = 0; [1]=4; [2]=0; [3]=0; [4]=8; [5]=0; [6]=0`：两段存储置空。
    *boids = FishBoids::default();
    // `FishSpawnLocations::new()` 就地覆盖资源。
    *locations = FishSpawnLocations::new();
}

/// 鱼 RNG（fastrand；出生位置/速度与随机转向，O）。
#[derive(Resource, Debug)]
pub struct FishRng(pub std::sync::Mutex<Rng>);

impl Default for FishRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(Rng::with_seed(0x606f)))
    }
}
