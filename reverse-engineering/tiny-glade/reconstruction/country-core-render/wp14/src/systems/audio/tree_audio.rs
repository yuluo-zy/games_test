//! systems/audio/tree_audio.rs — 树木音频（PlayTreeSoundCmd + TreeAmbience 最近树）。
//!
//! 反汇编证据（O）：`tree_audio_init_system`（0x1414633e0）4×`instantiate_loop`
//! （含 `event:/SFX/Tree/Amb` 循环；`TreeMove/TreePlace/TreeGizmo{Hover,Release}` 瞬发，I 映射）；
//! `tree_audio_system`（0x1414638e0）：`EventReader<PlayTreeSoundCmd>`（wp10 tree-tool 消费）+
//! `Query<&TreeAmbience>` → `(dist,pos)` 稳定排序（行 108）→ 最近树音频 set_position/音量。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Query, Res, ResMut};
use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayTreeSoundCmd;
use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `TreeAmbience` 组件（O 名；每树的环境音频标志，U 布局）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TreeAmbience(pub Gap);

/// `TreeAudioState`（O 名；4×AudioLoop，I）。
#[derive(Resource, Debug, Default)]
pub struct TreeAudioState {
    /// 循环 loop（O 数量；布局 U）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `tree_audio_init_system`（0x1414633e0，O）。
pub fn tree_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let names: [&'static str; 4] = [
        "event:/SFX/Tree/Amb",
        "event:/SFX/Tree/TreeMove",
        "event:/SFX/Tree/TreePlace",
        "event:/SFX/Tree/TreeGizmoHover",
    ];
    let mut loops = Vec::new();
    for name in names {
        let desc = AudioLoopDesc::new(name, None, 3, 0x32);
        if let Ok(loop_) = core.instantiate_loop(&desc) {
            loops.push(loop_);
        }
    }
    commands.insert_resource(TreeAudioState { loops, _gap: Gap });
}

/// `tree_audio_system`（0x1414638e0，O）。
pub fn tree_audio_system(
    mut _state: ResMut<TreeAudioState>,
    mut _events: EventReader<PlayTreeSoundCmd>,
    _trees: Query<&TreeAmbience>,
) {
    // 事件分派 + 最近树 `(dist, pos)` 排序（行 108，O）→ 树环境音频 set_position/音量。
}
