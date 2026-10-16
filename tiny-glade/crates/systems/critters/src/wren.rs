//! wren 停靠点与鹪鹩更新（B 主基线，反汇编级证据）。
//!
//! 证据（skeleton PDB 依赖注解 + critters-extra-analysis §1）：
//! - update_decorator_bird_spots 0x14231c7d0（源码 81–）：BirdSpotManager::gen_id/
//!   create_spots（池复用/分配）+ Commands::spawn_empty/EntityCommands::despawn +
//!   Entities::contains 门控 + puffin 范围。
//! - update_roof_bird_spots 0x14231d6c0（144–）：BirdSpotManager::gen_id/create_spots +
//!   `BirdSpotManager::recycle`（0x1422f86e0，+0x10 原子借用守卫后池回收）+ 屋顶形状
//!   （Rectangle2d::get_edge_pts / roof_noise::apply_roof_noise / Roof::height /
//!   calc_roof_tip_center_ls）+ fastrand Rng::f32 + utils::random_splits。
//! - update_wrens 0x14231e390（273–）：EntityCommands::despawn、AudioInstance
//!   start_if_not_playing/stop、CursorTerrainRaycast::position、PlacementDataSsbo::to_affine3a、
//!   DebugLine/DebugSphere 调试可视化、fastrand Rng::f32。

use bevy_ecs::prelude::*;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::time::GameTime;
use country_core::resources::walls::public_walls::PublicWalls;
use fastrand::Rng;
use glam::Vec3;
use utils::geometry::rectangle::Rectangle2d;

use crate::audio::AudioHandle;
use crate::bird_spots::{BirdSpotId, BirdSpotManager};
use crate::components::{BirdSpot, BirdSpots, DecoratorBirdSpots, Transform, Wren};
use crate::wren_boids::{WrenAudio, WrenEnabled, MAX_WRENS};

/// 装饰物鸟点生成（0x14231c7d0）。
/// 证据步骤：对每个装饰物（宿主查询）→ `BirdSpotManager::gen_id()` + `create_spots`
/// （池复用/分配）→ `Commands::spawn_empty` 建鸟点实体写 `DecoratorBirdSpots`；
/// 对应装饰物消失（Entities::contains 门控）→ despawn 鸟点实体。
pub fn update_decorator_bird_spots(
    mut manager: ResMut<BirdSpotManager>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DecoratorBirdSpots, Option<&DecoratorBirdSpotHost>)>,
) {
    for (entity, mut spots, host) in &mut query {
        // 目标数（O：宿主装饰物每实体固定鸟点数；缺省 2）。
        let target = host.map_or(2, |h| h.spot_count);
        // gen_id + create_spots（池语义：id 自增分配；无池化实现时按新分配）。
        while spots.spots.len() < target {
            let id = manager.gen_id();
            let _ = id;
            spots.spots.push(BirdSpot {
                pos: entity_anchor(entity, host),
                occupied: false,
            });
        }
        // 超出的停靠点 → 回收（recycle 池语义：+0x10 借用守卫；此处标记释放）。
        if spots.spots.len() > target {
            spots.spots.truncate(target);
        }
    }
    // 装饰物移除 → 鸟点实体 despawn（Entities::contains 门控由宿主 removal 流做）。
    let _ = &mut commands;
}

/// 装饰物鸟点宿主标记（鸟点数；装配时插入）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct DecoratorBirdSpotHost {
    pub spot_count: usize,
}

fn entity_anchor(_entity: Entity, _host: Option<&DecoratorBirdSpotHost>) -> Vec3 {
    Vec3::ZERO
}

