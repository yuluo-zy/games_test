//! startup/load_audio.rs — 音频启动装配（等银行样例加载，插触发器库等资源）。
//!
//! PDB 骨架：`load_audio`（0x140b0e0f0）。
//! 反汇编（O）：`Instant::now` → `AudioSamplesLoadedPromise::recv()` → 日志
//! `country_core::startup::load_audio` 耗时 → `Commands::insert_resource<AudioTriggerLibrary>`
//! → `insert_resource<StairsCounter>` → `insert_resource<GameLoadingAudioState>` →
//! `remove_resource<AudioSamplesLoadedPromise>`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::system::Commands;
use bevy_ecs::resource::Resource;

use crate::asset_libraries::audio_trigger_library::AudioTriggerLibrary;
use crate::resources::audio_resources::AudioSamplesLoadedPromise;
use crate::systems::audio::game_loading_audio::GameLoadingAudioState;
use crate::typed_gap::Gap;

/// `StairsCounter`（O 名；`load_audio` 插入的启动计数资源，语义 U）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct StairsCounter(pub Gap);

/// `load_audio`（0x140b0e0f0，O）：启动期音频装配系统。
pub fn load_audio(mut commands: Commands) {
    // 1) 等银行样例加载完成（O）。
    AudioSamplesLoadedPromise::default().recv();
    // 2) 日志耗时（`country_core::startup::load_audio`，O）。
    // 3) 装配触发器库 / 楼梯计数 / 加载状态（O）。
    commands.insert_resource(AudioTriggerLibrary::default());
    commands.insert_resource(StairsCounter(Gap));
    commands.insert_resource(GameLoadingAudioState::default());
    // 4) 移除 promise（O）。
    commands.remove_resource::<AudioSamplesLoadedPromise>();
}
