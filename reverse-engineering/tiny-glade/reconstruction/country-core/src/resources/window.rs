//! window.rs — 窗口/光标资源（WP12 facade）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

/// 窗口尺寸（O：`WindowSize`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

/// 屏幕空间光标移动（O：`CursorMovedScreenSpace`）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct CursorMovedScreenSpace {
    pub pos: glam::Vec2,
}
