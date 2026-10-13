//! `systems::history` — 历史回放系统族（WP17）。
//!
//! O（PDB 名 + 地址；合同 `contracts/replay-orchestration.md`）：
//!   - `replay_mode` 0x140a7c550 / `finish_replay` 0x1408b36a0 /
//!     `signifier_replay_active` 0x140b1b860 (+register_required_components
//!     0x140b1d680) / `replay_pop_bucket` 0x140b38250。
//!   - 领域回放：`replay_wall` 0x140951b80 / `replay_terrain` 0x140995150 /
//!     `replay_auto_clutter` 0x140aa4a60 / `replay_stairs` 0x140af75b0 /
//!     `replay_raster` 泛型（Path 0x1409febf0 / C1 0x1409fc290 / C2 0x1409f1cf0 /
//!     C3 0x1409f9930 / Garden 0x1409f6ff0 / Water 0x1409f4650）。
//!
//! wp13 的 `history_process`（HistoryCmd / undo/redo / replay_edit 0x140af6170
//! / generate_replay_cmds_from_edit）为已恢复边界；本模块在其上实现各领域
//! 回放消费端。bevy SystemParam 参数按骨架注解（enriched 文件级接口）记录，
//! 独立门禁用 typed-gap facade 编译。

pub mod boundary;
pub mod finish_replay;
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

pub use boundary::HistoryFacade as History;
pub use boundary::{AppMode, HistoryCmdFacade, PendingHistoryCommandsFacade};
