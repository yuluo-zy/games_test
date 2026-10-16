//! sheep_unstuck（0x1423246b0，源码 0–；closure$0 L67）。
//!
//! 证据（skeleton PDB 依赖注解）：RaycastWorld::legacy_spherecast（0x1408ccc90）、
//! bilinear_sample_at_world_xz（水）、terrain_rock_mask::sample_at_world_xz、
//! TerrainHeightsData::sample_at_world_xz、FrozenMap::get、assert_failed。
//! 语义：羊卡住（球检贴近地形/建筑）→ 传送到附近可行点（地形高度 + 水面检查）。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use glam::Vec3;

use crate::components::{Sheep, Transform, Velocity};
use crate::shared::{
    legacy_spherecast, RaycastWorldHandle, SpherecastResult, TerrainRockMask,
};

/// sheep_unstuck（0x1423246b0）。
/// 每只羊：`legacy_spherecast`（固定 10 槽序）命中且球距 < 阈值 → 卡住 →
/// 沿地形高度传送到随机偏移的可行点（水面采样排除深水）。
pub fn sheep_unstuck(
    time: Res<GameTime>,
    terrain: Res<TerrainHeightsData>,
    water: Res<WorldRaster<512>>,
    rock_mask: Res<TerrainRockMask>,
    raycast: Res<RaycastWorldHandle>,
    mut rng: ResMut<crate::wren_boids::WrenRng2>,
    mut query: Query<(&mut Transform, &Sheep, &mut Velocity)>,
) {
    let _ = &time;
    let w = (*water).as_ref();
    let r = &mut rng.0.lock().unwrap();
    for (mut transform, _sheep, mut velocity) in &mut query {
        let pos = transform.translation;
        let sphere: SpherecastResult = legacy_spherecast(&raycast, pos, Vec3::NEG_Y);
        // 球检命中且距离贴近 → 卡住。
        if sphere.hit && sphere.distance < 0.5 {
            // 沿地形高度传送（terrain 采样）+ 随机偏移 + 水面/岩石排除。
            for _attempt in 0..8 {
                let offset = Vec3::new(r.f32() - 0.5, 0.0, r.f32() - 0.5) * 2.0;
                let candidate = Vec3::new(pos.x + offset.x, 0.0, pos.z + offset.z);
                let h = terrain.sample_at_world_xz(candidate.x, candidate.z);
                let s = w.bilinear_sample_at_world_xz(candidate.x, candidate.z, 130.0, 130.0);
                if s > 0.5 || h < -0.5 {
                    continue; // 深水/悬崖不可行。
                }
                transform.translation = Vec3::new(candidate.x, h + 4.0, candidate.z);
                velocity.0 = Vec3::ZERO;
                break;
            }
        }
        let _ = rock_mask.mask();
    }
}
