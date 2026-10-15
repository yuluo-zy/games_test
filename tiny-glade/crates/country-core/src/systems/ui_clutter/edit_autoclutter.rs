//! Resolved-ID adapter for the real deletion and deferred recording owners.
//! This is not the still-unrestored native RMB picking/conversion function.
use super::{DeletedAutoClutter, defered_autoclutter_history::RecordAutoClutterHistory};
use crate::{
    recovered_building::history_transactions::HistoryTransactionError,
    recovered_systems::clutter::AutoClutterEdit,
    resources::history::{EditType, History},
    systems::history::recording::HistoryRecordingFailed,
};
use bevy_ecs::prelude::*;
use std::sync::Arc;

/// Caller resolves the actual generated clutter ID and opens the real history
/// session. IDs are not ECS Entity bits or wall IDs.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeleteAutoClutterCmd(pub u64);

pub fn delete_auto_clutter<S: Send + Sync + 'static>(
    mut events: EventReader<DeleteAutoClutterCmd>,
    mut deleted: ResMut<DeletedAutoClutter>,
    history: Res<History<S>>,
    mut records: EventWriter<RecordAutoClutterHistory>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    for event in events.read() {
        if !history.is_recording() {
            failures.write(HistoryRecordingFailed {
                edit: EditType::AutoClutter(Arc::new(AutoClutterEdit::Delete { id: event.0 })),
                error: HistoryTransactionError::RecordingNotStarted,
            });
            continue;
        }
        // The original insertion and history append are unconditional; an
        // already-present ID still has a record, rather than a guessed toggle.
        deleted.0.insert(event.0);
        records.write(RecordAutoClutterHistory(event.0));
    }
}
