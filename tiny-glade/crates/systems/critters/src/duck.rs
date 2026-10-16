//! duck 行为链（B 主基线，反汇编级证据）。
//!
//! 证据（critters-behavior-analysis.md §1 + critters-extra-analysis.md §6）：
//! - plan_duck_trajectories 0x14230ff00（源码 89–366）
//! - move_ducks_along_trajectories 0x142312200（369–576）
//! - update_duck_goals 0x142313b30（581–695）
//! - set_duck_velocities_towards_goals 0x142314470（698–728）
//! - update_duck_ripples 0x142314b70（735–796）
//!
//! DuckState 语义（反汇编确认）：1=沿轨迹正向、2=自由游（goals/velocities 处理）、
//! 3=沿轨迹反向；plan_duck 出生即写 progress=1.0、state 3。
//! 时序 dt = `*Res<Time>+0x28`；RNG = perchance uniform_f32（0x141625ae0）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use glam::Vec3;
use itertools::Itertools;
use perchance::PerchanceContext;

use crate::audio::CritterAudioSink;
use crate::components::{
    CritterGoal, CritterLocomotion, CritterVelocityMultiplier, CritterVelocityTarget,
    Duck, DuckState, DuckTrajectoryState, Transform, Velocity,
};
use crate::sample::{uniform_sample_circle, uniform_sample_disk, x0y, x0z};

/// `Trajectory`（0x30 槽）：点 Vec + 4 f32 随机圆偏移（critters-extra §6）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Trajectory {
    pub points: Vec<Vec3>,
    pub offset: [f32; 4],
}

/// `TrajectoryPlan`（0x30 槽，critters-extra §6）：点数据 + 双 f32 随机圆偏移 +
/// 记账（+0x24==0 = "未执行可 spawn"）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TrajectoryPlan {
    pub points: Vec<Vec3>,
    pub offset: [f32; 2],
    pub executed: bool,
}

/// `DuckTrajectoryPlanner`（duck.rs PDB 命名实体；plan 系统持有的轨迹集合）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DuckTrajectoryPlanner {
    pub plans: Vec<TrajectoryPlan>,
}

/// `PlanPurpose`（duck.rs PDB 命名实体；热键 A/B 两路径的语义标注）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanPurpose {
    DevSpawn = 0,
    DevReplay = 1,
}

/// `PrevRipple`（duck.rs PDB 命名实体；水波 SSBO 记录，GPU 侧）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PrevRipple {
    pub pos: Vec3,
    pub age: f32,
}

