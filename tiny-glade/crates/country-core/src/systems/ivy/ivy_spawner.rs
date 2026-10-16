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
//! - **ActionSet pressed gate**: `DigitalActionState::pressed` on the
//!   `ivy_spawn` action (SPEC G2.4; slot 37, binding 待取证).
//! - **point budget**: `total_point_count + 0x13 < 0x9c40` → return, i.e. the
//!   stored point total stays under 40,000; plus a per-frame `min(0x46,
//!   0x9c41 - total)` point cap.
//! - **pass dispatch**: `count = *(u32*)(IvyStorage + 0x120)` — below 3 the
//!   cursor pass (`try_ivy_root_next_to_cursor`) runs, at 3+ the random pass
//!   (`try_random_ivy_root`) runs.
//!
//! Cursor pass (path A, count < 3): cursor position → RNG-sampled spatial-hash
//! cell (world→cell scale 0.492307693 = 32/65, offset 32.0) → nearest segment
//! (`closest_segment_index`, max height 0.5 = 0x3f000000) → root offset ±0.34
//! (0.68 × 0.5) along the wall normal → reject when within 0.5 (0.25²) of an
//! existing root → vertical height `(rand*1.5 − 0.75)/wall_height +
//! segment_height` clamped to [0.0, 1.0] (VERTICAL_MIN = 0x142960690 = 0.0,
//! ① T3 定案；旧 0.000928482 为编造值，其字节 0x3a73655f 在整 EXE 0 次出现).
//! Up to 3 attempts.
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

use crate::input::action_set::MainActionSet;
use crate::resources::{
    cursor_terrain_raycast::CursorTerrainRaycast,
    ivy::{IvyId, IvySpawnNoise, IvyStorage},
    walls::public_walls::PublicWalls,
    walls::WallId,
};
use crate::systems::mode_manager::AppMode;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::{WallSegment, closest_segment_index, ivy_grower::wall_segments_from_state};

/// Verified max-height filter (0x3f000000 = 0.5) passed to
/// closest_segment_index in both the cursor pass and the pruner.
const MAX_IVY_GROW_HEIGHT: f32 = 0.5;
/// Total-point budget ceiling: native `total_point_count + 0x13 < 0x9c40`
/// (0x14094d09b..a7) — the spawner returns when the stored point count would
/// reach 40,000.
const TOTAL_POINT_CEILING: usize = 40_000;
/// Per-frame point cap: `eax = 0x9c41 - total; rsi = min(0x46, eax); rbp += rsi`
/// (0x14094db6e..b9e) — at most 0x46 (70) points are scheduled each frame,
/// bounded by the 40001 total ceiling.
const FRAME_POINT_CAP: usize = 0x46;
/// Spawned ivy point-count range start: native stack param 0x14 (20) at
/// [rsp+0x28] (SPEC G2.3). The upper bound is 待取证 (the second stack word
/// is a storage pointer, not a count); 40 stands in so maximum_point_count is
/// sampled from [20, 40).
const SPAWN_COUNT_RANGE: std::ops::Range<usize> = 20..40;
/// Initial growth cooldown scalar range per path: cursor → [0, 3.0], random →
/// [0, 1.0] (table 0x142ab3218 = 3.0, 1.0; SPEC G2.3).
const SCALAR_RANGE_CURSOR: std::ops::Range<f32> = 0.0..3.0;
const SCALAR_RANGE_RANDOM: std::ops::Range<f32> = 0.0..1.0;
/// Digital action slot for `ivy_spawn` (DIGITAL_NAMES[36]): the spawner's
/// ActionSet pressed gate (SPEC G2.4). The exact button binding is 待取证.
const IVY_SPAWN_ACTION_SLOT: usize = 36;
/// Cursor-pass root-offset half-width: 0.68 × 0.5 = 0.34 (constants
/// 0x142ab31e0 / 0x142ab3200 multiplied by 0.5 at 0x142925870/8e0/e4).
const ROOT_OFFSET: f32 = 0.34;
/// Random-pass root-offset magnitude: 0.68, selected directly by `get_bool`
/// (RE-ANALYSIS2: true → −0.68, false → +0.68; call 0x14094d6b0).
/// ① T3 定案：0x142ab31f0 = `7b142e3f` = 0.68f（capstone 直接引用）；0x142ab3210 =
/// `7b142ebf` = −0.68f。0x1429258d4 = 0.63f 是**不同常量**（非根偏移，用途待定）。
const RANDOM_ROOT_OFFSET: f32 = 0.68;
/// Cursor-pass minimum separation from existing roots: 0.25 (distance 0.5,
/// constant 0x142925950).
const MIN_ROOT_DISTANCE_SQ: f32 = 0.25;
/// Random-pass minimum separation from existing roots: 2.56 (distance 1.6,
/// constant 0x142ab3214).
const RANDOM_ROOT_DISTANCE_SQ: f32 = 2.56;
/// Vertical-height sample clamp lower bound (0x142960690 = f32 0.0, ① T3 定案；
/// 旧 0.000928482 为编造值，字节 0x3a73655f 整 EXE 0 次出现)。
const VERTICAL_MIN: f32 = 0.0;
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
/// with the owning wall id, the segment and the hit's interpolated value
/// (lerp of the segment height pair at the clamped hit `t`).
fn nearest_wall_normal(
    walls: &PublicWalls,
    point: Vec2,
) -> Option<(WallId, Vec2, WallSegment, f32)> {
    let mut best: Option<(f32, WallId, Vec2, WallSegment, f32)> = None;
    for (id, state) in walls.all() {
        let segments = wall_segments_from_state(state);
        let Ok(Some(hit)) = closest_segment_index(point, &segments, MAX_IVY_GROW_HEIGHT) else {
            continue;
        };
        let seg = &segments[hit.index];
        let dir = seg.p1 - seg.p0;
        let len = dir.length().max(1e-20);
        let normal = Vec2::new(-dir.y, dir.x) / len;
        match best {
            Some((d, _, _, _, _)) if d <= hit.distance => {}
            _ => best = Some((hit.distance, id, normal, *seg, hit.interpolated)),
        }
    }
    best.map(|(_, id, normal, seg, interpolated)| (id, normal, seg, interpolated))
}

