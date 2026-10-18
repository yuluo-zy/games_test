//! B `ivy_grower` 0x14094a910: the 10-phase state machine whose growth
//! closure (guard body 0x14094ac51) advances pending segments and proposes new
//! wall-anchored segments. Completed segments feed the leaf spawner through
//! `IvySegmentDoneGrowingMsg`.
//!
//! # Phase machine (jump table 0x142ab3be0, indexed by `world + 0x118`)
//!
//! Fresh disassembly (collision-eco/ivy-grower-disasm/, 2026-10-16):
//!
//! | phase | entry | semantics |
//! |---|---|---|
//! | 0 | 0x14094c09f | guard phase: toggle bit0 of the state's guard byte
//! |   |           | (+0x48), then run the growth closure when the bit is set
//! |   |           | and return (epilogue 0x14094c0c0) when clear |
//! | 1/3/4 | 0x14094ac20 | shared entry: guard bit set → closure, else return |
//! | 2 | 0x14094ac31 | timing entry: `ucomiss 0, [state+0x11c]`; timer ≤ 0 →
//! |   |           | 0x14094c0a6 (the guard-toggle path), else guard bit →
//! |   |           | closure / return |
//! | 5 | 0x14094f2f2 | secondary table 0x142ab3bf4 (5 entries) — all panic
//! |   |           | tail entries |
//! | 6-9 | 0x14094f308/31e/356/36c | panic tails (string + length → panic) |
//!
//! The state value written back through the +0x10 pointer targets the Bevy
//! `Local<GrowerState>` slot at `world+0x118`; this crate models the same
//! state as a `Local` resource. Because the phase value itself never advances,
//! phase 0 (the toggle) runs every frame and the closure alternates with the
//! no-op epilogue — growth advances every other frame, exactly the cadence of
//! the 0↔1 model used before (the closed-loop phase test covers both).
//!
//! # Growth closure (guard body 0x14094ac51)
//!
//! Per-ivy (hashbrown iteration over the storage map):
//! 1. Skip when `point_count >= maximum_point_count` (0x14094ae9b).
//! 2. Growth cooldown: while `ivy.growth_delay > 0.001` (0x1429cd060) the
//!    timer is decremented by `time.delta` and the ivy does not grow
//!    (0x14094aea9 / 0x14094adf5). The cooldown is seeded by `spawn_ivy`.
//! 3. Pending segment: advance it (`advance_growth`, 0x14094b1bb) when the
//!    ivy has more than one point; a single-point ivy clears the pending
//!    state (0x14094aedb).
//! 4. Otherwise, when the ivy has points, propose a new segment.
//!
//! The proposal (0x14094aef9..) resolves the tip's world position into the
//! wall spatial-hash cell with the CRT `floorf` import (0x142923ee0, twice),
//! runs `closest_segment_index` (0x14094c800) over the cell's segments at the
//! max ivy grow height 0.5, and accepts only a hit closer than 0.365
//! (0x142ab3190). The owning wall must have ≥ 2 points and a positive curve
//! length (0x14094b31d/0x14094b32c; `[wall-0x270]` is the **curve length**,
//! not a height). The proposed point is `WallCoord::into_world_space` (call
//! 0x14094b7b9) with the arc-distance coordinate from the wall curve length at
//! the tip's `u` (0x14094b594..b703); the direction proposer's output is
//! scaled by 0.21 (0x142ab3194). Height acceptance: `0 ≤ u_field ≤ length`
//! (0x14094b719/b72f) and `v ≤ arch_height + subtype band`
//! (0x14094b73d..b77a; band: +0.2 for the flag byte, −0.46 for subtype ≥ 2,
//! −0.79 + 0.2 otherwise). Wall jumps use
//! `intersect_ivy_growth_w_wall_segment` over `query_with_segment` and pick the
//! nearest re-anchor (0x14094b873..b9a4).

use crate::resources::{
    ivy::{Ivy, IvyDirectionProposer, IvyGrowthSegment, IvyPointId, IvyStorage, WallCoord},
    time::GameTime,
    walls::{
        public_walls::PublicWalls, public_wall_state::PublicWallState,
        wall_holes::PrevWallHoles, WallId,
    },
};
use crate::systems::wall::wall_state::wall_spatial_hash::{WallSpatialHash, WallSpatialSegment};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::{ivy_leaf_spawner::IvySegmentDoneGrowingMsg, WallSegment};

