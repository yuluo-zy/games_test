//! sheep_behavior（B 主基线，fresh 反编译证据）。
//!
//! 证据：`evidence/restorer-eco-critters/decomp/142315050.c`（update_sheep_goals）、
//! `142315e40.c`（set_sheep_velocities_towards_goals）、`142316490.c`（update_sheep_audio）。
//! 常量经 0x142315050 反汇编 + read_memory 复核：
//! - 跟随距离阈值 0.2（0x3e4ccccd，XMM9）；`dist < 0.2 → timer -= 3*dt`（3.0=0x40400000）。
//! - 跟随分支随机：timer>=0 → `shared_timers=(u1*12+4, u2*3+1)`（12.0=0x41400000、4.0=0x40800000、
//!   3.0=0x40400000）；timer<0 → `behavior_state=0` + `u*7+1`（7.0=0x40e00000）。
//! - 共享目标保留：`goal.active && dist2>=0.16`（0.16=0x3e23d70b）；换目标标度
//!   `active ? 0.8 : 0.5`（0x14304ad18 表 [0]=0.8 [1]=0.5）；`shared_timers[1] = u*3+1`；
//!   偏移量 = `normalize(v) * shared_timers[1]`；半径门 `r_new<65.0 || r_new<r_old`（65.0=0x42820000）；
//!   demo 门 `|pos|>35.0`（0x420c0000）→ `v = x0y(disk2)*25.0 - pos`（25.0=0x41c80000）。
//! - 水面有效判定（loc==2 时）：`bilinear(new_pos)<=0 && (bilinear(pos)>0 || bilinear(new_pos)<=0)`
//!   ——第二采样在 pos（二进制冗余三次采样，阈值 0.0=0x142960690）。
//!
//! set_sheep_velocities_towards_goals：`goal.active && Sheep+0x30==1 && !being_petted` → dir=
//! normalize(goal-pos)；否则零方向；写 CritterVelocityTarget.dir + CritterVelocityMultiplier=1.0。

use bevy_ecs::prelude::*;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use country_core::systems::camera_rig::main_camera::MainCamera;
use glam::{Quat, Vec2, Vec3};

use crate::audio::AudioHandle;
use crate::components::{CritterVelocityTarget, Sheep, SheepMoveState, Transform};
use crate::duck::CritterRng;
use crate::sample::x0y;
use crate::sheep::sheep_terrain::{STATE_GROUNDED, SheepTerrainState};

/// Existing public path retained as an alias to the shared goal Component.
/// Rust field order does not reproduce native Option<Vec3> layout.
pub use country_core::resources::critters::CritterGoal as SheepGoal;

/// `utils::uniform_sample_disk`（0x140c90700）的二进制签名：RCX 打包两 f32
/// `(u, angle)`，内部不调 RNG。`r=sqrt(u)`、`a=angle*TAU` → `(r*cos a, r*sin a)`。
/// （本 crate `sample::uniform_sample_disk` 是 `(rng,u)` 变体；此处按原码两参数复刻 RNG 序列。）
fn disk_sample(u: f32, angle: f32) -> Vec2 {
    let r = u.sqrt();
    let a = angle * std::f32::consts::TAU;
    Vec2::new(r * a.cos(), r * a.sin())
}

/// `_DEV_OVERRIDE_IS_DEMO_MODE`（B 静态字节 0x143634d90；发行版 false，demo 分支不可达）。
fn demo_mode() -> bool {
    false
}

/// Original movement phase: timer updates happen before deciding whether to
/// retain the existing goal, start a state transition, or choose a new goal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GoalAction {
    Keep,
    Refresh,
    StartIdle,
    StartWalking,
}

