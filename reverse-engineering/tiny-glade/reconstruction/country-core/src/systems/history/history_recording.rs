//! `systems::history::history_recording` — 录制启停系统（B 骨架对齐，WP13）。
//!
//! O：`history_start_recording` / `history_finish_recording` 0x1409dd790 驱动
//! `History::start_input_recording` / `finish_input_recording`（PDB 名 + 地址）。
//! 精确 ECS 参数/调度 U。

use crate::resources::history::History;

/// O：开始输入录制（委托 History::start_input_recording；AppMode replay 断言在
/// History 内）。
pub fn history_start_recording(history: &mut History) {
    history.start_input_recording();
}

/// O：结束输入录制（委托 History::finish_input_recording；提交为空时不提交）。
pub fn history_finish_recording(history: &mut History) {
    history.finish_input_recording();
}