/// Growth divisor: one segment takes 0.075 s at unit delta (0x142a9abd4).
pub const SEGMENT_DURATION: f32 = 0.075;
/// Segment completes at progress 0.99 (0x142a948d8).
pub const GROWTH_COMPLETE: f32 = 0.99;
/// Progress clamp maximum (0x142925904).
pub const PROGRESS_MAX: f32 = 1.0;
/// Growth-cooldown settled bound (0x1429cd060): while the per-ivy cooldown
/// exceeds this value the grower decrements it and skips the ivy.
pub const SETTLED_THRESHOLD: f32 = 0.001;
/// `closest_segment_index` max-height filter (0x3f000000 = 0.5).
pub const MAX_IVY_GROW_HEIGHT: f32 = 0.5;
/// New-segment acceptance radius (0x142ab3190): the tip must be within this
/// distance of the sampled wall segment.
pub const GROWTH_ACCEPT_DISTANCE: f32 = 0.365;
/// Direction-proposer output scale (0x142ab3194).
pub const DIRECTION_SCALE: f32 = 0.21;
/// Height-band offsets applied to the wall arch height (0x14094b73d..b77a):
/// flag byte set → +0.2 (0x1429cd068); subtype ≥ 2 → −0.46 (0x142a7f250);
/// otherwise → −0.79 (0x142a7e320) + 0.2.
pub const HEIGHT_BAND_FLAG: f32 = 0.2;
pub const HEIGHT_BAND_HIGH: f32 = -0.46;
pub const HEIGHT_BAND_LOW_A: f32 = -0.79;
/// Wall-hole AABB expansion (0x14094b840, constant 0x142ab31a0 =
/// (−0.1, −1.0, 0.1, 1.0)) applied before the tip-containment test.
pub const HOLE_BOX_EXPAND: [f32; 4] = [-0.1, -1.0, 0.1, 1.0];
/// Maximum tip-to-landing distance (0x142ab31b0 = 0.42): a proposed landing
/// farther than this from the growth tip is rejected
/// (0x14094bd38/0x14094bf26).
pub const LANDING_MAX_DISTANCE: f32 = 0.42;

/// Advances one ivy's pending segment by `delta` seconds and emits the
/// segment-completion event when it finishes. Point IDs come from the deque
/// tail order (`Ivy::last_segment_point_ids`); a missing pending segment is a
/// no-op exactly like the native empty-option path. `emit` is the completion
/// callback; the system forwards to its `EventWriter`, tests capture into a
/// local buffer.
pub fn advance_growth(
    ivy: &mut Ivy,
    delta: f32,
    emit: &mut impl FnMut(IvySegmentDoneGrowingMsg),
) {
    let Some(segment) = ivy.pending_growth else {
        return;
    };
    let progress = (ivy.growth_progress + delta / SEGMENT_DURATION).clamp(0.0, PROGRESS_MAX);
    ivy.growth_progress = progress;
    let position = segment.from * (1.0 - progress) + segment.to * progress;
    ivy.set_last_point_position(position);
    if progress >= GROWTH_COMPLETE {
        ivy.pending_growth = None;
        ivy.growth_progress = 0.0;
        let (point_a, point_b) = ivy.last_segment_point_ids();
        emit(IvySegmentDoneGrowingMsg {
            point_a,
            point_b,
            ivy: ivy.id,
        });
    }
}

/// Appends a point at `position` and arms the next pending segment. The new
/// point is placed at the current tail and the pending segment animates it
/// toward `next`, mirroring B `Ivy::add_point` (0x140869460 via 0x14094bf76)
/// followed by the pending store at 0x14094bf85.
pub fn start_growth_segment(
    ivy: &mut Ivy,
    position: Vec3,
    direction: Vec3,
    next: Vec3,
) -> IvyPointId {
    let id = ivy.add_point(position, direction);
    ivy.pending_growth = Some(IvyGrowthSegment {
        from: position,
        to: next,
    });
    ivy.growth_progress = 0.0;
    id
}

/// Double-sided wall normal (B 0x14094caa0). Given a curve and a world point,
/// returns the unit perpendicular to the nearest segment, oriented to face the
/// point (`normal` when the point is on the +normal side, `-normal` otherwise).
pub fn get_wall_double_sided_normal(curve: &crate::resources::roofs::geometry::surface::Curve2, point: Vec2) -> Vec2 {
    let index = curve.coord_at_u((point.x / curve.length.max(1e-20)).clamp(0.0, 1.0)).0;
    let index = index.min(curve.points.len().saturating_sub(2));
    let a = Vec2::from(curve.points[index]);
    let b = Vec2::from(curve.points[index + 1]);
    let tangent = (b - a).normalize_or_zero();
    let normal = Vec2::new(-tangent.y, tangent.x);
    if normal.dot(point - a) >= 0.0 {
        normal
    } else {
        -normal
    }
}

/// Builds the 56-byte `WallSegment` rows `closest_segment_index` reads from a
/// spatial-hash cell's native 48-byte records. The native segment layout is
/// `{p0.xy@0, p1.xy@8, u pair@+0x10, h0@+0x18, h1@+0x1c, wall@+0x28}`; the
/// crate's `WallSpatialSegment` mirrors it with `endpoints`, `source_u`,
/// `relative_heights` and `wall` in the same offsets.
pub fn wall_segment_from_spatial(segment: &WallSpatialSegment) -> WallSegment {
    WallSegment {
        p0: Vec2::new(segment.endpoints[0][0], segment.endpoints[0][1]),
        p1: Vec2::new(segment.endpoints[1][0], segment.endpoints[1][1]),
        extra_a: segment.source_u[0],
        extra_b: segment.source_u[1],
        h0: segment.relative_heights[0],
        h1: segment.relative_heights[1],
    }
}

/// The wall's curve length (`[wall - 0x270]` in 0x14094b32c / 0x14094b594 /
/// 0x140998dc0). The map bucket's value base is `PublicWallState`, whose
/// `simplified_curve_2d().length` sits at `+0x30` — exactly the field
/// `WallCoord::into_world_space` reads as the normalising denominator
/// (0x140b199e0 `[in_R8+0x30]`). The earlier `calculated_max_y()` reading was
/// wrong: the grower compares the arc-distance coordinate against this length,
/// and the spawner divides its vertical sample by it.
pub fn wall_length(state: &PublicWallState) -> f32 {
    state.simplified_curve_2d().length
}

