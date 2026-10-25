//! systems/audio/building_audio.rs — 建筑音频（PlayBuildingAudio → FMOD 胶水）。
//!
//! 反汇编证据（O，audio-events-analysis §4）：
//! - `building_audio_init_system`（0x1417d21a0）：7×`AudioCore::instantiate_loop`
//!   `event:/SFX/Building/{BuildingResize,BuildingRotateRectangle,BuildingRotateCircle,
//!   BuildingChangeHeight,BuildingMove}` + `event:/SFX/PlasterChangeLoop` + `BuildingOverlapMove`；
//!   参数 `editing_speed`、`BuildingSize`；`insert_resource<BuildingAudioState>`。
//! - `building_audio`（0x1417d2ad0）：`PlayBuildingAudio`（stride 0x20：id@+0、pos@+8、
//!   editing_speed@+0x14、kind@+0x18）→ kind 0–7 瞬发
//!   `event:/SFX/Shapes/{DetachFromGround,AttachToGround,PeakReached}` → instantiate_audio →
//!   set_position → set_parameter(editing_speed) + set_parameter(BuildingSize, inverse_lerp(area))
//!   → start_if_not_playing；`get_building_size_01`（0x1417d3f10，行 271）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayBuildingAudio;
use crate::resources::audio_resources::{AudioCore, AudioInstance};
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `BuildingAudioState`（O 名；7×AudioLoop，I 布局）。
#[derive(Resource, Debug, Default)]
pub struct BuildingAudioState {
    /// 7 个循环 loop（O 数量；布局 U）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `building_audio_init_system`（0x1417d21a0，O）。
pub fn building_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let names: [&'static str; 7] = [
        "event:/SFX/Building/BuildingResize",
        "event:/SFX/Building/BuildingRotateRectangle",
        "event:/SFX/Building/BuildingRotateCircle",
        "event:/SFX/Building/BuildingChangeHeight",
        "event:/SFX/Building/BuildingMove",
        "event:/SFX/PlasterChangeLoop",
        "event:/SFX/Building/BuildingOverlapMove",
    ];
    let mut loops = Vec::new();
    for name in names {
        let desc = AudioLoopDesc::new(name, Some("editing_speed"), 3, 0x32);
        if let Ok(loop_) = core.instantiate_loop(&desc) {
            loops.push(loop_);
        }
    }
    commands.insert_resource(BuildingAudioState { loops, _gap: Gap });
}

/// `building_audio`（0x1417d2ad0，O）：`PlayBuildingAudio` → FMOD 瞬发。
pub fn building_audio(
    mut _state: ResMut<BuildingAudioState>,
    mut events: EventReader<PlayBuildingAudio>,
    _core: Res<AudioCore>,
) {
    for ev in events.read() {
        let _kind = ev.kind;
        // kind 0–7 → 瞬发串（DetachFromGround/AttachToGround/PeakReached，O）；
        // instantiate_audio → set_position(pos) → set_parameter(editing_speed) +
        // set_parameter("BuildingSize", inverse_lerp(area)) → start_if_not_playing（libfmod 边界）。
        let _ = _kind;
    }
}

/// `get_building_size_01`（0x1417d3f10，行 271，O）：构建面积 → [0,1]。
pub fn get_building_size_01(_area: f32) -> f32 {
    0.0
}

/// `set_parameters`（行 214，O）：给活动实例设编辑速度/尺寸参数。
pub fn set_parameters(_instance: &AudioInstance, _editing_speed: f32, _size_01: f32) {
    let _ = _instance;
    let _ = _editing_speed;
    let _ = _size_01;
}
