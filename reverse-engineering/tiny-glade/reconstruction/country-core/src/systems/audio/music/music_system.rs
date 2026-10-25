//! systems/audio/music/music_system.rs — 音乐系统（richness/新 glade 时间参数）。
//!
//! 反汇编证据（O，audio-events-analysis §6）：`music_audio_init_system`（0x140a71f20）
//! `GladeAudio::default` → 3×instantiate_audio → `start` → `MusicPlayer::init` →
//! `insert_resource<MusicAudioState>`；参数串 `MusicRichness、NewGladeTime、PreviewGladeType`（O）。
//! `music_audio_system`（0x140a72810）：richness = clamp(time/const)，`time_continuous_lerp`
//! 平滑（State+0x1c4）；set_parameter(MusicRichness/NewGladeTime)；AppMode `|8==0xa` 时
//! `MusicPlayer::update`。`update_music_richness_based_on_time`（O 名）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{Res, ResMut};
use bevy_ecs::resource::Resource;
use bevy_ecs::system::Commands;

use crate::resources::audio_resources::AudioCore;
use crate::systems::audio::music::music_player::MusicPlayer;
use crate::systems::audio::App;
use crate::typed_gap::Gap;

/// `MusicAudioState`（O 名；richness 平滑字段 @+0x1c4）。
#[derive(Resource, Debug, Default)]
pub struct MusicAudioState {
    /// 音乐播放器（O）。
    pub player: MusicPlayer,
    /// richness 平滑 @+0x1c4（O 偏移）。
    pub richness: f32,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `music_audio_init_system`（0x140a71f20，O；arg1=App）。
pub fn music_audio_init_system(_app: &mut App, mut commands: Commands) {
    // GladeAudio::default → 3×instantiate_audio → start → MusicPlayer::init → insert_resource（O）。
    commands.insert_resource(MusicAudioState::default());
}

/// `music_audio_system`（0x140a72810，O）。
pub fn music_audio_system(
    _core: Res<AudioCore>,
    mut _state: ResMut<MusicAudioState>,
) {
    // richness = clamp(time/const) + time_continuous_lerp；set_parameter(MusicRichness/
    // NewGladeTime)；AppMode `|8==0xa` → MusicPlayer::update（O）。
}

/// `update_music_richness_based_on_time`（O 名）。
pub fn update_music_richness_based_on_time(_time: f32) -> f32 {
    0.0
}
