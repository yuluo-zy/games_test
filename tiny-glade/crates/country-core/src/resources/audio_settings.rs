//! `country_core::resources::audio_settings::AudioSettings`（B 原模块路径；组 A 未落库）。
//!
//! fresh 证据（Ghidra shared 只读）：
//!  * serialize `0x140168910`：struct 名 "AudioSettings"，字段按序
//!    `master_volume` @+0、`music_volume` @+4、`sfx_volume` @+8、
//!    `ambience_volume` @+0xc（4 × f32）。
//!  * `GameConfig::default`（0x140914880）在 audio 槽 @+0xf8 写 4 × 1.0f。
//!
//! 音频更新直接读取 GameConfig.audio；Music 与 Music-DawnJingle 共用
//! music_volume，同一四字段权威状态也供设置页面序列化。

use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub ambience_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 1.0,
            sfx_volume: 1.0,
            ambience_volume: 1.0,
        }
    }
}
