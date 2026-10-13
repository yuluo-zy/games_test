//! `resources::history::wall_compound_edit` — 墙复合历史编辑（WP17）。
//!
//! O（反编译 CompoundEditBuilder::new_edit_sequence 0x140a82b20 / next
//! 0x140a82b40 / build 0x140a82d50 / panic_deprecated_operation 0x140a83120；
//! `EditType::new_height_changed` 0x140a83fb0 写 0x108B 负载 + Arc refcount
//! `0x8000000000000002`；观察 JSON 墙命令组 `[[{Operation}…],…]`）：
//!   - `WallHistoryEdit`：墙历史编辑操作（enum；观察 10+ 内层操作）。
//!   - `ConcurrentEdits`：`Vec<Vec<Arc<WallHistoryEdit>>>`（组数组）。
//!   - `WallHistoryCompoundEdit`：`new`（0x108B）；serde 手写。
//!   - `CompoundEditBuilder`：new_edit_sequence / next / build / next_empty /
//!     with_concurrent；`wall_deleted`（0x140a876a0，wp12 记录）。

use crate::resources::walls::WallId;
use crate::typed_gap::Gap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 墙历史编辑操作（O：enum2$；观察 JSON 内层操作）。负载字段 U。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum WallHistoryEdit {
    /// CreateWall（wall_index / wall_height / elevation / color_id / after）。
    #[default]
    CreateWall,
    /// WallChanged（wall_index / before / after / operation_ty）。
    WallChanged,
    /// DeleteWall（wall_index / wall_height / color_id / before）。
    DeleteWall,
    /// AdjustHeight（wall_index / before / after）。
    AdjustHeight,
    /// MorphPremadeShape（wall_index / before / after）。
    MorphPremadeShape,
    /// ChangeStyle / ChangeElevation / ChangeArchHeight / MorphFreehandWall /
    /// ChangeIsEnclosedState / ChangeFreehandHeightType / ChangeArchType。
    MorphFreehandWall,
    /// 未证操作（观察 open）。
    Other(Gap),
}

/// 并发编辑组（O：`[[{Operation}…],…]` 组数组；元素 `Vec<Arc<WallHistoryEdit>>`）。
/// serde 原为手写 visitor（`_::impl$0`）；此处不 derive（Arc 非 serde）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConcurrentEdits(pub Vec<Vec<Arc<WallHistoryEdit>>>);

/// 墙复合历史编辑（O：`new`；0x108B 负载；serde 手写 `_::impl$0` visitor 族）。
#[derive(Clone, Debug, PartialEq)]
pub struct WallHistoryCompoundEdit {
    /// O：有序 `ConcurrentEdits` 组。
    pub concurrent_groups: ConcurrentEdits,
    /// U：额外负载（elevation/curve 等）。
    pub aux: Gap,
}

impl Default for WallHistoryCompoundEdit {
    fn default() -> Self {
        Self {
            concurrent_groups: ConcurrentEdits::default(),
            aux: Gap,
        }
    }
}

impl WallHistoryCompoundEdit {
    /// O：`new`（0x108B 负载构建）。
    pub fn new(groups: ConcurrentEdits) -> Self {
        Self {
            concurrent_groups: groups,
            aux: Gap,
        }
    }
    /// O：`impl$5::fmt` 0x140a87970（Display）。
    pub fn describe(&self) -> String {
        format!(
            "WallHistoryCompoundEdit({} groups)",
            self.concurrent_groups.0.len()
        )
    }
}

/// 复合编辑构建器（O：new_edit_sequence 0x140a82b20 / next 0x140a82b40 /
/// build 0x140a82d50；内部状态 U）。
#[derive(Clone, Debug, Default)]
pub struct CompoundEditBuilder {
    /// U：进行中的组/编辑序列。
    pub state: Gap,
}

impl CompoundEditBuilder {
    /// O：`new_edit_sequence` 0x140a82b20。
    pub fn new_edit_sequence() -> Self {
        Self { state: Gap }
    }
    /// O：`next_empty`（当前组是否为空）。
    pub fn next_empty(&self) -> bool {
        true
    }
    /// O：`with_concurrent`（加入并发组）。
    pub fn with_concurrent(&mut self, _group: Vec<Arc<WallHistoryEdit>>) {}
    /// O：`next` 0x140a82b40（推进到下一组）。
    pub fn next(&mut self) {}
    /// O：`build` 0x140a82d50 → `Option<WallHistoryCompoundEdit>`。
    pub fn build(&self) -> Option<WallHistoryCompoundEdit> {
        Some(WallHistoryCompoundEdit::default())
    }
    /// O：`wall_deleted`（0x140a876a0；wp12 记录 Delete(snapshot) 组）。
    pub fn wall_deleted(&mut self, _wall: WallId) {}
    /// O：`panic_deprecated_operation` 0x140a83120。
    pub fn panic_deprecated_operation() -> ! {
        panic!("Deprecated wall compound edit operation")
    }
}
