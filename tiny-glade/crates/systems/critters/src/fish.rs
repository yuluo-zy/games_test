//! fish 系统（B 主基线，反汇编级证据）。
//!
//! 证据（critters-extra-analysis.md §2 + reconstruction/critters/src/fish.rs）：
//! - fish 0x140f90cd0（7533B，源码 92–334）：出生上限 200、每帧最多 20（0x14）；
//!   候选点 hash 采样 FishSpawnLocations 网格；`bilinear_sample_at_world_xz` `> 0.5` 才生；
//!   出生 = spawn(Fish, …) + 写 Boid{pos,vel}；初始速度
//!   `v.x = fastrand::f32 + (-0.5)`、`v.y = fastrand::f32 + (-1.0)`、`v.z = fastrand::f32 - 1.0`。
//!   每帧 Boid::frame_update<fish closure>（boids 参数：sep_r 0.8、align 0.0584、
//!   (min,max) = is_corrupted 选 (0.9,4.0) 或 (3.0,8.0)）；4×水面采样求和 clamp[0,1] < 0.1 →
//!   WallSpatialHash 逐墙段，投影距离 < 1.12 → time_continuous_lerp_vec3 推开。
//!   实体转向：byte==2 的 Fish，2D 距离 < 2.0 → 速度朝目标 lerp（5.0/8.0 两档）。
//! - FishSpawnLocations::new 0x140f909f0：25×25=625 点 ∈[-52,52]²。
//! - despawn_fish 0x140f90c10。
//!
//! `FishBoids` 组件 = Boid 0x18 槽（pos+vel）的实体侧投影；flock 语义（数量上限、
//! 邻居迭代、swap_remove 清理）直接以查询实体集合表达。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
use fastrand::Rng;
use glam::Vec2;
use glam::Vec3;

use crate::boids::{frame_update, random_vel_change, Neighbor};
use crate::components::{Fish, FishBoids, FishSpawnLocations};

/// fish（0x140f90cd0）。
#[allow(clippy::too_many_arguments)]
pub fn fish(
    time: Res<GameTime>,
    spawn_locations: Res<FishSpawnLocations>,
    water: Res<WorldRaster<512>>,
    walls: Res<WallSpatialHash>,
    mut rng: ResMut<FishRng>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Fish, &mut FishBoids)>,
) {
    let dt = time.delta;
    let w = (*water).as_ref();
    let r = &mut rng.0.lock().unwrap();
    let (min_speed, max_speed) = min_max_speed();

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
            let boid = FishBoids {
                pos: candidate,
                vel: Vec3::new(r.f32() - 0.5, r.f32() - 1.0, r.f32() - 1.0),
                turn_timer: 0.0,
            };
            commands.spawn((Fish { state: 2 }, boid));
            spawned += 1;
        }
    }

    // 每帧 Boid 更新（L417–588）。邻居 = 全部鱼实体（heading 过滤在 frame_update 内）。
    let all: Vec<Neighbor> = query
        .iter()
        .map(|(_, _, b)| Neighbor {
            pos: b.pos,
            vel: b.vel,
        })
        .collect();
    let mut despawn: Vec<Entity> = Vec::new();
    for (entity, mut fish_c, mut fish_b) in &mut query {
        let self_idx = all
            .iter()
            .position(|n| n.pos.distance_squared(fish_b.pos) < 1e-4)
            .unwrap_or(usize::MAX);
        let mut boid = crate::boids::Boid {
            pos: fish_b.pos,
            vel: fish_b.vel,
        };
        // 转向目标（O L786–812：byte==2 的 Fish 向目标 lerp；目标 = 最近他鱼（2D），
        // 两档常量 5.0/8.0）。
        let steer_target = nearest_other(&all, self_idx);
        let accel_max = if in_water_checked(&w, boid.pos) && steer_target.is_some() {
            1.5
        } else {
            1.0
        };
        frame_update(
            &mut boid,
            &all,
            self_idx,
            0.8,
            0.0584,
            min_speed,
            max_speed,
            dt,
            accel_max,
        );
        // 水面判定（>= 0.5 判水；非水反弹）。
        let in_water =
            w.bilinear_sample_at_world_xz(boid.pos.x, boid.pos.z, 130.0, 130.0) >= 0.5;
        crate::boids::apply_fish_motion(&mut boid, dt, in_water, min_speed);

        // 壁面推开（4 采样求和 clamp[0,1] < 0.1 时逐墙段，垂距 < 1.12 → 沿法向推开）。
        let mut water_sum = 0.0f32;
        for (dx, dz) in FISH_WATER_SAMPLES {
            water_sum +=
                w.bilinear_sample_at_world_xz(boid.pos.x + dx, boid.pos.z + dz, 130.0, 130.0);
        }
        if water_sum.clamp(0.0, 1.0) < 0.1 {
            if let Some(push) = push_away_from_walls(&walls, boid.pos) {
                boid.pos = boid.pos.lerp(push, 5.0 * dt);
            }
        }

        // 实体转向（L786–812）：byte==2，2D 距离 < 2.0 → 速度朝目标 lerp
        // （5.0/8.0 两档常数；目标 = 最近他鱼，非自身）。
        if fish_c.state == 2 {
            if let Some(target) = steer_target {
                let d2d = Vec2::new(target.pos.x - boid.pos.x, target.pos.z - boid.pos.z);
                if d2d.length() < 2.0 {
                    boid.vel = boid.vel.lerp(target.vel, 5.0 * dt);
                }
            }
        }
        // 随机转向（O：Boid::random_vel_change；计时随 boid 记录）。
        fish_b.turn_timer -= dt;
        if fish_b.turn_timer <= 0.0 {
            if let Some(dir) = random_vel_change(boid.vel, &mut fish_b.turn_timer, dt, r) {
                boid.vel = dir;
            }
        }

        fish_b.pos = boid.pos;
        fish_b.vel = boid.vel;

        // 越界清理（半径 > 130 或离水）。
        if !in_water || boid.pos.length() > 130.0 {
            despawn.push(entity);
        }
    }
    for entity in despawn {
        commands.entity(entity).despawn();
    }
}

