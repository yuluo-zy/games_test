//! hint_terrain_focus（O 0x142283980，738）。
//!
//! 真逻辑：工具==10 且 GameProfile+0x1fb==0 且存在实体且事件字节==0 →
//! make_focus_or_camera_binding_hint 写 DisplayHintRequest（id 6，flags 0x100）；
//! 事件字节==2 → 写 TutorialHintComplete（tag 5）。

use bevy_ecs::prelude::*;

/// 地形焦点提示条件（O：hints_startup 初值 1B [0]=profile[0x1fb]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintFocusTerrainConditions {
    pub hint_done: bool,
}

/// hint_terrain_focus（O 0x142283980）：见模块头。
/// **未实现**：NotifyHintSystemTerrain + GameProfile 0x1fb 未落库。
pub fn hint_terrain_focus(
    _profile: Res<country_core::game_profile::GameProfile>,
    _state: ResMut<HintFocusTerrainConditions>,
    _mode: Res<country_core::systems::mode_manager::AppMode>,
) {
    // 未实现：见模块头。
}
