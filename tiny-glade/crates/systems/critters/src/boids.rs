//! boids 群体规则（B 主基线，fresh 反编译级证据，2026-10-17 重建）。
//!
//! 证据：
//! - `Boid::random_vel_change` 0x141dc2170；
//! - `Boid::frame_update<fish>` 0x141dc2740 / `<wren>` 0x141dc3510；
//! - `BoidFlock::remove` 0x141dc2340 / 0x141dc2540。
//!
//! 逐语句重建要点（fresh 反编译 + disassemble_function 复核）：
//! - `random_vel_change`：`timer -= dt`；`<=0` 时 3×`fastrand::Rng::f32()`：
//!   前两个进 `utils::uniform_sample_cone(u, v, cone=0x3f7e76c9≈0.99376)`（窄锥，z 轴近 +1），
//!   第三个进 `utils::lerp(0.1, 0.3, t)` 重设 timer；随后
//!   `time_continuous_lerp_vec3(dir_smoothed, dir, rate=4.0, dt)` 平滑方向，
//!   `Quat::from_rotation_arc(Vec3::Y, dir_smoothed)` 得旋转，`vel = q * vel`。
//! - `frame_update`（两实例共用泛型，仅 viable 闭包与 goal 标志不同）：
//!   1. `random_vel_change`；`vel.y > 0` → `time_continuous_lerp(vel.y, 0, 0.4, dt)` 水平化。
//!   2. 邻居循环（stride 6 f32）：`dist² < 16.0` 且双 heading 门
//!      （`vel·nvel >= 0` 与 `d·nvel/(|d||nvel|) >= -0.5`）才计入；
//!      分离 `sep -= normalize(d) * smoothstep(1 - dist/sep_r)`（(3-2t)t² 光滑步）；
//!      对齐 `vel = vel*(1-align_w) + mean_nvel/c*align_w`；
//!      凝聚 `vel += normalize(mean_pos/c - pos) * |vel| * 0.005`（fish/wren 同为 0.005）；
//!      `vel += sep * 1.7`。
//!   3. goal 激活 → `vel = time_continuous_lerp_vec3(vel, goal-pos, rate, dt)`
//!      （rate = max(1, 1/max(|d|,1e-5)²) * 100）。
//!   4. 前瞻采样 `viable(pos + lookahead*normalize(vel))`，lookahead = min(0.9|d|, 4.0) 或 4.0；
//!      直接可行 → accel 上限 1.0（goal 时 1.5）；否则 RAY_DIRS 32 向（按
//!      `from_rotation_arc(Vec3::Y, normalize(vel))` 旋转）逐个 `viable(pos + 4.0*r)` 找可行向，
//!      命中 → `vel = normalize(r)*|vel|*6.0 - vel*5.0`，accel 上限 3.0；全不可行 → 原地不动返回。
//!   5. accel 增量：`|vel - old_vel| > max` 时整体缩到 max；再仅对 y 分量 clamp[-max/2, max/2]；
//!      `vel = old_vel + (dx, clamp(dy), dz)`。
//!   6. 速度约束 [bound, max_speed]：bound = min_speed（无 goal）或
//!      `lerp(0.2, 1.0, smoothstep(min(1, |goal|*0.5))) * min_speed`（有 goal）；
//!      NaN → vel 归零。
//!   7. 移动：`disp = vel*dt`；`viable(pos)` → `pos += disp`，否则 `pos -= disp; vel = -vel*min_speed`。
//! - `BoidFlock::remove`：索引 Vec<u32> 倒序遍历；校验索引 < len 后对 boid 0x18 槽与
//!   aux 0x38 槽 swap_remove；`Entities::contains` 存活则 despawn。
//!
//! RAY_DIRS 表（32×Vec3，惰性初始化 Lazy）：精确 32 方向字面量未从原二进制提取
//! （Once init 闭包运行期填充），本文件以 golden-angle 球面序占位（分歧点，见 RESTORE.md）。

use fastrand::Rng;
use glam::{Quat, Vec3};
use std::sync::LazyLock;

/// Boid 记录（0x18）：pos Vec3@+0 + vel Vec3@+0xc。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Boid {
    pub pos: Vec3,
    pub vel: Vec3,
}

