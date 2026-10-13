//! mouse_capture.rs — 光标捕获命令（WP21）。
//!
//! O：`CursorCapturedPosition`（`new`/`is_currently_captured`；`impl$0::default`
//! 0xa86b30 折叠=0）；`mouse_capture`（0xb17830）系统；命令族
//! `CursorCaptureCmd`/`OsCursorCaptureCmd`/`OsCursorVisibilityCmd`/
//! `OsCursorVisibilityCmdOrigin`/`CustomCursorSsboCreatedCmd`/`MOUSE_CAPTURE`。

use bevy_ecs::event::Event;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `CursorCapturedPosition`（O：捕获位置记录）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CursorCapturedPosition {
    pub captured: bool,
    pub x: f32,
    pub y: f32,
}

impl CursorCapturedPosition {
    /// `new`（O 名，1 内联）。
    pub fn new(x: f32, y: f32) -> Self {
        Self { captured: true, x, y }
    }
    /// `is_currently_captured`（O 名，2 内联）。
    pub fn is_currently_captured(&self) -> bool {
        self.captured
    }
}

/// `CursorCaptureCmd`（O：捕获命令事件）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct CursorCaptureCmd(pub bool);

/// `OsCursorCaptureCmd`（O：OS 捕获命令）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct OsCursorCaptureCmd(pub Gap);

/// `OsCursorVisibilityCmd`（O：OS 光标可见命令）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct OsCursorVisibilityCmd(pub bool);

/// `OsCursorVisibilityCmdOrigin`（O：可见命令来源）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct OsCursorVisibilityCmdOrigin(pub Gap);

/// `CustomCursorSsboCreatedCmd`（O：自定义光标 SSBO 创建命令）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct CustomCursorSsboCreatedCmd(pub Gap);

/// `MOUSE_CAPTURE`（O：模块常量；U）。名称沿用骨架（非 camelCase）。
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MOUSE_CAPTURE(pub Gap);

/// `mouse_capture`（O 0xb17830，系统面）：读窗口状态，未捕获时发
/// SettingsMenuState / WindowFocusEvent（Bevy/winit 边界）。
pub fn mouse_capture(_captured: &mut CursorCapturedPosition, _state: &mut Gap) {
    // 系统级接线；骨架表面。
}
