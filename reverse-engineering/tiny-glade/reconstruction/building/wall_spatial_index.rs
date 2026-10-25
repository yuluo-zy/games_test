//! Game WallSpatialHash membership producer; std containers own storage.
//! B's original bucket grid is 64 × 64 over the strict (-65,65) world bounds.
use crate::{building_wall_state::WallId, surface::Curve2};
use std::collections::{HashMap, HashSet};
pub const GRID_SIDE: usize = 64;
pub const CELL_COUNT: usize = GRID_SIDE * GRID_SIDE;
pub const WORLD_SIZE: f32 = 130.0;
pub const WALL_SEGMENT_RADIUS: f32 = f32::from_bits(0x3e8f5c29);

pub fn world_space_to_grid_coord_unclamped(p: [f32; 2]) -> [i32; 2] {
    p.map(|p| ((WORLD_SIZE * 0.5 + p) / (WORLD_SIZE / GRID_SIDE as f32)).floor() as i32)
}
pub fn grid_coord_to_cell_index(p: [i32; 2]) -> i64 {
    p[1].wrapping_mul(GRID_SIDE as i32).wrapping_add(p[0]) as i64
}
fn inside(p: [f32; 2]) -> bool {
    p[0] > -65.0 && p[0] < 65.0 && p[1] > -65.0 && p[1] < 65.0
}

/// Original add_segment-specialized 1409ef870. The candidate box is only the
/// broad phase: each cell runs the recovered transformed Rectangle2d/Aabb test.
pub fn segment_cells(a: [f32; 2], b: [f32; 2], radius: f32) -> Vec<usize> {
    if !inside(a) || !inside(b) {
        return Vec::new();
    }
    let dx = b[0] - a[0];
    let dz = b[1] - a[1];
    let length = (dx * dx + dz * dz).sqrt();
    if length == 0.0 {
        let coord = world_space_to_grid_coord_unclamped(a);
        return vec![grid_coord_to_cell_index(coord) as usize];
    }
    let ux = dx / length;
    let uz = dz / length;
    let first = [a[0].min(b[0]), a[1].min(b[1])];
    let last = [a[0].max(b[0]), a[1].max(b[1])];
    let cell = WORLD_SIZE / GRID_SIDE as f32;
    let min = [
        (((first[0] - radius) + 65.0) / cell).floor() as i32,
        (((first[1] - radius) + 65.0) / cell).floor() as i32,
    ]
    .map(|v| v.max(0));
    let max = [
        ((last[0] + radius + 65.0) / cell).floor() as i32,
        ((last[1] + radius + 65.0) / cell).floor() as i32,
    ]
    .map(|v| v.min(63));
    let middle = [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5];
    // The original constructs a Rectangle2d in the segment-local basis, then
    // asks for its AABB. Keep that operation;
    // replacing it with a capsule distance or a global AABB changes coverage.
    let rx = ux;
    let rz = uz;
    let inverse = 1.0 / (rz * rz + rx * rx).sqrt();
    let rx = rx * inverse;
    let rz = rz * inverse;
    let extent = (rz.abs() * (cell * 0.5)) + (rx.abs() * (cell * 0.5));
    let half_length = length * 0.5;
    let half_width = f32::from_bits(0x3f0f5c29) * 0.5;
    let mut cells = Vec::new();
    for zi in min[1]..=max[1] {
        for xi in min[0]..=max[0] {
            let center = |i: i32| {
                let t = i as f32 / GRID_SIDE as f32;
                t * 65.0 - (1.0 - t) * 65.0 + cell * 0.5
            };
            let x = center(xi) - middle[0];
            let z = center(zi) - middle[1];
            let local_x = x * ux + z * uz;
            let local_z = x * (-uz) + z * ux;
            if half_length >= local_x - extent
                && local_x + extent >= -half_length
                && half_width >= local_z - extent
                && local_z + extent >= -half_width
            {
                cells.push(zi as usize * GRID_SIDE + xi as usize);
            }
        }
    }
    cells
}

#[derive(Clone, Debug)]
pub struct WallSpatialIndex {
    buckets: Vec<Vec<WallId>>,
    memberships: HashMap<WallId, Vec<usize>>,
}
impl Default for WallSpatialIndex {
    fn default() -> Self {
        Self {
            buckets: vec![Vec::new(); CELL_COUNT],
            memberships: HashMap::new(),
        }
    }
}
impl WallSpatialIndex {
    pub fn remove(&mut self, wall: WallId) {
        if let Some(cells) = self.memberships.remove(&wall) {
            for c in cells {
                self.buckets[c].retain(|id| *id != wall);
            }
        }
    }
    /// Original publisher/spatial_hash_update replaces all old segment rows and
    /// walks consecutive source Curve2 points. No synthetic closing edge.
    pub fn replace_curve(&mut self, wall: WallId, curve: &Curve2) {
        self.remove(wall);
        let mut cells = Vec::new();
        let mut seen = HashSet::new();
        for p in curve.points.windows(2) {
            for c in segment_cells(p[0], p[1], WALL_SEGMENT_RADIUS) {
                if seen.insert(c) {
                    self.buckets[c].push(wall);
                    cells.push(c);
                }
            }
        }
        self.memberships.insert(wall, cells);
    }
    pub fn wall_cells(&self, wall: WallId) -> Option<&[usize]> {
        self.memberships.get(&wall).map(Vec::as_slice)
    }
    /// DirtyTiles in original code walks ascending set bits and each bucket's
    /// wall-ID Vec. Deduplicate IDs using std; no hand-written hash table.
    pub fn candidates(&self, dirty: &[u64; 64]) -> Vec<WallId> {
        let mut seen = HashSet::new();
        let mut walls = Vec::new();
        for (block, mask) in dirty.iter().copied().enumerate() {
            let mut mask = mask;
            while mask != 0 {
                let bit = mask.trailing_zeros() as usize;
                mask &= mask - 1;
                for wall in &self.buckets[block * 64 + bit] {
                    if seen.insert(*wall) {
                        walls.push(*wall);
                    }
                }
            }
        }
        walls
    }
}