fn advance_move_state(
    state: &mut SheepMoveState,
    dt: f32,
    goal: &SheepGoal,
    position: Vec3,
) -> GoalAction {
    match state {
        SheepMoveState::Idle { timeout_s } => {
            *timeout_s -= dt;
            if *timeout_s <= 0.0 {
                GoalAction::StartWalking
            } else {
                GoalAction::Keep
            }
        }
        SheepMoveState::Walking {
            timeout_s,
            goal_change_timeout,
            force_reach_goal,
        } => {
            *timeout_s -= dt;
            *goal_change_timeout -= dt;
            if *timeout_s <= 0.0 && !*force_reach_goal {
                return GoalAction::StartIdle;
            }
            let dx = goal.position.x - position.x;
            let dz = goal.position.z - position.z;
            let distance_squared = dx * dx + dz * dz;
            // Ordered comparisons preserve the original UCOMISS NaN branches.
            #[allow(clippy::neg_cmp_op_on_partial_ord)]
            let retain = (!(*goal_change_timeout <= 0.0) || *force_reach_goal)
                && goal.goal_tag & 1 != 0
                && !(distance_squared < f32::from_bits(0x3e23_d70b));
            if retain {
                GoalAction::Keep
            } else {
                GoalAction::Refresh
            }
        }
    }
}

/// B 14231596a..142315acd: rotate fixed -Z by the existing quaternion,
/// halve/project it to XZ, add the resolved disk sample, then normalize.
/// This reuses current glam instead of reproducing its dependency internals.
fn sheep_goal_direction(rotation: Quat, disk: Vec2, active_goal: bool) -> Vec3 {
    let forward = (rotation * Vec3::NEG_Z) * 0.5;
    let scale = if active_goal { 0.8 } else { 0.5 };
    Vec3::new(disk.x + scale * forward.x, 0.0, disk.y + scale * forward.z).normalize_or_zero()
}

/// B update_sheep_goals 0x142315050. Retain the existing current-system
/// architecture/RNG; typed movement is shared with snapshot capture/restore.
#[allow(clippy::too_many_arguments)]
pub fn update_sheep_goals(
    time: Res<GameTime>,
    camera: Res<MainCamera>,
    cursor: Res<CursorTerrainRaycast>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    rng: Res<CritterRng>,
    mut query: Query<(&Transform, ComponentWrite<Sheep>, &SheepTerrainState, &mut SheepGoal)>,
) {
    let dt = time.delta;
    let mut rng_ctx = rng.0.lock().unwrap();
    let w = (*water).as_ref();
    for (transform, mut sheep, terrain_state, mut goal) in &mut query {
        let pos = transform.translation;
        // The original keeps a pending state transition while processing the
        // previous movement state. Idle's inactive timer/force bytes are not data.
        let follow_transition = if sheep.behavior_state == 1 {
            let target = if camera.mode_byte() == 3 {
                camera.eye_position() + camera.eye_forward() * 2.0
            } else {
                cursor.position()
            };
            goal.goal_tag = 1;
            goal.position = target;
            sheep.goal_timer -= dt;
            if (cursor.position() - pos).length() < 0.2 {
                sheep.goal_timer -= 3.0 * dt;
            }
            // Original UCOMISS/JBE takes this branch on unordered input too.
            if sheep.goal_timer.is_nan() || sheep.goal_timer >= 0.0 {
                let timeout_s = rng_ctx.uniform_f32() * 12.0 + 4.0;
                let goal_change_timeout = rng_ctx.uniform_f32() * 3.0 + 1.0;
                Some(SheepMoveState::Walking {
                    timeout_s,
                    goal_change_timeout,
                    force_reach_goal: false,
                })
            } else {
                sheep.behavior_state = 0;
                Some(SheepMoveState::Idle {
                    timeout_s: rng_ctx.uniform_f32() * 7.0 + 1.0,
                })
            }
        } else {
            None
        };

        match advance_move_state(&mut sheep.move_state, dt, &goal, pos) {
            GoalAction::StartWalking => {
                let timeout_s = rng_ctx.uniform_f32() * 12.0 + 4.0;
                let goal_change_timeout = rng_ctx.uniform_f32() * 3.0 + 1.0;
                sheep.move_state = SheepMoveState::Walking {
                    timeout_s,
                    goal_change_timeout,
                    force_reach_goal: false,
                };
                continue;
            }
            GoalAction::StartIdle => {
                sheep.move_state = SheepMoveState::Idle {
                    timeout_s: rng_ctx.uniform_f32() * 7.0 + 1.0,
                };
                continue;
            }
            GoalAction::Keep => {}
            GoalAction::Refresh => {
                let step = rng_ctx.uniform_f32() * 3.0 + 1.0;
                if let SheepMoveState::Walking {
                    goal_change_timeout,
                    ..
                } = &mut sheep.move_state
                {
                    *goal_change_timeout = step;
                }
                let disk = disk_sample(rng_ctx.uniform_f32(), rng_ctx.uniform_f32());
                let mut direction =
                    sheep_goal_direction(transform.rotation, disk, goal.goal_tag != 0);
                if demo_mode() && pos.length() > 35.0 {
                    let disk2 = disk_sample(rng_ctx.uniform_f32(), rng_ctx.uniform_f32());
                    direction = (x0y(disk2) * 25.0 - pos).normalize();
                }
                let new_pos = pos + direction * step;
                let r_new = (new_pos.x * new_pos.x + new_pos.z * new_pos.z).sqrt();
                let r_old = (pos.x * pos.x + pos.z * pos.z).sqrt();
                let allowed = (r_new < 65.0 || r_new < r_old)
                    && (terrain_state.0 != STATE_GROUNDED || {
                        let s1 = w.bilinear_sample_at_world_xz(new_pos.x, new_pos.z, 130.0, 130.0);
                        s1 <= 0.0 && {
                            let s2 = w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
                            s2 > 0.0
                                || w.bilinear_sample_at_world_xz(new_pos.x, new_pos.z, 130.0, 130.0)
                                    <= 0.0
                        }
                    });
                goal.goal_tag = u32::from(allowed);
                if allowed {
                    goal.position = new_pos;
                }
                // Both accepted and rejected replacement clear the original +31.
                if let SheepMoveState::Walking {
                    force_reach_goal, ..
                } = &mut sheep.move_state
                {
                    *force_reach_goal = false;
                }
            }
        }
        if let Some(state) = follow_transition {
            sheep.move_state = state;
        }
    }
}

