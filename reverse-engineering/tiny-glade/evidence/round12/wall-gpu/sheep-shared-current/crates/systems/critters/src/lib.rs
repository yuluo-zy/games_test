//! system-critters（B 主基线）：鸭子 / 羊 / 蝴蝶 / 萤火虫 / 鱼 / 鹪鹩行为状态机。
//!
//! 证据链（B VA）：
//! - duck 五段链：plan_duck_trajectories 0x14230ff00 → move_ducks_along_trajectories
//!   0x142312200 → update_duck_goals 0x142313b30 → set_duck_velocities_towards_goals
//!   0x142314470 → update_duck_ripples 0x142314b70（critters-behavior-analysis.md）。
//! - shared：critters_avoid_collisions 0x1423179a0 / smooth_critter_velocities 0x142318c10 /
//!   move_critters 0x142318fa0（同上）。
//! - butterfly：0x141d84d40/0x141d86060/0x141d867b0/0x141d86eb0/0x141d87140；
//!   firefly：0x142316940/0x142317040/0x142316760；fish 0x140f90cd0（critters-extra-analysis.md）。
//! - wren：update_decorator_bird_spots 0x14231c7d0 / update_roof_bird_spots 0x14231d6c0 /
//!   update_wrens 0x14231e390；sim_wren_boids 0x142326a80。
//! - sheep：sheep_behavior 0x142315050/0x142315e40/0x142316490、sheep_terrain 0x14231bd80、
//!   sheep_walk_animation 0x14231a630、sheep_pet_animation 0x14231ac20、sheep_floaties
//!   0x142320cf0、sheep_umbrellas 0x142321980、sheep_unstuck 0x1423246b0、
//!   sheep_input_record_replay 0x14231b720、react_to_being_inside_buildings 0x14231f610、
//!   sheep_reaction_emoji 0x14231ff70、ui_pet_sheep 0x142324f80。
//! - spawn/despawn：spawn_ducks 0x140f8f210 / despawn_ducks 0x140f8f8b0 /
//!   DuckSpawnLocations::new 0x140f8ee90 / FishSpawnLocations::new 0x140f909f0 /
//!   fish 0x140f90cd0 / despawn_fish 0x140f90c10；插件 build 0x14067d440（CritterPlugin）。
//!
//! 时序：dt 一律读 `country_core::resources::time::GameTime`（`*Res<Time>+0x28` 的 f32 delta）。
//! RNG 二分：goals 系用 `perchance::PerchanceContext::uniform_f32`（0x141625ae0；
//! 插件 build 传 B 种子 0xc0df），boids 随机转向与出生位置用 `fastrand::Rng::f32`
//! （critters-extra-analysis 横切证据）。
//!
//! GPU 上传（fireflies_send_to_gpu / butterflies_send_to_gpu）只做实例数据计算：
//! 元素类型消费 country-core `render_loop::resources::critters` 的
//! `FireflySsbo`（96 上限）/ `ButterflySsbo`（24 上限，anim_t = Butterfly+0x14 phase_ref），
//! 缓冲写入与绘制由宿主 render_loop draw_shaders 接线（GPU 后置）。

pub mod audio;
pub mod bird_spots;
pub mod boids;
pub mod butterfly;
pub mod components;
pub mod duck;
pub mod duck_splash_particle;
pub mod ducks_react_to_being_inside_buildings;
pub mod firefly;
pub mod fish;
pub mod sample;
pub mod shared;
pub mod sheep;
pub mod startup_sheep;
pub mod spawn_ducks;
pub mod startup_critters;
pub mod ui_pet_ducks;
pub mod wren;
pub mod wren_boids;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;

use components::{CritterControlState, DuckSpawnLocations, FishSpawnLocations};

/// B 种子（插件 build 0x14067d537 传 raw Perchance state 0xc0df → PerchanceContext）。
pub const CRITTER_RNG_SEED_B: u128 = 0x606f;

