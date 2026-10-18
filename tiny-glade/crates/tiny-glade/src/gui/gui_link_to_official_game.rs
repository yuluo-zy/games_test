//! `tiny_glade::gui::gui_link_to_official_game` — link to the official game.
//!
//! fresh Ghidra shared 2026-10-17：`display_link_to_official_game`
//! `0x1403d0c70`（757 B）。结构：线性插值推进布局（多组 `min/max` 减法后写
//! 回一个 `*mut u64`、变化时置 `*bool=1`）→ `MeiContext::reuse_or_create_node`
//! （id = `as_id() ^ 0xb1695320004fb443`，写节点 +0x5d/+0xb7 标志）→ 3 次
//! `child_with_global_id`（`calculate_hash<(u64, TypeId)>` 与
//! `calculate_hash<(u64,u64)>`）→ `TemplateView::template()` 与 2 次
//! `with_slot<closure_env$0/$1, &str>`。链接目标、标签、动画常量未取证。

/// B `display_link_to_official_game` (`0x1403d0c70`) — 未实现-需重做.
pub fn display_link_to_official_game() {
    todo!(
        "未实现-需重做: display_link_to_official_game 0x1403d0c70 —— MEI 模板常量/\
         链接目标未还原"
    );
}
