//! hint_focused_edit（O：condition 0x14227da70 / hint 0x14227dd60）。
//!
//! `hint_focused_edit_condition`：GameProfile+0x1f5!=0 → 状态置 1 返回；否则读工具变化
//! 事件（ModeJustChanged），工具∈{1,2}（`*piVar5-1U<2`）→ 置状态、写完成事件 tag 1；
//! 焦点实体工具字节==1 → 写事件 tag 0x13。
//! `hint_focused_edit_hint`：count++（`4<count` 置标志）；Query<(Roof,RoofTriggerVolume)>
//! + RaycastWorld::legacy_raycast（命中且 !=2）+ geo_src_to_edit_subject → 跳转表
//! DAT_1430386a0 生成提示。
//!
//! **未实现**：NotifyHintSystem* / ModeJustChanged 载荷、geo_src_to_edit_subject 跳转表
//! 未落库 → 标未实现。

use bevy_ecs::prelude::*;

/// 焦点编辑提示条件（O：hints_startup 初值 10B [0..7]=0,[8]=profile[0x1f5],[9]=0；
/// condition 置 state@+8）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct FocusedEditHintConditions {
    pub hint_done: bool,
    pub state: u8,
}

/// hint_focused_edit_condition（O 0x14227da70）：见模块头。
/// **未实现**：ModeJustChanged 载荷 + geo_src 链未落库。
pub fn hint_focused_edit_condition(
    _profile: Res<country_core::game_profile::GameProfile>,
    _state: ResMut<FocusedEditHintConditions>,
    _mode: EventReader<country_core::resources::events::ModeJustChanged>,
) -> bool {
    false
}

/// hint_focused_edit_hint（O 0x14227dd60）：见模块头。
/// **未实现**：RaycastWorld::legacy_raycast 返回 + geo_src_to_edit_subject 跳转表未落库。
pub fn hint_focused_edit_hint(
    _state: ResMut<FocusedEditHintConditions>,
    _raycast: Res<country_core::systems::collision::world::RaycastWorld>,
    _query: Query<&country_core::resources::roofs::roof_shape::Roof>,
) {
    // 未实现：见模块头。
}
