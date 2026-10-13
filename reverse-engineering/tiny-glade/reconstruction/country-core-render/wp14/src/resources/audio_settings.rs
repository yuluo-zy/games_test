//! resources/audio_settings.rs — 音频设置（GameConfig 内嵌，RON serde）。
//!
//! PDB 骨架：`impl$0::serialize/deserialize`、`impl$0::clone`、`impl$1::default`（O）。
//! 反汇编（audio_update_system 0x1409c8dd0）给出音量字段偏移（O）：`master@+0xf8`、
//! `music@+0xfc`（Music 与 Music-DawnJingle 两个 VCA 共用）、`sfx@+0x100`、`amb@+0x104`；
//! `@0..0xf8` 的其余字段 U（typed gap）。

#![allow(dead_code, unused_imports)]

use serde::{Deserialize, Serialize};

use bevy_ecs::resource::Resource;
use crate::typed_gap::Gap;

/// 音频设置（O：serde RON 结构；音量字段偏移 O）。
#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default)]
pub struct AudioSettings {
    /// @0..0xf8 未恢复字段（U）。
    pub _prefix: Gap,
    /// master_volume @+0xf8（O；`vca:/Master`）。
    pub master_volume: f32,
    /// music_volume @+0xfc（O；`vca:/Music` + `vca:/Music-DawnJingle`）。
    pub music_volume: f32,
    /// sfx_volume @+0x100（O；`vca:/SFX`）。
    pub sfx_volume: f32,
    /// ambience_volume @+0x104（O；`vca:/Amb`）。
    pub ambience_volume: f32,
}
