//! duck 行为链（B 主基线，fresh 反编译级证据 2026-10-17）。
//!
//! 证据（fresh decomp 14230ff00 / 142312200 / 142313b30 / 142314470 / 142314b70，
//! disasm 复核；符号实证见 `find_functions` 结果）：
//! - plan_duck_trajectories 0x14230ff00
//! - move_ducks_along_trajectories 0x142312200
//! - update_duck_goals 0x142313b30（既有重建，未在本批改动）
//! - set_duck_velocities_towards_goals 0x142314470
//! - update_duck_ripples 0x142314b70
//!
//! DuckState 语义（fresh）：0=出生初态（正向）、1=正向巡航、2=自由游、3=反向。
//! move_ducks 的 `lVar25 = state>1 ? state-1 : 0` → 0/1 正向、2 跳过、3 反向。
//! 时序 dt = GameTime.delta；RNG = perchance uniform_f32（0x141625ae0）。

use bevy_ecs::prelude::*;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use country_core::systems::collision::world::RaycastWorld;
use country_core::systems::paths::PathRastersResources;
use country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush;
use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::asset_libraries::shader_library::AssetShaderLibrary;
use country_core::components::drawable::DrawableMeshBundle;
use glam::{Vec2, Vec3};
use perchance::PerchanceContext;

use crate::components::{
    CritterGoal, CritterLocomotion, CritterVelocityMultiplier, CritterVelocityTarget, Duck,
    DuckState, DuckTrajectoryState, Transform, Velocity,
};
use crate::duck_splash_particle::{
    MeshLibraryHandle, ShaderLibraryHandle, SpawnSplashParticle,
};
use crate::sample::{golden_spiral, uniform_sample_circle, uniform_sample_disk, x0y};

/// `Trajectory`（B 元素 stride 0x20）：点 Vec + 完成字节 @+0x18。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Trajectory {
    /// `Vec<Vec3>`（0xc/点）。
    pub points: Vec<Vec3>,
    /// @+0x18：该轨迹已完成（末点越出 glade 半径）。
    pub done: bool,
}

/// `TrajectoryPlan`（B stride 0x30）：轨迹 Vec + 圆偏移（两 f32）+ key（+0x20/+0x24）
/// + 年龄计数（+0x28，`retain(age < 301)` 0x12d）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TrajectoryPlan {
    pub trajectories: Vec<Trajectory>,
    /// +0x18：两 f32 随机圆偏移。
    pub offset: [f32; 2],
    /// +0x20/+0x24：key（0 = 普通 spawn 计划；非 0 = dev 计划，保存目标鸭实体）。
    pub key: u64,
    /// +0x28：plan 执行计数（retain `< 0x12d`）。
    pub age: u32,
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

/// 计划点采样闭包（B `closure_env_4` / `closure_env_5`，0x14230fa90 / 0x14230fa40）：
/// - key==0 的计划 → **identity**（closure_env_5 空体）；
/// - key!=0 的计划 → `log1p(max((t/3)³, 1e-10))`（closure_env_4 fresh 反编译逐句）。
#[inline]
fn plan_sample_shape(key: u64, t: f32) -> f32 {
    if key == 0 {
        t
    } else {
        let v = t / 3.0;
        let cube = v * v * v;
        let v = if cube <= 1e-10 { 1e-10 } else { cube };
        v.ln_1p()
    }
}

