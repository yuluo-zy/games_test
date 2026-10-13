//! wall_spatial_hash.rs — spatial hash of wall segments (WallSpatialHash / WallSegment /
//! GridCell). Data structure; PDB procedure list O (see address-map: methods at 0x1409ef210..).
//!
//! Observed structure (O methods + prior walls-audit): a fixed 64×64 grid over the map
//! bounds; each cell holds wall-segment memberships; walls are added/removed by id on
//! publish/deletion. Query shapes: aabb, circle, rectangle, segment.

use std::collections::HashMap;

use crate::typed_gap::Gap;
use crate::utils::curve::{Curve, CurveInternalCoord};

/// One materialised wall segment held by the hash.
#[derive(Clone, Debug)]
pub struct WallSegment {
    pub wall: u64,
    pub curve: Curve,
    pub low: f32,
    pub high: f32,
    pub terrain_height: f32,
    pub raw: Gap,
}
impl WallSegment {
    /// O: `terrain_height_closest_to_ws_pos` (0x140aa50e0).
    pub fn terrain_height_closest_to_ws_pos(&self, _pos: glam::Vec3) -> f32 {
        self.terrain_height
    }
}

/// A grid cell: memberships of wall segments (by (wall_id, segment-index)).
#[derive(Clone, Debug, Default)]
pub struct GridCell {
    pub walls: Vec<(u64, u32)>,
}
impl GridCell {
    pub fn new() -> Self {
        Self { walls: Vec::new() }
    }
    pub fn walls(&self) -> &[(u64, u32)] {
        &self.walls
    }
    pub fn add(&mut self, wall: u64, segment: u32) {
        if !self.walls.contains(&(wall, segment)) {
            self.walls.push((wall, segment));
        }
    }
    pub fn remove_segments(&mut self, wall: u64) {
        self.walls.retain(|(w, _)| *w != wall);
    }
}

/// Wall spatial-hash resource.
#[derive(bevy_ecs::prelude::Resource, Clone, Debug)]
pub struct WallSpatialHash {
    pub bounds: glam::Vec2,
    pub dims: (u32, u32),
    pub cell_size: f32,
    pub cells: Vec<GridCell>,
    /// wall id -> (wall, curve segments) for removal / re-add.
    pub segments_by_wall: HashMap<u64, Vec<(u32, WallSegment)>>,
}
impl Default for WallSpatialHash {
    fn default() -> Self {
        Self::new(glam::Vec2::ZERO, (64, 64))
    }
}
impl WallSpatialHash {
    pub fn new(bounds: glam::Vec2, dims: (u32, u32)) -> Self {
        // typed gap: bounds.min/max are not Vec2 fields (E0615); cell size not wired.
        let cell_size = 1.0;
        Self {
            bounds,
            dims,
            cell_size,
            cells: vec![GridCell::new(); (dims.0 as usize) * (dims.1 as usize)],
            segments_by_wall: HashMap::new(),
        }
    }
    pub fn bounds_ws(&self) -> glam::Vec2 {
        self.bounds
    }
    pub fn dims(&self) -> (u32, u32) {
        self.dims
    }
    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }
    pub fn cell_center(&self, index: usize) -> glam::Vec2 {
        let (gx, gy) = self.cell_index_to_grid_coord(index);
        glam::Vec2::ZERO // typed gap: bounds.min origin not wired.
            + glam::Vec2::new(
                (gx as f32 + 0.5) * self.cell_size,
                (gy as f32 + 0.5) * self.cell_size,
            )
    }
    pub fn cell_index_to_grid_coord(&self, index: usize) -> (u32, u32) {
        (index as u32 % self.dims.0, index as u32 / self.dims.0)
    }
    pub fn grid_coord_to_cell_index(&self, gx: u32, gy: u32) -> usize {
        (gy * self.dims.0 + gx) as usize
    }
    pub fn world_space_to_grid_coord_unclamped(&self, p: glam::Vec3) -> (u32, u32) {
        // typed gap: bounds.min origin not wired.
        let rel = p.truncate();
        (
            (rel.x / self.cell_size) as u32,
            (rel.y / self.cell_size) as u32,
        )
    }
    pub fn world_space_to_grid_coord(&self, p: glam::Vec3) -> (u32, u32) {
        let (x, y) = self.world_space_to_grid_coord_unclamped(p);
        (
            x.min(self.dims.0 - 1),
            y.min(self.dims.1 - 1),
        )
    }
    pub fn get_cell_at_index(&self, index: usize) -> &GridCell {
        &self.cells[index]
    }
    pub fn get_cells_from_aabb(&self, _min: glam::Vec2, _max: glam::Vec2, _f: impl FnMut(usize)) {
    }
    pub fn get_cells_from_circle(&self, _c: glam::Vec2, _r: f32, _f: impl FnMut(usize)) {}
    pub fn get_cells_from_rectangle2d(
        &self,
        _c: glam::Vec2,
        _dims: glam::Vec2,
        _f: impl FnMut(usize),
    ) {
    }
    pub fn get_cells_from_segment(&self, _a: glam::Vec2, _b: glam::Vec2, _f: impl FnMut(usize)) {}
    /// O: `add_segment` (0x140aa5dd0) — insert a wall segment into covered cells.
    pub fn add_segment(&mut self, wall: u64, _low: f32, _high: f32, segment: WallSegment) {
        let idx = self.segments_by_wall.entry(wall).or_default().len() as u32;
        let a = segment.curve.get_pos_at_u(crate::utils::curve::CurveU { raw: Gap });
        let b = segment.curve.get_pos_at_u(crate::utils::curve::CurveU { raw: Gap });
        let (gx1, gy1) = self.world_space_to_grid_coord(a);
        let (gx2, gy2) = self.world_space_to_grid_coord(b);
        let (lo, hi) = (gx1.min(gx2), gy1.min(gy2));
        for gy in lo..=hi {
            for gx in lo..=hi {
                let index = self.grid_coord_to_cell_index(gx, gy);
                let cell = &mut self.cells[index];
                cell.add(wall, idx);
            }
        }
        self.segments_by_wall.entry(wall).or_default().push((idx, segment));
    }
    /// O: `remove_wall_id` (0x140aa5360) — remove all segments of a wall.
    pub fn remove_wall_id(&mut self, wall: u64) {
        for cell in &mut self.cells {
            cell.remove_segments(wall);
        }
        self.segments_by_wall.remove(&wall);
    }
    pub fn query(&self, _p: glam::Vec3, _r: f32, _f: impl FnMut(&WallSegment)) {}
    pub fn query_with_segment(&self, _a: glam::Vec2, _b: glam::Vec2, _f: impl FnMut(&WallSegment)) {
    }
    pub fn query_with_circle(&self, _c: glam::Vec2, _r: f32, _f: impl FnMut(&WallSegment)) {}
    pub fn query_with_aabb_clamped(
        &self,
        _min: glam::Vec2,
        _max: glam::Vec2,
        _f: impl FnMut(&WallSegment),
    ) {
    }
    pub fn query_with_rectangle2d(
        &self,
        _c: glam::Vec2,
        _dims: glam::Vec2,
        _f: impl FnMut(&WallSegment),
    ) {
    }
    pub fn query_with_circle_callback(&self, _c: glam::Vec2, _r: f32, _f: impl FnMut(&WallSegment)) {
    }
}
