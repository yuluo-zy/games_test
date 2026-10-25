//! startup_tree_tool.rs — 树工具重置（B 行 4，PDB inlinee；`reset_tree_tool`，
//! O 名 + 逻辑并入调用点）。
//!
//! 注册（O）：`add_systems<OnExit<AppLoadingState>, fn(ResMut<TreeStorage>)>`——
//! 装载状态退出时重置 TreeStorage。真实逻辑（O）：`*tree_storage =
//! TreeStorage::default()` = 清空 hashbrown map（+0x0/+0x10 头清零）+ 写随机
//! KEYS（+0x20）+ next_index=0（+0x30），随后置变更 tick（TreeStorage 布局
//! 0x40；0x143034a10/0x143034a20 清零常量）。

use bevy_ecs::prelude::*;

use country_core::resources::tree_storage::TreeStorage;

/// `reset_tree_tool`（O：装载退出时重置树工具状态——`ResMut<TreeStorage>`
/// 覆写为 default，含 next_index 归零；`clear()` 不清 next_index，与真实
/// default 覆写不符，故用 `*storage = default()`）。
pub fn reset_tree_tool(mut storage: ResMut<TreeStorage>) {
    *storage = TreeStorage::default();
}
