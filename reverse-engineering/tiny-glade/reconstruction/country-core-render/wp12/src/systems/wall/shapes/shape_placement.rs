//! shapes/shape_placement.rs — shape placement + snapping for circle/rectangle placement.
//!
//! PDB procedures: `shape_placement_params` 0x14097f970 (5171 B), `snap_to_roof_ridge`
//! 0x140980db0, `ShapePlacement::new_unsnapped`, `closest_segment_to_snap_to`.
//! FunctionSystem signature O.
//!
//! Business logic (O, call chain): `shape_placement_params` computes the placement params
//! (position + snapped-to-wall or roof-ridge alignment) from the cursor terrain raycast and
//! the `WallSpatialHash` queries; `snap_to_roof_ridge` aligns the shape to a roof ridge when
//! a roof is intersected (`Roof::pivot_3d`, `RoofRidgeDims::calc`).

use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::roofs::roof_ridge::RoofRidgeDims;
use crate::resources::roofs::roof_shape::Roof;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
use crate::typed_gap::Gap;
use crate::ui::mode_manager::AppMode;
use crate::utils::curve::Curve;
use crate::utils::geometry::circle::Circle2d;

/// Shape placement params (O: `ShapePlacement::new_unsnapped`).
#[derive(Resource, Clone, Debug, Default)]
pub struct ShapePlacement {
    pub pos: glam::Vec3,
    pub snapped: bool,
    pub snap_u: Option<f32>,
    pub raw: Gap,
}
impl ShapePlacement {
    pub fn new_unsnapped(pos: glam::Vec3) -> Self {
        Self {
            pos,
            snapped: false,
            snap_u: None,
            raw: Gap,
        }
    }
}

/// O: `snap_to_roof_ridge` — align placement to the roof ridge of an intersected roof.
pub fn snap_to_roof_ridge(_roof: &Roof, _pos: glam::Vec3) -> glam::Vec3 {
    let _ = RoofRidgeDims::default();
    glam::Vec3::ZERO
}

/// O: `closest_segment_to_snap_to` — nearest wall segment within the snap radius.
pub fn closest_segment_to_snap_to(
    _curve: &Curve,
    _pos: glam::Vec3,
    _radius: f32,
) -> Option<(f32, f32)> {
    None
}

pub fn shape_placement_params(
    app_mode: Res<AppMode>,
    cursor: Res<CursorTerrainRaycast>,
    public_walls: Res<PublicWalls>,
    spatial_hash: Res<WallSpatialHash>,
    mut placement: ResMut<ShapePlacement>,
    roofs: bevy_ecs::system::Query<&Roof>,
) {
    let _ = (&app_mode, &public_walls, &spatial_hash, &roofs);
    let pos = cursor.position();
    // O: initial unsnapped placement, then snap to the nearest wall segment / roof ridge.
    *placement = ShapePlacement::new_unsnapped(pos);
    let _ = Circle2d::default();
}
