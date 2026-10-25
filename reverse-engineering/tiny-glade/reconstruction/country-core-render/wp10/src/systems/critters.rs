//! `systems::critters` — 小动物系统（B 骨架 `systems/critters` crate）。
//!
//! 模块结构按骨架对齐：shared / firefly / butterfly / wren / wren_boids / boids / duck /
//! duck_splash_particle / ducks_react_to_being_inside_buildings / ui_pet_ducks / fish /
//! spawn_ducks。反编译地址见 `contracts/critters.md` §2。语义核心复用
//! `reconstruction/critters`（reva/radare2 反汇编钉死，B 骨架名核对）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::entity::Entity;
use bevy_ecs::query::{With, Without};
use bevy_ecs::system::{Query, Res};
use bevy_ecs::system::Commands;
use glam::{Quat, Vec3};
use perchance::PerchanceContext;

use crate::resources::critters::{
    BirdSpots, Critter, CritterControl, CritterGoal, CritterLocomotion, CritterRng,
    CritterVelocityMultiplier, CritterVelocityTarget, Duck, DuckState, Firefly, Velocity, Wren,
};
use crate::resources::render_loop_critters::{ButterflySsbo, FireflySsbo};
use crate::systems::support::{GameTime, Transform};
use crate::typed_gap::Gap;

pub const FIREFLY_COUNT: usize = 96;
pub const BUTTERFLY_COUNT: usize = 24;

/// 原插件对 `resources::critters` 注入的 perchance 种子（B 0x14067d537，O）。
pub const CRITTER_RNG_SEED: u128 = 0x606f;

/// `utils::uniform_sample_sphere`（B 0x140c907d0，O）；标准数学复用。
pub fn sample_sphere_game(u: f32, v: f32) -> Vec3 {
    let z = 1.0 - (u + u);
    let radial_squared = 1.0 - z * z;
    let radial = if radial_squared <= 0.0 { 0.0 } else { radial_squared }.sqrt();
    let angle = v * std::f32::consts::TAU;
    Vec3::new(radial * angle.cos(), radial * angle.sin(), z)
}

// ---------------------------------------------------------------------------
// shared：移动 / 避让 / 速度平滑（source shared.rs；VA 见契约）
// ---------------------------------------------------------------------------
pub mod shared {
    use super::*;

    /// `smooth_critter_velocities` 0x142318c10（O）：`retention = retention_per_second.powf(dt)`，
    /// `vel = vel*retention + target.dir*(1-retention)`（原顺序非 lerp 重排）。
    pub fn smooth_velocity(velocity: &mut Vec3, target: &CritterVelocityTarget, dt: f32) {
        let retention = target.retention_per_second.powf(dt);
        let weight = 1.0 - retention;
        *velocity = Vec3::new(
            velocity.x * retention + target.direction.x * weight,
            velocity.y * retention + target.direction.y * weight,
            velocity.z * retention + target.direction.z * weight,
        );
    }

    /// 单实体移动核心（move_critters 0x142318fa0 内层，O）：`y = max(y,0)`；
    /// `pos += vel * dt * multiplier`（Multiplier 缺省 1.0）。
    pub fn move_critter_core(pos: &mut Vec3, vel: Vec3, multiplier: f32, dt: f32, allow_move: bool) {
        pos.y = pos.y.max(0.0);
        if allow_move {
            *pos += vel * (dt * multiplier);
        }
    }

    /// 朝向更新核心（O）：`rot = nlerp(rot, arc(dir, Y), 1 - 0.01^dt)`。
    pub fn face_movement(rot: &mut Quat, move_dir: Vec3, dt: f32) {
        let decay = 0.01_f32.powf(dt);
        let weight = 1.0 - decay;
        let target = Quat::from_rotation_arc(move_dir.normalize_or_zero(), Vec3::Y);
        *rot = rot.lerp(target, weight).normalize();
    }

