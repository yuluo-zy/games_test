//! `utils/dirty_tiles.rs` — `DirtyTiles` (a 64×64 dirty-cell bit grid over the
//! world raster).
//!
//! Source: B skeleton `crates/country-core/src/utils/dirty_tiles.rs`.
//! PDB named entity: `DirtyTiles`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `DirtyTiles` = **0x200 bytes** = 64 `u64` words (4096 bits ⇒ 64×64 grid)
//!   — `clear` @0x140960c20 / `new_clear` @0x140960ac0 memset 0x200;
//!   `extend` @0x140960c30 ORs the 64 words.
//! - `cell_from_world_space` @0x140960b20 — world→cell via `fmaf` scale, valid
//!   when both cell coords `< 0x40` (64).
//! - `rasterize_world_space_rect` @0x140960ae0 / `intersects_world_space_rect`
//!   @0x140960af0 — via `iter_world_space_rect_cells` @0x140ade8e0.
//! - `rasterize_world_space_stroke` @0x140ade620 (28 callers) — conservative
//!   line raster (reused `conservative_line_raster::conservative_plot_line`).
//! - `iter_tiles` @0x140adeea0, `retain` @0x140adfcb0; `dilate` /
//!   `bitmap_set` / `bitmap_intersects` / `is_set` (inline hosts).

#![allow(dead_code, unused_imports)]

/// `DirtyTiles` (O: 64×64 bit grid).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirtyTiles {
    /// 64 u64 words = 4096 cells (O: 0x200 B).
    pub bits: [u64; 64],
}

impl Default for DirtyTiles {
    fn default() -> Self {
        DirtyTiles { bits: [0; 64] }
    }
}

impl DirtyTiles {
    /// `new_clear` @0x140960ac0 (O; 7 callers).
    pub fn new_clear() -> DirtyTiles {
        DirtyTiles::default()
    }

    /// `cell_from_world_space` @0x140960b20 (O): world XZ → (x,y) cell
    /// (both `< 64`).
    pub fn cell_from_world_space(_x: f32, _z: f32) -> Option<(u32, u32)> {
        todo!("recover DirtyTiles::cell_from_world_space (fmaf world->cell scale)")
    }

    /// `set` (O inline host).
    pub fn set(&mut self, x: u32, y: u32) {
        if x < 64 && y < 64 {
            let word = (y * 64 + x) as usize / 64;
            let bit = (y * 64 + x) as usize % 64;
            self.bits[word] |= 1 << bit;
        }
    }

    /// `is_set` (O inline host).
    pub fn is_set(&self, x: u32, y: u32) -> bool {
        if x < 64 && y < 64 {
            let word = (y * 64 + x) as usize / 64;
            let bit = (y * 64 + x) as usize % 64;
            self.bits[word] & (1 << bit) != 0
        } else {
            false
        }
    }

    /// `clear` @0x140960c20 (O).
    pub fn clear(&mut self) {
        self.bits = [0; 64];
    }

    /// `extend` @0x140960c30 (O): OR-merge another DirtyTiles.
    pub fn extend(&mut self, other: &DirtyTiles) {
        for (a, b) in self.bits.iter_mut().zip(other.bits.iter()) {
            *a |= *b;
        }
    }

    /// `rasterize_world_space_rect` @0x140960ae0 (O).
    pub fn rasterize_world_space_rect(&mut self, _min: glam::Vec2, _max: glam::Vec2) {
        todo!("recover DirtyTiles::rasterize_world_space_rect")
    }

    /// `intersects_world_space_rect` @0x140960af0 (O).
    pub fn intersects_world_space_rect(&self, _min: glam::Vec2, _max: glam::Vec2) -> bool {
        todo!("recover DirtyTiles::intersects_world_space_rect")
    }

    /// `rasterize_world_space_stroke` @0x140ade620 (O; 28 callers).
    pub fn rasterize_world_space_stroke<P: Iterator<Item = glam::Vec3>>(
        &mut self,
        _points: P,
        _radius: f32,
    ) {
        todo!("recover DirtyTiles::rasterize_world_space_stroke (conservative_plot_line)")
    }

    /// `iter_world_space_stroke_cells` (O inline).
    pub fn iter_world_space_stroke_cells(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.bits.iter().enumerate().flat_map(move |(w, &word)| {
            (0..64u32)
                .filter(move |b| word & (1 << b) != 0)
                .map(move |b| ((w * 64 + b as usize) as u32 % 64, (w * 64 + b as usize) as u32 / 64))
        })
    }

    /// `iter_tiles` @0x140adeea0 (O).
    pub fn iter_tiles(&self) -> Vec<(u32, u32)> {
        self.iter_world_space_stroke_cells().collect()
    }

    /// `retain` @0x140adfcb0 (O; tabloid subscriber retain).
    pub fn retain<F: FnMut((u32, u32)) -> bool>(&mut self, _f: F) {
        todo!("recover DirtyTiles::retain")
    }

    /// `dilate` (O).
    pub fn dilate(&mut self) {
        todo!("recover DirtyTiles::dilate")
    }
}
