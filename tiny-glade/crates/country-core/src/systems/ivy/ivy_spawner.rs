//! B `ivy_spawner` 0x14094cf90: root spawning — a cursor-root pass and a
//! random-root pass, both validated against the nearest wall segment before
//! `IvyStorage::spawn_ivy` (0x1408696f0) commits the root.
//!
//! O-level gates (2026-10-15 re-analyst, chapters ivy_spawner.rs.md +
//! RE-ANALYSIS2):
//! - **mode gate**: `if (*(byte)(*AppMode + 0x1c) != 0x01) return;` — the ivy
//!   spawner runs only in wall-editing mode (`AppMode::Editing(Wall)`, native
//!   tag 1). The exact +0x1c byte offset is 待取证; `native_tag == 1` is the
//!   resolved semantic.
//! - **ivy-count budget**: `total_ivy_count > 39999 + 0x12` → return, i.e.
//!   ~40,000 stored ivies.
//! - **pass dispatch**: `count = *(u32*)(IvyStorage + 0x120)` — below 3 the
//!   cursor pass (`try_ivy_root_next_to_cursor`) runs, at 3+ the random pass
//!   (`try_random_ivy_root`) runs.
//!
//! Cursor pass (path A, count < 3): cursor position → RNG-sampled spatial-hash
//! cell (world→cell scale 0.492307693 = 32/65, offset 32.0) → nearest segment
//! (`closest_segment_index`, max height 0.5 = 0x3f000000) → root offset ±0.34
//! (0.68 × 0.5) along the wall normal → reject when within 0.5 (0.25²) of an
//! existing root → vertical height `(rand*1.5 − 0.75)/wall_height +
//! segment_height` clamped to [0.000928482, 1.0]. Up to 3 attempts.
//!
//! Random pass (path B, count >= 3): pick a random wall
//! (`usize_less_than(wall_count)`, @0x14094d2e7), sample `u ∈ [0,1)`
//! (`uniform_f32`, @0x14094d394), gate on the spawn FastNoise density
//! (`noise >= res+0x78`, threshold 待取证), offset the root laterally by the
//! `perchance get_bool` side (`true → −0.68, false → +0.68`, @0x14094d6b0),
//! reject roots within 1.6 (2.56²) of an existing root, and set the vertical
//! spawn u to `(uniform_f32×1.5 − 0.75)/curve_length + u_on_curve` (exact
//! `get_approx_u_from_pos` projection), clamped. Attempts = `2 +
//! usize_less_than(3)` ∈ {2,3,4}.
//!
//! `func_0x000140c9cdc0` is `glam Vec2::x0y` (S_PUB32 swizzle) — the
//! "wall-segment geometry helper" is the Vec2 → (x, 0, y) lift used for every
//! sampled point. The spatial-hash cell sampling of path A is modeled with the
//! crate's `WallSpatialHash` / `wall_segments_from_state` helpers (待取证 for
//! exact cell equivalence).

use crate::resources::{
    cursor_terrain_raycast::CursorTerrainRaycast,
    ivy::{IvyId, IvySpawnNoise, IvyStorage},
    walls::public_walls::PublicWalls,
};
use crate::systems::mode_manager::AppMode;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::{WallSegment, closest_segment_index, ivy_grower::wall_segments_from_state};

/// Verified max-height filter (0x3f000000 = 0.5) passed to
/// closest_segment_index in both the cursor pass and the pruner.
const MAX_IVY_GROW_HEIGHT: f32 = 0.5;
/// ~40,000 ivy ceiling (0x14094cf90: `39999 < total + 0x12` → return).
const MAX_IVY_COUNT: u64 = 40_000;
/// Cursor-pass root-offset half-width: 0.68 × 0.5 = 0.34 (constants
/// 0x142ab31e0 / 0x142ab3200 multiplied by 0.5 at 0x142925870/8e0/e4).
const ROOT_OFFSET: f32 = 0.34;
/// Random-pass root-offset magnitude: 0.68, selected directly by `get_bool`
/// (RE-ANALYSIS2: true → −0.68, false → +0.68; call 0x14094d6b0).
const RANDOM_ROOT_OFFSET: f32 = 0.68;
/// Cursor-pass minimum separation from existing roots: 0.25 (distance 0.5,
/// constant 0x142925950).
const MIN_ROOT_DISTANCE_SQ: f32 = 0.25;
/// Random-pass minimum separation from existing roots: 2.56 (distance 1.6,
/// constant 0x142ab3214).
const RANDOM_ROOT_DISTANCE_SQ: f32 = 2.56;
/// Vertical-height sample clamp lower bound (0x142960690).
const VERTICAL_MIN: f32 = 0.000928482;
/// Vertical-height sample clamp upper bound (1.0 at 0x142925904).
const VERTICAL_MAX: f32 = 1.0;
/// Cursor-pass spawn attempt ceiling (0x142ab3218 = 3).
const SPAWN_ATTEMPTS: usize = 3;

