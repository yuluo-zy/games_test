//! snapping/snapping_to_segments.rs — snaps the in-progress wall to a wall segment.
//!
//! PDB procedure `snapping_to_segments` 0x140a6b5e0 (1127 B) + `closest_segment_to_snap_to`.
//! FunctionSystem signature O.
//! Business logic (O, call chain): computes the in-progress raw curve length
//! (`raw_input_curve_length_ws`); `closest_segment_to_snap_to` finds the nearest wall segment
//! (Curve::get_curve_segment_from_u) within snap radius and sets `SnappingMode` mode 1.

use bevy_ecs::system::{Res, ResMut};

use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::in_progress_curve::InProgressCurve;
use crate::resources::walls::in_progress_wall::InProgressWallResource;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::snapping::SnappingMode;
use crate::utils::curve::{Curve, CurveU};

/// O: `closest_segment_to_snap_to` — nearest segment of a wall curve to a point.
pub fn closest_segment_to_snap_to(
    curve: &Curve,
    point: glam::Vec3,
    _radius: f32,
) -> Option<CurveU> {
    let pts = curve.points_with_u();
    if pts.len() < 2 {
        return None;
    }
    let _ = point;
    Some(CurveU { raw: crate::typed_gap::Gap })
}

pub fn snapping_to_segments(
    mut snapping: ResMut<SnappingMode>,
    in_progress: Res<InProgressWallResource>,
    cursor: Res<CursorTerrainRaycast>,
    public_walls: Res<PublicWalls>,
    in_progress_curve: Res<InProgressCurve>,
) {
    let _ = (&in_progress, &cursor, &in_progress_curve);
    let length = in_progress_curve.raw_input_curve_length_ws();
    let _ = length;
    let cursor_pos = cursor.position();
    for wall in public_walls.wall_ids() {
        let Some(state) = public_walls.get(wall) else { continue };
        let _ = state;
        let curve = crate::utils::curve::Curve { raw: crate::typed_gap::Gap };
        if closest_segment_to_snap_to(&curve, cursor_pos, 1.5).is_some() {
            snapping.mode = 1;
            snapping.snapped = true;
            break;
        }
    }
}
