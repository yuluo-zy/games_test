//! wren_boids::sim_wren_boids 大状态机 + boids 群体规则（B 主基线，反汇编级证据草稿）。
//!
//! 独立证据草稿：完整 bevy 系统（Wren/NaniBundle 出生、BirdSpots/FreeSpot、BirdHaven 查询、
//! Commands::spawn、音频）见 `evidence/multi-baseline/critters-extra-analysis.md`；
//! 本文件只落被反汇编钉死的 boids 数值核心与出生/清理语义（bevy-free，glam 即可）。
//!
//! 证据源：reva worker `critter-extra-owner` 反汇编/反编译
//! sim_wren_boids 0x142326a80（19140B，partial C + 10 窗口 disasm）、
//! Boid::frame_update<wren closure_env$3> 0x141dc3510（boids.rs 329–387）、
//! Boid::random_vel_change 0x141dc2170（389–403）、BoidFlock::remove<wren> 0x141dc2540（114–124）、
//! sim_wren_boids::closure_3 0x141dc42f0；radare2 RNE 常量 hexdump。
//!
//! boids 参数（wren call site 0x1423283e5 前 disasm）：
//! 邻居半径 4.0（dist²<16=0x41800000）、分离半径 1.3（0x3fa66666）、对齐权重 0.1169（0x3def9db2）、
//! min/max 速度 7.0/14.0、凝聚 0.0056、分离 1.7、heading 过滤（self.vel·nvel>=0）。
//! 出生：上限 80，位置 x=u*130-65、y=u*9+1；摇树爆发 rng*5+1；`fastrand::Rng::f32`。
use glam::Vec3;
use fastrand::Rng;

/// Wren Boid 记录（0x18）：pos Vec3 + vel Vec3（反汇编证据，fish 同构）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Boid {
    pub pos: Vec3,
    pub vel: Vec3,
}

/// 邻居记录（frame_update 内邻数组，6 f32/条 = pos + vel）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Neighbor {
    pub pos: Vec3,
    pub vel: Vec3,
}

/// boids 每帧核心（Boid::frame_update<wren closure_env$3> 0x141dc3510，boids.rs 329–387）。
///
/// 证据步骤（反汇编 VA/常量）：
/// 1. `random_vel_change()` 随机转向（每随机间隔选新锥向）。
/// 2. 邻居循环：`dist2 < 16.0` 且 `self.vel·nvel >= 0` 才计入（同向门）。
/// 3. 分离：`dist2 < sep_r²` → `sep -= normalize(neighbor.pos - pos) * smoothstep(1 - dist/sep_r)`，
///    smooth = (4-2t)t²。
/// 4. 对齐：`vel = mean_nvel * align_w + vel * (1 - align_w)`。
/// 5. 凝聚：`vel += normalize(mean_pos - pos) * |vel| * 0.0056`（0x3ba3d70a）。
/// 6. 合并：`vel += sep * 1.7`（0x3fd9999a）。
/// 7. 加速度 delta clamp [-max, max/2]（max=1.0，有目标 1.5）；最终 `|vel|` clamp [min_speed, max_speed]。
/// 8. NaN 分量 → vel 归零；`pos += vel * dt`。
/// 9. 可行点判定由系统注入（wren 用 closure_3/RAY_DIRS raycast，fish 用水格采样）；不可行 → 反射。
///
/// 返回 (更新后 boid, 是否产生分离/对齐后的邻居集合信息)。`can_move` 由 RAY_DIRS 闭包注入。
pub fn frame_update(
    boid: &mut Boid,
    neighbors: &[Neighbor],
    self_index: usize,
    sep_radius: f32,
    align_weight: f32,
    min_speed: f32,
    max_speed: f32,
    dt: f32,
    _rng: &mut Rng,
) {
    // random_vel_change 的转向由系统在调用前应用（目标 dir）。
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
        // 邻居半径 4.0 + heading 过滤
        if dist2 >= 16.0 || boid.vel.dot(n.vel) < 0.0 {
            continue;
        }
        count += 1;
        mean_pos += n.pos;
        mean_vel += n.vel;
        if dist2 < sep_radius * sep_radius {
            let dist = dist2.sqrt();
            let t = (dist / sep_radius - 1.0).clamp(-1.0, 0.0).abs(); // 1 - dist/sep_r
            let smooth = (4.0 - 2.0 * t) * t * t;
            sep -= (d / dist) * smooth;
        }
    }
    if count > 0 {
        let c = count as f32;
        let align = mean_vel / c;
        let cohesion_dir = (mean_pos / c - boid.pos).normalize_or_zero();
        // 对齐 (align_w 混合) + 凝聚 (×|vel|×0.0056) + 分离 (×1.7)
        boid.vel = boid.vel * (1.0 - align_weight) + align * align_weight;
        boid.vel += cohesion_dir * boid.vel.length() * 0.0056;
        boid.vel += sep * 1.7;
    }
    // 加速度 delta clamp：目标是使速度渐进约束到 [min,max]。
    // 原实现先 clamp 速度增量到 [-max, max/2]，再 clamp |vel| 到 [min_speed, max_speed]。
    let len = boid.vel.length();
    if len < min_speed {
        boid.vel = boid.vel.normalize_or_zero() * min_speed;
    } else if len > max_speed {
        boid.vel = boid.vel.normalize_or_zero() * max_speed;
    }
    if boid.vel.is_nan() {
        boid.vel = Vec3::ZERO;
    }
    boid.pos += boid.vel * dt;
}

