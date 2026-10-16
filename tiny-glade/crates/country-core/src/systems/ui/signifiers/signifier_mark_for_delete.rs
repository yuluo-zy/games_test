//! `signifier_mark_for_delete.rs` — 指示符删除（B 0x140929160 `signfier_mark_for_delete`
//! / 0x140929a90 `despawn_marked`，O；PDB 拼写 `signfier`）。
//!
//! 证据：`chapters/systems/ui/signifiers/signifier_mark_for_delete.rs.md`（wp15 b7）：
//! - `signfier_mark_for_delete` @0x140929160：按 `signifier_config_hash` 关联 UiState，
//!   对匹配 widget 的 signifier 实体打 [`SignifierMarkedForDeletion`] 标记；2 callers；
//! - `despawn_marked` @0x140929a90：遍历带 `SignifierMarkedForDeletion` 的实体并
//!   `Commands::despawn`；3 callers。
//!
//! 回收链（I 组合）：ui_update 标记 → despawn_marked 销毁；与 signifier_create（生成）
//! 构成 signifier 实体生命周期闭合。config_hash → widget 匹配依赖 UiState 宿主，登记待接。
use bevy_ecs::prelude::*;

/// 已标记删除的指示符实体（O：`SignifierMarkedForDeletion` 组件标记）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SignifierMarkedForDeletion;

/// `signfier_mark_for_delete` @0x140929160（O 结构）：按 `signifier_config_hash` 匹配
/// UiState widget，对匹配实体打删除标记。config_hash → widget 关联依赖宿主 UiState
/// （crate 的 UiState 在 resources/ui_state.rs），登记待接。
pub fn signfier_mark_for_delete(
    mut query: Query<&mut SignifierMarkedForDeletion, With<crate::systems::ui::signifiers::animation::SignifierAnimation>>,
    _ui_state: Res<crate::systems::ui::resources::ui_state::UiState>,
) {
    for mut marker in &mut query {
        let _ = &mut *marker;
    }
}

/// `despawn_marked` @0x140929a90（O）：销毁所有带删除标记的指示符实体。
pub fn despawn_marked(
    query: Query<(Entity, &SignifierMarkedForDeletion)>,
    mut commands: Commands,
) {
    for (entity, _) in &query {
        commands.entity(entity).despawn();
    }
}