    /// `critters_avoid_collisions` 0x1423179a0 避让决策（O/I，shared.rs 草稿）：
    /// 水/岩石/墙体/spherecast 命中或目标仍 ≥1.0 远 → 保留目标；`<0.1` 命中 → despawn；
    /// 否则清 `CritterGoal.active`。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AvoidanceAction {
        KeepMoving,
        Arrived,
        Despawn,
    }

    pub fn decide_avoidance(
        wall_blocked: bool,
        water_blocked: bool,
        sphere_hit: bool,
        sphere_dist: f32,
    ) -> AvoidanceAction {
        if wall_blocked || water_blocked {
            return AvoidanceAction::KeepMoving;
        }
        if sphere_hit || sphere_dist >= 1.0 {
            return AvoidanceAction::KeepMoving;
        }
        if sphere_dist < f32::from_bits(0x3ecccccd) {
            AvoidanceAction::Despawn
        } else {
            AvoidanceAction::Arrived
        }
    }
}

// ---------------------------------------------------------------------------
// firefly（source firefly.rs；VA：update_goals 0x142316940 / set_vel 0x142317040 /
//          send_to_gpu 0x142316760）
// ---------------------------------------------------------------------------
pub mod firefly {
    use super::*;

    /// 每萤火虫目标刷新核心（B 0x142316d79..0x142316fc2，O）。控制关闭时不消耗 RNG。
    pub fn update_goal(
        firefly: &mut Firefly,
        goal: &mut CritterGoal,
        position: Vec3,
        velocity: Vec3,
        dt: f32,
        control_enabled: bool,
        rng: &mut PerchanceContext,
    ) {
        if !control_enabled {
            return;
        }
        firefly.goal_timer -= dt;
        let arrived = if goal.active {
            let d = goal.pos - position;
            // 原 SSE：y²+x² 再加 z²（非近似距离）。
            (d.y * d.y + d.x * d.x) + d.z * d.z < f32::from_bits(0x3c23d70b)
        } else {
            true
        };
        if !(firefly.goal_timer <= 0.0 || arrived) {
            return;
        }
        let inertia = if !goal.active { -0.5 } else { 0.5 };
        // 严格三次 `uniform_f32`，共享资源非每萤火虫。
        firefly.goal_timer = rng.uniform_f32() * 0.15 + 0.05;
        let u = rng.uniform_f32();
        let v = rng.uniform_f32();
        let sample = sample_sphere_game(u, v);
        let biased = Vec3::new(
            velocity.x * inertia + sample.x,
            sample.y * 0.5,
            velocity.z * inertia + sample.z,
        );
        let direction = biased.normalize_or_zero();
        let proposed = Vec3::new(
            firefly.goal_timer * direction.x + position.x,
            firefly.goal_timer * direction.y + position.y,
            direction.z * firefly.goal_timer + position.z,
        );
        let radius = (proposed.z * proposed.z + proposed.x * proposed.x).sqrt();
        let old_radius = (position.z * position.z + position.x * position.x).sqrt();
        goal.active = true;
        goal.pos = if radius < 65.0 || radius < old_radius {
            proposed
        } else {
            position
        };
    }

    /// 目标 → 速度方向（B 0x142317430..0x142317552，O）：无随机调用，multiplier 恒重置 1；
    /// 低空（y<1）把方向 y 抬到 ≥0.25 且不重归一化。
    pub fn set_velocity_target(
        position: Vec3,
        goal: &CritterGoal,
        target: &mut CritterVelocityTarget,
        multiplier: &mut f32,
    ) {
        let d = if goal.active { goal.pos - position } else { Vec3::ZERO };
        target.direction = d.normalize_or_zero();
        *multiplier = 1.0;
        if position.y < 1.0 {
            target.direction.y = target.direction.y.max(0.25);
        }
    }

    /// `update_firefly_goals`（0x142316940，O）：`CritterControl.+4==1` 门控 + 逐实体刷新目标。
    #[allow(clippy::too_many_arguments)]
    pub fn update_firefly_goals(
        control: &CritterControl,
        time: &GameTime,
        rng: &mut PerchanceContext,
        mut query: Query<(Entity, &Transform, &Firefly, &Velocity, &mut CritterGoal)>,
    ) {
        if !control.control_enabled {
            return;
        }
        let dt = time.delta_seconds;
        for (_e, tf, firefly, vel, mut goal) in query.iter_mut() {
            let mut f = *firefly;
            update_goal(
                &mut f, &mut goal, tf.translation, vel.0, dt, true, rng,
            );
        }
    }

