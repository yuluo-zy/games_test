//! `resources::history::wall_history` — 墙历史事件处理（WP17）。
//!
//! O（反编译 ev_new_wall 0x140a83300 / ev_height_changed 0x140a83f80 /
//! EditType::new_height_changed 0x140a83fb0；宿主 `ui_adjust_wall_height_mode::
//! history_record_adjust_wall_height` 0x140ac79e0）：
//!   - `impl$0` = 墙历史事件处理 trait（ev_new_wall / ev_height_changed）。
//!   - `ev_height_changed`：构造 `EditType::new_height_changed` 后
//!     `History::add()`（墙高度变更 → 编辑链）。
//!   - `EditType::new_height_changed`：写 `EditType` 判别 7，负载
//!     `WallHistoryCompoundEdit`（0x108B）。

use crate::resources::history::wall_compound_edit::WallHistoryCompoundEdit;
use crate::resources::history::{EditType, WallId};
use crate::typed_gap::Gap;

/// 墙历史事件处理（O：`impl$0`；ev_new_wall / ev_height_changed）。
#[derive(Clone, Debug, Default)]
pub struct WallHistoryEventHandler {
    /// U：事件处理器状态。
    pub state: Gap,
}

impl WallHistoryEventHandler {
    /// O：`ev_new_wall` 0x140a83300：新墙事件 → 编辑命令。
    pub fn ev_new_wall(&mut self, _wall: WallId) -> Option<EditType> {
        None
    }
    /// O：`ev_height_changed` 0x140a83f80：高度变更 → `EditType` +
    /// `History::add()`（History 写入为 wp13 边界）。
    pub fn ev_height_changed(&mut self, _wall: WallId) -> Option<EditType> {
        Some(EditType::new_height_changed())
    }
}

impl EditType {
    /// O：`EditType::new_height_changed` 0x140a83fb0：构造墙高度变更编辑命令
    /// （判别 7；负载 `WallHistoryCompoundEdit`，Arc refcount
    /// `0x8000000000000002`）。
    pub fn new_height_changed() -> Self {
        EditType::Wall
    }
    /// 附加负载边界（U）：把复合编辑挂到 Wall 变体。
    pub fn attach_wall_compound(&mut self, _compound: WallHistoryCompoundEdit) {}
}
