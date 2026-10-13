//! systems/audio/elevation_supports_audio.rs — 抬升支撑音频（PlayElevationSupportSoundCmd）。
//!
//! 反汇编证据（O）：`elevation_supports_audio`（0x140ad51c0）`PlayElevationSupportSoundCmd` →
//! `instantiate_audio`（`event:/SFX/Shapes/WoodenBracketsAppear`，I 映射）+ start。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayElevationSupportSoundCmd;
use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;

/// `ElevationSupportsAudioState`（O 名；U 布局）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ElevationSupportsAudioState(pub Gap);

/// `elevation_supports_audio`（0x140ad51c0，O）。
pub fn elevation_supports_audio(
    _core: Res<AudioCore>,
    mut _events: EventReader<PlayElevationSupportSoundCmd>,
) {
    // 事件 → instantiate_audio（WoodenBracketsAppear）+ start（libfmod 边界）。
}
