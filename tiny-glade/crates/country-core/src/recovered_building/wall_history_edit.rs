//! B Wall compound snapshot business. Vec/Arc/ledger ownership remains std.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
use crate::recovered_building::{
    building_wall_collection::RawWallPublicationSource,
    building_wall_state::{WallId, WallShapeSnapshot},
    surface::{Curve2, CurveError},
};

/// Additional original source fields copied into Wall history. Their meanings
/// are not inferred; caller supplies actual observed bits, including global-
/// dependent source+69 rather than manufacturing a universal creation default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallHistoryMetadata {
    pub source_60_bits: u32,
    pub source_67: u8,
    pub source_69: u8,
}
impl WallHistoryMetadata {
    /// Actual new_wall_w_params stores. The named game global is supplied by
    /// its startup owner; this function does not guess that owner's value.
    pub fn for_new_wall(dev_override_is_demo_mode: bool) -> Self {
        Self {
            source_60_bits: 0,
            source_67: 0,
            source_69: if dev_override_is_demo_mode { 0 } else { 5 },
        }
    }
}
#[derive(Clone, Debug)]
pub struct WallAuthoritySnapshot {
    pub source: RawWallPublicationSource,
    pub metadata: WallHistoryMetadata,
}
#[derive(Clone, Debug)]
pub enum WallHistoryEdit {
    Create {
        wall: WallId,
        source: WallAuthoritySnapshot,
        color_id: u32,
        restored_height: f32,
        restored_elevation: f32,
    },
    Delete {
        wall: WallId,
        source: WallAuthoritySnapshot,
        color_id: u32,
    },
    /// Original WallHistoryEdit niche 0x8000000000000004.
    EnclosureChange {
        wall: WallId,
        previous: WallShapeSnapshot,
        next: WallShapeSnapshot,
    },
    StateChange {
        wall: WallId,
        previous: WallAuthoritySnapshot,
        next: WallAuthoritySnapshot,
        /// Original operation tag and two float fields; interpreted by the
        /// actual replay consumer, not guessed from serialized enum names.
        observed_operation: [u8; 12],
    },
}
#[derive(Clone, Debug)]
pub struct WallCompoundHistoryEdit {
    /// Original ordered ConcurrentEdits groups; do not flatten the groups.
    pub groups: Vec<Vec<WallHistoryEdit>>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum WallSnapshotError {
    InvalidSlice,
    Curve(CurveError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallReplaySourceError {
    MissingSource,
    DuplicateSource,
    EnclosureCommandRequired,
}
#[derive(Clone, Debug)]
pub struct WallRestoreColorRequest {
    pub wall: WallId,
    pub color_id: u32,
    pub observed_special_owner: bool,
}
#[derive(Clone, Debug)]
pub struct WallReplayAudio {
    pub wall: WallId,
    pub anchor: [f32; 3],
    pub kind: u8,
    pub flag: u8,
}
#[derive(Clone, Debug)]
pub struct InstantWallReplayOutcome {
    pub new_source_notification: Option<WallId>,
    pub updated_source_notification: Option<WallId>,
    pub removed_source_notification: Option<WallId>,
    pub color_request: Option<WallRestoreColorRequest>,
    pub audio: Option<WallReplayAudio>,
}

pub fn record_deleted_wall(
    wall: WallId,
    source: &WallAuthoritySnapshot,
    color_id: u32,
) -> WallCompoundHistoryEdit {
    WallCompoundHistoryEdit {
        groups: vec![vec![WallHistoryEdit::Delete {
            wall,
            source: source.clone(),
            color_id,
        }]],
    }
}

/// Actual instant Create/Delete/StateChange authority writes. The caller keeps
/// its ID counter; original new_wall_with_id never increments that counter.
/// Missing remove/duplicate restore are original panic domains represented as
/// typed errors, without claiming their partially mutated std-map semantics.
pub fn apply_instant_wall_edit(
    sources: &mut std::collections::HashMap<WallId, WallAuthoritySnapshot>,
    edit: &WallHistoryEdit,
    undo: bool,
    terrain: &crate::recovered_building::building_wall_collection::TerrainHeightGrid512,
) -> Result<InstantWallReplayOutcome, WallReplaySourceError> {
    let mut out = InstantWallReplayOutcome {
        new_source_notification: None,
        updated_source_notification: None,
        removed_source_notification: None,
        color_request: None,
        audio: None,
    };
    let (wall, restore, source, color) = match edit {
        WallHistoryEdit::Create {
            wall,
            source,
            color_id,
            ..
        } => (*wall, !undo, source, *color_id),
        WallHistoryEdit::Delete {
            wall,
            source,
            color_id,
        } => (*wall, undo, source, *color_id),
        // Enclosure writes emit two updated notifications. Use the command
        // consumer, whose Vec preserves that original multiplicity.
        WallHistoryEdit::EnclosureChange { .. } => {
            return Err(WallReplaySourceError::EnclosureCommandRequired);
        }
        WallHistoryEdit::StateChange {
            wall,
            previous,
            next,
            ..
        } => {
            let selected = if undo { previous } else { next };
            if let Some(target) = sources.get_mut(wall) {
                *target = selected.clone();
                out.updated_source_notification = Some(*wall);
            }
            let [x, z] = selected.source.curve.pos_at_u(0.5);
            out.audio = Some(WallReplayAudio {
                wall: *wall,
                anchor: [x, terrain.sample_at_world_xz(x, z), z],
                kind: 2,
                flag: 1,
            });
            return Ok(out);
        }
    };
    if restore {
        if sources.contains_key(&wall) {
            return Err(WallReplaySourceError::DuplicateSource);
        }
        let mut restored = source.clone();
        if let WallHistoryEdit::Create {
            restored_height,
            restored_elevation,
            ..
        } = edit
        {
            restored.source.extra = *restored_height;
            restored.source.terrain_floor = *restored_elevation;
            out.updated_source_notification = Some(wall);
        }
        out.color_request = Some(WallRestoreColorRequest {
            wall,
            color_id: color,
            observed_special_owner: source.source.special,
        });
        sources.insert(wall, restored);
        out.new_source_notification = Some(wall);
    } else {
        sources
            .remove(&wall)
            .ok_or(WallReplaySourceError::MissingSource)?;
        out.removed_source_notification = Some(wall);
    }
    Ok(out)
}

/// Actual private Curve::crashy_slice rule; coordinate search/interpolation and
/// Curve recomputation reuse the existing recovered Curve2 APIs.
pub fn slice_source_curve(
    curve: &Curve2,
    start: f32,
    end: f32,
) -> Result<Curve2, WallSnapshotError> {
    if !start.is_finite() || !end.is_finite() || end <= start {
        return Err(WallSnapshotError::InvalidSlice);
    }
    let (first, _) = curve.coord_at_u(start);
    let (last, _) = curve.coord_at_u(end);
    let a = curve.pos_at_u(start);
    let b = curve.pos_at_u(end);
    let first_inside = first + if a == curve.points[first + 1] { 2 } else { 1 };
    let last_inside = if b == curve.points[last] {
        last.saturating_sub(1)
    } else {
        last
    };
    let mut points = vec![a];
    if first_inside <= last_inside {
        points.extend_from_slice(&curve.points[first_inside..=last_inside]);
    }
    points.push(b);
    Curve2::try_new(points, false).map_err(WallSnapshotError::Curve)
}

/// Actual ev_new_wall: seed-create group, then seed->complete-state group.
/// Geometry slice occurs before all cloning/recording, as in the original.
pub fn record_new_wall(
    wall: WallId,
    source: &WallAuthoritySnapshot,
    color_id: u32,
) -> Result<WallCompoundHistoryEdit, WallSnapshotError> {
    let ratio = 0.2 / source.source.curve.length;
    let end = if ratio < 0.0 { 0.0 } else { ratio };
    let end = if end > 1.0 { 1.0 } else { end };
    let curve = slice_source_curve(&source.source.curve, 0.0, end)?;
    let mut seed = source.clone();
    seed.source.curve = curve;
    seed.source.shape = WallShapeSnapshot::from_observed({
        let mut bytes = [0; 28];
        bytes[..4].copy_from_slice(&2u32.to_le_bytes());
        bytes
    });
    seed.source.curve_kind_tag = 2;
    let mut observed_operation = [0; 12];
    observed_operation[8..12].copy_from_slice(&end.to_le_bytes());
    Ok(WallCompoundHistoryEdit {
        groups: vec![
            vec![WallHistoryEdit::Create {
                wall,
                source: seed.clone(),
                color_id,
                restored_height: source.source.extra,
                restored_elevation: source.source.terrain_floor,
            }],
            vec![WallHistoryEdit::StateChange {
                wall,
                previous: seed,
                next: source.clone(),
                observed_operation,
            }],
        ],
    })
}

/// Rectangle2d::lerp business composition. The original angle approximation,
/// normalized dot and signed cross are glam Vec2::angle_to, reused as a dependency.
/// Other enclosure variants have not been recovered by this producer yet.
pub fn interpolate_rectangle_enclosure(
    previous: &WallShapeSnapshot,
    next: &WallShapeSnapshot,
    progress: f32,
) -> Result<
    WallShapeSnapshot,
    crate::recovered_building::building_wall_freeform_cache::ShapeHeightError,
> {
    use crate::recovered_building::building_wall_freeform_cache::ShapeHeightError;
    use glam::Vec2;
    if previous.tag() != 1 || next.tag() != 1 {
        return Err(ShapeHeightError::InvalidRectangle);
    }
    let read = |s: &WallShapeSnapshot, at: usize| {
        Vec2::new(
            f32::from_le_bytes(s.bytes()[at..at + 4].try_into().unwrap()),
            f32::from_le_bytes(s.bytes()[at + 4..at + 8].try_into().unwrap()),
        )
    };
    let old_axes = read(previous, 4);
    let next_axes = read(next, 4);
    let angle = old_axes.angle_to(next_axes) * progress;
    let (sin, cos) = angle.sin_cos();
    // Preserve the original SSE operand order rather than Vec2::rotate's
    // equivalent subtraction expression (signed-zero bits can differ).
    let axes = Vec2::new(
        (-sin) * old_axes.y + cos * old_axes.x,
        cos * old_axes.y + sin * old_axes.x,
    );
    let center = read(previous, 12) * (1.0 - progress) + read(next, 12) * progress;
    let dimensions = read(previous, 20) * (1.0 - progress) + read(next, 20) * progress;
    if !dimensions.is_finite()
        || !((axes.length_squared() - 1.0).abs() <= f32::from_bits(0x3951b717))
    {
        return Err(ShapeHeightError::InvalidRectangle);
    }
    let mut bytes = [0; 28];
    bytes[..4].copy_from_slice(&1u32.to_le_bytes());
    for (index, value) in [
        axes.x,
        axes.y,
        center.x,
        center.y,
        dimensions.x,
        dimensions.y,
    ]
    .into_iter()
    .enumerate()
    {
        bytes[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
    }
    Ok(WallShapeSnapshot::from_observed(bytes))
}

pub fn rectangle_from_enclosure(
    shape: &WallShapeSnapshot,
) -> Result<
    crate::recovered_building::building_wall_freeform_cache::PreviewRectangleObserved,
    crate::recovered_building::building_wall_freeform_cache::ShapeHeightError,
> {
    if shape.tag() != 1 {
        return Err(crate::recovered_building::building_wall_freeform_cache::ShapeHeightError::InvalidRectangle);
    }
    let value = |at| f32::from_le_bytes(shape.bytes()[at..at + 4].try_into().unwrap());
    Ok(
        crate::recovered_building::building_wall_freeform_cache::PreviewRectangleObserved {
            axes: [value(4), value(8)],
            center: [value(12), value(16)],
            dimensions: [value(20), value(24)],
        },
    )
}
