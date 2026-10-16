//! B `systems/wall/snapping/snapping_to_segments.rs`: wall segment snapping.
//!
//! Evidence: `chapters/systems/wall/snapping/snapping_to_segments.rs.md` (O,
//! 0x140a6b5e0). Queries the spatial hash with a ±8.0 world-unit AABB around
//! the cursor, then picks the first candidate within 1.5 of the cursor and
//! writes the mode-3 segment snap (terrain height at the cursor, candidate
//! position, wall id).

use bevy_ecs::prelude::*;

use super::{SNAP_MODE_SEGMENT, SNAP_MODE_SEGMENT_CANDIDATE, SnappingMode};
use crate::resources::{
    cursor_terrain_raycast::CursorTerrainRaycast,
    in_progress_curve::InProgressCurve,
    terrain_height::{TerrainHeightsData, TerrainHeightSsbo},
    walls::public_walls::PublicWalls,
};
use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

/// Segment-snap AABB half extent (±8.0, 0xc0100000/0x40100000 @0x142ab6630).
const AABB_HALF_EXTENT: f32 = 8.0;
/// Candidate distance threshold (0x3fc00000 @0x1429258d0).
const SNAP_DISTANCE: f32 = 1.5;

/// B 0x140a6b5e0 (6-param system).
pub fn snapping_to_segments(
    mut mode: ResMut<SnappingMode>,
    in_progress: Res<InProgressCurve>,
    cursor: Res<CursorTerrainRaycast>,
    public: Res<PublicWalls>,
    spatial: Res<WallSpatialHash>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
) {
    if mode.mode != SNAP_MODE_SEGMENT_CANDIDATE {
        return;
    }
    if in_progress.wall.is_none() {
        return;
    }
    let cursor_x = cursor.pos.x;
    let cursor_z = cursor.pos.z;
    let candidates = spatial.query_with_aabb_clamped(
        [cursor_x - AABB_HALF_EXTENT, cursor_z - AABB_HALF_EXTENT],
        [cursor_x + AABB_HALF_EXTENT, cursor_z + AABB_HALF_EXTENT],
    );
    for (candidate, wall_id) in candidates {
        let distance =
            ((candidate[0] - cursor_x).powi(2) + (candidate[1] - cursor_z).powi(2)).sqrt();
        if distance >= SNAP_DISTANCE {
            continue;
        }
        let Some(state) = public.get(wall_id) else {
            continue;
        };
        state.assert_valid();
        mode.mode = SNAP_MODE_SEGMENT;
        mode.snapped = state.building_projection().byte_0x29e() != 0;
        mode.snap = glam::Vec3::new(
            terrain.sample_at_world_xz(cursor_x, cursor_z),
            candidate[0],
            candidate[1],
        );
        mode.wall = wall_id;
        mode.end = 0;
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::WallId;

    fn run_segments(mode: u8, has_progress: bool) -> u8 {
        let mut world = World::new();
        world.insert_resource(SnappingMode::new());
        world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(PublicWalls::default());
        world.insert_resource(WallSpatialHash::default());
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        world.resource_mut::<SnappingMode>().mode = mode;
        if has_progress {
            let mut progress = InProgressCurve::new();
            progress.wall = Some(WallId(7));
            world.insert_resource(progress);
        } else {
            world.insert_resource(InProgressCurve::new());
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(snapping_to_segments);
        schedule.run(&mut world);
        world.resource::<SnappingMode>().mode
    }

    #[test]
    fn mode_and_empty_progress_gates_return_early() {
        assert_eq!(run_segments(SNAP_MODE_SEGMENT, false), SNAP_MODE_SEGMENT);
        assert_eq!(
            run_segments(SNAP_MODE_SEGMENT_CANDIDATE, false),
            SNAP_MODE_SEGMENT_CANDIDATE
        );
        assert_eq!(
            run_segments(SNAP_MODE_SEGMENT_CANDIDATE, true),
            SNAP_MODE_SEGMENT_CANDIDATE
        );
    }
}
