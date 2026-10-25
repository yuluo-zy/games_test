//! hint_undo（O 0x142284220，1280）。
//!
//! 真逻辑：DigitalActionState::just_pressed() → 写事件（tag 2）。读事件/工具
//! （cVar7）：==2 → timer+=dt；timer>10.0 → 写事件（tag 2）；再判 just_pressed →
//! 写事件。末尾：GameProfile+4==1 且 +0x1fe!=1 → adjacent(steam,adjacent(Option<KeyCode>))
//! → alternate 写 DisplayHintRequest（id 6，含 0xc9）。常量：+0xfc/+0x104 键码、阈值
//! 10.0、tag 2、标志 0x1fe。
//!
//! **未实现**：NotifyHintSystemSomethingWasErased + GameProfile 0x1fe + SteamInput +
//! 键码来源未落库。

use bevy_ecs::prelude::*;

/// 撤销提示条件（O：hints_startup 初值 8B 清零：u32@+8=0，u8@+0xc=0）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct UndoHintConditions {
    pub timer: f32,
    pub flag: u8,
}

/// hint_undo（O 0x142284220）：见模块头。
/// **未实现**：NotifyHintSystemSomethingWasErased + GameProfile 0x1fe + SteamInput 未落库。
pub fn hint_undo(
    _profile: Res<country_core::game_profile::GameProfile>,
    _state: ResMut<UndoHintConditions>,
    _erased: EventReader<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,
) {
    // 未实现：见模块头。
}
