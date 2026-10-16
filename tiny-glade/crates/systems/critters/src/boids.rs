//! boids 群体规则（B 主基线，反汇编级证据）。
//!
//! 证据（critters-extra-analysis.md §1 + reconstruction/critters/{fish,wren_boids}.rs）：
//! - `Boid::frame_update<fish>` 0x141dc2740 / `<wren>` 0x141dc3510（boids.rs 329–387）；
//! - `Boid::random_vel_change` 0x141dc2170（389–403）；
//! - `BoidFlock::remove` 0x141dc2340 / 0x141dc2540（114–124）。
//! call-site 权重常量（disasm 证实）：邻居半径 4.0（dist²<16）、分离半径 鱼 0.8 / 鹪鹩 1.3、
//! 对齐权重 鱼 0.0584 / 鹪鹩 0.1169、凝聚 0.0056、分离 1.7；
//! 速度约束 [min,max]：鱼 (0.9,4.0)|(3.0,8.0)（`BackupSaveFileKind::is_corrupted()` 谓词选）、
//! 鹪鹩 (7.0,14.0)。heading 过滤：`self.vel · nvel >= 0` 才计入。
//! NaN → vel 归零；`pos += vel * dt`。水面/可行点判定由调用方注入。

use fastrand::Rng;
use glam::Vec3;

/// Boid 记录（0x18）：pos Vec3@+0 + vel Vec3@+0xc。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Boid {
    pub pos: Vec3,
    pub vel: Vec3,
}

/// 邻居记录（6 f32/条 = pos + vel）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Neighbor {
    pub pos: Vec3,
    pub vel: Vec3,
}

/// boids 每帧核心（`Boid::frame_update`，与 reconstruction/critters 逐规则一致）。
///
/// `in_water`（鱼：`bilinear_sample_at_world_xz` 采样 >= 0.5 判水；非水 → 反向退并折返）
/// 与 `viable`（鹪鹩：RAY_DIRS 32 方向 raycast 判可行点；不可行 → 反射）由系统注入。
/// `accel_max`：加速度增量 clamp 上界（机器码 `fVar22 = 1.0; 水>=0.5 && 有目标 → 1.5`；
/// `clamp(delta, -max, max/2)` 双边界 + clamp::do_panic 断言；fish/wren 两实例同）。
#[allow(clippy::too_many_arguments)]
pub fn frame_update(
    boid: &mut Boid,
    neighbors: &[Neighbor],
    self_index: usize,
    sep_radius: f32,
    align_weight: f32,
    min_speed: f32,
    max_speed: f32,
    dt: f32,
    accel_max: f32,
) {
    let old_vel = boid.vel;
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
            // smoothstep(1 - dist/sep_r)，原式 (4-2t)t²
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
    // 加速度增量 clamp（O：fVar22 = 1.0 / 1.5（有目标），clamp(delta, -max, max/2)，
    // clamp::do_panic 断言）。
    if accel_max > 0.0 {
        let delta = (boid.vel - old_vel).clamp(
            Vec3::splat(-accel_max),
            Vec3::splat(accel_max * 0.5),
        );
        boid.vel = old_vel + delta;
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
}

/// 鱼实例收尾（frame_update 后）：水面 `pos += vel*dt`；陆面
/// `pos -= vel*dt; vel = -vel * min_speed`（反弹回水，O 反汇编）。
pub fn apply_fish_motion(boid: &mut Boid, dt: f32, in_water: bool, min_speed: f32) {
    if in_water {
        boid.pos += boid.vel * dt;
    } else {
        boid.pos -= boid.vel * dt;
        boid.vel = -boid.vel * min_speed;
    }
}

/// `Boid::random_vel_change`（0x141dc2170）：`timer -= dt`；`<=0` →
/// 2×`fastrand::f32` + `uniform_sample_cone` 取新随机锥向，`fastrand::f32`+`lerp` 重设 timer；
/// 随后 `from_rotation_arc` + 四元数旋转把 vel 转回该方向。
///
/// 返回新方向（None = 计时未到，方向不变）。
pub fn random_vel_change(
    vel: Vec3,
    timer: &mut f32,
    dt: f32,
    rng: &mut Rng,
) -> Option<Vec3> {
    *timer -= dt;
    if *timer > 0.0 {
        return None;
    }
    let u = rng.f32();
    let v = rng.f32();
    // utils::uniform_sample_cone（游戏自有；锥角由调用方注入）语义：
    let dir = uniform_sample_cone(u, v);
    *timer = 2.0 * rng.f32() + 0.5;
    let target = dir.normalize_or_zero();
    let rot = glam::Quat::from_rotation_arc(vel.normalize_or_zero(), target);
    Some(rot * vel)
}

/// `utils::uniform_sample_cone`（游戏自有采样工具；证据 0x140c90940 附近）。
/// 锥角参数由调用方常量注入（z 范围 [-1,1] 的球带采样）。
pub fn uniform_sample_cone(u: f32, v: f32) -> Vec3 {
    let z = 1.0 - 2.0 * u;
    let r = (1.0 - z * z).max(0.0).sqrt();
    let a = v * std::f32::consts::TAU;
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

/// `BoidFlock::remove` 核心（boids.rs 114–124）：对索引列表（u32 Vec）倒序遍历：
/// 校验索引 → boid 0x18 槽与 0x38 槽 swap_remove → `Entities::contains` 非空则 despawn。
/// `alive` 由系统注入（Entities::contains）。返回被移除的 boid（若存在）。
pub fn flock_remove(
    boids: &mut Vec<Boid>,
    extras: &mut Vec<f32>,
    idx: usize,
    alive: bool,
) -> Option<Boid> {
    if idx >= boids.len() {
        return None;
    }
    let removed = boids.swap_remove(idx);
    if idx < extras.len() {
        extras.swap_remove(idx);
    }
    if !alive {
        // 实体已不存在：槽已清理，无额外命令。
    }
    Some(removed)
}
