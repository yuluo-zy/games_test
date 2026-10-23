//! butterfly 行为链（B 主基线，fresh 反编译证据）。
//!
//! 证据：`evidence/restorer-eco-critters/decomp/*.c`（0x141d84d40/0x141d86060/0x141d867b0/
//! 0x141d86eb0/0x141d87140）+ 同址 fresh 反汇编（141d86060 86600-866e5 区）+ 常量读
//! （0x142925870=0.5、0x142925904=1.0、0x1429258d0=1.5、0x1429258d8=0.3、0x142925984=0.1、
//! 0x1429cd068=0.2、0x1429cd078=3.0、0x142934694=4.0、0x142a7fca4=0.45、0x142f67d30=1.76、
//! 0x142934604=8.0、0x142a83814=65.0、0x142f67d00=0.01、0x142f67d10=(1.0,0x3f5a2e8c)、
//! 0x142f67d20=(0.5,0xbe91745d)）。
//!
//! 重要修正（相对旧近似实现）：
//! - update_butterfly_goals：`d = timer * 1.76`（旧 1.77 错；0x3fe147ae）。方向 = 球面采样加权
//!   + x0z(velocity) 动量项；竖向偏置 select 按 `原 goal.active`（0x3f5a2e8c/0xbe91745d），
//!   0.5/1.0 权重按 `state==1`；查询含 `&Velocity`（PDB 候选签名）。
//! - set_butterfly_velocities_towards_goals：查询 `(Transform, &Butterfly, &CritterGoal,
//!   &mut CritterVelocityTarget, &mut CritterVelocityMultiplier)`（旧含 &Sheep 错）；
//!   `dir = normalize((dx, dz, dz)) * 1.76`（机器码把 transform.z 差分当竖直分量，长度
//!   = sqrt(dx²+dz²+dz²)）；`*Multiplier = Butterfly.phase + 1.0`（旧 `+= 1.0` 错）。
//! - update_butterfly_states：查询 `(Transform, &Velocity, &mut Butterfly,
//!   &mut ButterflyAudio, &CritterGoal)` + 独立 `(Transform, &Sheep)` 避羊查询；距离门
//!   mode==3 → 距 eye 阈值 3.0（实际距离），否则距光标阈值 4.0（平方）；phase = max(0,
//!   phase - 0.5*dt)，near||sheep_near → phase=1.0；dwell 状态机跳表 DAT_142f68008。
//!
//! 2026-10-17 fresh 反汇编关闭项：DAT_142f68008 的 state0/1/2/3 handler 已逐指令解码并
//! 落库（`Butterfly.state_t` = 记录 +4；continuation `phase_ref = fmod(phase_ref+flag*dt,1)`）；
//! 残余 U：ButterflyAudio 每帧 u32 写值（query 派生值）、goals/set_velocities 的 per-entity
//! 4B 写（列 +0x28 视图，值=query 派生值）、state4-7（跳表项指向其它函数的动画 handler）。
//!
//! 跳表解码（fresh 2026-10-17）：DAT_142f68008 8 项（0..3 状态 handler + 4..7 指向
//! `set_butterfly_velocities` 内的动画 handler）；set_butterfly_velocities 的
//! DAT_142f68018 4 项 = 清零/衰减+向下/钳制/固定上。

use bevy_ecs::prelude::*;
use country_core::render::ssbo::StorageElement;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::terrain_rock_mask;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::render_loop::resources::critters::ButterflySsbo;
use glam::{Vec2, Vec3};
use perchance::PerchanceContext;

use crate::audio::AudioHandle;
use crate::components::{
    Butterfly, CritterGoal, CritterVelocityMultiplier, CritterVelocityTarget, Sheep, Transform,
    Velocity,
};
use crate::duck::CritterRng;
use crate::sample::uniform_sample_sphere;

