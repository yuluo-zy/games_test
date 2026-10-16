//! sheep_behavior（B 主基线，反汇编级证据）。
//!
//! 证据（critters-behavior-analysis.md §4 + skeleton PDB）：
//! - update_sheep_goals 0x142315050（源码 26–221）：查询
//!   `(Transform(0x30), Sheep(0x68), CritterGoal(0x10), 2×4B)`。
//!   Sheep 字段：+8 行为状态 u32（==1 跟随）；+0xc 目标计时；+0x30 字节；+0x34 8B 双计时。
//!   - state==1（跟随）：`MainCamera::mode()==3` → eye_position + eye_forward*2 为目标；
//!     否则 CursorTerrainRaycast::position。写 CritterGoal={active,pos}。
//!     `timer@+0xc -= dt`；距目标 `<= fVar13` → `timer -= 3*dt`；`timer>=0` → 2×uniform_f32；
//!     `timer<0` → state=0 + 1×uniform_f32。
//!   - 共享目标刷新（L125–148）：`Sheep+0x30` 字节 → `+0x34` 两半 -= dt；全 >0 →
//!     goal active 且 dist2>=0.16 → 保留；否则 2×uniform_f32 + uniform_sample_disk → 新目标；
//!     bilinear 水面采样 → 无效回退。
//! - set_sheep_velocities_towards_goals 0x142315e40（224–245）：dir = normalize(goal-pos)。
//! - update_sheep_audio 0x142316490（248–253）：逐羊 set_position。

use bevy_ecs::prelude::*;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use country_core::systems::camera_rig::main_camera::MainCamera;
use glam::Vec3;
use perchance::PerchanceContext;

use crate::audio::AudioHandle;
use crate::components::{CritterGoal, CritterVelocityTarget, Sheep, Transform, Velocity};
use crate::duck::CritterRng;
use crate::sample::{uniform_sample_disk, x0y};

/// update_sheep_goals（0x142315050）。
#[allow(clippy::too_many_arguments)]
pub fn update_sheep_goals(
    time: Res<GameTime>,
    camera: Res<MainCamera>,
    cursor: Res<CursorTerrainRaycast>,
    water: Res<WorldRaster<512>>,
    rng: Res<CritterRng>,
    mut query: Query<(&Transform, &mut Sheep, &mut CritterGoal, &SheepFollowTarget)>,
) {
    let dt = time.delta;
    let mut rng_ctx = rng.0.lock().unwrap();
    let w = (*water).as_ref();
    for (transform, mut sheep, mut goal, follow) in &mut query {
        let pos = transform.translation;

        // state==1（跟随）分支（O L60–81）。
        if sheep.behavior_state == 1 {
            let target = match follow.0 {
                SheepFollowTargetMode::Camera => {
                    let eye = camera.eye_position();
                    let fwd = camera.eye_forward();
                    eye + fwd * 2.0
                }
                SheepFollowTargetMode::Cursor => cursor.position(),
            };
            goal.0 = Some(target);
            sheep.goal_timer -= dt;
            let d = (target - pos).length();
            if d <= f32::from_bits(0x3f000000) {
                sheep.goal_timer -= 3.0 * dt;
            }
            if sheep.goal_timer >= 0.0 {
                let _u1 = rng_ctx.uniform_f32();
                let _u2 = rng_ctx.uniform_f32();
            } else {
                sheep.behavior_state = 0;
                let _u = rng_ctx.uniform_f32();
            }
        }

        // 共享目标刷新（O L125–148）。
        if sheep.flag_30 != 0 {
            sheep.shared_timers[0] -= dt;
            sheep.shared_timers[1] -= dt;
            let keep = sheep.shared_timers[0] > 0.0
                && sheep.shared_timers[1] > 0.0
                && goal.0.map_or(false, |g| (g - pos).length_squared() >= 0.16);
            if !keep {
                let u = rng_ctx.uniform_f32();
                let disk = uniform_sample_disk(&mut rng_ctx, u);
                let new_pos = pos + x0y(disk) * 2.0;
                // 水面校验（bilinear <= 0 → 无效回退）。
                let s = w.bilinear_sample_at_world_xz(new_pos.x, new_pos.z, 130.0, 130.0);
                if s > 0.0 {
                    goal.0 = Some(new_pos);
                } else {
                    goal.0 = None;
                }
                sheep.shared_timers = [1.0, 1.0];
            }
        }
    }
}

/// 羊跟随目标模式（Sheep+8==1 时的目标来源；O MainCamera::mode==3 → 相机）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheepFollowTargetMode {
    Camera,
    Cursor,
}

impl Default for SheepFollowTargetMode {
    fn default() -> Self {
        Self::Cursor
    }
}

/// 羊跟随目标组件（宿主注入；跟随目标来源）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepFollowTarget(pub SheepFollowTargetMode);

/// set_sheep_velocities_towards_goals（0x142315e40）。
pub fn set_sheep_velocities_towards_goals(
    mut query: Query<(&Transform, &Sheep, &CritterGoal, &mut CritterVelocityTarget, &mut Velocity)>,
) {
    for (transform, _sheep, goal, mut target, mut velocity) in &mut query {
        let pos = transform.translation;
        let d = goal.0.unwrap_or(pos) - pos;
        target.direction = d.normalize_or_zero();
        velocity.0 = target.direction;
    }
}

/// update_sheep_audio（0x142316490）：逐羊 set_position（宿主缝）。
pub fn update_sheep_audio(mut query: Query<(&Transform, &mut SheepAudio)>) {
    for (transform, mut audio) in &mut query {
        audio.handle.set_position(transform.translation);
    }
}

/// 羊音频（宿主缝）。
#[derive(Component, Debug, Default)]
pub struct SheepAudio {
    pub handle: AudioHandle,
}