/// 羊跟随目标模式（Sheep+8==1 时的目标来源；O MainCamera::mode==3 → 相机）。
/// 宿主缝：二进制查询无此组件，跟随目标直接取 MainCamera/CursorTerrainRaycast。
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

/// 羊跟随目标组件（宿主注入；跟随目标来源）。二进制查询不含此组件（宿主缝）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepFollowTarget(pub SheepFollowTargetMode);

/// set_sheep_velocities_towards_goals（0x142315e40，源码 224–245）。
/// `goal.active && Sheep+0x30==1 && !being_petted` → dir=normalize(goal-pos)；否则零方向。
/// 写 CritterVelocityTarget.dir；CritterVelocityMultiplier=1.0（0x3f800000）。
pub fn set_sheep_velocities_towards_goals(
    mut query: Query<(
        &Transform,
        &Sheep,
        &SheepGoal,
        &mut CritterVelocityTarget,
        &mut crate::components::CritterVelocityMultiplier,
    )>,
) {
    for (transform, sheep, goal, mut target, mut multiplier) in &mut query {
        let pos = transform.translation;
        let d = if goal.goal_tag != 0
            && matches!(sheep.move_state, SheepMoveState::Walking { .. })
            && !sheep.being_petted()
        {
            goal.position - pos
        } else {
            Vec3::ZERO
        };
        target.direction = d.normalize_or_zero();
        multiplier.0 = 1.0;
    }
}

/// B update_sheep_audio142316490: set position, then audio+0x34 = dt + old.
pub fn update_sheep_audio(time: Res<GameTime>, mut query: Query<(&Transform, &mut SheepAudio)>) {
    for (transform, mut audio) in &mut query {
        audio.handle.set_position(transform.translation);
        audio.elapsed = time.delta + audio.elapsed;
    }
}

/// Original 56B audio record's semantic fields; the host owns AudioInstance.
#[derive(Component, Debug, Default)]
pub struct SheepAudio {
    pub handle: AudioHandle,
    /// B +0x30: same pitch used for startup's Pitch parameter.
    pub startup_pitch: f32,
    /// B +0x34, initialized zero and advanced by update_sheep_audio.
    pub elapsed: f32,
}

