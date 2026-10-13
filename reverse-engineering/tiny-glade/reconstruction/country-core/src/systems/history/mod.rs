//! `systems::history` — 历史系统族（B 骨架对齐，WP13 + WP17 回放域）。
//!
//! WP13：`history_process` / `history_recording`（HistoryCmd / undo/redo /
//! 录制命令消费端 0x140af5b30）。
//!
//! WP17：回放系统族。O（PDB 名 + 地址）：
//!   - `replay_mode` 0x140a7c550 / `finish_replay` 0x1408b36a0 /
//!     `signifier_replay_active` 0x140b1b860 (+register_required_components
//!     0x140b1d680) / `replay_pop_bucket` 0x140b38250。
//!   - 领域回放：`replay_wall` 0x140951b80 / `replay_terrain` 0x140995150 /
//!     `replay_auto_clutter` 0x140aa4a60 / `replay_stairs` 0x140af75b0 /
//!     `replay_raster` 泛型（Path 0x1409febf0 / C1 0x1409fc290 / C2 0x1409f1cf0 /
//!     C3 0x1409f9930 / Garden 0x1409f6ff0 / Water 0x1409f4650）。
//!
//! `AppMode`（本模块顶层的回放域边界）是独立 typed-gap：wp20 `mode_manager`
//! 恢复的 AppMode（None/Timelapse/Wall/Focus/Tool，serde 2-qword）未含
//! Replay 判别 0xd / `Other(u32)`，wp17 回放系统按 O（0xd=Replay）依赖；两个
//! 面差异结构过大，不合并不臆造变体集。见 INTEGRATION_FINAL.md。

pub mod finish_replay;
pub mod history_process;
pub mod history_recording;
pub mod replay_auto_clutter;
pub mod replay_garden;
pub mod replay_mode;
pub mod replay_path;
pub mod replay_pop_bucket;
pub mod replay_raster;
pub mod replay_stairs;
pub mod replay_terrain;
pub mod replay_wall;
pub mod replay_water;
pub mod signifier_replay_active;

/// 应用模式（回放域边界；O：观察 0xd=Replay）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppMode {
    #[default]
    Unknown,
    Replay,
    Other(u32),
}

impl AppMode {
    pub const REPLAY_TAG: u32 = 0xd;
    pub fn tag(&self) -> u32 {
        match self {
            AppMode::Replay => Self::REPLAY_TAG,
            AppMode::Other(t) => *t,
            AppMode::Unknown => u32::MAX,
        }
    }
    pub fn from_tag(tag: u32) -> Self {
        if tag == Self::REPLAY_TAG {
            AppMode::Replay
        } else {
            AppMode::Other(tag)
        }
    }
    pub fn is_replay(&self) -> bool {
        *self == AppMode::Replay
    }
}

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