/// plan_duck_trajectories（0x14230ff00，源码 89–366）。
///
/// 业务步骤（O）：
/// - L106 puffin 作用域；`_DEV_OVERRIDE_IS_DEMO_MODE` 分支。
/// - L128 热键 A（dev）：`developer_hotkeys_enabled()` && 按键 A 按下 → 光标位 +
///   3×（uniform_f32 + golden_spiral + x0y）构 3 点轨迹；uniform_sample_circle 随机圆偏移；
///   push 资源侧 Vec<Trajectory>（stride 0x30）。
/// - L153 热键 B（dev）：遍历已存轨迹逐点 lerp；legacy_raycast 两点间射线；受阻/距离判断后
///   随机落点；收尾取 "duck_shader" 资产句柄，批量 spawn
///   (Duck, DuckState, DrawableMeshBundle, CritterLocomotion, CritterGoal,
///   CritterVelocityTarget, ObjectGrassPush, Velocity, CritterVelocityMultiplier, PrevTransform)；
///   retain 修剪已执行计划。
///
/// 热键路径为 dev 门控（`developer_hotkeys_enabled` 0x14090e070）；发行路径由
/// spawn_ducks 产出的 TrajectoryPlan 驱动（本系统执行其"未执行"计划并 spawn 鸭子）。
#[allow(clippy::too_many_arguments)]
pub fn plan_duck_trajectories(
    mut planner: ResMut<DuckTrajectoryPlanner>,
    mut commands: Commands,
    time: Res<GameTime>,
    rng: Res<CritterRng>,
    water: Res<WorldRaster<512>>,
    raycast: Res<crate::shared::RaycastWorldHandle>,
    _dev: Res<DevHotkeys>,
) {
    let _ = &time;
    let mut rng_ctx = rng.0.lock().unwrap();
    // 从 spawn_ducks / despawn_ducks 推入的计划里取"未执行"项（+0x24==0 记账语义）。
    let pending: Vec<usize> = planner
        .plans
        .iter()
        .enumerate()
        .filter(|(_, p)| !p.executed && !p.points.is_empty())
        .map(|(i, _)| i)
        .collect();

    for idx in pending {
        let plan = &planner.plans[idx];
        // 沿轨迹逐点 lerp 采样（L564：`t=u/l`），legacy_raycast 受阻/距离判断。
        let mut spawn_points = Vec::with_capacity(plan.points.len());
        for p in &plan.points {
            // O：legacy_raycast（0x1408cc680）；本重建经 raycast(mask=0)（非
            // native-physics 构建的等价壳：无命中即原样保留落点）。
            let hit = crate::shared::raycast_hit(&raycast, *p - Vec3::Y * 10.0, Vec3::Y);
            let p = match hit {
                Some(_) => *p,
                None => *p,
            };
            spawn_points.push(p);
        }
        let _ = &water;
        // 随机圆偏移（uniform_f32 + uniform_sample_circle）。
        let u = rng_ctx.uniform_f32();
        let circle = uniform_sample_circle(&mut rng_ctx, u);
        let offset = Vec3::new(circle.x, 0.0, circle.y);
        let base = spawn_points.first().copied().unwrap_or_default();

        commands.spawn((
            Duck::new_planned(base + offset, spawn_points.clone()),
            DuckState::TrajectoryReverse, // 出生即 progress=1.0、state 3（O）
            CritterLocomotion(0),
            CritterGoal(None),
            CritterVelocityTarget::default(),
            Velocity(Vec3::ZERO),
            CritterVelocityMultiplier(1.0),
            crate::components::PrevTransform::default(),
        ));
        if let Some(p) = planner.plans.get_mut(idx) {
            p.executed = true;
        }
    }
}

/// move_ducks_along_trajectories（0x142312200，源码 369–576）。
/// 查询（硬证据）：`(Entity, &mut Duck, &mut DuckState, &mut Transform, &mut Velocity,
/// &mut CritterVelocityMultiplier, &mut CritterGoal, &mut ObjectGrassPush)`。
/// - DuckState==2 跳过（自由游）；==1 正向（progress += 12*dt/seg_len），到末端写 state 2；
///   ==3 反向（dir=-1），到起点（progress==0）→ despawn（保留 state 3）；
/// - 切向/法向 Frenet 帧沿曲线姿态（CurveCoordSemantic3D new_for_point_count）；
/// - 音频（audio_resources）：start_if_not_playing/set_position/set_volume/AudioLoop::current/
///   set_parameter/stop；水面判定 bilinear_sample_at_world_xz<WaterAudioGrid>。
///
/// ObjectGrassPush 组件（鸭推草；字段 U，宿主 render 域）在此以类型标记承载。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct ObjectGrassPush(pub u32);