#[cfg(test)]
mod original_movement_phase_tests {
    use super::*;
    fn f(bits: u32) -> f32 {
        f32::from_bits(bits)
    }
    #[test]
    fn movement_phase_matches_original_b_instruction_fixtures() {
        // Original resolved phase case0.
        let mut state = SheepMoveState::Idle {
            timeout_s: f(1073741824),
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Keep
        );
        assert!(
            matches!(state, SheepMoveState::Idle { timeout_s } if timeout_s.to_bits() == 1071644672)
        );
        // Original resolved phase case1.
        let mut state = SheepMoveState::Idle {
            timeout_s: f(1036831949),
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::StartWalking
        );
        assert!(
            matches!(state, SheepMoveState::Idle { timeout_s } if timeout_s.to_bits() == 3189348762)
        );
        // Original resolved phase case2.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1073741824),
            goal_change_timeout: f(1073741824),
            force_reach_goal: false,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Keep
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 1071644672);
                assert_eq!(goal_change_timeout.to_bits(), 1071644672);
                assert_eq!(force_reach_goal, false);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case3.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1073741824),
            goal_change_timeout: f(1073741824),
            force_reach_goal: true,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Keep
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 1071644672);
                assert_eq!(goal_change_timeout.to_bits(), 1071644672);
                assert_eq!(force_reach_goal, true);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case4.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1073741824),
            goal_change_timeout: f(1036831949),
            force_reach_goal: false,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Refresh
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 1071644672);
                assert_eq!(goal_change_timeout.to_bits(), 3189348762);
                assert_eq!(force_reach_goal, false);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case5.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1073741824),
            goal_change_timeout: f(1036831949),
            force_reach_goal: true,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Keep
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 1071644672);
                assert_eq!(goal_change_timeout.to_bits(), 3189348762);
                assert_eq!(force_reach_goal, true);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case6.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1036831949),
            goal_change_timeout: f(1073741824),
            force_reach_goal: false,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::StartIdle
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 3189348762);
                assert_eq!(goal_change_timeout.to_bits(), 1071644672);
                assert_eq!(force_reach_goal, false);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case7.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1036831949),
            goal_change_timeout: f(1073741824),
            force_reach_goal: true,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Keep
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 3189348762);
                assert_eq!(goal_change_timeout.to_bits(), 1071644672);
                assert_eq!(force_reach_goal, true);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case8.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1036831949),
            goal_change_timeout: f(1036831949),
            force_reach_goal: false,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::StartIdle
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 3189348762);
                assert_eq!(goal_change_timeout.to_bits(), 3189348762);
                assert_eq!(force_reach_goal, false);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case9.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1036831949),
            goal_change_timeout: f(1036831949),
            force_reach_goal: true,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Keep
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 3189348762);
                assert_eq!(goal_change_timeout.to_bits(), 3189348762);
                assert_eq!(force_reach_goal, true);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case10.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1073741824),
            goal_change_timeout: f(1036831949),
            force_reach_goal: true,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(0), f(1120403456), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Refresh
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 1071644672);
                assert_eq!(goal_change_timeout.to_bits(), 3189348762);
                assert_eq!(force_reach_goal, true);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case11.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1073741824),
            goal_change_timeout: f(1073741824),
            force_reach_goal: true,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 0,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Refresh
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 1071644672);
                assert_eq!(goal_change_timeout.to_bits(), 1071644672);
                assert_eq!(force_reach_goal, true);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case12.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1073741824),
            goal_change_timeout: f(1073741824),
            force_reach_goal: true,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1036831949), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Refresh
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 1071644672);
                assert_eq!(goal_change_timeout.to_bits(), 1071644672);
                assert_eq!(force_reach_goal, true);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case13.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1036831949),
            goal_change_timeout: f(1073741824),
            force_reach_goal: true,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1036831949), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Refresh
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 3189348762);
                assert_eq!(goal_change_timeout.to_bits(), 1071644672);
                assert_eq!(force_reach_goal, true);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case14.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(1073741824),
            goal_change_timeout: f(2143289344),
            force_reach_goal: false,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Keep
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 1071644672);
                assert_eq!(goal_change_timeout.to_bits(), 2143289344);
                assert_eq!(force_reach_goal, false);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
        // Original resolved phase case15.
        let mut state = SheepMoveState::Walking {
            timeout_s: f(2143289344),
            goal_change_timeout: f(1073741824),
            force_reach_goal: false,
        };
        let goal = SheepGoal {
            position: Vec3::new(f(1073741824), f(0), f(0)),
            goal_tag: 1,
        };
        assert_eq!(
            advance_move_state(
                &mut state,
                f(1048576000),
                &goal,
                Vec3::new(f(0), f(0), f(0))
            ),
            GoalAction::Keep
        );
        match state {
            SheepMoveState::Walking {
                timeout_s,
                goal_change_timeout,
                force_reach_goal,
            } => {
                assert_eq!(timeout_s.to_bits(), 2143289344);
                assert_eq!(goal_change_timeout.to_bits(), 1071644672);
                assert_eq!(force_reach_goal, false);
            }
            _ => panic!("phase changed the movement discriminant before its transition"),
        }
    }
    #[test]
    fn projected_goal_direction_matches_original_b_instruction_fixtures() {
        // Existing glam implementation is reused; native fixture case0.
        let actual = sheep_goal_direction(
            Quat::from_xyzw(f(0), f(0), f(0), f(1065353216)),
            Vec2::new(f(1045220557), f(1050253722)),
            true,
        );
        let expected = Vec3::new(f(1063581999), f(0), f(3202677038));
        assert!(
            (actual - expected).abs().max_element() < 2.0e-6,
            "case0: {actual:?} vs {expected:?}"
        );
        // Existing glam implementation is reused; native fixture case1.
        let actual = sheep_goal_direction(
            Quat::from_xyzw(f(0), f(0), f(0), f(1065353216)),
            Vec2::new(f(3201092813), f(1060320051)),
            false,
        );
        let expected = Vec3::new(f(3207205823), f(0), f(1061115447));
        assert!(
            (actual - expected).abs().max_element() < 2.0e-6,
            "case1: {actual:?} vs {expected:?}"
        );
        // Existing glam implementation is reused; native fixture case2.
        let actual = sheep_goal_direction(
            Quat::from_xyzw(f(0), f(1060439283), f(0), f(1060439283)),
            Vec2::new(f(1045220557), f(1050253722)),
            true,
        );
        let expected = Vec3::new(f(3205365972), f(0), f(1062535489));
        assert!(
            (actual - expected).abs().max_element() < 2.0e-6,
            "case2: {actual:?} vs {expected:?}"
        );
        // Existing glam implementation is reused; native fixture case3.
        let actual = sheep_goal_direction(
            Quat::from_xyzw(f(1060439283), f(0), f(0), f(1060439283)),
            Vec2::new(f(1045220557), f(1050253722)),
            true,
        );
        let expected = Vec3::new(f(1057882325), f(0), f(1062535488));
        assert!(
            (actual - expected).abs().max_element() < 2.0e-6,
            "case3: {actual:?} vs {expected:?}"
        );
        // Existing glam implementation is reused; native fixture case4.
        let actual = sheep_goal_direction(
            Quat::from_xyzw(f(0), f(0), f(1060439283), f(1060439283)),
            Vec2::new(f(3201092813), f(1060320051)),
            false,
        );
        let expected = Vec3::new(f(3207205823), f(0), f(1061115447));
        assert!(
            (actual - expected).abs().max_element() < 2.0e-6,
            "case4: {actual:?} vs {expected:?}"
        );
        // Existing glam implementation is reused; native fixture case5.
        let actual = sheep_goal_direction(
            Quat::from_xyzw(f(0), f(0), f(0), f(1065353216)),
            Vec2::new(f(0), f(1053609165)),
            true,
        );
        let expected = Vec3::new(f(0), f(0), f(0));
        assert!(
            (actual - expected).abs().max_element() < 2.0e-6,
            "case5: {actual:?} vs {expected:?}"
        );
        // Existing glam implementation is reused; native fixture case6.
        let actual = sheep_goal_direction(
            Quat::from_xyzw(f(0), f(0), f(0), f(1065353216)),
            Vec2::new(f(0), f(1048576000)),
            false,
        );
        let expected = Vec3::new(f(0), f(0), f(0));
        assert!(
            (actual - expected).abs().max_element() < 2.0e-6,
            "case6: {actual:?} vs {expected:?}"
        );
        // Existing glam implementation is reused; native fixture case7.
        let actual = sheep_goal_direction(
            Quat::from_xyzw(f(0), f(0), f(0), f(1065353216)),
            Vec2::new(f(0), f(0)),
            true,
        );
        let expected = Vec3::new(f(0), f(0), f(3212836864));
        assert!(
            (actual - expected).abs().max_element() < 2.0e-6,
            "case7: {actual:?} vs {expected:?}"
        );
    }
}

use country_core::render::extract::ComponentWrite;
