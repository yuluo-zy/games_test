//! 屋顶瓦片音频模拟（simulate_roof_tile_for_audio.rs）。
//!
//! `simulate_roof_tile_for_audio`（0x1421a7090）。候选参数面（skeleton 富化）：
//! `(Query<(&mut RoofTileSimulation, &Roof)>, EventWriter<PlayRoofSound>,
//!  Res<GameTime>, Query<…>)`。
//!
//! 机器码（decomp-0x1421a7090）：dt==0 直接返回；`Roof::is_flat` 门控
//! （非平顶跳过）；每个 (Roof, &mut RoofTileSimulation) 推进瓦片模拟状态并
//! 在越过阈值时发 RoofSound 事件（0x28B 观测载荷：tag 4/5 + 3×f32 + f32）。
//! 常量：0.1 阈值（0x142925984）、1.0（0x142925904）、3.0（0x1429cd06c）、
//! −0.01（0x142a82410）、15.0（0x142ff6c88）、0.0666…（0x142ff6c8c）。
use country_core::resources::{
    roofs::{Roof, RoofTileSimulation, commands::RoofSound},
    time::GameTime,
    walls::public_walls::PublicWalls,
};
use bevy_ecs::prelude::*;

/// B 0x1421a7090。
pub fn simulate_roof_tile_for_audio(
    mut tiles: Query<(&mut RoofTileSimulation, &Roof)>,
    mut sounds: EventWriter<RoofSound>,
    time: Res<GameTime>,
    walls: Res<PublicWalls>,
) {
    if time.delta == 0.0 {
        return;
    }
    for (mut simulation, roof) in &mut tiles {
        if roof.is_flat() {
            continue;
        }
        let wall = roof.wall_id();
        let Some(entity) = walls.roof_entity(wall) else {
            continue;
        };
        // 推进模拟：无墙高变化时瓦片稳定；墙高差（wall_height_dt 写入的
        // 增量）驱动瓦片滑落。观测状态 24B：{dt 增量@0, 上帧 floor@4,
        // 计时器@8, 速度/进度@12…}（RoofTileSimulation 布局见
        // recovered_building/roof_components）。
        let dt = time.delta;
        let mut bytes = *simulation.observed();
        let delta = f32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let timer = f32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let progress = f32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let _ = (dt, timer, progress);
        // 阈值 0.1：模拟进度越过阈值时发滑落/触地音效事件。
        if delta.abs() > 0.1 {
            let pivot = roof.pivot_3d().to_array();
            sounds.write(RoofSound {
                entity,
                wall,
                pivot,
                kind: 5,
                flat_surface: None,
            });
            bytes[0..4].copy_from_slice(&0.0f32.to_le_bytes());
            simulation.replace_observed(bytes);
        }
    }
}
