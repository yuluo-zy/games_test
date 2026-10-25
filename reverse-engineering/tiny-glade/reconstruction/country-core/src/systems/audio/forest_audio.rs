//! systems/audio/forest_audio.rs — 森林生长音频。
//!
//! 反汇编证据（O）：`forest_audio_init_system`（0x140aa18f0）1×`instantiate_loop`
//! （`event:/SFX/ForestGrow`）；`forest_audio_system`（0x140aa1f00）按事件驱动生长循环
//! （set_position ×2、start/stop ×2）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `ForestAudioState`（O 名；1×AudioLoop）。
#[derive(Resource, Debug, Default)]
pub struct ForestAudioState {
    /// 生长循环 loop（O 1 个）。
    pub grow_loop: Option<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `forest_audio_init_system`（0x140aa18f0，O）。
pub fn forest_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let desc = AudioLoopDesc::new("event:/SFX/ForestGrow", None, 3, 0x32);
    let grow_loop = core.instantiate_loop(&desc).ok();
    commands.insert_resource(ForestAudioState { grow_loop, _gap: Gap });
}

/// `forest_audio_system`（0x140aa1f00，O）。
pub fn forest_audio_system(mut _state: ResMut<ForestAudioState>) {
    // 森林生长事件 → 生长循环 set_position / start / stop（O）。
}
