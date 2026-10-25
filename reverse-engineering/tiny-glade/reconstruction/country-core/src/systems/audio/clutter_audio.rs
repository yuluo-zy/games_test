//! systems/audio/clutter_audio.rs — 杂物音频（PlayClutterSoundCmd）。
//!
//! 反汇编证据（O）：`clutter_audio_system`（0x1408c02f0）`PlayClutterSoundCmd` →
//! `instantiate_audio`（`event:/SFX/Cosmetics/{Clutter_Detach,Clutter_Pickup,Clutter_Rotate,
//! MoveBuilding,MoveGround}`，O）→ set_parameter + set_position + start_if_not_playing。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayClutterSoundCmd;
use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;

/// `ClutterAudioState`（O 名；U 布局）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ClutterAudioState(pub Gap);

/// `clutter_audio_system`（0x1408c02f0，O）。
pub fn clutter_audio_system(
    _core: Res<AudioCore>,
    mut _events: EventReader<PlayClutterSoundCmd>,
) {
    // 事件 → instantiate_audio（Clutter_Detach/Pickup/Rotate/MoveBuilding/MoveGround，O）
    // → set_parameter + set_position + start_if_not_playing（libfmod 边界）。
}
