//! fish 系统（B 主基线，反汇编级证据）。
//!
//! 证据（critters-extra-analysis.md §2 + fresh decomp 140f90cd0.c）：
//! - fish 0x140f90cd0（7533B，源码 92–334）：出生上限 200、每帧最多 20（0x14）；
//!   候选点 hash 采样 FishSpawnLocations 网格；`bilinear_sample_at_world_xz` `> 0.5` 才生；
//!   出生 = spawn(Fish, NaniBundle) + 写 Boid{pos,vel}。初始速度
//!   `vel = (rng.f32()-0.5, rng.f32()-0.5, rng.f32()-0.5)`（0x140f90cd0 三档全 -0.5）；
//!   出生位 = 候选位 + 微抖动 `(rng.f32()-0.5)*0.5`（x/z），y 额外 `-0.25`。
//! - 每帧 Boid::frame_update<fish closure_env$0>（boids 参数：sep_r 0.8、align 0.0584、
//!   (min,max) = is_corrupted 选 (0.9,4.0) 或 (3.0,8.0)）；viable = 水面 `>= 0.5`
//!   （frame_update 内做 random_vel_change + 移动/反弹，调用方无需再调 apply_fish_motion）。
//! - 逐鱼：水面 `<= 0.5` → 进移除列表（BoidFlock::remove 清理）；`> 0.5` → 完整 boids 更新。
//! - 墙推开：4×`bilinear_sample_at_world_xz`（PathRastersResources 四栅格）求和 clamp[0,1]
//!   `< 0.1` → WallSpatialHash 逐墙段，垂距 `< 1.12` → `time_continuous_lerp_vec3` 推开。
//! - 实体转向：byte==2 的 Fish，2D 距离 < 2.0 → 速度朝目标向 lerp（rate 5.0 档）。
//!
//! `FishBoids` 组件 = Boid 0x18 槽（pos+vel）的实体侧投影；真实 BoidFlock 资源（0x18 +
//! 0x38 aux 槽）未在 crate 表达（DIVERGE，见 lib.rs ⑦）；`turn_timer` = 0x38 槽
//! timer@+0x30 的投影。`dir_smoothed`/`dir` 无持久位（近似）。
//! 墙推开使用单水栅格近似（真实为 4 路径栅格求和，crate 未接线 PathRastersResources）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
use fastrand::Rng;
use glam::Vec2;
use glam::Vec3;

use crate::boids::{frame_update, Boid, BoidState, Neighbor};
use crate::components::{Fish, FishBoids, FishSpawnLocations};