/// `Curve::get_approx_u_from_pos` (FUN_141469a20, S_PUB32): the exact `u`
/// parameter of the closest point on the curve. Shared-utils cross-domain;
/// the projection below walks the polyline and interpolates `points_u`.
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

/// Vertical spawn `u`: `(rand*1.5 − 0.75)/divisor + addend`, clamped to
/// [0.0, 1.0] (VERTICAL_MIN = 0.0, ① T3). For the cursor pass the divisor is the
/// wall's max ivy grow height (待取证 for the getter; 1.0 stands in) and the
/// addend is the hit segment's interpolated height. For the random pass the
/// divisor is the wall curve length and the addend is `u_on_curve` (exact
/// projection).
fn vertical_u(storage: &mut IvyStorage, divisor: f32, addend: f32) -> f32 {
    let rand = storage.rng_mut().uniform_f32();
    let value = (rand * 1.5 - 0.75) / divisor.max(1e-20) + addend;
    value.clamp(VERTICAL_MIN, VERTICAL_MAX)
}

/// `try_ivy_root_next_to_cursor` (line 245, inline, path A): place a root next
/// to the cursor. Nearest wall segment under the cursor → root offset ±0.34 →
/// separation check → vertical height → `spawn_ivy`. Up to `SPAWN_ATTEMPTS`
/// tries. `budget` is the frame point cap, decremented by each spawn.
pub fn try_ivy_root_next_to_cursor(
    storage: &mut IvyStorage,
    cursor: &CursorTerrainRaycast,
    walls: &PublicWalls,
    budget: &mut usize,
) -> Option<IvyId> {
    let position = cursor.position();
    let point = Vec2::new(position.x, position.z);
    for _ in 0..SPAWN_ATTEMPTS {
        let Some((wall_id, normal, _, interpolated)) = nearest_wall_normal(walls, point) else {
            continue;
        };
        let root = Vec3::new(point.x, 0.0, point.y) + Vec3::new(normal.x, 0.0, normal.y) * ROOT_OFFSET;
        if root_too_close(storage, root, MIN_ROOT_DISTANCE_SQ) {
            continue;
        }
        let direction = Vec3::new(normal.x, 0.0, normal.y);
        let height = vertical_u(storage, 1.0, interpolated);
        return spawn_validated_root(
            storage,
            Vec3::new(root.x, height, root.z),
            direction,
            wall_id,
            SCALAR_RANGE_CURSOR,
            budget,
        );
    }
    None
}