/// `is_point_inside_wall_segment` (line 294, inline). Predicate: a point is
/// inside the wall segment when its perpendicular distance to the segment's
/// infinite line is below the root half-width, matching where a root would be
/// placed.
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

/// Reject roots that land too close to an existing ivy root. The native check
/// is a facing dot (`0.0 <= existing_dir·(new − existing)`) plus the squared
/// distance threshold (0.25 for the cursor pass, 2.56 for the random pass).
fn root_too_close(storage: &IvyStorage, position: Vec3, threshold_sq: f32) -> bool {
    storage
        .roots()
        .any(|root| (root - position).length_squared() < threshold_sq)
}

/// Wall double-sided normal at the nearest segment under `point`, together
/// with the segment and the hit's interpolated value (lerp of the segment
/// height pair at the clamped hit `t`).
fn nearest_wall_normal(walls: &PublicWalls, point: Vec2) -> Option<(Vec2, WallSegment, f32)> {
    let mut best: Option<(f32, Vec2, WallSegment, f32)> = None;
    for (_, state) in walls.all() {
        let segments = wall_segments_from_state(state);
        let Ok(Some(hit)) = closest_segment_index(point, &segments, MAX_IVY_GROW_HEIGHT) else {
            continue;
        };
        let seg = &segments[hit.index];
        let dir = seg.p1 - seg.p0;
        let len = dir.length().max(1e-20);
        let normal = Vec2::new(-dir.y, dir.x) / len;
        match best {
            Some((d, _, _, _)) if d <= hit.distance => {}
            _ => best = Some((hit.distance, normal, *seg, hit.interpolated)),
        }
    }
    best.map(|(_, normal, seg, interpolated)| (normal, seg, interpolated))
}

