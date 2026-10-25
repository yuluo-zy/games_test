//! commands.rs — 屋顶命令事件（WP12 系统向 facade）。

#![allow(dead_code, unused_imports)]

use crate::resources::walls::WallId;

/// 创建屋顶命令（O：`CreateRoofCmd`）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct CreateRoofCmd {
    pub wall: WallId,
}

/// 删除屋顶命令（O：`DeleteRoofCmd`）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct DeleteRoofCmd {
    pub wall: WallId,
}
