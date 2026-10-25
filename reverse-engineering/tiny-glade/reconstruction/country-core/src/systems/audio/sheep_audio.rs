//! systems/audio/sheep_audio.rs — 绵羊音频（PlaySheepReactSound）。
//!
//! 反汇编证据（O）：`sheep_audio_init_system`（行 26，`(Res<AudioCore>, Commands)`）
//! `impl$0::new`（O 名）；`sheep_audio_system`（0x1409d5ea0）：`PlaySheepReactSound` →
//! `event:/SFX/Sheep/{Idle,Eyes,Confused,Pet,Aw,Baah,So,UmbrellaClose,UmbrellaOpen}`（O）
//! + 参数 `FirstPersonOff`；4×instantiate_audio + 3 AudioLoop。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlaySheepReactSound;
use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;

/// `SheepAudio`（O 名；`impl$0::new`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SheepAudio(pub Gap);

/// `sheep_audio_init_system`（行 26，O）。
pub fn sheep_audio_init_system(_core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    commands.insert_resource(SheepAudio(Gap));
}

/// `sheep_audio_system`（0x1409d5ea0，O）。
pub fn sheep_audio_system(
    _core: Res<AudioCore>,
    mut _events: EventReader<PlaySheepReactSound>,
) {
    // 绵羊反应事件 → Idle/Eyes/Confused/Pet/… 实例（O）+ 参数 FirstPersonOff。
}
