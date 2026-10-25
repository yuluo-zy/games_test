//! `systems::garden` — 花园系统（B 骨架 `country-core/src/systems/garden`）+ 植被接线。
//!
//! 骨架函数：`draw_garden`（closure$0/1/2）、`record_garden_history`。
//! 植被 spawn/foliage exclusion 的 CPU 侧数据见 `resources::vegetation`；GPU pass 侧
//! （garden_spawn_vegetation W16 / spawn_vegetation_around_buildings W14 /
//! rasterize_foliage_exclusion_mask W7）属 WP5 消费契约，本包只记录生产者/数据。

#![allow(dead_code, unused_imports)]

use bevy_ecs::system::Query;
use bevy_ecs::system::Commands;
use glam::Vec2;

use crate::resources::vegetation::{GardenMaskSsbo, TerrainHeightsData};
use crate::typed_gap::Gap;

/// 花园画笔输入（O 名；payload/形状规则 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct GardenBrushStroke(pub Gap);

/// `draw_garden`（O）：花园画笔输入 → 更新 GardenMask/掩码（closure$0/1/2）。
pub fn draw_garden(
    _brush: GardenBrushStroke,
    _mask: &mut GardenMaskSsbo,
    _terrain: &TerrainHeightsData<Gap>,
) {
}

/// `record_garden_history`（O）：花园编辑历史记录。
pub fn record_garden_history(_commands: Commands) {}

/// 植被环绕建筑放置（O：compute_passes::spawn_vegetation_around_buildings（W14）CPU 数据源；
/// 读 `ActiveSession::is_autumn` + TerrainHeightsData → `/garden/spawn_{bushes,flowers}_buildings`）。
pub fn vegetation_around_buildings_cpu_data(
    _terrain: &TerrainHeightsData<Gap>,
    _mask: &GardenMaskSsbo,
    _autumn: bool,
) -> Vec<Vec2> {
    Vec::new()
}

/// 花园放置（O：compute_garden::garden_spawn_vegetation（W16）CPU 数据源；
/// TerrainHeightsData + GardenMask → `/garden/spawn_{bushes_garden,flowers_garden,
/// small_flowers_garden}`）。
pub fn garden_vegetation_cpu_data(_terrain: &TerrainHeightsData<Gap>, _mask: &GardenMaskSsbo) {}
