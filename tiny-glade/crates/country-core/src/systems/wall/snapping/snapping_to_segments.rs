//! B `systems/wall/snapping/snapping_to_segments.rs`: wall segment snapping.
//!
//! Evidence: `chapters/systems/wall/snapping/snapping_to_segments.rs.md` (O,
//! 0x140a6b5e0). Queries the spatial hash with a ±8.0 world-unit AABB around
//! the cursor reference, then picks the **nearest** candidate (sorted) within
//! 1.5 of the cursor and writes the mode-3 segment snap (terrain height at the
//! candidate point, candidate position, wall id).

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

/// B 0x140a6b5e0 (6-param system). fresh 反编译（2026-10-17）：`query_with_aabb_clamped`
/// → `from_iter<SnapProposal, filter_map<closest_segment_to_snap_to::closure_env_0>>`
/// （每段取到参考点的最近点作 SnapProposal，fresh 0x140b16500）→ **`sort_by`
/// 取排序后最近候选**（`*local_a8`）→ `SQRT(dist) < 1.5` 才写 mode=3。
/// 参考点 = `cursor.tag & 1 != 0` ? `{pos.x, pos.z}` : `{0,0}`（disasm
/// 0x140a6b62f-0x140a6b660）。Rust 此前遍历取任意首个（无排序）且不判 tag——
/// 已重建。
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
    // fresh 0x140a6b5e0：参考点 = cursor.tag 命中 ? {pos.x, pos.z} : {0,0}
    // （disasm `TEST byte [R8],0x1` + `MOV [RBP+0x18],qword [R8+0x4]` +
    // `MOV [RBP+0x20],dword [R8+0xc]`）。
    let (cursor_x, cursor_z) = if (cursor.tag & 1) != 0 {
        (cursor.pos.x, cursor.pos.z)
    } else {
        (0.0, 0.0)
    };
    let mut candidates = spatial.query_with_aabb_clamped(
        [cursor_x - AABB_HALF_EXTENT, cursor_z - AABB_HALF_EXTENT],
        [cursor_x + AABB_HALF_EXTENT, cursor_z + AABB_HALF_EXTENT],
    );
    // 原按候选到参考点的距离排序取最近（fresh `sort_by` + `*local_a8`）。
    candidates.sort_by(|a, b| {
        segment_candidate_distance2(a.0, cursor_x, cursor_z)
            .total_cmp(&segment_candidate_distance2(b.0, cursor_x, cursor_z))
    });
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
        // fresh 0x140a6b5e0：`x0y` swizzle（utils::geo::x0y 0x140c9cdc0）把候选点
        // {x,y} 提升为 {x,0,y} 再 `sample_at_world_xz`——地形高度在 **候选(snap)点**
        // 采样，非游标。
        mode.snap = glam::Vec3::new(
            terrain.sample_at_world_xz(candidate[0], candidate[1]),
            candidate[0],
            candidate[1],
        );
        mode.wall = wall_id;
        // fresh 机器码不写 +0x18（end）；mode 3 不读 end。
        return;
    }
}

/// 排序键：候选（段最近点）到参考点距离平方（等价于原最近优先序）。
fn segment_candidate_distance2(candidate: [f32; 2], cursor_x: f32, cursor_z: f32) -> f32 {
    (candidate[0] - cursor_x).powi(2) + (candidate[1] - cursor_z).powi(2)
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
