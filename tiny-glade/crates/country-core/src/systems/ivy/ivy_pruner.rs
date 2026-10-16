//! Location command consumer B 0x140999330, producer B 0x140999860.
//! The separate wall-change validity pruner has additional spatial dependencies.
use crate::resources::{
    ivy::{IvyLeafStorage, IvyStorage},
    walls::{public_walls::{OnWallChanged, PublicWalls}, WallId},
};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::{closest_segment_index, ivy_grower::wall_segments_from_state};

/// Verified max-height filter: the pruner's `is_point_still_valid`
/// (0x140998dc0) calls closest_segment_index with 0x3f000000 = 0.5.
const MAX_IVY_GROW_HEIGHT: f32 = 0.5;

/// Native payload is 40 bytes. The first two u64s are forwarded by the producer
/// but are not read by this consumer; retain them without invented semantics.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct RemoveIvyCmd {
    pub source_0: u64,
    pub source_8: u64,
    pub center: Vec3,
    pub dimensions: Vec3,
}
impl RemoveIvyCmd {
    pub fn at_location(
        center: Vec3,
        width: f32,
        height: f32,
        source_0: u64,
        source_8: u64,
    ) -> Self {
        Self {
            source_0,
            source_8,
            center,
            dimensions: Vec3::new(width, height, width),
        }
    }
}

pub fn remove_ivy_at_location(
    commands: &mut EventWriter<RemoveIvyCmd>,
    center: Vec3,
    width: f32,
    height: f32,
    source_0: u64,
    source_8: u64,
) {
    commands.write(RemoveIvyCmd::at_location(
        center, width, height, source_0, source_8,
    ));
}

/// Removes the suffix beginning at the first point strictly inside the box.
/// Equality and NaN reject the hit. Even an entirely removed ivy keeps its map
/// entry and cached root; the native command does not delete whole ivies.
pub fn apply_remove_ivy_cmd(
    command: &RemoveIvyCmd,
    storage: &mut IvyStorage,
    leaves: &mut IvyLeafStorage,
) {
    let half = command.dimensions * 0.5;
    for (_, ivy) in storage.iter_mut() {
        let first_hit = ivy.points.iter().position(|point| {
            let delta = (point.position - command.center).abs();
            delta.x < half.x && delta.y < half.y && delta.z < half.z
        });
        let Some(first_hit) = first_hit else {
            continue;
        };
        while ivy.point_count() > first_hit {
            let removed = ivy.remove_last_point().expect("ivy tail existed");
            ivy.growth_progress = 0.0;
            ivy.pending_growth = None;
            leaves.delete_affected_leaves(ivy.id, removed.id);
        }
    }
}

pub fn ivy_pruner_cmd(
    mut commands: EventReader<RemoveIvyCmd>,
    mut storage: ResMut<IvyStorage>,
    mut leaves: ResMut<IvyLeafStorage>,
) {
    for command in commands.read() {
        apply_remove_ivy_cmd(command, &mut storage, &mut leaves);
    }
}

/// `is_point_still_valid` (0x140998dc0): a point remains valid when the
/// nearest wall segment under it is within reach at the max ivy grow height.
/// The original first resolves the terrain cell under the point (world
/// origin/scale at resource +0x48/+0x50), then runs closest_segment_index
/// (0x14094c800) with max height 0.5 (0x3f000000), then re-checks the point
/// against the segment's additional sample points. The terrain-cell sampling
/// is replaced here by a direct nearest-wall query over the published walls
/// (待取证 for the exact cell layout).
pub fn is_point_still_valid(point: Vec3, walls: &PublicWalls) -> bool {
    let p = Vec2::new(point.x, point.z);
    for (_, state) in walls.all() {
        let segments = wall_segments_from_state(state);
        let Ok(Some(hit)) = closest_segment_index(p, &segments, MAX_IVY_GROW_HEIGHT) else {
            continue;
        };
        if hit.distance <= super::INITIAL_BEST_DISTANCE {
            return true;
        }
    }
    false
}

/// `ivy_pruner` (0x140997410): after wall changes, every ivy point is
/// re-validated against the affected wall's segments. Invalid points are
/// removed together with the leaves attached to them. The original consumes a
/// dirty-wall deque produced by wall change events and logs
/// "Spatial Hash returned a wall id " when the spatial hash owns a segment for
/// a changed wall; the event reader below is the crate's equivalent source.
pub fn ivy_pruner(
    mut changed: EventReader<OnWallChanged>,
    mut storage: ResMut<IvyStorage>,
    mut leaves: ResMut<IvyLeafStorage>,
    walls: Res<PublicWalls>,
) {
    let affected: Vec<WallId> = changed.read().map(|event| event.0).collect();
    if affected.is_empty() {
        return;
    }
    for (_, ivy) in storage.iter_mut() {
        // Points are re-validated from the tail; a hit removes the suffix so
        // later indices stay valid while iterating.
        let mut index = 0;
        while index < ivy.point_count() {
            let position = ivy.points[index].position;
            if is_point_still_valid(position, &walls) {
                index += 1;
                continue;
            }
            // Remove this point and every leaf touching it, mirroring the
            // location-command removal path.
            while ivy.point_count() > index {
                let removed = ivy.remove_last_point().expect("ivy tail existed");
                ivy.growth_progress = 0.0;
                ivy.pending_growth = None;
                leaves.delete_affected_leaves(ivy.id, removed.id);
            }
        }
    }
}
