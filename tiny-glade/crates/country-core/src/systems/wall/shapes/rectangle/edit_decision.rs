//! B finish_edit_rectangle business decision and actual enclosure owner.
//! EventReader cursor/Bevy writes and DecoratorArchivist storage remain owners.
use crate::resources::history::wall_edit::MorphPremadeShapeEdit;
use crate::resources::history::wall_edit::WallCompoundHistoryEdit;
use crate::resources::history::wall_edit::WallHistoryEdit;
use crate::resources::walls::publication_fields::WallId;
use crate::resources::walls::publication_source::RawWallPublicationSource;
use crate::systems::wall::valid_enclosure::ShapeParameters;

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
        // fresh 0x141464a60: variant 5 `MorphPremadeShape`
        // `{wall @+8, previous ShapeParameters @+0x10, next @+0x2c}`; the next
        // shape is the current InnerWalls premade source.
        out.history = Some(WallCompoundHistoryEdit {
            groups: vec![vec![WallHistoryEdit::MorphPremadeShape(
                MorphPremadeShapeEdit::new(
                    wall,
                    ShapeParameters::from_observed(previous),
                    ShapeParameters::from_snapshot(&source.shape),
                ),
            )]],
        });
        out.empty_notification = true;
        out.wall_notification = Some(wall);
    } else {
        out.restore_archived_decorators = true;
    }
    state.0[..4].fill(0);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::history::wall_edit::WallHistoryOperationTag;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::publication_fields::WallShapeSnapshot;
    use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    use glam::Vec2;
    use utils::geometry::Rectangle2d;

    #[test]
    fn finish_produces_the_morph_premade_shape_payload() {
        // Rectangle orientation-edit state: variant 1 (+0 wall id @16, old axes @32,
        // old dimensions @48). The new source has rotated axes, so the
        // orientation criterion crosses the 0.01° guard.
        let mut state = RectangleEditState([0; 80]);
        state.0[..4].copy_from_slice(&1u32.to_le_bytes());
        state.0[16..24].copy_from_slice(&7u64.to_le_bytes());
        let floats = [
            (32usize, 1.0f32),
            (36, 0.0),
            (40, 0.0),
            (44, 0.0),
            (48, 2.0),
            (52, 2.0),
            (56, 0.0),
            (60, 1.0),
            (72, 2.0),
            (76, 2.0),
        ];
        for (at, value) in floats {
            state.0[at..at + 4].copy_from_slice(&value.to_le_bytes());
        }
        let source = RawWallPublicationSource {
            curve: Curve2::try_new(vec![[0.0, 0.0], [1.0, 0.0]], false).unwrap(),
            curve_kind_tag: 2,
            terrain_floor: 0.0,
            shape: WallShapeSnapshot::from_observed(
                ShapeParameters::from_shape(Shape::Rectangle(Rectangle2d::new_axis_aligned(
                    Vec2::ZERO,
                    Vec2::new(2.0, 2.0),
                )))
                .to_record(),
            ),
            style: 0,
            extra: 0.0,
            special: false,
            observed_path_scalar: 0.0,
            clutter_version: 0,
            observed_source_flag_64: 0,
        };
        let decision = finish_rectangle_edit_decision(&mut state, true, Some(&source)).unwrap();
        let compound = decision.history.expect("changed rectangle records history");
        assert_eq!(compound.groups.len(), 1);
        let edit = &compound.groups[0][0];
        assert_eq!(
            edit.operation_tag(),
            WallHistoryOperationTag::MorphPremadeShape
        );
        match edit {
            WallHistoryEdit::MorphPremadeShape(payload) => {
                assert_eq!(payload.wall_index, 7);
                assert_eq!(payload.before.tag(), 1);
                assert_eq!(payload.after.tag(), 1);
                // before axes come from the state record (+32/+36).
                assert_eq!(
                    f32::from_le_bytes(payload.before.bytes()[4..8].try_into().unwrap()),
                    1.0
                );
                // after center comes from the current rectangle source.
                assert_eq!(
                    f32::from_le_bytes(payload.after.bytes()[0xc..0x10].try_into().unwrap()),
                    0.0
                );
            }
            other => panic!("unexpected edit: {other:?}"),
        }
        assert_eq!(state.tag(), 0, "finish resets the active UI state");
    }
}