/// plan_duck_trajectories（0x14230ff00）。
///
/// 结构（fresh 逐语句）：
/// - `_DEV_OVERRIDE_IS_DEMO_MODE != 0` → 整个系统直接返回。
/// - dev 热键 A（0x1f）/B（0x27）：`developer_hotkeys_enabled()` 门（发行关闭）。
///   A：光标点 + 2×`golden_spiral(k+uniform)` → 3 点轨迹 + 随机圆偏移 → push 计划；
///   B：状态==2 且 `x²+z²<=25350` 的鸭 → 以实体为 key 建 1 点计划（采样时用 log1p 闭包）。
/// - 主路径：逐计划（最多 1 次 key==0 / 20 次 key!=0）：
///   - 每条未完成轨迹：
///     - len==1 → 新点 `(p.x+off.x*0.5, shape(0.5), p.z+off.y*0.5)`（xvy 语义）；
///     - len>1  → `t=(idx/count)*0.05+2.15`，`mid=lerp(last-2,last,t)`（x/z），
///       `d=|first.xz-mid.xz|` → 新点 `(mid.x, shape(d), mid.z)`；
///     - 追加；有 raycast 资源时两点间 `legacy_raycast`：命中码!=2 且命中点距 <= 段长² →
///       重采样圆偏移并把本计划所有轨迹重置为 1 点；否则 len>1 且末点 `x²+z²>25350`
///       （159.2²）→ done=1。
///   - 所有轨迹 done → 结束本计划。
/// - key==0 计划：`"duck_shader"` 网格/着色器句柄 + 每条轨迹：`Duck::new`（先取 1 个
///   uniform_f32）→ 拷贝点并**反转**（首末交换）→ spawn 10 组件 bundle（DuckState=0）。
/// - key!=0 计划：按 key 找鸭 → 覆盖轨迹（同样反转）→ `progress=1.0`、`DuckState=3`。
/// - `Vec::retain(age < 301)`。
///
/// 残余（登记）：B `DEV_OVERRIDE_IS_DEMO_MODE` 资源不在 crate（host seam）；
/// duck 出生音频参数（Duck::new 的 uniform_f32 实参）U。
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn plan_duck_trajectories(
    mut planner: ResMut<DuckTrajectoryPlanner>,
    mut commands: Commands,
    time: Res<GameTime>,
    rng: Res<CritterRng>,
    raycast: Option<Res<crate::shared::RaycastWorldHandle>>,
    cursor: Option<Res<CursorTerrainRaycast>>,
    mesh_library: Option<Res<MeshLibraryHandle>>,
    shader_library: Option<Res<ShaderLibraryHandle>>,
    dev: Res<DevHotkeys>,
    mut ducks: Query<(Entity, &mut Duck, &mut DuckState, &Transform)>,
) {
    let _ = time;
    let mut rng_ctx = rng.0.lock().unwrap();

    // ── dev 热键 A：光标 3 点轨迹（O 0x14231020c–0x142310429）────────────────────
    if dev.enabled && dev.just_pressed_a {
        if let Some(cursor) = cursor.as_ref() {
            let base = cursor.position();
            let mut points = Vec::with_capacity(3);
            points.push(base);
            for k in 1..3 {
                let u = rng_ctx.uniform_f32();
                points.push(base + x0y(golden_spiral(k as f32 + u)));
            }
            let u = rng_ctx.uniform_f32();
            let circle = uniform_sample_circle(&mut rng_ctx, u);
            planner.plans.push(TrajectoryPlan {
                trajectories: vec![Trajectory { points, done: false }],
                offset: [circle.x, circle.y],
                key: 0,
                age: 0,
            });
        }
    }
    // ── dev 热键 B：状态 2 + 半径内鸭 → keyed 计划（O 0x14231044x 起）──────────
    if dev.enabled && dev.just_pressed_b {
        for (entity, _duck, state, transform) in &mut ducks {
            if state.to_byte() != 2 {
                continue;
            }
            let p = transform.translation;
            if p.x * p.x + p.z * p.z > 25350.0 {
                continue;
            }
            let key = entity.to_bits();
            if planner.plans.iter().any(|plan| plan.key == key) {
                continue;
            }
            let u = rng_ctx.uniform_f32();
            let circle = uniform_sample_circle(&mut rng_ctx, u);
            planner.plans.push(TrajectoryPlan {
                trajectories: vec![Trajectory {
                    points: vec![p],
                    done: false,
                }],
                offset: [circle.x, circle.y],
                key,
                age: 0,
            });
        }
    }

    let mut plans = std::mem::take(&mut planner.plans);
    for plan in &mut plans {
        let max_iter = if plan.key == 0 { 1 } else { 0x14 };
        let shape_key = plan.key;
        for _ in 0..max_iter {
            let count = plan.trajectories.len();
            let mut all_done = count > 0;
            let mut resample = false;
            for idx in 0..count {
                let traj = &mut plan.trajectories[idx];
                if traj.done {
                    continue;
                }
                all_done = false;
                let len = traj.points.len();
                if len == 0 {
                    // B：`panic_bounds_check`（空轨迹）。
                    panic!("empty duck trajectory");
                }
                if len == 1 {
                    let p = traj.points[0];
                    let off = Vec2::new(plan.offset[0], plan.offset[1]) * 0.5;
                    let y = plan_sample_shape(shape_key, 0.5);
                    traj.points.push(Vec3::new(p.x + off.x, y, p.z + off.y));
                } else {
                    let t = (idx as f32 / count as f32) * 0.05 + 2.15;
                    let a = traj.points[len - 2];
                    let b = traj.points[len - 1];
                    let mid_x = a.x * (1.0 - t) + b.x * t;
                    let mid_z = a.z * (1.0 - t) + b.z * t;
                    let first = traj.points[0];
                    let dx = first.x - mid_x;
                    let dz = first.z - mid_z;
                    let d = (dx * dx + dz * dz).sqrt();
                    traj.points.push(Vec3::new(mid_x, plan_sample_shape(shape_key, d), mid_z));
                }
                plan.age = plan.age.wrapping_add(1);
                let len = traj.points.len();
                // raycast 检查（O：命中码!=2 且命中点距² <= 段长² → 重采样）。
                if let Some(raycast) = raycast.as_ref() {
                    if len >= 2 {
                        let a = traj.points[len - 2];
                        let b = traj.points[len - 1];
                        let delta = Vec3::new(b.x - a.x, b.y - a.y, b.z - a.z);
                        let res = raycast.0.legacy_raycast(a, delta);
                        if res.toi.to_bits() != country_core::systems::collision::world::RAYCAST_WORLD_RESULT_MISS {
                            let hp = res.position;
                            let d2 = (hp - b).length_squared();
                            let seg2 = delta.length_squared();
                            if d2 <= seg2 {
                                resample = true;
                                break;
                            }
                        }
                    }
                }
                if len > 1 {
                    let last = traj.points[len - 1];
                    if last.x * last.x + last.z * last.z > 25350.0 {
                        traj.done = true;
                    }
                }
            }
            if resample {
                // O：uniform_f32 + uniform_sample_circle 重写偏移，所有轨迹重置为 1 点。
                let u = rng_ctx.uniform_f32();
                let circle = uniform_sample_circle(&mut rng_ctx, u);
                plan.offset = [circle.x, circle.y];
                for traj in &mut plan.trajectories {
                    traj.points.truncate(1);
                    traj.done = false;
                }
                continue;
            }
            if all_done {
                break;
            }
        }
    }

    // ── 计划收尾（O 0x142310ec3 起）──────────────────────────────────────────────
    // B 资产库资源未就绪时：无 bundle（宿主缝；B 侧为 unwrap）。
    let mesh = mesh_library
        .as_ref()
        .and_then(|l| l.0.as_ref())
        .map(|lib| lib.get_handle_by_name("duck_shader"));
    let shader = shader_library
        .as_ref()
        .and_then(|l| l.0.as_ref())
        .and_then(|lib| lib.get_raster_handle_by_name("duck_shader"));
    for plan in &mut plans {
        if plan.key == 0 {
            for traj in &plan.trajectories {
                if traj.points.is_empty() {
                    continue;
                }
                // O：Duck::new 前先取 1×uniform_f32（音频参数源 U）。
                let _audio_param = rng_ctx.uniform_f32();
                let mut points = traj.points.clone();
                points.reverse();
                let base = points.first().copied().unwrap_or_default();
                let duck = Duck::new_planned(base, points);
                let mut entity = commands.spawn((
                    duck,
                    DuckState::Initial,
                    CritterLocomotion(0),
                    CritterGoal {
                        position: Vec3::ZERO,
                        goal_tag: 0,
                    },
                    CritterVelocityTarget::default(),
                    ObjectGrassPush(0.0),
                    Velocity(Vec3::ZERO),
                    CritterVelocityMultiplier(1.0),
                    crate::components::PrevTransform::default(),
                ));
                if let (Some(mesh), Some(shader)) = (mesh.clone(), shader) {
                    entity.insert(DrawableMeshBundle {
                        mesh,
                        shader,
                        transform: Transform::default(),
                    });
                } else {
                    // bundle 缺席时仍提供独立 Transform（move/update 查询面）。
                    entity.insert(Transform::default());
                }
            }
        } else {
            // key!=0：按 key 找鸭，覆盖轨迹（反转）+ progress=1.0 + state=3。
            let key = plan.key;
            for (entity, mut duck, mut state, _transform) in &mut ducks {
                if entity.to_bits() != key {
                    continue;
                }
                if let Some(traj) = plan.trajectories.first() {
                    let mut points = traj.points.clone();
                    points.reverse();
                    duck.trajectory = points;
                    duck.progress = 1.0;
                    *state = DuckState::TrajectoryReverse;
                }
                break;
            }
        }
    }
    plans.retain(|plan| plan.age < 0x12d);
    planner.plans = plans;
}