/// `try_random_ivy_root` (line 212, inline, path B): pick a random wall, sample
/// a random `u ∈ [0,1)`, gate on the spawn FastNoise density, place a root at
/// the wall curve point offset laterally by the `get_bool` side (±0.34),
/// reject roots within 1.6 of an existing root (2.56²), set the vertical `u`,
/// and spawn. Attempts = 2..4 (re-analyst RE-ANALYSIS2). `budget` is the frame
/// point cap, decremented by each spawn.
pub fn try_random_ivy_root(
    storage: &mut IvyStorage,
    walls: &PublicWalls,
    noise: &IvySpawnNoise,
    budget: &mut usize,
) -> Option<IvyId> {
    let wall_rows: Vec<_> = walls.all().collect();
    if wall_rows.is_empty() {
        return None;
    }
    let attempts = 2 + storage.rng_mut().usize_less_than(3);
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
        let x = t * b[0] + (1.0 - t) * a[0];
        let z = t * b[1] + (1.0 - t) * a[1];
        let dx = b[0] - a[0];
        let dz = b[1] - a[1];
        let reciprocal = 1.0 / (dz * dz + dx * dx).sqrt().max(1e-20);
        // FastNoise density gate: the noise must be strictly greater than the
        // threshold at res+0x78 (0x14094cf90: `if noise < t || noise == t →
        // return`); threshold value 待取证.
        let density = noise.noise.get_noise3d(x, 0.0, z);
        if density <= noise.threshold {
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
        let Some(id) = spawn_validated_root(
            storage,
            Vec3::new(root.x, height, root.z),
            direction,
            wall_id,
            SCALAR_RANGE_RANDOM,
            budget,
        ) else {
            break; // frame point budget exhausted (0x14094db6e..b9e)
        };
        last = Some(id);
    }
    last
}

/// Shared spawn commit: root position/direction with the point budget sampled
/// from 20..40 (native stack param 0x14; upper bound 待取证) and the owning
/// wall id forwarded to the leaf storage (`source_60` = wall id, [rsp+0xf8]).
/// Returns `None` once the per-frame point budget is spent.
fn spawn_validated_root(
    storage: &mut IvyStorage,
    position: Vec3,
    direction: Vec3,
    wall_id: WallId,
    scalar_range: std::ops::Range<f32>,
    budget: &mut usize,
) -> Option<IvyId> {
    if *budget == 0 {
        return None;
    }
    let id = storage.spawn_ivy(position, direction, scalar_range, SPAWN_COUNT_RANGE, wall_id.0);
    // The ivy's sampled point ceiling counts against this frame's cap.
    *budget = budget.saturating_sub(
        storage
            .get(id)
            .map_or(0, |ivy| ivy.maximum_point_count),
    );
    Some(id)
}

/// `ivy_spawner` (0x14094cf90). Mode gate + ActionSet pressed gate + point
/// budget + pass dispatch per the O analysis.
pub fn ivy_spawner(
    mut storage: ResMut<IvyStorage>,
    mode: Res<AppMode>,
    cursor: Res<CursorTerrainRaycast>,
    walls: Res<PublicWalls>,
    noise: Res<IvySpawnNoise>,
    actions: Res<MainActionSet>,
) {
    if mode.native_tag() != 1 {
        return;
    }
    // ActionSet pressed gate (SPEC G2.4): roots are only attempted while the
    // `ivy_spawn` digital action (slot 37) is pressed. The exact button
    // binding is 待取证.
    if !actions.digital[IVY_SPAWN_ACTION_SLOT].pressed() {
        return;
    }
    // Total-point budget (0x14094d09b..a7): `total_point_count + 0x13 <
    // 0x9c40`, i.e. the stored point total stays under 40,000.
    if storage.total_point_count() + 0x13 >= TOTAL_POINT_CEILING {
        return;
    }
    // Per-frame point cap (0x14094db6e..b9e): min(0x46, 0x9c41 - total).
    let mut budget = FRAME_POINT_CAP.min((TOTAL_POINT_CEILING + 1).saturating_sub(storage.total_point_count()));
    if storage.next_id() < 3 {
        try_ivy_root_next_to_cursor(&mut storage, &cursor, &walls, &mut budget);
    } else {
        try_random_ivy_root(&mut storage, &walls, &noise, &mut budget);
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
