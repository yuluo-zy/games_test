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
use glam::{Vec2, Vec3};

use crate::audio::AudioHandle;
use crate::components::{CritterVelocityTarget, Sheep, Transform};
use crate::duck::CritterRng;
use crate::sample::x0y;
use crate::sheep::sheep_terrain::{SheepTerrainState, STATE_GROUNDED};

/// 羊目标组件（二进制 CritterGoal 布局：active u32@+0、pos Vec3@+4；stride 0x10）。
/// 注：country_core::resources::critters::CritterGoal 已被并发 agent 改为
/// `{position@+0, goal_tag@+0xc}`（与 fresh 反编译 `*goal=active; +4=pos` 不符）；
/// 本重建以本地组件保持二进制布局（集成 owner 裁决是否对齐共享类型）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepGoal {
    /// +0：active（bit0）。
    pub active: u32,
    /// +4：目标位置。
    pub pos: Vec3,
}

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

/// update_sheep_goals（0x142315050，源码 26–221）。
///
/// 逐羊（O，控制流与常量均自 fresh 反编译）：
/// - `behavior_state==1`（跟随）：mode==3 → `eye+fwd*2`；否则 `CursorTerrainRaycast::position`；
///   写 CritterGoal={active=1,pos}；`timer@+0xc -= dt`；**再次**取光标位，`dist(cur,pos)<0.2`
///   → `timer -= 3*dt`；`timer>=0` → 2×uniform_f32 → `t0=u1*12+4、t1=u2*3+1`、sub=1；
///   `timer<0` → `behavior_state=0`、1×uniform_f32 → `t0=u*7+1`、sub=0。
/// - 共享目标刷新（`flag_30!=0`）：两半 `-=dt`；`两半>0 && goal.active && dist2>=0.16` → 保留；
///   否则换目标：`scale=active?0.8:0.5`、`shared_timers[1]=u*3+1`、disk 采样 → `rot=quat*sample`、
///   `v=(sample.x+scale*rot.x, 0, sample.y+scale*rot.z)`（demo 门 >35 时 `v=x0y(disk2)*25-pos`）、
///   `new_pos = pos + normalize(v)*shared_timers[1]`；`r_new<65||r_new<r_old` 且（loc!=2 或水面<=0）
///   → 写 goal；否则 goal=inactive。
/// - 结尾写（`behavior_state` 低字节 !=0）：`flag_30=sub、flag_31=0、shared_timers=(t0,t1)`。
///
/// 已知分歧：`flag_31@+0x31` 本函数恒写 0（读分支死路径），未落 Sheep 字段；XMM7/XMM8 寄存器残留
/// 按"沿用本羊当前 shared_timers"近似（非跟随非刷新羊的结尾写值无法从反编译确定性还原）。
/// 2×4B 查询环境列写（bevy QueryIter 胶水）不建模。
#[allow(clippy::too_many_arguments)]
pub fn update_sheep_goals(
    time: Res<GameTime>,
    camera: Res<MainCamera>,
    cursor: Res<CursorTerrainRaycast>,
    water: Res<WorldRaster<512>>,
    rng: Res<CritterRng>,
    mut query: Query<(
        &Transform,
        &mut Sheep,
        &SheepTerrainState,
        &mut SheepGoal,
    )>,
) {
    let dt = time.delta;
    let mut rng_ctx = rng.0.lock().unwrap();
    let w = (*water).as_ref();
    for (transform, mut sheep, terrain_state, mut goal) in &mut query {
        let pos = transform.translation;
        let behavior = sheep.behavior_state;

        // uVar15 默认 2；t0/t1 为 XMM7/XMM8 结尾写值（残留近似）。
        let mut sub_state: u8 = 2;
        let mut t0 = sheep.shared_timers[0];
        let mut t1 = sheep.shared_timers[1];

        if behavior == 1 {
            // 跟随分支（O L187–229）。
            let target = if camera.mode_byte() == 3 {
                camera.eye_position() + camera.eye_forward() * 2.0
            } else {
                cursor.position()
            };
            goal.active = 1;
            goal.pos = target;
            sheep.goal_timer -= dt;
            // O：目标写入后再取一次光标位，到达判定用当前光标（非存下目标）。
            let cur = cursor.position();
            if (cur - pos).length() < 0.2 {
                sheep.goal_timer -= 3.0 * dt;
            }
            let timer = sheep.goal_timer;
            if timer >= 0.0 {
                let u1 = rng_ctx.uniform_f32();
                let u2 = rng_ctx.uniform_f32();
                t0 = u1 * 12.0 + 4.0;
                t1 = u2 * 3.0 + 1.0;
                sub_state = 1;
            } else {
                sheep.behavior_state = 0;
                let u = rng_ctx.uniform_f32();
                t0 = u * 7.0 + 1.0;
                // t1 = XMM8 残留（此处未写，沿用近似值）。
                sub_state = 0;
            }
        }

        // 共享目标刷新（O L234–357）。
        if sheep.flag_30 != 0 {
            sheep.shared_timers[0] -= dt;
            sheep.shared_timers[1] -= dt;
            let keep = sheep.shared_timers[0] > 0.0
                && sheep.shared_timers[1] > 0.0
                && goal.active != 0
                && (goal.pos - pos).length_squared() >= 0.16;
            if !keep {
                // flag_31 恒 0 → 其"保留检查"分支为死路径（O L249/L251）。
                let scale = if goal.active != 0 { 0.8 } else { 0.5 };
                let u = rng_ctx.uniform_f32();
                sheep.shared_timers[1] = u * 3.0 + 1.0;
                let disk = disk_sample(rng_ctx.uniform_f32(), rng_ctx.uniform_f32());
                let sample = x0y(disk);
                let rot = transform.rotation * sample;
                let mut v = Vec3::new(
                    sample.x + scale * rot.x,
                    0.0,
                    sample.y + scale * rot.z,
                );
                if demo_mode() && pos.length() > 35.0 {
                    let disk2 = disk_sample(rng_ctx.uniform_f32(), rng_ctx.uniform_f32());
                    v = x0y(disk2) * 25.0 - pos;
                }
                let dir = v.normalize_or_zero();
                let new_pos = pos + dir * sheep.shared_timers[1];
                let r_new = (new_pos.x * new_pos.x + new_pos.z * new_pos.z).sqrt();
                let r_old = (pos.x * pos.x + pos.z * pos.z).sqrt();
                if r_new < 65.0 || r_new < r_old {
                    // O：loc==2 时水面判定（三次 bilinear：new_pos / pos / new_pos，阈值 0.0）。
                    let s1 = w.bilinear_sample_at_world_xz(new_pos.x, new_pos.z, 130.0, 130.0);
                    let s2 = w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
                    let s3 = w.bilinear_sample_at_world_xz(new_pos.x, new_pos.z, 130.0, 130.0);
                    if terrain_state.state != STATE_GROUNDED || (s1 <= 0.0 && (s2 > 0.0 || s3 <= 0.0))
                    {
                        goal.active = 1;
                        goal.pos = new_pos;
                    } else {
                        goal.active = 0;
                    }
                } else {
                    goal.active = 0;
                }
            }
        } else {
            // 单计时路径（O L358–367）：仅低半 -= dt；<=0 → t0/u12+4、t1/u3+1、sub=1。
            sheep.shared_timers[0] -= dt;
            if sheep.shared_timers[0] <= 0.0 {
                let u1 = rng_ctx.uniform_f32();
                let u2 = rng_ctx.uniform_f32();
                t0 = u1 * 12.0 + 4.0;
                t1 = u2 * 3.0 + 1.0;
                sub_state = 1;
            }
        }

        // 结尾写（O L368–376）：behavior_state 低字节 !=0。
        // 原码同时写 `flag_31@+0x31 = 0`（恒 0，未落 Sheep 字段——见模块头分歧注释）。
        if (behavior & 0xff) != 0 {
            sheep.flag_30 = sub_state;
            sheep.shared_timers[0] = t0;
            sheep.shared_timers[1] = t1;
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
        let d = if goal.active != 0 && sheep.flag_30 == 1 && !sheep.being_petted() {
            goal.pos - pos
        } else {
            Vec3::ZERO
        };
        target.direction = d.normalize_or_zero();
        multiplier.0 = 1.0;
    }
}

/// update_sheep_audio（0x142316490，源码 248–253）：逐羊 `audio_resources::set_position`。
/// 原码每羊同时 `[0x38-stride 音频记录]+0x34 += dt`（音频内部计时，本重建未落组件，DIVERGE）。
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