/// move_ducks_along_trajectories（0x142312200）。
///
/// fresh 逐语句重建：
/// - state==2 跳过；0/1 正向（dir=+1），3 反向（dir=-1），其余 panic（本重建忽略）。
/// - `target = dir*0.5+0.5`；`progress = clamp01(Duck+0xec)`。
/// - `progress == target`（有序且非 NaN）→ 结束帧：正向 `state=2`；反向 despawn 并
///   `state=3`。写 `goal = {active=1, pos = translation + (rot*Z).xz}`、
///   `Duck+0xe0/+0xe1 = 1,0`、`dwell = (4.0, 2.0)`（0x142b2e410 实读）、multiplier=1.0；
///   然后跳到下一只（本帧不移动）。
/// - 移动帧：
///   - `chord = sqrt((last.x-first.x)²+(last.z-first.z)²)`；multiplier=0；
///   - `progress += dir*12.0*dt/chord`（12.0@0x1429cd2d4）；
///   - `t = (dir>0 ? progress : progress²) * (len-1)`，clamp `[0,len-1]`；
///     `truncf` → 段索引/小数 → `CurveInternalCoord::new_for_point_count`（末点 t=0 归上一段）；
///   - 姿态方向：正向 = `normalize(x0y(p[i+1]-p[i]))`（水平）；反向 =
///     `normalize(p[i]-p[i+1])`（3D）；`translation = lerp(p[i],p[i+1],frac)`；
///   - `f_f4 = clamp01(inverse_lerp(1-20/chord, 1.0, progress))`（-20@0x14295c698）；
///   - `smooth = smoothstep(clamp01(2*f4-1))`；`ObjectGrassPush = 2*smooth`；
///   - 音频：`(f4<=0.3 || state>1)` 时仅 `(f4>0.5 && state==3)` 播放；
///     否则直接播放（`start_if_not_playing + set_position + set_volume(水栅格采样)`）；
///     `progress<0.01 || 0.99<=progress || state==2` → loop stop，否则 loop 起播+
///     set_position+2×set_parameter（参数名/值 U）；
///   - `f4>0.25 && state<2 && state&1==0` → `state=1` + 写 `SpawnSplashParticle(entity)`；
///   - 高度积分：`f_f0 += (smooth2*4 + (1-smooth2))*dt`，其中
///     `smooth2 = smoothstep(clamp01((progress-(1-50/chord))/((1-20/chord)-(1-50/chord))))`。
///
/// 分歧/未实现（登记）：
/// - `Quat::from_rotation_arc` 的两端向量：fresh 为内联 glam 弧函数，输入平面与
///   `x0y` 结果混排，**未逐寄存器闭合** → 本重建用 `from_rotation_arc(Vec3::Y, dir)`
///   （与 boids 同型），登记 U。
/// - `AudioLoop::set_parameter` 两处参数名/值（寄存器）U。
/// - `set_volume` 源 = `WaterAudioGrid` 双线性采样；core 未暴露其 `FloatGridCells`，
///   本重建跳过 set_volume 并登记。
/// - `DEV_OVERRIDE_IS_DEMO_MODE`（demo 下逐鸭 despawn）资源不在 crate（host seam）。
#[allow(clippy::type_complexity)]
pub fn move_ducks_along_trajectories(
    time: Res<GameTime>,
    mut commands: Commands,
    mut splash: EventWriter<SpawnSplashParticle>,
    water: Res<WorldRaster<512>>,
    mut query: Query<(
        Entity,
        &mut Duck,
        &mut DuckState,
        &mut Transform,
        &mut Velocity,
        &mut CritterVelocityMultiplier,
        &mut CritterGoal,
        &mut ObjectGrassPush,
    )>,
) {
    let dt = time.delta;
    let w = (*water).as_ref();
    for (entity, mut duck, mut state, mut transform, mut velocity, mut multiplier, mut goal, mut grass)
        in &mut query
    {
        let state_byte = state.to_byte();
        // O：lVar25 = state>1 ? state-1 : 0；==1（state 2）跳过、空轨迹跳过。
        if state_byte == 2 || duck.trajectory.is_empty() {
            continue;
        }
        let dir = if state_byte == 3 { -1.0_f32 } else { 1.0 };
        let target = dir * 0.5 + 0.5;
        let progress = duck.progress.max(0.0).min(1.0);
        let points = duck.trajectory.clone();
        if points.len() == 1 {
            // B：`if lVar18 == 1 panic`。
            panic!("single-point duck trajectory");
        }
        let first = points[0];
        let last = points[points.len() - 1];

        if progress == target {
            // ── 结束帧 ────────────────────────────────────────────────
            let mut new_state = DuckState::FreeSwim;
            if dir < 0.0 {
                commands.entity(entity).despawn();
                new_state = DuckState::TrajectoryReverse;
            }
            *state = new_state;
            // goal = pos + (rot * Z).xz（O：x0y(rotated column)）。
            let fwd = transform.rotation * Vec3::Z;
            goal.position = Vec3::new(
                transform.translation.x + fwd.x,
                transform.translation.y,
                transform.translation.z + fwd.z,
            );
            goal.goal_tag = 1;
            duck.moving = true;
            duck.aux = 0;
            duck.dwell = [4.0, 2.0];
            multiplier.0 = 1.0;
            continue;
        }

        // ── 移动帧 ─────────────────────────────────────────────────────
        let dx = last.x - first.x;
        let dz = last.z - first.z;
        let chord = (dx * dx + dz * dz).sqrt();
        multiplier.0 = 0.0;
        let progress_adv = (dir * 12.0 * dt) / chord + duck.progress;
        duck.progress = progress_adv;
        let len_m1 = (points.len() - 1) as f32;
        let t = if dir > 0.0 { progress_adv } else { progress_adv * progress_adv } * len_m1;
        let t = t.max(0.0).min(len_m1);
        let seg = t.trunc() as usize;
        let frac = t - t.trunc();
        let coord = utils::curve::CurveInternalCoord::new_for_point_count(
            seg,
            frac,
            duck.trajectory.len(),
        );
        let p0 = duck.trajectory[coord.segment];
        let p1 = duck.trajectory[coord.segment + 1];
        let frac = coord.t;
        // 方向：正向水平（x0y），反向 3D（O 142313025/064 分支）。
        let d = if dir > 0.0 {
            Vec3::new(p1.x - p0.x, 0.0, p1.z - p0.z)
        } else {
            p0 - p1
        };
        let inv = 1.0 / (d.x * d.x + d.y * d.y + d.z * d.z).sqrt();
        let d = Vec3::new(d.x * inv, d.y * inv, d.z * inv);
        // 仅位置/朝向/缩放的位姿（旋转端点 U，见函数注）。
        let old_translation = transform.translation;
        let new_translation = p0.lerp(p1, frac);
        transform.translation = new_translation;
        transform.rotation = glam::Quat::from_rotation_arc(Vec3::Y, d);
        transform.scale = Vec3::ONE;
        // O 1423131f4：Velocity = x0y(lerp - old) / (2*dt)——y 分量恒 0。
        let two_dt = dt + dt;
        velocity.0 = Vec3::new(
            (new_translation.x - old_translation.x) / two_dt,
            0.0,
            (new_translation.z - old_translation.z) / two_dt,
        );

        // O：inverse_lerp(1-20/chord, 1.0, progress) → clamp01 → Duck+0xf4。
        let f4 = inverse_lerp(1.0 - 20.0 / chord, 1.0, progress_adv);
        let f4 = f4.max(0.0).min(1.0);
        duck.f_f4 = f4;
        let s = (f4 + f4 - 1.0).max(0.0).min(1.0);
        let smooth = (3.0 - s - s) * s * s;
        grass.0 = smooth + smooth;

        // 音频（O 1423133f0/491）：两分支等价 `start_if_not_playing + set_position`。
        let play = if f4 <= 0.3 || state_byte > 1 {
            f4 > 0.5 && state_byte == 3
        } else {
            true
        };
        if play {
            duck.audio.quack.start_if_not_playing();
            duck.audio.quack.set_position(transform.translation);
            // set_volume(WaterAudioGrid bilinear @130) 未实现（core 未暴露采样），登记。
            let _ = w;
        }
        // loop 控制（O：progress<0.01 || 0.99<=progress || state==2 → stop）。
        if duck.progress < 0.01 || 0.99 <= duck.progress || state_byte == 2 {
            duck.audio.stop_loops();
        } else {
            duck.audio.start_loops(transform.translation);
            // AudioLoop::set_parameter ×2：参数名/值 U（登记）。
        }

        // O：f4>0.25 && state<2 && (state&1)==0 → state=1 + SpawnSplashParticle(entity)。
        if f4 > 0.25 && state_byte < 2 && (state_byte & 1) == 0 {
            *state = DuckState::TrajectoryForward;
            splash.write(SpawnSplashParticle(entity));
        }

        // 高度积分（O 142313665 起）：
        // smooth2 = smoothstep(clamp01((progress-a)/(b-a))),
        // a=1-50/chord, b=1-20/chord。
        let a = 1.0 - 50.0 / chord;
        let b = 1.0 - 20.0 / chord;
        let x = ((duck.progress - a) / (b - a)).max(0.0).min(1.0);
        let s2 = (3.0 - x - x) * x * x;
        duck.f_f0 += (s2 * 4.0 + (1.0 - s2)) * dt;
    }
}

