//! hint_disable_snapping（O 0x14227e140，661）。
//!
//! 真逻辑：事件字节[+8]==0 且 GameProfile+0x1fc==0 且某输入状态 0 且工具==6
//! → alternate(Option<KeyCode>)（键来自 GameProfile+0x120/+0x128）写 DisplayHintRequest
//! （id=bVar2+6）；事件字节!=0 且某条件 → 写 TutorialHintComplete（tag 3）。
//!
//! **未实现**：NotifyHintSystemSnapping + GameProfile 0x1fc/键槽 + DisplayHintRequest
//! 写面未落库。

use bevy_ecs::prelude::*;

/// 吸附禁用提示条件（O：hints_startup 初值 1B [0]=profile[0x1fc]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintDisableSnappingConditions {
    pub hint_done: bool,
}

/// hint_disable_snapping（O 0x14227e140）：见模块头。
/// **未实现**：NotifyHintSystemSnapping + GameProfile 键槽未落库。
pub fn hint_disable_snapping(
    _profile: Res<country_core::game_profile::GameProfile>,
    _state: ResMut<HintDisableSnappingConditions>,
    _mode: Res<country_core::systems::mode_manager::AppMode>,
) {
    // 未实现：见模块头。
}
