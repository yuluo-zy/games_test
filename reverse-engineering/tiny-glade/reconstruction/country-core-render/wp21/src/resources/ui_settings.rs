//! ui_settings.rs — UI 设置（WP21）。
//!
//! O：`ToolbarAlignment`（is_horizontal 0xb0e070、fmt 0xb0e080 lang_lib 本地化名）、
//! `UiSettings` Default（0xb12f10）：`{ +0x0 f32=1.0, +0x4 f32=24.0, +0x8 u32=0x00000101,
//! +0xc u8=0 }`。

use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

/// `ToolbarAlignment`（O：工具栏对齐枚举）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolbarAlignment {
    Horizontal,
    Vertical,
    Bottom,
    Side,
}

impl Default for ToolbarAlignment {
    fn default() -> Self {
        ToolbarAlignment::Horizontal
    }
}

impl ToolbarAlignment {
    /// `is_horizontal`（O 0xb0e070）。
    pub fn is_horizontal(&self) -> bool {
        matches!(self, ToolbarAlignment::Horizontal)
    }
    /// `fmt`（O 0xb0e080：lang_lib 本地化名，U 内容）。
    pub fn name(&self) -> &'static str {
        match self {
            ToolbarAlignment::Horizontal => "Horizontal",
            ToolbarAlignment::Vertical => "Vertical",
            ToolbarAlignment::Bottom => "Bottom",
            ToolbarAlignment::Side => "Side",
        }
    }
}

/// `UiSettings`（O：UI 设置资源）。
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct UiSettings {
    /// +0x0 f32=1.0（O：工具栏缩放，I）。
    pub scale: f32,
    /// +0x4 f32=24.0（O：间距，I）。
    pub spacing: f32,
    /// +0x8 u32=0x00000101（O）。
    pub a: u32,
    /// +0xc u8=0（O）。
    pub b: u8,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self { scale: 1.0, spacing: 24.0, a: 0x00000101, b: 0, _gap: Gap::default() }
    }
}
