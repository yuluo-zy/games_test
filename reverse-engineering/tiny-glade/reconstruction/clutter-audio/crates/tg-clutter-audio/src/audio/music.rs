//! country-core systems/audio/music：music_audio_system 的 MusicRichness / NewGladeTime 参数计算（B 主基线，反汇编级证据草稿）
//!
//! 依据：evidence/multi-baseline/audio-extra-analysis.md §4（2026-10-13 reva audio-extra-owner
//! 线性反汇编 + High P-code 反编译；radare2 RNE 常量）。
//! - MusicRichness 计算内联自 `update_music_richness_based_on_time`（music_system.rs line 136）。
//! - FMOD（libfmod）/ bevy 框架调用只标边界 `[fmod]`；资源身份部分未定（见 evidence §5）。

/// MusicAudioState 中 music 相关字段（evidence：State = arg3 deref）。
#[derive(Default)]
pub struct MusicAudioStateFields {
    /// +0x1c0：平滑后的 MusicRichness。
    pub richness_smoothed: f32,
    /// +0x1c4：MusicRichness 累加器（每帧 += time/120）。
    pub richness_accum: f32,
    /// +0x1c8：NewGladeTime 累加器（每帧 += time，上限 0.8）。
    pub new_glade_time_accum: f32,
}

/// MusicRichness 计算（证据，常量 RNE 确认）。
/// `accum = clamp(accum + time / 120.0, 0, 1)`，随后"snap off"边界：
/// `[0.24,0.25)→0.251`、`[0.49,0.5]→0.501`、`[0.74,0.75]→0.751`。
/// 再 `time_continuous_lerp(State+0x1c0, target, …)` 平滑（target = 1.0 if mode==0x13 else accum），
/// clamp [0,1] 并再次 snap off → State+0x1c0。
pub fn compute_music_richness(
    time: f32,
    state: &mut MusicAudioStateFields,
    _mode: u32, // [evidence] arg1+0 == 0x13 → target 强制 1.0
) -> f32 {
    const TIME_DIV: f32 = 120.0;   // 0x1429cd0f8
    const ONE: f32 = 1.0;          // 0x142925904
    const LERP_K: f32 = 0.99;      // 0x142a948d8（time_continuous_lerp 参数，语义未定）
    let mut accum = (state.richness_accum + time / TIME_DIV).clamp(0.0, ONE);
    accum = snap_off_boundaries(accum);
    state.richness_accum = accum;
    let target = if _mode == 0x13 { ONE } else { accum };
    // [fmod] smoothed = time_continuous_lerp(state.richness_smoothed, target, LERP_K); // 0x140c904e0
    let mut smoothed = target; // [evidence] 平滑 + clamp [0,1]；证据位
    smoothed = snap_off_boundaries(smoothed);
    state.richness_smoothed = smoothed;
    let _ = LERP_K;
    smoothed
}

/// 避免恰好 0.25/0.5/0.75（FMOD 参数边界）：置 0.251/0.501/0.751。
pub fn snap_off_boundaries(x: f32) -> f32 {
    // 0.24/0.25/0.251 (0x142a7fdf8/0x142925950/0x142ad58e4)
    if (0.24..0.25).contains(&x) {
        return 0.251;
    }
    // 0.49/0.5/0.501 (0x142ad58d4/0x142925870/0x142ad58e0)
    if (0.49..=0.5).contains(&x) {
        return 0.501;
    }
    // 0.74/0.75/0.751 (0x142ad58d8/0x1429edb90/0x142ad58dc)
    if (0.74..=0.75).contains(&x) {
        return 0.751;
    }
    x
}

/// NewGladeTime 计算（证据）：`t = clamp(time + State+0x1c8, 0, 0.8)` → State+0x1c8。
pub fn compute_new_glade_time(time: f32, state: &mut MusicAudioStateFields) -> f32 {
    const MAX: f32 = 0.8; // 0x142a16cb8
    let t = (time + state.new_glade_time_accum).clamp(0.0, MAX);
    state.new_glade_time_accum = t;
    t
}

/// music_audio_system（VA 0x140a72810）的参数资源角色（evidence，identity 部分未定）：
/// - arg1: 资源 +0 u32（==0x13 → richness 目标 1.0）
/// - arg2: ResMut<MusicPlayer>（*[rdx]）
/// - arg3: ResMut<MusicAudioState>（*[r8]）
/// - arg4: Res<AppMode>（*[r9]，+0 u32；(m|8)==0xa → MusicPlayer::update）
/// - arg5 [rbp+0xe0]: 时间资源 +0x28 f32
/// - arg6 [rbp+0xd8]: 资源 +0x100 byte（置位 → MusicRichness 用 1.0）
/// - arg7 [rbp+0xe8]: 资源 +0x58 f32（MusicPlayer::update 的节拍参数）
pub struct MusicSystemArgs {
    pub mode_gate_19: u32,        // arg1 +0
    pub flag_force_max: bool,     // arg6 +0x100
    pub beat: f32,                // arg7 +0x58
}

/// 每帧主流程（证据序）。
pub fn music_audio_system_step(
    time: f32, // [evidence] arg5 +0x28
    state: &mut MusicAudioStateFields,
    _player: &[u8], // [evidence] arg2 → MusicPlayer（set_parameter / update / pause）
    _app_mode: u32, // [evidence] arg4 +0；(m|8)==0xa 才 update
    args: &MusicSystemArgs,
    _force_max_richness: bool, // [evidence] flag_force_max
) {
    // 1) MusicRichness。
    let richness = compute_music_richness(time, state, args.mode_gate_19);
    let _richness_param = if args.flag_force_max { 1.0 } else { richness };
    // [fmod] set_parameter(player_event, "MusicRichness"(0x142ad6398, 0xd), richness_param); // 0x140a53db0

    // 2) NewGladeTime。
    let ngt = compute_new_glade_time(time, state);
    // [fmod] set_parameter(player_event, "NewGladeTime"(0x142ad63a5, 0xc), ngt);

    // 3) 帧尾：((app_mode | 8) == 0xa) → MusicPlayer::update(player, args.beat)（0x1408b5500），
    //    否则 MusicPlayer::pause()（0x1408b7490）；随后 stop_photomode_snapshots / start_if_not_playing
    //    / libfmod stop/set_paused/get_paused（0x1408b6250/0x1408b70d0/0x141437440/0x141437160/0x141436fe0）。
    let _ = _app_mode;
    let _ = _player;
}

/// 参数串表（0x142ad6398 起）。
pub const MUSIC_PARAM_NAMES: [&str; 3] = [
    "MusicRichness",     // 0x142ad6398, len 0xd
    "NewGladeTime",      // 0x142ad63a5, len 0xc
    "PreviewGladeType",  // 0x142ad63b1, len 0x10
];

/// 初始化事件（music_audio_init_system，VA 0x140a71f20）：
/// GladeAudio::default → 3× instantiate_audio：event:/Music/mus_GladePreview（0x142ad6318）、
/// mus_GladePreview 短名（0x142ad6325）、event:/Amb/GladePreview（0x142ad6350）→ start（0x1414372d0）
/// → MusicPlayer::init（0x1408b63c0）→ insert_resource<MusicAudioState>（0x1412e00c0）。
pub const MUSIC_INIT_EVENTS: [&str; 3] = [
    "event:/Music/mus_GladePreview",
    "mus_GladePreview",
    "event:/Amb/GladePreview",
];
