//! systems/audio/firstperson_steps.rs — 第一人称脚步声。
//!
//! 反汇编证据（O）：`firstperson_steps_init_system`（行 29，`(Res<AudioCore>, Commands)`）
//! `instantiate_loop`（`event:/SFX/Player/Walking`）；`firstperson_steps`（0x140a889b0）：
//! 全局参数 `FirstPersonOff`（O 串）+ 地面类型参数（`GroundType::to_fmod_param` 行 128）；
//! `GroundType==3` 且相机运动时双线性采样地形栅格定材质（RasterBufferRef<512>）；`GroundType==0`
//! 播放行走 loop 否则停止。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// 地面类型（O 名；变体 U。`to_fmod_param` 行 128 映射参数值）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GroundType(pub u8);

impl GroundType {
    /// `GroundType::to_fmod_param`（行 128，O）。
    pub fn to_fmod_param(self) -> f32 {
        self.0 as f32
    }
}

/// `FirstPersonViewAudioState`（O 名；1×AudioLoop：行走）。
#[derive(Resource, Debug, Default)]
pub struct FirstPersonViewAudioState {
    /// 行走 loop（O）。
    pub walk_loop: Option<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `firstperson_steps_init_system`（行 29，O）。
pub fn firstperson_steps_init_system(
    core: Res<AudioCore>,
    mut commands: bevy_ecs::system::Commands,
) {
    let desc = AudioLoopDesc::new("event:/SFX/Player/Walking", None, 3, 0x32);
    let walk_loop = core.instantiate_loop(&desc).ok();
    commands.insert_resource(FirstPersonViewAudioState { walk_loop, _gap: Gap });
}

/// `firstperson_steps`（0x140a889b0，O）。
pub fn firstperson_steps(
    _core: Res<AudioCore>,
    mut _state: ResMut<FirstPersonViewAudioState>,
) {
    // 全局参数 FirstPersonOff + 地面类型参数（O）；GroundType==0 → 行走 loop start/stop。
}