/// ButterflyAudio（core `resources::critters::ButterflyAudio` 的消费侧：set_position
/// 0x140f85ba0 = 5B thunk → 0x140a57da0 共享实现）。stride-4 槽；`update_butterfly_states`
/// 另有 per-frame u32 写（query 派生值，U，见 step_state 注释）。
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

/// 距离门输入代理（旧近似资源；lib.rs 仍 init_resource。恢复后 update_butterfly_states
/// 直接消费 `Res<MainCamera>` + `Res<CursorTerrainRaycast>`，本资源已不被系统使用）。
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

/// update_butterfly_states（0x141d84d40）。
///
/// 查询（PDB 候选 + fresh 反编译 QueryIterationCursor）：`(Transform, &Velocity,
/// &mut Butterfly, &mut ButterflyAudio, &CritterGoal)`；另收集 `(Transform, &Sheep)`
/// 的 Vec2 位置用于避羊。资源：`ResMut<CritterRng>`、`Res<CursorTerrainRaycast>`、
/// `Res<GameTime>`、`Res<MainCamera>`（另 PDB 记录 `Res<AssetSsboLibrary>`、
/// `Res<WaterRaster>` 前导胶水，无业务副作用，未建模）。
///
/// 每蝴蝶（O 0x141d85655）：
/// - 距离门：相机 mode==3 → `dist(eye, pos) < 3.0`（0x141d856e4 SQRTSS + 0x40400000）；
///   否则 `dist²(cursor, pos) < 4.0`（0x141d8570d + 0x40800000，平方距离）。
/// - 避羊：任一羊 `dist²(pos, sheep) < 4.0` → 视为近（0x141d85759 UCOMISS XMM13）。
/// - `phase = max(0, phase - 0.5*dt)`；`near || sheep_near` → `phase = 1.0`
///   （0x141d85765-857a9）。
/// - dwell 状态机 + 跳表派发（见 step_state）。
#[allow(clippy::too_many_arguments)]
pub fn update_butterfly_states(
    time: Res<GameTime>,
    camera: Res<MainCamera>,
    cursor: Res<CursorTerrainRaycast>,
    rng: Res<CritterRng>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    buffers: Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    mut query: Query<(
        &Transform,
        &Velocity,
        &mut Butterfly,
        &mut ButterflyAudio,
        &CritterGoal,
    )>,
    sheep_query: Query<(&Transform, &Sheep)>,
    readbacks: Option<Res<country_core::render::gpu_readback::GpuReadbacks>>,
) {
    let dt = time.delta;
    // Canonical rock-mask readback (same owner as shared::move_critters), from
    // the render thread's latest publication rather than the in-flight mapping.
    buffers
        .get::<terrain_rock_mask::TerrainRockMaskSsbo>()
        .expect("update_butterfly_states requires canonical mapped TerrainRockMaskSsbo");
    let rock_mask = terrain_rock_mask::TerrainRockMaskSsbo::published(readbacks.as_deref());
    let mut rng_ctx = rng.0.lock().unwrap();
    // O 0x141d851cb：`from_iter<Vec2, Map<QueryIter<(Transform, Sheep)>>>` 收集避羊位。
    let sheep_positions: Vec<Vec2> = sheep_query
        .iter()
        .map(|(t, _)| Vec2::new(t.translation.x, t.translation.z))
        .collect();
    let eye_mode = camera.mode_byte() == 3;
    let eye = camera.eye_position();
    let cursor_pos = cursor.position();
    for (transform, velocity, mut butterfly, _audio, goal) in &mut query {
        let pos = transform.translation;
        // 2D（x,z）距离（O 0x141d856d3/85702：SUBPS/MULPS 只含 x,z 分量）。
        // 近门 = 二进制 `ucomiss 阈值,dist; SETA`：`dist < 阈值`（unordered → SETA=0
        // → 不近）。mode3 阈值 3.0；否则平方距离阈值 4.0。
        let near = if eye_mode {
            let dx = eye.x - pos.x;
            let dz = eye.z - pos.z;
            (dx * dx + dz * dz).sqrt() < 3.0
        } else {
            let dx = cursor_pos.x - pos.x;
            let dz = cursor_pos.z - pos.z;
            dx * dx + dz * dz < 4.0
        };
        // 避羊：任一羊 `dist² < 4.0`（0x141d85759 `ucomiss 4.0,d²; jbe 继续` 在
        // d²>=4 或 unordered 时继续，仅有序小于 4.0 视为找到）。
        let sheep_near = sheep_positions.iter().any(|s| {
            let dx = pos.x - s.x;
            let dz = pos.z - s.y;
            dx * dx + dz * dz < 4.0
        });
        // state handler 消费：goal.active + 高度（state1 0.6 阈值 / state3 1.5 阈值）。
        let goal_active = goal.goal_tag != 0;
        step_state(
            &mut butterfly,
            velocity.0,
            dt,
            near,
            sheep_near,
            &mut rng_ctx,
            goal_active,
            pos,
            &water,
            rock_mask,
        );
    }
}

