//! world_raster_defs.rs — 命名世界光栅定义（WP6 恢复 + WP8 复验扩展）。
//!
//! 源：B skeleton `resources/world_raster.rs` 内子模块（无独立 bundle——已按 `ls` 确认；
//! PDB 实体：`Cobblestone1/2/3Raster::new`、`PathRaster::new`、`GardenRaster::new`、`WaterRaster`，
//! 均 Deref 到 `RasterBufferRef<512>`；`*RasterStagingSsbo` 暂存 SSBO 家族；`AllWorldRasterRead::read` /
//! `AllWorldRasterWrite`）。
//! `WaterRaster` 是 `nani_update_preplaced_flower` 的 `Res<WaterRaster>`（O ECS 签名）。
//! 上传：`upload_raster_staging_to_texture<T>`@0x1403d8bd0（W1，六实例）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::world_raster::RasterBufferRef;
use crate::typed_gap::Gap;

/// 水光栅（O：`Res<WaterRaster>`；preplaced_flower 读水位 ≤0.01 判定）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct WaterRaster(pub RasterBufferRef<512>);

/// 路径光栅（O：命名实体；`PathRaster::new`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PathRaster(pub RasterBufferRef<512>);

/// 花园光栅（O：命名实体；`GardenRaster::new`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct GardenRaster(pub RasterBufferRef<512>);

/// 鹅卵石光栅 1/2/3（O：命名实体；`Cobblestone{1,2,3}Raster::new`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct Cobblestone1Raster(pub RasterBufferRef<512>);
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct Cobblestone2Raster(pub RasterBufferRef<512>);
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct Cobblestone3Raster(pub RasterBufferRef<512>);

/// 暂存 SSBO 标记类型（O：`upload_raster_staging_to_texture<T::StagingSsbo>`、
/// `replay_raster<StagingSsbo, Raster>` 泛型参数）。布局 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct PathRasterStagingSsbo;
#[derive(Clone, Copy, Debug, Default)]
pub struct WaterRasterStagingSsbo;
#[derive(Clone, Copy, Debug, Default)]
pub struct GardenRasterStagingSsbo;
#[derive(Clone, Copy, Debug, Default)]
pub struct Cobblestone1RasterStagingSsbo;
#[derive(Clone, Copy, Debug, Default)]
pub struct Cobblestone2RasterStagingSsbo;
#[derive(Clone, Copy, Debug, Default)]
pub struct Cobblestone3RasterStagingSsbo;

/// 全光栅只读 SystemParam（O 名；`AllWorldRasterRead::read`；内部 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct AllWorldRasterRead(pub Gap);

impl AllWorldRasterRead {
    pub fn read(&self) -> Gap {
        Gap
    }
}

/// 全光栅写 SystemParam（O 名；`AllWorldRasterWrite`；内部 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct AllWorldRasterWrite(pub Gap);