/// fish（0x140f90cd0）。
#[allow(clippy::too_many_arguments)]
pub fn fish(
    time: Res<GameTime>,
    spawn_locations: Res<FishSpawnLocations>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    walls: Res<WallSpatialHash>,
    mut rng: ResMut<FishRng>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Fish, &mut FishBoids)>,
) {
    let dt = time.delta;
    let w = (*water).as_ref();
    let r = &mut rng.0.lock().unwrap();
    let (min_speed, max_speed) = min_max_speed();
    // 出生谓词：`BackupSaveFileKind::is_corrupted()` 选速度档；本重建缺省低档
    // （B 发行 pred 值 U，见 min_max_speed）。

    // 出生（L166–330）：数量 < 200 时逐帧补，每帧最多 20。
    let live = query.iter().count();
    if live < MAX_FISH {
        let mut spawned = 0;
        while spawned < FISH_SPAWN_PER_FRAME && live + spawned < MAX_FISH {
            let candidate = pick_spawn_candidate(&spawn_locations.candidates, r);
            let in_water =
                w.bilinear_sample_at_world_xz(candidate.x, candidate.z, 130.0, 130.0) > 0.5;
            if !in_water {
                spawned += 1;
                continue;
            }
            // 出生位：候选位 + 微抖动（O：`(f32-0.5)*0.5`；z 另 `-0.25`）。
            // 真实候选含 used 字节（spawn 后置 1，跳过已用；crate FishSpawnLocations 无 flag，
            // DIVERGE——重复采样候选，见 pick_spawn_candidate 注）。
            // FIX⑤：出生微抖动 = (x: (u-0.5)*0.5, y: u-0.25, z: (u-0.5)*0.5)——二进制
            // jy=u-0.25 无 0.5 缩放、jz=(u-0.5)*0.5 无 -0.25（旧 Rust 把 -0.25 错位到 z）。
            let jx = (r.f32() - 0.5) * 0.5;
            let jy = r.f32() - 0.25;
            let jz = (r.f32() - 0.5) * 0.5;
            let boid = FishBoids {
                pos: Vec3::new(candidate.x + jx, candidate.y + jy, candidate.z + jz),
                vel: Vec3::new(r.f32() - 0.5, r.f32() - 0.5, r.f32() - 0.5),
                turn_timer: 0.0,
            };
            commands.spawn((Fish { state: 2 }, boid));
            spawned += 1;
        }
    }

    // 每帧 Boid 更新（L417–588）。邻居 = 全部鱼实体。
    let all: Vec<Neighbor> = query
        .iter()
        .map(|(_, _, b)| Neighbor {
            pos: b.pos,
            vel: b.vel,
        })
        .collect();
    let mut despawn: Vec<Entity> = Vec::new();
    for (entity, mut fish_c, mut fish_b) in &mut query {
        // 水面 `<= 0.5` → 移除（O L567：`if (0.5 < sample) break` 处理，否则进移除列表）。
        let in_water =
            w.bilinear_sample_at_world_xz(fish_b.pos.x, fish_b.pos.z, 130.0, 130.0) > 0.5;
        if !in_water {
            despawn.push(entity);
            continue;
        }

        let self_idx = all
            .iter()
            .position(|n| n.pos.distance_squared(fish_b.pos) < 1e-4)
            .unwrap_or(usize::MAX);
        let mut boid = Boid {
            pos: fish_b.pos,
            vel: fish_b.vel,
        };
        // 0x38 aux 槽：timer 持久化经组件 turn_timer；dir_smoothed/dir 无持久位（近似）。
        let mut state = BoidState {
            goal: None,
            dir_smoothed: Vec3::ZERO,
            dir: Vec3::ZERO,
            timer: fish_b.turn_timer,
        };
        // viable = 水面（frame_update 内做随机转向 + 移动/反弹；`accel_max` 由
        // frame_update 依 viable 前瞻与 goal 判定，鱼实例无 goal → 1.0）。
        let viable = |p: Vec3| w.bilinear_sample_at_world_xz(p.x, p.z, 130.0, 130.0) >= 0.5;
        frame_update(
            &mut boid,
            &mut state,
            &all,
            self_idx,
            0.8,
            0.0584,
            min_speed,
            max_speed,
            dt,
            r,
            &viable,
        );
        fish_b.turn_timer = state.timer;

        // 实体转向（L786–812）：byte==2，2D 距离 < 2.0 → 速度朝目标向 lerp（rate 5.0）。
        // 目标 = 最近他鱼（2D；非自身）。近似：朝目标速度 lerp（真实朝向目标方向向量，
        // DIVERGE，见 nearest_other 注）。
        if fish_c.state == 2 {
            if let Some(target) = nearest_other(&all, self_idx) {
                let d2d = Vec2::new(target.pos.x - boid.pos.x, target.pos.z - boid.pos.z);
                if d2d.length() < 2.0 {
                    let t = 1.0 - (-5.0 * dt).exp();
                    boid.vel = boid.vel * (1.0 - t) + target.vel * t;
                }
            }
        }

        fish_b.pos = boid.pos;
        fish_b.vel = boid.vel;

        // 墙推开（O：4 栅格和 clamp[0,1] < 0.1 → WallSpatialHash 逐墙段，垂距 < 1.12 →
        // 沿法向推开。crate 以单水栅格 4 偏移采样近似——DIVERGE，见模块注）。
        let mut water_sum = 0.0f32;
        for (dx, dz) in FISH_WATER_SAMPLES {
            water_sum += w.bilinear_sample_at_world_xz(boid.pos.x + dx, boid.pos.z + dz, 130.0, 130.0);
        }
        if water_sum.clamp(0.0, 1.0) < 0.1 {
            if let Some(push) = push_away_from_walls(&walls, boid.pos) {
                let t = 1.0 - (-5.0 * dt).exp();
                boid.pos = boid.pos * (1.0 - t) + push * t;
            }
        }
        fish_b.pos = boid.pos;
        fish_b.vel = boid.vel;

        // 越界清理（真实：水面判定驱动移除；半径 130 为本 crate 既有收尾）。
        if boid.pos.length() > 130.0 {
            despawn.push(entity);
        }
    }
    for entity in despawn {
        commands.entity(entity).despawn();
    }
}