/// 状态机计时核心（O 0x141d857b4-0x141d852df）。跳表 DAT_142f68008 按 `Butterfly.state`
/// 索引派发；handler 经 `butterfly_state_dispatch` 实现（2026-10-17 fresh 反汇编）。
///
/// 控制流（O）：
/// - `dwell <= 0`：`dwell += dt`；仍 `<= 0` → 派发（t=新 dwell）；否则
///   `dwell = uniform*0.3 + 0.2` → 派发（t=新 dwell）。
/// - `dwell > 0 && velocity.y > 0` → 派发（t=0）。
/// - `dwell > 0 && |0.5 - phase_ref| < 0.2 || 0.45 <= |0.5 - phase_ref|` → 派发（t=0）。
/// - 否则（|0.5-phase_ref| ∈ [0.2, 0.45)）：`dwell -= dt`；仍 `> 0` → 派发（t=flag）；
///   否则 `dwell = -(uniform*1.5 + 0.5)` → 派发（t=新 dwell）。
/// 派发后统一写 state / `state_t` / `phase_ref = fmod(phase_ref + flag*dt, 1.0)`。
pub fn step_state(
    st: &mut Butterfly,
    velocity: Vec3,
    dt: f32,
    near: bool,
    sheep_near: bool,
    rng: &mut PerchanceContext,
    goal_active: bool,
    pos: Vec3,
    water: &WorldRaster<512>,
    rock_mask: &[terrain_rock_mask::TerrainRockMaskSsbo],
) {
    // 相位推进（O 0x141d85765）：phase = max(0, phase - 0.5*dt)；近/避羊 → 1.0。
    st.phase = (st.phase - 0.5 * dt).max(0.0);
    if near || sheep_near {
        st.phase = 1.0;
    }
    // 派发 flag（fresh 0x141d857b4-0x141d85981）：`phase>0 ? 4.0 : baseline`；
    // baseline 逐路：**3.0**（dwell<=0 两路 / velocity.y>0 / 相位门外）或 **1.0**
    // （|0.5-phase_ref|∈[0.2,0.45) 两路）。常量实读：1.0@0x142925904、
    // 3.0@0x1429cd078、4.0@0x142934694。
    let phase_pos = st.phase > 0.0;
    let flag_hi = if phase_pos { 4.0 } else { 3.0 };
    let flag_lo = if phase_pos { 4.0 } else { 1.0 };
    // 各派发点 XMM0 的 `t` 初值：dwell<=0 两路/随机重置路 = 新 dwell；
    // velocity.y>0 与相位门外 = 0.0；相位门内 dwell-dt>0 路 = `mask&4`（0.0/4.0）。
    let dwell = st.dwell;
    let (t_in, flag_in) = if dwell <= 0.0 {
        st.dwell = dwell + dt;
        if st.dwell <= 0.0 {
            (st.dwell, flag_hi)
        } else {
            // O 0x141d858f8：dwell = uniform*0.3 + 0.2。
            st.dwell = rng.uniform_f32() * 0.3 + 0.2;
            (st.dwell, flag_hi)
        }
    } else if velocity.y > 0.0 {
        (0.0, flag_hi)
    } else {
        let phase_off = (0.5 - st.phase_ref).abs();
        if phase_off < 0.2 || 0.45 <= phase_off {
            (0.0, flag_hi)
        } else {
            // |0.5 - phase_ref| ∈ [0.2, 0.45)：dwell -= dt。
            st.dwell = dwell - dt;
            if st.dwell > 0.0 {
                (if phase_pos { 4.0 } else { 0.0 }, flag_lo)
            } else {
                // O 0x141d85883：dwell = -(uniform*1.5 + 0.5)。
                st.dwell = -(rng.uniform_f32() * 1.5 + 0.5);
                (st.dwell, flag_lo)
            }
        }
    };
    let (next_state, t_out, flag_out) = butterfly_state_dispatch(
        st,
        t_in,
        flag_in,
        dt,
        goal_active,
        pos,
        near || sheep_near,
        water,
        rock_mask,
        rng,
    );
    // O 0x141d852df 续段：`state=r15`、`[+4]=xmm0`、
    // `phase_ref = fmod(phase_ref + flag*dt, 1.0)`（0x142923f10）。
    st.state = next_state;
    st.state_t = t_out;
    st.phase_ref = (st.phase_ref + flag_out * dt) % 1.0;
}

