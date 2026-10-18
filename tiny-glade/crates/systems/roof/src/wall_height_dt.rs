//! 墙高→瓦片模拟增量（wall_height_dt.rs）。
//!
//! `wall_height_dt`（0x1421a2b60，574B/142）。参数面（decomp 观测）：PublicWalls
//! 两段查询（wall→roof 哈希表）迭代。
//!
//! 判等（fresh decomp-0x1421a2b60）：逐记录 `roof[0x4c]`（基底 Y）与
//! `RoofTileSimulation[4..8]`（上帧缓存高）差 → 写 `[0..4]`，再写 `[4..8]`
//! 更新缓存；写 changed tick。`RoofTileSimulation::update_wall_height(floor_y)`
//! 与机器码逐语句一致（`bytes[..4] = floor_y−prev`、`bytes[4..8]=floor_y`）；
//! `&mut` 查询迭代自动标 Changed（等价原 tick 写）。ECS 迭代形状（原
//! wall→roof 两段 vs 本实现 Query）语义等价。
use country_core::resources::roofs::{Roof, RoofTileSimulation};
use bevy_ecs::prelude::*;

pub fn wall_height_dt(mut roofs: Query<(&mut RoofTileSimulation, &Roof)>) {
    for (mut simulation, roof) in &mut roofs {
        simulation.update_wall_height(roof.floor_y());
    }
}