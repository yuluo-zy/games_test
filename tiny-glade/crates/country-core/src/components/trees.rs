//! trees.rs — 树组件族（组 A restorer 接线；B skeleton `components/` 的树/预制体
//! 组件，`components/registry.rs` 已登记名称串）。
//!
//! 消费方（O，BASELINE 参数面）：`forest::*`（handplaced/forest_tree_animation 的
//! `TreeAnimation`/`TreeMarkedForDeletion`）、`system_tree::*`（ui_delete_tree 的
//! `TreeComponent`、tree_move_bend 的 `TreeMoveBend`、convert_preplaced_trees 的
//! `ConvertedPreplacedTree`）。字段布局除登记项外 U（typed gap 或单元组件，
//! 不造数据）。

use bevy_ecs::component::Component;
use glam::Vec3;

/// 树删除标记（O：`components::TreeMarkedForDeletion` 名称；`system_tree`
/// despawn_trees 以 `Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>`
/// 遍历回收）。单元组件，载荷 U。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TreeMarkedForDeletion;

/// 树动画组件（O：`components::TreeAnimation`；`forest::tree_animation`
/// 0x142279500/0x14227a0d0 驱动。字段布局 U（sway 相位/速率族）——
/// `alpha` 消退进度为 `despawn_trees` 的 α≤0.01 门控读取面（组 A 修复，I）。
#[derive(Component, Clone, Debug, Default)]
pub struct TreeAnimation {
    /// 消退进度 α（I：0x142960674 = 0.01 阈值的读取面；字段偏移 U）。
    pub alpha: f32,
    /// typed gap：动画相位/速率字段（U）。
    pub _gap: u64,
}

/// 树移动弯曲组件（O：`components::TreeMoveBend`；`system_tree::tree_move_bend`
/// 0x1422689c0 驱动。字段布局 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TreeMoveBend {
    /// typed gap。
    pub _gap: u64,
}

/// 树实体标记（O：`components::TreeComponent`；ui_delete_tree 的
/// `Query<(&PrefabInstanceData, &PrefabInstance, &TreeComponent)>` 面）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TreeComponent;

/// 已转换的预置树标记（O：`components::ConvertedPreplacedTree`；
/// convert_preplaced_trees 的 `Query<(&PrefabInstanceData, &ConvertedPreplacedTree)>`）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConvertedPreplacedTree;

/// 树 canopy 最大剔除距离（O：`tree_edit::TreeCanopyMaxCullDistance`——权威定义在
/// `resources/history/tree_edit.rs`，见该处）。
pub use crate::resources::history::tree_edit::TreeCanopyMaxCullDistance;

/// 预制体实例数据组件（O：`systems::prefab::PrefabInstanceData`；
/// forest/tree-tool 各系统以 `&mut PrefabInstanceData` 改位置/缩放。
/// 字段 = {位置, 缩放, 旋转, 预制体名…}，语义 I，布局 U）。
#[derive(Component, Clone, Debug, Default)]
pub struct PrefabInstanceData {
    /// 世界位置（I：字段语义；O 面为 0x58B 树命令/实例记录的载荷族）。
    pub position: Vec3,
    /// 缩放（I）。
    pub scale: f32,
    /// 度制旋转（I）。
    pub rotation_degrees: f32,
    /// typed gap：其余实例字段（U）。
    pub _gap: u64,
}

/// UI 2D 范围组件（O：`components::Ui2dExtent`；color_button_animations 的
/// `&mut Ui2dExtent` 面——按钮缩放动画目标。字段 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Ui2dExtent {
    /// typed gap。
    pub _gap: u64,
}

/// UI 2D 图像组件（O：`components::Ui2dImage`；color_menu 查询面）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Ui2dImage {
    /// typed gap。
    pub _gap: u64,
}

/// UI 绘制序组件（O：`components::UiDrawOrder`；color_button_animations 的
/// `&mut UiDrawOrder` 面）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiDrawOrder(pub u32);