/// The wall's arch height used for the subtype height band. Native
/// `[wall - 0x1c]` at 0x14094b73d..b76f (待取证 for the exact native field).
pub fn wall_arch_height(state: &PublicWallState) -> f32 {
    state.authority.arch_height()
}

/// Acceptance height for the proposed point: the arch height plus a subtype
/// band (0x14094b73d..b77a). The `flag_byte`/`subtype` come from the wall
/// authority (待取证 for the exact native offsets `[wall-0xa]`/`[wall-0x48]`).
pub fn wall_height_band(state: &PublicWallState, flag_byte: bool, subtype: u32) -> f32 {
    if flag_byte {
        wall_arch_height(state) + HEIGHT_BAND_FLAG
    } else if subtype < 2 {
        wall_arch_height(state) + HEIGHT_BAND_LOW_A + HEIGHT_BAND_FLAG
    } else {
        wall_arch_height(state) + HEIGHT_BAND_HIGH
    }
}

/// `get_ivy_x_direction_relative_to_wall_direction` (line 491, inline): the
/// ivy's sideways (x) growth axis is the wall-direction tangent rotated 90°,
/// i.e. the wall-face normal. This is the axis `ivy_leaf_spawner` uses as
/// `branch_direction × normal` when scattering leaves across the wall face.
pub fn get_ivy_x_direction_relative_to_wall_direction(wall_direction: Vec2) -> Vec2 {
    Vec2::new(-wall_direction.y, wall_direction.x)
}

/// The point a growth proposal lands on plus the exact `WallCoord` fields the
/// native checks before converting: `u_field` is an arc distance along the
/// curve, `v` is the world-space height (0x14094b719..b77a).
#[derive(Clone, Copy, Debug)]
pub struct GrowthProposal {
    pub world: Vec3,
    pub normal: Vec3,
    pub u_field: f32,
    pub v: f32,
}

/// `Curve::get_pos_at_u` (utils 0x141467270 / 0x141467ce0): interpolated 2D
/// curve point at the normalised `u` (clamped to `[0,1]`).
pub fn curve_pos_at_u(
    curve: &crate::resources::roofs::geometry::surface::Curve2,
    u: f32,
) -> Vec2 {
    let u = u.clamp(0.0, 1.0);
    let (index, t) = curve.coord_at_u(u);
    let a = Vec2::from(curve.points[index]);
    let b = Vec2::from(curve.points[index + 1]);
    a + (b - a) * t
}

/// `Curve::get_tangent_at_u` (utils 0x141468f90): unit tangent of the segment
/// under `u`.
pub fn curve_tangent_at_u(
    curve: &crate::resources::roofs::geometry::surface::Curve2,
    u: f32,
) -> Vec2 {
    let u = u.clamp(0.0, 1.0);
    let (index, _) = curve.coord_at_u(u);
    let a = Vec2::from(curve.points[index]);
    let b = Vec2::from(curve.points[index + 1]);
    (b - a).normalize_or_zero()
}

/// `Curve::is_on_right_side` (utils 0x141469000): the point is on the right
/// side of the nearest polyline segment.
pub fn curve_is_on_right_side(
    curve: &crate::resources::roofs::geometry::surface::Curve2,
    point: Vec2,
) -> bool {
    let u = approx_u_from_pos(curve, point);
    let (index, _) = curve.coord_at_u(u);
    let a = Vec2::from(curve.points[index]);
    let b = Vec2::from(curve.points[index + 1]);
    let from_start = (point - a).normalize_or_zero();
    let reverse_segment = (a - b).normalize_or_zero();
    reverse_segment.y * from_start.x < reverse_segment.x * from_start.y
}

/// Proposes the next world-space growth point (guard body 0x14094aef9..0x14094b7b9).
///
/// Field-by-field from the fresh disassembly:
/// - `u = clamp(get_approx_u_from_pos(tail.xz))` (0x14094b536/b53b..b5b5).
/// - lateral sign: the tail's offset from the curve point at `u` is projected
///   on the tangent; `tangent.y*dx > tangent.x*dz` selects `-direction.x*0.21`
///   (0x14094b6d9..b6ff), otherwise `+direction.x*0.21`.
/// - `WallCoord.u = length*u + lateral` (0x14094b5a5 mulss / 0x14094b703 addss),
///   `WallCoord.v = tail.y + direction.y*0.21` (0x14094b661/665).
/// - `into_world_space` side argument is `Curve::is_on_right_side(tail)`
///   (0x14094b7a0 call 0x141469000, result in r9d).
/// - the pending point direction is the normalised double-sided wall normal at
///   the landing point (0x14094be99 call 0x14094caa0, normalised at
///   0x14094bee7..bef2).
pub fn get_growth_point_proposal(
    wall: &PublicWallState,
    tail: Vec3,
    direction: Vec2,
) -> GrowthProposal {
    let curve = wall.simplified_curve_2d();
    let tail_xz = Vec2::new(tail.x, tail.z);
    let u = approx_u_from_pos(curve, tail_xz).clamp(0.0, 1.0);
    let interp = curve_pos_at_u(curve, u);
    let tangent = curve_tangent_at_u(curve, u);
    let cross = (tail_xz.x - interp.x) * tangent.y - (tail_xz.y - interp.y) * tangent.x;
    let lateral_sign = if cross > 0.0 { -1.0 } else { 1.0 };
    let lateral = direction.x * DIRECTION_SCALE * lateral_sign;
    let u_field = wall_length(wall) * u + lateral;
    let v = tail.y + direction.y * DIRECTION_SCALE;
    let side = curve_is_on_right_side(curve, tail_xz);
    let world = WallCoord { u: u_field, v }.into_world_space(wall, side);
    let normal2 = get_wall_double_sided_normal(curve, Vec2::new(world.x, world.z))
        .normalize_or_zero();
    GrowthProposal {
        world,
        normal: Vec3::new(normal2.x, 0.0, normal2.y),
        u_field,
        v,
    }
}

