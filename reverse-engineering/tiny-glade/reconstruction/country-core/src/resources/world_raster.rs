//! world_raster.rs — 世界光栅缓冲族（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/world_raster.rs`；PDB 命名实体：
//! `WorldRaster` / `RasterBufferRef` / `DirtyTilePublisher` / `StrokeBlendMode` /
//! `UploadWorldRasterBuffersToStagingCmd`。
//! `RasterBufferRef<512>` 是 512×512 光栅的引用（O：preplaced_flower 反编译
//! `bilinear_sample_at_world_xz<resources::world_raster::RasterBufferRef<512>>`）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 固定尺寸光栅缓冲引用（O：`RasterBufferRef<512>` 泛型；布局 U）。
/// 方法 `bilinear_sample_at_world_xz`（O：`utils::float_grid` 双线性采样）。
#[derive(Clone, Copy, Debug, Default)]
pub struct RasterBufferRef<const N: usize> {
    /// typed gap：{数据指针/网格, 世界范围} 字段 U。
    pub _gap: Gap,
}

/// 世界光栅（O：命名实体；`WorldRaster<512>` 512×512 u8 栅格）。WP20
/// `clear_canvas` / `world_raster` 系统以 `WorldRaster<512>` 参数化。
#[derive(Resource, Clone, Debug, Default)]
pub struct WorldRaster<const N: usize>(pub Gap);
