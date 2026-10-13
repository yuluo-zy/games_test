//! systems/audio/water_ambience_audio.rs — 水面环境音频（WaterAudioGrid 逐格）。
//!
//! 反汇编证据（O）：`water_ambience_audio_init_system`（0x1409ebf20，行 49）
//! `AudioLoopDesc::new` + `instantiate_loop`（`event:/SFX/Water/Amb`）；`impl$0::load_index_unchecked`
//! （0x1409ebe80，O）；`water_ambience_audio_system`（0x1409ecea0）：演示模式禁用
//! （`_DEV_OVERRIDE_IS_DEMO_MODE`）；逐 `WaterAudioGrid` 格（stride 0x18）：有水面 →
//! `start_if_not_playing + set_position（相机相对）+ set_volume`，无 → `stop`。
//! wp7 `update_water_audio_grid` 写 24B/格 网格（run_if 含 `Res<WaterAudioGrid>` + `Res<MainCamera>`）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `WaterAmbienceAudio`（O 名；`{loops: Vec<AudioLoop>, …}`，stride 0x38 证据）。
#[derive(Resource, Debug, Default)]
pub struct WaterAmbienceAudio {
    /// 逐格环境 loop（O 数量 = 网格容量上限）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `WaterAudioGrid`（O 名，wp7 提供；24B/格）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WaterAudioGrid(pub Gap);

/// `water_ambience_audio_init_system`（0x1409ebf20，O）。
pub fn water_ambience_audio_init_system(
    core: Res<AudioCore>,
    mut commands: bevy_ecs::system::Commands,
) {
    let desc = AudioLoopDesc::new("event:/SFX/Water/Amb", None, 3, 0x32);
    let mut loops = Vec::new();
    if let Ok(loop_) = core.instantiate_loop(&desc) {
        loops.push(loop_);
    }
    commands.insert_resource(WaterAmbienceAudio { loops, _gap: Gap });
}

/// `water_ambience_audio_system`（0x1409ecea0，O）。
pub fn water_ambience_audio_system(
    mut _state: ResMut<WaterAmbienceAudio>,
    _grid: Res<WaterAudioGrid>,
) {
    // 逐格：有水面 → start_if_not_playing + set_position + set_volume；无 → stop（O）。
}

/// `impl$0::load_index_unchecked`（0x1409ebe80，O）。
pub fn load_index_unchecked(_index: u32) {
    let _ = _index;
}
