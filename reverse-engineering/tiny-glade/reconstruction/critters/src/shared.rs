//! shared 移动 / 避让链（B 主基线，反汇编级证据草稿）。
//!
//! 独立证据草稿：完整系统签名（含 bevy Query 参数）见
//! `evidence/multi-baseline/critters-behavior-analysis.md`；本文件只落反汇编钉死的
//! 组件布局与每实体语义核心（bevy-free，glam+perchance 即可）。
//!
//! 证据源：reva worker `critter-owner` 反汇编/反编译
//! critters_avoid_collisions 0x1423179a0（源码 shared.rs 27–196）、
//! smooth_critter_velocities 0x142318c10（199–210）、move_critters 0x142318fa0（214–374）；
//! radare2 RNE oracle 抽点核对；PDB 行号表 shared.rs.pdb.jsonl.gz。
//!
//! 查询签名（硬证据）：
//! - move_critters init@0x14230ecf0：`(&mut Transform, &CritterLocomotion, &Velocity,
//!   Option<&CritterVelocityMultiplier>, Option<&Duck>, Option<&Sheep>, Option<&mut BirdSpots>)`。
//! - 避让为普通 QueryIter，逐实体取 Transform/目标/4B/8B/可选 0x68 记录。
//! 时序：dt 读 `*Res<Time> + 0x28`；常量：1.0=0x3f800000，0.01=0x3c23d70a（旋转衰减底）。
use glam::Quat;
use glam::Vec3;

/// Velocity 组件（12B）：x@+0, y@+4, z@+8（反汇编证据，各系统一致）。
pub type Velocity = Vec3;

/// CritterVelocityMultiplier 组件（4B f32 @+0）。写查询闭包环境 u32（语义 unknown，见分析 §6）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityMultiplier(pub f32);

/// CritterLocomotion 组件（8B 槽，u32 @+0）。反汇编出现值 2/3/4/6/7 与位测
/// `99U >> (v & 0x1f) & 1`（0x63 位集 = {0,1,5,6}）；枚举语义 unknown。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CritterLocomotion(pub u32);

/// CritterVelocityTarget（与 firefly.rs VelocityTarget 同布局，0x10 槽）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityTarget {
    pub direction: Vec3,
    pub retention_per_second: f32,
}

/// 速度平滑（smooth_critter_velocities 0x142318c10 核心，源码 199–210；= firefly.rs smooth_velocity）。
/// 证据：`retention = powf(retention_per_second, dt)`；`velocity = velocity*retention +
/// target.dir*(1-retention)`（3 轴，原顺序非 lerp 重排）。L206 两处 powf@0x142923f70。
pub fn smooth_velocity(velocity: &mut Vec3, target: VelocityTarget, dt: f32) {
    let retention = target.retention_per_second.powf(dt);
    let weight = 1.0 - retention;
    *velocity = Vec3::new(
        velocity.x * retention + target.direction.x * weight,
        velocity.y * retention + target.direction.y * weight,
        velocity.z * retention + target.direction.z * weight,
    );
}

/// 单实体移动核心（move_critters 0x142318fa0 核心，源码 223–318）。
///
/// 证据步骤：
/// - `translation.y = max(y, 0)`（下限夹取）。
/// - `displacement = velocity * dt * multiplier`（Multiplier 缺省 1.0）。
/// - `allow_move` 由系统避让结果注入（水/岩石/墙体/spherecast，见 critters_avoid_collisions）。
/// - 应用位移后朝向：`new_rot = normalize(target_rot*(1-decay) + old_rot*decay)`，
///   `decay = powf(0.01, dt)`；`target_rot = Quat::from_rotation_arc(dir, up)`。
///   证据：`fVar31=powf(0x3c23d70a, dt)`，`fVar38=1-fVar31`；SIMD divps/sqrtps 归一化。
pub fn move_critter_core(pos: &mut Vec3, vel: Vec3, multiplier: f32, dt: f32, allow_move: bool) {
    pos.y = pos.y.max(0.0);
    if allow_move {
        *pos += vel * (dt * multiplier);
    }
}

/// 朝向更新核心（证据同 move_critter_core）。
pub fn face_movement(
    rot: &mut Quat,
    move_dir: Vec3,
    dt: f32,
) {
    let decay = 0.01_f32.powf(dt);
    let weight = 1.0 - decay;
    let target = Quat::from_rotation_arc(move_dir.normalize_or_zero(), Vec3::Y);
    // 原实现为四元数分量线性混合 + 归一化（nlerp）。
    *rot = rot.lerp(target, weight).normalize();
}