/// BoidFlock 的 aux 槽（stride 0x38）：goal + 平滑方向/原始方向/计时。
/// 布局（反汇编）：goal_active u32@+0、goal Vec3@+4、dir_smoothed@+0x18、
/// dir@+0x24、timer f32@+0x30。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoidState {
    /// 目标（None = goal_active==0）。
    pub goal: Option<Vec3>,
    /// 时间连续插值后的方向（@+0x18）。
    pub dir_smoothed: Vec3,
    /// 最新随机锥向（@+0x24）。
    pub dir: Vec3,
    /// 下次随机转向的计时（@+0x30）。
    pub timer: f32,
}

/// 邻居记录（6 f32/条 = pos + vel）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Neighbor {
    pub pos: Vec3,
    pub vel: Vec3,
}

/// `Boid::random_vel_change`（0x141dc2170）。
///
/// 时序（disasm 141dc2170–141dc2170）：
/// `timer -= dt`；`<=0` → 3×`rng.f32()`：u/v 进 `uniform_sample_cone(cone=0.3f7e76c9)`、
/// t 进 `lerp(0.1, 0.3, t)` 写 timer；随后 `time_continuous_lerp_vec3(dir_smoothed, dir,
/// 4.0, dt)` 平滑；`Quat::from_rotation_arc(Vec3::Y, dir_smoothed)` 旋转速度写回。
pub fn random_vel_change(boid: &mut Boid, state: &mut BoidState, dt: f32, rng: &mut Rng) {
    state.timer -= dt;
    if state.timer <= 0.0 {
        let u = rng.f32();
        let v = rng.f32();
        state.dir = uniform_sample_cone(u, v, CONE_MAX);
        state.timer = utils_lerp(0.1, 0.3, rng.f32());
    }
    state.dir_smoothed = time_continuous_lerp_vec3(state.dir_smoothed, state.dir, 4.0, dt);
    let q = Quat::from_rotation_arc(Vec3::Y, state.dir_smoothed);
    boid.vel = q * boid.vel;
}

