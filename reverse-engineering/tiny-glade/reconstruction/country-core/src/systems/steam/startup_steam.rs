//! startup_steam.rs — Steam 启动（O 0x140a9bbf0）。
//!
//! 源 `systems/steam/startup_steam.rs`。启动系统：初始化 Steam 客户端并加载
//! 输入动作集句柄（`check_get_{action_set,digital_action,analog_action}_handle`
//! 内联于 `SteamMainActionSet/SteamMenuActionSet::try_load`）。

#![allow(dead_code)]

/// `startup_steam`（O 0x140a9bbf0）。
pub fn startup_steam(_world: &mut bevy_ecs::world::World) {
    // U：steamworks Client 初始化细节（typed gap）。
}

/// 动作集句柄检查（O：`impl$0::check_get_action_set_handle` 内联）。
pub fn check_get_action_set_handle(_input: &crate::reused::steam::Input) -> Option<crate::reused::steam::ActionSetHandle> {
    None
}

/// 数字动作句柄检查（O）。
pub fn check_get_digital_action_handle(_input: &crate::reused::steam::Input) -> Option<crate::reused::steam::DigitalActionHandle> {
    None
}

/// 模拟动作句柄检查（O）。
pub fn check_get_analog_action_handle(_input: &crate::reused::steam::Input) -> Option<crate::reused::steam::AnalogActionHandle> {
    None
}
