//! Original B collision DirtyTiles rectangle policy; implementation not yet
//! instruction-verified. std supplies fused multiply/add, floor and ceil.
//! The existing 64-word mask is authoritative input, never an invented default.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::manual_clamp)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirtyTiles64 {
    rows: [u64; 64],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirtyTileRect {
    pub minimum: [u32; 2],
    /// Original raster end coordinates are exclusive and clamp to 63.
    pub end: [u32; 2],
}
impl DirtyTileRect {
    pub fn is_empty(self) -> bool {
        self.minimum[0] >= self.end[0] || self.minimum[1] >= self.end[1]
    }
}
/// Native constants at B 142925874/14292587c and 142ae19d0.
/// scale is exactly 0x3efc0fc1, bias32, clamp63; fmaf occurs before rounding.
fn cell_coordinate(world: f32, upper: bool) -> u32 {
    let value = world.mul_add(f32::from_bits(0x3efc0fc1), 32.0);
    let value = if upper { value.ceil() } else { value.floor() };
    // Original CMPORD mask converts NaN to zero before the upper clamp.
    if value.is_nan() {
        0
    } else {
        value.max(0.0).min(63.0) as u32
    }
}
pub fn world_rect_cells(rect: [f32; 4]) -> DirtyTileRect {
    DirtyTileRect {
        minimum: [
            cell_coordinate(rect[0], false),
            cell_coordinate(rect[1], false),
        ],
        end: [
            cell_coordinate(rect[2], true),
            cell_coordinate(rect[3], true),
        ],
    }
}
impl DirtyTiles64 {
    /// B new_clear140960ac0 explicitly zeros all512 bytes.
    pub fn new_clear() -> Self {
        Self { rows: [0; 64] }
    }
    pub fn from_rows(rows: [u64; 64]) -> Self {
        Self { rows }
    }
    /// B clear140960c20; this resets mask contents, not owner availability.
    pub fn clear(&mut self) {
        self.rows.fill(0);
    }
    pub fn extend(&mut self, other: &Self) {
        for (target, source) in self.rows.iter_mut().zip(other.rows) {
            *target |= source;
        }
    }
    /// B flush copies the bitmap then clears it before publishing the copy.
    pub fn take_and_clear(&mut self) -> Self {
        let snapshot = self.clone();
        self.clear();
        snapshot
    }
    pub fn tile_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.rows
            .iter()
            .copied()
            .enumerate()
            .flat_map(|(row, mut bits)| {
                std::iter::from_fn(move || {
                    if bits == 0 {
                        return None;
                    }
                    let column = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    Some(row * 64 + column)
                })
            })
    }
    pub fn rows(&self) -> &[u64; 64] {
        &self.rows
    }
    pub fn rasterize_world_rect(&mut self, rect: [f32; 4]) -> DirtyTileRect {
        let cells = world_rect_cells(rect);
        if !cells.is_empty() {
            for z in cells.minimum[1]..cells.end[1] {
                for x in cells.minimum[0]..cells.end[0] {
                    self.rows[z as usize] |= 1u64 << x;
                }
            }
        }
        cells
    }
}
