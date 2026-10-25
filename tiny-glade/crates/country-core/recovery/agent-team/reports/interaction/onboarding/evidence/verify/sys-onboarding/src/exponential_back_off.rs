//! exponential_back_off（O 0x142284800，1294）：提示重复触发的指数退避。
//!
//! 真逻辑（evidence/batch3-core.txt）：
//! - `Local<TimeSinceMouseMoved>` 累加 dt；`CursorMovedScreenSpace` 有事件 → 归零。
//! - 每帧 `GameProfile[0x1f0] += dt`（累加器）。
//! - 两对 (计时,冷却)：A=0x1e0/0x1e4、B=0x1e8/0x1ec；冷却缺省 720.0（0x44340000）。
//! - 读事件 reader → 对应计时归零、`newCooldown=clamp(old*1.5,720,10800)` 写回、
//!   写 WriteSettingsFileCmd。
//! - 对每标志（0x1f4/0x1f5）：`local<10.0` 才计时+=dt、`clamp[0,cooldown]`（cooldown<0
//!   走 f32::clamp panic）；`timer>=cooldown` → 写 WriteSettingsFileCmd + 清标志。
//!
//! **未实现**（分歧点）：GameProfile 0x1e0..0x1f5 未落库、WriteSettingsFileCmd 未落库、
//! reader B/C 事件类型未确认。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;

/// 自上次鼠标移动的时间（O：`Local<TimeSinceMouseMoved>`，f32 累加器）。
#[derive(Clone, Copy, Debug, Default)]
pub struct TimeSinceMouseMoved(pub f32);

/// exponential_back_off（O 0x142284800）：见模块头真逻辑。
/// **未实现**：GameProfile 提示区 + WriteSettingsFileCmd 未落库；骨架保留。
pub fn exponential_back_off(
    mut _local: Local<TimeSinceMouseMoved>,
    _mouse: EventReader<country_core::resources::window::CursorMovedScreenSpace>,
    _time: Res<GameTime>,
    _profile: ResMut<country_core::game_profile::GameProfile>,
) {
    // 未实现：见模块头分歧点。
}

/// inline helper（O 133）：冷却已过。语义 I。
pub fn has_cooldown_elapsed(_profile: &country_core::game_profile::GameProfile) -> bool {
    false
}

/// inline helper（O 137）：推进退避。语义 I。
pub fn advance_exp_backoff(_profile: &mut country_core::game_profile::GameProfile) {}
