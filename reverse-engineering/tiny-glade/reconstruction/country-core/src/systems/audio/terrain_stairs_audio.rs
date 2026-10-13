//! systems/audio/terrain_stairs_audio.rs — 地形楼梯音频（SuppressPathSound + StairDraw）。
//!
//! 反汇编证据（O）：`terrain_stairs_audio_init_system`（行 49，`(Res<AudioCore>, Commands)`）
//! `AudioLoopDesc::new`（行 12）+ `instantiate_loop`；`terrain_stairs_audio_system`
//! （0x141462dc0，行 147/95）：`Vec::retain(is_playing)` 清理活动实例（closure$0 行 100）+
//! `event:/SFX/StairDraw_LoopTremble /StairDraw_OneShot`（RNE .data，O）；消费 `SuppressPathSound`
//! 事件（register_event #2）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::audio_events::SuppressPathSound;
use crate::resources::audio_resources::{AudioCore, ActiveAudioInstances};
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `TerrainStairsAudioState`（O 名；活动实例 + 循环，I）。
#[derive(Resource, Debug, Default)]
pub struct TerrainStairsAudioState {
    /// 活动实例（O：`Vec::retain(is_playing)` 清理）。
    pub active: ActiveAudioInstances,
    /// 循环 loop（O）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `terrain_stairs_audio_init_system`（行 49，O）。
pub fn terrain_stairs_audio_init_system(
    core: Res<AudioCore>,
    mut commands: bevy_ecs::system::Commands,
) {
    let desc = AudioLoopDesc::new("event:/SFX/StairDraw_LoopTremble", None, 3, 0x32);
    let mut loops = Vec::new();
    if let Ok(loop_) = core.instantiate_loop(&desc) {
        loops.push(loop_);
    }
    commands.insert_resource(TerrainStairsAudioState {
        active: ActiveAudioInstances::default(),
        loops,
        _gap: Gap,
    });
}

/// `terrain_stairs_audio_system`（0x141462dc0，O）。
pub fn terrain_stairs_audio_system(
    mut _state: ResMut<TerrainStairsAudioState>,
    mut _events: EventReader<SuppressPathSound>,
) {
    // 帧首 `Vec::retain(is_playing)`（closure$0 行 100，O）→ StairDraw 循环/瞬发。
}
