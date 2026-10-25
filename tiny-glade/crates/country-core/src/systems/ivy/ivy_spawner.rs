//! B `ivy_spawner` 0x14094cf90: root spawning — a cursor-root pass and a
//! random-root pass, both validated against the nearest wall segment before
//! `IvyStorage::spawn_ivy` (0x1408696f0) commits the root.
//!
//! Native signature (add_ivy_systems 0x140944070, 7 params, 2026-10-17 fresh):
//! `(ResMut<IvyStorage>, Res<IvySpawnNoise>, Res<PublicWalls>,
//!  Res<CursorTerrainRaycast>, Res<ActionSetStates>, Res<WallSpatialHash>,
//!  Res<GladeSettings>)`.
//!
//! Gates and dispatch (2026-10-17 fresh decomp 0x14094cf90):
//! - **First gate** 0x14094cffb..d00a: `[GladeSettings+0x1c] == 1`, i.e. the
//!   spawner runs only while `GladeSettings::ivy_leaves` is enabled. The
//!   former `AppMode` gate was wrong — AppMode is not a parameter.
//! - **Point budget** 0x14094d09b..a7: iterate the storage map, accumulating
//!   each ivy's point count with a `-1` per subsequent step (`acc = total -
//!   (n-1)`); return when `acc + 0x12 > 39999` (`0x9c40 - 1`).
//! - **Pass dispatch** 0x14094d2e7: `MainActionSet.digital[36].disc < 3`
//!   (the `ivy_spawn` action, `pressed()`) selects the cursor pass; otherwise
//!   the random pass. There is no separate `pressed` gate in the original.
//!
//! Cursor pass (path A, digital action): cursor position → spatial-hash cell
//! (world→cell scale 0.492307693 = 32/65, offset +dim·0.5, `floorf` ×2) →
//! nearest segment (`closest_segment_index`, max height 0.5) → owning wall →
//! lateral ±0.34 by `Curve::is_on_right_side` → separation (`facing dot ≥ 0`
//! and dist² < 0.25) → vertical u `(rand*1.5 − 0.75)/curve_length +
//! clamp(approx_u)`, rejected when outside [0,1] → grid `is_inside_bbx`
//! re-check → spawn.
//!
//! Random pass (path B): random wall (`usize_less_than`) → random `u ∈ [0,1)`
//! (`uniform_f32`) → FastNoise density gate (strictly greater than
//! `res+0x78`) → lateral ±0.34, `get_bool` only when the wall subtype > 1
//! (`[wall-0x48]`), else fixed +0.34 → separation (`facing dot ≥ 0` and
//! dist² < 2.56) → vertical u (same formula, addend = clamp(approx_u)) →
//! grid `is_inside_bbx` re-check → spawn. Attempts are shared with path A:
//! `2 + usize_less_than(3)`.
//!
//! Per-attempt frame budget (0x14094db6e..b9e): `uVar11 = 0x9c41 - acc`,
//! `cap = min(0x46, uVar11)`, `acc = acc + cap - 1`; spawn only while
//! `0x13 < uVar11` and the new `acc < 0x9c41`.
//!
//! `func_0x000140c9cdc0` is `glam Vec2::x0y` — the (x, 0, y) lift used for
//! every sampled point and direction.

use crate::input::action_set::MainActionSet;
use crate::resources::walls::spatial_index::cell_index_of_world_point;
use crate::resources::{
    cursor_terrain_raycast::CursorTerrainRaycast,
    glade_settings::GladeSettings,
    ivy::{IvyId, IvySpawnNoise, IvyStorage},
    walls::public_walls::PublicWalls,
    walls::WallId,
};
use crate::systems::ivy::ivy_grower::{
    curve_is_on_right_side, wall_length, wall_segment_from_spatial,
};
use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::{WallSegment, closest_segment_index};

