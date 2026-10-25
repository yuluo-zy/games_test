//! country-core systems/audio：事件→FMOD 胶水的事件与状态类型（B 主基线，反汇编级证据草稿）
//!
//! 依据：evidence/multi-baseline/audio-events-analysis.md（2026-10-13 reva/radare2 精读）。
//! - 事件槽 stride 与槽内偏移来自 reva（B）与 radare2（RNE oracle）逐条一致的反汇编。
//! - 槽 @+0x0 为 bevy EventInstance 书签（依赖内部，复用不还原）；游戏 payload 从槽 @+0x8 起。
//! - 未确认字段保留 `unknown`/`_private`，不伪造。

use glam::Vec3;

// ---- 事件类型（register_event 注册，add_audio_events 证据顺序）----

/// 墙体音效事件（wall_audio_fmod）。
/// 事件槽 0x28；payload 从槽 @+0x8 起：
/// - kind u8 @+0x8（分派键，跳表 0x142ab4998）
/// - f32 @+0xc（部分瞬发 case 读取；unknown）
/// - position Vec3 @+0x10（x,y 以 u64 读、z @+0x18 u32）
/// - volume f32 @+0x1c（→ set_volume / instantiate 音量）
/// - byte @+0x20（request_play 以 r8d 传入，未使用）
pub struct PlayWallAudio {
    /// kind u8 @+8（分派键，跳表 0x142ab4998；0–23 有效）。
    pub kind: u8,
    /// f32 @+0xc（unknown 语义）。
    pub field_c: f32,
    /// 位置。
    pub position: Vec3,
    /// 音量（@+0x1c）。
    pub volume: f32,
    /// @+0x20 字节（unknown）。
    pub field_20: u8,
}

impl PlayWallAudio {
    /// kind 行为归类（证据：case 反汇编 request_play / instantiate_audio_w_position_and_volume）。
    /// 完整逐条 kind→case 映射见 evidence 第 3b 节；此处仅按 case 行为归类。
    pub fn kind_is_loop(&self) -> bool {
        matches!(
            self.kind,
            0 | 1 | 2 | 6 | 7 | 10 | 13 | 17 | 18 | 19 | 21
        )
    }
}

/// 建筑音效事件（building_audio）。
/// 事件槽 0x20；payload 从槽 @+0 起（无 bevy 书签，证据：filter_map 收集后元素 +0 仍为 id）：
/// - id u64 @+0（hashbrown 键，定位 0x2a8 构建条目）
/// - position Vec3 @+8（x,y u64 / z @+0x10 u32）
/// - editing_speed f32 @+0x14（set_parameter 参数）
/// - kind u8 @+0x18（分派键，表 0x142ce4c88/0x142ce4ca0）
pub struct PlayBuildingAudio {
    /// 构建/墙全局 id（u64，hash_one<RandomState,GlobalId> 键）。
    pub id: u64,
    /// 位置。
    pub position: Vec3,
    /// 编辑速度（set_parameter "editing_speed"）。
    pub editing_speed: f32,
    /// 种类（0–7，映射 event:/SFX/Shapes/…）。
    pub kind: u8,
}

/// 玩家自制楼梯音效命令（playermade_stairs_audio）。
/// 事件槽 0x38；payload 从槽 @+0x8 起：
/// - kind u8 @+0x8（跳表 0x142bad9f0，≤18）
/// - 字节 @+0x9/@+0xa/@+0xb
/// - f32 @+0xc/@+0x10/@+0x14/@+0x18
/// - position u64(x,y) @+0x1c / u32(z) @+0x24
/// - f32 @+0x28/@+0x2c
pub struct PlayStairsAudioCmd {
    pub kind: u8,
    pub b9: u8,
    pub ba: u8,
    pub bb: u8,
    /// f32 @+0xc（unknown，部分 case 读取）。
    pub field_c: f32,
    pub f10: f32,
    pub f14: f32,
    pub f18: f32,
    /// 位置。
    pub position: Vec3,
    /// f32 @+0x28（unknown）。
    pub field_28: f32,
    /// f32 @+0x2c（unknown）。
    pub field_2c: f32,
}

