//! systems/audio/wall_audio_ui.rs — 墙体 UI 音频（PlayWallUiSound）。
//!
//! 反汇编证据（O）：`wall_audio_ui`（0x1408b7690）`PlayWallUiSound` → 事件串
//! `event:/SFX/Color{Bricks,Gate,Halftimber,Window}`、`ChangeWallStyle`、`WindowDrag`、
//! `event:/UI/…`（RNE .data，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayWallUiSound;
use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;

/// `WallAudioUiState`（O 名；U 布局）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallAudioUiState(pub Gap);

/// `wall_audio_ui`（0x1408b7690，O）。
pub fn wall_audio_ui(
    _core: Res<AudioCore>,
    mut _events: EventReader<PlayWallUiSound>,
) {
    // ColorBricks/Gate/Halftimber/Window、ChangeWallStyle、WindowDrag（O）。
}
