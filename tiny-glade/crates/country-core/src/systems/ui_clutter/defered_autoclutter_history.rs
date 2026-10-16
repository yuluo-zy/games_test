//! B 0x1409a9160 consumes every unread ID and appends EditType tag 13.
//! The original function does not open or finish a recording session.
//!
//! 原函数名映射（reviewer 基线）：`defered_autoclutter_history`（本实现名）=
//! 原 `history_record_auto_clutter` @0x1409a9160。文件路径按原 skeleton 的
//! `defered_autoclutter_history.rs` 保留。
use crate::{
    recovered_systems::clutter::AutoClutterEdit,
    resources::history::{EditType, History},
    systems::history::recording::HistoryRecordingFailed,
};
use bevy_ecs::prelude::*;
use std::sync::Arc;

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecordAutoClutterHistory(pub u64);

/// 原 `history_record_auto_clutter` @0x1409a9160。
pub fn defered_autoclutter_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordAutoClutterHistory>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    for event in events.read() {
        let edit = EditType::AutoClutter(Arc::new(AutoClutterEdit::Delete { id: event.0 }));
        if let Err(error) = history.add(edit.clone()) {
            failures.write(HistoryRecordingFailed { edit, error });
        }
    }
}
