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

/// 固定尺寸光栅缓冲引用（O：`RasterBufferRef<512>` 泛型）。
/// 布局（O，preplaced_flower 调用点 + 采样函数 `[rcx]` 取首 qword）：`{ ptr: *const u8, len: usize }`，即一个
/// `&[u8]` 切片（调用方从 `WorldRaster` 的双缓冲里按 `+0x298` 字节选当前缓冲后构造；`WorldRaster` 本体布局 U）。
/// 格点 = `u8 / 255.0`，按 `row*N + col` 排列。
#[derive(Clone, Copy, Debug)]
pub struct RasterBufferRef<const N: usize> {
    pub data: &'static [u8],
}

impl<const N: usize> Default for RasterBufferRef<N> {
    fn default() -> Self {
        Self { data: &[] }
    }
}

impl<const N: usize> RasterBufferRef<N> {
    pub const fn new(data: &'static [u8]) -> Self {
        Self { data }
    }
}

impl RasterBufferRef<512> {
    /// O：`utils::float_grid::bilinear_sample_at_world_xz<RasterBufferRef<512>>`（世界范围 `extent_x/z`，
    /// 调用点恒为 130.0）。
    pub fn bilinear_sample_at_world_xz(&self, x: f32, z: f32, extent_x: f32, extent_z: f32) -> f32 {
        crate::utils::float_grid::bilinear_sample_at_world_xz(self, x, z, extent_x, extent_z)
    }
}

impl crate::utils::float_grid::FloatGridCells for RasterBufferRef<512> {
    /// O：`movzx eax, byte [ptr + idx]` 后 `/255.0`（采样函数无边界检查；这里越界 panic）。
    fn cell(&self, index: usize) -> f32 {
        self.data[index] as f32 / 255.0
    }
}

/// 世界光栅（O：命名实体；`WorldRaster<512>` 512×512 u8 栅格）。WP20
/// `clear_canvas` / `world_raster` 系统以 `WorldRaster<512>` 参数化。
#[derive(Resource, Clone, Debug, Default)]
pub struct WorldRaster<const N: usize>(pub Gap);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raster_ref_samples_u8_over_255_with_130_extent() {
        static DATA: [u8; 512 * 512 + 600] = [255; 512 * 512 + 600];
        let r = RasterBufferRef::<512>::new(&DATA);
        assert_eq!(r.bilinear_sample_at_world_xz(0.0, 0.0, 130.0, 130.0), 1.0);
        static ZERO: [u8; 512 * 512 + 600] = [0; 512 * 512 + 600];
        assert_eq!(
            RasterBufferRef::<512>::new(&ZERO).bilinear_sample_at_world_xz(5.0, -3.0, 130.0, 130.0),
            0.0
        );
    }
}
