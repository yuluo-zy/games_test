//! hint_clutter_focus_edit（O：condition 0x141c3d890 / hint 0x141c3db80 /
//! make_focus_or_camera_binding_hint 0x141c3de00）。
//!
//! `hint_clutter_focused_edit_condition`：GameProfile+0x200!=0 → 状态置 1 返回；工具
//! ==3 → 置状态、写完成事件 tag 9；焦点实体工具字节==9 写事件 tag 0x13。
//! `hint_clutter_focused_edit_hint`：Query + RaycastWorld::raycast，geo_src_to_edit_subject
//! _w_deco_clutter 结果==3 → make_focus_or_camera_binding_hint 写 DisplayHintRequest
//! （flags 0x100，id 3）。
//! `make_focus_or_camera_binding_hint`：profile 0x1c4 字形模式 → alternate([Graphic
//! ("hint/mouse"),Graphic("hint/rmb")]) / [mouse,mmb] / alternate(MouseButton)。
//!
//! **未实现**：GameProfile 0x1c4/0x200 + NotifyHintSystem* + geo_src 链 + SteamInput
//! 未落库 → 标未实现。

use bevy_ecs::prelude::*;

/// 杂物焦点提示条件（O：hints_startup 初值 2B [0]=profile[0x200],[1]=0）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct FocusedClutterHintConditions {
    pub hint_done: bool,
    pub state: u8,
}

/// make_focus_or_camera_binding_hint（O 0x141c3de00）：profile 0x1c4 字形模式组合。
/// **未实现**：profile 0x1c4 + SteamInput + lang 串未落库。
pub fn make_focus_or_camera_binding_hint(
    _profile: Res<country_core::game_profile::GameProfile>,
) -> crate::BindingHintDisplay {
    crate::BindingHintDisplay::Empty
}

/// hint_clutter_focused_edit_condition（O 0x141c3d890）：见模块头。
/// **未实现**：NotifyHintSystem* + ModeJustChanged 载荷未落库。
pub fn hint_clutter_focused_edit_condition(
    _profile: Res<country_core::game_profile::GameProfile>,
    _state: ResMut<FocusedClutterHintConditions>,
    _mode: EventReader<country_core::resources::events::ModeJustChanged>,
) -> bool {
    false
}

/// hint_clutter_focused_edit_hint（O 0x141c3db80）：见模块头。
/// **未实现**：RaycastWorld::raycast 返回 + geo_src_to_edit_subject_w_deco_clutter
/// 跳转表未落库。
pub fn hint_clutter_focused_edit_hint(
    _state: ResMut<FocusedClutterHintConditions>,
    _raycast: Res<country_core::systems::collision::world::RaycastWorld>,
    _query: Query<&country_core::resources::roofs::roof_shape::Roof>,
) {
    // 未实现：见模块头。
}
