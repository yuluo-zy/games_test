//! duck 行为链（B 主基线，反汇编级证据草稿）。
//!
//! 独立证据草稿：依赖 bevy/country-core 的完整系统签名见
//! `evidence/multi-baseline/critters-behavior-analysis.md`；本文件只落被反汇编钉死的
//! 组件布局与每实体语义核心（bevy-free，glam+perchance 即可）。整合时再接入 ECS 包装。
//!
//! 证据源：reva worker `critter-owner`（B 持久库）反汇编/反编译
//! plan_duck_trajectories 0x14230ff00 → move_ducks_along_trajectories 0x142312200 →
//! update_duck_goals 0x142313b30 → set_duck_velocities_towards_goals 0x142314470 →
//! update_duck_ripples 0x142314b70；radare2 RNE oracle 抽点核对；PDB 行号表
//! skeleton/B/.../duck.rs.pdb.jsonl.gz（89–366/369–576/581–695/698–728/735–796）。
//!
//! 时序：bevy Time delta_seconds 一律读 `*Res<Time> + 0x28`（f32）。
//! RNG：`perchance::PerchanceContext::uniform_f32`（0x141625ae0），无 fastrand。
//! 常量（反汇编/radare2 证实）：0.16=0x3e23d70b（距离阈值），0.8=0x3f4ccccd（目标偏置），
//! 0.1=0x3dcccccd，0.5=0x3f000000，1.0=0x3f800000（.data 0x142925904），3.0=0x40400000，
//! 12.0=0x41400000（轨迹速度），1.0/0.5（update_duck_goals 新计时区间）。
use glam::Vec3;
use perchance::PerchanceContext;

/// CritterGoal 组件（0x10 槽，反汇编证据：update_duck_goals 0x142313b30 /
/// set_duck_velocities_towards_goals 0x142314470 / critters_avoid_collisions 0x1423179a0）。
/// `active u32 @ +0`（bit0 参与位测），`pos x@+4, y@+8, z@+0xc`。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CritterGoal {
    pub active: bool,
    pub pos: Vec3,
}

impl CritterGoal {
    pub fn from_bytes(b: [u8; 16]) -> Self {
        let active = u32::from_le_bytes(b[0..4].try_into().unwrap());
        let x = f32::from_le_bytes(b[4..8].try_into().unwrap());
        let y = f32::from_le_bytes(b[8..12].try_into().unwrap());
        let z = f32::from_le_bytes(b[12..16].try_into().unwrap());
        Self { active: active & 1 != 0, pos: Vec3::new(x, y, z) }
    }
}

/// CritterVelocityTarget 组件（0x10 槽）：`dir Vec3 @+0..+8`，`retention_per_second f32 @+0xc`。
/// 与 firefly.rs `VelocityTarget` 同布局（set_duck_velocities 0x142314470 /
/// smooth_critter_velocities 0x142318c10）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityTarget {
    pub direction: Vec3,
    pub retention_per_second: f32,
}

/// Duck 组件（stride 0x100）中被目标链读写的字段（证据）：
/// +0xe0 目标活动字节、+0xe1 次标志字节、+0xe4/+0xe8 两个 f32 倒数计时、
/// +0x98 Vec<Vec3> 轨迹点、+0xa0 点数 u64、+0xec f32 沿轨迹进度、+0xfc 冻结字节。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DuckGoalState {
    /// Duck+0xe0：非 0 = 目标选择循环进行中。
    pub moving: bool,
    /// Duck+0xe1：每帧归零的次标志（语义 unknown）。
    pub aux: u8,
    /// Duck+0xe4 / +0xe8：两个 f32 倒数计时（update_duck_goals 用 8B 打包 CONCAT44 读写）。
    pub dwell: [f32; 2],
}

/// DuckState 枚举字节（B 反汇编）：set_duck_velocities 只处理 ==2；move_ducks 写 2/3。
/// 2/3 的确切语义（游水/水面）为推断，标注 unknown。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DuckState(pub u8);

