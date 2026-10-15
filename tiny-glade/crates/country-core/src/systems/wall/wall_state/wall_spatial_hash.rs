//! B wall spatial membership and segment data. Standard containers replace only
//! dependency storage; endpoint projection and cell coverage retain game policy.

use bevy_ecs::prelude::*;
use hashbrown::{HashMap, HashSet};

use crate::recovered_building::{
    decorator_subtype::nearest_segment,
    wall_spatial_index::{CELL_COUNT, WALL_SEGMENT_RADIUS, segment_cells},
};
use crate::resources::walls::{
    WallId,
    public_walls::{OnWallChanged, OnWallDeleted, PublicWallState, PublicWalls},
};
use crate::utils::dirty_tiles::DirtyTiles;

/// The 48-byte game payload at add_segment 0x140aa5dd0 is two Vec2 endpoints,
/// three pairs of floats, and WallId. The separate u32 segment key is generated
/// by the spatial owner. This typed representation does not emulate map ABI.
#[derive(Clone, Copy, Debug)]
pub struct WallSpatialSegment {
    pub endpoints: [[f32; 2]; 2],
    pub source_u: [f32; 2],
    pub relative_heights: [f32; 2],
    pub terrain_heights: [f32; 2],
    pub wall: WallId,
}

#[derive(Resource, Debug)]
pub struct WallSpatialHash {
    cells: Vec<Vec<u32>>,
    segments: HashMap<u32, WallSpatialSegment>,
    wall_segments: HashMap<WallId, Vec<u32>>,
    segment_cells: HashMap<u32, Vec<usize>>,
    next_segment_id: u32,
}

impl Default for WallSpatialHash {
    fn default() -> Self {
        Self {
            cells: vec![Vec::new(); CELL_COUNT],
            segments: HashMap::new(),
            wall_segments: HashMap::new(),
            segment_cells: HashMap::new(),
            next_segment_id: 0,
        }
    }
}

impl WallSpatialHash {
    pub fn remove_wall_id(&mut self, wall: WallId) {
        if let Some(ids) = self.wall_segments.remove(&wall) {
            for id in ids {
                if let Some(cells) = self.segment_cells.remove(&id) {
                    for cell in cells {
                        self.cells[cell].retain(|old| *old != id);
                    }
                }
                self.segments.remove(&id);
            }
        }
    }

    /// 0x140aa5dd0 increments +0x58 before inserting into every covered bucket,
    /// including a segment whose geometry lies outside the strict world bounds.
    pub fn add_segment(&mut self, segment: WallSpatialSegment) -> u32 {
        let id = self.next_segment_id;
        self.next_segment_id = self.next_segment_id.wrapping_add(1);
        let cells = segment_cells(
            segment.endpoints[0],
            segment.endpoints[1],
            WALL_SEGMENT_RADIUS,
        );
        for &cell in &cells {
            self.cells[cell].push(id);
        }
        self.wall_segments.entry(segment.wall).or_default().push(id);
        self.segment_cells.insert(id, cells);
        self.segments.insert(id, segment);
        id
    }

    pub fn segments_for_wall(&self, wall: WallId) -> impl Iterator<Item = &WallSpatialSegment> {
        self.wall_segments
            .get(&wall)
            .into_iter()
            .flatten()
            .filter_map(|id| self.segments.get(id))
    }
    pub fn segments_in_cell(&self, cell: usize) -> impl Iterator<Item = &WallSpatialSegment> {
        self.cells
            .get(cell)
            .into_iter()
            .flatten()
            .filter_map(|id| self.segments.get(id))
    }
    pub fn candidates(&self, dirty: &DirtyTiles) -> Vec<WallId> {
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for cell in dirty.tile_indices() {
            for segment in self.segments_in_cell(cell) {
                if seen.insert(segment.wall) {
                    result.push(segment.wall);
                }
            }
        }
        result
    }

    /// 0x140a8fd90 uses simplified Curve2 endpoints/U, then projects each
    /// endpoint onto the *first* terrain-mapped Curve3, before path preparation.
    pub fn replace_wall(&mut self, id: WallId, state: &PublicWallState) {
        state.assert_valid();
        self.remove_wall_id(id);
        let source = state.simplified_curve_2d();
        assert_eq!(source.points.len(), source.points_u.len());
        let mapped = &state.geometry.outline;
        let relative = &state.geometry.relative_heights;
        assert_eq!(mapped.len(), relative.len());
        let project = |point: [f32; 2]| {
            let (index, t) = nearest_segment(mapped, [point[0], 0.0, point[1]])
                .expect("invalid terrain-mapped wall curve");
            assert!((0.0..=1.0).contains(&t));
            let height = (relative[index + 1] - relative[index]) * t + relative[index];
            let terrain_y = t * mapped[index + 1][1] + (1.0 - t) * mapped[index][1];
            (height, terrain_y)
        };
        for (i, endpoints) in source.points.windows(2).enumerate() {
            let a = project(endpoints[0]);
            let b = project(endpoints[1]);
            self.add_segment(WallSpatialSegment {
                endpoints: [endpoints[0], endpoints[1]],
                source_u: [source.points_u[i], source.points_u[i + 1]],
                relative_heights: [a.0, b.0],
                terrain_heights: [a.1, b.1],
                wall: id,
            });
        }
    }
}

pub fn spatial_hash_update(
    mut changed: EventReader<OnWallChanged>,
    mut deleted: EventReader<OnWallDeleted>,
    public: Res<PublicWalls>,
    mut spatial: ResMut<WallSpatialHash>,
) {
    for &OnWallChanged(id) in changed.read() {
        let Some(state) = public.get(id) else {
            // 0x140d4a330 is GlobalLogger::log (S_PUB32), not a panic helper.
            log::warn!(target: "country_core::systems::wall::wall_state::spatial_hash_update",
                "SpatialQuery received OnWallChanged with wall index {}", id.0);
            continue;
        };
        spatial.replace_wall(id, state);
    }
    for &OnWallDeleted(id) in deleted.read() {
        spatial.remove_wall_id(id);
    }
}
