//! butterfly 行为状态机核心（B 主基线，反汇编级证据草稿）。
//!
//! 独立证据草稿：完整 bevy 系统（状态跳表 DAT_142f68008/DAT_142f68018 各 handler、
//! ButterflyAudio、Sheep 避让、SSBO 上传）见 `evidence/multi-baseline/critters-extra-analysis.md`；
//! 本文件只落反汇编钉死的目标刷新/速度核心（bevy-free，glam + perchance 即可）。
//!
//! 证据源：reva worker `critter-extra-owner` 反汇编/反编译
//! update_butterfly_states 0x141d84d40（源码 43–253）、update_butterfly_goals 0x141d86060（256–331）、
//! set_butterfly_velocities_towards_goals 0x141d867b0（334–378）；radare2 RNE 常量 hexdump。
//!
//! 常量（反汇编/RNE 证实）：速度标度 1.77（0x3fe147ae）、竖向偏置 state==1 → (-0.2835, 0.5)、
//! 否则 → (0.8529, 1.0)（0x142f67d24=0xbe91745d / 0x142f67d14=0x3f5a2e8c）、目标距离 = timer*1.77、
//! y clamp 8.0、半径 65.0、dwell 区间 [0.2,0.5] 与 [0.1,0.6]、距离门 4.0（相机）/8.0（光标）。
use glam::Vec3;
use perchance::PerchanceContext;

/// CritterGoal 组件（0x10 槽，同 duck/shared 布局）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CritterGoal {
    pub active: bool,
    pub pos: Vec3,
}

/// 目标刷新核心（update_butterfly_goals 0x141d86060 每实体，源码 256–331）。
///
/// 证据步骤：
/// - `timer -= dt`（butterfly 记录 f32 @ 0x…+2 段）；`timer<=0 || !goal.active || dist2(goal,pos)<0.16`
///   → 换目标。
/// - 新 timer = `uniform_f32 * 0.5 + 0.1`。
/// - 2×`uniform_f32` + `uniform_sample_sphere` + `x0z` → 球面采样；按状态选竖向偏置
///   （state==1 → (-0.2835,0.5)，否则 (0.8529,1.0)）加权；`dir = normalize(采样*(1+偏置))`。
/// - `proposed = pos + dir * (timer * 1.77)`；y clamp `<= 8.0`。
/// - 校验 `sqrt(z²+x²) < 65.0 || < 原半径` → 接受，否则 `goal.active=false`。
///
/// `sample_sphere` 为游戏 utils::uniform_sample_sphere 语义（依赖复用，可注入）。
pub fn update_goal(
    state: u32,
    timer: &mut f32,
    goal: &mut CritterGoal,
    pos: Vec3,
    dt: f32,
    rng: &mut PerchanceContext,
    mut sample_sphere: impl FnMut(&mut PerchanceContext) -> Vec3,
) {
    *timer -= dt;
    let arrived = if goal.active {
        let d = goal.pos - pos;
        d.length_squared() < f32::from_bits(0x3c23d70b)
    } else {
        true
    };
    if *timer > 0.0 && !arrived {
        return;
    }
    *timer = rng.uniform_f32() * 0.5 + 0.1;
    let s = sample_sphere(rng);
    // 状态依赖竖向偏置（反汇编常量）：state==0 → 0x142f67d24=0xbe91745d(-0.2835) 配 (0.5)；
    // state==1 → 0x142f67d14=0x3f5a2e8c(0.8529) 配 (1.0)。两常量对 64 位打包 select，逐分量配对为推断（unknown）。
    let (bias_a, bias_b) = if state == 1 {
        (0.8529_f32, 1.0_f32)
    } else {
        (-0.2835_f32, 0.5_f32)
    };
    // 反汇编组合：dir.x = sx*(bias_a*0.5 + bias_b)；dir.y/z = s*(bias_a + bias_b)（fVar14=0.5）。
    let dir = Vec3::new(
        s.x * (bias_a * 0.5 + bias_b),
        s.y * (bias_a + bias_b),
        s.z * (bias_a + bias_b),
    )
    .normalize_or_zero();
    let d = *timer * 1.77;
    let mut proposed = pos + dir * d;
    proposed.y = proposed.y.min(8.0);
    let r = (proposed.z * proposed.z + proposed.x * proposed.x).sqrt();
    let old_r = (pos.z * pos.z + pos.x * pos.x).sqrt();
    if r < 65.0 || r < old_r {
        goal.active = true;
        goal.pos = proposed;
    } else {
        goal.active = false;
    }
}