/// `utils::inverse_lerp(a,b,t)` = `(t-a)/(b-a)`（内联叶函数 0x140c904a0）。
#[inline]
fn inverse_lerp(a: f32, b: f32, t: f32) -> f32 {
    (t - a) / (b - a)
}

/// 沿轨迹分段长度（保留旧 API；新 move 路径用 chord 归一，不再调用）。
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

/// 曲线帧（旧 API；保留）。
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

/// 进度推进（旧 API）。
pub fn advance_trajectory(progress: f32, speed_dir: f32, segment_len: f32, dt: f32) -> f32 {
    progress + (speed_dir * 12.0 * dt) / segment_len
}

/// update_duck_goals（0x142313b30，源码 581–695；fresh 反编译重建 2026-10-17）。
///
/// 状态机（disasm 142313ebd–142313ff6，fresh）：
/// - `moving==0`（Duck+0xe0）：`dwell[0] -= dt`；`<=0` → 2×uniform_f32 →
///   `dwell=(u1*3+1, u2*3+1)` [1,4]，moving=1，aux=0。
/// - `moving==1`：两半 `-= dt`；复位（moving→0）当 `dwell[0]<=0 && aux==0`：
///   1×uniform_f32 → `dwell[0]=u*0.5+0.5`（低半；高半 = RNG 残留寄存器，U）、
///   moving=0，aux=0。
/// - 否则目标逻辑：keep 门 = `aux!=0 || dwell[1]>0`；
///   keep 时 `goal.active && dist²>=0.16`（0x3e23d70b）→ 保留，否则换目标。
/// - 换目标：写 `dwell[1] = 0.1`；偏移 = `x0y(disk) + x0z(rotation*disk)*scale`
///   （scale = goal 曾 active ? 0.8 : 0.0），**归一化后**加 pos；校验
///   `!within_glade && dist(pos)<=dist(new)`、水(new)<=0.5、4 栅格和 clamp>=0.1 任一 →
///   goal.active=0。
/// - 结尾 `aux=0`。
#[allow(clippy::too_many_arguments)]
pub fn update_duck_goals(
    time: Res<GameTime>,
    water: Res<WorldRaster<512>>,
    path_rasters: PathRastersResources,
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
        let rotation = transform.rotation;
        let mut gs = duck.goal_state();
        let mut g = goal_value(&goal);
        update_duck_goal_core(
            &mut gs,
            &mut g,
            pos,
            rotation,
            dt,
            &mut rng_ctx,
            &|_x, _z| false, // is_within_glade_shape（dev 门控 false；见模块注）
            &|x, z| w.bilinear_sample_at_world_xz(x, z, 130.0, 130.0),
            &|x, z| {
                let mut s = 0.0f32;
                for grid in path_rasters.as_float_grid() {
                    s += grid.bilinear_sample_at_world_xz(x, z, 130.0, 130.0);
                }
                s
            },
        );
        goal_write(&mut goal, g);
        duck.apply_goal_state(&gs);
        // L695：*Duck+0xe1 = 0（次标志每帧归零）。
        duck.aux = 0;
        let _ = state;
    }
}