/// 鸭子目标刷新（update_duck_goals 0x142313b30 每实体核心，源码 duck.rs 581–695）。
///
/// 证据业务步骤（偏移相对 0x142313b30）：
/// - `moving`（Duck+0xe0）为 0（LAB_142313f00 @0x5d0）：`dwell[0] -= dt`；`<=0` →
///   2×`uniform_f32`（@0x631/0x63c）→ `dwell = (u1*3+1, u2*3+1)`（区间 [1,4]），moving=1。
/// - `moving` 非 0：`dwell` 两半均 `-= dt`（@0x?）；两半 `<=0` → 1×`uniform_f32`（@0x7ce，
///   L685）→ moving=0，高半 `u*0.5+0.5`（低半为寄存器残留，标注 unknown）。
/// - 目标保留：`goal.active` 且 `dist2(goal.pos,pos) >= 0.16` → 保留；否则
///   换目标（pick_new；选新目标时原实现另写 Duck+0xe8=0.1，语义 unknown）。
///
/// 采样/校验（`is_within_glade_shape`、`bilinear_sample_at_world_xz<RasterBufferRef<512>>`）
/// 由系统注入 pick_new 闭包（game 调用，依赖复用，不在此重实现）。
#[allow(clippy::too_many_arguments)]
pub fn update_duck_goal(
    state: &mut DuckGoalState,
    goal: &mut CritterGoal,
    pos: Vec3,
    dt: f32,
    rng: &mut PerchanceContext,
    mut pick_new: impl FnMut(&mut PerchanceContext) -> Option<Vec3>,
) {
    if !state.moving {
        state.dwell[0] -= dt;
        if state.dwell[0] <= 0.0 {
            let u1 = rng.uniform_f32();
            let u2 = rng.uniform_f32();
            // 8B 写 = (低 u1*3+1, 高 u2*3+1)，两计时区间 [1,4]。
            state.dwell[0] = u1 * 3.0 + 1.0;
            state.dwell[1] = u2 * 3.0 + 1.0;
            state.moving = true;
            state.aux = 0;
        }
        return;
    }
    state.dwell[0] -= dt;
    state.dwell[1] -= dt;
    if state.dwell[0] <= 0.0 && state.dwell[1] <= 0.0 {
        let u = rng.uniform_f32();
        // 两半均耗尽：写 8B，高半 = u*0.5+0.5；低半为寄存器残留（Ghidra 变量复用，unknown）。
        state.dwell[1] = u * 0.5 + 0.5;
        state.moving = false;
        state.aux = 0;
        return;
    }
    // 距当前目标 < 0.4（0.16 平方）或目标无 active → 换目标。
    let arrived = if goal.active {
        let d = goal.pos - pos;
        (d.x * d.x + d.y * d.y + d.z * d.z) < f32::from_bits(0x3e23d70b)
    } else {
        true
    };
    if arrived {
        if let Some(np) = pick_new(rng) {
            goal.active = true;
            goal.pos = np;
        } else {
            goal.active = false;
        }
    }
    state.aux = 0;
}

/// 目标向量 -> 速度方向（set_duck_velocities_towards_goals 0x142314470 核心，源码 698–728）。
/// 证据：仅当 `goal.active` 且 Duck+0xe0==1 时用目标方向；否则方向取零。
/// `multiplier` 各系统写查询闭包环境 u32（语义 unknown，见分析文档 §6）。
pub fn set_velocity_towards_goal(goal: &CritterGoal, duck_moving: bool, pos: Vec3, target: &mut VelocityTarget) {
    let d = if goal.active && duck_moving { goal.pos - pos } else { Vec3::ZERO };
    target.direction = d.normalize_or_zero();
}

/// 沿轨迹推进（move_ducks_along_trajectories 0x142312200 核心，源码 431–435）。
/// 证据：`progress += dir * 12.0 * dt / segment_len`（`__real_41400000`=12.0；
/// dir=±1 依"到达末端回退"，lVar26∈{0,2}）。
/// 曲线索引/姿态（`new_for_point_count<CurveCoordSemantic3D>` + Frenet 帧）见分析 §1.2。
pub fn advance_trajectory(progress: f32, speed_dir: f32, segment_len: f32, dt: f32) -> f32 {
    progress + (speed_dir * 12.0 * dt) / segment_len
}

/// Duck 记录 -> DuckGoalState（stride 0x100 裸字节）。
pub fn goal_state_from_duck_record(record: &[u8; 0x100]) -> DuckGoalState {
    let moving = record[0xe0] != 0;
    let aux = record[0xe1];
    let d0 = f32::from_le_bytes(record[0xe4..0xe8].try_into().unwrap());
    let d1 = f32::from_le_bytes(record[0xe8..0xec].try_into().unwrap());
    DuckGoalState { moving, aux, dwell: [d0, d1] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goal_tick_arrived_picks_new() {
        let mut rng = PerchanceContext::new(0x606f);
        let mut state = DuckGoalState { moving: true, aux: 0, dwell: [2.0, 2.0] };
        let mut goal = CritterGoal { active: false, pos: Vec3::ZERO };
        // 目标未活动 -> arrived -> pick_new 返回 None -> active=false。
        update_duck_goal(&mut state, &mut goal, Vec3::ZERO, 0.016, &mut rng, |_| None);
        assert!(!goal.active);
    }

    #[test]
    fn velocity_zero_when_inactive() {
        let mut t = VelocityTarget { direction: Vec3::ONE, retention_per_second: 0.05 };
        set_velocity_towards_goal(&CritterGoal { active: false, pos: Vec3::ONE }, true, Vec3::ZERO, &mut t);
        assert_eq!(t.direction, Vec3::ZERO);
    }

    #[test]
    fn trajectory_advance_simple() {
        let p = advance_trajectory(0.0, 1.0, 10.0, 0.5);
        assert_eq!(p, 0.6);
    }
}