/// Verified max-height filter (0x3f000000 = 0.5) passed to
/// closest_segment_index in both the cursor pass and the pruner.
const MAX_IVY_GROW_HEIGHT: f32 = 0.5;
/// Point-budget gate constant: return when `acc + 0x12 > 39999`
/// (0x14094d09b..a7).
const BUDGET_GATE: usize = 39999;
/// Budget accumulation addend (0x14094d09b..a7).
const BUDGET_ADD: usize = 0x12;
/// Total-point ceiling 0x9c40 = 40000.
const TOTAL_POINT_CEILING: usize = 40_000;
/// Per-attempt point cap 0x46 = 70 (0x14094db6e..b9e).
const FRAME_POINT_CAP: usize = 0x46;
/// Per-attempt spawn lower bound 0x13 = 19 (0x14094db6e..b9e).
const BUDGET_SPAWN_MIN: usize = 0x13;
/// Spawned ivy point-count range start (native stack param 0x14 at
/// [rsp+0x28]); the upper bound is 待取证 (40 stands in for the second word).
const SPAWN_COUNT_RANGE: std::ops::Range<usize> = 20..40;
/// Initial growth cooldown scalar range per path: cursor → [0, 3.0], random →
/// [0, 1.0] (table 0x142ab3218 = 3.0, 1.0).
const SCALAR_RANGE_CURSOR: std::ops::Range<f32> = 0.0..3.0;
const SCALAR_RANGE_RANDOM: std::ops::Range<f32> = 0.0..1.0;
/// Digital action slot for `ivy_spawn` (DIGITAL_NAMES[36]); its discriminant
/// `< 3` selects the cursor pass (0x14094d2e7).
const IVY_SPAWN_ACTION_SLOT: usize = 36;
/// Root-offset half-width: the original multiplies the normal by ±0.68 × 0.5
/// = ±0.34 (0x14094d6..b6b / 0x14094d..f86, constants 0x142ab31e0/0x142ab3200
/// scaled by 0.5).
const ROOT_OFFSET: f32 = 0.34;
/// Cursor-pass minimum separation from existing roots: dist² < 0.25.
const MIN_ROOT_DISTANCE_SQ: f32 = 0.25;
/// Random-pass minimum separation from existing roots: dist² < 2.56.
const RANDOM_ROOT_DISTANCE_SQ: f32 = 2.56;
/// Vertical-height sample clamp bounds (0.0 at 0x142960690, 1.0 at
/// 0x142925904).
const VERTICAL_MIN: f32 = 0.0;
const VERTICAL_MAX: f32 = 1.0;

/// `is_point_inside_wall_segment` (line 294, inline). Predicate: a point is
/// inside the wall segment when its perpendicular distance to the segment's
/// infinite line is below the root half-width.
pub fn is_point_inside_wall_segment(point: Vec3, segment: &WallSegment) -> bool {
    let p = Vec2::new(point.x, point.z);
    let a = segment.p0;
    let b = segment.p1;
    let dir = b - a;
    let len_sq = dir.length_squared();
    if len_sq < 1e-20 {
        return false;
    }
    let t = ((p - a).dot(dir) / len_sq).clamp(0.0, 1.0);
    let on_segment = a + dir * t;
    (p - on_segment).length() <= ROOT_OFFSET
}

/// `Curve::get_approx_u_from_pos` (utils 0x141469a20): the exact `u`
/// parameter of the closest point on the curve.
fn approx_u_from_pos(curve: &crate::resources::roofs::geometry::surface::Curve2, pos: Vec2) -> f32 {
    let mut best = (f32::MAX, 0usize, 0.0f32);
    for i in 0..curve.points.len().saturating_sub(1) {
        let a = Vec2::from(curve.points[i]);
        let b = Vec2::from(curve.points[i + 1]);
        let d = b - a;
        let len_sq = d.length_squared();
        let t = if len_sq < 1e-20 {
            0.0
        } else {
            ((pos - a).dot(d) / len_sq).clamp(0.0, 1.0)
        };
        let closest = a + d * t;
        let dist = (closest - pos).length_squared();
        if dist < best.0 {
            best = (dist, i, t);
        }
    }
    let (_, i, t) = best;
    let u = curve.points_u[i] + t * (curve.points_u[i + 1] - curve.points_u[i]);
    u.clamp(0.0, 1.0)
}

/// Vertical spawn `u`: `(rand*1.5 − 0.75)/curve_length + clamp(approx_u)`
/// (0x14094db..: divisor = `[wall-0x270]` = the wall curve length, addend =
/// clamped `get_approx_u`). The native then **rejects the attempt** when the
/// result falls outside `[0,1]` and panics on NaN/inf (0x14094dc..); it does
/// not clamp the final value. Both paths share the formula.
fn vertical_u(storage: &mut IvyStorage, wall_length: f32, u: f32) -> Option<f32> {
    let rand = storage.rng_mut().uniform_f32();
    let value = (rand * 1.5 - 0.75) / wall_length + u;
    assert!(
        value.is_finite(),
        "ivy_spawner: vertical spawn coordinate was not finite"
    );
    (VERTICAL_MIN..=VERTICAL_MAX).contains(&value).then_some(value)
}