    /// `set_firefly_velocities_towards_goals`（0x142317040，O）：
    /// 查询 `(Transform,<critter>,CritterGoal,&mut CritterVelocityTarget,&mut CritterVelocityMultiplier)`。
    pub fn set_firefly_velocities_towards_goals(
        mut query: Query<(
            &Transform,
            &CritterGoal,
            &mut CritterVelocityTarget,
            &mut CritterVelocityMultiplier,
        )>,
    ) {
        for (tf, goal, mut target, mut mult) in query.iter_mut() {
            set_velocity_target(tf.translation, goal, &mut target, &mut mult.0);
        }
    }

    /// `fireflies_send_to_gpu`（0x142316760，O）：`Vec<FireflySsbo>` from
    /// `Query<(Transform,Firefly)>`；断言 96；`cpu_write<FireflySsbo>`（GPU 写为 rhapsody 复用面）。
    pub fn fireflies_send_to_gpu(query: Query<(&Transform, &Firefly)>, _out: &mut Vec<FireflySsbo>) {
        let _count = query.iter().count();
        debug_assert_eq!(_count, FIREFLY_COUNT, "firefly GPU cap 96");
        let _ = query.iter().map(|(_tf, _f)| FireflySsbo(crate::typed_gap::Gap)).collect::<Vec<_>>();
    }
}

// ---------------------------------------------------------------------------
// butterfly（source butterfly.rs；VA 见契约）
// ---------------------------------------------------------------------------
pub mod butterfly {
    use super::*;

    /// `update_butterfly_states`（0x141d84d40，O 名；姿态/拍翅状态机，细节 U）。
    pub fn update_butterfly_states(
        _time: &GameTime,
        _query: Query<(&Transform, &crate::resources::critters::Butterfly)>,
    ) {
    }

    /// `update_butterfly_goals`（0x141d86060，O；同 firefly 目标刷新）。
    pub fn update_butterfly_goals(
        _control: &CritterControl,
        _time: &GameTime,
        _query: Query<(
            Entity,
            &Transform,
            &crate::resources::critters::Butterfly,
            &Velocity,
            &mut CritterGoal,
        )>,
    ) {
    }

    /// `set_butterfly_velocities_towards_goals`（0x141d867b0，O；同 firefly 速度定向）。
    pub fn set_butterfly_velocities_towards_goals(
        mut query: Query<(
            &Transform,
            &CritterGoal,
            &mut CritterVelocityTarget,
            &mut CritterVelocityMultiplier,
        )>,
    ) {
        for (tf, goal, mut target, mut mult) in query.iter_mut() {
            firefly::set_velocity_target(tf.translation, goal, &mut target, &mut mult.0);
        }
    }

    /// `update_butterfly_audio`（0x141d86eb0，O；FMOD 位置同步）。
    pub fn update_butterfly_audio(
        _query: Query<(
            &Transform,
            &mut crate::resources::critters::ButterflyAudio,
        )>,
    ) {
    }

    /// `butterflies_send_to_gpu`（0x141d87140，O）：24 蝴蝶 → `cpu_write<ButterflySsbo>`。
    pub fn butterflies_send_to_gpu(
        query: Query<(&Transform, &crate::resources::critters::Butterfly)>,
        _out: &mut Vec<ButterflySsbo>,
    ) {
        let _count = query.iter().count();
        debug_assert_eq!(_count, BUTTERFLY_COUNT, "butterfly GPU cap 24");
    }
}

// ---------------------------------------------------------------------------
// wren（source wren.rs；VA：update_wrens 0x14231e390 / roof 0x14231d6c0 /
//          decorator 0x14231c7d0）
// ---------------------------------------------------------------------------
pub mod wren {
    use super::*;
    use crate::resources::critters::{BirdSpotManager, BirdSpots};

