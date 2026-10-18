//! wren_boids::sim_wren_boids（0x142326a80，fresh 反编译 + 符号实证 2026-10-17）。
//!
//! PDB 系统签名（`InfallibleSystemWrapper` 符号 0x141d82260）：
//! `fn(Raycasting, Res<GameTime>, Res<CritterControl>, Res<AudioCore>,
//!     Res<UiSignifierStream>, ResMut<WrenBoids>, Query<(&mut Transform, &mut Wren)>,
//!     Local<WrenRng>, Query<&BirdSpots>,
//!     Query<(Entity, &mut BirdHaven, &PrefabInstanceData), Without<TreeMarkedForDeletion>>,
//!     Query<(&PrefabInstanceData, &BirdHaven), Added<TreeMarkedForDeletion>>,
//!     Commands)`。
//!
//! 逐段（fresh）：
//! - 顶层门：`byte[*CritterControl+5] == 0` → 清空 WrenBoids（boid/aux Vec 置空）+
//!   逐鹪鹩实体 despawn（`Entities::contains` 门控）。
//! - 出生（dt != 0）：补满至 80（0x50）。位置 `x=u*130-65, y=u*9+1, z=u*130-65`，
//!   速度三分量 `u-0.5`；`spawn<(Wren, NaniBundle<WrenInstanceData>, PrevTransform)>`
//!   （先 `Transform::default()`）；音频 `AudioCore::instantiate_audio`
//!   **"event:/SFX/Birds/takeoff"**（0x14304c383，len 0x18）+ stop_on_drop；
//!   Boid(0x18) + aux(0x38) 记录 push 到 WrenBoids 资源。
//! - FreeSpot：由 `Query<&BirdSpots>` 的未占用点构建 `HashMap<Entity, FreeSpot>`
//!   （key = 认领的鹪鹩实体）；`remove_entry` 释放。认领半径 8.0（64.0 平方）。
//! - 主循环保留条件 `sqrt(x²+y²) <= 91.92388 && y <= 30.0`；否则
//!   `BoidFlock::remove`（三数组 swap_remove + 存活实体 despawn）。
//!   界内 → `Boid::frame_update<wren>`（sep_r=1.3、align=0.1169、min/max=(7,14)）。
//! - 摇树出生（haven 分支）：庇护鹪鹩计时 `-=dt`；`timer<=0 && dist/dt>10` →
//!   log "Tree has been shaken. Spawning birds!" + burst=`floor(rng*5)+1` +
//!   timer=`0.8+rng*5.2`；出生位 = 树心 + normalize(dir)*dist*1.01，
//!   dir=(u-0.5, u*0.5+0.2, u-0.5)。
//!
//! 登记（未闭合）：`FreeSpot` 值的完整字段/认领释放时机、`UiSignifierStream`
//! 调试写入、`Added<TreeMarkedForDeletion>` 分支的具体动作、NaniBundle 实例描述
//! （host render 缝）、closure_3 viable 的 32 向 raycast 表（boids 层已有近似）。

use bevy_ecs::prelude::*;
use country_core::components::TreeMarkedForDeletion;
use country_core::resources::time::GameTime;
use country_core::systems::prefab::PrefabInstanceData;
use fastrand::Rng;
use glam::Vec3;
use std::collections::HashMap;

use crate::audio::{AudioHandle, CritterAudioCore};
use crate::boids::{frame_update, Boid, BoidState, Neighbor};
use crate::components::{
    BirdSpots, CritterControlState, PrevTransform, Transform, Wren,
};
use crate::shared::{legacy_spherecast, RaycastWorldHandle};

/// 鹪鹩出生音频事件（B 字面量 0x14304c383，len 24）。
pub const WREN_TAKEOFF_EVENT: &str = "event:/SFX/Birds/takeoff";

/// `WrenBoids`（B Resource）：Boid Vec（0x18/条）+ aux Vec（0x38/条）+ 实体表。
#[derive(Resource, Default, Debug)]
pub struct WrenBoids {
    pub boids: Vec<Boid>,
    pub states: Vec<BoidState>,
    pub entities: Vec<Entity>,
}

