//! butterfly 行为链（B 主基线，反汇编级证据）。
//!
//! 证据（critters-extra-analysis.md §3 + reconstruction/butterfly.rs 草稿）：
//! - update_butterfly_states 0x141d84d40（源码 43–253）：查询
//!   `(Transform, &Velocity, &mut Butterfly, &mut ButterflyAudio, &CritterGoal)`；
//!   同时收集 Sheep pos 避让。距离门：相机 mode==3 → 距 eye 4.0；否则距光标 8.0。
//!   状态机为跳表（DAT_142f68008）；dwell `<=0` → 重置 uniform*0.3+0.2；
//!   相位 `|0.5 - phase_ref|` 落 [0.2,0.9) 外 → 跳表分支。
//! - update_butterfly_goals 0x141d86060（256–331）：timer -= dt；刷新条件同 duck；
//!   新 timer = uniform*0.5+0.1；球面采样 + 状态依赖竖向偏置
//!   （state==1 → (-0.2835, 0.5)，否则 (0.8529, 1.0)）；proposed = pos + dir*(timer*1.77)；
//!   y clamp <= 8.0；校验 sqrt(z²+x²) < 65 || < 原半径。
//! - set_butterfly_velocities_towards_goals 0x141d867b0（334–378）：
//!   `dir = normalize(goal - pos) * 1.77`；multiplier += 1.0（每帧累加，语义 unknown）。
//! - update_butterfly_audio 0x141d86eb0（381–385）：逐蝴蝶 ButterflyAudio::set_position。
//! - butterflies_send_to_gpu 0x141d87140（472B）：SSBO 上传（GPU 后置；CPU 侧 = core
//!   `ButterflySsbo`，24 上限；anim_t = Butterfly+0x14 phase_ref）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use country_core::render::ssbo::StorageElement;
use country_core::systems::render_loop::resources::critters::ButterflySsbo;
use glam::Vec3;
use perchance::PerchanceContext;

use crate::audio::AudioHandle;
use crate::components::{
    Butterfly, CritterGoal, CritterVelocityMultiplier, CritterVelocityTarget, Sheep, Transform,
    Velocity,
};
use crate::duck::CritterRng;
use crate::sample::uniform_sample_sphere;

/// ButterflyAudio（core `resources::critters::ButterflyAudio` 的消费侧：set_position
/// 0x140f85ba0 = 5B thunk → 0x140a57da0 共享实现）。
#[derive(Component, Debug, Default)]
pub struct ButterflyAudio {
    pub handle: AudioHandle,
}

impl ButterflyAudio {
    /// `ButterflyAudio::set_position`（O：转发 set_position）。
    pub fn set_position(&mut self, position: Vec3) {
        self.handle.set_position(position);
    }
}

/// 距离门输入（相机 mode==3 → eye 位置；否则光标位置；阈值 4.0/8.0，O）。
#[derive(Resource, Clone, Copy, Debug)]
pub struct ButterflyProximity {
    pub eye_mode: bool,
    pub eye: Vec3,
    pub cursor: Vec3,
}

impl Default for ButterflyProximity {
    fn default() -> Self {
        Self {
            eye_mode: false,
            eye: Vec3::ZERO,
            cursor: Vec3::ZERO,
        }
    }
}

/// update_butterfly_states（0x141d84d40）：远距才推进状态机。
/// `near = dist(pos, focus) < 阈值`（eye 4.0 / cursor 8.0）；本重建把跳表 handler 的
/// 语义核心 `step_state`（dwell/相位跳变）落为可测函数，动画分支归宿主渲染域。
pub fn update_butterfly_states(
    time: Res<GameTime>,
    proximity: Res<ButterflyProximity>,
    rng: Res<CritterRng>,
    mut query: Query<(
        &Transform,
        &Velocity,
        &mut Butterfly,
        &mut ButterflyAudio,
        &CritterGoal,
        &Sheep,
    )>,
) {
    let dt = time.delta;
    let mut rng_ctx = rng.0.lock().unwrap();
    for (transform, _velocity, mut butterfly, mut audio, _goal, _sheep) in &mut query {
        let pos = transform.translation;
        let focus = if proximity.eye_mode {
            proximity.eye
        } else {
            proximity.cursor
        };
        let threshold = if proximity.eye_mode { 4.0 } else { 8.0 };
        let near = (pos - focus).length() < threshold;
        let jump = step_state(&mut butterfly, dt, near, &mut rng_ctx);
        // 音频位置（O：逐蝴蝶 set_position）。
        audio.set_position(pos);
        let _ = jump;
    }
}

