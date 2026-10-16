//! startup_tree_tool.rs — 树工具重置（B 行 4，PDB inlinee；O 名——`reset_tree_tool`，
//! 逻辑并入调用点，本处按 BASELINE 参数面落地）。

use bevy_ecs::prelude::*;

use country_core::resources::tree_storage::TreeStorage;

/// `reset_tree_tool`（O 名：重置树工具状态——`ResMut<TreeStorage>` 清空面）。
pub fn reset_tree_tool(mut storage: ResMut<TreeStorage>) {
    // O 面：工具重置（TreeStorage 清空语义；条目保留策略 I）。
    storage.clear();
}
