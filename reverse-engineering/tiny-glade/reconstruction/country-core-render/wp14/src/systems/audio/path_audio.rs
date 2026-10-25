//! systems/audio/path_audio.rs — 路径绘制音频（update_draw_audio 共享辅助）。
//!
//! 反汇编证据（O）：`path_audio_init_system`（0x1409d08d0）`instantiate_loop`
//! （`event:/SFX/CobblestoneDraw`）+ `instantiate_loop_snappy`（行 65）；
//! `path_audio_system`（0x1409d0f80）`update_draw_audio`（模板 0x14166d890/0x14166d9f0，
//! 行 69）×N；`brush_editing_speed_01`（行 234，O，water/garden/terrain/tree/wall 共享内联）。
//! `update_draw_audio`：按"绘制栅格编辑索引 + 相机位置"驱动绘制循环（应播标志 → start/stop）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `PathAudioState`（O 名；4×AudioLoop）。
#[derive(Resource, Debug, Default)]
pub struct PathAudioState {
    /// 绘制 loop（O 数量）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `path_audio_init_system`（0x1409d08d0，O）。
pub fn path_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let mut loops = Vec::new();
    let desc = AudioLoopDesc::new("event:/SFX/CobblestoneDraw", Some("editing_speed"), 3, 0x32);
    if let Ok(loop_) = core.instantiate_loop(&desc) {
        loops.push(loop_);
    }
    let snappy = AudioLoopDesc::new("event:/SFX/CobblestoneDraw", None, 3, 0x32);
    if let Ok(loop_) = core.instantiate_loop_snappy(&snappy) {
        loops.push(loop_);
    }
    commands.insert_resource(PathAudioState { loops, _gap: Gap });
}

/// `path_audio_system`（0x1409d0f80，O）。
pub fn path_audio_system(
    mut _state: ResMut<PathAudioState>,
    mut _events: EventReader<crate::typed_gap::GapEventPlaceholder>,
) {
    // update_draw_audio ×N（行 69，O）。
}

/// `brush_editing_speed_01`（行 234，O）：画笔编辑速度 → [0,1] 共享辅助。
pub fn brush_editing_speed_01(_speed: f32) -> f32 {
    0.0
}

/// `update_draw_audio`（行 69，O）：按编辑栅格索引驱动一个绘制 loop。
pub fn update_draw_audio<F: FnMut()>(_loop_: &mut AudioLoop, _f: &mut F) {
    // 应播标志 → start_if_not_playing / stop（O）。
}
