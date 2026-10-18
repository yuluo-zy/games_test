//! trees.rs — 树组件族（组 A restorer 接线；B skeleton `components/` 的树/预制体
//! 组件，`components/registry.rs` 已登记名称串）。
//!
//! 消费方（O，BASELINE 参数面）：`forest::*`（handplaced/forest_tree_animation 的
//! `TreeAnimation`/`TreeMarkedForDeletion`）、`system_tree::*`（ui_delete_tree 的
//! `TreeComponent`、tree_move_bend 的 `TreeMoveBend`、convert_preplaced_trees 的
//! `ConvertedPreplacedTree`）。字段布局除登记项外 U（typed gap 或单元组件，
//! 不造数据）。

use bevy_ecs::component::Component;
use glam::Vec2;

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

/// 预制体实例数据组件（O：`systems::prefab::PrefabInstanceData`，**原生 0x58 B**）。
///
/// 跨文件统一（2026-10-17 ui-restorer-seams）：权威定义在
/// [`crate::systems::prefab::PrefabInstanceData`]（`+0x08` GPU Vec / `+0x20` RT Vec /
/// `+0x38` 音频 Vec / `+0x48` PrefabName）。此处只 re-export，避免同名不同体。
pub use crate::systems::prefab::PrefabInstanceData;

/// UI 2D 范围组件（O：`components::Ui2dExtent`，**原生 0x10 B**）。
///
/// 2026-10-17 fresh 反编译（0x142338990 `render_taffy_ui_to_widgets`）：
/// `+0x00` = `pos`（中心，Vec2）、`+0x08` = `size`（Vec2）；
/// 矩形 = `pos ± (size + expand) / 2`（`fVar25=((float)size.x+expand)*0.5`、
/// `rect_min=pos-half`、`rect_max=pos+half`）。
/// `color_button_animations` 0x1422979b0 以 `idx*0x10` 取元素，分别写
/// `+0x00/+0x08` 两个 Vec2（位置插值/尺寸缩放）。
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Ui2dExtent {
    /// +0x00：中心位置（UI 屏幕空间）。
    pub pos: Vec2,
    /// +0x08：尺寸。
    pub size: Vec2,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// 原生 0x10 B：pos@0x00、size@0x08（0x142338990 / 0x1422979b0）。
    #[test]
    fn ui2d_extent_matches_native_0x10() {
        assert_eq!(core::mem::size_of::<Ui2dExtent>(), 0x10);
        let e = Ui2dExtent::default();
        let base = &e as *const _ as usize;
        assert_eq!(&e.pos as *const _ as usize - base, 0x00);
        assert_eq!(&e.size as *const _ as usize - base, 0x08);
    }
}