/// 注册全部 critters 系统与资源（B 0x14067d440 `CritterPlugin::build` 语义）。
///
/// fresh 反编译（evidence/restorer-eco-critters/decomp/14067d440.c）证据：
/// - Update 上两组 `add_systems`：GROUP_1（chain_ignore_deferred×2 + in_set，确切清单 U）
///   与 GROUP_2 = `(butterflies_send_to_gpu, fireflies_send_to_gpu).in_set<GameSystemSet>()`；
/// - 资源（插入顺序 O）：`insert_resource<CritterRng>`、`<WrenBoids>`、`<CritterControl
///   (::default)>`、`<DuckTrajectoryPlanner>`、`<DuckSpawnLocations(::new)>`、
///   `<BirdSpotManager>`、`<FishBoids>`、`<FishSpawnLocations(::new)>`；
/// - 事件（O）：`register_event<DespawnDuckCmd>`、`<SpawnSplashParticle>`
///   （各带 get_resource_id 幂等门：不存在才注册）；
/// - `Schedules::add_systems(OnExit<AppLoadingState>, (wren_cleanup: ResMut<WrenBoids>,
///   despawn_fish: ResMut<FishBoids>+ResMut<FishSpawnLocations>))`——本 crate 无
///   AppLoadingState 生命周期，未接线（DIVERGE）；
/// - 前置 require 资源（TypeId 0x8e1bd251962ba784，身份 U）缺省 panic_fmt（DIVERGE）。
///
/// 本重建沿用 country-core 的 World/Schedule 装配约定；宿主 App 负责把
/// `GameTime`/`MainCamera`/`CursorTerrainRaycast`/`WorldRaster<512>`/`TerrainHeightsData`/
/// `RaycastWorld`/`WallSpatialHash`/`PublicWalls` 与音频源注入。
pub fn configure(world: &mut World, schedule: &mut Schedule) {
    // ── GROUP_1（近似）：行为系统数据流链。decompile 只显示由 chain_ignore_deferred×2 +
    // in_set 装配（确切系统清单 U）。数据流顺序：目标 → 速度 → 平滑 → 避让 → 移动。
    schedule.add_systems(
        (
            (
                spawn_ducks::spawn_ducks,
                spawn_ducks::despawn_ducks,
                duck::plan_duck_trajectories,
                duck::move_ducks_along_trajectories,
                duck::update_duck_goals,
                duck::set_duck_velocities_towards_goals,
                duck::update_duck_ripples,
                firefly::update_firefly_goals,
                firefly::set_firefly_velocities_towards_goals,
                butterfly::update_butterfly_states,
                butterfly::update_butterfly_goals,
            )
                .chain(),
            (
                butterfly::set_butterfly_velocities_towards_goals,
                butterfly::update_butterfly_audio,
                fish::fish,
                // 真实注册面：`OnExit<AppLoadingState>` 上的资源重置（0x14067d440），
                // 非 Update 系统。crate 无 bevy_state，宿主在退出加载态时调用
                // `fish::despawn_fish`；不得每帧执行（资源布局重建）。
                sheep::sheep_behavior::update_sheep_goals,
                sheep::sheep_behavior::set_sheep_velocities_towards_goals,
                sheep::sheep_behavior::update_sheep_audio,
                sheep::sheep_terrain::sheep_walk_on_terrain,
                sheep::sheep_walk_animation::sheep_walk_animation,
                sheep::sheep_pet_animation::sheep_pet_animation,
                sheep::sheep_floatie::sheep_floaties,
            )
                .chain(),
            (
                sheep::sheep_umbrella::sheep_umbrellas,
                sheep::sheep_unstuck::sheep_unstuck,
                sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings,
                sheep::sheep_react_to_being_inside_buildings::sheep_reaction_emoji,
                sheep::sheep_input_record_replay::sheep_input_record_replay,
                sheep::ui_pet_sheep::ui_pet_sheep,
                ui_pet_ducks::ui_pet_ducks,
                ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings,
                duck_splash_particle::duck_splash_particle,
                wren::update_decorator_bird_spots,
                wren::update_roof_bird_spots,
                wren::update_wrens,
            )
                .chain(),
            (
                wren_boids::sim_wren_boids,
                shared::critters_avoid_collisions,
                shared::smooth_critter_velocities,
                shared::move_critters,
            )
                .chain(),
        )
            .chain(),
    );

    // ── GROUP_2（O 0x14067d440）：蝴蝶/萤火虫 SSBO 上传，独立 add_systems +
    // `in_set<GameSystemSet>`（SSBO 上传与行为系统分属两注册组）。
    schedule.add_systems(
        (
            butterfly::butterflies_send_to_gpu,
            firefly::fireflies_send_to_gpu,
        )
            .chain(),
    );

    // ── 资源注册（O 0x14067d440 顺序：8 个 insert_resource）────────────────────────
    // ① insert_resource<critters::CritterRng>（DIVERGE：crate 落位 duck.rs）。
    world.init_resource::<duck::CritterRng>();
    // ② insert_resource<wren_boids::WrenBoids>（B 符号 0x140f85470）。
    world.init_resource::<wren_boids::WrenBoids>();
    // ③ insert_resource<critters::CritterControl> —— crate 用 CritterControlState 承载
    //   +0 intensity / +4 firefly enabled / +5 wren enabled（B 类型符号未取）。
    world.init_resource::<CritterControlState>();
    // ④ insert_resource<DuckTrajectoryPlanner>。
    world.init_resource::<duck::DuckTrajectoryPlanner>();
    // ⑤ spawn_ducks::DuckSpawnLocations::new + insert_resource。
    world.insert_resource(DuckSpawnLocations::new());
    // ⑥ insert_resource<critters::BirdSpotManager> —— DIVERGE：B 内联初值 [1,1,0,0,8,0]，
    //   crate 用 bird_spots.rs 池模型 Default。
    world.init_resource::<bird_spots::BirdSpotManager>();
    // ⑦ insert_resource<fish::FishBoids>——B 符号实证为 Resource（0x140f84e80）。
    world.init_resource::<components::FishBoids>();
    // ⑧ fish::FishSpawnLocations::new + insert_resource。
    world.insert_resource(FishSpawnLocations::new());

    // ── 事件注册（O 0x14067d440：register_event，各带 get_resource_id 幂等门）────────
    world.init_resource::<Events<spawn_ducks::DespawnDuckCmd>>();
    // `register_event<SpawnSplashParticle>`（O 0x140f94300；载荷 = Entity，EventInstance 16B）.
    world.init_resource::<Events<duck_splash_particle::SpawnSplashParticle>>();

    // ── 近似依赖资源（非 B build 注册集；crate 现有系统消费，待各模块恢复后移除）────────
    world.init_resource::<duck::DevHotkeys>();
    world.init_resource::<duck::DuckRipples>();
    world.init_resource::<fish::FishRng>();
    world.init_resource::<wren::WrenRng>();
    world.init_resource::<wren_boids::WrenRng2>();
    world.init_resource::<wren_boids::WrenEnabled>();
    world.init_resource::<shared::TerrainRockMask>();
    world.init_resource::<butterfly::ButterflyProximity>();
    world.init_resource::<butterfly::ButterflyGpuData>();
    world.init_resource::<firefly::FireflyGpuData>();
    world.init_resource::<sheep::ui_pet_sheep::PetSheepTarget>();
    world.init_resource::<sheep::sheep_umbrella::Raining>();
    world.init_resource::<Events<sheep::sheep_input_record_replay::SheepInputReplayCmd>>();
    world.init_resource::<Events<sheep::sheep_pet_animation::PlaySheepPet>>();
    // update_roof_bird_spots 消费的屋顶/墙变更事件（宿主亦发布；缺失时 EventReader 初始化失败）。
    world.init_resource::<Events<country_core::resources::roofs::commands::RecalculateRoofCmd>>();
    world.init_resource::<Events<country_core::resources::walls::public_walls::OnWallChanged>>();
}
