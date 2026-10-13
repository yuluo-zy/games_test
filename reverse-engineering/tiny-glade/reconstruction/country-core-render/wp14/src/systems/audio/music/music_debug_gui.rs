//! systems/audio/music/music_debug_gui.rs — 音乐调试 GUI。
//!
//! PDB 骨架（O）：`music_debug_gui`（0x14083c4a0，381B，行 544 区段）。签名（add_audio_systems
//! 证据）：`(Option<Res<EguiContextResource>>, ResMut<MusicAudioState>, ResMut<DebugGui>)`。
//! egui 为 REUSED 依赖；此处以 typed-gap 占位。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::systems::audio::music::music_system::MusicAudioState;
use crate::typed_gap::Gap;

/// `EguiContextResource`（REUSED egui 边界，U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct EguiContextResource(pub Gap);

/// `DebugGui`（O 名；调试 GUI 状态，U 布局）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DebugGui(pub Gap);

/// `music_debug_gui`（0x14083c4a0，O）：调试面板（egui），驱动 MusicPlayer 停止/测试。
pub fn music_debug_gui(
    _egui: Option<Res<EguiContextResource>>,
    mut _music: ResMut<MusicAudioState>,
    mut _debug_gui: ResMut<DebugGui>,
) {
    // egui Ui + AudioInstance::stop_immediately（行 544 内联，O）。
}
