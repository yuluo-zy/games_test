//! gui.rs — UI 状态资源（WP12 facade）。
//!
//! O 名；布局 U（typed gap）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// Esc 菜单状态（O：`EscMenuState`）。
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EscMenuState {
    Closed,
    Open,
}

impl EscMenuState {
    pub fn is_closed(&self) -> bool {
        matches!(self, EscMenuState::Closed)
    }
    pub fn is_open(&self) -> bool {
        matches!(self, EscMenuState::Open)
    }
}

/// 调试 GUI（O：`DebugGui`；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DebugGui {
    pub raw: Gap,
}

impl DebugGui {
    pub fn enabled(&self) -> bool {
        false
    }
}
