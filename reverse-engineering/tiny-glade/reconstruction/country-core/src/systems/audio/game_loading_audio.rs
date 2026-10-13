//! systems/audio/game_loading_audio.rs — 加载界面音频。
//!
//! 反汇编证据（O）：`init_game_loading_audio`（0x140b1ae70）创建并立即 start
//! `snapshot:/Loading`；`loading_audio`（0x140b1b1b0）：加载进行（栅格编辑非空）时播放
//! `event:/UI/Loading`（瞬发），完成后停止 snapshot（状态→2、清标志），1s 定时器二次保护停止。
//! `GameLoadingAudioState`（O 偏移）：`{loading_snapshot: AudioInstance @0, flag @+0x30,
//! timer f32 @+0x34}`；`impl$0::default`（行 11）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::{AudioCore, AudioInstance};
use crate::typed_gap::Gap;

/// `GameLoadingAudioState`（O 名/偏移；`load_audio` 装配）。
#[derive(Resource, Debug, Default)]
pub struct GameLoadingAudioState {
    /// `snapshot:/Loading` 实例 @0（O）。
    pub loading_snapshot: Option<AudioInstance>,
    /// 快照播放标志 @+0x30（O 偏移；U 语义）。
    pub snapshot_active: bool,
    /// 停止定时器 f32 @+0x34（O 偏移）。
    pub timer: f32,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `init_game_loading_audio`（0x140b1ae70，O）。
pub fn init_game_loading_audio(
    core: Res<AudioCore>,
    mut state: ResMut<GameLoadingAudioState>,
) {
    let inst = core.instantiate_audio("snapshot:/Loading");
    match inst {
        Ok(inst) => {
            inst.start(); // libfmod 边界（O）。
            state.loading_snapshot = Some(inst);
            state.snapshot_active = true;
        }
        Err(_) => {}
    }
}

/// `loading_audio`（0x140b1b1b0，O）。
pub fn loading_audio(
    mut _state: ResMut<GameLoadingAudioState>,
    _core: Res<AudioCore>,
) {
    // 加载进行 → `event:/UI/Loading` 瞬发（O）；完成 → 停 snapshot、1s 定时器保护（O）。
}
