//! Location command consumer B 0x140999330, producer B 0x140999860.
//! The separate wall-change validity pruner (0x140997410) re-validates each
//! ivy's tail/root against the wall spatial hash plus the arch height band.
use crate::resources::{
    critters::sheep::CmdCheckForSheepInsideBuilding,
    ivy::{IvyId, IvyLeafStorage, IvyStorage},
    walls::{
        public_walls::{OnWallChanged, PublicWalls},
        public_wall_state::PublicWallState,
        wall_holes::PrevWallHoles,
        WallId,
    },
};
use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::{
    closest_segment_index,
    ivy_grower::{approx_u_from_pos, wall_arch_height, wall_length, wall_segment_from_spatial},
};

/// Verified max-height filter: the pruner's `is_point_still_valid`
/// (0x140998dc0) calls closest_segment_index with 0x3f000000 = 0.5.
const MAX_IVY_GROW_HEIGHT: f32 = 0.5;
/// Point-validity radius: `0.365 <= distance` rejects the point
/// (0x140998dc0, same 0.365 as the grower's 0x142ab3190).
const POINT_VALID_RADIUS: f32 = 0.365;

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

/// Pruner tail-height band (0x140997410, LOGIC 2026-10-17):
/// `arch + (flag ? +0.2 : subtype < 2 ? (-0.79 - 0.1) : -0.46)`. This differs
/// from the grower's proposal band (`-0.79 + 0.2` for subtype < 2).
fn pruner_height_band(state: &PublicWallState) -> f32 {
    let arch = wall_arch_height(state);
    let flag = state.authority.source_flag_65() != 0;
    let subtype = state.authority.arch_type() as u32;
    if flag {
        arch + 0.2
    } else if subtype < 2 {
        arch - 0.79 - 0.1
    } else {
        arch - 0.46
    }
}

/// `is_point_still_valid` (0x140998dc0): returns the **owning wall** when the
/// point is valid, `None` otherwise. Fresh semantics:
/// 1. the wall spatial-hash cell under the point holds a segment closer than
///    0.365;
/// 2. the owning wall has ≥ 2 points and a positive curve length — the
///    original **panics** here (0x140999118..138);
/// 3. the interpolated u sits inside `(-0.01/length, 1 + 0.01/length)`
///    (0x14099913e..174);
/// 4. a previous-frame finalized wall hole whose AABB (expanded ±0.1,
///    constant 0x142abaab0) contains the `(u·length, world_y)` probe makes the
///    point **invalid** (0x140999285..2ad `Aabb2::contains` → `XOR EAX,EAX`).
///    A missing holes entry or an empty hole list leaves the point valid
///    (0x140999218 / 0x1409992b8 `mov rax,r14`).
pub fn is_point_still_valid(
    point: Vec3,
    walls: &PublicWalls,
    spatial: &WallSpatialHash,
    prev_holes: &PrevWallHoles,
) -> Option<WallId> {
    let p = Vec2::new(point.x, point.z);
    let cell = crate::resources::walls::spatial_index::cell_index_of_world_point([
        p.x,
        p.y,
    ]);
    let rows: Vec<(super::WallSegment, WallId)> = spatial
        .segments_in_cell(cell)
        .map(|s| (wall_segment_from_spatial(s), s.wall))
        .collect();
    if rows.is_empty() {
        return None;
    }
    let segments: Vec<super::WallSegment> = rows.iter().map(|(s, _)| *s).collect();
    let Ok(Some(hit)) = closest_segment_index(p, &segments, MAX_IVY_GROW_HEIGHT) else {
        return None;
    };
    if hit.distance >= POINT_VALID_RADIUS {
        return None;
    }
    let wall_id = rows[hit.index].1;
    let state = walls.get(wall_id)?;
    // Owning wall must have ≥ 2 points and a positive curve length; the
    // native **panics** on this violation (0x140999118..138).
    if state.simplified_curve_2d().points.len() < 2 || wall_length(state) <= 0.0 {
        panic!("ivy_pruner: is_point_still_valid on an invalid wall");
    }
    // Interpolated u band: `(-0.01/length, 1 + 0.01/length)`
    // (0x14099913e..174; 0.01 at 0x142960674, sign mask 0x80000000).
    let band = 0.01 / wall_length(state);
    if hit.interpolated <= -band || hit.interpolated >= band + 1.0 {
        return None;
    }
    // Finalized wall-hole gate: a point inside a previous-frame hole is not
    // valid ivy surface. Probe = `(approx_u * wall length, world y)`, the
    // hole producers' coordinate space. `[wall-0x14]` is the exact native
    // scale (待取证); the wall's simplified curve length is the recovered
    // value (same field family as `[wall-0x270]`).
    if let Some(previous) = prev_holes.wall_holes(wall_id) {
        let probe = Vec2::new(
            approx_u_from_pos(state.simplified_curve_2d(), p) * wall_length(state),
            point.y,
        );
        for hole in &previous.holes {
            let [min_x, min_y, max_x, max_y] = hole.aabb;
            if probe.x >= min_x - 0.1
                && probe.x <= max_x + 0.1
                && probe.y >= min_y - 0.1
                && probe.y <= max_y + 0.1
            {
                return None;
            }
        }
    }
    Some(wall_id)
}

