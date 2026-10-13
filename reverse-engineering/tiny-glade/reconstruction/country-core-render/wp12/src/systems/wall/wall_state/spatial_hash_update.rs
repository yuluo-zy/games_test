//! spatial_hash_update.rs — keeps the wall spatial hash in sync with publication events.
//!
//! PDB procedure `spatial_hash_update` 0x140a8fd90 (4187 bytes). FunctionSystem signature O.
//! On `OnWallChanged`: rebuild the wall's segments (curve `get_pos_at_coord` per point,
//! `sample_at_coord`) and `add_segment`; on `OnWallDeleted`: `remove_wall_id`.

use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::wall_state::publish_public_wall_state::{OnWallChanged, OnWallDeleted};
use crate::systems::wall::wall_state::wall_spatial_hash::{WallSegment, WallSpatialHash};

pub fn spatial_hash_update(
    mut on_changed: EventReader<OnWallChanged>,
    mut on_deleted: EventReader<OnWallDeleted>,
    public_walls: Res<PublicWalls>,
    mut wall_spatial_hash: ResMut<WallSpatialHash>,
) {
    for OnWallChanged(wall) in on_changed.read() {
        let Some(state) = public_walls.get(*wall) else {
            continue;
        };
        // O: materialise segment(s) from the published curve; field mapping U.
        let points = state.published_curve_points();
        if !points.is_empty() {
            let segment = WallSegment {
                wall: wall.get_inner(),
                curve: crate::utils::curve::Curve { raw: crate::typed_gap::Gap },
                low: 0.0,
                high: 1.0,
                terrain_height: 0.0,
                raw: crate::typed_gap::Gap,
            };
            wall_spatial_hash.add_segment(wall.get_inner(), 0.0, 1.0, segment);
        }
    }
    for OnWallDeleted(wall) in on_deleted.read() {
        wall_spatial_hash.remove_wall_id(wall.get_inner());
    }
}
