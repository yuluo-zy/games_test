//! deferred_terrain_history.rs — 延迟地形历史记录（WP8 恢复）。
//!
//! 源：B skeleton `systems/terrain/deferred_terrain_history.rs`。反编译 `history_record_terrain`@0x140ac9770（行 16）。
//! ECS 签名（O）：
//! ```
//! fn history_record_terrain(
//!   ResMut<History>, ResMut<PendingTerrainHistory>, EventWriter<DebugHistoryMsg>)
//! ```
//! 业务（O）：`PendingTerrainHistory` 置脏标记（`0x8000000000000002` sentinel）；非首次变更跳过；
//! 首次构建历史记录并 `History::add`，发 `DebugHistoryMsg`。
//! `PendingTerrainHistory::default`（O，行 10）。

#![allow(dead_code, unused_imports)]

use crate::systems::terrain::{DebugHistoryMsg, PendingTerrainHistory};
use crate::typed_gap::Gap;

/// 历史记录系统（O：`history_record_terrain`@0x140ac9770，行 16）。
pub fn history_record_terrain(
    _history: &mut Gap,
    _pending: &mut PendingTerrainHistory,
    _msg: &mut Vec<DebugHistoryMsg>,
) {
    unimplemented!("recover history_record_terrain dirty-flag + History::add")
}
