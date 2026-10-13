//! systems/audio/terrain_audio.rs — 地形编辑音频。
//!
//! 反汇编证据（O）：`terrain_audio_init_system`（0x1414613e0）4×`instantiate_loop`
//! （`event:/SFX/TerrainEditShape /TerrainEditHeight /TerrainCreateChunk /TerrainMove`，I 顺序）；
//! `terrain_audio_system`（0x1414618c0）`PlayTerrainSound` + 地形编辑栅格索引 → kind 跳表 →
//! 对 4 loop 依应播标志 `start_if_not_playing + set_parameter + set_position` 或 `stop`。
//! `EditStateAudio::new`（行 18）/ `update`（行 25，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayTerrainSound;
use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `TerrainAudioState`（O 名；4×AudioLoop stride 0x40，应播标志 +0x38/+0x78/+0xb8/+0xf8）。
#[derive(Resource, Debug, Default)]
pub struct TerrainAudioState {
    /// 4 个循环 loop（O 数量；布局 U）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `EditStateAudio`（O 名；`new` 行 18、`update` 行 25）。
#[derive(Resource, Clone, Debug, Default)]
pub struct EditStateAudio(pub Gap);

impl EditStateAudio {
    /// `new`（行 18，O）。
    pub fn new() -> Self {
        Self(Gap)
    }
    /// `update`（行 25，O）。
    pub fn update(&mut self) {
        let _ = self;
    }
}

/// `terrain_audio_init_system`（0x1414613e0，O）。
pub fn terrain_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let names: [&'static str; 4] = [
        "event:/SFX/TerrainEditShape",
        "event:/SFX/TerrainEditHeight",
        "event:/SFX/TerrainCreateChunk",
        "event:/SFX/TerrainMove",
    ];
    let mut loops = Vec::new();
    for name in names {
        let desc = AudioLoopDesc::new(name, Some("editing_speed"), 3, 0x32);
        if let Ok(loop_) = core.instantiate_loop(&desc) {
            loops.push(loop_);
        }
    }
    commands.insert_resource(TerrainAudioState { loops, _gap: Gap });
}

/// `terrain_audio_system`（0x1414618c0，O）。
pub fn terrain_audio_system(
    mut _state: ResMut<TerrainAudioState>,
    mut _events: EventReader<PlayTerrainSound>,
) {
    // 地形编辑栅格索引 → kind 跳表 → 4 loop 依应播标志驱动（O）。
}