/// 跳表 DAT_142f68008 handler（按 `Butterfly.state` 索引，0x141d85983/0x141d85a22/
/// 0x141d85ada/0x141d85b08 + 4..7=vel 表共享偏移）。
///
/// 2026-10-17 fresh 反汇编逐 handler 解码（`Byterfly.state_t` = 记录 +4）：
/// - **state0**（0x141d85983）：`0.5 >= phase_ref` 时
///   `flag = min((0.5-phase_ref)*0.1/dt, flag)`（0x142925984=0.1）；`t -= dt`；
///   `t <= 0（含 unordered）|| near/sheep` → state3，否则 state0。
/// - **state1**（0x141d85a22）：默认 state2。`phase == 0.0` 且 `goal.active` 且
///   `rock_masksample==0` 且 `water<0.01`（0x142960674）才进入：`height > 0.6`
///   （0x142925908）→ state1, t=0.6；否则 `t=uniform*10+5`（0x1429345a8/0x142925988）→ state0。
/// - **state2**（0x141d85ada）：`t -= dt`；`goal.active` → state = 2 + (t<=0 && phase==0)；
///   否则 `t = max(t, 5.0)`、state=2。
/// - **state3**（0x141d85b08）：`t = height`；`height >= 1.5`（0x1429258d0）→
///   `t = uniform*5+5`、state2；否则 state3（NaN 保持 3）。
/// - state4..7：跳表项指向 `set_butterfly_velocities` 内的动画 handler（非本上下文
///   有效 handler）——保持 state/t（登记）。
#[allow(clippy::too_many_arguments)]
fn butterfly_state_dispatch(
    st: &mut Butterfly,
    t_in: f32,
    flag_in: f32,
    dt: f32,
    goal_active: bool,
    pos: Vec3,
    near_or_sheep: bool,
    water: &WorldRaster<512>,
    rock_mask: &[terrain_rock_mask::TerrainRockMaskSsbo],
    rng: &mut PerchanceContext,
) -> (u32, f32, f32) {
    let mut flag = flag_in;
    match st.state {
        0 => {
            let phase_ref = st.phase_ref;
            // `UCOMISS 0.5,phase_ref; JB skip`：仅 `phase_ref <= 0.5`（有序）时取 min；
            // unordered（NaN）→ JB 命中 → 跳过。
            if phase_ref <= 0.5 {
                let candidate = (0.5 - phase_ref) * 0.1 / dt;
                flag = candidate.min(flag);
            }
            let t = t_in - dt;
            let next = if !(t > 0.0) || near_or_sheep { 3 } else { 0 };
            (next, t, flag)
        }
        1 => {
            let mut next = 2u32;
            let mut t = t_in;
            if st.phase == 0.0 && goal_active {
                let rock = terrain_rock_mask::TerrainRockMaskSsbo::sample_mapped_at_world_xz(rock_mask, pos.x, pos.z) > 0;
                if !rock {
                    let w = water
                        .as_ref()
                        .bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
                    if w < f32::from_bits(0x3c23d70a) {
                        next = 1;
                        // `UCOMISS 0.6,height; JB keep1`：`!(height <= 0.6)`（含 NaN）→ keep 1。
                        if !(pos.y <= 0.6) {
                            t = f32::from_bits(0x3f19999a);
                        } else {
                            t = rng.uniform_f32() * 10.0 + 5.0;
                            next = 0;
                        }
                    }
                }
            }
            (next, t, flag)
        }
        2 => {
            let t = t_in - dt;
            if goal_active {
                // B-ecstate FIX：二进制 0x141d85ada —— xmm2=(t<=0)?-1:0、
                // xmm1=(phase==0)?-1:0、andps、+2 → 条件成立 = 0xFFFFFFFF+2 = **1**，否则 2。
                let next = if t <= 0.0 && st.phase == 0.0 { 1 } else { 2 };
                (next, t, flag)
            } else {
                (2, t.max(5.0), flag)
            }
        }
        3 => {
            if pos.y >= f32::from_bits(0x3fc00000) {
                (
                    2,
                    rng.uniform_f32() * f32::from_bits(0x40a00000) + f32::from_bits(0x40a00000),
                    flag,
                )
            } else {
                (3, pos.y, flag)
            }
        }
        // 跳表 4..7 项指向 `set_butterfly_velocities` 的动画 handler；在 states
        // 上下文无对应写面（登记）。
        _ => (st.state, t_in, flag),
    }
}

