//! country-core systems/audio：playermade_stairs_audio 的 kind 逐条表 + terrain_stairs_audio（B 主基线，反汇编级证据草稿）
//!
//! 依据：evidence/multi-baseline/audio-extra-analysis.md §3a/§3b（2026-10-13 reva audio-extra-owner +
//! radare2 RNE oracle；跳表 0x142bad9f0 重算）。
//! - playermade：事件槽 stride 0x38、kind @+0x8；`cmp r12, 0x12; ja → 忽略`，kind 0–18。
//! - 循环类 → 帧首已 stop 的 6 个 loop 之一 start_if_not_playing + "stair_width" 参数；
//!   瞬发类 → instantiate_audio + set_position + 参数分发。
//! - terrain_stairs：事件 →(SSBO 状态比对)→ instantiate_audio("StairDraw_OneShot") + set_parameter
//!   + start_if_not_playing。FMOD（libfmod）调用只标边界 `[fmod]`。

use crate::audio::events::{AudioCore, AudioLoopPlayer, PlayStairsAudioCmd};
use glam::Vec3;

/// playermade_stairs 的 kind 分派（跳表 0x142bad9f0）。
pub enum StairKind {
    /// 循环 loop（索引 = init 顺序 0..5；帧首已 stop，这里 start_if_not_playing + "stair_width"）。
    Loop(usize),
    /// 瞬发（事件名，(ptr,len) 已知；后续 set_position + set_parameter 参数分发）。
    OneShot(&'static str),
    /// 忽略（kind 16；kind 17/18 复用 StairSupportAppear）。
    Ignored,
}

/// kind 0–18 逐条表（kind 19+ 越界忽略）。
pub const STAIR_KIND_TABLE: [StairKind; 19] = [
    StairKind::Loop(0),                    // 0  StairMove
    StairKind::Loop(1),                    // 1  StairChangeHeight
    StairKind::Loop(2),                    // 2  StairChangeWidth
    StairKind::OneShot("event:/SFX/Stairs/StairPlace"),             // 3
    StairKind::OneShot("event:/SFX/Stairs/StairDelete"),            // 4
    StairKind::OneShot("event:/SFX/Stairs/StairsMerge"),            // 5
    StairKind::OneShot("event:/SFX/Stairs/StairsMergeConfirm"),     // 6
    StairKind::Loop(3),                    // 7  LadderMove
    StairKind::OneShot("event:/SFX/Stairs/LadderAppear"),           // 8
    StairKind::Loop(4),                    // 9  LadderChangeHeight
    StairKind::Loop(5),                    // 10 LadderChangeWidth
    StairKind::OneShot("event:/SFX/Stairs/LadderPlace"),           // 11
    StairKind::OneShot("event:/SFX/Stairs/StepsAppear"),           // 12
    StairKind::OneShot("event:/SFX/Stairs/WalkwayAppear"),         // 13
    StairKind::OneShot("event:/SFX/Stairs/StairSupportAppear"),    // 14
    StairKind::OneShot("event:/SFX/Stairs/StairPillarAppear"),     // 15
    StairKind::Ignored,                                            // 16（回循环顶）
    StairKind::OneShot("event:/SFX/Stairs/StairSupportAppear"),    // 17（同 14）
    StairKind::OneShot("event:/SFX/Stairs/StairSupportAppear"),    // 18（同 14）
];

pub fn stair_kind(kind: u8) -> &'static StairKind {
    STAIR_KIND_TABLE.get(kind as usize).unwrap_or(&StairKind::Ignored)
}

/// playermade stairs 参数名表（0x142bad818 起连续切片）。
pub const STAIR_PARAM_NAMES: [&str; 11] = [
    "stair_width",          // 0x142bad818, len 0xb
    "size",                 // 0x142bad823, len 0x4
    "editing_speed",        // 0x142bad827
    "extend_amount",        // 0x142bad834
    "h_offset",             // 0x142bad841
    "avg_arch_supports",    // 0x142bad849
    "height_above_ground",  // 0x142bad86e
    "in_water",             // 0x142bad876
    "stair_edge_length",    // 0x142bad887
    "is_stub_preview",      // 0x142bad896
    "move_all",             // 0x142bad896 后（长度/精确边界未全部钉死）
];

/// stairs_audio_system（VA 0x140f880a0）骨架：帧首 stop 6 loop，事件 kind 分派。
pub fn stairs_audio_system(
    loops: &mut [AudioLoopPlayer; 6], // [evidence] state+0x0/+0x38/+0x70/+0xa8/+0xe0/+0x118
    _elements: &AudioCore,
    ev: &PlayStairsAudioCmd,
) {
    // 帧首：6 个 loop stop。
    for l in loops.iter_mut() {
        // [fmod] stop(l);
    }
    match stair_kind(ev.kind) {
        StairKind::Loop(i) => {
            // [fmod] start_if_not_playing(&loops[i]);
            // [fmod] set_parameter(loop_elem(i), "stair_width", inverse_lerp(...));
        }
        StairKind::OneShot(_name) => {
            // [fmod] instantiate_audio(&out, core+0x10, name, len)
            // [fmod] set_position(out, ev.position)       // 事件 @+0x1c/+0x24
            // [fmod] set_parameter(out, "stair_width", ...)
            // [fmod] start_if_not_playing(out)
            // 二次参数分发（跳表 0x142bada3c，kind≤0xa）→ size/extend_amount/h_offset/...
        }
        StairKind::Ignored => {}
    }
}

// ===== terrain_stairs_audio（terrain_stairs_audio_system，VA 0x141462dc0）=====

/// 事件名常量（RNE .data）：地形楼梯绘制一次音效。
pub const TERRAIN_STAIR_DRAW_ONESHOT: &str = "event:/SFX/StairDraw_OneShot"; // 0x142c62e70

/// 模式：`[AppMode]==5` 时直接返回；SuppressPathSound 事件 + SSBO BufferState id 比对；
/// 相同 → AudioLoop::stop；不同 → instantiate_audio + start_if_not_playing + set_parameter。
pub fn terrain_stairs_audio_system(
    _app_mode: u32,          // [evidence] [arg1]+0 == 5 → early return
    _stairs_counter: &[u8],  // [evidence] 内部 StairsCounter 状态（retain<AudioInstance> + get_playback_state）
    _ssbo: &AudioCore,       // [evidence] FrozenMap::get<SsboId, BufferState>（0x140969720）
    _position: Vec3,         // [evidence] 位置
    _suppressed: bool,       // [evidence] SuppressPathSound 事件（unknown 精确字段）
) {
    // 若 SSBO id == 当前 → stop；否则 instantiate_audio(TERRAIN_STAIR_DRAW_ONESHOT) +
    // set_parameter + start_if_not_playing，压入实例 Vec。
    let _ = TERRAIN_STAIR_DRAW_ONESHOT;
}

/// 事件字段布局（PlayStairsAudioCmd，槽 stride 0x38，payload @+0x8 起）：
/// kind u8 @+8；字节 @+9/@+a/@+b；f32 @+c/@+0x10/@+0x14/@+0x18；
/// position u64(x,y) @+0x1c / u32(z) @+0x24；f32 @+0x28/@+0x2c。
#[allow(dead_code)]
pub fn stair_position(_ev: &PlayStairsAudioCmd) -> Vec3 {
    Vec3::ZERO // [evidence] @+0x1c/+0x24
}
