//! 墙高→瓦片模拟增量（wall_height_dt.rs）。
//!
//! `wall_height_dt`（0x1421a2b60）。候选参数面（skeleton 富化）：
//! `(Query<(&mut RoofTileSimulation, &Roof)>)` —— 单参数查询。
use country_core::resources::roofs::{Roof, RoofTileSimulation};
use bevy_ecs::prelude::*;

pub fn wall_height_dt(mut roofs: Query<(&mut RoofTileSimulation, &Roof)>) {
    for (mut simulation, roof) in &mut roofs {
        simulation.update_wall_height(roof.floor_y());
    }
}