    /// `update_wrens`（0x14231e390，O）：`Query<(Entity,&mut Transform,&mut Wren)>`。
    pub fn update_wrens(
        _time: &GameTime,
        _query: Query<(Entity, &mut Transform, &mut Wren)>,
    ) {
    }

    /// `update_roof_bird_spots`（0x14231d6c0，O）：`SittableForBirds + Transform` → 屋顶鸟位；
    /// `BirdSpotManager` gen_id/create_spots/recycle（闭包 ×3）。
    pub fn update_roof_bird_spots(
        _manager: &mut BirdSpotManager,
        _query: Query<(&crate::components::SittableForBirds, &Transform)>,
        _spots: Query<&mut BirdSpots>,
    ) {
    }

    /// `update_decorator_bird_spots`（0x14231c7d0，O）：装饰物鸟位。
    pub fn update_decorator_bird_spots(
        _manager: &mut BirdSpotManager,
        _query: Query<(&crate::components::SittableForBirds, &Transform)>,
    ) {
    }
}

// ---------------------------------------------------------------------------
// boids（source boids.rs；Boid/BoidAux/BoidFlock）
// ---------------------------------------------------------------------------
pub mod boids {
    use super::*;

    /// 单体 boid（O 名；字段 U）。
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Boid(pub Gap);

    /// boid 辅助状态（O 名；字段 U）。
    #[derive(Clone, Copy, Debug, Default)]
    pub struct BoidAux(pub Gap);

    /// boid 群容器（O 名；push/clone_state/iter_mut/len/remove/drain）。
    #[derive(Clone, Debug, Default)]
    pub struct BoidFlock {
        pub boids: Vec<Boid>,
    }

    impl BoidFlock {
        pub fn push(&mut self, b: Boid) {
            self.boids.push(b);
        }
        pub fn len(&self) -> usize {
            self.boids.len()
        }
        pub fn is_empty(&self) -> bool {
            self.boids.is_empty()
        }
        pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Boid> {
            self.boids.iter_mut()
        }
        pub fn drain(&mut self) -> impl Iterator<Item = Boid> + '_ {
            self.boids.drain(..)
        }
        /// `BoidFlock::remove<closure_env>`（O：fish/wren_boids 单态）。
        pub fn remove<F>(&mut self, mut f: F)
        where
            F: FnMut(&Boid) -> bool,
        {
            self.boids.retain(|b| !f(b));
        }
    }

    /// `Boid::new` / `random_vel_change`（0x141dc2170，O 名）/ `apply_rules` /
    /// `unobstructed_dir` / `frame_update`（0x141dc3510）：boids 规则（聚集/分离/对齐）。
    /// 权重与阈值 U（typed gap）。
    pub struct BoidRules {
        pub _gap: Gap,
    }

    pub fn apply_rules(_flock: &BoidFlock, _b: &mut Boid) {}
    pub fn unobstructed_dir(_b: &Boid) -> Vec3 {
        Vec3::ZERO
    }
    pub fn random_vel_change(_b: &mut Boid, _rng: &mut PerchanceContext) {}
    pub fn frame_update(_b: &mut Boid, _dt: f32) {}
}

// ---------------------------------------------------------------------------
// wren_boids（source wren_boids.rs；WrenBoids/WrenRng/FreeSpot，sim 0x142326a80）
// ---------------------------------------------------------------------------
pub mod wren_boids {
    use super::*;
    use crate::resources::critters::{BirdSpotManager, BirdSpots, Wren};
    use boids::BoidFlock;

    /// WrenBoids 资源（O 名；字段 U）。
    #[derive(Default)]
    pub struct WrenBoids {
        pub flock: BoidFlock,
    }

    /// WrenRng 本地资源（O 名）。
    pub struct WrenRng(pub PerchanceContext);

    impl Default for WrenRng {
        fn default() -> Self {
            WrenRng(PerchanceContext::new(CRITTER_RNG_SEED))
        }
    }

    /// FreeSpot（O 名；着陆点，字段 U）。
    #[derive(Clone, Copy, Debug, Default)]
    pub struct FreeSpot(pub Gap);

