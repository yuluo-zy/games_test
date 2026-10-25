//! systems/audio/ivy_audio.rs — 常春藤生长音频。
//!
//! 反汇编证据（O）：`ivy_audio_init_system`（0x140ad5350）`instantiate_audio`
//! （`event:/SFX/IvyGrow`）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::Res;
use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;

/// `IvyAudioState`（O 名；U 布局）。
#[derive(Resource, Clone, Debug, Default)]
pub struct IvyAudioState(pub Gap);

/// `ivy_audio_init_system`（0x140ad5350，O）。
pub fn ivy_audio_init_system(_core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    // AudioCore::instantiate_audio("event:/SFX/IvyGrow")（libfmod 边界；串 O）。
    commands.insert_resource(IvyAudioState(Gap));
}