/// 避让决策（critters_avoid_collisions 0x1423179a0 每实体核心，源码 51–196）。
///
/// 证据步骤（反汇编 VA）：
/// - 仅处理 `CritterGoal.active == 1`（`while(*piVar17 != 1)`）。
/// - dir = normalize(goal - pos)（x/z 2D）。
/// - `Circle2d::new_rotated` + `LineInterval::line_segment`（构移动段）；`bilinear_sample
///   at_world_xz`（水/地形）+ `terrain_rock_mask::sample_at_world_xz`。
/// - 墙体：`WallSpatialHash::query` + `PublicWalls::get` + `max_y_gable < pos.y` 跳过，
///   `LineInterval::relate` 判相交；水 8×bilinear 采样。
/// - `RaycastWorld::legacy_spherecast`：命中或球距 `<=1.0` → 保持目标（blocked）；
///   球距 `<0.1` 且命中标志 → `EventWriter::write<DespawnDuckCmd>`（despawn）。
/// - 收尾清 `CritterGoal.active = 0`。
///
/// 本函数把查询/几何结果（wall_blocked / sphere_hit / dist）作为输入，返回系统应做的动作。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvoidanceAction {
    /// 球检命中或目标仍 `>=1.0` 远：保持当前目标继续避让（不移动/不撤销）。
    KeepMoving,
    /// 已抵达（无命中且 `<1.0`）：清 CritterGoal.active。
    Arrived,
    /// 已贴近（`<0.1`）且命中标志：发 DespawnDuckCmd 并清目标。
    Despawn,
}

#[allow(clippy::too_many_arguments)]
#[allow(unused_variables)] // goal_pos/pos 为 ABI 输入；方向与 1.0 距离门在调用方按证据计算。
pub fn decide_avoidance(
    goal_pos: Vec3,
    pos: Vec3,
    wall_blocked: bool,
    water_blocked: bool,
    sphere_hit: bool,
    sphere_dist: f32,
) -> AvoidanceAction {
    if wall_blocked || water_blocked {
        return AvoidanceAction::KeepMoving;
    }
    // 证据（反汇编）：`if (spherecast 命中 || 1.0 <= dist) goto continue`（保留目标）；
    // 否则 `if (dist < 0.1 && flag) write<DespawnDuckCmd>`；随后 `*goal.active = 0`。
    if sphere_hit || sphere_dist >= 1.0 {
        return AvoidanceAction::KeepMoving;
    }
    if sphere_dist < f32::from_bits(0x3ecccccd) {
        AvoidanceAction::Despawn
    } else {
        AvoidanceAction::Arrived
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smooth_matches_firefly_draft() {
        let mut vel = Vec3::ONE;
        let target = VelocityTarget { direction: Vec3::new(0.0, 0.0, 1.0), retention_per_second: 0.05 };
        smooth_velocity(&mut vel, target, 1.0);
        assert!(vel.z > 0.9);
        assert!(vel.x < 0.1);
    }

    #[test]
    fn move_scales_by_dt_times_multiplier() {
        let mut p = Vec3::ZERO;
        move_critter_core(&mut p, Vec3::new(2.0, 0.0, 0.0), 0.5, 0.25, true);
        assert_eq!(p, Vec3::new(0.25, 0.0, 0.0));
    }

    #[test]
    fn block_prevents_move() {
        let mut p = Vec3::ZERO;
        move_critter_core(&mut p, Vec3::new(2.0, 0.0, 0.0), 1.0, 0.25, false);
        assert_eq!(p, Vec3::ZERO);
    }

    #[test]
    fn avoid_decision_near_goal() {
        let a = decide_avoidance(Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO, false, false, false, 0.05);
        assert_eq!(a, AvoidanceAction::Despawn);
        let b = decide_avoidance(Vec3::new(5.0, 0.0, 0.0), Vec3::ZERO, true, false, false, 0.0);
        assert_eq!(b, AvoidanceAction::KeepMoving);
        let c = decide_avoidance(Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO, false, false, true, 0.3);
        assert_eq!(c, AvoidanceAction::KeepMoving);
        let d = decide_avoidance(Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO, false, false, false, 0.5);
        assert_eq!(d, AvoidanceAction::Arrived);
    }
}
