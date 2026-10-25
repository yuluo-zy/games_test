//! firefly 行为链（B 主基线，反汇编级证据）。
//!
//! 证据（critters-extra-analysis.md §4；reviewer-B RNE 抽验 0x142ccfc58 双重确认）：
//! - update_firefly_goals 0x142316940（源码 46–112）：入口 `*(char *)(*in_R8 + 4) != 1 → return`
//!   （control 门）。每萤：`timer -= dt`；刷新条件 timer<=0 || goal.inactive || dist2<0.16。
//!   惯性 bias = `0xbf0000003f000000` 双浮点按 goal.inactive 取：**有目标 +0.5 / 无目标 -0.5**
//!   （critters-extra §4"校正 1"声称 -1.0 系误读，RNE 字节证实 -0.5；reconstruction/firefly.rs
//!   草稿正确）。新 timer = `uniform*0.15+0.05`；proposed = pos + dir*timer；
//!   校验 `sqrt(z²+x²) < 65 || < 原半径`。
//! - set_firefly_velocities_towards_goals 0x142317040（115–136）：`dir = normalize(goal-pos)`；
//!   `multiplier = 1.0`（0x3f800000）；海拔下限：`pos.y <= 1.0` 时 `dir.y = max(dir.y, 0.25)`。
//! - fireflies_send_to_gpu 0x142316760：SSBO 上传（GPU 后置；CPU 侧实例数据 = core
//!   `FireflySsbo`，96 上限）。

use bevy_ecs::prelude::*;
use country_core::render::ssbo::StorageElement;
use country_core::resources::time::GameTime;
use country_core::systems::render_loop::resources::critters::FireflySsbo;
use glam::Vec3;
use perchance::PerchanceContext;

use crate::components::{
    CritterControlState, CritterGoal, CritterVelocityMultiplier, CritterVelocityTarget, Firefly,
    Transform, Velocity,
};
use crate::duck::CritterRng;
use crate::sample::uniform_sample_sphere;

/// update_firefly_goals（0x142316940）。
pub fn update_firefly_goals(
    time: Res<GameTime>,
    control: Res<CritterControlState>,
    rng: Res<CritterRng>,
    mut query: Query<(&Transform, &mut Firefly, &Velocity, &mut CritterGoal)>,
) {
    // control 门（O）：`*res + 4 != 1 → return`（不消费 RNG）。
    if !control.enabled {
        return;
    }
    let dt = time.delta;
    let mut rng_ctx = rng.0.lock().unwrap();
    for (transform, mut firefly, velocity, mut goal) in &mut query {
        let pos = transform.translation;
        // CritterGoal 桥接（country-core {position,goal_tag}，DIVERGE 见 shared.rs 头）。
        let mut goal_opt = if goal.goal_tag != 0 {
            Some(goal.position)
        } else {
            None
        };
        update_firefly_goal_core(
            &mut firefly.goal_timer,
            &mut goal_opt,
            pos,
            velocity.0,
            dt,
            &mut rng_ctx,
        );
        match goal_opt {
            Some(p) => {
                goal.goal_tag = 1;
                goal.position = p;
            }
            None => {
                goal.goal_tag = 0;
            }
        }
    }
}

/// 萤火虫目标刷新核心（与 reconstruction/firefly.rs::update_goal 一致；
/// 惯性 bias：有目标 +0.5 / 无目标 -0.5，RNE 0x142ccfc58）。
pub fn update_firefly_goal_core(
    timer: &mut f32,
    goal: &mut Option<Vec3>,
    position: Vec3,
    velocity: Vec3,
    dt: f32,
    rng: &mut PerchanceContext,
) {
    *timer -= dt;
    let arrived = if let Some(target) = *goal {
        let d = target - position;
        (d.y * d.y + d.x * d.x) + d.z * d.z < f32::from_bits(0x3c23d70b)
    } else {
        true
    };
    if !(*timer <= 0.0 || arrived) {
        return;
    }
    let inertia = if goal.is_none() { -0.5 } else { 0.5 };
    // 三次调用，顺序固定（共享资源非每萤）；新 timer = u*0.15+0.05。
    *timer = rng.uniform_f32() * 0.15 + 0.05;
    let sample = uniform_sample_sphere(rng);
    let biased = Vec3::new(
        velocity.x * inertia + sample.x,
        sample.y * 0.5,
        velocity.z * inertia + sample.z,
    );
    let direction = biased.normalize_or_zero();
    let proposed = Vec3::new(
        position.x + *timer * direction.x,
        position.y + *timer * direction.y,
        position.z + *timer * direction.z,
    );
    let radius = (proposed.z * proposed.z + proposed.x * proposed.x).sqrt();
    let old_radius = (position.z * position.z + position.x * position.x).sqrt();
    *goal = if radius < 65.0 || radius < old_radius {
        Some(proposed)
    } else {
        None
    };
}

/// set_firefly_velocities_towards_goals（0x142317040）。
/// Original PDB: `(Transform, Firefly, CritterGoal, VelocityTarget, Multiplier)`.
pub fn set_firefly_velocities_towards_goals(
    mut query: Query<(
        &Transform,
        &Firefly,
        &CritterGoal,
        &mut CritterVelocityTarget,
        &mut CritterVelocityMultiplier,
    )>,
) {
    for (transform, _firefly, goal, mut target, mut multiplier) in &mut query {
        let pos = transform.translation;
        // CritterGoal 桥接：goal_tag!=0 → goal.position；否则零方向。
        let d = if goal.goal_tag != 0 {
            goal.position - pos
        } else {
            Vec3::ZERO
        };
        target.direction = d.normalize_or_zero();
        multiplier.0 = 1.0;
        if pos.y < 1.0 {
            target.direction.y = target.direction.y.max(0.25);
        }
    }
}

/// fireflies_send_to_gpu（0x142316760）：CPU 侧把萤火虫实例投影为 `FireflySsbo`
/// （96 上限，O core render_loop 合同）；缓冲上传由宿主接线（GPU 后置）。
pub fn fireflies_send_to_gpu(
    mut out: ResMut<FireflyGpuData>,
    query: Query<(&Transform, &Firefly)>,
) {
    out.instances.clear();
    for (transform, _firefly) in &query {
        if out.instances.len() >= FireflySsbo::BUFFER_LEN {
            break;
        }
        out.instances
            .push(FireflySsbo::from_translation(transform.translation));
    }
}

/// 萤火虫 GPU 实例数据资源（宿主写入 Firefly Buffer；GPU 后置）。
#[derive(Resource, Clone, Debug, Default)]
pub struct FireflyGpuData {
    pub instances: Vec<FireflySsbo>,
}
