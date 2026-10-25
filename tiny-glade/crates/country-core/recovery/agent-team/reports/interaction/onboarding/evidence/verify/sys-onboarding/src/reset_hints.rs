//! reset_hints（O 0x141c1c8d0，838）：复位全部提示状态（新 glade/新会话）。
//!
//! 真逻辑（evidence/batch3-core.txt）：读事件（+8）。事件==0 → GameProfile
//! 0x1f4..0x206 整段置 1（0x0101010101010101），状态=2；否则 → 12 条件资源复位
//! （CameraHintProgress 阈值写 0.7/0.7/60=0x3f333333…/0x42700000，与 hints_startup
//! 相同）、GameProfile 0x1f4..0x206 清零、0x1e0/0x1e8 清零，状态=0；最后写
//! WriteSettingsFileCmd。常量：0x0101010101010101、0.7/0.7/60。
//!
//! **未实现**：触发事件来源 + GameProfile 0x1e0..0x206 + WriteSettingsFileCmd 未落库。

use bevy_ecs::prelude::*;

/// 提示复位状态（O：reset 后状态 2/0 语义 U）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintResetState {
    pub pending: bool,
}

/// reset_hints（O 0x141c1c8d0）：见模块头。
/// **未实现**：GameProfile 提示区 + WriteSettingsFileCmd 未落库；骨架保留。
pub fn reset_hints(
    _profile: ResMut<country_core::game_profile::GameProfile>,
    _state: ResMut<HintResetState>,
) {
    // 未实现：见模块头分歧点（触发事件来源 U + GameProfile 提示区未落库）。
}