/// 目标→速度核心（set_butterfly_velocities_towards_goals 0x141d867b0，源码 334–378）。
/// 证据：`dir = normalize(goal - pos) * 1.77` 写 VelocityTarget；`multiplier += 1.0`（每帧，语义 unknown）。
/// 之后按 Butterfly.state 跳 DAT_142f68018（动画 handler，系统层，不在此重实现）。
pub fn set_velocity_towards_goal(
    goal: &CritterGoal,
    pos: Vec3,
    target_dir: &mut Vec3,
    multiplier: &mut f32,
) {
    let d = if goal.active { goal.pos - pos } else { Vec3::ZERO };
    *target_dir = d.normalize_or_zero() * 1.77;
    *multiplier += 1.0;
}

/// Butterfly 状态记录（stride 0x18，反汇编证据）：`state u32 @ +0`、`dwell f32 @ +0xc`、
/// `phase f32 @ +0x10`、`phase_ref f32 @ +0x14`（update_butterfly_states 读写）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButterflyState {
    pub state: u32,
    pub dwell: f32,
    pub phase: f32,
    pub phase_ref: f32,
}

/// 状态机计时步进核心（update_butterfly_states 0x141d84d40 每实体，源码 43–253）。
/// 距离门（相机 4.0 / 光标 8.0）由系统注入；`near` 为 true 时本函数不推进。
/// dwell 到期 → 重置 [0.2,0.5]；相位 `|0.5 - phase_ref|` 落 [0.2,0.9) 外 → 触发跳表分支（`jump=true`）。
pub fn step_state(
    st: &mut ButterflyState,
    dt: f32,
    near: bool,
    rng: &mut PerchanceContext,
) -> bool {
    if near {
        return false;
    }
    st.dwell -= dt;
    let mut jump = false;
    if st.dwell <= 0.0 {
        st.dwell = rng.uniform_f32() * 0.3 + 0.2;
        jump = true;
    }
    let phase_off = (0.5_f32 - st.phase_ref).abs();
    if phase_off < 0.2 || phase_off >= 0.9 {
        jump = true;
    }
    jump
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goal_refresh_inside_radius() {
        let mut rng = PerchanceContext::new(0x606f);
        let mut t = 0.0;
        let mut g = CritterGoal { active: false, pos: Vec3::ZERO };
        let s = sample_sphere_game;
        update_goal(0, &mut t, &mut g, Vec3::new(10.0, 0.0, 0.0), 1.0, &mut rng, s);
        assert!(g.active);
        assert!((g.pos.x * g.pos.x + g.pos.z * g.pos.z).sqrt() < 65.0);
    }

    #[test]
    fn velocity_scaled_by_1_77() {
        let mut d = Vec3::ZERO;
        let mut m = 0.0;
        set_velocity_towards_goal(
            &CritterGoal { active: true, pos: Vec3::new(2.0, 0.0, 0.0) },
            Vec3::ZERO,
            &mut d,
            &mut m,
        );
        assert!((d.x - 1.77).abs() < 1e-3);
        assert_eq!(m, 1.0);
    }

    #[test]
    fn dwell_expires_sets_jump() {
        let mut rng = PerchanceContext::new(0x606f);
        let mut st = ButterflyState { state: 0, dwell: 0.01, phase: 0.0, phase_ref: 0.0 };
        let jump = step_state(&mut st, 0.016, false, &mut rng);
        assert!(jump);
        assert!(st.dwell > 0.2 && st.dwell <= 0.5);
    }

    fn sample_sphere_game(rng: &mut PerchanceContext) -> Vec3 {
        // utils::uniform_sample_sphere（游戏自有；此处仅为测试注入简单采样）。
        let u = rng.uniform_f32();
        let v = rng.uniform_f32();
        let z = 1.0 - 2.0 * u;
        let r = (1.0 - z * z).max(0.0).sqrt();
        let a = v * std::f32::consts::TAU;
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }
}
