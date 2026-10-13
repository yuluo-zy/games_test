//! systems/audio/wall_audio_fmod.rs — 墙体音频（PlayWallAudio 主事件系统）。
//!
//! 反汇编证据（O，audio-events-analysis §3）：`wall_audio_init_system`（0x14094fb80）10×
//! `AudioCore::instantiate_loop`（`event:/SFX/Wall Draw /Wall Change Height /FenceExtend /
//! LintelExtend /StoneArchExtend /WideningGate /PlatformStairsExtend /SewerGateWiden /
//! FenceHeight /FenceMove`）→ `insert_resource<WallAudioState>`；`wall_audio_fmod`
//! （0x140950920）：kind 跳表 0-23，循环类经 `AudioLoopPlayer::request_play`
//! （editing_speed/FenceLength 参数），瞬发类经 `instantiate_audio_w_position_and_volume`
//! （FenceAppear/Merge/Rearrange、LintelAppear/HasBricks/Merge、StoneArchAppear、Pillar{Stone,
//! Wood}Appear、GateAppear、PlatformStairsAppear、WallSplit、SewerGateAppear）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayWallAudio;
use crate::resources::audio_resources::AudioCore;
use crate::systems::audio::wall_audio_utils::AudioLoopPlayer;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::AudioLoopDesc;

/// `WallAudioState`（O 名/布局：10×`AudioLoopPlayer` stride 0x40 + 元素参数 Vec + 音量）。
#[derive(Resource, Debug, Default)]
pub struct WallAudioState {
    /// 10 个 loop player（O 布局 stride 0x40）。
    pub loops: Vec<AudioLoopPlayer>,
    /// 音量调制 @+0x280（O）。
    pub volume: f32,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `wall_audio_init_system`（0x14094fb80，O）。
pub fn wall_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let names: [&'static str; 10] = [
        "event:/SFX/Wall Draw",
        "event:/SFX/Wall Change Height",
        "event:/SFX/FenceExtend",
        "event:/SFX/LintelExtend",
        "event:/SFX/StoneArchExtend",
        "event:/SFX/WideningGate",
        "event:/SFX/PlatformStairsExtend",
        "event:/SFX/SewerGateWiden",
        "event:/SFX/FenceHeight",
        "event:/SFX/FenceMove",
    ];
    for name in names {
        let desc = AudioLoopDesc::new(name, Some("editing_speed"), 3, 0x32);
        let _ = core.instantiate_loop(&desc);
    }
    commands.insert_resource(WallAudioState::default());
}

/// `wall_audio_fmod`（0x140950920，O）。
pub fn wall_audio_fmod(
    _core: Res<AudioCore>,
    mut _state: ResMut<WallAudioState>,
    mut _events: EventReader<PlayWallAudio>,
) {
    // kind 跳表 0-23：循环（request_play + editing_speed/FenceLength）/
    // 瞬发（instantiate_audio_w_position_and_volume）→ 帧尾 10 loop start/stop（O）。
}