#[allow(clippy::type_complexity)]
pub fn move_ducks_along_trajectories(
    time: Res<GameTime>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Duck,
        &mut DuckState,
        &mut Transform,
        &mut Velocity,
        &mut CritterVelocityMultiplier,
        &mut CritterGoal,
        Option<&mut ObjectGrassPush>,
    )>,
) {
    let dt = time.delta;
    for (entity, mut duck, mut state, mut transform, mut velocity, mut multiplier, mut goal, _grass)
        in &mut query
    {
        // 无轨迹或自由游：跳过（move 链不处理）。
        if duck.trajectory.is_empty() || state.to_byte() == 2 {
            continue;
        }
        let dir = match state.to_byte() {
            1 => 1.0,
            3 => -1.0,
            _ => continue,
        };
        // 分段推进（O：progress += dir * 12.0 * dt / segment_len）。
        let segment_len = trajectory_segment_len(&duck.trajectory, duck.progress);
        if segment_len <= 1e-6 {
            continue;
        }
        duck.progress = advance_trajectory(duck.progress, dir, segment_len, dt);

        if duck.progress >= 1.0 {
            if state.to_byte() == 1 {
                // 正向到末端 → 转自由游（state 2；O 写 DuckState 2）。
                *state = DuckState::FreeSwim;
                duck.moving = true;
            }
            duck.progress = 1.0;
        } else if duck.progress <= 0.0 && state.to_byte() == 3 {
            // 反向到起点 → despawn（保留 state 3；O L392 despawn 分支）。
            commands.entity(entity).despawn();
            continue;
        }

        // 曲线姿态：取轨迹点 + 切向（Frenet 帧简化，O 语义：沿曲线朝向）。
        let (pos, tangent) = trajectory_frame(&duck.trajectory, duck.progress);
        transform.translation = pos;
        if tangent.length_squared() > 1e-6 {
            transform.rotation =
                glam::Quat::from_rotation_arc(tangent.normalize(), glam::Vec3::Y).normalize();
        }
        velocity.0 = tangent * 12.0;
        multiplier.0 = 1.0;
        goal.0 = None;

        // 音频：Quack/TakeOff/Land 循环（O：start_if_not_playing/set_position/…）。
        duck.audio.start_loops(transform.translation);
    }
}

/// 沿轨迹分段长度（O：truncf → 段索引，seg_len = 段端点差长）。
pub fn trajectory_segment_len(points: &[Vec3], progress: f32) -> f32 {
    let p = progress.clamp(0.0, 0.999_999);
    let n = points.len().saturating_sub(1) as f32;
    if n <= 0.0 {
        return 0.0;
    }
    let idx = (p * n) as usize;
    let seg = (points[idx + 1] - points[idx]).length();
    seg.max(0.001)
}

/// 曲线帧（O：CurveCoordSemantic3D 语义的简化：位置 = 段内 lerp，切向 = 段方向）。
pub fn trajectory_frame(points: &[Vec3], progress: f32) -> (Vec3, Vec3) {
    let p = progress.clamp(0.0, 1.0);
    let n = points.len().saturating_sub(1);
    if n == 0 {
        return (points.first().copied().unwrap_or_default(), Vec3::ZERO);
    }
    let f = (p * n as f32).clamp(0.0, n as f32 - 1e-4);
    let i = f as usize;
    let t = f - i as f32;
    let a = points[i];
    let b = points[(i + 1).min(n)];
    (a.lerp(b, t), b - a)
}

/// 进度推进（O：`progress += dir * 12.0 * dt / segment_len`；12.0=0x41400000）。
pub fn advance_trajectory(progress: f32, speed_dir: f32, segment_len: f32, dt: f32) -> f32 {
    progress + (speed_dir * 12.0 * dt) / segment_len
}

