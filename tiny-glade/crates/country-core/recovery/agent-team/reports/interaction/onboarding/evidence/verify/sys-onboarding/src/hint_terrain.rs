//! hint_terrain（O 0x142283cc0，844）。
//!
//! 真逻辑：工具==10（Terrain）且 GameProfile+0x1f9==0 且事件计数>1 且存在实体且事件
//! 字节[+1]==0 → alternate(Option<KeyCode>) 写 DisplayHintRequest（id 6，flags 0x600）；
//! 事件字节==1 且 [+1]==0 → 写 TutorialHintComplete（tag 6）。

use bevy_ecs::prelude::*;

/// 地形分层提示条件（O：hints_startup 初值 10B [0..7]=0,[8]=profile[0x1f9]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintTerrainConditions {
    pub hint_done: bool,
    pub event_byte: u8,
}

/// hint_terrain_layering（O 0x142283cc0）：见模块头。
/// **未实现**：NotifyHintSystemTerrain + GameProfile 0x1f9 + DisplayHintRequest 写面未落库。
pub fn hint_terrain_layering(
    _profile: Res<country_core::game_profile::GameProfile>,
    _state: ResMut<HintTerrainConditions>,
    _mode: Res<country_core::systems::mode_manager::AppMode>,
    _terrain: EventReader<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,
) {
    // 未实现：见模块头。
}