    /// `sim_wren_boids`（0x142326a80，O）：boid 模拟 + 着陆点判定。
    pub fn sim_wren_boids(
        _time: &GameTime,
        _boids: &mut WrenBoids,
        _rng: &mut WrenRng,
        _spots: &mut BirdSpotManager,
        _query: Query<(&Transform, &mut Wren, &mut BirdSpots)>,
    ) {
    }

    /// `is_viable_landing_spot`（O 名；U 阈值）。
    pub fn is_viable_landing_spot(_spot: FreeSpot) -> bool {
        false
    }

    /// `sim_wren_boids::lerp`（O 名；U）。
    pub fn lerp(_a: f32, _b: f32, _t: f32) -> f32 {
        _a + (_b - _a) * _t
    }

    /// `despawn_birds`（O 名）。
    pub fn despawn_birds(_commands: Commands, _query: Query<(Entity, &Wren)>) {}
}

// ---------------------------------------------------------------------------
// duck（source duck.rs；VA 见契约）
// ---------------------------------------------------------------------------
pub mod duck {
    use super::*;

    /// `PlanPurpose`（O 名；`is_for_entity`）。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct PlanPurpose(pub u64);

    impl PlanPurpose {
        pub fn is_for_entity(&self, _entity: u64) -> bool {
            self.0 == _entity
        }
    }

    /// 鸭子目标刷新核心（update_duck_goals 0x142313b30，duck.rs 已恢复）。
    #[allow(clippy::too_many_arguments)]
    pub fn update_duck_goal(
        moving: &mut bool,
        dwell: &mut [f32; 2],
        goal: &mut CritterGoal,
        pos: Vec3,
        dt: f32,
        rng: &mut PerchanceContext,
        mut pick_new: impl FnMut(&mut PerchanceContext) -> Option<Vec3>,
    ) {
        if !*moving {
            dwell[0] -= dt;
            if dwell[0] <= 0.0 {
                let u1 = rng.uniform_f32();
                let u2 = rng.uniform_f32();
                dwell[0] = u1 * 3.0 + 1.0;
                dwell[1] = u2 * 3.0 + 1.0;
                *moving = true;
            }
            return;
        }
        dwell[0] -= dt;
        dwell[1] -= dt;
        if dwell[0] <= 0.0 && dwell[1] <= 0.0 {
            let u = rng.uniform_f32();
            dwell[1] = u * 0.5 + 0.5;
            *moving = false;
            return;
        }
        let arrived = if goal.active {
            let d = goal.pos - pos;
            (d.x * d.x + d.y * d.y + d.z * d.z) < f32::from_bits(0x3e23d70b)
        } else {
            true
        };
        if arrived {
            if let Some(np) = pick_new(rng) {
                goal.active = true;
                goal.pos = np;
            } else {
                goal.active = false;
            }
        }
    }

    /// `set_duck_velocities_towards_goals`（0x142314470）核心：仅 `DuckState==2` 且 moving 用目标方向。
    pub fn set_velocity_towards_goal(
        goal: &CritterGoal,
        duck_moving: bool,
        state: DuckState,
        pos: Vec3,
        target: &mut CritterVelocityTarget,
    ) {
        let d = if goal.active && duck_moving && state.0 == 2 {
            goal.pos - pos
        } else {
            Vec3::ZERO
        };
        target.direction = d.normalize_or_zero();
    }

    /// `move_ducks_along_trajectories`（0x142312200）核心：`progress += dir*12.0*dt/segment_len`。
    pub fn advance_trajectory(progress: f32, speed_dir: f32, segment_len: f32, dt: f32) -> f32 {
        progress + (speed_dir * 12.0 * dt) / segment_len
    }

    /// `plan_duck_trajectories`（0x14230ff00，O；闭包 ×8；曲线索引/Frenet 姿态 U）。
    pub fn plan_duck_trajectories(
        _time: &GameTime,
        _query: Query<(Entity, &Transform, &Duck, &mut CritterGoal)>,
    ) {
    }

    /// `update_duck_ripples`（0x142314b70，O 名；水波特效算法 U）。
    pub fn update_duck_ripples(_time: &GameTime, _query: Query<(&Transform, &Duck)>) {}
}