/// Native hole containment (`PrevWallHoles::wall_holes` 0x140f86d00 + the box
/// test 0x141240140, grower 0x14094b7be..b86e): when a previous-frame hole of
/// kind 0 holds the growth tip inside its expanded AABB, the ivy pauses
/// (`flag_8c = 1`) and the grower skips the point that frame
/// (`mov byte [r14-4],1; jmp main-loop`). The probe is the tip's u distance
/// along the wall and its world y, the coordinate space the hole producers
/// write (`aabb = [u - w/2, y - h/2, u + w/2, y + h/2]`, RawWallHole
/// center/dimensions in wall-platform-stairs). The exact native probe
/// transform (`u * [wall-0x14]` then 0x14003bb80) is 待取证.
fn hole_blocks_growth(prev_holes: &PrevWallHoles, wall_id: WallId, probe: Vec2) -> bool {
    let Some(previous) = prev_holes.wall_holes(wall_id) else {
        return false;
    };
    for hole in &previous.holes {
        // The asm skips entries whose +0x10 byte is nonzero
        // (`cmp byte ptr [rsi+0x10], 0; jne next`); holes of kind 0 block growth.
        if hole.kind != 0 {
            continue;
        }
        let [min_x, min_y, max_x, max_y] = hole.aabb;
        let min_x = min_x + HOLE_BOX_EXPAND[0];
        let min_y = min_y + HOLE_BOX_EXPAND[1];
        let max_x = max_x + HOLE_BOX_EXPAND[2];
        let max_y = max_y + HOLE_BOX_EXPAND[3];
        if probe.x >= min_x && probe.x <= max_x && probe.y >= min_y && probe.y <= max_y {
            return true;
        }
    }
    false
}

/// `intersect_ivy_growth_w_wall_segment` (0x14094cc60). Called from
/// `check_for_wall_jump`'s filter-map closure (0x141308501 / 0x1413085fa), so
/// it is **not** dead code.
///
/// Arguments: a wall segment (`p0,p1`), the growth tip `query`, and the first
/// proposed landing `other`. The native:
/// 1. builds the `(normal, tangent)` frame of the segment around its midpoint;
/// 2. maps both points into that frame and intersects the segment `query→other`
///    with the box `[-0.315,0.315] x [-0.5·len,0.5·len]` via
///    `utils::geometry::aabb::intersection_aabb_segment2` (0x141240b30);
/// 3. projects the hit back onto the wall segment (`t = dot(mid + offset - p0,
///    dir)/len²`, clamped) and returns the nearer of `seg_point ± 0.315·unit
///    tangent` to `query` (0x14094ced3..cf22).
pub fn intersect_ivy_growth_w_wall_segment(
    segment: &WallSegment,
    query: Vec2,
    other: Vec2,
) -> Option<Vec2> {
    let p0 = segment.p0;
    let p1 = segment.p1;
    let dir = p1 - p0;
    let len_sq = dir.length_squared();
    if len_sq < 1e-20 {
        return None;
    }
    let len = len_sq.sqrt();
    let unit = dir / len;
    let normal = Vec2::new(-unit.y, unit.x);
    let mid = (p0 + p1) * 0.5;
    let qn = normal.dot(query - mid);
    let qu = unit.dot(query - mid);
    let sn = normal.dot(other - mid);
    let su = unit.dot(other - mid);
    let half = 0.5 * len;
    let hit = utils::geometry::aabb::intersection_aabb_segment2(
        Vec2::new(-ROOT_BOX_HALF, -half),
        Vec2::new(ROOT_BOX_HALF, half),
        Vec2::new(qn, qu),
        Vec2::new(sn, su),
    )?;
    let world_offset = normal * hit.x + unit * hit.y;
    let t = ((mid + world_offset - p0).dot(dir) / len_sq).clamp(0.0, 1.0);
    let seg_point = p0 + dir * t;
    let offset = unit * ROOT_BOX_HALF;
    let a = seg_point - offset;
    let b = seg_point + offset;
    let da = (a - query).length_squared();
    let db = (b - query).length_squared();
    Some(if db > da { a } else { b })
}

/// Half extent used by the native intersection box: `0.63 * 0.5` (constants
/// 0x142aa4550 and 0x1429258e0).
const ROOT_BOX_HALF: f32 = 0.63 * 0.5;

