//! systems/audio/wall_audio_utils.rs — 墙体循环播放器（AudioLoopPlayer）。
//!
//! 反汇编证据（O）：`AudioLoopPlayer::new`（行 46，wall_audio_init 内联）、`resolve`（行 32，
//! wall_audio_fmod 内联）、`request_play`（0x1409285a0，行 95）：`+0x38=1`（应播）、
//! `instances[idx]`（0x30 字节 AudioInstance）：`set_position` → `set_parameter("editing_speed",
//! 0xd, vol)` → `EventInstance::set_volume`；`+0x28` 当前索引 < `+0x10` 池容量。

#![allow(dead_code, unused_imports)]

use crate::resources::audio_resources::AudioInstance;
use crate::typed_gap::Gap;

/// 墙体循环播放器（O 布局部分恢复；`+0x28` 当前、`+0x38` 应播标志、池 Vec、元素参数 Vec）。
#[derive(Debug, Default)]
pub struct AudioLoopPlayer {
    /// 实例池（O）。
    pub instances: Vec<AudioInstance>,
    /// 当前索引 @+0x28（O）。
    pub current: usize,
    /// 应播标志 @+0x38（O）。
    pub should_play: bool,
    /// 其余（U）。
    pub _gap: Gap,
}

impl AudioLoopPlayer {
    /// `new`（行 46，O）。
    pub fn new(_instances: Vec<AudioInstance>) -> Self {
        Self::default()
    }

    /// `resolve`（行 32，O）：取当前实例。
    pub fn resolve(&self) -> Option<&AudioInstance> {
        self.instances.get(self.current)
    }

    /// `request_play`（0x1409285a0，O）。
    pub fn request_play(&mut self, _position: glam::Vec3, _volume: f32, _editing_speed: f32) {
        self.should_play = true;
        // set_position + set_parameter("editing_speed", …) + EventInstance::set_volume（libfmod 边界）。
    }
}
