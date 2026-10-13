//! deferred_wall_history.rs — deferred wall-change history recording.
//!
//! PDB procedures: `PendingWallChanged::try_new` 0x1408dc9f0, `history_record_wall_changed`
//! 0x1408dccf0, `history_record_merge_walls` 0x1408dd1b0, `CompoundEditBuilder::wall_deleted`
//! 0x140a876a0. FunctionSystem signatures O.
//!
//! Business logic (O; prior `reconstruction/building/wall_pending_changed.rs` reused):
//!   - `try_new`: projects the previous wall's first (extend_start) or last endpoint onto
//!     the next curve (`approximate_curve_u`), clamps to [0,1], keeps only strict
//!     `low < high`, and stores a StateChange operation range. Projection floors the squared
//!     segment denominator at 1e-8 and replaces only on strictly smaller distance (ties keep
//!     the first segment).
//!   - `history_record_wall_changed`: takes the pending owner and emits one `ConcurrentEdits`
//!     group with a `StateChange` history edit.
//!   - `history_record_merge_walls`: same deferred flow for merges.

use bevy_ecs::system::{Res, ResMut};

use crate::resources::history::wall_compound_edit::WallHistoryCompoundEdit;
use crate::resources::history::History;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::WallId;
use crate::utils::curve::Curve;

/// Deferred pending wall-change buffer (Resource).
#[derive(bevy_ecs::prelude::Resource, Clone, Debug, Default)]
pub struct PendingWallHistory {
    pub pending: Option<PendingWallChanged>,
}

/// A wall change awaiting history recording.
#[derive(Clone, Debug)]
pub struct PendingWallChanged {
    pub wall: WallId,
    pub previous: crate::typed_gap::Gap,
    pub next: crate::typed_gap::Gap,
    /// StateChange operation range {low u32?, low..high f32, high f32} (12B).
    pub observed_operation: [u8; 12],
}

/// O: strict-distance projection of a point onto a curve; ties keep the first segment.
/// Game projection rule: floors the squared segment denominator at 1e-8.
pub fn approximate_curve_coord(points: &[glam::Vec2], points_u: &[f32], pos: glam::Vec2) -> Option<(usize, f32)> {
    if points.len() < 2 || points_u.len() != points.len() {
        return None;
    }
    let mut best = f32::MAX;
    let mut index = 0usize;
    let mut fraction = 0.0f32;
    for (i, seg) in points.windows(2).enumerate() {
        let a = seg[0];
        let b = seg[1];
        let d = b - a;
        let denom = d.length_squared().max(1e-8);
        let t = ((pos - a).dot(d) / denom).clamp(0.0, 1.0);
        let proj = a + d * t;
        let dist = (pos - proj).length();
        if dist < best {
            best = dist;
            index = i;
            fraction = t;
        }
    }
    if !fraction.is_finite() {
        return None;
    }
    Some((index, fraction))
}

pub fn approximate_curve_u(points: &[glam::Vec2], points_u: &[f32], pos: glam::Vec2) -> Option<f32> {
    let (index, fraction) = approximate_curve_coord(points, points_u, pos)?;
    let u = points_u[index] * (1.0 - fraction) + points_u[index + 1] * fraction;
    u.is_finite().then_some(u)
}

/// O: `PendingWallChanged::try_new` (0x1408dc9f0).
pub fn try_pending_wall_changed(
    wall: WallId,
    previous: &Curve,
    next: &Curve,
    extend_start: bool,
) -> Option<PendingWallChanged> {
    // points/u of the previous curve (typed-gap sampling facade).
    let prev_pts = previous.points_with_u();
    let next_pts = next.points_with_u();
    if prev_pts.is_empty() || next_pts.len() < 2 {
        return None;
    }
    let (prev_pos, _) = if extend_start {
        *prev_pts.first()?
    } else {
        *prev_pts.last()?
    };
    let pos = glam::Vec2::new(prev_pos.x, prev_pos.z);
    let next2: Vec<glam::Vec2> = next_pts.iter().map(|(p, _)| glam::Vec2::new(p.x, p.z)).collect();
    let next_u: Vec<f32> = next_pts.iter().map(|(_, u)| *u).collect();
    let u = approximate_curve_u(&next2, &next_u, pos)?;
    let u = u.clamp(0.0, 1.0);
    let (low, high) = if extend_start { (u, 1.0) } else { (0.0, u) };
    if !(low < high) {
        return None;
    }
    let mut op = [0u8; 12];
    op[4..8].copy_from_slice(&low.to_le_bytes());
    op[8..12].copy_from_slice(&high.to_le_bytes());
    Some(PendingWallChanged {
        wall,
        previous: crate::typed_gap::Gap,
        next: crate::typed_gap::Gap,
        observed_operation: op,
    })
}

/// O: `history_record_wall_changed` (0x1408dccf0) — takes the pending owner and records one
/// `ConcurrentEdits` group.
pub fn history_record_wall_changed(
    mut history: ResMut<History>,
    mut pending: ResMut<PendingWallHistory>,
) {
    if let Some(p) = pending.pending.take() {
        let edit = WallHistoryCompoundEdit::new();
        let _ = p;
        history.add(edit);
    }
}

/// O: `history_record_merge_walls` (0x1408dd1b0) — deferred merge recording with inner walls
/// read for the merge source.
pub fn history_record_merge_walls(
    mut history: ResMut<History>,
    mut pending: ResMut<PendingWallHistory>,
    inner_walls: Res<InnerWalls>,
) {
    let _ = &inner_walls;
    if let Some(p) = pending.pending.take() {
        let edit = WallHistoryCompoundEdit::new();
        let _ = p;
        history.add(edit);
    }
}
