//! `deferred_terrain_history.rs` — 延迟地形历史（B 0x140ac9770，O）。
//!
//! 证据：`terrain-water/decomp/decomp-0x140ac9770.txt`、章节、re-analyst-interaction：
//! - `PendingTerrainHistory` 脏标记（0x8000000000000002 哨兵）；
//! - 每帧先置脏哨兵；若**上一状态非哨兵**（本次变化发生）→ 用 pending 4 qword 构建地形
//!   历史记录 → `History::add` + push DebugHistoryMsg；之后不重复记录。
//!
//! decomp 结构：`param_2` = ResMut<PendingTerrainHistory>（`state u64 @0` + `pending [u64;4]
//! @8..0x28`）；`param_1` = ResMut<History>；`param_3` = EventWriter（8 B 事件）。
//! 历史记录 = 0x38 B 分配（tag 对 + 4 个 pending qword + 旧 state），经 `FUN_140a84ec0`
//! 入历史。`TerrainEdit` 字段语义待取证（falloff/height/position/id 序）。
use bevy_ecs::prelude::*;
use std::sync::Arc;

use crate::resources::history::{EditType, History};

/// 脏哨兵（O：0x140ac9770 `-0x7ffffffffffffffe` = i64::MIN+2）。
pub const DEFERRED_TERRAIN_SENTINEL: u64 = 0x8000_0000_0000_0002;

/// 延迟地形历史状态资源（O：state @0 + pending 4 qword @8）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PendingTerrainHistory {
    /// 状态（哨兵 = 脏/待记录；其他 = 上一记录后的标记）。
    pub state: u64,
    /// 待构建历史记录的地形编辑数据（4 qword；语义 U，布局 O）。
    pub pending: [u64; 4],
}

/// 地形历史编辑记录（I：0x38 B 记录 {tag 对, pending 4 qword, 旧 state}；字段语义 U）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TerrainEdit {
    pub pending: [u64; 4],
    pub previous_state: u64,
}

/// `history_record_terrain` @0x140ac9770（O）。
///
/// 每帧：置脏哨兵；若上帧非哨兵（刚发生变化）→ 构建 [`TerrainEdit`] 并 `History::add` +
/// push [`DebugHistoryMsg`]；之后不重复。`DebugHistoryMsg` 载荷语义 U。
pub fn history_record_terrain<S: Send + Sync + 'static>(
    mut history: ResMut<History<S>>,
    mut pending: ResMut<PendingTerrainHistory>,
    mut debug: EventWriter<DebugHistoryMsg>,
) {
    let previous = pending.state;
    pending.state = DEFERRED_TERRAIN_SENTINEL;
    if previous == DEFERRED_TERRAIN_SENTINEL {
        return;
    }
    let edit = TerrainEdit {
        pending: pending.pending,
        previous_state: previous,
    };
    // 原版 fresh 0x140ac9770：`History::add()` 后**无条件**写 8B debug 事件，
    // 对 add 返回值无分支（`resources::history::History::add()` 行 40 后直接进事件写）。
    // 按协议判等修正：删早期 `if let Err → log+return` 错误分支（add 失败时原版仍写）。
    let _ = history.add(EditType::Terrain(Arc::new(edit)));
    debug.write(DebugHistoryMsg(previous));
}

/// 地形历史记录后的调试消息（I：8 B 事件，载荷 = 旧状态标记）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DebugHistoryMsg(pub u64);
