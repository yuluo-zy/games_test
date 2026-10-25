//! Bilinear sampling with texel-center and legacy corner coordinate policies.
use glam::Vec2;
use std::sync::atomic::{AtomicBool, Ordering};

pub const GRID_SIDE: usize = 512;
pub const DEFAULT_WORLD_EXTENT: f32 = 130.0;
/// Global compatibility policy. New save versions select centered sampling;
/// explicit-mode sampling avoids changing this shared process state.
pub static CENTERED_PATH: AtomicBool = AtomicBool::new(false);

/// Row-major cell access for a fixed 512-by-512 raster.
pub trait FloatGridCells {
    fn cell(&self, index: usize) -> f32;
}

struct Fixed512<'a, G: ?Sized>(&'a G);
impl<G: FloatGridCells + ?Sized> FloatGrid for Fixed512<'_, G> {
    fn dimensions(&self) -> [usize; 2] {
        [GRID_SIDE, GRID_SIDE]
    }
    fn load_index_unchecked(&self, index: usize) -> f32 {
        self.0.cell(index)
    }
}

pub fn bilinear_sample_at_world_xz<G: FloatGridCells + ?Sized>(
    grid: &G,
    x: f32,
    z: f32,
    extent_x: f32,
    extent_z: f32,
) -> f32 {
    bilinear_sample_at_world_xz_with_mode(
        grid,
        x,
        z,
        extent_x,
        extent_z,
        CENTERED_PATH.load(Ordering::Relaxed),
    )
}

/// Samples using an explicit policy, independently of the global setting.
pub fn bilinear_sample_at_world_xz_with_mode<G: FloatGridCells + ?Sized>(
    grid: &G,
    x: f32,
    z: f32,
    extent_x: f32,
    extent_z: f32,
    centered: bool,
) -> f32 {
    Fixed512(grid).bilinear_sample_at_world_xz(
        Vec2::new(x, z),
        Vec2::new(extent_x, extent_z),
        centered,
    )
}

pub fn bilinear_sample_at_world_xz_130<G: FloatGridCells + ?Sized>(
    grid: &G,
    x: f32,
    z: f32,
) -> f32 {
    bilinear_sample_at_world_xz(grid, x, z, DEFAULT_WORLD_EXTENT, DEFAULT_WORLD_EXTENT)
}

pub use crate::scalar::lerp;
pub trait FloatGrid {
    fn dimensions(&self) -> [usize; 2];
    fn load_index_unchecked(&self, index: usize) -> f32;
}
pub trait FloatGridSampler: FloatGrid {
    /// Samples row-major cells in a world rectangle centered at the origin.
    /// Dimensions must be at least two and their product must fit in `usize`.
    /// Centered sampling clamps to edge texels. Legacy sampling deliberately
    /// extrapolates at the positive edge; changing it would alter existing saves.
    // Keep max/min rather than clamp: their NaN policies differ.
    #[allow(clippy::manual_clamp)]
    fn bilinear_sample_at_world_xz(&self, xz: Vec2, world_extent: Vec2, sampling_fix: bool) -> f32 {
        let [width, height] = self.dimensions();
        assert!(width >= 2 && height >= 2);
        assert!(
            width.checked_mul(height).is_some(),
            "grid dimensions overflow"
        );
        let normalized = xz / world_extent + Vec2::splat(0.5);
        let axis = |v: f32, n: usize| -> (usize, usize, f32) {
            if sampling_fix {
                let v = if v < 1.0 { v } else { 1.0 };
                let texel = (v * n as f32 - 0.5).max(0.0);
                let floor = texel as usize;
                let fraction = texel - floor as f32;
                (
                    floor.min(n - 1),
                    floor.saturating_add(1).min(n - 1),
                    fraction,
                )
            } else {
                let texel = v.max(0.0).min(1.0) * n as f32;
                let floor = (texel as usize).min(n - 2);
                (floor, floor + 1, texel - floor as f32)
            }
        };
        let (x0, x1, tx) = axis(normalized.x, width);
        let (z0, z1, tz) = axis(normalized.y, height);
        let a = self.load_index_unchecked(z0 * width + x0);
        let b = self.load_index_unchecked(z1 * width + x0);
        let c = self.load_index_unchecked(z0 * width + x1);
        let d = self.load_index_unchecked(z1 * width + x1);
        let left = a + (b - a) * tz;
        let right = c + (d - c) * tz;
        left + (right - left) * tx
    }
}
impl<T: FloatGrid + ?Sized> FloatGridSampler for T {}
/// Row-major unsigned-byte cells normalized to the interval `[0, 1]`.
pub struct ByteFloatGrid<'a> {
    pub values: &'a [u8],
    pub width: usize,
    pub height: usize,
}
impl FloatGrid for ByteFloatGrid<'_> {
    fn dimensions(&self) -> [usize; 2] {
        [self.width, self.height]
    }
    fn load_index_unchecked(&self, index: usize) -> f32 {
        self.values[index] as f32 / 255.0
    }
}

#[cfg(test)]
#[path = "oracle/grid_tests.rs"]
mod native_oracle;