/// `FreeSpot`（B wren_boids::FreeSpot）：停靠点位置。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FreeSpot {
    pub pos: Vec3,
}

/// `Local<WrenRng>`（B wren_boids::WrenRng；fastrand 状态）。
#[derive(Debug)]
pub struct WrenRng(pub Rng);

impl Default for WrenRng {
    fn default() -> Self {
        Self(Rng::with_seed(0x606f))
    }
}

/// 出生上限（0x50=80）与范围常量（0x42b7d907=91.92388、0x41f00000=30.0）。
pub const MAX_WRENS: usize = 80;
pub const WREN_RADIUS: f32 = 91.92388;
pub const WREN_MAX_Y: f32 = 30.0;
/// 认领半径（B `swap_remove` 认领距离 8.0）。
pub const FREE_SPOT_RADIUS: f32 = 8.0;

/// 树庇护（BirdHaven 的 crate 投影；B 类型在 country_core::resources::tree_storage）。
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct BirdHaven {
    /// 冠球（中心 + 半径；树查询产出）。
    pub sphere: (Vec3, f32),
    /// 已入庇护的鹪鹩（pos + 摇树计时）。
    pub wrens: Vec<HavenWren>,
}

/// 树庇护内一只鹪鹩（decomp 0x28-stride 树侧状态：pos + 计时 语义投影）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HavenWren {
    pub entity: Entity,
    pub pos: Vec3,
    /// 摇树计时（@+0x24；`<=0 && 距离/dt>10` → 摇树出生）。
    pub timer: f32,
}

/// 鹪鹩实体 boid 记录（历史 API；B 权威状态在 WrenBoids 资源）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct WrenBoidRecord {
    pub boid: Boid,
    pub state: BoidState,
}

/// 鹪鹩 RNG（历史 Resource；B 为 Local<WrenRng>）。
#[derive(Resource, Debug)]
pub struct WrenRng2(pub std::sync::Mutex<Rng>);

impl Default for WrenRng2 {
    fn default() -> Self {
        Self(std::sync::Mutex::new(Rng::with_seed(0x606f)))
    }
}

/// 鹪鹩开关（历史 Resource；B 门控在 CritterControl+5）。
#[derive(Resource, Clone, Copy, Debug)]
pub struct WrenEnabled(pub bool);

impl Default for WrenEnabled {
    fn default() -> Self {
        Self(true)
    }
}

/// 鹪鹩音频（update_wrens / sim_wren_boids 的 AudioInstance；宿主缝）。
#[derive(Component, Debug, Default)]
pub struct WrenAudio {
    pub handle: AudioHandle,
}

/// closure_3（0x141dc42f0）语义：射线终点 `p + dir*max_dist` 的 y∈[0,30]、
/// 半径 `sqrt(x²+z²)<=91.92388`（glade 标志非 0 时跳过）→ `Raycasting::raycast`
/// （shared 缝：`legacy_spherecast` 命中 = 障碍）。返回"该方向有障碍命中"。
pub fn wren_ray_hits(
    raycast: &RaycastWorldHandle,
    p: Vec3,
    dir: Vec3,
    max_dist: f32,
    glade: bool,
) -> bool {
    let endpoint = p + dir * max_dist;
    if !(0.0..=WREN_MAX_Y).contains(&endpoint.y) {
        return false;
    }
    let r = (endpoint.x * endpoint.x + endpoint.z * endpoint.z).sqrt();
    if !glade && r > WREN_RADIUS {
        return false;
    }
    // closure_3 为 raycast（无球半径）——radius 0.0。
    legacy_spherecast(raycast, p, dir, 0.0).hit
}

