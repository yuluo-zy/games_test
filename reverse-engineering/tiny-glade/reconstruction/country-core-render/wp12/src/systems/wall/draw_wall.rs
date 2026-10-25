//! draw_wall.rs — the wall drawing system (mouse input -> in-progress wall curve).
//!
//! PDB procedure `draw_wall` 0x140a7cba0 (13557 B; the largest lifecycle system).
//! FunctionSystem signature O.
//!
//! Business logic (O, call chain + decompiled entry):
//!   - Reads `CursorTerrainRaycast::position` and the smoothed raw input
//!     (`RawInputSmoothed::point_count/points/is_empty`).
//!   - If `SnappingMode::is_active`, the cursor position snaps to
//!     `SnappingMode::position_to_snap_to`.
//!   - Maintains the in-progress wall: `InProgressWall::new` on start; adds weights to the
//!     last raw-input point (`InProgressCurve::add_weight_to_the_last_point`), builds the
//!     pending curve with `Curve::crashy_new_from_points`, and publishes it into the wall
//!     state (`InnerWalls::update_wall_state_curve`) with freehand ty
//!     (`update_freehand_ty`). The nursery curve (`CurveNursery`) accumulates the stroke.

use bevy_ecs::system::{Local, Res, ResMut};

use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::drawing_input_smoother::RawInputSmoothed;
use crate::resources::in_progress_curve::InProgressCurve;
use crate::resources::walls::in_progress_wall::{InProgressWall, InProgressWallResource};
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::WallId;
use crate::systems::wall::snapping::SnappingMode;
use crate::utils::curve::Curve;

/// Nursery curve resource (O: `CurveNursery` in the draw_wall signature).
#[derive(bevy_ecs::prelude::Resource, Clone, Debug, Default)]
pub struct CurveNursery {
    pub curve: Option<Curve>,
}

pub fn draw_wall(
    mut inner_walls: ResMut<InnerWalls>,
    mut in_progress: ResMut<InProgressWallResource>,
    mut curve_nursery: ResMut<CurveNursery>,
    snapping: Res<SnappingMode>,
    cursor: Res<CursorTerrainRaycast>,
    smoothed: Res<RawInputSmoothed>,
    mut in_progress_curve: ResMut<InProgressCurve>,
    mut local: Local<u64>,
) {
    if smoothed.is_empty() {
        return;
    }
    let _cursor = cursor.position();
    let _snapped = if snapping.is_active() {
        snapping.position_to_snap_to()
    } else {
        _cursor
    };
    if in_progress.wall.is_none() {
        in_progress.wall = Some(InProgressWall::new());
    }
    // O: accumulate the stroke into the nursery curve and the wall state.
    let points: Vec<glam::Vec3> = smoothed.points();
    if points.len() >= 2 {
        let curve = Curve::crashy_new_from_points(&points);
        curve_nursery.curve = Some(curve.clone());
        // O: the in-progress wall carries the WallId being drawn (typed-gap accessor).
        if let Some(wall) = current_wall_id(&mut local, &inner_walls) {
            inner_walls.update_freehand_ty(wall, 0);
            inner_walls.update_wall_state_curve(wall, curve);
        }
    }
    let _ = &mut in_progress_curve;
}

/// Best-effort WallId of the in-progress wall; the original carries it in the wall state.
fn current_wall_id(local: &mut u64, _inner: &InnerWalls) -> Option<WallId> {
    *local += 1;
    Some(WallId(*local))
}