/// `utils::uniform_sample_cone(u, v, cone)`（0x140c90880）：
/// `z = cone*u + (1-u)`；`r = min(1, sqrt(1-z²))`；`a = v*2π`；`(r*cos a, r*sin a, z)`。
pub fn uniform_sample_cone(u: f32, v: f32, cone: f32) -> Vec3 {
    let z = cone * u + (1.0 - u);
    let r = (1.0 - z * z).max(0.0).sqrt().min(1.0);
    let a = v * std::f32::consts::TAU;
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

/// 锥采样上限（random_vel_change 调用点 0x142f7d5f4 = 0x3f7e76c9）。
pub const CONE_MAX: f32 = 0.99376;

/// `utils::lerp(a, b, t)` = a + (b-a)*t。
pub fn utils_lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// `utils::time_continuous_lerp(cur, target, rate, dt)`（0x140c904e0）：
/// `cur + (target-cur) * (1 - exp(-rate*dt))`。
pub fn time_continuous_lerp(current: f32, target: f32, rate: f32, dt: f32) -> f32 {
    current + (target - current) * (1.0 - (-rate * dt).exp())
}

/// `utils::time_continuous_lerp_vec3(cur, target, rate, dt)`（0x140c90600）逐分量。
pub fn time_continuous_lerp_vec3(current: Vec3, target: Vec3, rate: f32, dt: f32) -> Vec3 {
    let f = 1.0 - (-rate * dt).exp();
    Vec3::new(
        current.x + (target.x - current.x) * f,
        current.y + (target.y - current.y) * f,
        current.z + (target.z - current.z) * f,
    )
}

/// boids 每帧核心（`Boid::frame_update`，0x141dc2740/<wren> 0x141dc3510 共用泛型）。
///
/// `viable` 闭包 = 某点可否进入：fish 实例 = `bilinear_sample_at_world_xz(x,z,130,130) >= 0.5`
/// （水面）；wren 实例 = `!closure_3(ray)`（RAY_DIRS 射线未命中障碍）。
/// `goal` 经 `state.goal` 表达；fish 实例 active!=0 即 lerp，wren 实例仅 active==1 时 lerp
/// （调用方在 active!=1 时传 None）。
#[allow(clippy::too_many_arguments)]
pub fn frame_update(
    boid: &mut Boid,
    state: &mut BoidState,
    neighbors: &[Neighbor],
    self_index: usize,
    sep_radius: f32,
    align_weight: f32,
    min_speed: f32,
    max_speed: f32,
    dt: f32,
    rng: &mut Rng,
    viable: &dyn Fn(Vec3) -> bool,
) {
    let old_vel = boid.vel;

    random_vel_change(boid, state, dt, rng);

    // 水平化：vel.y > 0 → 向 0 衰减（rate 0.4，0x142925970）。
    if boid.vel.y > 0.0 {
        boid.vel.y = time_continuous_lerp(boid.vel.y, 0.0, 0.4, dt);
    }

    // 邻居循环。
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
        if dist2 < 16.0 {
            let nvel = n.vel;
            // FIX⑦：二进制 loop-versioning——count==0（尚无计数的邻居）只判
            // `d·nvel/(|d||nvel|) >= -0.5` 门；count>0 才加 `vel·nvel >= 0` 门。
            let dot_heading = d.dot(nvel) / (d.length() * nvel.length());
            let gate = if count == 0 {
                dot_heading >= -0.5
            } else {
                boid.vel.dot(nvel) >= 0.0 && dot_heading >= -0.5
            };
            if gate {
                count += 1;
                mean_pos += n.pos;
                mean_vel += nvel;
                if dist2 < sep_radius * sep_radius {
                    let dist = dist2.sqrt();
                    let t = (1.0 - dist / sep_radius).clamp(0.0, 1.0);
                    let smooth = (3.0 - 2.0 * t) * t * t;
                    sep -= (d / dist) * smooth;
                }
            }
        }
    }
    if count > 0 {
        let c = count as f32;
        boid.vel = boid.vel * (1.0 - align_weight) + (mean_vel / c) * align_weight;
        let cohesion = (mean_pos / c - boid.pos).normalize_or_zero();
        boid.vel += cohesion * boid.vel.length() * COHESION_WEIGHT;
    }
    boid.vel += sep * SEP_WEIGHT;

    // goal 激活 → 向目标位移方向时间连续插值（fish 0x141dc2cf4 块）。
    if let Some(g) = state.goal {
        let d = g - boid.pos;
        let rate = (1.0 / d.length().max(1e-5).powi(2)).max(1.0) * 100.0;
        boid.vel = time_continuous_lerp_vec3(boid.vel, d, rate, dt);
    }

    // 前瞻距离（0x142a7c88c=0.9, 0x142934694=4.0, 0x142950d70=1e-5, 0x14292c76c=100.0）。
    let lookahead = match state.goal {
        Some(g) => (0.9 * (g - boid.pos).length()).min(4.0),
        None => 4.0,
    };
    let dir_norm = boid.vel.normalize_or_zero();

    let accel_max = if viable(boid.pos + dir_norm * lookahead) {
        if state.goal.is_some() {
            1.5
        } else {
            1.0
        }
    } else {
        // RAY_DIRS 搜索：rays 按 from_rotation_arc(Vec3::Y, normalize(vel)) 旋转，
        // 逐个 viable(pos + 4.0*r)；命中 → 新速度 + accel 上限 3.0。
        let q = Quat::from_rotation_arc(Vec3::Y, dir_norm);
        let mut found = false;
        for ray in RAY_DIRS.iter() {
            let r = q * *ray;
            if viable(boid.pos + r * 4.0) {
                let rn = r.normalize_or_zero();
                boid.vel = rn * boid.vel.length() * 6.0 - boid.vel * 5.0;
                found = true;
                break;
            }
        }
        if !found {
            // 无可走方向：原地不动（disasm 141dc3034 → epilogue）。
            return;
        }
        3.0
    };

    // accel 增量 clamp（disasm 141dc3147）：
    // |delta| > max → 整体缩到 max；再仅对 y 分量 clamp[-max/2, max/2]。
    let mut delta = boid.vel - old_vel;
    if delta.length_squared() > accel_max * accel_max {
        delta = delta / delta.length() * accel_max;
    }
    let dy = delta.y.clamp(-accel_max * 0.5, accel_max * 0.5);
    boid.vel = old_vel + Vec3::new(delta.x, dy, delta.z);

    // 速度约束（0x1429cd068=0.2, 0x1429cd078=3.0, 0x142925870=0.5）。
    let bound = match state.goal {
        Some(g) => {
            let t = (1.0 * (g - boid.pos).length() * 0.5).min(1.0);
            let s = (3.0 - 2.0 * t) * t * t;
            utils_lerp(0.2, 1.0, s) * min_speed
        }
        None => min_speed,
    };
    let len = boid.vel.length();
    if len < bound {
        boid.vel = boid.vel.normalize_or_zero() * bound;
    } else if len > max_speed {
        boid.vel = boid.vel.normalize_or_zero() * max_speed;
    }

    // NaN → 归零。
    if boid.vel.is_nan() {
        boid.vel = Vec3::ZERO;
    }

    // 移动：viable(pos) → pos += vel*dt；否则 pos -= vel*dt; vel = -vel*min_speed。
    let disp = boid.vel * dt;
    if viable(boid.pos) {
        boid.pos += disp;
    } else {
        boid.pos -= disp;
        boid.vel = -boid.vel * min_speed;
    }
}