// ---------------------------------------------------------------------------
// duck_splash_particle / ducks_react_to_being_inside_buildings / ui_pet_ducks
// ---------------------------------------------------------------------------
pub mod duck_splash_particle {
    use super::*;
    /// `duck_splash_particle`（O 名；水花粒子发射）。
    pub fn duck_splash_particle(
        _commands: Commands,
        _query: Query<(&Transform, &Duck, &Velocity)>,
    ) {
    }
}

pub mod ducks_react_to_being_inside_buildings {
    use super::*;
    /// `ducks_react_to_being_inside_buildings`（O 名；闭包 ×2）。
    pub fn ducks_react_to_being_inside_buildings(
        _query: Query<(&Transform, &Duck)>,
        _control: Res<CritterControl>,
    ) {
    }
}

pub mod ui_pet_ducks {
    use super::*;
    /// `ui_pet_ducks`（O 名；UI 抚摸鸭子，闭包 ×2）。
    pub fn ui_pet_ducks(
        _time: &GameTime,
        _query: Query<(&Transform, &Duck)>,
    ) {
    }
}

// ---------------------------------------------------------------------------
// fish（source fish.rs；FishSpawnLocations::new 0x140f909f0 / fish 0x140f90cd0）
// ---------------------------------------------------------------------------
pub mod fish {
    use super::*;
    use crate::resources::critters::Fish;

    /// 鱼生成位置（O：0x140f909f0 25×25 网格，-52..52，625 点，每点 12B）。
    #[derive(Clone, Copy, Debug, Default)]
    pub struct FishSpawnLocation {
        pub x: f32,
        pub z: f32,
    }

    pub struct FishSpawnLocations {
        pub points: Vec<FishSpawnLocation>,
    }

    impl FishSpawnLocations {
        /// `FishSpawnLocations::new`（0x140f909f0，O）：`x = (row/24)*52 - (1-row/24)*52`，
        /// `z = (col/24)*52 - (1-col/24)*52`。
        pub fn new() -> Self {
            let mut points = Vec::with_capacity(25 * 25);
            for row in 0..25usize {
                let t = row as f32 / 24.0;
                let x = t * 52.0 - (1.0 - t) * 52.0;
                for col in 0..25usize {
                    let u = col as f32 / 24.0;
                    let z = u * 52.0 - (1.0 - u) * 52.0;
                    points.push(FishSpawnLocation { x, z });
                }
            }
            FishSpawnLocations { points }
        }
    }

    impl Default for FishSpawnLocations {
        fn default() -> Self {
            Self::new()
        }
    }

    /// `spawn_fish` / `despawn_fish`（0x140f90c10 / O 名）。
    pub fn spawn_fish(_commands: Commands, _locations: &FishSpawnLocations) {}
    pub fn despawn_fish(_commands: Commands, _query: Query<Entity, With<Fish>>) {}

    /// `fish`（0x140f90cd0，O）：Boid 驱鱼（watermask 范围）。
    pub fn fish(
        _time: &GameTime,
        _flock: &mut crate::systems::critters::boids::BoidFlock,
        _query: Query<(&Transform, &Fish)>,
    ) {
    }
}

// ---------------------------------------------------------------------------
// spawn_ducks（source spawn_ducks.rs；new 0x140f8ee90 / spawn 0x140f8f210 /
//              despawn 0x140f8f8b0）
// ---------------------------------------------------------------------------
pub mod spawn_ducks {
    use super::*;
    use crate::resources::critters::Duck;

    /// 鸭子生成位置（O：0x140f8ee90 9 列 × 4 行网格，12B/点；x = t*52-(1-t)*52, t=i/9；
    /// z ∈ {-52, -40.22, -28.9, -18.66}）。
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DuckSpawnLocation {
        pub x: f32,
        pub z: f32,
    }

    pub struct DuckSpawnLocations {
        pub points: Vec<DuckSpawnLocation>,
    }

