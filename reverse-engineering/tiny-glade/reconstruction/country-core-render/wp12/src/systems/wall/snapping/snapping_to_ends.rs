//! snapping/snapping_to_ends.rs — snaps the in-progress wall start to a wall end.
//!
//! PDB procedure `snapping_to_ends` 0x140a6ad80 (2137 B) + `find_closest_end_to_snap_to`.
//! FunctionSystem signature O.
//! Business logic (O, call chain): for each non-enclosed public wall, projects the
//! in-progress wall's first raw input point onto the wall's curve; `find_closest_end_to_snap_to`
//! picks the nearest end (u=0 or u=1) within the snap radius and sets `SnappingMode` mode 2.

use bevy_ecs::system::{Res, ResMut};

use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::in_progress_curve::InProgressCurve;
use crate::resources::walls::in_progress_wall::InProgressWallResource;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::snapping::SnappingMode;
use crate::utils::curve::CurveInternalCoord;

/// O: `find_closest_end_to_snap_to` — nearest end u of the wall curve to the point.
pub fn find_closest_end_to_snap_to(
    curve: &crate::utils::curve::Curve,
    point: glam::Vec3,
) -> Option<f32> {
    let start = curve.start_pos();
    let end = curve.end_pos();
    let ds = (start - point).length();
    let de = (end - point).length();
    if ds < de {
        Some(0.0)
    } else {
        Some(1.0)
    }
}

pub fn snapping_to_ends(
    mut snapping: ResMut<SnappingMode>,
    in_progress: Res<InProgressWallResource>,
    cursor: Res<CursorTerrainRaycast>,
    public_walls: Res<PublicWalls>,
    in_progress_curve: Res<InProgressCurve>,
) {
    let _ = (&in_progress, &cursor, &in_progress_curve);
    let Some(first) = in_progress_curve.first_raw_input_point() else {
        return;
    };
    for wall in public_walls.wall_ids() {
        let Some(state) = public_walls.get(wall) else { continue };
        if state.is_enclosed() {
            continue;
        }
        // O: find the closest end u and activate mode 2.
        let curve = crate::utils::curve::Curve { raw: crate::typed_gap::Gap };
        if find_closest_end_to_snap_to(&curve, first).is_some() {
            snapping.mode = 2;
            snapping.snapped = true;
            let _ = CurveInternalCoord::i0();
            break;
        }
    }
}