/// 最近他鱼（2D；排除自身；O：转向目标选择）。
fn nearest_other(all: &[Neighbor], self_idx: usize) -> Option<Neighbor> {
    let self_pos = all.get(self_idx).map_or(Vec3::ZERO, |n| n.pos);
    let mut best: Option<(f32, Neighbor)> = None;
    for (i, n) in all.iter().enumerate() {
        if i == self_idx {
            continue;
        }
        let d = (n.pos - self_pos).length_squared();
        if best.map_or(true, |(bd, _)| d < bd) {
            best = Some((d, *n));
        }
    }
    best.map(|(_, n)| n)
}

/// 出生候选（O：fastrand hash 采样 FishSpawnLocations 网格）。
/// 真实候选点 = Vec<(Vec2, u8)>（含 used 字节，spawn 后置 1，已用跳过）；
/// crate FishSpawnLocations 为 Vec<Vec3>（无 flag，DIVERGE——可能重复采样已用位）。
pub fn pick_spawn_candidate(candidates: &[Vec3], rng: &mut Rng) -> Vec3 {
    let idx = (rng.usize(..)) % candidates.len();
    candidates[idx]
}

/// 速度档（`BackupSaveFileKind::is_corrupted()` 谓词选 (0.9,4.0) 或 (3.0,8.0)；
/// 本重建由宿主注入档位，缺省低档）。
pub fn min_max_speed() -> (f32, f32) {
    (0.9, 4.0)
}

/// 出生上限 / 每帧上限（O：200 / 0x14=20）。
pub const MAX_FISH: usize = 200;
pub const FISH_SPAWN_PER_FRAME: usize = 20;

/// 4 水面采样偏移（墙推开触发近似：单水栅格求和 clamp[0,1]；真实为 4 路径栅格求和）。
pub const FISH_WATER_SAMPLES: [(f32, f32); 4] = [(0.0, 0.0), (0.8, 0.0), (0.0, 0.8), (0.8, 0.8)];

/// 墙推开（O：WallSpatialHash 邻域段，点到段垂距 < 0x3f8f5c29≈1.12 → 沿法向推开）。
/// 触发：4 路径栅格求和 clamp[0,1] < 0.1（crate 以单水栅格 4 采样近似，DIVERGE）。
pub fn push_away_from_walls(walls: &WallSpatialHash, pos: Vec3) -> Option<Vec3> {
    let min = [pos.x - 4.0, pos.z - 4.0];
    let max = [pos.x + 4.0, pos.z + 4.0];
    let mut best: Option<(f32, Vec2)> = None;
    for (_endpoint, wall_id) in walls.query_with_aabb_clamped(min, max) {
        for seg in walls.segments_for_wall(wall_id) {
            let a = Vec2::new(seg.endpoints[0][0], seg.endpoints[0][1]);
            let b = Vec2::new(seg.endpoints[1][0], seg.endpoints[1][1]);
            let p = Vec2::new(pos.x, pos.z);
            let ab = b - a;
            let len2 = ab.length_squared();
            if len2 < 1e-9 {
                continue;
            }
            let t = ((p - a).dot(ab) / len2).clamp(0.0, 1.0);
            let closest = a + ab * t;
            let d = p - closest;
            let dist = d.length();
            let normal = d.normalize_or_zero();
            if dist < 1.12 {
                if best.map_or(true, |(bd, _)| dist < bd) {
                    best = Some((dist, normal));
                }
            }
        }
    }
    best.map(|(_, n)| pos + Vec3::new(n.x, 0.0, n.y) * 1.12)
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
