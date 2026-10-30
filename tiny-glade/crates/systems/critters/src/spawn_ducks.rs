//! spawn_ducks / despawn_ducks（B 主基线，fresh 反编译级证据）。
//!
//! 证据（critters-extra-analysis.md §5 + fresh decomp 140f8f210.c / 140f8f8b0.c）：
//! - DuckSpawnLocations::new 0x140f8ee90（54–75）：10×10=100 候选点，元素 12B =
//!   `(Vec2{x@0,z@4}, bool used@8)`；行值 `(i/9)*52-(1-i/9)*52`（i=0..9，+52→-52），
//!   列值 = 十档对称常量（-52,-40.44,-28.89,-17.33,-5.78,5.78,17.33,28.89,40.44,52）；
//!   资源尾 timer f32@+0x18 = **25.0**（0x41c80000，初始值非 0）。
//! - spawn_ducks 0x140f8f210（80–197）：门控（非 demo 且 `is_corrupted()==false`）；
//!   计时：资源 +0x18 累加 dt；query 桶非空（有活鸭）→ 额外 +2×dt（合计 3×，O disasm
//!   0x140f8f32c）；timer >= 30.0（0x41f00000）才生，生后归 0；活鸭数 `>= 15`（0xf）不生。
//!   候选 `sorted_by<IterMut<(Vec2,bool)>>`；逐点 `bilinear 水采样 <= 0.99`（0x3f7d70a4）清 used 并跳过、
//!   `TerrainHeightsData::sample_at_world_xz` 高度 > 0 跳过（须水面）；取中 →
//!   uniform_f32×2 + golden_spiral(k+u) + x0y 构 **2** 点轨迹（k=1,2；首点 = 候选位直接落，
//!   无 golden_spiral(0) 额外抽数；**无 ×2.0 缩放**）→ Trajectory；uniform_f32 +
//!   uniform_sample_circle 随机圆偏移 → push TrajectoryPlan（0x30 槽）；标记候选已用。
//! - despawn_ducks 0x140f8f8b0（206–278）：逐 Duck：
//!   - `DuckState==2`：首点（水栅格 @pos）`< 1e-5` → 离场；否则 4 栅格（PathRasters
//!     求和 clamp[0,1] `> 0.99` → 离场（`UCOMISS XMM0,XMM12; SETA`，0x142a948d8=0.99）。
//!   - `DuckState<2` 且带轨迹（len>0 且末点≠ptr+0xc）：末点 水栅格 `< 0.1` 或
//!     clamp(4 栅格和) `> 0.9` → 离场。
//!   - `DuckState==3` 不生成离场计划。
//!   离场 = 生成 1 点 TrajectoryPlan（首点 + uniform_sample_circle 偏移）+ despawn。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use country_core::systems::paths::PathRastersResources;
use glam::Vec3;

use crate::components::{Duck, DuckSpawnLocations, DuckState, Transform};
use crate::duck::{DuckTrajectoryPlanner, Trajectory, TrajectoryPlan};
use crate::sample::{golden_spiral, uniform_sample_circle, x0y};

/// `DespawnDuckCmd`（spawn_ducks.rs PDB 命名实体）：撤销鸭子事件（critters_avoid_collisions 写）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct DespawnDuckCmd {
    pub entity: Entity,
}

/// 四路径栅格求和（O：despawn/update_duck_goals 的 4×`bilinear_sample_at_world_xz` 源 =
/// `PathRastersResources::as_float_grid(index)`，同位置 4 栅格求和；clamp[0,1] 由调用方做）。
fn path_grid_sum(r: &PathRastersResources, x: f32, z: f32) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..4usize {
        sum += r.as_float_grid()[i].bilinear_sample_at_world_xz(x, z, 130.0, 130.0);
    }
    sum
}

