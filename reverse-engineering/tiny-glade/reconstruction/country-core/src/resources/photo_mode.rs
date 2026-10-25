//! photo_mode.rs — 拍照模式（WP21）。
//!
//! O：`PhotoModeCameraMode`（判别枚举；`next` 0xb59720、`next_demo` 0xb596b0 mode 3
//! panic）、`PhotoModeCmd` 事件、`PhotoModePlugin`（impl$0::build 0xb59640
//! register_event<PhotoModeCmd>）。

use bevy_ecs::event::Event;

use crate::typed_gap::Gap;

/// `PhotoModeCameraMode`（O：相机模式枚举）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhotoModeCameraMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
    Other(u8),
}

impl Default for PhotoModeCameraMode {
    fn default() -> Self {
        PhotoModeCameraMode::Mode0
    }
}

impl PhotoModeCameraMode {
    /// `next`（O 0xb59720）：循环下一模式。
    pub fn next(self) -> Self {
        match self {
            PhotoModeCameraMode::Mode0 => PhotoModeCameraMode::Mode1,
            PhotoModeCameraMode::Mode1 => PhotoModeCameraMode::Mode2,
            PhotoModeCameraMode::Mode2 => PhotoModeCameraMode::Mode3,
            PhotoModeCameraMode::Mode3 => PhotoModeCameraMode::Mode0,
            PhotoModeCameraMode::Other(_) => PhotoModeCameraMode::Mode0,
        }
    }
    /// `next_demo`（O 0xb596b0：mode 3 panic）。
    pub fn next_demo(self) -> Self {
        match self {
            PhotoModeCameraMode::Mode3 => panic!("next_demo"),
            m => m.next(),
        }
    }
}

/// `PhotoModeCmd`（O：命令事件）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct PhotoModeCmd(pub Gap);

/// `PhotoModePlugin`（O：Bevy 插件）。
#[derive(Clone, Copy, Debug, Default)]
pub struct PhotoModePlugin;
