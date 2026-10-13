//! WP14 — country-core audio content domain（B 版本）
//!
//! 恢复范围（业务逻辑→骨架实现，差分仅作裁决）：
//!   - `resources::audio_resources`   AudioCore / AudioCoreInner / AudioInstance / Bank /
//!                                    DelayedBankLoader / AudioSamplesLoaded(Promise) 状态
//!   - `resources::audio_settings`    AudioSettings（GameConfig 内嵌，RON serde）
//!   - `startup::load_audio`          银行样例等待 + AudioTriggerLibrary 等装配
//!   - `startup::load_glade_audio`    GladeAudio（预览 glade 音乐/环境装载）
//!   - `asset_libraries::audio_trigger_library`  触发盒空间库（trigger→sound 映射）
//!   - `systems::audio/*`             全部音频 ECS 系统（事件→FMOD 胶水；31 模块）
//!   - `utils::audio_utils`           AudioLoopDesc / AudioLoop（池化循环播放器）
//!
//! 方法：PDB 骨架（enriched/structures）+ Ghidra 反编译（evidence/audio*-result.json，
//! worker wp14-audio）+ 复用 `evidence/multi-baseline/audio-events-analysis.md`（wall/
//! building/stairs/music 精读）。全部 implemented_unverified：无 runtime 验收；合同是
//! 当前可审阅依据。FMOD/libfmod 为 REUSED 依赖（S18：恢复游戏胶水，不重新实现 FMOD），
//! 只以注释标注 libfmod 调用边界。

#![allow(dead_code, unused_imports, clippy::too_many_arguments)]

extern crate alloc;

pub mod asset_libraries;
pub mod resources;
pub mod startup;
pub mod systems;
pub mod utils;

pub mod typed_gap;

/// 音频事件类型（`mod.rs add_audio_events` 16×register_event，O；见 contracts/audio-systems.md）。
pub mod audio_events {
    use crate::typed_gap::Gap;

    /// PlayTerrainSound（register_event #1）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct PlayTerrainSound(pub Gap);
    /// SuppressPathSound（#2）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct SuppressPathSound(pub Gap);
    /// PlayReplayAudio（#3）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct PlayReplayAudio(pub Gap);
    /// PlayDecoratorSoundCmd（#4）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct PlayDecoratorSoundCmd(pub Gap);
    /// PlayStairsAudioCmd（#5；槽 stride 0x38，字段 O，见 audio-systems.md §4）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq)]
    pub struct PlayStairsAudioCmd {
        /// kind u8 @+0x8（O）。
        pub kind: u8,
        /// position @+0x1c/+0x24（O）。
        pub position: glam::Vec3,
        /// 其余 f32 @+0xc/+0x10/+0x14/+0x18/@+0x28/@+0x2c（O 偏移，U 语义）。
        pub _gap: [Gap; 3],
    }
    impl Default for PlayStairsAudioCmd {
        fn default() -> Self {
            Self {
                kind: 0,
                position: glam::Vec3::ZERO,
                _gap: [Gap, Gap, Gap],
            }
        }
    }
    /// PlayWallUiSound（#6）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct PlayWallUiSound(pub Gap);
    /// PlayWallAudio（#7；槽 stride 0x28：kind@+8、position@+0x10、volume f32@+0x1c）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq)]
    pub struct PlayWallAudio {
        /// kind u8 @+0x8（O）。
        pub kind: u8,
        /// position Vec3 @+0x10（O）。
        pub position: glam::Vec3,
        /// volume f32 @+0x1c（O）。
        pub volume: f32,
        /// 其余（@+0xc f32、@+0x20 byte，O 偏移 U 语义）。
        pub _gap: Gap,
    }
    impl Default for PlayWallAudio {
        fn default() -> Self {
            Self {
                kind: 0,
                position: glam::Vec3::ZERO,
                volume: 1.0,
                _gap: Gap,
            }
        }
    }
    /// RequestWallMaterialAudio（#8）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct RequestWallMaterialAudio(pub Gap);
    /// WallBendAudioParamReset（#9，mod.rs 命名实体）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct WallBendAudioParamReset(pub Gap);
    /// PlayTimberframeSound（#10）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct PlayTimberframeSound(pub Gap);
    /// PlaySheepReactSound（#11）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct PlaySheepReactSound(pub Gap);
    /// PlayMenuUiSound（#12）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct PlayMenuUiSound(pub Gap);
    /// PlayBuildingAudio（#13；槽 stride 0x20：id@+0、position@+8、editing_speed@+0x14、kind@+0x18）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq)]
    pub struct PlayBuildingAudio {
        /// 构建条目 id（hashbrown 键，O）。
        pub id: u64,
        /// position Vec3 @+8（O）。
        pub position: glam::Vec3,
        /// editing_speed f32 @+0x14（O）。
        pub editing_speed: f32,
        /// kind u8 @+0x18（O）。
        pub kind: u8,
    }
    impl Default for PlayBuildingAudio {
        fn default() -> Self {
            Self {
                id: 0,
                position: glam::Vec3::ZERO,
                editing_speed: 1.0,
                kind: 0,
            }
        }
    }
    /// PlayElevationSupportSoundCmd（#14）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct PlayElevationSupportSoundCmd(pub Gap);
    /// PlayClutterSoundCmd（#15）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct PlayClutterSoundCmd(pub Gap);
    /// PlayTreeSoundCmd（#16；tree_audio 消费，树工具写入）。
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct PlayTreeSoundCmd(pub Gap);
}