/// `Curve::get_approx_u_from_pos` (FUN_141469a20, S_PUB32): the exact `u`
/// parameter of the closest point on the curve. Shared-utils cross-domain;
/// the projection below walks the polyline and interpolates `points_u`.
fn approx_u_from_pos(curve: &system_roof::surface::Curve2, pos: Vec2) -> f32 {
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

/// Vertical spawn `u`: `(rand*1.5 − 0.75)/divisor + addend`, clamped to
/// [0.000928482, 1.0]. For the cursor pass the divisor is the wall's max ivy
/// grow height (待取证 for the getter; 1.0 stands in) and the addend is the
/// hit segment's interpolated height. For the random pass the divisor is the
/// wall curve length and the addend is `u_on_curve` (exact projection).
fn vertical_u(storage: &mut IvyStorage, divisor: f32, addend: f32) -> f32 {
    let rand = storage.rng_mut().uniform_f32();
    let value = (rand * 1.5 - 0.75) / divisor.max(1e-20) + addend;
    value.clamp(VERTICAL_MIN, VERTICAL_MAX)
}

/// `try_ivy_root_next_to_cursor` (line 245, inline, path A): place a root next
/// to the cursor. Nearest wall segment under the cursor → root offset ±0.34 →
/// separation check → vertical height → `spawn_ivy`. Up to `SPAWN_ATTEMPTS`
/// tries.
pub fn try_ivy_root_next_to_cursor(
    storage: &mut IvyStorage,
    cursor: &CursorTerrainRaycast,
    walls: &PublicWalls,
) -> Option<IvyId> {
    let position = cursor.position();
    let point = Vec2::new(position.x, position.z);
    for _ in 0..SPAWN_ATTEMPTS {
        let Some((normal, _, interpolated)) = nearest_wall_normal(walls, point) else {
            continue;
        };
        let root = Vec3::new(point.x, 0.0, point.y) + Vec3::new(normal.x, 0.0, normal.y) * ROOT_OFFSET;
        if root_too_close(storage, root, MIN_ROOT_DISTANCE_SQ) {
            continue;
        }
        let direction = Vec3::new(normal.x, 0.0, normal.y);
        let height = vertical_u(storage, 1.0, interpolated);
        return Some(spawn_validated_root(storage, Vec3::new(root.x, height, root.z), direction));
    }
    None
}

/// `try_random_ivy_root` (line 212, inline, path B): pick a random wall, sample
/// a random `u ∈ [0,1)`, gate on the spawn FastNoise density, place a root at
/// the wall curve point offset laterally by the `get_bool` side (±0.34),
/// reject roots within 1.6 of an existing root (2.56²), set the vertical `u`,
/// and spawn. Attempts = 2..4 (re-analyst RE-ANALYSIS2).
pub fn try_random_ivy_root(
    storage: &mut IvyStorage,
    walls: &PublicWalls,
    noise: &IvySpawnNoise,
) -> Option<IvyId> {
    let wall_rows: Vec<_> = walls.all().collect();
    if wall_rows.is_empty() {
        return None;
    }
    let attempts = 2 + storage.rng_mut().usize_less_than(3);
    let mut last = None;
    for _ in 0..attempts {
        let wall = wall_rows[storage.rng_mut().usize_less_than(wall_rows.len())].1;
        let curve = wall.simplified_curve_2d();
        if curve.points.len() < 2 || curve.length <= 0.0 {
            continue;
        }
        let u = storage.rng_mut().uniform_f32();
        let (index, t) = curve.coord_at_u(u);
        let a = curve.points[index];
        let b = curve.points[index + 1];
        let x = t * b[0] + (1.0 - t) * a[0];
        let z = t * b[1] + (1.0 - t) * a[1];
        let dx = b[0] - a[0];
        let dz = b[1] - a[1];
        let reciprocal = 1.0 / (dz * dz + dx * dx).sqrt().max(1e-20);
        // FastNoise density gate: `noise >= threshold` (threshold at res+0x78,
        // 待取证 for the exact source; the resource's `threshold` field).
        let density = noise.noise.get_noise3d(x, 0.0, z);
        if density < noise.threshold {
            continue;
        }
        // Lateral side: `perchance get_bool` — true → −0.68, false → +0.68
        // (RE-ANALYSIS2, call 0x14094d6b0). Path A uses 0.68 × 0.5 = 0.34.
        let side = if storage.rng_mut().get_bool() { -1.0 } else { 1.0 };
        let offset_x = (-dz * reciprocal) * RANDOM_ROOT_OFFSET * side;
        let offset_z = (dx * reciprocal) * RANDOM_ROOT_OFFSET * side;
        let root = Vec3::new(x + offset_x, 0.0, z + offset_z);
        if root_too_close(storage, root, RANDOM_ROOT_DISTANCE_SQ) {
            continue;
        }
        let u_on_curve = approx_u_from_pos(curve, Vec2::new(x, z));
        let height = vertical_u(storage, curve.length, u_on_curve);
        let direction = Vec3::new(-dz * reciprocal, 0.0, dx * reciprocal);
        last = Some(spawn_validated_root(storage, Vec3::new(root.x, height, root.z), direction));
    }
    last
}

/// Shared spawn commit: root position/direction with the point budget sampled
/// from 4..16 so a new root has room to grow (待取证 for the original range).
fn spawn_validated_root(storage: &mut IvyStorage, position: Vec3, direction: Vec3) -> IvyId {
    storage.spawn_ivy(position, direction, 1.0..2.0, 4..16, 0)
}

/// `ivy_spawner` (0x14094cf90). Mode gate + count gate + pass dispatch per the
/// O analysis. `storage.next_id()` stands in for the native +0x120 count.
pub fn ivy_spawner(
    mut storage: ResMut<IvyStorage>,
    mode: Res<AppMode>,
    cursor: Res<CursorTerrainRaycast>,
    walls: Res<PublicWalls>,
    noise: Res<IvySpawnNoise>,
) {
    if mode.native_tag() != 1 {
        return;
    }
    if storage.next_id() >= MAX_IVY_COUNT {
        return;
    }
    if storage.next_id() < 3 {
        try_ivy_root_next_to_cursor(&mut storage, &cursor, &walls);
    } else {
        try_random_ivy_root(&mut storage, &walls, &noise);
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
    fn vertical_u_clamps_to_the_machine_constant() {
        let mut storage = IvyStorage::new();
        for _ in 0..32 {
            let value = vertical_u(&mut storage, 1.0, 0.0);
            assert!((VERTICAL_MIN..=VERTICAL_MAX).contains(&value), "height {value}");
        }
    }

    #[test]
    fn approx_u_from_pos_projects_to_the_curve_parameter() {
        let points = vec![[0.0, 0.0], [10.0, 0.0], [20.0, 0.0]];
        let points_u = vec![0.0, 0.5, 1.0];
        let curve = system_roof::surface::Curve2 {
            points,
            points_u,
            length: 20.0,
        };
        // A point just off the first segment projects to u≈0.25.
        let u = approx_u_from_pos(&curve, Vec2::new(5.0, 0.3));
        assert!((u - 0.25).abs() < 1e-3, "u = {u}");
    }
}
