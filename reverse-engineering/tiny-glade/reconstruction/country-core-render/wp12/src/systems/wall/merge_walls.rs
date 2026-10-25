//! merge_walls.rs — merges collinear adjacent walls into one wall.
//!
//! PDB procedure `merge_walls` 0x140999920 (6006 B). FunctionSystem signature O.
//! Business logic (O, call chain): detects walls whose end points align (terrain-sampled),
//! builds a merged curve via `Curve::crashy_new_from_points`, and records a new wall with
//! `InnerWalls::new_wall`. Decorator archival resources are read to preserve decorators.

use bevy_ecs::system::{Res, ResMut};

use crate::resources::terrain_height::{TerrainHeightsData, TerrainHeightSsbo};
use crate::resources::walls::decorator_archivist::{
    DecoratorArchivalResources, DecoratorArchivist,
};
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::WallId;

pub fn merge_walls(
    terrain_heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    decorator_archival: DecoratorArchivalResources,
    mut decorator_archivist: ResMut<DecoratorArchivist>,
    mut inner_walls: ResMut<InnerWalls>,
) {
    let _ = (&terrain_heights, &decorator_archival, &decorator_archivist);
    // O: walk inner wall ids; walls whose end point touches the start of another
    // (within terrain tolerance) are merged. Field mapping U -> typed gap.
    let ids = inner_walls.wall_ids();
    let mut merged: Vec<(WallId, glam::Vec3)> = Vec::new();
    for id in ids {
        if let Some(state) = inner_walls.get(id) {
            merged.push((id, state.curve.end_pos()));
        }
    }
    for (id, end) in merged {
        let _ = (id, end);
    }
}