/// 最近他鱼（2D；排除自身；O：转向目标选择）。
fn nearest_other(all: &[Neighbor], self_idx: usize) -> Option<Neighbor> {
    let mut best: Option<(f32, Neighbor)> = None;
    for (i, n) in all.iter().enumerate() {
        if i == self_idx {
            continue;
        }
        let d = (n.pos - self_pos_of(all, self_idx)).length_squared();
        if best.map_or(true, |(bd, _)| d < bd) {
            best = Some((d, *n));
        }
    }
    best.map(|(_, n)| n)
}

fn self_pos_of(all: &[Neighbor], self_idx: usize) -> Vec3 {
    all.get(self_idx).map_or(Vec3::ZERO, |n| n.pos)
}

/// 水面判定（>= 0.5 判水；O）。
fn in_water_checked(w: &country_core::resources::world_raster::RasterBufferRef<'_, 512>, pos: Vec3) -> bool {
    w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0) >= 0.5
}

/// 出生候选（O：hash 采样 FishSpawnLocations 网格）。
pub fn pick_spawn_candidate(candidates: &[Vec3], rng: &mut Rng) -> Vec3 {
    let idx = (rng.u64(..) as usize) % candidates.len();
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

/// 4 水面采样偏移（求和 clamp[0,1]）。
pub const FISH_WATER_SAMPLES: [(f32, f32); 4] = [(0.0, 0.0), (0.8, 0.0), (0.0, 0.8), (0.8, 0.8)];

/// 壁面推开（O：WallSpatialHash 邻域段，点到段垂距 < 0x3f8f5c29≈1.12 → 沿法向推开）。
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

/// despawn_fish（0x140f90c10）：清理（离开水面或超界）。
pub fn despawn_fish(
    water: Res<WorldRaster<512>>,
    mut commands: Commands,
    query: Query<(Entity, &FishBoids)>,
) {
    let w = (*water).as_ref();
    for (entity, fish_b) in &query {
        let in_water =
            w.bilinear_sample_at_world_xz(fish_b.pos.x, fish_b.pos.z, 130.0, 130.0) > 0.5;
        let far = fish_b.pos.length() > 130.0;
        if !in_water || far {
            commands.entity(entity).despawn();
        }
    }
}

/// 鱼 RNG（fastrand；出生位置/速度与随机转向，O）。
#[derive(Resource, Debug)]
pub struct FishRng(pub std::sync::Mutex<Rng>);

impl Default for FishRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(Rng::with_seed(0x606f)))
    }
}