/// update_butterfly_goals（0x141d86060）。
///
/// 查询（PDB 候选 `(Transform, &Velocity, &mut Butterfly, &mut CritterGoal)` + 反编译
/// 6 组件）。每蝴蝶：`timer@+8 -= dt`；`timer<=0 || goal.active==0 || dist² < 0.01`
/// → 刷新目标。
///
/// 未实现（U）：反编译显示两个 per-entity stride-4 写 `= query+0x14`（列 +0x28 视图；
/// 值与组件身份未恢复），本系统未建模。
pub fn update_butterfly_goals(
    time: Res<GameTime>,
    rng: Res<CritterRng>,
    mut query: Query<(&Transform, &Velocity, &mut Butterfly, &mut CritterGoal)>,
) {
    let dt = time.delta;
    let mut rng_ctx = rng.0.lock().unwrap();
    for (transform, velocity, mut butterfly, mut goal) in &mut query {
        let pos = transform.translation;
        // CritterGoal 桥接（country-core {position,goal_tag}，DIVERGE 见 shared.rs 头）。
        let mut goal_opt = if goal.goal_tag != 0 {
            Some(goal.position)
        } else {
            None
        };
        update_butterfly_goal_core(
            butterfly.state,
            &mut butterfly.goal_timer,
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

/// 蝴蝶目标刷新核心（O 0x141d864af-0x141d86734）。
///
/// - `timer -= dt`；`timer > 0 && goal.active && dist² >= 0.01` → 保留（不刷新）。
/// - 刷新：`timer = uniform*0.5 + 0.1`；`uniform_sample_sphere`（2×uniform_f32 打包）；
///   `x0z` 作用于 `&Velocity` 得动量项 (vel.x, 0, vel.z)。
/// - 权重（O 0x141d865b1 打包 select，0x142f67d10/0x142f67d20）：
///   `w0 = state==1 ? 0.5 : 1.0`；`w1 = prev_active ? 0x3f5a2e8c : 0xbe91745d`。
/// - 方向（O 0x141d8661a-0x141d86627）：`dir = (s.x*w0 + vel.x*w1, s.y*w0*0.5,
///   s.z*w0 + vel.z*w1)`，normalize_or_zero（0x141d86644 is-normal 检查）。
/// - `proposed = pos + dir*(timer*1.76)`（0x142f67d30=1.76）；y clamp <= 8.0；
///   `sqrt(z²+x²) < 65.0 || < 原半径` → 接受（写 goal），否则清 goal。
pub fn update_butterfly_goal_core(
    state: u32,
    timer: &mut f32,
    goal: &mut Option<Vec3>,
    pos: Vec3,
    velocity: Vec3,
    dt: f32,
    rng: &mut PerchanceContext,
) {
    *timer -= dt;
    let prev_active = goal.is_some();
    let arrived = match *goal {
        Some(target) => {
            let d = target - pos;
            d.length_squared() < f32::from_bits(0x3c23d70b)
        }
        None => true,
    };
    if *timer > 0.0 && prev_active && !arrived {
        return;
    }
    *timer = rng.uniform_f32() * 0.5 + 0.1;
    let s = uniform_sample_sphere(rng);
    let w0 = if state == 1 { 0.5 } else { 1.0 };
    let w1 = if prev_active {
        f32::from_bits(0x3f5a2e8c)
    } else {
        f32::from_bits(0xbe91745d)
    };
    let dir = Vec3::new(
        s.x * w0 + velocity.x * w1,
        s.y * w0 * 0.5,
        s.z * w0 + velocity.z * w1,
    )
    .normalize_or_zero();
    let d = *timer * 1.76;
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
///
/// 查询（PDB 候选）：`(Transform, &Butterfly, &CritterGoal,
/// &mut CritterVelocityTarget, &mut CritterVelocityMultiplier)`（旧含 &Sheep 为错）。
///
/// 每蝴蝶（O 0x141d86cb7-0x141d86de3）：
/// - `dx = (active ? goal.x : pos.x) - pos.x`；`dz = (active ? goal.z : pos.z) - pos.z`。
/// - 长度 = sqrt(dx²+dy²+dz²)，`dir = normalize_or_zero((dx, dy, dz)) * 1.76`
///   （FIX③；机器码按 transform.z 差分作竖直分量与 glam y 槽一致）。
/// - `*CritterVelocityMultiplier = Butterfly.phase + 1.0`。
/// - 动画跳表 DAT_142f68018 按 `Butterfly.state` 索引派发（见 butterfly_anim_dispatch）。
///
/// 未实现（U）：反编译显示另两个 per-entity stride-4 写 `= query 派生值`
/// （R9/RDX 槽，列 +0x28 视图；值 U），本系统未建模。
pub fn set_butterfly_velocities_towards_goals(
    mut query: Query<(
        &Transform,
        &Butterfly,
        &CritterGoal,
        &mut CritterVelocityTarget,
        &mut CritterVelocityMultiplier,
    )>,
) {
    for (transform, butterfly, goal, mut target, mut multiplier) in &mut query {
        let pos = transform.translation;
        // CritterGoal 桥接：goal_tag!=0 → goal.position；否则零方向。
        let d = if goal.goal_tag != 0 {
            goal.position - pos
        } else {
            Vec3::ZERO
        };
        // FIX③：二进制 dir=normalize((dx,dy,dz))*1.76（dy=goal.y-trans.y；机器码把
        // transform.z 差分当竖直分量，与 glam y 槽一致）。
        target.direction = Vec3::new(d.x, d.y, d.z).normalize_or_zero() * 1.76;
        // O 0x141d86dc3：*Multiplier = Butterfly.phase + 1.0（非每帧 +=1.0）。
        multiplier.0 = butterfly.phase + 1.0;
        // O 0x141d86dd5-0x141d86de3：动画跳表 DAT_142f68018 按 state 索引。
        butterfly_anim_dispatch(butterfly.state, &mut target, &mut multiplier, butterfly);
    }
}

/// 跳表 DAT_142f68018 handler（动画/姿态，按 `Butterfly.state` 索引；
/// 0x141d86de5/0x141d86e39/0x141d86e09/0x141d86e2a）。
///
/// 已解码（RE-ANALYSIS §1.5 + handlers-butterfly.json）：
/// - state0（0x141d86de5）：清零 target/velocity `[rbx]=0; [rbx+8]=0` + `[rdi]=0`（multiplier=0）。
/// - state1（0x141d86e39）：`[rbx] *= 0.5(xmm3)`；`[rbx+8] *= 0.5`（衰减 x/z）；
///   `[rbx+4]=0xbf6147ae`（-y）。
/// - state2（0x141d86e09）：`xmm4(1.5@0x1429258d0) > [r15+0x14](phase_ref)` 门 →
///   `[rbx+4]=max([rbx+4], xmm5(0x3ee147ae≈0.44))`。
/// - state3（0x141d86e2a）：`[rbx+4]=0x3f6147ae`（+y）。
///
/// R9/RDX 槽写（`[r9]=ebp`/`[rdx]=[rsp+0x4c]`）与主循环一致（查询派生值），不建模。
fn butterfly_anim_dispatch(
    state: u32,
    target: &mut CritterVelocityTarget,
    multiplier: &mut CritterVelocityMultiplier,
    butterfly: &Butterfly,
) {
    match state {
        0 => {
            target.direction = Vec3::ZERO;
            multiplier.0 = 0.0;
        }
        1 => {
            target.direction.x *= f32::from_bits(0x3f000000);
            target.direction.z *= f32::from_bits(0x3f000000);
            target.direction.y = f32::from_bits(0xbf6147ae);
        }
        2 => {
            if f32::from_bits(0x3fc00000) > butterfly.phase_ref {
                target.direction.y = target.direction.y.max(f32::from_bits(0x3ee147ae));
            }
        }
        3 => {
            target.direction.y = f32::from_bits(0x3f6147ae);
        }
        _ => {}
    }
}

/// update_butterfly_audio（0x141d86eb0）：逐蝴蝶 `ButterflyAudio::set_position`。
/// 查询 `(Transform, &mut ButterflyAudio)`（PDB 候选；反编译 0x141d86eb0 双分支遍历）。
pub fn update_butterfly_audio(mut query: Query<(&Transform, &mut ButterflyAudio)>) {
    for (transform, mut audio) in &mut query {
        audio.set_position(transform.translation);
    }
}

/// Original full population projection, count validation, and mapped SSBO write.
pub fn butterflies_send_to_gpu(
    mut buffers: ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    query: Query<(&Transform, &Butterfly)>,
) {
    let instances: Vec<ButterflySsbo> = query
        .iter()
        .map(|(transform, butterfly)| {
            ButterflySsbo::from_components(
                transform.translation,
                transform.rotation,
                butterfly.phase_ref,
            )
        })
        .collect();
    if instances.is_empty() {
        return;
    }
    // Original author asserts the complete fixed population before looking up its SSBO.
    assert_eq!(
        instances.len(),
        24,
        "Unexpected butterfly population for GPU upload"
    );
    if let Some(buffer) = buffers.get_mut::<ButterflySsbo>() {
        buffer.cpu_write(&instances);
    }
}

/// 蝴蝶 GPU 实例数据资源（宿主把 `instances` 写入 Butterfly Buffer；GPU 后置）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ButterflyGpuData {
    pub instances: Vec<ButterflySsbo>,
}
