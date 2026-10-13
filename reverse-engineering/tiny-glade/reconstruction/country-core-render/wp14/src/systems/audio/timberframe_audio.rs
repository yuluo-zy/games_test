//! systems/audio/timberframe_audio.rs — 木框架音频（PlayTimberframeSound）。
//!
//! 反汇编证据（O）：`timberframe_audio_init_system`（0x140a67810）
//! `AudioLoopDesc::new`（行 12）+ `instantiate_loop`；`timberframe_audio`（0x140a67b90）：
//! `PlayTimberframeSound` → `event:/SFX/Timberframe{Move,Length,Width,Appear,Disappear,
//! RectangleAppear}`（O）+ `editing_speed` 参数。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayTimberframeSound;
use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `TimberframeAudioState`（O 名；循环 + 瞬发，I）。
#[derive(Resource, Debug, Default)]
pub struct TimberframeAudioState {
    /// 循环 loop（O）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `timberframe_audio_init_system`（0x140a67810，O）。
pub fn timberframe_audio_init_system(
    core: Res<AudioCore>,
    mut commands: bevy_ecs::system::Commands,
) {
    let desc = AudioLoopDesc::new("event:/SFX/TimberframeMove", Some("editing_speed"), 3, 0x32);
    let mut loops = Vec::new();
    if let Ok(loop_) = core.instantiate_loop(&desc) {
        loops.push(loop_);
    }
    commands.insert_resource(TimberframeAudioState { loops, _gap: Gap });
}

/// `timberframe_audio`（0x140a67b90，O）。
pub fn timberframe_audio(
    _core: Res<AudioCore>,
    mut _state: ResMut<TimberframeAudioState>,
    mut _events: EventReader<PlayTimberframeSound>,
) {
    // 事件 → Move/Length/Width/Appear/Disappear/RectangleAppear（O）+ editing_speed。
}