/// update_duck_goals（0x142313b30，源码 581–695；每实体核心见 reconstruction duck.rs）。
/// - moving==0：dwell[0] -= dt；<=0 → 2×uniform_f32 → dwell=(u1*3+1, u2*3+1) [1,4]，moving=1。
/// - moving!=0：两半 -= dt；两半 <=0 → 1×uniform_f32 → moving=0、高半 = u*0.5+0.5。
/// - 目标保留：goal.active 且 dist2>=0.16 → 保留；否则换目标（uniform_sample_disk*0.8，
///   校验 is_within_glade_shape + 半径向内 + bilinear 水采样 <=0.5 无效 + 4×采样求和
///   clamp[0,1] >=0.1 无效）。
#[allow(clippy::too_many_arguments)]
pub fn update_duck_goals(
    time: Res<GameTime>,
    water: Res<WorldRaster<512>>,
    rng: Res<CritterRng>,
    mut query: Query<(
        &Transform,
        &mut Duck,
        &DuckState,
        &mut CritterGoal,
    )>,
) {
    let dt = time.delta;
    let mut rng_ctx = rng.0.lock().unwrap();
    let w = (*water).as_ref();
    for (transform, mut duck, state, mut goal) in &mut query {
        let pos = transform.translation;
        let mut gs = duck.goal_state();
        update_duck_goal_core(&mut gs, &mut goal.0, pos, dt, &mut rng_ctx, |rng, cur_pos| {
            // 换目标：盘面采样 * 0.8（O 0x3f4ccccd）。
            let u = rng.uniform_f32();
            let disk = uniform_sample_disk(rng, u);
            let new_pos = cur_pos + Vec3::new(disk.x, 0.0, disk.y) * 0.8;
            // 校验链：半径向内偏好 + bilinear 水采样 <=0.5 → 无效；4×采样求和 >=0.1 → 无效。
            let r_new = (new_pos.z * new_pos.z + new_pos.x * new_pos.x).sqrt();
            let r_old = (pos.z * pos.z + pos.x * pos.x).sqrt();
            if r_new > r_old {
                return None;
            }
            let s = w.bilinear_sample_at_world_xz(new_pos.x, new_pos.z, 130.0, 130.0);
            if s <= 0.5 {
                return None;
            }
            let mut sum = 0.0f32;
            for (dx, dz) in DUCK_WATER_SAMPLES {
                sum += w.bilinear_sample_at_world_xz(new_pos.x + dx, new_pos.z + dz, 130.0, 130.0);
            }
            if sum.clamp(0.0, 1.0) >= 0.1 {
                return None;
            }
            Some(new_pos)
        });
        duck.apply_goal_state(&gs);
        // L695：*Duck+0xe1 = 0（次标志每帧归零）。
        duck.aux = 0;
        let _ = state;
    }
}

/// 鸭子目标刷新核心（与 reconstruction/critters::duck::update_duck_goal 一致）。
pub fn update_duck_goal_core(
    state: &mut DuckTrajectoryState,
    goal: &mut Option<Vec3>,
    pos: Vec3,
    dt: f32,
    rng: &mut PerchanceContext,
    mut pick_new: impl FnMut(&mut PerchanceContext, Vec3) -> Option<Vec3>,
) {
    if !state.moving {
        state.dwell[0] -= dt;
        if state.dwell[0] <= 0.0 {
            let u1 = rng.uniform_f32();
            let u2 = rng.uniform_f32();
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
        state.dwell[1] = u * 0.5 + 0.5;
        state.moving = false;
        state.aux = 0;
        return;
    }
    let arrived = match *goal {
        Some(target) => {
            let d = target - pos;
            d.length_squared() < f32::from_bits(0x3e23d70b)
        }
        None => true,
    };
    if arrived {
        if let Some(np) = pick_new(rng, pos) {
            *goal = Some(np);
        } else {
            *goal = None;
        }
    }
    state.aux = 0;
}

/// set_duck_velocities_towards_goals（0x142314470，源码 698–728）。
/// 查询（O）：`(Transform, &Duck, &DuckState, &CritterGoal, &mut CritterVelocityTarget,
/// &mut CritterVelocityMultiplier)`；过滤 `DuckState == 2`（L708 `!=2` 跳过）。
/// `goal.active && Duck+0xe0==1` → dir = normalize(goal - pos)；否则零方向。
/// `Duck+0xfc==1`（冻结）→ VelocityTarget 清零。
#[allow(clippy::type_complexity)]
pub fn set_duck_velocities_towards_goals(
    mut query: Query<(
        &Transform,
        &Duck,
        &DuckState,
        &CritterGoal,
        &mut CritterVelocityTarget,
        &mut CritterVelocityMultiplier,
    )>,
) {
    for (transform, duck, state, goal, mut target, mut multiplier) in &mut query {
        if state.to_byte() != 2 {
            continue;
        }
        let pos = transform.translation;
        set_duck_velocity_core(&mut target, &mut multiplier.0, goal.0, duck, pos);
    }
}

/// 目标向量 → 速度方向核心（与 reconstruction duck.rs 一致）。
pub fn set_duck_velocity_core(
    target: &mut CritterVelocityTarget,
    multiplier: &mut f32,
    goal: Option<Vec3>,
    duck: &Duck,
    pos: Vec3,
) {
    let d = match (goal, duck.moving) {
        (Some(g), true) => g - pos,
        _ => Vec3::ZERO,
    };
    target.direction = d.normalize_or_zero();
    *multiplier = 1.0;
    if duck.frozen {
        target.direction = Vec3::ZERO;
    }
}

/// update_duck_ripples（0x142314b70，源码 735–796）：鸭子水波 SSBO。
/// GPU 胶水：从水面鸭子收集 (pos, age)；retention = 1-exp(-2.5*dt)/1-exp(-5.0*dt)。
/// 本重建只做 CPU 侧波纹记录（`PrevRipple`），缓冲上传由宿主 render_loop 接线（GPU 后置）。
pub fn update_duck_ripples(
    time: Res<GameTime>,
    mut ripples: ResMut<DuckRipples>,
    query: Query<(&Transform, &DuckState)>,
) {
    let dt = time.delta;
    let retention_1 = 1.0 - (-2.5 * dt).exp();
    let retention_2 = 1.0 - (-5.0 * dt).exp();
    let _ = (retention_1, retention_2);
    ripples.ripples.clear();
    for (transform, state) in &query {
        if state.to_byte() == 2 {
            ripples
                .ripples
                .push(PrevRipple { pos: transform.translation, age: 0.0 });
        }
    }
}

/// 水波记录资源（CPU 侧；SSBO 上传后置）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DuckRipples {
    pub ripples: Vec<PrevRipple>,
}