/// 兼容辅助：单实体移动收尾（与 frame_update 内移动分支语义一致；fish 调用方
/// 若改用 frame_update(viable=水面) 则无需单独调用）。水面 `pos += vel*dt`；
/// 陆面 `pos -= vel*dt; vel = -vel * min_speed`。
pub fn apply_fish_motion(boid: &mut Boid, dt: f32, in_water: bool, min_speed: f32) {
    let disp = boid.vel * dt;
    if in_water {
        boid.pos += disp;
    } else {
        boid.pos -= disp;
        boid.vel = -boid.vel * min_speed;
    }
}

/// 凝聚权重（fish/wren 两实例同 0.005，0x142f7d774）。
pub const COHESION_WEIGHT: f32 = 0.005;
/// 分离权重 1.7（0x142f7d790）。
pub const SEP_WEIGHT: f32 = 1.7;

/// RAY_DIRS 32 向（Lazy 惰性初始化；精确字面量未提取，golden-angle 球面序占位）。
pub static RAY_DIRS: LazyLock<[Vec3; 32]> = LazyLock::new(make_ray_dirs);

fn make_ray_dirs() -> [Vec3; 32] {
    // 占位：fibonacci/golden-angle 球面序（x/z 用于水面采样；方向向量语义在
    // frame_update 内按原算法使用）。精确值需从原二进制 RAY_DIRS Lazy 提取。
    // 该表仅影响"避障转向"的 32 个候选方向，不影响其余控制流。
    let mut out = [Vec3::ZERO; 32];
    for i in 0..32 {
        let ga = i as f32 * 2.3999632;
        let z = (i as f32 / 16.0 - 1.0) * 0.9; // -0.9..0.9
        let r = (1.0 - z * z).sqrt();
        out[i] = Vec3::new(r * ga.cos(), r * ga.sin(), z).normalize();
    }
    out
}

/// `BoidFlock::remove`（0x141dc2340 / 0x141dc2540）：索引列表倒序遍历，
/// boid 0x18 槽 + aux 0x38 槽 swap_remove；`alive(i)` 非空则返回该索引供系统 despawn。
/// 返回存活实体的索引（倒序；调用方按序 despawn）。
pub fn flock_remove(
    boids: &mut Vec<Boid>,
    extras: &mut Vec<BoidState>,
    indices: &mut Vec<u32>,
    alive: impl Fn(u32) -> bool,
) -> Vec<u32> {
    let mut to_despawn = Vec::new();
    for k in (0..indices.len()).rev() {
        let idx = indices[k] as usize;
        // FIX⑥：二进制 boid 越界 → break（停止整段处理；swap_remove 语义）。
        if idx >= boids.len() {
            break;
        }
        boids.swap_remove(idx);
        // FIX⑥：二进制 aux 越界 → swap_remove::assert_failed（panic）——Rust
        // `Vec::swap_remove` 对越界同样 panic，去掉条件守卫以匹配。
        extras.swap_remove(idx);
        if alive(indices[k]) {
            to_despawn.push(indices[k]);
        }
    }
    indices.clear();
    to_despawn
}
