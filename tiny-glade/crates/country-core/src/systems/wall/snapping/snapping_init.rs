//! B `systems/wall/snapping/snapping_init.rs`: segment-snap candidate setup.
//!
//! Evidence: `chapters/systems/wall/snapping/snapping_init.rs.md` (O,
//! 0x140ada600). Resets mode, returns on debug disable / no in-progress wall,
//! bails when the wall curve length is <= 1.65, then walks the curve points
//! while each stays within 1.5 of the first segment; the final `snapped`
//! flag records whether any point deviated.

use bevy_ecs::prelude::*;

use super::{
    SNAP_MODE_NONE, SNAP_MODE_SEGMENT_CANDIDATE, SnappingMode,
};
use crate::resources::walls::{InnerWalls, WallId};

/// Wall curve length below which snapping is skipped (0x3fd33334 @0x142ae0d64).
const MIN_SNAP_CURVE_LENGTH: f32 = 1.65;
/// First-segment perpendicular distance threshold (0x3fc00000 @0x1429258d0).
const MAX_FIRST_SEGMENT_DISTANCE: f32 = 1.5;
/// Distance-squared division epsilon (0x1e3ce508 @0x142a7baa0).
const DIST_SQ_EPS: f32 = f32::from_bits(0x1e3ce508);

/// Local proxy for the B debug resource byte at +0xfd: when non-zero the
/// original skips snapping entirely (0x140ada600). Exact resource identity
/// 待取证.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SnappingDebugDisabled(pub bool);

/// B 0x140ada600 (4-param system).
pub fn snapping_init(
    mut mode: ResMut<SnappingMode>,
    inner: Res<InnerWalls>,
    in_progress: Res<crate::resources::in_progress_curve::InProgressCurve>,
    debug_disabled: Res<SnappingDebugDisabled>,
) {
    // 1. reset mode only; snapped survives.
    mode.mode = SNAP_MODE_NONE;
    // 2. debug flag set -> skip.
    if debug_disabled.0 {
        return;
    }
    // 3. no in-progress wall.
    let Some(wall_id) = in_progress.wall else {
        return;
    };
    let Some(state) = inner.get(wall_id) else {
        return;
    };
    let curve = &state.source().curve;
    // 5. wall curve length <= 1.65 -> skip (B reads WallState +0x30 length).
    if curve.length <= MIN_SNAP_CURVE_LENGTH {
        return;
    }
    let points = &curve.points;
    // 7. projection walk along the first segment.
    let mut stop_index = 2usize;
    if points.len() >= 2 {
        let p0 = points[0];
        let p1 = points[1];
        let mut dx = p0[0] - p1[0];
        let mut dy = p0[1] - p1[1];
        let mut dist_sq = dx * dx + dy * dy;
        if dist_sq <= 0.0 {
            dx = 0.0;
            dy = 0.0;
        }
        if dist_sq < DIST_SQ_EPS {
            dist_sq = DIST_SQ_EPS;
        }
        for point in &points[2..] {
            let t = ((point[0] - p1[0]) * dx + (point[1] - p1[1]) * dy) / dist_sq;
            let rx = point[0] - (t * p0[0] + (1.0 - t) * p1[0]);
            let ry = point[1] - (t * p0[1] + (1.0 - t) * p1[1]);
            if (rx * rx + ry * ry).sqrt() > MAX_FIRST_SEGMENT_DISTANCE {
                break;
            }
            stop_index += 1;
        }
    }
    // 8. mode = 1; snapped = end of the walk reached the curve end?
    mode.mode = SNAP_MODE_SEGMENT_CANDIDATE;
    mode.snapped = stop_index != points.len();
    mode.wall = WallId(wall_id.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{
        in_progress_curve::InProgressCurve,
        walls::{WallId, WallState},
    };
    use crate::resources::roofs::geometry::surface::Curve2;

    fn run_init(points: Vec<[f32; 2]>) -> SnappingMode {
        let curve = Curve2::try_new(points, false).unwrap();
        let mut world = World::new();
        world.insert_resource(SnappingMode::new());
        world.insert_resource(SnappingDebugDisabled(false));
        let mut inner = InnerWalls::default();
        let id = inner.new_wall(WallState::new(curve, 2.0, 0, 0, false));
        world.insert_resource(inner);
        let mut progress = InProgressCurve::new();
        progress.wall = Some(id);
        world.insert_resource(progress);
        let mut schedule = Schedule::default();
        schedule.add_systems(snapping_init);
        schedule.run(&mut world);
        *world.resource::<SnappingMode>()
    }

    #[test]
    fn straight_curve_leaves_snapped_false() {
        let mode = run_init(vec![[0.0, 0.0], [10.0, 0.0], [20.0, 0.0], [30.0, 0.0]]);
        assert_eq!(mode.mode, SNAP_MODE_SEGMENT_CANDIDATE);
        assert!(!mode.snapped);
    }

    #[test]
    fn deviating_curve_marks_snapped() {
        let mode = run_init(vec![[0.0, 0.0], [10.0, 0.0], [20.0, 2.0], [30.0, 0.0]]);
        assert_eq!(mode.mode, SNAP_MODE_SEGMENT_CANDIDATE);
        assert!(mode.snapped);
    }
}
