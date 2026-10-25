//! hint_deselect_tool（O 0x1422853c0，1116）。
//!
//! 真逻辑：GameProfile+0x1f4!=0 → 返回。读事件与 AppMode::is_tool_selected()、
//! DigitalActionState::pressed()：工具已选、数字动作按下、鼠标未按住
//! （`state@+9==1` 且 `[1]==0`）→ make_focus_or_camera_binding_hint；id =
//! (`*piVar27==6`)? 6+*(byte*)(piVar27+1) : 6，写 DisplayHintRequest。
//!
//! **未实现**：NotifyHintSystemToolWasJustUsed 之外的事件 + GameProfile 0x1f4 +
//! DisplayHintRequest 写面未落库。

use bevy_ecs::prelude::*;

/// 取消选择提示条件（O：hints_startup 初值 2B [0]=profile[0x1f4],[1]=0；deselect_tool
/// 置 state@+9）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DeselectToolHintConditions {
    pub hint_done: bool,
    pub event_byte: u8,
    pub state: u8,
}

/// hint_deselect_tool（O 0x1422853c0）：见模块头。
/// **未实现**：NotifyHintSystemToolWasJustUsed 载荷 + GameProfile 0x1f4 + AppMode/
/// DigitalActionState 写面依赖未落库。
pub fn hint_deselect_tool(
    _profile: Res<country_core::game_profile::GameProfile>,
    _state: ResMut<DeselectToolHintConditions>,
    _mode: Res<country_core::systems::mode_manager::AppMode>,
    _erased: EventReader<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,
) {
    // 未实现：见模块头。
}
