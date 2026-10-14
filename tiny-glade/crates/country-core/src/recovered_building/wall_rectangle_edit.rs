//! B finish_edit_rectangle business decision and actual enclosure owner.
//! EventReader cursor/Bevy writes and DecoratorArchivist storage remain owners.
use crate::recovered_building::{
    building_wall_collection::RawWallPublicationSource,
    building_wall_history_edit::{WallCompoundHistoryEdit, WallHistoryEdit},
    building_wall_state::{WallId, WallShapeSnapshot},
};

#[derive(Clone, Debug)]
pub struct RectangleEditState(pub [u8; 80]);
#[derive(Clone, Debug)]
pub struct RectangleFinishDecision {
    /// Active state is emitted before reading finish events, including when
    /// all finish events were already consumed. EventId is owned by Bevy.
    pub active_state_snapshot: Option<[u8; 80]>,
    pub history: Option<WallCompoundHistoryEdit>,
    /// This is a required storage operation, not proof of a completed restore.
    pub restore_archived_decorators: bool,
    /// The original emits an empty payload event and a separate WallId event.
    pub empty_notification: bool,
    pub wall_notification: Option<WallId>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RectangleFinishError {
    InactiveFinish,
    UnsupportedState,
    MissingSource,
    NonRectangleSource,
}
impl RectangleEditState {
    pub fn tag(&self) -> u32 {
        u32::from_le_bytes(self.0[..4].try_into().unwrap())
    }
    pub fn wall(&self) -> Option<WallId> {
        let at = match self.tag() {
            1 => 16,
            2 => 8,
            _ => return None,
        };
        Some(WallId(u64::from_le_bytes(
            self.0[at..at + 8].try_into().unwrap(),
        )))
    }
}

/// The original compares orientation and dimension length; a center-only
/// difference does not cross this guard. Unordered comparisons retain the
/// x86 JA/JBE behavior rather than treating NaN as an arbitrary change.
pub fn finish_rectangle_edit_decision(
    state: &mut RectangleEditState,
    has_unread_finish_event: bool,
    current_source: Option<&RawWallPublicationSource>,
) -> Result<RectangleFinishDecision, RectangleFinishError> {
    let tag = state.tag();
    if tag > 2 {
        return Err(RectangleFinishError::UnsupportedState);
    }
    let mut out = RectangleFinishDecision {
        active_state_snapshot: None,
        history: None,
        restore_archived_decorators: false,
        empty_notification: false,
        wall_notification: None,
    };
    if tag != 0 {
        let mut snapshot = state.0;
        // Active variant1 only owns its 32-bit field at +8. Its inactive
        // neighboring word is canonicalized by the actual event writer.
        if tag == 1 {
            snapshot[12..16].fill(0);
        }
        out.active_state_snapshot = Some(snapshot);
    }
    if !has_unread_finish_event {
        return Ok(out);
    }
    let wall = state.wall().ok_or(RectangleFinishError::InactiveFinish)?;
    let old_at = if tag == 1 { 32 } else { 24 };
    let axes_at = if tag == 1 { 56 } else { 48 };
    let dims_at = if tag == 1 { 72 } else { 64 };
    let f = |at: usize| f32::from_le_bytes(state.0[at..at + 4].try_into().unwrap());
    let dot = f(old_at) * f(axes_at) + f(old_at + 4) * f(axes_at + 4);
    // Original acosf is a platform math import, distinct from glam's angle_to
    // approximation used by Rectangle2d::lerp in the replay consumer.
    let orientation = dot.acos().abs() * f32::from_bits(0x42652ee1);
    let x = f(old_at + 16) - f(dims_at);
    let y = f(old_at + 20) - f(dims_at + 4);
    let dimensions = ((x * x) + (y * y)).sqrt();
    if orientation > f32::from_bits(0x3c23d70a) || dimensions > f32::from_bits(0x3c23d70a) {
        let source = current_source.ok_or(RectangleFinishError::MissingSource)?;
        if source.shape.tag() != 1 {
            return Err(RectangleFinishError::NonRectangleSource);
        }
        let mut previous = [0; 28];
        previous[..4].copy_from_slice(&1u32.to_le_bytes());
        previous[4..].copy_from_slice(&state.0[old_at..old_at + 24]);
        out.history = Some(WallCompoundHistoryEdit {
            groups: vec![vec![WallHistoryEdit::EnclosureChange {
                wall,
                previous: WallShapeSnapshot::from_observed(previous),
                next: source.shape.clone(),
            }]],
        });
        out.empty_notification = true;
        out.wall_notification = Some(wall);
    } else {
        out.restore_archived_decorators = true;
    }
    state.0[..4].fill(0);
    Ok(out)
}