    impl DuckSpawnLocations {
        /// `DuckSpawnLocations::new`（0x140f8ee90，O）。
        pub fn new() -> Self {
            const ZS: [f32; 4] = [-52.0, -40.22, -28.9, -18.66];
            let mut points = Vec::with_capacity(9 * 4);
            for i in 0..9usize {
                let t = i as f32 / 9.0;
                let x = t * 52.0 - (1.0 - t) * 52.0;
                for &z in &ZS {
                    points.push(DuckSpawnLocation { x, z });
                }
            }
            DuckSpawnLocations { points }
        }
    }

    impl Default for DuckSpawnLocations {
        fn default() -> Self {
            Self::new()
        }
    }

    /// `spawn_ducks`（0x140f8f210，O）：门控 demo 模式关 且 存档未损坏。
    pub fn spawn_ducks(
        _commands: Commands,
        _locations: &DuckSpawnLocations,
        _demo_mode: bool,
        _save_corrupted: bool,
    ) {
        if !(_demo_mode || _save_corrupted) {
            // 生成 Duck（`SpawnCommand` 族，payload U）。
        }
    }

    /// `despawn_ducks`（0x140f8f8b0，O）。
    pub fn despawn_ducks(_commands: Commands, _query: Query<Entity, With<Duck>>) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::critters::CritterVelocityTarget;
    use shared::move_critter_core;
    use shared::smooth_velocity;

    #[test]
    fn smooth_velocity_matches_firefly_core() {
        let mut vel = Vec3::ONE;
        let target = CritterVelocityTarget {
            direction: Vec3::new(0.0, 0.0, 1.0),
            retention_per_second: 0.05,
        };
        smooth_velocity(&mut vel, &target, 1.0);
        assert!(vel.z > 0.9);
        assert!(vel.x < 0.1);
    }

    #[test]
    fn move_critter_scales_by_dt_times_multiplier() {
        let mut p = Vec3::ZERO;
        move_critter_core(&mut p, Vec3::new(2.0, 0.0, 0.0), 0.5, 0.25, true);
        assert_eq!(p, Vec3::new(0.25, 0.0, 0.0));
        // allow_move=false → 不动。
        move_critter_core(&mut p, Vec3::new(2.0, 0.0, 0.0), 1.0, 0.25, false);
        assert_eq!(p, Vec3::new(0.25, 0.0, 0.0));
    }

    #[test]
    fn duck_goal_tick_picks_new() {
        let mut rng = PerchanceContext::new(CRITTER_RNG_SEED);
        let mut moving = true;
        let mut dwell = [2.0, 2.0];
        let mut goal = CritterGoal {
            active: false,
            pos: Vec3::ZERO,
        };
        duck::update_duck_goal(
            &mut moving,
            &mut dwell,
            &mut goal,
            Vec3::ZERO,
            0.016,
            &mut rng,
            |_| None,
        );
        assert!(!goal.active);
    }

    #[test]
    fn duck_spawn_locations_grid() {
        let locs = spawn_ducks::DuckSpawnLocations::new();
        assert_eq!(locs.points.len(), 9 * 4);
        // x = t*52 - (1-t)*52，t = i/9（i∈0..8）→ 首列 -52，末列 (7/9)*52 ≈ 40.44。
        assert!((locs.points[0].x - (-52.0)).abs() < 0.01);
        assert!((locs.points[35].x - (7.0 / 9.0 * 52.0)).abs() < 0.01);
        // z 行序 -52 / -40.22 / -28.9 / -18.66。
        assert!((locs.points[1].z - (-40.22)).abs() < 0.01);
        assert!((locs.points[3].z - (-18.66)).abs() < 0.01);
    }

    #[test]
    fn fish_spawn_locations_grid() {
        let locs = fish::FishSpawnLocations::new();
        assert_eq!(locs.points.len(), 25 * 25);
    }

    #[test]
    fn duck_velocity_zero_when_inactive() {
        let mut t = CritterVelocityTarget {
            direction: Vec3::ONE,
            retention_per_second: 0.05,
        };
        duck::set_velocity_towards_goal(
            &CritterGoal {
                active: false,
                pos: Vec3::ONE,
            },
            true,
            DuckState(2),
            Vec3::ZERO,
            &mut t,
        );
        assert_eq!(t.direction, Vec3::ZERO);
    }
}
