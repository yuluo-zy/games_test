//! systems/audio/replay_audio.rs — 回放音频（PlayReplayAudio：Undo/Redo/Deny）。
//!
//! 反汇编证据（O）：`replay_audio_init_system`（0x140ad54d0）2×`instantiate_audio`
//! （`event:/UI/Undo`、`event:/UI/Redo`）；`replay_audio_system`（0x140ad5800）：
//! 回放命令栅格（stride 0x10，byte@+8：0/1 → Undo/Redo start、2 → `event:/UI/DenyInput`
//! 瞬发）+ AppMode==0xd 时 2×全局 set_parameter。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayReplayAudio;
use crate::resources::audio_resources::{AudioCore, AudioInstance};
use crate::typed_gap::Gap;

/// `ReplayAudioState`（O 名；2×AudioInstance：Undo/Redo）。
#[derive(Resource, Debug, Default)]
pub struct ReplayAudioState {
    /// Undo 实例（O）。
    pub undo: Option<AudioInstance>,
    /// Redo 实例（O）。
    pub redo: Option<AudioInstance>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `replay_audio_init_system`（0x140ad54d0，O）。
pub fn replay_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let undo = core.instantiate_audio("event:/UI/Undo").ok();
    let redo = core.instantiate_audio("event:/UI/Redo").ok();
    commands.insert_resource(ReplayAudioState {
        undo,
        redo,
        _gap: Gap,
    });
}

/// `replay_audio_system`（0x140ad5800，O）。
pub fn replay_audio_system(
    _core: Res<AudioCore>,
    mut _events: EventReader<PlayReplayAudio>,
) {
    // 回放命令栅格 → Undo/Redo/DenyInput（O）。
}
