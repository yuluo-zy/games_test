//! hint_terrain_steepness（O 0x142283640，754）。
//!
//! 真逻辑：工具==10 且 GameProfile+0x1fa==0 且事件字节==1 且 [+1]==0 且存在实体 →
//! make_lbm_drag_binding_hint 写 DisplayHintRequest（id 6，flags 0x400）；事件字节!=0
//! 且 [+1] → 写 TutorialHintComplete（tag 4）。

use bevy_ecs::prelude::*;

/// 地形陡峭提示条件（O：hints_startup 初值 2B [0]=profile[0x1fa],[1]=profile[0x1fa]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintTerrainSteepnessConditions {
    pub hint_done: bool,
    pub event_byte: u8,
}

/// hint_terrain_steepness（O 0x142283640）：见模块头。
/// **未实现**：NotifyHintSystemTerrainSteepness + GameProfile 0x1fa 未落库。
pub fn hint_terrain_steepness(
    _profile: Res<country_core::game_profile::GameProfile>,
    _state: ResMut<HintTerrainSteepnessConditions>,
    _mode: Res<country_core::systems::mode_manager::AppMode>,
) {
    // 未实现：见模块头。
}
