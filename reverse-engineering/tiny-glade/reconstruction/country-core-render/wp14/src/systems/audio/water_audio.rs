//! systems/audio/water_audio.rs — 水面绘制/擦除音频（wp7 契约消费方）。
//!
//! 反汇编证据（O）：`water_audio_init_system`（0x140873140）`AudioLoopDesc::new` ×4（行 12）
//! + `instantiate_loop`（`event:/SFX/Water/{WaterDraw,WaterErase,IceDraw,IceErase,
//! PathWaterDraw,PathWaterErase}`，串 O 映射 I）；`water_audio_system`（0x140873630）：
//! `WaterRaster` 栅格编辑索引 → `path_audio::update_draw_audio`（garden 模板）×2 + 2 loop stop；
//! `mode u8 @+0x58`。wp7 `update_water_audio_grid` 写 `WaterAudioGrid`；本系统消费 `WaterDrawEvent/
//! WaterEraseEvent`（mod.rs run_if）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `WaterAudioState`（O 名；4×AudioLoop；`mode u8 @+0x58`）。
#[derive(Resource, Debug, Default)]
pub struct WaterAudioState {
    /// 绘制/擦除 loop（O 数量）。
    pub loops: Vec<AudioLoop>,
    /// mode @+0x58（O 偏移；U 语义）。
    pub mode: u8,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `WaterDrawEvent`（O 名；run_if 消费者，water 域写入）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Default)]
pub struct WaterDrawEvent(pub Gap);

/// `WaterEraseEvent`（O 名）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Default)]
pub struct WaterEraseEvent(pub Gap);

/// `water_audio_init_system`（0x140873140，O）。
pub fn water_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let names: [&'static str; 4] = [
        "event:/SFX/Water/WaterDraw",
        "event:/SFX/Water/WaterErase",
        "event:/SFX/Water/IceDraw",
        "event:/SFX/Water/IceErase",
    ];
    let mut loops = Vec::new();
    for name in names {
        let desc = AudioLoopDesc::new(name, Some("editing_speed"), 3, 0x32);
        if let Ok(loop_) = core.instantiate_loop(&desc) {
            loops.push(loop_);
        }
    }
    commands.insert_resource(WaterAudioState {
        loops,
        mode: 0,
        _gap: Gap,
    });
}

/// `water_audio_system`（0x140873630，O；wp7 消费）。
pub fn water_audio_system(
    mut _state: ResMut<WaterAudioState>,
    mut _draw_events: EventReader<WaterDrawEvent>,
    mut _erase_events: EventReader<WaterEraseEvent>,
) {
    // WaterRaster 栅格编辑索引 → update_draw_audio（garden 模板）×2 → 2 loop stop（O）。
}
