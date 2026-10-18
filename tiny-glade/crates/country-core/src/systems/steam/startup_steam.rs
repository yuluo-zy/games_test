//! `systems/steam/startup_steam.rs` — Steam 启动系统（首帧一次性资源插入）。
//!
//! 原函数（B）：`country_core::systems::steam::startup_steam::startup_steam`
//! 0x140a9bbf0（业务 1；3 个 check_get_*_handle 为内联校验包装）。
//!
//! 行为（O，fresh 反编译 0x140a9bbf0）：
//! - 从 `Res<SteamClientResource>` 克隆客户端 Arc（门：无客户端则跳过）。
//! - `input.init()`（SteamAPI_ISteamInput_Init）。
//! - 构造 `SteamInput`（含 HashMap，`hashmap_random_keys` 初始化 RandomState）经
//!   `Commands::insert_resource` 插入。
//!
//! 原系统注册于 `PreStartup`（add_steam_integration 0x140945320），首帧一次性执行。

use bevy_ecs::prelude::*;
use std::collections::HashMap;

use crate::resources::steam::{SteamInput, SteamMenuActionSet, SteamMainActionSet};

/// startup_steam 0x140a9bbf0：Steam 客户端存在且输入初始化成功 → 插入 SteamInput 资源。
pub fn startup_steam(mut commands: Commands, client: Res<SteamClientResource>) {
    let Some(sdk) = client.client.clone() else {
        return;
    };
    if sdk.init_input() {
        commands.insert_resource(SteamInput {
            client: sdk,
            active_controller: 0,
            last_controller: 0,
            main_action_set: SteamMainActionSet::default(),
            menu_action_set: SteamMenuActionSet::default(),
            prev_digital: [0; 25],
            frame_actions: Vec::new(),
            released: 0,
            pressed: 0,
            actions_by_type: HashMap::new(),
        });
    }
}

// 复用 systems/steam::SteamClientResource（同模块域）。
use super::SteamClientResource;
