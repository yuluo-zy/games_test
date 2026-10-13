//! systems/audio/menu_audio.rs — 菜单音频（PlayMenuUiSound）。
//!
//! 反汇编证据（O）：`menu_audio_init_system`（0x140a0b340）1×`instantiate_audio`
//! （`snapshot:/PauseMenu`）；`menu_audio`（0x140a0b4c0，行 47 分派）：`PlayMenuUiSound` →
//! 13 个 UI 串（`event:/UI/Menu/{Hover,Select,Settings,SettingsTab,SliderTick,ShowSessions,
//! NewSession,StartSession,Back,Open,Close}` + `event:/UI/Ui2dHover /Ui2dLockedClickNew`，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayMenuUiSound;
use crate::resources::audio_resources::{AudioCore, AudioInstance};
use crate::typed_gap::Gap;

/// `MenuAudioState`（O 名；含 `snapshot:/PauseMenu` 实例）。
#[derive(Resource, Debug, Default)]
pub struct MenuAudioState {
    /// PauseMenu 快照实例（O）。
    pub pause_snapshot: Option<AudioInstance>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `menu_audio_init_system`（0x140a0b340，O）。
pub fn menu_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let pause_snapshot = core.instantiate_audio("snapshot:/PauseMenu").ok();
    commands.insert_resource(MenuAudioState {
        pause_snapshot,
        _gap: Gap,
    });
}

/// `menu_audio`（0x140a0b4c0，O）。
pub fn menu_audio(
    _core: Res<AudioCore>,
    mut _events: EventReader<PlayMenuUiSound>,
) {
    // 13 个 UI 串分派（O；行 47 跳表）。
}
