//! texture_presets.rs — 纹理预设（WP21）。
//!
//! O：`TexturePreset`（枚举）；`impl$1::from`（0xa59890）按变体建 rhapsody
//! `TextureDesc::new_2d/sampled/storage/color_attachment/depth_stencil_attachment` +
//! `mip_levels`（变体 8），带 20 个资源 hash 常量。`TextureFromDisk` 磁盘纹理事件。
//! rhapsody = 复用边界。

use bevy_ecs::event::Event;

use crate::typed_gap::Gap;

/// `TexturePreset`（O：纹理预设枚举；From<u8> 建 TextureDesc）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TexturePreset {
    Preset0,
    Preset1,
    Preset2,
    Preset3,
    Preset4,
    Preset5,
    Preset6,
    Preset7,
    Preset8,
    Preset9,
    Preset10,
    Preset11,
    Preset12,
    Preset13,
    Preset14,
    Preset15,
    Preset16,
    Preset17,
    Preset18,
    Other(u8),
}

impl Default for TexturePreset {
    fn default() -> Self {
        TexturePreset::Preset0
    }
}

impl From<u8> for TexturePreset {
    /// `From<u8>`（O 0xa59890）。
    fn from(v: u8) -> Self {
        match v {
            0 => TexturePreset::Preset0,
            1 => TexturePreset::Preset1,
            2 => TexturePreset::Preset2,
            3 => TexturePreset::Preset3,
            4 => TexturePreset::Preset4,
            5 => TexturePreset::Preset5,
            6 => TexturePreset::Preset6,
            7 => TexturePreset::Preset7,
            8 => TexturePreset::Preset8,
            9 => TexturePreset::Preset9,
            10 => TexturePreset::Preset10,
            11 => TexturePreset::Preset11,
            12 => TexturePreset::Preset12,
            13 => TexturePreset::Preset13,
            14 => TexturePreset::Preset14,
            15 => TexturePreset::Preset15,
            16 => TexturePreset::Preset16,
            17 => TexturePreset::Preset17,
            18 => TexturePreset::Preset18,
            _ => TexturePreset::Other(v),
        }
    }
}

impl TexturePreset {
    /// O：预设是否 mip（变体 8）/depth（10/11/12）/color attachment（15/16）。
    pub fn usage(&self) -> (bool, bool, bool) {
        let (mips, depth, color) = match self {
            TexturePreset::Preset8 => (true, false, false),
            TexturePreset::Preset10 | TexturePreset::Preset11 | TexturePreset::Preset12 => {
                (false, true, false)
            }
            TexturePreset::Preset15 | TexturePreset::Preset16 => (false, false, true),
            _ => (false, false, false),
        };
        (mips, depth, color)
    }
}

/// `TextureFromDisk`（O：磁盘纹理事件；U）。
#[derive(Event, Clone, Debug, Default)]
pub struct TextureFromDisk(pub Gap);