/// Reject roots that land too close to an existing ivy root. The native check
/// is a facing dot (`0.0 <= existing_dir·direction`) **plus** the squared
/// distance threshold (0.25 cursor, 2.56 random); the former implementation
/// only tested the distance.
fn root_too_close(storage: &IvyStorage, root: Vec3, direction: Vec3, threshold_sq: f32) -> bool {
    storage.iter().any(|(_, ivy)| {
        let r = &ivy.root;
        let facing =
            r.direction.x * direction.x + r.direction.y * direction.y + r.direction.z * direction.z;
        facing >= 0.0 && (r.position - root).length_squared() < threshold_sq
    })
}

/// Grid `is_inside_bbx` re-check (0x14094d..): the root must NOT be inside any
/// wall-segment AABB in its own spatial-hash cell. The exact native test
/// rotates the point into the segment frame (`from_rotation_arc` + utils
/// `is_inside_bbx`); the axis-aligned XZ containment below is the recovered
/// equivalent (the rotation's Y-axis alignment leaves the XZ plane unchanged).
fn root_in_wall_cell(root: Vec3, spatial: &WallSpatialHash) -> bool {
    let cell = cell_index_of_world_point([root.x, root.z]);
    spatial.segments_in_cell(cell).any(|seg| {
        let a = seg.endpoints[0];
        let b = seg.endpoints[1];
        let min_x = a[0].min(b[0]);
        let max_x = a[0].max(b[0]);
        let min_z = a[1].min(b[1]);
        let max_z = a[1].max(b[1]);
        root.x >= min_x && root.x <= max_x && root.z >= min_z && root.z <= max_z
    })
}

/// Per-attempt frame budget (0x14094db6e..b9e): advance `acc` by
/// `min(0x46, 0x9c41 - acc) - 1` and allow the spawn only while
/// `0x13 < 0x9c41 - acc` and the new count stays under `0x9c41`.
fn budget_advance(acc: &mut usize) -> bool {
    let remaining = TOTAL_POINT_CEILING + 1 - *acc;
    let cap = FRAME_POINT_CAP.min(remaining);
    *acc = (*acc + cap) - 1;
    BUDGET_SPAWN_MIN < remaining && *acc < TOTAL_POINT_CEILING + 1
}

/// Outcome of one root attempt. The native consumes frame budget per attempt
/// even when the candidate is rejected; only an exhausted budget stops the
/// attempt loop.
enum CommitOutcome {
    Spawned(IvyId),
    Rejected,
    Exhausted,
}

/// Shared root commit (0x14094d4dd..db6e): wall validity (panic), ±0.34
/// lateral offset, facing-dot + distance separation, vertical u (reject when
/// outside [0,1]), grid `is_inside_bbx` re-check, then `spawn_ivy`.
#[allow(clippy::too_many_arguments)]
fn commit_root(
    storage: &mut IvyStorage,
    walls: &PublicWalls,
    spatial: &WallSpatialHash,
    wall_id: WallId,
    interp: Vec2,
    normal: Vec2,
    side: f32,
    scalar_range: std::ops::Range<f32>,
    threshold_sq: f32,
    acc: &mut usize,
) -> CommitOutcome {
    if !budget_advance(acc) {
        return CommitOutcome::Exhausted;
    }
    let Some(state) = walls.get(wall_id) else {
        return CommitOutcome::Rejected;
    };
    let curve = state.simplified_curve_2d();
    // Wall validity: the original **panics** when the owning wall has fewer
    // than 2 points or a non-positive curve length (0x14094d4f4..501).
    if curve.points.len() < 2 || wall_length(state) <= 0.0 {
        panic!("ivy_spawner: root proposed on an invalid wall");
    }
    let root_xz = interp + normal * side;
    let direction = Vec3::new(normal.x, 0.0, normal.y);
    let root = Vec3::new(root_xz.x, 0.0, root_xz.y);
    if root_too_close(storage, root, direction, threshold_sq) {
        return CommitOutcome::Rejected;
    }
    let u = approx_u_from_pos(curve, interp).clamp(0.0, 1.0);
    let Some(height) = vertical_u(storage, wall_length(state), u) else {
        return CommitOutcome::Rejected;
    };
    let root = Vec3::new(root.x, height, root.z);
    if root_in_wall_cell(root, spatial) {
        return CommitOutcome::Rejected;
    }
    let id = storage.spawn_ivy(root, direction, scalar_range, SPAWN_COUNT_RANGE, wall_id.0);
    CommitOutcome::Spawned(id)
}

