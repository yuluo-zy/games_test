//! fish::fish 系统 + boids 群体规则（B 主基线，反汇编级证据草稿）。
//!
//! 独立证据草稿：完整 bevy 系统（Fish/NaniBundle 出生、WallSpatialHash 避让、
//! Fish 实体转向、光标/相机逃避、BoidFlock::remove）见
//! `evidence/multi-baseline/critters-extra-analysis.md`；本文件只落反汇编钉死的
//! 数值核心（bevy-free，glam + fastrand 即可）。
//!
//! 证据源：reva worker `critter-extra-owner` 反汇编/反编译
//! fish 0x140f90cd0（7533B，源码 92–334）、FishSpawnLocations::new 0x140f909f0、
//! Boid::frame_update<fish closure_env$0> 0x141dc2740（boids.rs 329–387）、
//! Boid::random_vel_change 0x141dc2170、BoidFlock::remove<fish> 0x141dc2340；
//! radare2 RNE 常量 hexdump。
//!
//! boids 参数（fish call site 0x140f9164d 前 disasm）：
//! 邻居半径 4.0、分离半径 0.8（0x3f4ccccd）、对齐权重 0.0584（0x3d6f9db2）、
//! min/max 速度 (0.9,4.0) 或 (3.0,8.0)（`BackupSaveFileKind::is_corrupted()` 谓词选）、凝聚 0.0056、分离 1.7。
//! 出生：上限 200，每帧最多 20；fastrand 出生位置/速度。
use glam::Vec3;
use fastrand::Rng;

/// Boid 记录（0x18）：pos Vec3 + vel Vec3（反汇编证据，与 wren 同构）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Boid {
    pub pos: Vec3,
    pub vel: Vec3,
}

/// 邻居记录（6 f32/条）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Neighbor {
    pub pos: Vec3,
    pub vel: Vec3,
}

/// boids 每帧核心（Boid::frame_update<fish closure_env$0> 0x141dc2740）。
/// 与 wren 变体逐规则一致，仅参数与"水面判定"注入不同：fish 对
/// `bilinear_sample_at_world_xz<RasterBufferRef<512>>` 采样 `>= 0.5` 判水；
/// 非水 → `pos -= vel*dt` 且 `vel = -vel * 0.9`（反弹回水）。
/// 见 `wren_boids::frame_update` 规则注释；此处 `in_water` 由系统注入。
pub fn frame_update(
    boid: &mut Boid,
    neighbors: &[Neighbor],
    self_index: usize,
    sep_radius: f32,
    align_weight: f32,
    min_speed: f32,
    max_speed: f32,
    dt: f32,
    in_water: bool,
) {
    let mut sep = Vec3::ZERO;
    let mut mean_pos = Vec3::ZERO;
    let mut mean_vel = Vec3::ZERO;
    let mut count = 0u32;
    for (i, n) in neighbors.iter().enumerate() {
        if i == self_index {
            continue;
        }
        let d = n.pos - boid.pos;
        let dist2 = d.length_squared();
        if dist2 >= 16.0 || boid.vel.dot(n.vel) < 0.0 {
            continue;
        }
        count += 1;
        mean_pos += n.pos;
        mean_vel += n.vel;
        if dist2 < sep_radius * sep_radius {
            let dist = dist2.sqrt();
            let t = (dist / sep_radius - 1.0).clamp(-1.0, 0.0).abs();
            let smooth = (4.0 - 2.0 * t) * t * t;
            sep -= (d / dist) * smooth;
        }
    }
    if count > 0 {
        let c = count as f32;
        boid.vel = boid.vel * (1.0 - align_weight) + (mean_vel / c) * align_weight;
        boid.vel += (mean_pos / c - boid.pos).normalize_or_zero() * boid.vel.length() * 0.0056;
        boid.vel += sep * 1.7;
    }
    let len = boid.vel.length();
    if len < min_speed {
        boid.vel = boid.vel.normalize_or_zero() * min_speed;
    } else if len > max_speed {
        boid.vel = boid.vel.normalize_or_zero() * max_speed;
    }
    if boid.vel.is_nan() {
        boid.vel = Vec3::ZERO;
    }
    if in_water {
        boid.pos += boid.vel * dt;
    } else {
        // 陆面反弹：反向退并折返速度（反汇编：`pos -= vel*dt; vel = -vel * min_speed`）。
        boid.pos -= boid.vel * dt;
        boid.vel = -boid.vel * min_speed;
    }
}