/// Re-anchors growth onto whichever wall owns the segment nearest to the
/// growth path, resolved through the wall spatial hash.
///
/// Fresh control flow (0x14094b873..0x14094b95c):
/// `WallSpatialHash::query_with_segment` (0x140aa5850) over the box spanned by
/// the tail and the first proposed point → `Vec::retain` drops segments on the
/// current wall (0x1412f6b60 compares `WallSegment+0x28`) → the filter-map
/// closure (0x141308470) runs `intersect_ivy_growth_w_wall_segment` on every
/// segment, keeps the hits and records `(distance-to-tail, hit, wall)` → the
/// vector is sorted by distance and the nearest candidate wins
/// (0x14094b961..0x14094b9a4).
pub fn check_for_wall_jump(
    walls: &PublicWalls,
    spatial: &WallSpatialHash,
    tail: Vec3,
    proposed: Vec3,
    current_wall: Option<WallId>,
) -> Option<WallId> {
    let _ = walls;
    let tail_xz = Vec2::new(tail.x, tail.z);
    let proposed_xz = Vec2::new(proposed.x, proposed.z);
    let mut best: Option<(f32, WallId)> = None;
    for segment in spatial.query_with_segment(
        [tail_xz.x, tail_xz.y],
        [proposed_xz.x, proposed_xz.y],
        0.0,
    ) {
        if current_wall == Some(segment.wall) {
            continue;
        }
        let row = wall_segment_from_spatial(segment);
        let Some(hit) = intersect_ivy_growth_w_wall_segment(&row, tail_xz, proposed_xz) else {
            continue;
        };
        let distance = (hit - tail_xz).length();
        if best.map_or(true, |(best_distance, _)| distance < best_distance) {
            best = Some((distance, segment.wall));
        }
    }
    best.map(|(_, wall)| wall)
}

/// Builds `WallSegment` rows from a published wall's simplified curve. Heights
/// come from the published segment heights when present, otherwise zero.
pub fn wall_segments_from_state(
    state: &crate::resources::walls::public_wall_state::PublicWallState,
) -> Vec<WallSegment> {
    let curve = state.simplified_curve_2d();
    let heights = &state.segment_heights;
    curve
        .points
        .windows(2)
        .enumerate()
        .map(|(i, pair)| WallSegment {
            p0: Vec2::from(pair[0]),
            p1: Vec2::from(pair[1]),
            extra_a: 0.0,
            extra_b: 0.0,
            h0: heights.get(i).copied().unwrap_or(0.0),
            h1: heights.get(i + 1).copied().unwrap_or(0.0),
        })
        .collect()
}

