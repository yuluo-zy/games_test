//! `systems::history` — 历史系统族（B 骨架对齐，WP13）。
//!
//! O（PDB 名 + 地址）：`add_history_systems` / `insert_history_resources` /
//! `add_history_events`；回放族 `finish_replay` 0x1408b36a0、`replay_mode`
//! 0x140a7c550、`replay_pop_bucket` 0x140b38250、`signifier_replay_active`
//! 0x140b1b860，及各 domain 回放系统（wall/roof/terrain/path/water/garden/
//! stairs/auto_clutter/raster）。每个 domain 的展开算法 U；本模块提供骨架声明。

pub mod history_process;
pub mod history_recording;

use crate::resources::history::History;
use crate::typed_gap::Gap;

/// O：`add_history_systems`（注册系统集合；原调度图 U）。
pub fn add_history_systems() {
    let _ = (
        history_process::process_history_commands,
        history_recording::history_finish_recording,
    );
}

/// O：`insert_history_resources`（插入 History 等资源）。
pub fn insert_history_resources(_history: Option<History>) {}

/// O：`add_history_events`（注册 HistoryCmd / 录制事件）。
pub fn add_history_events() {
    let _ = history_process::HistoryCmd(0);
}

/// O：`finish_replay` 0x1408b36a0（回放完成处理；语义 U）。
pub fn finish_replay(_history: &mut History) {}

/// O：`replay_mode` 0x140a7c550（回放节奏；语义 U）。
pub fn replay_mode(_history: &mut History) {}

/// O：`replay_pop_bucket` 0x140b38250（每调用最多弹出一个外层桶）。
pub fn replay_pop_bucket(_buffer: &mut Gap) {}

/// O：`signifier_replay_active` 0x140b1b860（回放激活标记；语义 U）。
pub fn signifier_replay_active(_buffer: &Gap) -> bool {
    false
}
