//! systems/audio/decorator_audio.rs — 装饰物音频（PlayDecoratorSoundCmd）。
//!
//! 反汇编证据（O）：`decorator_audio_init_system`（0x14145c6b0）16×`instantiate_loop`
//! （`event:/SFX/Window{Place,PlacePreview,Remove,Combine,Hover,Drag} /
//! DecoratorMoveWoodenFloorTEMP / Lantern/{MoveWall,MoveGround,Rotate,Amb} /
//! ChimneyChangeHeight / ChimneyDrag / Flags/{Rotate,Drag,DragGround} /
//! Cosmetics/Clutter_Rotate / Cosmetics/MoveGround`，串 O 映射 I）；`decorator_audio_system`
//! （0x14145dc20，行 811 稳定排序 `(f32,Vec3)` 距离衰减）→ 25×instantiate_audio +
//! set_position/set_parameter/start。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayDecoratorSoundCmd;
use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `DecoratorAudioState`（O 名；16×AudioLoop，I）。
#[derive(Resource, Debug, Default)]
pub struct DecoratorAudioState {
    /// 循环 loop（O 数量）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `decorator_audio_init_system`（0x14145c6b0，O）。
pub fn decorator_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let names: [&'static str; 16] = [
        "event:/SFX/WindowDrag",
        "event:/SFX/WindowPlace",
        "event:/SFX/WindowPlacePreview",
        "event:/SFX/WindowRemove",
        "event:/SFX/WindowCombine",
        "event:/SFX/WindowHover",
        "event:/SFX/DecoratorMoveWoodenFloorTEMP",
        "event:/SFX/Lantern/MoveWall",
        "event:/SFX/Lantern/MoveGround",
        "event:/SFX/Lantern/Rotate",
        "event:/SFX/Lantern/Amb",
        "event:/SFX/ChimneyChangeHeight",
        "event:/SFX/ChimneyDrag",
        "event:/SFX/Flags/Rotate",
        "event:/SFX/Flags/Drag",
        "event:/SFX/Flags/DragGround",
    ];
    let mut loops = Vec::new();
    for name in names {
        let desc = AudioLoopDesc::new(name, Some("editing_speed"), 3, 0x32);
        if let Ok(loop_) = core.instantiate_loop(&desc) {
            loops.push(loop_);
        }
    }
    commands.insert_resource(DecoratorAudioState { loops, _gap: Gap });
}

/// `decorator_audio_system`（0x14145dc20，O）。
pub fn decorator_audio_system(
    mut _state: ResMut<DecoratorAudioState>,
    mut _events: EventReader<PlayDecoratorSoundCmd>,
) {
    // 事件 → kind → 循环/瞬发（`(f32,Vec3)` 稳定排序行 811 距离衰减，O）。
}