/// 树音效事件（tree_audio；process_tree_cmds 发送，tree-tool 侧证据完整）。
/// 事件槽 0x20；payload：kind u8 @+0x8（insert=0/move=1/delete=2）、in_water bool @+0x9、
/// flag bool @+0xa、height f32 @+0xc、position Vec3 @+0x10。
pub struct PlayTreeSoundCmd {
    pub kind: u8,
    pub in_water: bool,
    pub flag: bool,
    pub height: f32,
    pub position: Vec3,
}

// ---- 状态资源（insert_resource 注册）----

/// 墙体音效状态（wall_audio_init_system 装配，insert_resource<WallAudioState>）。
/// 布局证据（wall_audio_fmod 侧读取）：
/// - AudioLoopPlayer ×10 @+0x0/+0x40/+0x80/+0xc0/+0x100/+0x140/+0x180/+0x1c0/+0x200/+0x240
/// - 每 loop 的"应播"标志 @+0x38/+0x78/+0xb8/+0xf8/+0x138/+0x178/+0x1b8/+0x1f8/+0x238/+0x278
/// - 元素参数 Vec（ptr/cap/len，元素 0x30）@+0x48..、+0x88..、+0xc8..、+0x148..
/// - 音量调制 f32 @+0x280
#[derive(Default)]
pub struct WallAudioState {
    /// 10 个循环 player。
    pub loops: [AudioLoopPlayer; 10],
    /// 每 loop 应播标志（帧尾 start_if_not_playing/stop）。
    pub playing_flags: [bool; 10],
    /// 元素参数缓冲区（unknown 内部布局）。
    pub _element_params: Vec<[u8; 0x30]>,
    /// 音量调制（乘 powf 时间项）。
    pub volume_mod: f32,
}

/// 建筑音效状态（building_audio_init_system，insert_resource<BuildingAudioState>）。
/// 内部布局 largely unknown（事件系统按 id 查 0x2a8 条目，不常驻状态）。
pub struct BuildingAudioState {
    pub _private: [u8; 0],
}

/// 楼梯音效状态（stairs_audio_init_system，insert_resource<StairsAudioState>）。
/// 6 个 AudioLoopPlayer @+0x0/+0x38/+0x70/+0xa8/+0xe0/+0x118（stride 0x38）。
#[derive(Default)]
pub struct StairsAudioState {
    pub loops: [AudioLoopPlayer; 6],
}

// ---- FMOD 胶水对象（audio_resources / wall_audio_utils，字段为证据级）----

/// 循环播放器（wall_audio_utils::AudioLoopPlayer；request_play 反编译证据）。
/// 布局：+0x0 instances Vec ptr；+0x8 cap；+0x10 cap；+0x20 u32=1000000000；
/// +0x28 count；+0x30 u32（=AudioLoopDesc.max，墙为 0x32）；+0x34/+0x35/+0x36 字节；
/// +0x38 playing 标志。
#[derive(Default)]
pub struct AudioLoopPlayer {
    /// AudioInstance 数组（Vec，元素 0x30）。
    pub instances: Vec<AudioInstance>,
    /// count（request_play 索引，须 < cap）。
    pub count: u64,
    /// 最大并发/循环数（instantiate_loop 写 desc+0x18；墙 0x32、楼梯同）。
    pub max: u32,
    /// playing 标志（request_play 置 1；帧尾据此 start/stop）。
    pub playing: bool,
}

/// 单实例（audio_resources::impl$10）。字段证据：
/// +0x0 判别（1=存在），+0x30 stride；set_position/set_parameter/set_volume 作用于内部 FMOD 事件。
#[derive(Default)]
pub struct AudioInstance {
    /// 判别（1 = Some）。
    pub present: u32,
    /// FMOD EventInstance（libfmod 边界，不还原）。
    pub _fmod: [u8; 0x20],
}

/// 音频核心（country_core::resources::audio_resources::AudioCore）。
/// 证据：+0x10 AudioCoreInner（instantiate_audio/update 的目标）；instantiate_loop 的 self。
#[derive(Default)]
pub struct AudioCore {
    pub _private: [u8; 0],
}

/// AudioCore 内部（instantiate_audio(self) → Option<AudioInstance>）。
pub struct AudioCoreInner {
    pub _private: [u8; 0],
}
