//! hint_unlink_windows（O 0x14227d760，707）。
//!
//! 真逻辑：GameProfile+0x1fd!=0 → 返回。事件字节!=0 → 写 TutorialHintComplete（tag 8）；
//! ==0 → alternate(Option<KeyCode>) 写 DisplayHintRequest（id 6，flags 0x800）。
//!
//! **未实现**：NotifyHintSystemWindow + GameProfile 0x1fd + DisplayHintRequest 写面未落库。

use bevy_ecs::prelude::*;

/// hint_unlink_windows（O 0x14227d760）：见模块头。
/// **未实现**：NotifyHintSystemWindow + GameProfile 0x1fd 未落库。
pub fn hint_unlink_windows(
    _profile: Res<country_core::game_profile::GameProfile>,
) {
    // 未实现：见模块头。
}
