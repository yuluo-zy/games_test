//! `decorator_resources.rs` — 装饰系统共享小资源/工具（原文件仅
//! `to_widget_name_hash` 行 36，PDB 模块 1499）。
//!
//! `to_widget_name_hash`（内联，行 36；宿主 = decorator_on_wall_height_changed
//! 行 36 同址）：UI 控件名 → u64 哈希。算法 I：对照 utils `NameHash`
//! （xxh3 直通）族；具体哈希未定点（U 登记——调用面为 widget 名注册）。
//!
//! 注意（SOURCE_MAP 分歧登记）：systems SOURCE_MAP 把
//! entities/add_decorator_visual_entity/seal_persistent_entities 归入本文件；
//! skeleton PDB 模块 1499/3068 归属 `decorator_visual_state.rs`（行号一致）。
//! 本批次按 PDB 模块归属（visual 状态在 decorator_visual_state.rs），
//! 此处保留 to_widget_name_hash + 无歧义资源。

use utils::name_hash::NameHash;

/// `to_widget_name_hash`（行 36 内联；算法 I，U 登记——xxh3 族候选）。
pub fn to_widget_name_hash(name: &str) -> u64 {
    NameHash::compute(name).0
}
