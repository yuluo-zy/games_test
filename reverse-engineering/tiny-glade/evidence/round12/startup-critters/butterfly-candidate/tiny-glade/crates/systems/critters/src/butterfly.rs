//! Butterfly policy from original author bytes; actual resource owners and all4 state handlers.
//! State/velocity original fixture evidence: round12/startup-critters.
use bevy_ecs::prelude::*;
use country_core::render::ssbo::StorageElement;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::time::GameTime;
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::render_loop::resources::critters::ButterflySsbo;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    resources::{terrain_rock_mask::TerrainRockMaskSsbo, world_raster_defs::WaterRaster},
};
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

/// Original exact query and mandatory canonical mapped-rock/water owners.
#[allow(clippy::too_many_arguments)]
pub fn update_butterfly_states(
    time: Res<GameTime>,
    camera: Res<MainCamera>,
    cursor: Res<CursorTerrainRaycast>,
    rng: Res<CritterRng>,
    water: Res<WaterRaster>,
    buffers: Res<AssetSsboLibrary>,
    mut query: Query<(
        &Transform,
        &Velocity,
        &mut Butterfly,
        &mut ButterflyAudio,
        &CritterGoal,
    )>,
    sheep_query: Query<(&Transform, &Sheep)>,
) {
    let mut rng = rng.0.lock().unwrap();
    // Original lookup occurs even when the species query is empty.
    let rock = buffers
        .get::<TerrainRockMaskSsbo>()
        .expect("update_butterfly_states requires TerrainRockMaskSsbo");
    let environment = LiveButterflyEnvironment {
        water: water.0.as_ref(),
        rock: rock.mapped,
    };
    let sheep: Vec<_> = sheep_query
        .iter()
        .map(|(t, _)| Vec2::new(t.translation.x, t.translation.z))
        .collect();
    let mode = camera.mode_byte();
    let eye = if mode == 3 {
        camera.eye_position()
    } else {
        Vec3::ZERO
    };
    for (transform, velocity, mut butterfly, _audio, goal) in &mut query {
        crate::butterfly_state::update_state(
            &mut butterfly,
            transform.translation,
            velocity.0,
            goal.goal_tag & 1 != 0,
            time.delta,
            mode,
            eye,
            cursor.position(),
            &sheep,
            &environment,
            &mut || rng.uniform_f32(),
        );
    }
}
struct LiveButterflyEnvironment<'a> {
    water: country_core::resources::world_raster::RasterBufferRef<'a, 512>,
    rock: &'a [TerrainRockMaskSsbo],
}
impl crate::butterfly_state::ButterflyEnvironment for LiveButterflyEnvironment<'_> {
    fn rock(&self, p: Vec2) -> u32 {
        TerrainRockMaskSsbo::sample_mapped_at_world_xz(self.rock, p.x, p.y)
    }
    fn water(&self, p: Vec2) -> f32 {
        self.water
            .bilinear_sample_at_world_xz(p.x, p.y, 130.0, 130.0)
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
