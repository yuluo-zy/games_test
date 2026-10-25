//! Resolved-ID adapter for the real deletion and deferred recording owners.
//! This is not the still-unrestored native RMB picking/conversion function.
use super::{DeletedAutoClutter, defered_autoclutter_history::RecordAutoClutterHistory};
use crate::resources::history::transactions::HistoryTransactionError;
use crate::resources::history::auto_clutter_edit::AutoClutterEdit;
use crate::resources::history::EditType;
use crate::resources::history::History;
use crate::systems::history::recording::HistoryRecordingFailed;
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
        // 删除本身无条件（O：原 rmb_autoclutter 0x140976ca0 直接 push 删除事件+历史）；
        // 录音门只影响是否写历史。
        deleted.0.insert(event.0);
        if history.is_recording() {
            records.write(RecordAutoClutterHistory(event.0));
        } else {
            failures.write(HistoryRecordingFailed {
                edit: EditType::AutoClutter(Arc::new(AutoClutterEdit::Delete { id: event.0 })),
                error: HistoryTransactionError::RecordingNotStarted,
            });
        }
    }
}
