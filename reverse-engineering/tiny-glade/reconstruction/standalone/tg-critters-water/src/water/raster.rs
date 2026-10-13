//! `WorldRaster<512>` (country_core::resources::world_raster, world_raster.rs 275-541).
//!
//! Layout evidence (water-cpu-analysis "字段布局"): composite (+0x08/+0x10) and scratch (+0x20/+0x28)
//! u8 buffers, 512 B dirty-tile bitmap (+0x98), read selector byte +0x298 (0 composite / 1 scratch),
//! +0x299 staging-clear dirty, +0x29a published, +0x29b re-upload dirty.
//! Grid is 512x512 u8 (water-extra-analysis 1), u8 -> f32 via `/255.0` (0x14293ac70).

use bevy_ecs::prelude::Resource;
use glam::Vec3;

pub const GRID: usize = 512;
/// World window size: `grid = (world / 130 + 0.5) * 512` (0x43020000, 0x3f000000, 0x44000000).
pub const WORLD_SCALE: f32 = 130.0;
pub const U8_NORMALIZER: f32 = 255.0;
/// 512 B bitmap = 64 u64. UNKNOWN(evidence: water-cpu-analysis "+0x98..+0x297"): the tile size
/// (8 cells = 512/64 is only an inference); it is used for dirty tracking only, never for output.
pub const DIRTY_WORDS: usize = 64;
pub const TILE_CELLS: usize = 8;

/// World xz -> raster grid coordinates (rasterize_stroke<512>).
pub fn world_to_grid(x: f32, z: f32) -> [f32; 2] {
    [(x / WORLD_SCALE + 0.5) * GRID as f32, (z / WORLD_SCALE + 0.5) * GRID as f32]
}

/// Which of the double buffers is read (`+0x298`: 0 composite, 1 scratch).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferSel {
    Composite,
    Scratch,
}

/// Borrowed u8 grid (`RasterBufferRef<512>`).
#[derive(Clone, Copy)]
pub struct RasterBufferRef<'a> {
    pub data: &'a [u8],
}

