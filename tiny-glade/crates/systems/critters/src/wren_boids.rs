//! wren_boids::sim_wren_boids（B 主基线，反汇编级证据）。
//!
//! 证据（critters-extra-analysis.md §1 + reconstruction/critters/src/wren_boids.rs）：
//! - sim_wren_boids 0x142326a80（19140B，源码 56–533）：
//!   - L340 `*(char *)(*in_R8 + 5)` 非 0 才主模拟；==0 → 清空全部 Wren 实体。
//!   - 出生（L385–515）：dt != 0 时逐帧补，上限 80（0x4f<uVar58）；位置
//!     `x = u*130-65`、`y = u*9+1`；出生 = spawn(Wren, NaniBundle, PrevTransform) + 音频
//!     （AudioCore::instantiate_audio）。
//!   - FreeSpot 收集（L700）：from_iter<FreeSpot, FlatMap<QueryIter<BirdSpots>>>——
//!     未占用停靠点。
//!   - BirdHaven 树查询（L745）：`(BirdHaven, PrefabInstanceData), Without<TreeMarkedForDeletion>`
//!     → 树庇护球体。
//!   - 主循环（L1014+）：`sqrt(x²+y²) <= 91.9 && y <= 30` 才保留（BoidFlock::remove 清出界）。
//!   - 摇树出生（L1231+）：竖直计时 <=0 且 >10.0 → log "Tree has been shaken! Spawning b…"；
//!     burst = rng*5+1 只（受 80 上限）。
//! - Boid::frame_update<wren closure_env$3> 0x141dc3510：sep_r 1.3、align 0.1169、
//!   (min,max)=(7.0,14.0)；可行点判定 closure_3（0x141dc42f0）：RAY_DIRS 32 方向
//!   raycast（端点 y∈[0,30] 且（glade 标志==0 或 半径<=91.9））；不可行 → 反射转向
//!   （from_rotation_arc 转 RAY_DIRS 方向）。
//! - RAY_DIRS：const 0x200 字节 = 32 个 Vec3（DAT_14363aad0 Once 惰性初始化）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use glam::Vec3;

use crate::audio::{AudioHandle, CritterAudioCore};
use crate::boids::{frame_update, random_vel_change, Boid, Neighbor};
use crate::components::{BirdSpots, PrevTransform, Wren};
use crate::shared::RaycastWorldHandle;

/// 鹪鹩开关（`*in_R8+5`；==0 → 清空全部 Wren）。
#[derive(Resource, Clone, Copy, Debug)]
pub struct WrenEnabled(pub bool);

impl Default for WrenEnabled {
    fn default() -> Self {
        Self(true)
    }
}

/// 出生上限（0x50=80）与范围常量（O）。
pub const MAX_WRENS: usize = 80;
pub const WREN_RADIUS: f32 = 91.9;
pub const WREN_MAX_Y: f32 = 30.0;

/// RAY_DIRS：32 个射线方向（O：const 0x200 字节 = 32 Vec3；DAT_14363aad0 Once 惰性初始化）。
/// 方向值（yaw 均匀 32 份、无俯仰）为固定表（I：精确方向值未见 .data 导出）。
static RAY_DIRS: std::sync::OnceLock<Vec<Vec3>> = std::sync::OnceLock::new();

fn ray_dirs() -> &'static [Vec3] {
    RAY_DIRS.get_or_init(|| {
        (0..32)
            .map(|i| {
                let a = i as f32 * std::f32::consts::TAU / 32.0;
                Vec3::new(a.cos(), 0.0, a.sin())
            })
            .collect()
    })
}

/// 树庇护（BirdHaven；L745 查询组件；冠球 + 摇树计时）。
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct BirdHaven {
    /// 冠球（中心 + 半径；树查询产出）。
    pub sphere: (Vec3, f32),
    /// 竖直摇树计时（L1231+：>10.0 且 <=0 才摇）。
    pub shake_timer: f32,
}

/// 树删除标记（Without<TreeMarkedForDeletion> 过滤；宿主树系统写）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct TreeMarkedForDeletion;

