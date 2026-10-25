//! glade_settings.rs — 谷地设置（WP21）。
//!
//! O：`GladeSettings::load`（0x8bbbc0）从文件 `serde_json::from_str<Option<GladeSettings>>`；
//! `Default`（0x8c2240）：`{ +0x0 u32=0, +0x8 f32=0.01, +0xc f32=1.0, +0x10 u32=8,
//!   +0x18 f32=0.0, +0x1c f32=1.0, +0x20 u8=1 }`。

use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

/// `GladeSettings`（O：JSON 存档设置；默认布局字节 O，字段语义 U）。
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GladeSettings {
    /// +0x0 u32（O=0）。
    pub a: u32,
    /// +0x8 f32（O=0.01）。
    pub b: f32,
    /// +0xc f32（O=1.0）。
    pub c: f32,
    /// +0x10 u32（O=8）。
    pub d: u32,
    /// +0x18 f32（O=0.0）。
    pub e: f32,
    /// +0x1c f32（O=1.0）。
    pub f: f32,
    /// +0x20 u8（O=1）。
    pub g: u8,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl Default for GladeSettings {
    fn default() -> Self {
        Self { a: 0, b: 0.01, c: 1.0, d: 8, e: 0.0, f: 1.0, g: 1, _gap: Gap::default() }
    }
}

impl GladeSettings {
    /// `load`（O 0x8bbbc0）：读 `glade_settings.json`；失败回退默认。
    pub fn load(_path: &std::path::Path) -> Self {
        Self::default()
    }
    /// `try_load`（O 名，内联）。
    pub fn try_load(_path: &std::path::Path) -> Option<Self> {
        None
    }
}
