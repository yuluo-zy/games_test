//! prefab.rs — `country_core::resources::prefab` 预制体引用组件（组 A restorer
//! 接线；BASELINE 参数面 `resources::prefab::PrefabInstance`——ui_delete_tree /
//! ui_move_tree 的 `Query<(&PrefabInstanceData, &PrefabInstance)>` 面）。

use bevy_ecs::component::Component;

/// 预制体实例引用（O：`resources::prefab::PrefabInstance`；字段 = {实体/句柄/
/// 名}，语义 I，布局 U）。
#[derive(Component, Clone, Debug, Default)]
pub struct PrefabInstance {
    /// 预制体名（I）。
    pub name: Option<std::sync::Arc<std::string::String>>,
    /// typed gap。
    pub _gap: u64,
}