/// spawn_ducks（0x140f8f210）。
#[allow(clippy::too_many_arguments)]
pub fn spawn_ducks(
    time: Res<GameTime>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    terrain: Res<TerrainHeightsData>,
    mut locations: ResMut<DuckSpawnLocations>,
    mut planner: ResMut<DuckTrajectoryPlanner>,
    mut commands: Commands,
    query: Query<(), With<Duck>>,
) {
    let dt = time.delta;
    let w = (*water).as_ref();
    let live = query.iter().count();

    // 计时（O）：+0x18 先累加 dt；有活鸭（query 桶非空）→ 再 +2×dt；>= 30.0 才生。
    locations.timer += dt;
    if live > 0 {
        locations.timer += 2.0 * dt;
    }
    if locations.timer < 30.0 {
        return;
    }
    // 活鸭 >= 15 → 不生成（O disasm `CMP RBX,0xe; JA return`）。
    if live >= 15 {
        return;
    }
    locations.timer = 0.0;

    // 候选：深水 > 0.99 + 未用 + 地形高度 <= 0。真实为 sorted_by
    // 稳定序（comparator closure U）；本重建按候选下标序（DIVERGE，见模块注）。
    let DuckTrajectoryPlanner { rng: rng_ctx, plans } = &mut *planner;
    let mut picked: Option<Vec3> = None;
    for i in 0..locations.candidates.len() {
        let c = locations.candidates[i];
        // 原 C 140f8f210 L135–142：水面 <= 0.99 时清 used 并跳过。
        let water_v = w.bilinear_sample_at_world_xz(c.x, c.z, 130.0, 130.0);
        if water_v <= f32::from_bits(0x3f7d_70a4) {
            locations.used[i] = false;
            continue;
        }
        if locations.used[i] {
            continue;
        }
        let h = terrain.sample_at_world_xz(c.x, c.z);
        if h > 0.0 {
            continue;
        }
        picked = Some(c);
        locations.used[i] = true;
        break;
    }
    let Some(center) = picked else {
        return;
    };

    // 3 点轨迹（O：首点 = 候选位直接落；k=1,2 各 uniform_f32 + golden_spiral(k+u) + x0y；
    // 无 ×2.0 缩放，无 k=0 抽数）。
    let mut points = Vec::with_capacity(3);
    points.push(center);
    for k in 1..3 {
        let u = rng_ctx.uniform_f32();
        let spiral = golden_spiral(k as f32 + u);
        points.push(center + x0y(spiral));
    }
    // 随机圆偏移（O：uniform_f32 + uniform_sample_circle）。
    let u = rng_ctx.uniform_f32();
    let circle = uniform_sample_circle(rng_ctx, u);
    let plan = TrajectoryPlan {
        trajectories: vec![Trajectory { points, done: false }],
        offset: [circle.x, circle.y],
        key: 0,
        age: 0,
    };
    plans.push(plan);
    let _ = &mut commands;
}

/// despawn_ducks（0x140f8f8b0）。
/// 逐 Duck 判断深水/路径越界 → push 离场 TrajectoryPlan（由 plan_duck 执行）+ despawn。
#[allow(clippy::too_many_arguments)]
pub fn despawn_ducks(
    time: Res<GameTime>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    path_rasters: PathRastersResources,
    mut planner: ResMut<DuckTrajectoryPlanner>,
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Duck, &DuckState)>,
) {
    let _ = &time;
    let DuckTrajectoryPlanner { rng: rng_ctx, plans } = &mut *planner;
    let w = (*water).as_ref();
    for (entity, transform, duck, state) in &query {
        let b = state.to_byte();
        let mut leave = false;
        if b == 2 {
            // DuckState==2：水栅格 @pos < 1e-5 → 离场；否则 4 栅格求和 clamp[0,1] > 0.99。
            let s0 = w.bilinear_sample_at_world_xz(
                transform.translation.x,
                transform.translation.z,
                130.0,
                130.0,
            );
            if s0 < 1e-5 {
                leave = true;
            } else {
                let sum = path_grid_sum(&path_rasters, transform.translation.x, transform.translation.z);
                leave = sum.clamp(0.0, 1.0) > 0.99;
            }
        } else if b < 2 && !duck.trajectory.is_empty() {
            // DuckState<2 且带轨迹（真实还须末点≠首点（len*0xc+ptr != 0xc）；本重建以
            // len>=2 近似）：末点 水栅格 < 0.1 或 4 栅格和 clamp[0,1] > 0.9 → 离场。
            if duck.trajectory.len() >= 2 {
                let last = duck.trajectory[duck.trajectory.len() - 1];
                let s0 = w.bilinear_sample_at_world_xz(last.x, last.z, 130.0, 130.0);
                let sum = path_grid_sum(&path_rasters, last.x, last.z);
                leave = s0 < 0.1 || sum.clamp(0.0, 1.0) > 0.9;
            }
        }
        // DuckState==3：不生成离场计划（O）。
        if leave {
            // O：离场计划 = **1 点** Trajectory + uniform_f32 + uniform_sample_circle 偏移。
            let u = rng_ctx.uniform_f32();
            let circle = uniform_sample_circle(rng_ctx, u);
            let plan = TrajectoryPlan {
                trajectories: vec![Trajectory {
                    points: vec![transform.translation],
                    done: false,
                }],
                offset: [circle.x, circle.y],
                key: 0,
                age: 0,
            };
            plans.push(plan);
            commands.entity(entity).despawn();
        }
    }
}

/// DuckSpawnLocations 真实网格构建（O 0x140f8ee90）：10×10，行值公式 + 十档列常量，
/// 元素 = `(Vec2{x@0,z@4}, bool@8)`（stride 0xc）。资源构造在 components.rs
/// `DuckSpawnLocations::new`（硬编码近似 + timer 0，非本文件可改——DIVERGE 待组件组）。
pub fn duck_spawn_grid_b() -> [(f32, f32); 100] {
    let cols: [f32; 10] = [
        -52.0,
        -40.444_443,
        -28.888_889,
        -17.333_334,
        -5.777_778,
        5.777_778,
        17.333_334,
        28.888_889,
        40.444_443,
        52.0,
    ];
    let mut out = [(0.0f32, 0.0f32); 100];
    for i in 0..10usize {
        let row = (i as f32 / 9.0) * 52.0 - (1.0 - i as f32 / 9.0) * 52.0;
        for (j, &col) in cols.iter().enumerate() {
            out[i * 10 + j] = (row, col);
        }
    }
    out
}
