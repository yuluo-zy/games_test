//! systems/audio/garden_audio.rs — 花园绘制音频。
//!
//! 反汇编证据（O）：`garden_audio_init_system`（0x140a88fa0）4×`instantiate_loop`
//! （`event:/SFX/Garden/{GardenDraw,GardenDrawWater,GardenErase,GardenEraseWater}`）；
//! `garden_audio_system`（0x140a89480）`EventReader<…>` → `path_audio::update_draw_audio`
//! 模板（行 77/93/109/125 closure）；`brush_editing_speed_01`（path_audio 行 234）共享。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `GardenAudioState`（O 名；4×AudioLoop）。
#[derive(Resource, Debug, Default)]
pub struct GardenAudioState {
    /// 4 个绘制/擦除 loop（O 数量）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `garden_audio_init_system`（0x140a88fa0，O）。
pub fn garden_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let names: [&'static str; 4] = [
        "event:/SFX/Garden/GardenDraw",
        "event:/SFX/Garden/GardenDrawWater",
        "event:/SFX/Garden/GardenErase",
        "event:/SFX/Garden/GardenEraseWater",
    ];
    let mut loops = Vec::new();
    for name in names {
        let desc = AudioLoopDesc::new(name, Some("editing_speed"), 3, 0x32);
        if let Ok(loop_) = core.instantiate_loop(&desc) {
            loops.push(loop_);
        }
    }
    commands.insert_resource(GardenAudioState { loops, _gap: Gap });
}

/// `garden_audio_system`（0x140a89480，O）：`update_draw_audio` 模板实例。
pub fn garden_audio_system(
    mut _state: ResMut<GardenAudioState>,
    mut _events: EventReader<crate::typed_gap::GapEventPlaceholder>,
) {
    // update_draw_audio<closure_env$1>（行 77/93/109/125，O）。
}