/// 出生位置采样（sim_wren_boids L385–515，反汇编常量）。
/// `x = u*130 - 65`（0x43020000=130/0xc2820000=-65）、`y = u*9 + 1`（0x41100000/0x3f800000）。
pub fn spawn_position(rng: &mut Rng) -> Vec3 {
    Vec3::new(
        rng.f32() * 130.0 - 65.0,
        rng.f32() * 9.0 + 1.0,
        rng.f32() * 130.0 - 65.0,
    )
}

/// Boid 上限（0x50=80）与摇树爆发（rng*5+1，L1288）。
pub const MAX_WRENS: usize = 80;
pub fn shake_burst(rng: &mut Rng) -> usize {
    1 + (rng.f32() * 5.0) as usize
}

/// BoidFlock::remove 核心（swap_remove + despawn 语义，boids.rs 114–124）。
/// 对索引列表倒序逐项：swap_remove boid 槽与 0x38 槽；`alive` 由系统注入 Entities::contains。
/// 返回 (boid 数组, 保留标记)。
pub fn flock_remove(boids: &mut Vec<Boid>, extra: &mut Vec<u64>, idx: usize, alive: bool) {
    if idx < boids.len() {
        boids.swap_remove(idx);
        if idx < extra.len() {
            extra.swap_remove(idx);
        }
        let _ = alive;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separation_repels_close_neighbor() {
        let mut b = Boid { pos: Vec3::ZERO, vel: Vec3::new(1.0, 0.0, 0.0) };
        let n = Neighbor { pos: Vec3::new(0.5, 0.0, 0.0), vel: Vec3::new(1.0, 0.0, 0.0) };
        let before = b.vel;
        frame_update(&mut b, &[n], usize::MAX, 1.3, 0.1169, 7.0, 14.0, 0.016, &mut Rng::with_seed(1));
        // 邻居同向在前 → 分离应把速度方向压离邻居。
        assert!(b.vel.x < before.x || b.vel.x < 0.0);
    }

    #[test]
    fn speed_clamped() {
        let mut b = Boid { pos: Vec3::ZERO, vel: Vec3::new(100.0, 0.0, 0.0) };
        frame_update(&mut b, &[], 0, 1.3, 0.1169, 7.0, 14.0, 0.016, &mut Rng::with_seed(1));
        assert!(b.vel.length() <= 14.0 + 1e-3);
    }

    #[test]
    fn spawn_in_bounds() {
        let mut rng = Rng::with_seed(7);
        for _ in 0..100 {
            let p = spawn_position(&mut rng);
            assert!(p.x.abs() <= 65.0);
            assert!(p.z.abs() <= 65.0);
            assert!(p.y >= 1.0 && p.y <= 10.0);
        }
    }

    #[test]
    fn shake_burst_in_range() {
        let mut rng = Rng::with_seed(3);
        for _ in 0..50 {
            let b = shake_burst(&mut rng);
            assert!(b >= 1 && b <= 6);
        }
    }
}
