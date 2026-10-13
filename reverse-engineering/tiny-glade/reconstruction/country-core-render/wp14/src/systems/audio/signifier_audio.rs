//! systems/audio/signifier_audio.rs — UI 符号流音频（UiSignifierStream 参数更新）。
//!
//! 反汇编证据（O）：`signifier_audio_init_system`（0x1409d6320）1×`instantiate_loop` +
//! 4×`instantiate_audio`（`event:/Cursor/{Grab,Release}` 等）；`signifier_audio_system`
//! （0x1409d6b90，行 82/268）：`UiSignifierStream` → 21×set_parameter（全局 + 实例参数
//! `FirstPersonOff` 等，O 串部分恢复）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::{AudioCore, AudioInstance};
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `SignifierAudioState`（O 名；1×AudioLoop + 4×AudioInstance，I）。
#[derive(Resource, Debug, Default)]
pub struct SignifierAudioState {
    /// 循环 loop（O 1 个）。
    pub loop_: Option<AudioLoop>,
    /// 瞬发实例（O 4 个）。
    pub instances: Vec<AudioInstance>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `UiSignifierStream`（O 名；UI 符号流，参数更新消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct UiSignifierStream(pub Gap);

/// `signifier_audio_init_system`（0x1409d6320，O）。
pub fn signifier_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let desc = AudioLoopDesc::new("event:/Cursor/Grab", None, 3, 0x32);
    let loop_ = core.instantiate_loop(&desc).ok();
    let mut instances = Vec::new();
    for name in ["event:/Cursor/Grab", "event:/Cursor/Release"] {
        if let Ok(inst) = core.instantiate_audio(name) {
            instances.push(inst);
        }
    }
    commands.insert_resource(SignifierAudioState {
        loop_,
        instances,
        _gap: Gap,
    });
}

/// `signifier_audio_system`（0x1409d6b90，O）。
pub fn signifier_audio_system(
    mut _state: ResMut<SignifierAudioState>,
    _stream: Res<UiSignifierStream>,
) {
    // 符号流 → 参数更新（set_parameter ×21，O）。
}