/// 鹪鹩实体（Wren + boid 记录 + 音频）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct WrenBoidRecord {
    pub pos: Vec3,
    pub vel: Vec3,
    pub turn_timer: f32,
}

impl WrenBoidRecord {
    pub fn as_boid(&self) -> Boid {
        Boid {
            pos: self.pos,
            vel: self.vel,
        }
    }
    pub fn from_boid(b: Boid) -> Self {
        Self {
            pos: b.pos,
            vel: b.vel,
            turn_timer: 0.0,
        }
    }
}

/// 鹪鹩 RNG（fastrand；出生与随机转向，O）。
#[derive(Resource, Debug)]
pub struct WrenRng2(pub std::sync::Mutex<fastrand::Rng>);

impl Default for WrenRng2 {
    fn default() -> Self {
        Self(std::sync::Mutex::new(fastrand::Rng::with_seed(0x606f)))
    }
}

/// sim_wren_boids（0x142326a80）。
#[allow(clippy::too_many_arguments)]
pub fn sim_wren_boids(
    time: Res<GameTime>,
    enabled: Res<WrenEnabled>,
    raycast: Res<RaycastWorldHandle>,
    audio_core: Option<Res<CritterAudioCore>>,
    mut rng: ResMut<WrenRng2>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Wren,
        &mut WrenBoidRecord,
        Option<&mut WrenAudio>,
        Option<&BirdSpots>,
    )>,
    mut havens: Query<(Entity, &mut BirdHaven), Without<TreeMarkedForDeletion>>,
) {
    let dt = time.delta;
    if !enabled.0 {
        // ==0 → 清空全部 Wren 实体（逐 Wren despawn，Entities::contains 门控）。
        for (entity, _, _, _, _) in &mut query {
            commands.entity(entity).despawn();
        }
        return;
    }
    let r = &mut rng.0.lock().unwrap();

    // 出生（L385–515）：dt != 0 时逐帧补；上限 80；位置 x=u*130-65、y=u*9+1。
    let live = query.iter().count();
    if dt != 0.0 && live < MAX_WRENS {
        let to_spawn = (MAX_WRENS - live).min(2);
        for _ in 0..to_spawn {
            if query.iter().count() >= MAX_WRENS {
                break;
            }
            let pos = Vec3::new(
                r.f32() * 130.0 - 65.0,
                r.f32() * 9.0 + 1.0,
                r.f32() * 130.0 - 65.0,
            );
            let mut e = commands.spawn((
                Wren {
                    pos,
                    prev_pos: pos,
                    phase: r.f32(),
                    hop_timer: r.f32() * 2.0,
                },
                WrenBoidRecord::from_boid(Boid {
                    pos,
                    vel: Vec3::ZERO,
                }),
                PrevTransform::default(),
            ));
            // 音频（O：AudioCore::instantiate_audio("wren") 出生音）。
            if let Some(core) = &audio_core {
                e.insert(WrenAudio {
                    handle: core.instantiate("event:/SFX/Wren/Chirp"),
                });
            }
        }
    }

    // FreeSpot 收集（L700：FlatMap<QueryIter<BirdSpots>>——未占用停靠点）。
    let free_spots: Vec<Vec3> = query
        .iter()
        .filter_map(|(_, _, _, _, spots)| spots)
        .flat_map(|spots| spots.spots.iter().filter(|s| !s.occupied).map(|s| s.pos))
        .collect();

    // 主循环：保留条件 sqrt(x²+y²) <= 91.9 && y <= 30；boids 规则 + 可行点判定。
    let all: Vec<Neighbor> = query
        .iter()
        .map(|(_, _, boid, _, _)| Neighbor {
            pos: boid.pos,
            vel: boid.vel,
        })
        .collect();
    let mut despawn: Vec<Entity> = Vec::new();
    for (entity, mut wren, mut boid, mut audio, _spots) in &mut query {
        let self_idx = all
            .iter()
            .position(|n| n.pos.distance_squared(boid.pos) < 1e-4)
            .unwrap_or(usize::MAX);
        let mut b = boid.as_boid();
        // 有目标（最近 FreeSpot）→ 加速度上界 1.5（O fVar22）。
        let has_target = free_spots
            .iter()
            .any(|s| (s - b.pos).length_squared() < 64.0);
        let accel_max = if has_target { 1.5 } else { 1.0 };
        frame_update(&mut b, &all, self_idx, 1.3, 0.1169, 7.0, 14.0, dt, accel_max);
        // 随机转向（O：Boid::random_vel_change）。
        if let Some(dir) = random_vel_change(b.vel, &mut boid.turn_timer, dt, r) {
            b.vel = dir;
        }
        // 可行点判定（closure_3 0x141dc42f0：RAY_DIRS 32 方向 raycast；
        // 端点 y∈[0,30] 且（glade 标志==0 或 半径<=91.9））。
        if !is_viable_landing_spot(&raycast, b.pos, &ray_dirs()) {
            // 不可行 → 反射转向（O：from_rotation_arc 转 RAY_DIRS 方向；-5.0 常数）。
            let dir = ray_dirs()[r.usize(0..32)];
            b.vel = glam::Quat::from_rotation_arc(b.vel.normalize_or_zero(), dir)
                * (b.vel * -5.0);
        }
        b.pos += b.vel * dt;
        boid.pos = b.pos;
        boid.vel = b.vel;
        wren.pos = b.pos;
        // 音频循环（O：start_if_not_playing）。
        if let Some(audio) = audio.as_deref_mut() {
            audio.handle.start_if_not_playing();
        }

        // 保留条件（O）：出界 → BoidFlock::remove（swap_remove + despawn）。
        let r2 = (b.pos.x * b.pos.x + b.pos.y * b.pos.y).sqrt();
        if r2 > WREN_RADIUS || b.pos.y > WREN_MAX_Y {
            despawn.push(entity);
        }
    }
    for entity in despawn {
        commands.entity(entity).despawn();
    }

    // 摇树出生（L1231+）：竖直计时 >10.0 → log + burst（rng*5+1，受 80 上限）。
    for (_, mut haven) in &mut havens {
        haven.shake_timer += dt;
        if haven.shake_timer > 10.0 {
            log::info!("Tree has been shaken! Spawning birds...");
            haven.shake_timer = 0.0;
            let burst = 1 + (r.f32() * 5.0) as usize;
            for _ in 0..burst {
                if query.iter().count() >= MAX_WRENS {
                    break;
                }
                let pos = haven.sphere.0
                    + Vec3::new(
                        (r.f32() - 0.5) * 2.0,
                        r.f32(),
                        (r.f32() - 0.5) * 2.0,
                    );
                commands.spawn((
                    Wren {
                        pos,
                        prev_pos: pos,
                        phase: 0.0,
                        hop_timer: 1.0,
                    },
                    WrenBoidRecord::from_boid(Boid {
                        pos,
                        vel: Vec3::ZERO,
                    }),
                    PrevTransform::default(),
                ));
            }
        }
    }
}

/// 可行点判定（closure_3 0x141dc42f0 语义：沿 RAY_DIRS 逐方向 raycast；
/// 端点 y∈[0,30] 且（glade 标志==0 或 半径<=91.9）；`DEBUG_SHOW_BIRD_RAYCASTS` 时画
/// DebugLine）。raycast 命中（任一方向）即可行。
pub fn is_viable_landing_spot(
    raycast: &RaycastWorldHandle,
    pos: Vec3,
    dirs: &[Vec3],
) -> bool {
    if !(0.0..=WREN_MAX_Y).contains(&pos.y) {
        return false;
    }
    let r = (pos.x * pos.x + pos.z * pos.z).sqrt();
    if r > WREN_RADIUS {
        return false;
    }
    dirs.iter().any(|d| {
        crate::shared::raycast_hit(raycast, pos + *d * 1.0, *d).is_some()
    })
}

/// 鹪鹩音频（update_wrens / sim_wren_boids 的 AudioInstance；宿主缝）。
#[derive(Component, Debug, Default)]
pub struct WrenAudio {
    pub handle: AudioHandle,
}