/// `Curve::get_approx_u_from_pos` (utils 0x141469a20): the exact `u`
/// parameter of the closest point on the curve. Shared-utils cross-domain;
/// the projection below walks the polyline and interpolates `points_u`.
pub fn approx_u_from_pos(curve: &crate::resources::roofs::geometry::surface::Curve2, pos: Vec2) -> f32 {
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

/// `Local` continuation state mirroring the original `world+0x118` phase slot.
/// The guard byte bit0 lives in `IvyStorage::flag_48` (+0x48 of the state the
/// original `[r14]` points at); only the phase and timing slot stay local.
/// Public so the system can be registered on a schedule from sibling modules.
#[derive(Clone, Debug, Default)]
pub struct GrowerState {
    /// +0x1c: the current phase value.
    phase: u8,
    /// +0x11c: timing gate. The original never modifies it from this system;
    /// it is compared (`ucomiss 0, timer`) and externally written. The value
    /// is kept for phase-2 semantics and documented as externally owned.
    timer: f32,
    /// ① T3 ivy §3: 墙洞命中产生的受影响叶/根 id 列表（native ptr@rbp+0x50/len@
    /// rbp+0x58，洞约束 0x14094b7be..b86e 填充）。本库以 GrowerState 跨帧持留
    /// （native 局部 drop 语义待取证）；+0x60 记录字段语义待取证，以 ivy id 建模标注。
    affected_ids: Vec<u64>,
}

/// Runs the growth closure for every stored ivy (guard body 0x14094ac51).
/// GPU/upload consumption is out of scope here.
#[allow(clippy::too_many_arguments)]
fn run_growth_pass(
    storage: &mut IvyStorage,
    delta: f32,
    walls: &PublicWalls,
    spatial: &WallSpatialHash,
    proposer: &IvyDirectionProposer,
    prev_holes: &PrevWallHoles,
    affected_ids: &mut Vec<u64>,
    completed: &mut EventWriter<IvySegmentDoneGrowingMsg>,
) {
    // ① T3 ivy §3: 取上轮的受影响列表用于 flag 消费谓词（0x14094ae63..96），本轮
    // 在洞命中时重建（propose_new_segment 填充）。+0x60 字段语义待取证，以 ivy id 建模。
    let prev_affected = std::mem::take(affected_ids);
    for (_, ivy) in storage.iter_mut() {
        // Consume the wall-hole flag (0x14094ae63..96): clear only when this
        // ivy's id is in the affected-leaves list (flag 置位 且 id ∈ 列表 → 清零).
        if ivy.flag_8c != 0 && prev_affected.contains(&ivy.id) {
            ivy.flag_8c = 0;
        }
        // 1. maximum-point-count gate (0x14094ae9b..).
        if ivy.point_count() >= ivy.maximum_point_count {
            continue;
        }
        // 2. Growth cooldown (0x14094aea9 / 0x14094adf5).
        if ivy.growth_delay > SETTLED_THRESHOLD {
            ivy.growth_delay -= delta;
            continue;
        }
        // 3. Pending segment (0x14094aeca..).
        if ivy.pending_growth.is_some() {
            if ivy.point_count() > 1 {
                advance_growth(ivy, delta, &mut |message| {
                    completed.write(message);
                });
            } else {
                ivy.pending_growth = None;
                ivy.growth_progress = 0.0;
            }
            continue;
        }
        // 4. New-segment proposal needs at least one point.
        if ivy.point_count() == 0 {
            continue;
        }
        propose_new_segment(
            ivy,
            walls,
            spatial,
            proposer,
            prev_holes,
            affected_ids,
            &mut |message| {
                completed.write(message);
            },
        );
    }
}

/// New-segment proposal (guard body 0x14094aef9..). See the module docs for
/// the machine-code-verified cell sampling, acceptance and height gates.
#[allow(clippy::too_many_arguments)]
fn propose_new_segment(
    ivy: &mut Ivy,
    walls: &PublicWalls,
    spatial: &WallSpatialHash,
    proposer: &IvyDirectionProposer,
    prev_holes: &PrevWallHoles,
    affected_ids: &mut Vec<u64>,
    _emit: &mut impl FnMut(IvySegmentDoneGrowingMsg),
) {
    let tail = ivy.last_point().position;
    let point = Vec2::new(tail.x, tail.z);
    // Spatial-hash cell of the tip (floorf ×2; 0x14094af3c..0x14094af87).
    let cell = crate::resources::walls::spatial_index::cell_index_of_world_point([
        point.x,
        point.y,
    ]);
    let rows: Vec<(WallSegment, WallId)> = spatial
        .segments_in_cell(cell)
        .map(|s| (wall_segment_from_spatial(s), s.wall))
        .collect();
    if rows.is_empty() {
        return;
    }
    let segments: Vec<WallSegment> = rows.iter().map(|(s, _)| *s).collect();
    let Ok(Some(hit)) = super::closest_segment_index(point, &segments, MAX_IVY_GROW_HEIGHT) else {
        return;
    };
    if hit.distance >= GROWTH_ACCEPT_DISTANCE {
        return;
    }
    let (_, wall_id) = rows[hit.index];
    let Some(state) = walls.get(wall_id) else {
        return;
    };
    let curve = state.simplified_curve_2d();
    // Wall validity (0x14094b31d / 0x14094b32c): point count >= 2 and curve
    // length > 0. The original **panics** here (0x14094c1e3) rather than
    // skipping — a proposal on an invalid wall is an invariant violation.
    if curve.points.len() < 2 || wall_length(state) <= 0.0 {
        panic!("ivy_grower: growth proposed on an invalid wall");
    }
    // Direction from the proposer (0x14094b4cc); the seed is the ivy id at
    // record +0x20 (`mov r8, qword ptr [r14 - 0x70]`, 0x14094b4bd).
    let direction = proposer.get_direction(tail, ivy.id);
    let proposal = get_growth_point_proposal(state, tail, direction);
    // Height acceptance (0x14094b719..b77a): the WallCoord.u arc distance must
    // stay in [0, curve length], and v (world height) must be at or below the
    // arch height plus the subtype band. Note the native compares the **u
    // field**, not the world Y, against the curve length.
    if proposal.u_field < 0.0 || proposal.u_field > wall_length(state) {
        return;
    }
    let band = wall_height_band(
        state,
        state.authority.source_flag_65() != 0,
        state.authority.arch_type() as u32,
    );
    if proposal.v > band {
        return;
    }
    // Wall-hole gate (0x14094b7be..b86e): a previous-frame hole whose
    // expanded AABB holds the growth tip pauses the ivy and flags it; the
    // next main-loop pass consumes the flag (0x14094ae63..96).
    let hole_probe = Vec2::new(approx_u_from_pos(curve, point) * curve.length, tail.y);
    if hole_blocks_growth(prev_holes, wall_id, hole_probe) {
        ivy.flag_8c = 1;
        // ① T3 ivy §3: 洞命中 → 受影响叶/根 id 列表 push（native 洞约束段填充；
        // 记录 +0x60 字段语义待取证，以 ivy id 建模标注）。
        affected_ids.push(ivy.id);
        return;
    }
    // Wall-jump re-anchor (0x14094b7c6..0x14094b95c): query the spatial hash
    // over the box between the tail and the first proposal, intersect every
    // other-wall segment, and re-anchor onto the nearest candidate.
    if let Some(jump) = check_for_wall_jump(walls, spatial, tail, proposal.world, Some(wall_id)) {
        if jump != wall_id {
            if let Some(state_jump) = walls.get(jump) {
                let curve_jump = state_jump.simplified_curve_2d();
                // Wall-jump validity (0x14094b410 / 0x14094b43a): the two
                // re-anchor candidates also **panic** (0x14094c200 / 0x14094c21d)
                // when invalid.
                if curve_jump.points.len() < 2 || wall_length(state_jump) <= 0.0 {
                    panic!("ivy_grower: growth re-anchored on an invalid wall");
                }
                let jump_proposal = get_growth_point_proposal(state_jump, tail, direction);
                // Landing gate (0x14094bd38..40): the re-anchored landing must
                // stay within 0.42 of the tip.
                if (jump_proposal.world - tail).length() > LANDING_MAX_DISTANCE {
                    return;
                }
                start_growth_segment(ivy, tail, jump_proposal.normal, jump_proposal.world);
                return;
            }
        }
    }
    // Landing gate (0x14094bf26..2e): reject landings farther than 0.42.
    if (proposal.world - tail).length() > LANDING_MAX_DISTANCE {
        return;
    }
    start_growth_segment(ivy, tail, proposal.normal, proposal.world);
}

/// `ivy_grower` (0x14094a910). Dispatch on the persisted phase; the guard bit
/// of `IvyStorage::flag_48` gates whether the growth closure runs. Phases 5-9
/// are the original panic tails (secondary table 0x142ab3bf4 / panic strings).
pub fn ivy_grower(
    mut storage: ResMut<IvyStorage>,
    time: Res<GameTime>,
    proposer: Res<IvyDirectionProposer>,
    walls: Res<PublicWalls>,
    spatial: Res<WallSpatialHash>,
    prev_holes: Res<PrevWallHoles>,
    mut completed: EventWriter<IvySegmentDoneGrowingMsg>,
    mut state: Local<GrowerState>,
) {
    match state.phase {
        // Guard phase (0x14094c09f): toggle bit0 of the state's +0x48 byte,
        // then run the closure when the bit is set. The bit is not cleared
        // here — the next toggle flips it — so a growth pass alternates with a
        // no-op frame, matching the machine code where the epilogue
        // (0x14094c0c0) returns.
        0 => {
            storage.flag_48 ^= 1;
            if storage.flag_48 & 1 != 0 {
                run_growth_pass(
                    &mut storage,
                    time.delta,
                    &walls,
                    &spatial,
                    &proposer,
                    &prev_holes,
                    &mut state.affected_ids,
                    &mut completed,
                );
            }
        }
        // Shared main entries (0x14094ac20): run the closure when the guard
        // bit is set, otherwise return.
        1 | 3 | 4 => {
            if storage.flag_48 & 1 != 0 {
                run_growth_pass(
                    &mut storage,
                    time.delta,
                    &walls,
                    &spatial,
                    &proposer,
                    &prev_holes,
                    &mut state.affected_ids,
                    &mut completed,
                );
            }
        }
        // Timing entry (0x14094ac31): `ucomiss 0, timer`. While the timer is
        // <= 0 the guard-toggle path runs; otherwise the closure runs only on
        // guard-set frames.
        2 => {
            if state.timer <= 0.0 {
                storage.flag_48 ^= 1;
                if storage.flag_48 & 1 != 0 {
                    run_growth_pass(
                        &mut storage,
                        time.delta,
                        &walls,
                        &spatial,
                        &proposer,
                        &prev_holes,
                        &mut state.affected_ids,
                        &mut completed,
                    );
                }
            } else if storage.flag_48 & 1 != 0 {
                run_growth_pass(
                    &mut storage,
                    time.delta,
                    &walls,
                    &spatial,
                    &proposer,
                    &prev_holes,
                    &mut state.affected_ids,
                    &mut completed,
                );
            }
        }
        // Secondary table (0x14094f2f2) dispatches to five panic tails; the
        // original strings are documented in the disassembly.
        5 => {
            panic!("ivy_grower: secondary table phase 5 is a panic entry");
        }
        // Original panic tails (0x14094f308/31e/356/36c).
        6..=9 => {
            panic!("ivy_grower: invalid phase {}", state.phase);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::ivy::IvyStorage;
    use glam::Vec3;

    #[test]
    fn intersect_returns_the_shifted_box_corner_on_a_crossing_segment() {
        // Horizontal wall segment p0=(0,0) p1=(10,0); the growth segment runs
        // vertically through it at x=4.9. The box is ±0.315 along the unit
        // tangent around the projected hit. Because the query's tangent
        // coordinate is preserved by the projection, both corners are
        // equidistant, and the native `d_b > d_a` test selects the +tangent
        // corner: 4.9 + 0.315 = 5.215.
        let segment = WallSegment {
            p0: Vec2::new(0.0, 0.0),
            p1: Vec2::new(10.0, 0.0),
            extra_a: 0.0,
            extra_b: 0.0,
            h0: 0.0,
            h1: 0.0,
        };
        let hit = intersect_ivy_growth_w_wall_segment(
            &segment,
            Vec2::new(4.9, 1.0),
            Vec2::new(4.9, -1.0),
        )
        .expect("growth segment crosses the wall box");
        assert!((hit.x - 5.215).abs() < 1e-3, "hit = {hit:?}");
        assert!(hit.y.abs() < 1e-3, "hit = {hit:?}");
        // A growth segment that misses the box entirely is rejected.
        assert!(intersect_ivy_growth_w_wall_segment(
            &segment,
            Vec2::new(4.9, 5.0),
            Vec2::new(4.9, 4.0),
        )
        .is_none());
    }

    #[test]
    fn curve_frame_helpers_follow_the_polyline() {
        let curve = crate::resources::roofs::geometry::surface::Curve2 {
            points: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]],
            points_u: vec![0.0, 0.5, 1.0],
            length: 20.0,
        };
        assert!(curve_pos_at_u(&curve, 0.25).abs_diff_eq(Vec2::new(5.0, 0.0), 1e-4));
        assert!(curve_tangent_at_u(&curve, 0.25).abs_diff_eq(Vec2::X, 1e-4));
        assert!(curve_tangent_at_u(&curve, 0.75).abs_diff_eq(Vec2::Y, 1e-4));
        // Right side of the first segment (downward normal is -Y).
        assert!(curve_is_on_right_side(&curve, Vec2::new(5.0, -1.0)));
        assert!(!curve_is_on_right_side(&curve, Vec2::new(5.0, 1.0)));
    }

    #[test]
    fn growth_completes_and_emits_one_event_per_segment() {
        let mut storage = IvyStorage::new();
        let id = storage.spawn_ivy(Vec3::ZERO, Vec3::X, 1.0..2.0, 3..4, 7);
        {
            let ivy = storage.get_mut(id).unwrap();
            let _ = ivy.add_point(Vec3::X, Vec3::X);
            ivy.pending_growth = Some(IvyGrowthSegment {
                from: Vec3::X,
                to: Vec3::X * 2.0,
            });
        }
        let mut emitted = Vec::new();
        {
            let mut ivy = storage.get_mut(id).unwrap();
            advance_growth(&mut ivy, 0.075, &mut |message| emitted.push(message));
            assert_eq!(ivy.pending_growth, None);
            assert_eq!(ivy.growth_progress, 0.0);
            assert!(ivy.last_point().position.abs_diff_eq(Vec3::X * 2.0, 1e-6));
            // A second tick must not emit again.
            advance_growth(&mut ivy, 0.075, &mut |message| emitted.push(message));
        }
        assert_eq!(emitted.len(), 1);
        let (point_a, point_b) = emitted
            .into_iter()
            .next()
            .map(|e| (e.point_a, e.point_b))
            .unwrap();
        assert_eq!((point_a, point_b), (0, 2));
    }

    #[test]
    fn growth_interpolates_tail_at_partial_progress() {
        let mut storage = IvyStorage::new();
        let id = storage.spawn_ivy(Vec3::ZERO, Vec3::X, 1.0..2.0, 3..4, 7);
        let mut ivy = storage.get_mut(id).unwrap();
        let _ = ivy.add_point(Vec3::X, Vec3::X);
        ivy.pending_growth = Some(IvyGrowthSegment {
            from: Vec3::X,
            to: Vec3::X * 2.0,
        });
        let mut emitted = Vec::new();
        // 0.0375 s is half of 0.075: tail sits midway at (1.5, 0, 0).
        advance_growth(&mut ivy, 0.0375, &mut |message| emitted.push(message));
        assert!(ivy.last_point().position.abs_diff_eq(Vec3::new(1.5, 0.0, 0.0), 1e-6));
        assert!(ivy.pending_growth.is_some());
        assert!(emitted.is_empty());
    }

    #[test]
    fn start_growth_appends_point_and_arms_pending_segment() {
        let mut storage = IvyStorage::new();
        let id = storage.spawn_ivy(Vec3::ZERO, Vec3::X, 1.0..2.0, 3..4, 7);
        let mut ivy = storage.get_mut(id).unwrap();
        let point_id = start_growth_segment(&mut ivy, Vec3::X, Vec3::Y, Vec3::X + Vec3::Y);
        assert_eq!(point_id, 2);
        assert_eq!(ivy.point_count(), 2);
        assert!(ivy.last_point().position.abs_diff_eq(Vec3::X, 1e-6));
        assert_eq!(ivy.pending_growth.map(|s| (s.from, s.to)), Some((Vec3::X, Vec3::X + Vec3::Y)));
        assert_eq!(ivy.growth_progress, 0.0);
    }

    #[test]
    fn phase_cycle_keeps_advancing_growth_and_fires_completion_events() {
        // Runs the full `ivy_grower` system across many frames so the
        // guard-toggle phase cycle is exercised. A pending segment must advance
        // on the growth passes and complete, emitting one event per segment.
        let mut world = World::new();
        world.init_resource::<IvyStorage>();
        world.init_resource::<IvyDirectionProposer>();
        world.init_resource::<PublicWalls>();
        world.init_resource::<WallSpatialHash>();
        world.init_resource::<PrevWallHoles>();
        world.init_resource::<Events<IvySegmentDoneGrowingMsg>>();
        world.insert_resource(GameTime {
            delta: 0.075,
            ..Default::default()
        });
        let id;
        {
            let mut storage = world.resource_mut::<IvyStorage>();
            id = storage.spawn_ivy(Vec3::ZERO, Vec3::X, 1.0..2.0, 3..4, 7);
            let ivy = storage.get_mut(id).unwrap();
            ivy.add_point(Vec3::X, Vec3::X);
            ivy.pending_growth = Some(IvyGrowthSegment {
                from: Vec3::X,
                to: Vec3::X * 2.0,
            });
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(ivy_grower);
        // ~60 frames: the guard toggle gives ~30 growth passes of 0.075 s each
        // (> 0.99), so the segment completes.
        for _ in 0..60 {
            schedule.run(&mut world);
        }
        let storage = world.resource::<IvyStorage>();
        let ivy = storage.get(id).unwrap();
        assert_eq!(ivy.pending_growth, None, "pending segment must complete");
        assert!(ivy.last_point().position.abs_diff_eq(Vec3::X * 2.0, 1e-6));
        // The completion event fired at least once (drained by the schedule).
        assert!(!world.resource::<Events<IvySegmentDoneGrowingMsg>>().is_empty());
    }
}