/// Circle2d/Rectangle2d containment used by the dirty-wall filter
/// (0x140997a58..a98 / 0x140997aab..ae3): the shape discriminant is
/// `PublicWallState::shape().tag()` (`& 1` selects rectangle in the native;
/// freeform shapes have no 2D primitive and are skipped here).
fn wall_shape_contains(state: &PublicWallState, point: Vec2) -> bool {
    if let Some(circle) = state.is_circle() {
        circle.contains_point(point)
    } else if let Some(rectangle) = state.is_rectangle() {
        rectangle.contains_point(point)
    } else {
        false
    }
}

/// Re-validates one ivy's tail and root. Returns `true` when the root loop
/// emptied the ivy. Shared by every dirty wall, matching the native's per-event
/// per-ivy loop.
fn revalidate_ivy(
    ivy: &mut crate::resources::ivy::Ivy,
    walls: &PublicWalls,
    spatial: &WallSpatialHash,
    prev_holes: &PrevWallHoles,
    leaves: &mut IvyLeafStorage,
) -> bool {
    // Tail validity (0x140997f..): an invalid tail is removed with its leaves
    // and the in-flight growth, until the tail is valid again.
    while ivy.point_count() > 0 {
        let tail = ivy.last_point().position;
        let Some(owning_wall) = is_point_still_valid(tail, walls, spatial, prev_holes) else {
            let removed = ivy.remove_last_point().expect("ivy tail existed");
            ivy.growth_progress = 0.0;
            ivy.pending_growth = None;
            leaves.delete_affected_leaves(ivy.id, removed.id);
            continue;
        };
        // Tail height gate (0x140997bda..b30 in the main loop):
        // `if tail.y < arch + band { stop } else pop the tail`.
        let Some(state) = walls.get(owning_wall) else {
            break;
        };
        let band = pruner_height_band(state);
        if tail.y < band {
            break;
        }
        let removed = ivy.remove_last_point().expect("ivy tail existed");
        ivy.growth_progress = 0.0;
        ivy.pending_growth = None;
        leaves.delete_affected_leaves(ivy.id, removed.id);
    }
    // Root validity (0x1409984de..): an invalid root is popped and the cached
    // root copy (`Ivy::root`) is overwritten by the new front.
    while ivy.point_count() > 0 {
        let root_pos = ivy.points.front().expect("ivy root existed").position;
        let Some(_) = is_point_still_valid(root_pos, walls, spatial, prev_holes) else {
            let removed = ivy.remove_root().expect("ivy root existed");
            if let Some(front) = ivy.points.front() {
                ivy.root = *front;
            }
            leaves.delete_affected_leaves(ivy.id, removed.id);
            continue;
        };
        break;
    }
    ivy.point_count() == 0
}

/// `ivy_pruner` (0x140997410). Fresh signature (7 params, 0x1414bf4e2):
/// `(ResMut<IvyStorage>, ResMut<IvyLeafStorage>, Res<PublicWalls>,
///   Res<WallSpatialHash>, Res<PrevWallHoles>,
///   EventReader<CmdCheckForSheepInsideBuilding>,
///   EventReader<OnCircleRotated>)`.
///
/// Both events carry the changed wall id at +0x08. For each event the native
/// looks the wall up in `PublicWalls`, panics if it is invalid
/// (0x1409974f4..501), then walks every ivy and only re-validates an ivy whose
/// **tail or root** lies inside that wall's Circle2d/Rectangle2d
/// (0x140997a58..ae3). Emptied ivies are `remove_entry`-deleted from the
/// storage map after the loop (0x140998711..1312).
///
/// Cross-domain registration: `resources::events::OnCircleRotated` is not yet
/// declared in this crate. Until it lands, the crate's existing
/// `OnWallChanged` event is the equivalent dirty-wall source (same wall id
/// payload); the ECS signature must switch to the two native readers when
/// `OnCircleRotated` is recovered.
pub fn ivy_pruner(
    mut storage: ResMut<IvyStorage>,
    mut leaves: ResMut<IvyLeafStorage>,
    walls: Res<PublicWalls>,
    spatial: Res<WallSpatialHash>,
    prev_holes: Res<PrevWallHoles>,
    mut sheep_cmds: EventReader<CmdCheckForSheepInsideBuilding>,
    mut changed: EventReader<OnWallChanged>,
) {
    let mut dirty: Vec<WallId> = sheep_cmds.read().map(|command| command.wall).collect();
    dirty.extend(changed.read().map(|event| event.0));
    if dirty.is_empty() {
        return;
    }
    let mut emptied: Vec<IvyId> = Vec::new();
    for wall_id in dirty {
        let Some(state) = walls.get(wall_id) else {
            continue;
        };
        // Event-wall validity (0x1409974f4..501): the native panics.
        if state.simplified_curve_2d().points.len() < 2 || wall_length(state) <= 0.0 {
            panic!("ivy_pruner: event referenced an invalid wall");
        }
        for (_, ivy) in storage.iter_mut() {
            if ivy.point_count() == 0 {
                continue;
            }
            let tail = ivy.last_point().position;
            let root = ivy.points.front().expect("ivy root existed").position;
            let affected = wall_shape_contains(state, Vec2::new(tail.x, tail.z))
                || wall_shape_contains(state, Vec2::new(root.x, root.z));
            if !affected {
                continue;
            }
            if revalidate_ivy(ivy, &walls, &spatial, &prev_holes, &mut leaves) {
                emptied.push(ivy.id);
            }
        }
    }
    for id in emptied {
        storage.remove(id);
    }
}
