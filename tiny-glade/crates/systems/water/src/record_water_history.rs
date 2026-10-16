//! record_water_history.rs — 水面编辑历史记录（B 行 12，PDB inlinee，无独立 VA；
//! re-analyst-A 契约 §1：逻辑并入调用点，本 crate 按 core garden 域
//! `record_garden_history` 惯例以独立系统落地，宿主可接入）。
//!
//! 参数面（O，BASELINE 候选接口）：`(RasterHistoryRecordResources,
//! EventReader<RecordWaterEditToHistoryCmd>)`——`RasterHistoryRecordResources`
//! core 侧未恢复，按 `record_garden_history`（0x141695e10 同族）展开为
//! strokes + history + failures。

use bevy_ecs::prelude::*;

use country_core::resources::active_raster_stroke::ActiveRasterStrokesStorage;
use country_core::resources::history::{EditType, History};
use country_core::systems::history::recording::HistoryRecordingFailed;
use country_core::systems::world_raster::record_raster_history;

/// 水面编辑 → 历史记录命令（O：`draw_water` 的 on_record 闭包
/// `closure$3` 行 167 以 `EventWriter::write_default` 发射——单元事件）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct RecordWaterEditToHistoryCmd;

/// `record_water_history`（O 面：消费 `RecordWaterEditToHistoryCmd` 末条 →
/// `record_raster_history` 记录 `EditType::Water`（槽位 4）；失败发射
/// `HistoryRecordingFailed`）。
pub fn record_water_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordWaterEditToHistoryCmd>,
    mut strokes: ResMut<ActiveRasterStrokesStorage>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    if events.read().last().is_none() {
        return;
    }
    let mut attempted = None;
    if let Err(error) = record_raster_history(&mut strokes, 4, &mut history, |stroke| {
        let edit = EditType::Water(stroke);
        attempted = Some(edit.clone());
        edit
    }) {
        failures.write(HistoryRecordingFailed {
            edit: attempted.unwrap(),
            error,
        });
    }
}
