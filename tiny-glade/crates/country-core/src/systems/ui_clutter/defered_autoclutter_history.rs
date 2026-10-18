//! B 0x1409a9160 consumes every unread ID and appends EditType tag 13.
//! The original function does not open or finish a recording session.
//!
//! 原函数名映射（reviewer 基线）：`defered_autoclutter_history`（本实现名）=
//! 原 `history_record_auto_clutter` @0x1409a9160。文件路径按原 skeleton 的
//! `defered_autoclutter_history.rs` 保留。
use crate::resources::history::auto_clutter_edit::AutoClutterEdit;
use crate::resources::history::EditType;
use crate::resources::history::History;
use bevy_ecs::prelude::*;
use std::sync::Arc;

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecordAutoClutterHistory(pub u64);

/// 原 `history_record_auto_clutter` @0x1409a9160。
///
/// fresh 0x1409a9160：逐事件（stride 0x10）alloc 0x18 记录 {tag 1,1,id u64} →
/// `History::add()`，**忽略返回值、无失败事件**。按判等修正：删早期
/// `if let Err → HistoryRecordingFailed` 错误分支（原版无该分支）。
pub fn defered_autoclutter_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordAutoClutterHistory>,
    mut history: ResMut<History<S>>,
) {
    for event in events.read() {
        let edit = EditType::AutoClutter(Arc::new(AutoClutterEdit::Delete { id: event.0 }));
        // 原版忽略 History::add 返回值（fresh 对 add 结果无分支）。
        let _ = history.add(edit);
    }
}
