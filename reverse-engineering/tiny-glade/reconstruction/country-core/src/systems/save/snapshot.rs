//! `systems::save::snapshot` — 快照捕获/加载（B 骨架对齐，WP13）。
//!
//! O 证据（合同见 `contracts/save-systems.md`）：
//!   - `capture_game_snapshot` 0x1417a1d00、`capture_game_snapshot_save_data`
//!     0x1417a0430、`try_chunk` 0x1417a6d70、`try_load_snapshot_data`、
//!     `validate_and_load_closest_snapshot_data` 0x1417a6e50、
//!     `init_game_state_from_snapshot` 0x1417a9c50。

use crate::typed_gap::Gap;

/// O：`capture_game_snapshot_save_data` 0x1417a0430（+闭包；快照存档数据捕获）。
pub fn capture_game_snapshot_save_data(_world: &Gap) -> Gap {
    Gap
}

/// O：`capture_game_snapshot` 0x1417a1d00（日志 “Capturing a game snapshot”）。
pub fn capture_game_snapshot(_world: &Gap) -> Gap {
    Gap
}

/// O：`try_chunk` 0x1417a6d70：`Result<Option<chunk>, anyhow>`（块/文件分块）。
pub fn try_chunk(_path: &std::path::Path) -> Result<Option<Gap>, anyhow::Error> {
    Ok(None)
}

/// O：`try_load_snapshot_data`（+闭包；读快照数据）。
pub fn try_load_snapshot_data(_blob: &[u8]) -> Result<Gap, anyhow::Error> {
    anyhow::bail!("snapshot schema not recovered")
}

/// O：`validate_and_load_closest_snapshot_data` 0x1417a6e50（失败日志 “Failed to
/// load history snapshot”）。
pub fn validate_and_load_closest_snapshot_data(_history: &Gap) -> Result<(), anyhow::Error> {
    Ok(())
}

/// O：`init_game_state_from_snapshot` 0x1417a9c50（日志 “Loading game state from a
/// snapshot”）。
pub fn init_game_state_from_snapshot(_snapshot: &Gap) {}
