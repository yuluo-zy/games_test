//! Roof recording writes the same History owner used by mode_manager and
//! replay. UI record notifications are separate from live edit commands so
//! replaying an edit does not record another copy of itself.
use crate::resources::history::EditType;
use crate::resources::history::History;
use crate::resources::history::building_transactions::BuildingEdit;
use crate::resources::history::building_transactions::RoofHistoryEdit;
use crate::resources::history::building_transactions::RoofVisualHistoryRecord;
use crate::resources::history::transactions::HistoryTransactionError;
use crate::resources::roofs::commands::RecordRoofHistory;
use bevy_ecs::prelude::*;

#[derive(Event, Clone, Debug)]
pub struct RecordRoofVisualHistory(pub RoofVisualHistoryRecord);
#[derive(Event, Clone, Copy, Debug)]
pub struct RecordRoofDirectionHistory {
    pub wall_id: u64,
    pub from: u8,
    pub to: u8,
}
#[derive(Event, Clone, Debug)]
pub struct HistoryRecordingFailed {
    pub edit: EditType,
    pub error: HistoryTransactionError,
}

fn record<S>(
    history: &mut History<S>,
    edit: RoofHistoryEdit,
    failures: &mut EventWriter<HistoryRecordingFailed>,
) {
    let edit = EditType::from(edit);
    if let Err(error) = history.add(edit.clone()) {
        failures.write(HistoryRecordingFailed { edit, error });
    }
}
pub fn record_roof_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordRoofHistory>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    for event in events.read() {
        record(&mut history, event.0.clone(), &mut failures);
    }
}
/// Native deferred visual reader consumes only its last unread event.
pub fn record_roof_visual_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordRoofVisualHistory>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    let Some(event) = events.read().last() else {
        return;
    };
    let BuildingEdit::Roof(edit) = event.0.edit() else {
        unreachable!()
    };
    record(&mut history, edit, &mut failures);
}
/// Ridge-direction recording consumes every event in original order.
pub fn record_roof_direction_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordRoofDirectionHistory>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    for event in events.read() {
        record(
            &mut history,
            RoofHistoryEdit::direction(event.wall_id, event.from, event.to),
            &mut failures,
        );
    }
}
pub fn register_recording_events(world: &mut World) {
    world.init_resource::<Events<RecordRoofVisualHistory>>();
    world.init_resource::<Events<RecordRoofDirectionHistory>>();
    world.init_resource::<Events<HistoryRecordingFailed>>();
}

/// B `history_finish_recording` 0x1409dd790（fresh）。
///
/// 语义（decomp 逐段）：
/// 1. `pending == None` → 直接 return（`SBORROW8(0, History+0x30)`）。
/// 2. 读取三个 owner 的归档计数差（decorator/stairs/第三 owner），把各自的
///    计数水位写回 `end - ((end - start) - cur)`；任一非零即"本轮有变更"。
///    另 `entries.len > 1 && entry[+0x19] < 2 && (*(*entry).+0xb9)` 也置变更位。
/// 3. 有变更：`DecoratorArchivist::extract_diff` +
///    `StairsState::extract_diff_and_make_backup` →
///    `History::add_decorator_diff`（tag 0xE 记录 + 两 aux Arc）→
///    `History::finish_input_recording`。
/// 4. 末尾写 `History+0x1b0 = 1`（标记字节）。
///
/// **登记 U**：3 个计数 owner 的具体资源类型与 +0xb9/+0x1b0 字段归属未锚定；
/// 本实现以"传入 diff 是否非空"作为变更位（等价语义面），并由调用方在提交后
/// 自行维护标记字节。
pub fn history_finish_recording<S: Send + Sync + 'static>(
    history: &mut History<S>,
    decorator_diff: &crate::resources::history::decorator_edit::DecoratorHistoryDiffMap,
    stairs_diff: &crate::resources::stairs::stairs_state::StairsHistoryDiff,
    stairs_edges: &mut std::collections::HashSet<
        crate::resources::stairs::stairs_state::StairGraphEdge,
    >,
    verify_stair_checksums: bool,
) -> Option<crate::resources::history::transactions::HistoryCommit> {
    if !history.is_recording() {
        return None;
    }
    let changed = !decorator_diff.is_empty() || !stairs_diff.is_empty();
    if !changed {
        return None;
    }
    history.add_decorator_diff(
        decorator_diff,
        stairs_diff,
        stairs_edges,
        verify_stair_checksums,
    );
    match history.finish_input_recording() {
        Ok(commit) => commit,
        Err(error) => {
            log::error!("history_finish_recording: {error:?}");
            None
        }
    }
}
