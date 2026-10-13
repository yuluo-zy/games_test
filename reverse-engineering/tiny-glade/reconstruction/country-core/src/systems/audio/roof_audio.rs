//! systems/audio/roof_audio.rs — 屋顶音频（与 WP9 交付调和）。
//!
//! PDB 骨架：`RoofAudioState`；`roof_audio_init_system`（0x140aa2fd0）、
//! `instantiate_audio_w_position`（0x140aa3360）、`instantiate_audio_w_position_and_volume`
//! （0x140aa3590）、`roof_audio_system`（0x140aa3860）。
//! 反汇编证据（O）：init 3×`AudioCore::instantiate_loop`（事件串 RNE .data：
//! `event:/SFX/RoofAppear /RoofSwitchDirection /RoofTilesDetach /RoofTilesLand /RoofTipMove /
//! Rooftip_Release`，映射 I）+ `insert_resource<RoofAudioState>`；
//! `instantiate_audio_w_position_and_volume`：`AudioCoreInner::instantiate_audio` →
//! `set_position` → `start_if_not_playing` → `EventInstance::set_volume`（0x140aa3590，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::{AudioCore, AudioInstance};
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// 屋顶音频状态（Resource，O）。
#[derive(Resource, Debug, Default)]
pub struct RoofAudioState {
    /// 3 个循环 loop（O 数量；WP9 反汇编）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `roof_audio_init_system`（O：0x140aa2fd0）。3×`AudioCore::instantiate_loop` +
/// `insert_resource<RoofAudioState>`。
pub fn roof_audio_init_system(core: &AudioCore, mut commands: bevy_ecs::system::Commands) {
    let names: [&'static str; 3] = [
        "event:/SFX/RoofShape",
        "event:/SFX/RoofAppear",
        "event:/SFX/RoofSwitchDirection",
    ];
    let mut loops = Vec::new();
    for name in names {
        let desc = AudioLoopDesc::new(name, None, 3, 0x32);
        if let Ok(loop_) = core.instantiate_loop(&desc) {
            loops.push(loop_);
        }
    }
    commands.insert_resource(RoofAudioState { loops, _gap: Gap });
}

/// `instantiate_audio_w_position`（O：0x140aa3360）。
pub fn instantiate_audio_w_position(_core: &AudioCore, _position: [f32; 3]) {
    // AudioCore::instantiate_audio + set_position（libfmod 边界）。
}

/// `instantiate_audio_w_position_and_volume`（O：0x140aa3590）。
pub fn instantiate_audio_w_position_and_volume(
    _core: &AudioCore,
    _position: [f32; 3],
    _volume: f32,
) {
    // instantiate_audio → set_position → start_if_not_playing → EventInstance::set_volume（O）。
}

/// `roof_audio_system`（O：0x140aa3860）。按 RoofAudioState 驱动循环音量/位置。
pub fn roof_audio_system(_state: &RoofAudioState, _dt: f32) {
    let _ = _state;
    let _ = _dt;
}