/// 鸭子目标刷新核心（disasm 142313ebd 逐语句重建）。
pub fn update_duck_goal_core(
    state: &mut DuckTrajectoryState,
    goal: &mut Option<Vec3>,
    pos: Vec3,
    rotation: glam::Quat,
    dt: f32,
    rng: &mut PerchanceContext,
    within_glade: &dyn Fn(f32, f32) -> bool,
    water_sample: &dyn Fn(f32, f32) -> f32,
    path_sum: &dyn Fn(f32, f32) -> f32,
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
    // FIX②：moving==1 的复位条件 = `dwell[0]<=0 && aux==0`。
    if state.dwell[0] <= 0.0 && state.aux == 0 {
        let u = rng.uniform_f32();
        state.dwell[0] = u * 0.5 + 0.5;
        state.moving = false;
        state.aux = 0;
        return;
    }
    // 目标逻辑：keep 门（disasm 0x142313f5f/0x142313f76/0x142313f7e）。
    let keep_gate = state.aux != 0 || state.dwell[1] > 0.0;
    if keep_gate {
        let keep = match *goal {
            Some(g) => (g - pos).length_squared() >= f32::from_bits(0x3e23d70b),
            None => false,
        };
        if keep {
            state.aux = 0;
            return;
        }
    }
    // 换目标：写 Duck+0xe8 = 0.1。
    state.dwell[1] = 0.1;
    pick_duck_goal(goal, pos, rotation, rng, within_glade, water_sample, path_sum);
    state.aux = 0;
}