/// sim_wren_boids（0x142326a80）。
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn sim_wren_boids(
    time: Res<GameTime>,
    control: Res<CritterControlState>,
    raycast: Option<Res<RaycastWorldHandle>>,
    audio_core: Option<Res<CritterAudioCore>>,
    mut commands: Commands,
    mut flock: ResMut<WrenBoids>,
    mut rng: Local<WrenRng>,
    mut wrens: Query<(&mut Transform, &mut Wren)>,
    spots_q: Query<&BirdSpots>,
    mut havens: Query<
        (Entity, &mut BirdHaven, &PrefabInstanceData),
        Without<TreeMarkedForDeletion>,
    >,
    removed: Query<(&PrefabInstanceData, &BirdHaven), Added<TreeMarkedForDeletion>>,
) {
    let dt = time.delta;
    let r = &mut rng.0;

    // ── 顶层门（O：`byte[*CritterControl+5] == 0`）─────────────────────────────
    if !control.birds_enabled {
        flock.boids.clear();
        flock.states.clear();
        for entity in flock.entities.drain(..) {
            if wrens.get_mut(entity).is_ok() {
                commands.entity(entity).despawn();
            }
        }
        return;
    }

    // ── 出生（dt != 0，补满 80）───────────────────────────────────────────────
    if dt != 0.0 {
        while flock.entities.len() < MAX_WRENS {
            let pos = Vec3::new(
                r.f32() * 130.0 - 65.0,
                r.f32() * 9.0 + 1.0,
                r.f32() * 130.0 - 65.0,
            );
            let vel = Vec3::new(r.f32() - 0.5, r.f32() - 0.5, r.f32() - 0.5);
            let mut spawn = commands.spawn((
                Transform::default(),
                Wren {
                    pos,
                    prev_pos: pos,
                    phase: 0.0,
                    hop_timer: 0.0,
                },
                PrevTransform::default(),
            ));
            if let Some(core) = audio_core.as_ref() {
                spawn.insert(WrenAudio {
                    handle: core.instantiate(WREN_TAKEOFF_EVENT),
                });
            }
            flock.boids.push(Boid { pos, vel });
            flock.states.push(BoidState::default());
            flock.entities.push(spawn.id());
        }
    }

    // ── FreeSpot 池 + 认领表（O：HashMap<Entity, FreeSpot>）───────────────────
    let mut free_spots: Vec<Vec3> = spots_q
        .iter()
        .flat_map(|spots| {
            spots
                .spots
                .iter()
                .filter(|s| !s.occupied)
                .map(|s| s.pos)
        })
        .collect();
    let mut claims: HashMap<Entity, FreeSpot> = HashMap::new();

    // ── 主循环：保留条件 / goal 认领 / frame_update ───────────────────────────
    let mut removal: Vec<usize> = Vec::new();
    for i in 0..flock.boids.len() {
        let b = flock.boids[i];
        let keep = (b.pos.x * b.pos.x + b.pos.y * b.pos.y).sqrt() <= WREN_RADIUS
            && b.pos.y <= WREN_MAX_Y;
        if !keep {
            removal.push(i);
            continue;
        }
        let entity = flock.entities[i];
        // goal：未认领 → 最近 FreeSpot（<=8）；已认领 → 保持（抵达/释放时机 U）。
        let goal = if let Some(spot) = claims.get(&entity) {
            Some(spot.pos)
        } else {
            let mut best: Option<(usize, f32)> = None;
            for (j, &s) in free_spots.iter().enumerate() {
                let d = s.distance_squared(b.pos);
                if d <= FREE_SPOT_RADIUS * FREE_SPOT_RADIUS
                    && best.map_or(true, |(_, bd)| d < bd)
                {
                    best = Some((j, d));
                }
            }
            best.map(|(j, _)| {
                let pos = free_spots.swap_remove(j);
                claims.insert(entity, FreeSpot { pos });
                pos
            })
        };
        let mut boid = b;
        let mut state = flock.states[i];
        state.goal = goal;
        let neighbors: Vec<Neighbor> = flock
            .boids
            .iter()
            .map(|n| Neighbor {
                pos: n.pos,
                vel: n.vel,
            })
            .collect();
        let viable = |p: Vec3| -> bool {
            match raycast.as_ref() {
                Some(ray) => !wren_ray_hits(ray, p, Vec3::NEG_Y, 4.0, false),
                None => true,
            }
        };
        frame_update(
            &mut boid,
            &mut state,
            &neighbors,
            i,
            1.3,
            0.1169,
            7.0,
            14.0,
            dt,
            r,
            &viable,
        );
        flock.boids[i] = boid;
        flock.states[i] = state;
        if let Ok((mut transform, mut wren)) = wrens.get_mut(entity) {
            wren.prev_pos = wren.pos;
            wren.pos = boid.pos;
            transform.translation = boid.pos;
        }
    }

    // ── 移除（BoidFlock::remove：三数组 swap_remove + despawn）────────────────
    for &i in removal.iter().rev() {
        let entity = flock.entities.swap_remove(i);
        flock.boids.swap_remove(i);
        flock.states.swap_remove(i);
        if let Some(spot) = claims.remove(&entity) {
            free_spots.push(spot.pos);
        }
        if wrens.get_mut(entity).is_ok() {
            commands.entity(entity).despawn();
        }
    }

    // ── 摇树出生（O：haven 分支）──────────────────────────────────────────────
    for (_, mut haven, _prefab) in &mut havens {
        // 关联维护：群内落在冠球内且未入列的鹪鹩加入（近似；B 侧关联结构 U）。
        for i in 0..flock.boids.len() {
            let b = flock.boids[i];
            let inside = (b.pos - haven.sphere.0).length() <= haven.sphere.1;
            let entity = flock.entities[i];
            if inside && !haven.wrens.iter().any(|w| w.entity == entity) {
                haven.wrens.push(HavenWren {
                    entity,
                    pos: b.pos,
                    timer: 0.0,
                });
            }
        }
        haven.wrens.retain(|w| wrens.get_mut(w.entity).is_ok());
        let sphere = haven.sphere.0;
        for w in &mut haven.wrens {
            if let Some(idx) = flock.entities.iter().position(|e| *e == w.entity) {
                w.pos = flock.boids[idx].pos;
            }
            w.timer -= dt;
            let dist = (w.pos - sphere).length();
            if w.timer <= 0.0 && dist / dt.max(1e-10) > 10.0 {
                log::info!("Tree has been shaken. Spawning birds!");
                w.timer = 0.8 + r.f32() * 5.2;
                let burst = 1 + (r.f32() * 5.0) as usize;
                for _ in 0..burst {
                    if flock.entities.len() >= MAX_WRENS {
                        break;
                    }
                    let dir = Vec3::new(r.f32() - 0.5, r.f32() * 0.5 + 0.2, r.f32() - 0.5)
                        .normalize_or_zero();
                    let pos = sphere + dir * dist * 1.01;
                    let mut spawn = commands.spawn((
                        Transform::default(),
                        Wren {
                            pos,
                            prev_pos: pos,
                            phase: 0.0,
                            hop_timer: 0.0,
                        },
                        PrevTransform::default(),
                    ));
                    if let Some(core) = audio_core.as_ref() {
                        spawn.insert(WrenAudio {
                            handle: core.instantiate(WREN_TAKEOFF_EVENT),
                        });
                    }
                    flock.boids.push(Boid { pos, vel: dir });
                    flock.states.push(BoidState::default());
                    flock.entities.push(spawn.id());
                }
            }
        }
    }
    // `Added<TreeMarkedForDeletion>` 分支：树被标记删除时释放该冠球内认领（B 动作 U，
    // 至少释放停靠点以免悬挂）。prefab/haven 数据仅用于定位。
    for (_prefab, haven) in &removed {
        let center = haven.sphere.0;
        let radius = haven.sphere.1;
        let release: Vec<Entity> = claims
            .iter()
            .filter(|(_, spot)| (spot.pos - center).length() <= radius)
            .map(|(e, _)| *e)
            .collect();
        for e in release {
            if let Some(spot) = claims.remove(&e) {
                free_spots.push(spot.pos);
            }
        }
    }
}
