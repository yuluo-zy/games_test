//! `resources::history::stairs_history_edit` — 楼梯历史编辑记录（WP17）。
//!
//! O（观察 JSON `stairs` 字段：before/after 为整数边对数组，hash_before/
//! hash_after；Stairs 命令 SupportsDelete 7/SupportsRecreate 2。serde 手写
//! serialize 0x140a18ba0 / deserialize 0x140a18ee0）：
//!   - `StairsHistoryEdit`：楼梯编辑记录（ReplayCmd 判别 10 附近负载）。
//!   - `StairsHistoryDiff`：楼梯拓扑 diff（边对 + hash）；
//!     replay_pop_bucket 的 `StairsState::restore_backup/apply_diff` 消费。
//!   - `is_stairs_history_diff_empty` 0x140a842a0（wp13 已实现）。

use crate::resources::history::StairGraphEdge;
use crate::typed_gap::Gap;
use serde::{Deserialize, Serialize};

/// 楼梯编辑记录（O：边对 before/after + hash；载荷字段 U）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StairsHistoryEdit {
    /// O：`before` 边集（观察：边对数组）。
    #[serde(default)]
    pub before: Vec<StairGraphEdge>,
    /// O：`after` 边集。
    #[serde(default)]
    pub after: Vec<StairGraphEdge>,
    /// O：`hash_before` / `hash_after`（u64）。
    #[serde(default)]
    pub hash_before: u64,
    #[serde(default)]
    pub hash_after: u64,
    /// U：额外载荷。
    #[serde(default)]
    pub aux: Gap,
}

/// 楼梯历史 diff（O：replay_edit → ReplayBucket 附加；边对 + hash）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StairsHistoryDiff {
    #[serde(default)]
    pub before: Vec<StairGraphEdge>,
    #[serde(default)]
    pub after: Vec<StairGraphEdge>,
    #[serde(default)]
    pub hash_before: u64,
    #[serde(default)]
    pub hash_after: u64,
}

impl StairsHistoryDiff {
    /// O：`is_stairs_history_diff_empty` 0x140a842a0（容器 +0x30 非空）。
    pub fn is_empty(&self) -> bool {
        self.before.is_empty() && self.after.is_empty()
    }
}