/// 换目标（disasm 142314033 起逐语句重建）。
fn pick_duck_goal(
    goal: &mut Option<Vec3>,
    pos: Vec3,
    rotation: glam::Quat,
    rng: &mut PerchanceContext,
    within_glade: &dyn Fn(f32, f32) -> bool,
    water_sample: &dyn Fn(f32, f32) -> f32,
    path_sum: &dyn Fn(f32, f32) -> f32,
) {
    let u = rng.uniform_f32();
    let disk = uniform_sample_disk(rng, u);
    let base = Vec3::new(disk.x, 0.0, disk.y);
    let rotated = rotation * base;
    let scale = if goal.is_some() { 0.8 } else { 0.0 };
    let offset = base + Vec3::new(rotated.x, 0.0, rotated.z) * scale;
    let n = offset.normalize_or_zero();
    let new_pos = pos + n;
    let within = within_glade(new_pos.x, new_pos.z);
    let dist_pos2 = pos.x * pos.x + pos.z * pos.z;
    let dist_new2 = new_pos.x * new_pos.x + new_pos.z * new_pos.z;
    if !within && dist_pos2 <= dist_new2 {
        *goal = None;
        return;
    }
    if water_sample(new_pos.x, new_pos.z) <= 0.5 {
        *goal = None;
        return;
    }
    if path_sum(new_pos.x, new_pos.z).clamp(0.0, 1.0) >= 0.1 {
        *goal = None;
        return;
    }
    *goal = Some(new_pos);
}

