//! spawn_ducks / despawn_ducks（B 主基线，反汇编级证据）。
//!
//! 证据（critters-extra-analysis.md §5）：
//! - DuckSpawnLocations::new 0x140f8ee90（54–75）：10×10=100 候选点；
//! - spawn_ducks 0x140f8f210（80–197）：门控（非 demo 且 `is_corrupted()==false`）；
//!   计时：资源 +0x18 累加 dt；活鸭数 < 15 时 3×速率累加；timer >= 30.0 才生，生后归 0。
//!   候选：`sorted_by<IterMut<(Vec2,bool)>>`（已用标志）；逐点 `bilinear 水采样 > 0.99` 跳过、
//!   `TerrainHeights::sample` 高度 > 0 跳过（须水面）；取中 → uniform_f32+golden_spiral+x0y
//!   构 3 点轨迹 → Trajectory；uniform_f32+uniform_sample_circle 随机圆偏移 →
//!   push TrajectoryPlan（0x30 槽，被 plan_duck 消费）；标记候选已用。
//! - despawn_ducks 0x140f8f8b0（206–278）：逐 Duck：`DuckState==2` 且水面采样
//!   （首点 + 4 采样求和）判断——深水/浅水越界 → 生成离场 TrajectoryPlan（push 资源）；
//!   `DuckState<2` 且带轨迹（+0xa0!=0）同理（4+1 采样，`0.86 < 水 || 采样 < 0.1` → 离场）；
//!   `DuckState==3` 不生成。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use glam::Vec3;
use itertools::Itertools;
use perchance::PerchanceContext;

use crate::components::{Duck, DuckSpawnLocations, DuckState, Transform};
use crate::duck::{CritterRng, DuckTrajectoryPlanner, TrajectoryPlan};
use crate::sample::{golden_spiral, uniform_sample_circle, x0y};

/// `DespawnDuckCmd`（spawn_ducks.rs PDB 命名实体）：撤销鸭子事件（critters_avoid_collisions 写）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct DespawnDuckCmd {
    pub entity: Entity,
}

/// spawn_ducks（0x140f8f210）。
#[allow(clippy::too_many_arguments)]
pub fn spawn_ducks(
    time: Res<GameTime>,
    water: Res<WorldRaster<512>>,
    terrain: Res<TerrainHeightsData>,
    mut locations: ResMut<DuckSpawnLocations>,
    mut planner: ResMut<DuckTrajectoryPlanner>,
    rng: Res<CritterRng>,
    mut commands: Commands,
    query: Query<(), With<Duck>>,
) {
    let dt = time.delta;
    let w = (*water).as_ref();
    let live = query.iter().count();

    // 计时（O）：+0x18 累加 dt；活鸭 < 15 → 3×速率；>= 30.0 才生。
    let rate = if live < 15 { 3.0 } else { 1.0 };
    locations.timer += dt * rate;
    if locations.timer < 30.0 {
        return;
    }
    locations.timer = 0.0;

    // 候选：排序遍历（sorted_by；已用标志在前）。
    let mut rng_ctx = rng.0.lock().unwrap();
    let mut order: Vec<usize> = (0..locations.candidates.len()).collect();
    order.sort_by_key(|&i| i);
    let mut picked: Option<Vec3> = None;
    for &i in &order {
        if locations.used[i] {
            continue;
        }
        let c = locations.candidates[i];
        // 水面（bilinear 采样 > 0.99 跳过；O 0x3f7d70a4）且地形高度 <= 0（须水面）。
        let water_v = w.bilinear_sample_at_world_xz(c.x, c.z, 130.0, 130.0);
        if water_v > 0.99 {
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

    // 3 点轨迹（O：uniform_f32 + golden_spiral + x0y 构点）。
    let mut points = Vec::with_capacity(3);
    for k in 0..3 {
        let u = rng_ctx.uniform_f32();
        let spiral = golden_spiral(k as f32 + u);
        points.push(center + x0y(spiral) * 2.0);
    }
    // 随机圆偏移（O：uniform_f32 + uniform_sample_circle）。
    let u = rng_ctx.uniform_f32();
    let circle = uniform_sample_circle(&mut rng_ctx, u);
    let plan = TrajectoryPlan {
        points,
        offset: [circle.x, circle.y],
        executed: false,
    };
    planner.plans.push(plan);
    let _ = &mut commands;
}

/// despawn_ducks（0x140f8f8b0）。
/// 逐 Duck 判断深水/浅水越界 → push 离场 TrajectoryPlan（由 plan_duck 执行）。
pub fn despawn_ducks(
    time: Res<GameTime>,
    water: Res<WorldRaster<512>>,
    mut planner: ResMut<DuckTrajectoryPlanner>,
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Duck, &DuckState)>,
) {
    let _ = &time;
    let w = (*water).as_ref();
    for (entity, transform, duck, state) in &query {
        let pos = transform.translation;
        let b = state.to_byte();
        let mut leave = false;
        if b == 2 {
            // DuckState==2：首点 + 4 采样求和判断深水/浅水越界。
            let s0 = w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
            let mut sum = s0;
            for (dx, dz) in crate::duck::DUCK_WATER_SAMPLES {
                sum += w.bilinear_sample_at_world_xz(pos.x + dx, pos.z + dz, 130.0, 130.0);
            }
            leave = s0 > 0.86 || sum.clamp(0.0, 1.0) < 0.1;
        } else if b < 2 && !duck.trajectory.is_empty() {
            // DuckState<2 且带轨迹：4+1 采样，`0.86 < 水 || 采样 < 0.1` → 离场。
            let s0 = w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
            let mut sum = 0.0f32;
            for (dx, dz) in crate::duck::DUCK_WATER_SAMPLES {
                sum += w.bilinear_sample_at_world_xz(pos.x + dx, pos.z + dz, 130.0, 130.0);
            }
            leave = s0 > 0.86 || sum.clamp(0.0, 1.0) < 0.1;
        }
        // DuckState==3：不生成离场计划（O）。
        if leave {
            // 离场轨迹：从当前位置向外（远离 glade 中心）。
            let out_dir = pos.normalize_or_zero() * 12.0;
            let plan = TrajectoryPlan {
                points: vec![pos, pos + out_dir, pos + out_dir * 2.0],
                offset: [0.0, 0.0],
                executed: false,
            };
            planner.plans.push(plan);
            commands.entity(entity).despawn();
        }
    }
}
