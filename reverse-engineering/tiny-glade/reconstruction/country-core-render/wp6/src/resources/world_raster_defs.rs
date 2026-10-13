//! world_raster_defs.rs — 命名世界光栅（WP6 恢复）。
//!
//! 源：B skeleton `resources::world_raster_defs`（game_profile.rs 依赖证据：
//! `Cobblestone1/2/3Raster`、`GardenRaster`、`PathRaster`、`WaterRaster`，均实现
//! `Deref`/`DerefMut` 到 `RasterBufferRef<512>`）。
//! `WaterRaster` 是 `nani_update_preplaced_flower` 的 `Res<WaterRaster>`（O ECS 签名）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::world_raster::RasterBufferRef;
use crate::typed_gap::Gap;

/// 水光栅（O：`Res<WaterRaster>`；preplaced_flower 读水位 ≤0.01 判定）。
/// 布局：newtype 到 `RasterBufferRef<512>`（I：Deref 证据）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct WaterRaster(pub RasterBufferRef<512>);

/// 路径光栅（O：命名实体；未消费）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PathRaster(pub RasterBufferRef<512>);

/// 花园光栅（O：命名实体；未消费）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct GardenRaster(pub RasterBufferRef<512>);

/// 鹅卵石光栅 1/2/3（O：命名实体；未消费）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct Cobblestone1Raster(pub RasterBufferRef<512>);
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct Cobblestone2Raster(pub RasterBufferRef<512>);
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct Cobblestone3Raster(pub RasterBufferRef<512>);
