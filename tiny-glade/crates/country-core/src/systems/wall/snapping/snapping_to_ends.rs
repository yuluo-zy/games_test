//! B `systems/wall/snapping/snapping_to_ends.rs`: wall-end snapping.
//!
//! Evidence: `chapters/systems/wall/snapping/snapping_to_ends.rs.md` (O,
//! 0x140a6ad80). Only freehand walls participate; the in-progress wall uses
//! its first raw input point as the candidate endpoint. The closest end within
//! 1.5 world units becomes the mode-2 snap target.

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::{SNAP_MODE_END, SNAP_MODE_SEGMENT_CANDIDATE, SnappingMode};
use crate::recovered_building::decorator_subtype::nearest_segment;
use crate::resources::{
    cursor_terrain_raycast::CursorTerrainRaycast,
    in_progress_curve::InProgressCurve,
    walls::public_walls::PublicWalls,
};

/// Minimum distance sentinel (0x4cbebc20 @0x142ad4950).
const MIN_DISTANCE_SENTINEL: f32 = 1.0e8;
/// End-snap distance threshold (0x3fc00000 @0x1429258d0).
const SNAP_DISTANCE: f32 = 1.5;

/// B 0x140a6ad80 (4-param system).
pub fn snapping_to_ends(
    mut mode: ResMut<SnappingMode>,
    in_progress: Res<InProgressCurve>,
    cursor: Res<CursorTerrainRaycast>,
    public: Res<PublicWalls>,
) {
    if mode.mode != SNAP_MODE_SEGMENT_CANDIDATE {
        return;
    }
    if in_progress.wall.is_none() {
        return;
    }
    let cursor = Vec2::new(cursor.pos.x, cursor.pos.z);
    let mut snapped = mode.snapped;
    let mut best: Option<(crate::resources::walls::WallId, [f32; 2], u8)> = None;
    let mut min_distance = MIN_DISTANCE_SENTINEL;

    for (id, state) in public.all() {
        if snapped {
            break;
        }
        state.assert_valid();
        if state.shape().tag() != 2 {
            continue; // only freehand walls expose ends to snap to.
        }
        if id == in_progress.current_wall {
            // The in-progress wall reuses its first raw input point; its flag
            // bit 0 forces the occupancy flag and stops further probing.
            if in_progress.raw_input_points.is_empty() {
                continue;
            }
            let point = in_progress.first_raw_input_point();
            if in_progress.flags & 1 != 0 {
                snapped = true;
            }
            let distance = (point - cursor).length();
            if distance < min_distance {
                min_distance = distance;
                best = Some((id, [point.x, point.y], 0));
            }
            continue;
        }
        let points = state.simplified_curve_2d().points.clone();
        if points.len() < 2 {
            continue;
        }
        let start = points[0];
        let end = points[points.len() - 1];
        let d_start = ((start[0] - cursor.x).powi(2) + (start[1] - cursor.y).powi(2)).sqrt();
        let d_end = ((end[0] - cursor.x).powi(2) + (end[1] - cursor.y).powi(2)).sqrt();
        let (candidate, end_flag) = if d_start < d_end { (start, 0u8) } else { (end, 1u8) };
        let distance = if d_start < d_end { d_start } else { d_end };
        if distance < min_distance {
            min_distance = distance;
            best = Some((id, candidate, end_flag));
        }
    }

    if min_distance >= SNAP_DISTANCE {
        return;
    }
    let Some((best_id, best_point, best_end)) = best else {
        return;
    };
    let state = public.get(best_id).unwrap();
    // 3D snap point: project the 2D candidate onto the terrain-mapped outline.
    let (index, t) = nearest_segment(&state.geometry.outline, [best_point[0], 0.0, best_point[1]])
        .expect("wall snap target outline");
    let a = state.geometry.outline[index];
    let b = state.geometry.outline[index + 1];
    let snap = [
        b[0] * t + a[0] * (1.0 - t),
        b[1] * t + a[1] * (1.0 - t),
        b[2] * t + a[2] * (1.0 - t),
    ];

    mode.mode = SNAP_MODE_END;
    mode.snapped = state.building_projection().byte_0x29e() != 0;
    mode.snap = glam::Vec3::new(snap[0], snap[1], snap[2]);
    mode.wall = best_id;
    mode.end = best_end;
}
