//! startup/load_glade_audio.rs — 预览 glade 音频装载（GladeAudio）。
//!
//! PDB 骨架：`GladeAudio`（serde_json Deserialize + Default）、`GladeAudio::load`（行 101）、
//! `GladeAudio::try_load`（行 119）、`stop_and_play_new`（行 81）、`load_glade_audio`（0x1408be360）。
//! 反汇编证据（O）：`load_glade_audio` 经 `AudioCore::instantiate_audio` 建
//! `event:/Music/mus_GladePreview`（×2 短名）+ `event:/Amb/GladePreview` 并 `start`；
//! `stop_and_play_new` 先停旧实例再播新 glade（行 81）。`GladeAudio` 为 serde_json 结构
//! （deserialize 行 101，visit_map/visit_str/visit_seq 证据）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::{AudioCore, AudioInstance};
use crate::typed_gap::Gap;

/// 预览 glade 音频描述（O：serde_json 反序列化结构；字段 U）。
#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct GladeAudio {
    /// 音乐事件名（I：`event:/Music/mus_GladePreview`）。
    pub _music: Gap,
    /// 其余字段（U）。
    pub _gap: Gap,
}

impl GladeAudio {
    /// `GladeAudio::load`（0x1408bf460，行 101，O）。
    pub fn load(&mut self) {
        let _ = self;
    }

    /// `GladeAudio::try_load`（行 119，O）。
    pub fn try_load(&mut self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    /// `stop_and_play_new`（行 81，O）：停旧实例并播放新 glade。
    pub fn stop_and_play_new(&mut self) {
        let _ = self;
    }
}

/// `load_glade_audio`（0x1408be360，O）：启动期装载预览 glade 音乐/环境。
pub fn load_glade_audio(_core: &AudioCore, _glade: &GladeAudio) {
    // 3× AudioCore::instantiate_audio（mus_GladePreview ×2 + Amb/GladePreview）→ start
    // （libfmod 边界；串 O）。
}