impl RasterBufferRef<'_> {
    fn cell(&self, row: usize, col: usize) -> f32 {
        self.data[row.min(GRID - 1) * GRID + col.min(GRID - 1)] as f32 / U8_NORMALIZER
    }

    /// `bilinear_sample_at_world_xz<RasterBufferRef<512>>` (0x140a14df0).
    /// `uv = clamp(xz / world_size + 0.5, 0, 1)`; indices clamp to 511; 4 corners / 255; three lerps.
    /// UNKNOWN(evidence: water-extra-analysis 3): half-cell offset of the sample position is not
    /// visible in the recovered disassembly; here `p = uv * 512`, corner i at integer p.
    pub fn bilinear_sample_at_world_xz(&self, x: f32, z: f32, world_size: f32) -> f32 {
        let u = (x / world_size + 0.5).clamp(0.0, 1.0) * GRID as f32;
        let v = (z / world_size + 0.5).clamp(0.0, 1.0) * GRID as f32;
        let (c0, r0) = (u.floor(), v.floor());
        let (fu, fv) = (u - c0, v - r0);
        let (c0, r0) = (c0 as usize, r0 as usize);
        let top = lerp(self.cell(r0, c0), self.cell(r0, c0 + 1), fu);
        let bot = lerp(self.cell(r0 + 1, c0), self.cell(r0 + 1, c0 + 1), fu);
        lerp(top, bot, fv)
    }

    /// Same with the default 130-unit window used by all recovered call sites (0x142a7b658).
    pub fn sample(&self, x: f32, z: f32) -> f32 {
        self.bilinear_sample_at_world_xz(x, z, WORLD_SCALE)
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// `ResMut<WaterRaster>` of the docs: the `WorldRaster<512>` resource.
#[derive(Resource, Clone)]
pub struct WorldRaster {
    pub composite: Vec<u8>,
    pub scratch: Vec<u8>,
    pub read_select: BufferSel,
    pub dirty: [u64; DIRTY_WORDS],
    /// CPU-side contents of `WaterRasterStagingSsbo` (full image copy; see copy_to_staging).
    pub staging: Vec<u8>,
    /// +0x299
    pub staging_clear_pending: bool,
    /// +0x29a
    pub published: bool,
    /// +0x29b. UNKNOWN(evidence: water-extra-analysis 1): the writer of this flag is not located.
    pub reupload_dirty: bool,
    /// The GPU water-mask texture produced by `water_rasterize_from_cpu`.
    pub gpu_mask: Vec<u8>,
    pub gpu_upload_count: u32,
}

impl Default for WorldRaster {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldRaster {
    /// `WorldRaster::new<512>` (0x140a15530). UNKNOWN(evidence: water-cpu-analysis 2): the original
    /// fills a 0x4000 B block with a repeated u64 pair whose decode is unknown; we start empty (0).
    pub fn new() -> Self {
        Self {
            composite: vec![0; GRID * GRID],
            scratch: vec![0; GRID * GRID],
            read_select: BufferSel::Composite,
            dirty: [0; DIRTY_WORDS],
            staging: vec![0; GRID * GRID],
            staging_clear_pending: false,
            published: false,
            reupload_dirty: false,
            gpu_mask: vec![0; GRID * GRID],
            gpu_upload_count: 0,
        }
    }

    pub fn layer(&self, l: BufferSel) -> &[u8] {
        match l {
            BufferSel::Composite => &self.composite,
            BufferSel::Scratch => &self.scratch,
        }
    }

    pub fn layer_mut(&mut self, l: BufferSel) -> &mut [u8] {
        match l {
            BufferSel::Composite => &mut self.composite,
            BufferSel::Scratch => &mut self.scratch,
        }
    }

    /// Reader view honouring the +0x298 selector (the duck/sheep systems' "双缓冲水格选择").
    pub fn read_ref(&self) -> RasterBufferRef<'_> {
        RasterBufferRef { data: self.layer(self.read_select) }
    }

    pub fn sample_at_world_xz(&self, x: f32, z: f32) -> f32 {
        self.read_ref().sample(x, z)
    }

    /// `clear_buffer<512>` (0x140a15a10): zero both layers.
    pub fn clear_buffer(&mut self) {
        self.composite.fill(0);
        self.scratch.fill(0);
    }

    /// `copy_composite_to_scratch<512>` (0x142262680).
    pub fn copy_composite_to_scratch(&mut self) {
        self.scratch.copy_from_slice(&self.composite);
    }

    /// `commit_scratch_to_composite<512>` (0x142262a60).
    pub fn commit_scratch_to_composite(&mut self) {
        self.composite.copy_from_slice(&self.scratch);
    }

    /// `rasterize_world_space_stroke`: OR the touched tiles into the dirty bitmap.
    pub fn mark_dirty_world_stroke(&mut self, points: &[Vec3], half_width_cells: f32) {
        for p in points {
            let [gx, gz] = world_to_grid(p.x, p.z);
            let m = half_width_cells + 1.0;
            let (x0, x1) = (tile_of(gx - m), tile_of(gx + m));
            let (z0, z1) = (tile_of(gz - m), tile_of(gz + m));
            for tz in z0..=z1 {
                for tx in x0..=x1 {
                    let bit = tz * (GRID / TILE_CELLS) + tx;
                    self.dirty[bit / 64] |= 1 << (bit % 64);
                }
            }
        }
    }

    pub fn dirty_tile_count(&self) -> u32 {
        self.dirty.iter().map(|w| w.count_ones()).sum()
    }

    /// `copy_to_staging<512>` (0x142261760): optionally clear staging, set the selector, copy the
    /// selected layer into the staging SSBO, publish (+0x29a = 1).
    /// UNKNOWN(evidence: water-cpu-analysis copy_to_staging): the original copies per dirty tile via
    /// `DirtyTiles::retain`; we copy the whole layer (same end state for a fully-dirty raster).
    pub fn copy_to_staging(&mut self, select: BufferSel) {
        if self.staging_clear_pending {
            self.staging.fill(0);
            self.staging_clear_pending = false;
        }
        self.read_select = select;
        let src = self.layer(select).to_vec();
        self.staging.copy_from_slice(&src);
        self.published = true;
    }

    /// Setter for +0x29b. UNKNOWN(evidence: water-extra-analysis 1): who sets it is unknown; callers
    /// (events consumer / demo) choose when.
    pub fn mark_reupload_dirty(&mut self) {
        self.reupload_dirty = true;
    }

    /// `water_rasterize_from_cpu` (0x140a12680): if +0x29b is set, clear it, clear the dirty bitmap
    /// (memset 0x200) and upload staging into the GPU mask (the 4 compute passes are modelled as one
    /// copy). Returns whether an upload happened.
    pub fn water_rasterize_from_cpu(&mut self) -> bool {
        if !self.reupload_dirty {
            return false;
        }
        self.reupload_dirty = false;
        self.dirty = [0; DIRTY_WORDS];
        self.gpu_mask.copy_from_slice(&self.staging);
        self.gpu_upload_count += 1;
        true
    }

    /// Count of cells whose normalised level exceeds `threshold` (0.5 is the consumer threshold).
    pub fn cells_above(&self, layer: BufferSel, threshold: f32) -> usize {
        self.layer(layer).iter().filter(|&&c| c as f32 / U8_NORMALIZER > threshold).count()
    }
}

fn tile_of(g: f32) -> usize {
    ((g.max(0.0) as usize).min(GRID - 1)) / TILE_CELLS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_maps_to_cell_256() {
        assert_eq!(world_to_grid(0.0, 0.0), [256.0, 256.0]);
        assert_eq!(world_to_grid(-65.0, 65.0), [0.0, 512.0]);
    }

    #[test]
    fn bilinear_normalises_by_255() {
        let mut r = WorldRaster::new();
        r.composite.fill(255);
        assert!((r.sample_at_world_xz(0.0, 0.0) - 1.0).abs() < 1e-6);
        r.composite.fill(51);
        assert!((r.sample_at_world_xz(10.0, -3.0) - 0.2).abs() < 1e-6);
    }

    #[test]
    fn bilinear_interpolates_and_clamps_outside() {
        let mut r = WorldRaster::new();
        r.composite[256 * GRID + 256] = 0;
        r.composite[256 * GRID + 257] = 255;
        r.composite[257 * GRID + 256] = 0;
        r.composite[257 * GRID + 257] = 255;
        // halfway between columns 256 and 257: p = 256.5 -> x = (256.5/512 - 0.5) * 130
        let x = (256.5 / 512.0 - 0.5) * 130.0;
        let z = (256.0 / 512.0 - 0.5) * 130.0;
        assert!((r.sample_at_world_xz(x, z) - 0.5).abs() < 1e-4);
        // far outside: clamps to the corner cell (0 here), no panic
        assert_eq!(r.sample_at_world_xz(1e6, 1e6), 0.0);
        assert_eq!(r.sample_at_world_xz(-1e6, -1e6), 0.0);
    }

    #[test]
    fn double_buffer_copies() {
        let mut r = WorldRaster::new();
        r.composite[5] = 77;
        r.copy_composite_to_scratch();
        assert_eq!(r.scratch[5], 77);
        r.scratch[9] = 3;
        r.commit_scratch_to_composite();
        assert_eq!(r.composite[9], 3);
        r.clear_buffer();
        assert!(r.composite.iter().chain(&r.scratch).all(|&c| c == 0));
    }

    #[test]
    fn staging_publish_and_selector() {
        let mut r = WorldRaster::new();
        r.scratch[1] = 200;
        r.staging_clear_pending = true;
        r.staging[7] = 9;
        r.copy_to_staging(BufferSel::Scratch);
        assert_eq!(r.read_select, BufferSel::Scratch);
        assert_eq!(r.staging[1], 200);
        assert_eq!(r.staging[7], 0);
        assert!(r.published && !r.staging_clear_pending);
        assert_eq!(r.read_ref().data[1], 200);
    }

    #[test]
    fn gpu_upload_gated_by_reupload_flag() {
        let mut r = WorldRaster::new();
        r.composite[3] = 10;
        r.copy_to_staging(BufferSel::Composite);
        assert!(!r.water_rasterize_from_cpu());
        assert_eq!(r.gpu_mask[3], 0);
        r.mark_reupload_dirty();
        r.dirty[0] = 5;
        assert!(r.water_rasterize_from_cpu());
        assert_eq!(r.gpu_mask[3], 10);
        assert!(!r.reupload_dirty);
        assert_eq!(r.dirty_tile_count(), 0);
        assert_eq!(r.gpu_upload_count, 1);
    }

    #[test]
    fn dirty_tiles_mark_and_count() {
        let mut r = WorldRaster::new();
        r.mark_dirty_world_stroke(&[Vec3::ZERO], 1.0);
        assert!(r.dirty_tile_count() >= 1);
        assert_eq!(r.cells_above(BufferSel::Composite, 0.5), 0);
    }
}
