//! country-core systems/audio：wall_audio_fmod 的 PlayWallAudio kind 0–23 逐条表（B 主基线，反汇编级证据草稿）
//!
//! 依据：evidence/multi-baseline/audio-extra-analysis.md §1（2026-10-13 reva audio-extra-owner +
//! radare2 RNE oracle；跳表 0x142ab4998 逐字节确认，目标 = base + sign_ext(dword[base+kind*4])）。
//! - 事件槽 stride 0x28；payload 从槽 @+0x8 起（@+0..+8 bevy 书签）。
//! - 循环类 → AudioLoopPlayer::request_play；瞬发类 → instantiate_audio_w_position_and_volume。
//! - 未确认字段/常量保留注释，不伪造。FMOD（libfmod）调用只标边界 `[fmod]`。

use crate::events::{AudioLoopPlayer, AudioCore, PlayWallAudio};
use glam::Vec3;

/// wall_audio_fmod 的 kind 分派表（证据：RNE .data 跳表 + case 反汇编）。
/// 每项：kind → (类别, 事件名, 参数)。
pub enum WallKind {
    /// 循环 loop：直接 request_play，无参数。
    Loop(usize),
    /// 循环 loop + set_parameter(elem, 参数名, 值)。
    LoopParam {
        /// loop 索引（init 顺序；也即 loop player 偏移 / 0x40）。
        loop_index: usize,
        /// 参数名（.data 切片）。
        param: &'static str,
        /// inverse_lerp 范围（min, max），输入为事件 +0xc 的 f32。
        inverse_lerp_bounds: (f32, f32),
    },
    /// 循环 loop + 布尔参数（事件 +0x9 非零 → 1.0 else 0.0）。
    LoopBoolParam {
        loop_index: usize,
        param: &'static str,
        /// 事件字段偏移（字节）。
        flag_offset: usize,
    },
    /// 瞬发：instantiate_audio_w_position_and_volume(事件名)。
    OneShot(&'static str),
    /// 忽略（跳表越界 / 不处理）。
    Ignored,
}

/// kind 0–23 逐条表（全部 24 项；证据见 evidence §1）。
/// loop 索引对应 WallAudioState 的 AudioLoopPlayer ×10（init 顺序）。
pub const WALL_KIND_TABLE: [WallKind; 24] = [
    WallKind::Loop(0),                                          // 0  Wall Draw
    WallKind::LoopParam {                                       // 1  Wall Change Height
        loop_index: 1,
        param: "WallHeight",        // 0x142ab41c8, len 0xa
        inverse_lerp_bounds: (0.5, 14.0),   // [0x142925870]=0.5, [0x142a7de00]=14.0
    },
    WallKind::LoopParam {                                       // 2  FenceExtend
        loop_index: 2,
        param: "FenceLength",       // 0x142ab41d2, len 0xb
        inverse_lerp_bounds: (8.0, 50.0),   // [0x142934604]=8.0, [0x1429cd070]=50.0
    },
    WallKind::OneShot("event:/SFX/FenceAppear"),                // 3
    WallKind::OneShot("event:/SFX/FenceMerge"),                 // 4
    WallKind::OneShot("event:/SFX/FenceRearrange"),             // 5
    WallKind::Loop(4),                                          // 6  StoneArchExtend
    WallKind::LoopParam {                                       // 7  LintelExtend
        loop_index: 3,
        param: "FenceLength",       // 0x142ab41d2, len 0xb
        inverse_lerp_bounds: (3.0, 15.0),   // [0x1429cd078]=3.0, [0x1429cd06c]=15.0
    },
    WallKind::OneShot("event:/SFX/LintelAppear"),               // 8
    WallKind::OneShot("event:/SFX/LintelRearrange"),            // 9  (0x142ab4238, 0x1a)
    WallKind::LoopBoolParam {                                   // 10 WideningGate
        loop_index: 5,
        param: "HasBricks",         // 0x142ab4252, len 0x9
        flag_offset: 0x9,
    },
    WallKind::OneShot("event:/SFX/LintelMerge"),                // 11
    WallKind::OneShot("event:/SFX/StoneArchAppear"),            // 12
    WallKind::Loop(6),                                          // 13 PlatformStairsExtend
    WallKind::OneShot("event:/SFX/PillarStoneAppear"),          // 14
    WallKind::OneShot("event:/SFX/PillarWoodAppear"),           // 15
    WallKind::OneShot("event:/SFX/LintelRearrange"),            // 16（与 kind 9 同名同处理）
    WallKind::OneShot("event:/SFX/GateAppear"),                 // 17
    WallKind::Loop(7),                                          // 18 SewerGateWiden
    WallKind::OneShot("event:/SFX/PlatformStairsAppear"),       // 19
    WallKind::Loop(9),                                          // 20 FenceMove
    WallKind::OneShot("event:/SFX/WallSplit"),                  // 21
    WallKind::OneShot("event:/SFX/SewerGateAppear"),            // 22
    WallKind::Loop(8),                                          // 23 FenceHeight
];

/// 取 kind 的行为（kind ≥ 24 视为忽略；跳表只覆盖 0–23）。
pub fn wall_kind(kind: u8) -> &'static WallKind {
    WALL_KIND_TABLE.get(kind as usize).unwrap_or(&WallKind::Ignored)
}

