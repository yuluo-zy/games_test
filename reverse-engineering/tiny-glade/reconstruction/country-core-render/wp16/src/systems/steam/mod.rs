//! systems::steam — Steam 集成（WP16 恢复）。
//!
//! 源 SHA256（B skeleton）`systems/steam/{mod,startup_steam}.rs`。PDB 实体：
//! `AppSteamIntegration` / `SteamClientResource` / `SteamOverlayOpenResources` /
//! `PrevSteamInputState` / `SteamScreenshotRequest(s)`。
//!
//! steamworks crate 为被复用依赖（`crate::reused::steam`）。本模块恢复游戏侧
//! 包装：客户端初始化、overlay/截图请求、输入更新（把 Steam 动作映射到游戏
//! 输入资源）。

#![allow(dead_code)]

use bevy_ecs::resource::Resource;

use crate::reused::steam;
use crate::typed_gap::Gap;

pub mod startup_steam;

/// Steam 客户端资源（O：`Res<SteamClientResource>`；Arc<steamworks::Client>）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SteamClientResource {
    pub client: Option<steam::ClientManager>,
    pub _gap: Gap,
}

impl SteamClientResource {
    /// `show_floating_gamepad_text_input`（O 0x140ae9390）：
    /// `SteamAPI_ISteamUtils_ShowFloatingGamepadTextInput` + Dismissed 回调。
    pub fn show_floating_gamepad_text_input(&mut self, _mode: u32) {
        // U：回调注册细节（typed gap）。
    }
    /// `is_floating_gamepad_text_input_open`（O 0x140ae9470）。
    pub fn is_floating_gamepad_text_input_open(&self) -> bool {
        false
    }
    /// `input_show_binding_panel`（O 0x140ae9480）：
    /// `SteamAPI_ISteamInput_ShowBindingPanel`。
    pub fn input_show_binding_panel(&self, _action_set: u64) {}
}

/// 应用 Steam 集成（O：`AppSteamIntegration`；`try_new` 0x140ae94b0 /
/// `client` 0x67dd80 / `user_id` 0x140ae96f0）。
#[derive(Clone, Debug, Default)]
pub struct AppSteamIntegration {
    pub client: steam::ClientManager,
    pub user_id: u64,
}

impl AppSteamIntegration {
    /// `try_new`（O 0x140ae94b0）：创建 steamworks 客户端，取 user id。
    pub fn try_new() -> Option<Self> {
        None
    }
    pub fn client(&self) -> &steam::ClientManager {
        &self.client
    }
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

/// 上一帧 Steam 输入状态（O：`PrevSteamInputState` 资源）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PrevSteamInputState(pub Gap);

/// Overlay 打开资源（O：`SteamOverlayOpenResources`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SteamOverlayOpenResources(pub Gap);

impl SteamOverlayOpenResources {
    /// `can_use_overlay`（O 0x140aef1a0）。
    pub fn can_use_overlay(&self) -> bool {
        false
    }
    /// `open_browser_to_webpage<&str>`（O 0x14026d080）。
    pub fn open_browser_to_webpage(&self, _url: &str) {}
    /// `open_overlay_or_browser_to_webpage<&str>`（O 0x14026d3c0）。
    pub fn open_overlay_or_browser_to_webpage(&self, _url: &str) {}
    /// `open_overlay_or_browser_to_webpage<String>`（O 0x14026d580）。
    pub fn open_overlay_or_browser_to_webpage_string(&self, _url: alloc::string::String) {}
}

/// 截图请求（O：`SteamScreenshotRequest`）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct SteamScreenshotRequest(pub Gap);

/// 截图请求集合（O：`SteamScreenshotRequests`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SteamScreenshotRequests(pub Vec<SteamScreenshotRequest>);

/// `add_steam_integration`（O 0x140945320）：初始化 Steam 客户端、注册
/// screenshot/overlay 回调、插入输入动作集资源（`SteamMainActionSet` /
/// `SteamMenuActionSet`，O：startup_steam `try_load` 内联）。
pub fn add_steam_integration(_world: &mut bevy_ecs::world::World) {
    // U：动作集加载与回调细节（typed gap）。
}

/// `update_steam_inputs`（O 0x140ae9740）：每帧读 Steam 数字/模拟动作并写入
/// 游戏输入资源（PrevSteamInputState）。
pub fn update_steam_inputs(_world: &mut bevy_ecs::world::World) {}

/// 主动作集（O：`SteamMainActionSet`；`try_load<steamworks::Input>` 内联）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SteamMainActionSet(pub Gap);

/// 菜单动作集（O：`SteamMenuActionSet`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SteamMenuActionSet(pub Gap);
