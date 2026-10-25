//! record_water_history.rs — 水面编辑历史记录（B 行 12，PDB inlinee，无独立 VA；
//! fresh 2026-10-17 重建）。
//!
//! 记录形式 `inline_instance, inlinee_declaration; instances=5`；内联宿主 =
//! `bevy_ecs::system::schedule_system::run_unsafe<FunctionSystem<fn record_water_history,…>>`
//! **0x140e92a80**（module 1011，inline_record 5196）。逻辑（O，`rwh_host.txt`）：
//! 1. 用资源 id 从 World 取 `ActiveRasterStrokesStorage`、`History`、`Events<Cmd>`；
//! 2. `EventReader` 游标读双缓冲未读事件数（`saturating_sub` + 缓冲区长度差）；
//! 3. 任一缓冲有未读 → 调 `country_core::systems::world_raster::record_raster_history`
//!    （0x957990）提交历史事务；
//! 4. 更新 reader 游标（last_event_count）与 last_run tick。
//!
//! 结果被忽略——原版无 `HistoryRecordingFailed` 发射（先前按 garden 惯例近似，已删）。

use bevy_ecs::prelude::*;

use country_core::resources::active_raster_stroke::ActiveRasterStrokesStorage;
use country_core::resources::history::{EditType, History};
use country_core::systems::world_raster::record_raster_history;

/// 水面编辑 → 历史记录命令（O：`draw_water` 的 on_record 闭包 `closure$3` 行 167 以
/// `EventWriter::write_default` 发射——单元事件）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct RecordWaterEditToHistoryCmd;

/// `record_water_history`（O：内联宿主 0x140e92a80，`read().last()` 语义 + 单次转交）。
///
/// 参数面（O，BASELINE 候选接口）：`(RasterHistoryRecordResources,
/// EventReader<RecordWaterEditToHistoryCmd>)`——`RasterHistoryRecordResources` core 侧
/// 未恢复，按内联宿主取用两个资源展开为 strokes + history（无 failures，见模块注）。
pub fn record_water_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordWaterEditToHistoryCmd>,
    mut strokes: ResMut<ActiveRasterStrokesStorage>,
    mut history: ResMut<History<S>>,
) {
    // O：有未读事件（`.read().last()` 非 None）才转交历史记录。
    if events.read().last().is_none() {
        return;
    }
    // O：`record_raster_history`（0x957990）：取槽位 4 的待提交笔划，构造
    //    `EditType::Water` 入历史事务；结果忽略（原版无错误发射）。
    let _ = record_raster_history(&mut strokes, 4, &mut history, |stroke| {
        EditType::Water(stroke)
    });
}
