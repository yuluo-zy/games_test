//! country-core systems/audio：wall_audio_fmod 的 PlayWallAudio kind 0–23 逐条表（B 主基线，反汇编级证据草稿）
//!
//! 依据：evidence/multi-baseline/audio-extra-analysis.md §1（2026-10-13 reva audio-extra-owner +
//! radare2 RNE oracle；跳表 0x142ab4998 逐字节确认，目标 = base + sign_ext(dword[base+kind*4])）。
//! - 事件槽 stride 0x28；payload 从槽 @+0x8 起（@+0..+8 bevy 书签）。
//! - 循环类 → AudioLoopPlayer::request_play；瞬发类 → instantiate_audio_w_position_and_volume。
//! - 未确认字段/常量保留注释，不伪造。FMOD（libfmod）调用只标边界 `[fmod]`。

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
    WallKind::Loop(0), // 0  Wall Draw
    WallKind::LoopParam {
        // 1  Wall Change Height
        loop_index: 1,
        param: "WallHeight",              // 0x142ab41c8, len 0xa
        inverse_lerp_bounds: (0.5, 14.0), // [0x142925870]=0.5, [0x142a7de00]=14.0
    },
    WallKind::LoopParam {
        // 2  FenceExtend
        loop_index: 2,
        param: "FenceLength",             // 0x142ab41d2, len 0xb
        inverse_lerp_bounds: (8.0, 50.0), // [0x142934604]=8.0, [0x1429cd070]=50.0
    },
    WallKind::OneShot("event:/SFX/FenceAppear"),    // 3
    WallKind::OneShot("event:/SFX/FenceMerge"),     // 4
    WallKind::OneShot("event:/SFX/FenceRearrange"), // 5
    WallKind::Loop(4),                              // 6  StoneArchExtend
    WallKind::LoopParam {
        // 7  LintelExtend
        loop_index: 3,
        param: "FenceLength",             // 0x142ab41d2, len 0xb
        inverse_lerp_bounds: (3.0, 15.0), // [0x1429cd078]=3.0, [0x1429cd06c]=15.0
    },
    WallKind::OneShot("event:/SFX/LintelAppear"),    // 8
    WallKind::OneShot("event:/SFX/LintelRearrange"), // 9  (0x142ab4238, 0x1a)
    WallKind::LoopBoolParam {
        // 10 WideningGate
        loop_index: 5,
        param: "HasBricks", // 0x142ab4252, len 0x9
        flag_offset: 0x9,
    },
    WallKind::OneShot("event:/SFX/LintelMerge"),     // 11
    WallKind::OneShot("event:/SFX/StoneArchAppear"), // 12
    WallKind::Loop(6),                               // 13 PlatformStairsExtend
    WallKind::OneShot("event:/SFX/PillarStoneAppear"), // 14
    WallKind::OneShot("event:/SFX/PillarWoodAppear"), // 15
    WallKind::OneShot("event:/SFX/LintelRearrange"), // 16（与 kind 9 同名同处理）
    WallKind::OneShot("event:/SFX/GateAppear"),      // 17
    WallKind::Loop(7),                               // 18 SewerGateWiden
    WallKind::OneShot("event:/SFX/PlatformStairsAppear"), // 19
    WallKind::Loop(9),                               // 20 FenceMove
    WallKind::OneShot("event:/SFX/WallSplit"),       // 21
    WallKind::OneShot("event:/SFX/SewerGateAppear"), // 22
    WallKind::Loop(8),                               // 23 FenceHeight
];

/// 取 kind 的行为（kind ≥ 24 视为忽略；跳表只覆盖 0–23）。
pub fn wall_kind(kind: u8) -> &'static WallKind {
    WALL_KIND_TABLE
        .get(kind as usize)
        .unwrap_or(&WallKind::Ignored)
}

/// B MusicRichness's three resolved boundary bands. This excludes the unknown
/// MusicPlayer/smoothing adapter and accepts finite scalar values only.
pub fn snap_off_boundaries(value: f32) -> f32 {
    if (0.24..0.25).contains(&value) {
        0.251
    } else if (0.49..=0.5).contains(&value) {
        0.501
    } else if (0.74..=0.75).contains(&value) {
        0.751
    } else {
        value
    }
}

/// Resolved NewGladeTime accumulator, projected from MusicAudioState+0x1c8.
/// Finite time/accumulator inputs only; no complete FMOD state or native layout.
pub fn advance_new_glade_time(time: f32, accumulator: &mut f32) -> f32 {
    *accumulator = (time + *accumulator).clamp(0.0, 0.8);
    *accumulator
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_wall_jump_table_contract() {
        match wall_kind(1) {
            WallKind::LoopParam {
                loop_index,
                param,
                inverse_lerp_bounds,
            } => {
                assert_eq!(
                    (*loop_index, *param, *inverse_lerp_bounds),
                    (1, "WallHeight", (0.5, 14.0))
                );
            }
            _ => panic!("expected original WallHeight case"),
        }
        match wall_kind(10) {
            WallKind::LoopBoolParam {
                loop_index,
                param,
                flag_offset,
            } => {
                assert_eq!((*loop_index, *param, *flag_offset), (5, "HasBricks", 0x9));
            }
            _ => panic!("expected original HasBricks case"),
        }
        assert!(matches!(wall_kind(24), WallKind::Ignored));
    }
    #[test]
    fn original_music_parameter_boundaries() {
        assert_eq!(snap_off_boundaries(0.24), 0.251);
        assert_eq!(snap_off_boundaries(0.2499), 0.251);
        assert_eq!(snap_off_boundaries(0.25), 0.25);
        assert_eq!(snap_off_boundaries(0.5), 0.501);
        assert_eq!(snap_off_boundaries(0.74), 0.751);
        assert_eq!(snap_off_boundaries(0.75), 0.751);
        assert_eq!(snap_off_boundaries(0.3), 0.3);
        let mut accumulator = 0.0;
        assert_eq!(advance_new_glade_time(0.5, &mut accumulator), 0.5);
        assert_eq!(advance_new_glade_time(0.5, &mut accumulator), 0.8);
    }
}