/// 证据辅助：由事件构造 FMOD 调用（wall_audio_fmod，VA 0x140950920）。
/// 循环类：request_play(loop, pos, 事件+0x1c→set_volume, 事件+0x20, editing_speed)。
/// 瞬发类：instantiate_audio_w_position_and_volume(core, name, pos, 事件+0x1c)。
pub fn play_wall_kind(
    _core: &AudioCore,
    loops: &mut [AudioLoopPlayer; 10],
    _element_params: &mut [Vec<[u8; 0x30]>], // [evidence] 各 loop 关联参数 Vec（+0x48/+0x88/+0xc8/+0x148）
    ev: &PlayWallAudio,
    _normalized_volume: f32,
) {
    match wall_kind(ev.kind) {
        WallKind::Loop(i) | WallKind::LoopParam { loop_index: i, .. }
        | WallKind::LoopBoolParam { loop_index: i, .. } => {
            // [fmod] request_play(&loops[*i], ev.position, ev.volume, editing_speed)
            let _ = &loops[*i];
            // 参数类再按表 set_parameter(elem, param, value)；value 语义见 evidence §1。
        }
        WallKind::OneShot(name) => {
            // [fmod] instantiate_audio_w_position_and_volume(_core, name, ev.position, ev.volume)
            let _ = name;
        }
        WallKind::Ignored => {}
    }
}

// ---- 瞬发事件名常量（RNE .data，供表外引用核对）----
pub const WALL_ONESHOT_NAMES: [&str; 14] = [
    "event:/SFX/FenceAppear",
    "event:/SFX/FenceMerge",
    "event:/SFX/FenceRearrange",
    "event:/SFX/LintelAppear",
    "event:/SFX/LintelRearrange",
    "event:/SFX/LintelMerge",
    "event:/SFX/StoneArchAppear",
    "event:/SFX/PillarStoneAppear",
    "event:/SFX/PillarWoodAppear",
    "event:/SFX/GateAppear",
    "event:/SFX/PlatformStairsAppear",
    "event:/SFX/WallSplit",
    "event:/SFX/SewerGateAppear",
    "event:/SFX/LintelRearrange",
];

// ---- 帧尾（0x14095145e 起）：按"应播"标志 start_if_not_playing/stop，音量调制 ----
pub fn wall_frame_tail(_loops: &mut [AudioLoopPlayer; 10], _playing_flags: &mut [bool; 10]) {
    // [fmod] start_if_not_playing / stop per flag；清标志。
    // [evidence] [state+0x280] *= powf(0.1, 5.0) 时间调制（0x142925984=0.1、0x142925988=5.0）。
}

/// 位置（事件槽 @+0x10 的 Vec3；x,y 以 u64 读、z @+0x18 u32）。
pub fn wall_event_position(_ev: &PlayWallAudio) -> Vec3 {
    Vec3::ZERO // [evidence] @+0x10
}