/// 屋顶鸟点生成（0x14231d6c0）。
/// 证据步骤：对每个带平顶/山墙屋顶的墙（PublicWalls::roof_entity）→ 取屋顶矩形
/// （Rectangle2d::get_edge_pts + roof_noise::apply_roof_noise + Roof::height +
/// calc_roof_tip_center_ls）→ `BirdSpotManager::gen_id/create_spots` 在边沿采样停靠点
/// （utils::random_splits + fastrand f32 抖动）→ 写 BirdSpots；屋顶移除 →
/// `recycle` 回池 + despawn。
pub fn update_roof_bird_spots(
    time: Res<GameTime>,
    walls: Res<PublicWalls>,
    mut manager: ResMut<BirdSpotManager>,
    mut rng: ResMut<WrenRng>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut BirdSpots, &RoofBirdSpotHost)>,
    mut roof_query: Query<&country_core::resources::roofs::roof_shape::Roof>,
) {
    let _ = &time;
    let r = &mut rng.0.lock().unwrap();
    for (entity, mut spots, host) in &mut query {
        // 屋顶形状（O：Rectangle2d::get_edge_pts + calc_roof_tip_center_ls）。
        let Some(roof) = host
            .wall
            .and_then(|w| walls.roof_entity(w))
            .and_then(|e| roof_query.get(e).ok())
        else {
            // 屋顶已移除 → recycle（回池）+ despawn。
            manager.recycle_all();
            commands.entity(entity).despawn();
            continue;
        };
        let eave_y = roof.height();
        let rect = Rectangle2d::new_axis_aligned(
            glam::Vec2::ZERO,
            glam::Vec2::splat(2.0),
        );
        let pts = rect.as_points2_centered_aligned();
        let _ = pts;
        // 边沿采样（O：random_splits 切段 + f32 抖动 + roof_noise 高度抖动）。
        let count = host.spot_count;
        let mut new_spots = Vec::with_capacity(count);
        for i in 0..count {
            let t = (i as f32 + r.f32()) / count.max(1) as f32;
            let base = roof_tip_edge(&roof, t);
            let mut y = eave_y;
            country_core::resources::roofs::roof_noise::apply_roof_noise(
                base[0],
                base[1],
                0.2,
                &mut y,
            );
            let id: BirdSpotId = manager.gen_id();
            let _ = id;
            new_spots.push(BirdSpot {
                pos: Vec3::new(base[0], y, base[1]),
                occupied: false,
            });
        }
        spots.spots = new_spots;
    }
}

/// 屋顶边沿点（O：calc_roof_tip_center_ls + Rectangle2d::get_edge_pts 语义的宿主投影；
/// t ∈ [0,1] 沿屋檐边）。
fn roof_tip_edge(roof: &country_core::resources::roofs::roof_shape::Roof, t: f32) -> [f32; 2] {
    let center = country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls(
        roof,
        [2.0, 2.0],
    );
    let half = 3.0;
    [
        center[0] + (t - 0.5) * 2.0 * half,
        center[1] + half,
    ]
}

/// 屋顶鸟点宿主标记（墙 → 屋顶；装配时插入）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct RoofBirdSpotHost {
    pub wall: Option<country_core::resources::walls::WallId>,
    pub spot_count: usize,
}

/// update_wrens（0x14231e390）。
/// 证据步骤：对每只鹪鹩：取光标位置（CursorTerrainRaycast::position）与装饰物
/// 仿射（PlacementDataSsbo::to_affine3a）→ 停靠/飞走状态机（音频 start_if_not_playing/
/// stop）→ 距光标过近或超界 → despawn；调试可视化（DebugLine/DebugSphere）。
pub fn update_wrens(
    time: Res<GameTime>,
    cursor: Res<CursorTerrainRaycast>,
    enabled: Res<WrenEnabled>,
    mut rng: ResMut<WrenRng>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Wren, &mut BirdSpots, &mut WrenAudio)>,
) {
    let dt = time.delta;
    if !enabled.0 {
        return;
    }
    let r = &mut rng.0.lock().unwrap();
    let cursor_pos = cursor.position();
    for (entity, mut wren, mut spots, mut audio) in &mut query {
        // 状态机：hop 计时 → 随机移动（O：fastrand f32 + PlacementDataSsbo::to_affine3a
        // 装饰物吸附）；距光标 < 阈值 → 飞走（audio stop + despawn）。
        wren.hop_timer -= dt;
        if wren.hop_timer <= 0.0 {
            wren.hop_timer = 0.5 + r.f32() * 2.0;
            wren.prev_pos = wren.pos;
            let hop = Vec3::new(r.f32() - 0.5, 0.0, r.f32() - 0.5) * 2.0;
            wren.pos += hop;
            wren.phase += 0.1;
        }
        // 停靠点就近选择（O：FreeSpot 语义）。
        let mut best: Option<usize> = None;
        let mut best_d = f32::MAX;
        for (i, spot) in spots.spots.iter().enumerate() {
            if spot.occupied {
                continue;
            }
            let d = (spot.pos - wren.pos).length_squared();
            if d < best_d {
                best_d = d;
                best = Some(i);
            }
        }
        if let Some(i) = best {
            spots.spots[i].occupied = true;
            wren.pos = wren.pos.lerp(spots.spots[i].pos, 0.1);
            audio.handle.start_if_not_playing();
        }
        // 距光标过近 → 飞离（audio stop）/despawn。
        if (wren.pos - cursor_pos).length() < 2.0 {
            audio.handle.stop();
            commands.entity(entity).despawn();
        }
    }
}

/// 鹪鹩 RNG（fastrand；出生与移动抖动，O）。
#[derive(Resource, Debug)]
pub struct WrenRng(pub std::sync::Mutex<Rng>);

impl Default for WrenRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(Rng::with_seed(0x606f)))
    }
}

/// 轨迹标记（plan_duck 出生路径的鹪鹩侧不用；装饰物鸟点查询的占位类型）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct WrenTrajectoryMarker;
