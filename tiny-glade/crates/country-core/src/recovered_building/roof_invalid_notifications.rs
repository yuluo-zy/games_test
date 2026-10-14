//! B process_invalid_roof_notifications over real resolved Query rows.
//! Query iteration, EventReader/EventWriter, Arc and History remain std/Bevy
//! owners. Every returned History mutation must reach an explicitly configured
//! ledger; producing the mutation is not a successful History commit.
use crate::recovered_building::{
    building_history_transactions::RoofHistoryEdit, building_wall_state::WallId,
};

#[derive(Clone, Copy, Debug)]
pub struct InvalidRoofRow<'a> {
    pub roof: &'a [u8; 88],
    pub marked_for_deletion: bool,
}

#[derive(Clone, Debug)]
pub struct InvalidRoofDeletion {
    pub wall_id: WallId,
    pub observed_delete_flag: u8,
    /// The reset-all branch emits delete events without a History mutation.
    pub history_mutation: Option<RoofHistoryEdit>,
}

/// Actual reset notification wins over per-wall invalid notifications. In the
/// normal path, each unmarked matching Query row emits once, irrespective of
/// duplicate invalid IDs; marked rows skip both event and History recording.
pub fn process_invalid_roof_notifications(
    rows: &[InvalidRoofRow<'_>],
    invalid_ids: &[WallId],
    has_reset_notification: bool,
) -> Vec<InvalidRoofDeletion> {
    rows.iter()
        .filter_map(|row| {
            let wall_id = WallId(u64::from_le_bytes(row.roof[..8].try_into().unwrap()));
            if !has_reset_notification
                && (row.marked_for_deletion || !invalid_ids.contains(&wall_id))
            {
                return None;
            }
            Some(InvalidRoofDeletion {
                wall_id,
                observed_delete_flag: 0,
                history_mutation: (!has_reset_notification)
                    .then(|| RoofHistoryEdit::delete_from_roof(row.roof)),
            })
        })
        .collect()
}