/// FishSpawnLocations::new（0x140f909f0，源码 63–81）：25×25 网格候选在 [-52,52]²。
/// `0x42500000=52.0`、`0x41c00000=24.0`（分 25 档：`x = (i/24)*52 - (1-i/24)*52`）。
pub const FISH_SPAWN_GRID: usize = 25;
pub const FISH_SPAWN_RANGE: f32 = 52.0;
pub fn fish_spawn_grid_positions() -> Vec<Vec3> {
    let mut out = Vec::with_capacity(FISH_SPAWN_GRID * FISH_SPAWN_GRID);
    for iy in 0..FISH_SPAWN_GRID {
        let fy = (iy as f32 / (FISH_SPAWN_GRID - 1) as f32) * 2.0 - 1.0;
        for ix in 0..FISH_SPAWN_GRID {
            let fx = (ix as f32 / (FISH_SPAWN_GRID - 1) as f32) * 2.0 - 1.0;
            out.push(Vec3::new(fx * FISH_SPAWN_RANGE, 0.0, fy * FISH_SPAWN_RANGE));
        }
    }
    out
}

/// 出生（fish L166–330 核心）：上限 200、每帧最多 20；位置取自网格候选，须水面。
/// 初始速度（反汇编）：`v.x = fastrand::f32 + fVar42(-0.5)`、`v.y = fastrand::f32 + fVar51(-1.0)`、
/// `v.z = fastrand::f32 - 1.0`。`in_water` 由系统采样注入。
pub const MAX_FISH: usize = 200;
pub const FISH_SPAWN_PER_FRAME: usize = 20;
pub fn spawn_boid(pos: Vec3, in_water: bool, rng: &mut Rng) -> Option<Boid> {
    if !in_water {
        return None;
    }
    Some(Boid {
        pos,
        vel: Vec3::new(
            rng.f32() - 0.5,
            rng.f32() - 1.0,
            rng.f32() - 1.0,
        ),
    })
}

/// 壁面投影/推开核心（fish L619–698）：4 采样水面求 clamp[0,1] `< 0.1`（0x3dcccccd）时
/// `WallSpatialHash::query` 逐墙段；点到墙段垂距 `< fVar42`（0x3f8f5c29≈1.12）→ 沿法向推开。
/// `wall_hit` 由系统注入（LineInterval::relate 结果）。返回推开方向。
pub fn avoid_walls(
    _pos: Vec3,
    water_sum: f32,
    wall_dir: Option<Vec3>,
    wall_dist: f32,
) -> Option<Vec3> {
    if water_sum >= 0.1 {
        return None;
    }
    let d = wall_dir?;
    if wall_dist < 1.12 {
        Some(d.normalize_or_zero())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_has_625_points() {
        let g = fish_spawn_grid_positions();
        assert_eq!(g.len(), 625);
        assert!(g[0].x.abs() < 52.0 + 1e-3);
        assert!(g.iter().all(|p| p.x.abs() <= 52.0 + 1e-3));
    }

    #[test]
    fn spawn_requires_water() {
        let mut rng = Rng::with_seed(2);
        assert!(spawn_boid(Vec3::ZERO, false, &mut rng).is_none());
        let b = spawn_boid(Vec3::ZERO, true, &mut rng).unwrap();
        assert_eq!(b.pos, Vec3::ZERO);
    }

    #[test]
    fn avoid_wall_returns_dir() {
        let dir = avoid_walls(Vec3::ZERO, 0.0, Some(Vec3::new(1.0, 0.0, 0.0)), 0.5);
        assert_eq!(dir, Some(Vec3::new(1.0, 0.0, 0.0)));
        assert_eq!(avoid_walls(Vec3::ZERO, 0.5, None, 0.0), None);
    }

    #[test]
    fn boid_bounces_out_of_water() {
        let mut b = Boid { pos: Vec3::ZERO, vel: Vec3::new(1.0, 0.0, 0.0) };
        frame_update(&mut b, &[], 0, 0.8, 0.0584, 0.9, 4.0, 1.0, false);
        assert!(b.vel.x < 0.0);
        assert!(b.pos.x < 0.0);
    }
}
