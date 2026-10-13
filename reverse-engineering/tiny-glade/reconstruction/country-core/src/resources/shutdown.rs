//! shutdown.rs — 游戏关闭（WP21）。
//!
//! O：`ShutdownGame` 命令资源；`initiate_game_shutdown`（0xb2ba40）读事件、触发时
//! `Commands::insert_resource(ShutdownGame)`。

use bevy_ecs::resource::Resource;

/// `ShutdownGame`（O：关闭命令资源）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ShutdownGame;

/// `initiate_game_shutdown`（O 0xb2ba40）：有触发则请求关闭。
pub fn initiate_game_shutdown(_events: &mut Vec<ShutdownGame>, _commands: &mut Vec<ShutdownGame>) {
    let _ = (_events, _commands);
}