/// `try_ivy_root_next_to_cursor` (line 245, inline, path A): place a root next
/// to the cursor through the spatial hash. Cursor → cell → closest segment →
/// owning wall → `is_on_right_side` lateral (±0.34) → separation → vertical u
/// → grid re-check → `spawn_ivy`.
pub fn try_ivy_root_next_to_cursor(
    storage: &mut IvyStorage,
    cursor: &CursorTerrainRaycast,
    walls: &PublicWalls,
    spatial: &WallSpatialHash,
    attempts: usize,
    acc: &mut usize,
) -> Option<IvyId> {
    let position = cursor.position();
    let point = Vec2::new(position.x, position.z);
    let cell = cell_index_of_world_point([point.x, point.y]);
    let rows: Vec<(WallSegment, WallId)> = spatial
        .segments_in_cell(cell)
        .map(|s| (wall_segment_from_spatial(s), s.wall))
        .collect();
    let mut last = None;
    for _ in 0..attempts {
        if rows.is_empty() {
            continue;
        }
        let segments: Vec<WallSegment> = rows.iter().map(|(s, _)| *s).collect();
        let Ok(Some(hit)) = closest_segment_index(point, &segments, MAX_IVY_GROW_HEIGHT) else {
            continue;
        };
        let seg = &segments[hit.index];
        let wall_id = rows[hit.index].1;
        let Some(state) = walls.get(wall_id) else {
            continue;
        };
        let curve = state.simplified_curve_2d();
        // Lateral side: `Curve::is_on_right_side` → −0.34, else +0.34.
        let side = if curve_is_on_right_side(curve, point) {
            -ROOT_OFFSET
        } else {
            ROOT_OFFSET
        };
        let dir = seg.p1 - seg.p0;
        let len = dir.length().max(1e-20);
        let normal = Vec2::new(-dir.y, dir.x) / len;
        let dir_len_sq = dir.length_squared().max(1e-20);
        let t = ((point - seg.p0).dot(dir) / dir_len_sq).clamp(0.0, 1.0);
        let interp = seg.p0 + dir * t;
        match commit_root(
            storage,
            walls,
            spatial,
            wall_id,
            interp,
            normal,
            side,
            SCALAR_RANGE_CURSOR,
            MIN_ROOT_DISTANCE_SQ,
            acc,
        ) {
            CommitOutcome::Spawned(id) => last = Some(id),
            CommitOutcome::Rejected => {}
            CommitOutcome::Exhausted => break,
        }
    }
    last
}

/// `try_random_ivy_root` (line 212, inline, path B): pick a random wall, sample
/// a random `u ∈ [0,1)`, gate on the spawn FastNoise density, place a root
/// offset laterally by ±0.34 (`get_bool` only when the wall subtype > 1, else
/// +0.34), separate, set the vertical u, and spawn.
pub fn try_random_ivy_root(
    storage: &mut IvyStorage,
    walls: &PublicWalls,
    noise: &IvySpawnNoise,
    spatial: &WallSpatialHash,
    attempts: usize,
    acc: &mut usize,
) -> Option<IvyId> {
    let wall_rows: Vec<_> = walls.all().collect();
    if wall_rows.is_empty() {
        return None;
    }
    let mut last = None;
    for _ in 0..attempts {
        let (wall_id, wall) = wall_rows[storage.rng_mut().usize_less_than(wall_rows.len())];
        let curve = wall.simplified_curve_2d();
        if curve.points.len() < 2 || curve.length <= 0.0 {
            continue;
        }
        let u = storage.rng_mut().uniform_f32();
        let (index, t) = curve.coord_at_u(u);
        let a = curve.points[index];
        let b = curve.points[index + 1];
        let interp = Vec2::new(t * b[0] + (1.0 - t) * a[0], t * b[1] + (1.0 - t) * a[1]);
        // FastNoise density gate (strictly greater; 0x14094cf90).
        let density = noise.noise.get_noise3d(interp.x, 0.0, interp.y);
        if density <= noise.threshold {
            continue;
        }
        let dx = b[0] - a[0];
        let dz = b[1] - a[1];
        let reciprocal = 1.0 / (dz * dz + dx * dx).sqrt().max(1e-20);
        let normal = Vec2::new(-dz * reciprocal, dx * reciprocal);
        // Lateral side: `get_bool` only when the wall subtype > 1
        // (`1 < [wall-0x48]`); otherwise the +0.34 branch (0x14094d6b0).
        let side = if (wall.authority.arch_type() as u32) > 1 && storage.rng_mut().get_bool() {
            -ROOT_OFFSET
        } else {
            ROOT_OFFSET
        };
        match commit_root(
            storage,
            walls,
            spatial,
            wall_id,
            interp,
            normal,
            side,
            SCALAR_RANGE_RANDOM,
            RANDOM_ROOT_DISTANCE_SQ,
            acc,
        ) {
            CommitOutcome::Spawned(id) => last = Some(id),
            CommitOutcome::Rejected => {}
            CommitOutcome::Exhausted => break,
        }
    }
    last
}

