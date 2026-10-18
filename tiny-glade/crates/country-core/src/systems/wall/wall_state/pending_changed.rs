//! Actual PendingWallChanged.try_new and history_record_wall_changed producers.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
use crate::resources::history::wall_edit::WallAuthoritySnapshot;
use crate::resources::history::wall_edit::WallCompoundHistoryEdit;
use crate::resources::history::wall_edit::WallHistoryEdit;
use crate::resources::roofs::geometry::surface::Curve2;
use crate::resources::walls::publication_fields::WallId;
#[derive(Clone, Debug)]
pub struct PendingWallChanged {
    pub wall: WallId,
    pub previous: WallAuthoritySnapshot,
    pub next: WallAuthoritySnapshot,
    pub observed_operation: [u8; 12],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PendingWallChangedError {
    InvalidCurve,
    InvalidProjection,
}

/// Original Curve2::approx_coord_at_pos and approx_u_at_pos: strict distance
/// replacement preserves the first segment on ties. Duplicate endpoints use
/// the original 1e-8 denominator floor; this is a game projection rule.
pub fn approximate_curve_coordinate(
    curve: &Curve2,
    position: [f32; 2],
) -> Result<(usize, f32), PendingWallChangedError> {
    if curve.points.len() < 2 || curve.points_u.len() != curve.points.len() {
        return Err(PendingWallChangedError::InvalidCurve);
    }
    let mut best = f32::MAX;
    let mut index = 0;
    let mut fraction = 0.;
    for (i, segment) in curve.points.windows(2).enumerate() {
        let a = segment[0];
        let b = segment[1];
        let x = b[0] - a[0];
        let z = b[1] - a[1];
        let denom = x * x + z * z;
        let denom = if denom > f32::from_bits(0x322bcc77) {
            denom
        } else {
            f32::from_bits(0x322bcc77)
        };
        let t = ((position[0] - a[0]) * x + (position[1] - a[1]) * z) / denom;
        let t = if t < 0. { 0. } else { t };
        let t = if t > 1. { 1. } else { t };
        let px = (1. - t) * a[0] + t * b[0];
        let pz = (1. - t) * a[1] + t * b[1];
        let dx = position[0] - px;
        let dz = position[1] - pz;
        let distance = (dx * dx + dz * dz).sqrt();
        if distance < best {
            best = distance;
            index = i;
            fraction = t;
        }
    }
    if !fraction.is_finite() {
        return Err(PendingWallChangedError::InvalidProjection);
    }
    Ok((index, fraction))
}
pub fn approximate_curve_u(
    curve: &Curve2,
    position: [f32; 2],
) -> Result<f32, PendingWallChangedError> {
    let (index, fraction) = approximate_curve_coordinate(curve, position)?;
    let u = curve.points_u[index] * (1. - fraction) + curve.points_u[index + 1] * fraction;
    if !u.is_finite() {
        return Err(PendingWallChangedError::InvalidProjection);
    }
    Ok(u)
}
pub fn try_pending_wall_changed(
    wall: WallId,
    previous: WallAuthoritySnapshot,
    next: WallAuthoritySnapshot,
    extend_start: bool,
) -> Result<Option<PendingWallChanged>, PendingWallChangedError> {
    let position = if extend_start {
        previous.source.curve.points.first()
    } else {
        previous.source.curve.points.last()
    }
    .ok_or(PendingWallChangedError::InvalidCurve)?;
    let u = approximate_curve_u(&next.source.curve, *position)?;
    let u = if u < 0. { 0. } else { u };
    let u = if u > 1. { 1. } else { u };
    let (low, high) = if extend_start { (u, 1.) } else { (0., u) };
    if !(low < high) {
        return Ok(None);
    }
    let mut op = [0; 12];
    op[4..8].copy_from_slice(&low.to_le_bytes());
    op[8..12].copy_from_slice(&high.to_le_bytes());
    Ok(Some(PendingWallChanged {
        wall,
        previous,
        next,
        observed_operation: op,
    }))
}
/// Consumes the real pending owner even before History.add; no snapshot Eq
/// skip is manufactured. The ledger decides whether that owner is recorded.
pub fn take_pending_wall_changed(
    pending: &mut Option<PendingWallChanged>,
) -> Option<WallCompoundHistoryEdit> {
    pending.take().map(|p| WallCompoundHistoryEdit {
        groups: vec![vec![WallHistoryEdit::StateChange {
            wall: p.wall,
            previous: p.previous,
            next: p.next,
            observed_operation: p.observed_operation,
        }]],
    })
}