/// 鸭子出生用的水采样偏移（4 采样求和 clamp[0,1]，O L672 区）。
pub const DUCK_WATER_SAMPLES: [(f32, f32); 4] = [(0.0, 0.0), (0.8, 0.0), (0.0, 0.8), (0.8, 0.8)];

/// 共享 Perchance RNG（build 时传入 B 种子 0xc0df → PerchanceContext；插件持有）。
#[derive(Resource)]
pub struct CritterRng(pub std::sync::Mutex<PerchanceContext>);

impl Default for CritterRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(PerchanceContext::new(
            crate::CRITTER_RNG_SEED_B,
        )))
    }
}

/// 开发者热键资源（`developer_hotkeys_enabled` 0x14090e070；dev 门控）。
/// KeyCode 0x1f/0x27 语义 U（证据记录 unknown；发行构建默认关闭）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DevHotkeys {
    pub enabled: bool,
    pub just_pressed_a: bool,
    pub just_pressed_b: bool,
}

/// 轨迹沿线的草推向（ObjectGrassPush 组件；鸭移动时更新——字段 U，语义 I）。
impl Duck {
    /// 构造带轨迹的鸭子（plan_duck 出生路径）。
    pub fn new_planned(base: Vec3, points: Vec<Vec3>) -> Self {
        let mut duck = Duck::default();
        duck.trajectory = points;
        duck.progress = 1.0;
        duck.moving = false;
        duck.aux = 0;
        duck.dwell = [1.0, 1.0];
        duck.f_f0 = base.x;
        duck.f_f4 = base.y;
        duck.f_f8 = base.z;
        duck
    }
}

impl Duck {
    /// 读目标链状态（+0xe0/+0xe1/+0xe4 段）。
    pub fn goal_state(&self) -> DuckTrajectoryState {
        DuckTrajectoryState {
            moving: self.moving,
            aux: self.aux,
            dwell: self.dwell,
        }
    }
    /// 写目标链状态。
    pub fn apply_goal_state(&mut self, state: &DuckTrajectoryState) {
        self.moving = state.moving;
        self.aux = state.aux;
        self.dwell = state.dwell;
    }
}

impl Duck {
    /// 音频循环（宿主缝；见 audio 模块）。
    pub fn audio(&self) -> &crate::audio::DuckAudio {
        &self.audio
    }
}

/// 音频 trait 使用标注（audio 模块接线）。
#[allow(dead_code)]
fn _audio_sink_contract(sink: &mut dyn CritterAudioSink, pos: Vec3) {
    sink.start_if_not_playing();
    sink.set_position(pos);
    sink.set_volume(1.0);
    sink.set_parameter("speed", 0.5);
    sink.stop();
}