/// set_duck_velocities_towards_goals（0x142314470，源码 698–728）。
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
        let g = goal_value(&goal);
        set_duck_velocity_core(&mut target, &mut multiplier.0, g, duck, pos);
    }
}

/// CritterGoal ↔ Option<Vec3> 桥（tag!=0 = Some）。
fn goal_value(g: &CritterGoal) -> Option<Vec3> {
    if g.goal_tag != 0 {
        Some(g.position)
    } else {
        None
    }
}

/// 写回 CritterGoal（tag=1 = Some）。
fn goal_write(g: &mut CritterGoal, v: Option<Vec3>) {
    g.goal_tag = if v.is_some() { 1 } else { 0 };
    if let Some(p) = v {
        g.position = p;
    }
}

/// 目标向量 → 速度方向核心。
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

/// update_duck_ripples（0x142314b70，源码 735–796）：GPU 水波 SSBO 的 CPU 记录。
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

/// 鸭子出生用的水采样偏移（历史近似；plan 主路径已不用）。
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
/// 发行构建默认关闭（函数体为空 → 返回 false）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DevHotkeys {
    pub enabled: bool,
    pub just_pressed_a: bool,
    pub just_pressed_b: bool,
}

impl Duck {
    /// 构造带轨迹的鸭子（plan_duck 出生路径）：progress=0（spawn bundle 不写 +0xec）。
    pub fn new_planned(base: Vec3, points: Vec<Vec3>) -> Self {
        let mut duck = Duck::default();
        duck.trajectory = points;
        duck.progress = 0.0;
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
fn _audio_sink_contract(sink: &mut dyn crate::audio::CritterAudioSink, pos: Vec3) {
    sink.start_if_not_playing();
    sink.set_position(pos);
    sink.set_volume(1.0);
    sink.set_parameter("speed", 0.5);
    sink.stop();
}

// 保留：AssetShaderLibrary/AssetMeshLibrary 类型在签名面被使用（Option 资源包装）。
#[allow(dead_code)]
fn _asset_type_anchor(_m: &AssetMeshLibrary, _s: &AssetShaderLibrary, _r: &RaycastWorld) {}