/// 状态机计时步进核心（reconstruction/butterfly.rs::step_state 一致）。
pub fn step_state(
    st: &mut Butterfly,
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

/// update_butterfly_goals（0x141d86060）。
pub fn update_butterfly_goals(
    time: Res<GameTime>,
    rng: Res<CritterRng>,
    mut query: Query<(&Transform, &mut Butterfly, &mut CritterGoal)>,
) {
    let dt = time.delta;
    let mut rng_ctx = rng.0.lock().unwrap();
    for (transform, mut butterfly, mut goal) in &mut query {
        let pos = transform.translation;
        update_butterfly_goal_core(
            butterfly.state,
            &mut butterfly.goal_timer,
            &mut goal.0,
            pos,
            dt,
            &mut rng_ctx,
        );
    }
}

/// 蝴蝶目标刷新核心（reconstruction/butterfly.rs::update_goal 一致）。
pub fn update_butterfly_goal_core(
    state: u32,
    timer: &mut f32,
    goal: &mut Option<Vec3>,
    pos: Vec3,
    dt: f32,
    rng: &mut PerchanceContext,
) {
    *timer -= dt;
    let arrived = if let Some(target) = *goal {
        let d = target - pos;
        d.length_squared() < f32::from_bits(0x3c23d70b)
    } else {
        true
    };
    if *timer > 0.0 && !arrived {
        return;
    }
    *timer = rng.uniform_f32() * 0.5 + 0.1;
    let s = uniform_sample_sphere(rng);
    // 状态依赖竖向偏置（O 常量）：state==1 → (-0.2835, 0.5)，否则 (0.8529, 1.0)。
    let (bias_a, bias_b) = if state == 1 {
        (-0.2835_f32, 0.5_f32)
    } else {
        (0.8529_f32, 1.0_f32)
    };
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
    *goal = if r < 65.0 || r < old_r {
        Some(proposed)
    } else {
        None
    };
}

/// set_butterfly_velocities_towards_goals（0x141d867b0）。
/// 查询（O）：`(Transform, Sheep, CritterGoal, &mut CritterVelocityTarget,
/// &mut CritterVelocityMultiplier)`。
pub fn set_butterfly_velocities_towards_goals(
    mut query: Query<(
        &Transform,
        &Sheep,
        &CritterGoal,
        &mut CritterVelocityTarget,
        &mut CritterVelocityMultiplier,
    )>,
) {
    for (transform, _sheep, goal, mut target, mut multiplier) in &mut query {
        let pos = transform.translation;
        let d = goal.0.unwrap_or(pos) - pos;
        target.direction = d.normalize_or_zero() * 1.77;
        // O：multiplier += 1.0（每帧累加，语义 unknown）。
        multiplier.0 += 1.0;
    }
}

/// update_butterfly_audio（0x141d86eb0）：逐蝴蝶 set_position（状态机内已做；本函数
/// 为证据对照的独立系统面，宿主装配时二选一注册）。
pub fn update_butterfly_audio(mut query: Query<(&Transform, &mut ButterflyAudio)>) {
    for (transform, mut audio) in &mut query {
        audio.set_position(transform.translation);
    }
}

/// butterflies_send_to_gpu（0x141d87140）：CPU 侧实例投影为 `ButterflySsbo`
/// （24 上限；anim_t = Butterfly+0x14 phase_ref，O core render_loop 合同）。
/// 缓冲上传由宿主接线（GPU 后置）。
pub fn butterflies_send_to_gpu(
    mut out: ResMut<ButterflyGpuData>,
    query: Query<(&Transform, &Butterfly)>,
) {
    out.instances.clear();
    for (transform, butterfly) in &query {
        if out.instances.len() >= ButterflySsbo::BUFFER_LEN {
            break;
        }
        out.instances.push(ButterflySsbo::from_components(
            transform.translation,
            transform.rotation,
            butterfly.phase_ref,
        ));
    }
}

/// 蝴蝶 GPU 实例数据资源（宿主把 `instances` 写入 Butterly Buffer；GPU 后置）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ButterflyGpuData {
    pub instances: Vec<ButterflySsbo>,
}