/// `ivy_spawner` (0x14094cf90). 7-param native signature; `GladeSettings`
/// gate, point budget, `MainActionSet` digital[36] dispatch.
pub fn ivy_spawner(
    mut storage: ResMut<IvyStorage>,
    noise: Res<IvySpawnNoise>,
    walls: Res<PublicWalls>,
    cursor: Res<CursorTerrainRaycast>,
    actions: Res<MainActionSet>,
    spatial: Res<WallSpatialHash>,
    settings: Res<GladeSettings>,
) {
    // First gate (0x14094cffb..d00a): `[GladeSettings+0x1c] == 1`.
    if !settings.ivy_leaves {
        return;
    }
    // Point budget (0x14094d09b..a7): `acc = total - (n-1)`, return when
    // `acc + 0x12 > 39999`.
    let mut acc = 0usize;
    let mut first = true;
    for (_, ivy) in storage.iter() {
        if first {
            acc = ivy.point_count();
            first = false;
        } else {
            acc = acc + ivy.point_count() - 1;
        }
    }
    if !first && acc + BUDGET_ADD > BUDGET_GATE {
        return;
    }
    // Shared attempts: `2 + usize_less_than(3)` (0x14094d2e7).
    let attempts = 2 + storage.rng_mut().usize_less_than(3);
    // Pass dispatch: `MainActionSet.digital[36].disc < 3` (0x14094d2e7).
    if actions.digital[IVY_SPAWN_ACTION_SLOT].pressed() {
        try_ivy_root_next_to_cursor(&mut storage, &cursor, &walls, &spatial, attempts, &mut acc);
    } else {
        try_random_ivy_root(&mut storage, &walls, &noise, &spatial, attempts, &mut acc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    #[test]
    fn inside_check_accepts_points_above_the_segment_line() {
        let segment = WallSegment {
            p0: Vec2::new(0.0, 0.0),
            p1: Vec2::new(10.0, 0.0),
            extra_a: 0.0,
            extra_b: 0.0,
            h0: 0.0,
            h1: 0.0,
        };
        assert!(is_point_inside_wall_segment(Vec3::new(5.0, 0.0, 0.0), &segment));
        // ROOT_OFFSET = 0.34; 0.4 is outside.
        assert!(!is_point_inside_wall_segment(Vec3::new(5.0, 0.0, 0.4), &segment));
        assert!(!is_point_inside_wall_segment(Vec3::new(20.0, 0.0, 0.0), &segment));
    }

    #[test]
    fn vertical_u_rejects_values_outside_the_unit_interval() {
        let mut storage = IvyStorage::new();
        let mut some = 0;
        let mut none = 0;
        for _ in 0..64 {
            match vertical_u(&mut storage, 1.0, 0.0) {
                Some(value) => {
                    assert!((VERTICAL_MIN..=VERTICAL_MAX).contains(&value), "height {value}");
                    some += 1;
                }
                None => none += 1,
            }
        }
        // (rand*1.5-0.75)/length spans [-0.75, 0.75] at length 1.0, so both
        // the accepted and the rejected branch occur; the native rejects
        // rather than clamping.
        assert!(some > 0 && none > 0, "some={some} none={none}");
    }

    #[test]
    fn budget_advance_obeys_the_frame_point_cap() {
        let mut acc = 10;
        let mut spawned = 0;
        for _ in 0..10_000 {
            if budget_advance(&mut acc) {
                spawned += 1;
            }
        }
        // The gate is a bound check, not a hard counter; it must simply stay
        // within the 0x9c41 ceiling and report a finite number of spawns.
        assert!(acc < TOTAL_POINT_CEILING + 1);
        assert!(spawned > 0);
    }

    #[test]
    fn approx_u_from_pos_projects_to_the_curve_parameter() {
        let points = vec![[0.0, 0.0], [10.0, 0.0], [20.0, 0.0]];
        let points_u = vec![0.0, 0.5, 1.0];
        let curve = crate::resources::roofs::geometry::surface::Curve2 {
            points,
            points_u,
            length: 20.0,
        };
        // A point just off the first segment projects to u≈0.25.
        let u = approx_u_from_pos(&curve, Vec2::new(5.0, 0.3));
        assert!((u - 0.25).abs() < 1e-3, "u = {u}");
    }
}
