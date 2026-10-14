//! systems/audio/trigger_audio.rs — 音频触发器系统（相机穿过触发盒的响度）。
//!
//! 反汇编证据（O）：
//! - `trigger_audio_init_system`（0x1409ea8d0）：`AudioCoreInner::instantiate_audio` ×4：
//!   `event:/SFX/Camera Tree Rustle`、`event:/Amb/High Wind`、`event:/SFX/Camera Whoosh`、
//!   `snapshot:/InForest`；`insert_resource<TriggerAudioState>`。
//! - `trigger_audio_system`（0x1409eafa0）：相机段对 `AudioTriggerLibrary` 全部盒
//!   `Aabb3::intersect_segment`；命中 → audibility +0.5、否则 -0.1，clamp [0,1]（`+0x34`）；
//!   audibility<=0 → 停 High Wind；音量 = 相机位移/阈值 ×0.0125 × 位置衰减，lerp 平滑（`+0x35`）；
//!   相机 z>=11 或 dist<=70 → 停 InForest 快照，否则 start + set_parameter；set_volume。
//! 见 contracts/audio-trigger.md。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::asset_libraries::audio_trigger_library::{AudioTriggerCollection, AudioTriggerLibrary};
use crate::resources::audio_resources::{AudioCore, AudioInstance};
use crate::typed_gap::Gap;

/// `TriggerAudioState`（O 名；4×AudioInstance + 相机缓存，O 偏移语义 I）。
#[derive(Resource, Debug, Default)]
pub struct TriggerAudioState {
    /// 4 实例（O：Camera Tree Rustle / High Wind / Camera Whoosh / InForest 快照）。
    pub instances: Vec<AudioInstance>,
    /// 相机缓存：`flag@+0x30、cached_pos@+0x31..+0x33、audibility f32@+0x34、
    /// volume f32@+0x35、byte@+0x36、state@+0x18、playing@+0x24`（O 偏移；U 语义）。
    pub camera: TriggerCameraCache,
    /// 阈值 f32 @+0x58（O：为 0 时系统早退）。
    pub threshold: f32,
    /// 其余（U）。
    pub _gap: Gap,
}

/// 相机/响度缓存（O 偏移；U 语义）。
#[derive(Clone, Copy, Debug, Default)]
pub struct TriggerCameraCache {
    /// @+0x30 flag（O）。
    pub valid: bool,
    /// @+0x31..+0x33 cached position（O）。
    pub cached_position: glam::Vec3,
    /// @+0x34 audibility 平滑值（O）。
    pub audibility: f32,
    /// @+0x35 volume 平滑值（O）。
    pub volume: f32,
    /// @+0x36 byte（O 存在；U 语义）。
    pub _byte: u8,
    /// @+0x18 state（O；U 语义）。
    pub _state: i32,
    /// @+0x24 playing（O）。
    pub playing: i32,
}

/// `trigger_audio_init_system`（0x1409ea8d0，O）。
pub fn trigger_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let names: [&'static str; 4] = [
        "event:/SFX/Camera Tree Rustle",
        "event:/Amb/High Wind",
        "event:/SFX/Camera Whoosh",
        "snapshot:/InForest",
    ];
    let mut instances = Vec::new();
    for name in names {
        if let Ok(inst) = core.instantiate_audio(name) {
            instances.push(inst);
        }
    }
    commands.insert_resource(TriggerAudioState {
        instances,
        camera: TriggerCameraCache::default(),
        threshold: 1.0,
        _gap: Gap,
    });
}

/// `trigger_audio_system`（0x1409eafa0，O）。
pub fn trigger_audio_system(
    mut _state: ResMut<TriggerAudioState>,
    _library: Res<AudioTriggerLibrary>,
) {
    // 相机段 × 全部触发盒 intersect_segment → audibility/volume 平滑 → 4 实例驱动（O）。
}
