//! `load_glade_audio.rs` — glade 音频的常量（B `startup::load_glade_audio`，串池 0x142aa0c33 起，逐字节核对）。
//!
//! `GladeAudio::load`（0x1408bf460）读 `{path}/audio.json`，随后创建 4 个 Halloween 音乐事件，并绑定两个 FMOD 参数。
//! 两个参数名共享尾部串：`IsHalloween`（0x142aa0cf1）是 `GladeTypeIsHalloween`（0x142aa0ce8）从第 9 字节起的后缀，
//! 文档里的"(9)"是这个偏移，不是长度。
//!
//! `load_glade_audio`（0x1408be360）本体（2026-10-17 接线）：`MusicPlayer::stop_immediately` →
//! `GladeAudio::load`（读 audio.json，serde `Option<GladeAudio>`；**audio.json schema U 待①取证**，
//! 本函数以 `HALLOWEEN_MUSIC_EVENTS` 常量承载曲目名）→ 建 4 个 `event:/Music/Halloween/...` 路径 →
//! `MusicPlayer::load_tracks` → 主题字节（`+0x58 == 4`，Winter）写 `glade_theme_flag = 0.25`
//! （f32 `0x3e800000` @0x142925950；其余 0）→ `MusicBackend::set_global_parameter`
//! （`GladeTypeIsHalloween` / `IsHalloween`）→ 实例化/启动。audio.json 解析面与 FMOD
//! 实例化面留待音频域/host（本函数只接库内可接链）。

use std::path::Path;

use crate::systems::audio::music::music_player::{MusicBackend, MusicEvent, MusicPlayer};

/// 4 个 Halloween 音乐事件（O：串池顺序）。
pub const HALLOWEEN_MUSIC_EVENTS: [&str; 4] = [
    "event:/Music/Halloween/Single Leaf",
    "event:/Music/Halloween/Tiny Olden Glade",
    "event:/Music/Halloween/Forgotten Ruins",
    "event:/Music/Halloween/Enchanted Forest",
];

/// 绑定的 FMOD 参数名（O）。
pub const GLADE_TYPE_IS_HALLOWEEN_PARAMETER: &str = "GladeTypeIsHalloween";
pub const IS_HALLOWEEN_PARAMETER: &str = "IsHalloween";

/// `{path}/audio.json`（O：日志 `Reading {path}/audio.json`，0x142aa0d44）。
pub fn audio_json_path(glade_dir: &str) -> String {
    format!("{glade_dir}/audio.json")
}

/// 主题音量门（O：load_glade_audio 0x1408be360 写 `0.25`（f32 0x3e800000）当主题字节
/// `+0x58 == 4`（Winter）否则 `0`）。MusicPlayer 的 `glade_theme_flag` 字段即此语义。
pub const fn halloween_theme_flag(theme_byte: u8) -> f32 {
    if theme_byte == 4 {
        f32::from_bits(0x3e800000) // 0.25
    } else {
        0.0
    }
}

/// `load_glade_audio` 0x1408be360（域内可接链，2026-10-17 接线）。
///
/// 原生流程：`MusicPlayer::stop_immediately` → `GladeAudio::load`（读 audio.json）→
/// 建 4 个 Halloween 音乐事件路径 → `MusicPlayer::load_tracks` → 主题音量门 → 绑定
/// `GladeTypeIsHalloween`/`IsHalloween` 参数 → 实例化/启动。
///
/// 本函数落库内可接链：`stop_immediately` + `load_tracks(HALLOWEEN_MUSIC_EVENTS)` +
/// `glade_theme_flag` 主题门 + `MusicBackend::set_global_parameter`。audio.json 的
/// `GladeAudio` serde schema（U）与 FMOD 实例化/启动由音频域/host 接入，不伪造解析。
pub fn load_glade_audio<E, B>(
    player: &mut MusicPlayer<E>,
    backend: &B,
    _glade_dir: &Path,
    theme_byte: u8,
) where
    E: MusicEvent,
    B: MusicBackend<Event = E>,
{
    // 1. 停止当前音乐（O：MusicPlayer::stop_immediately）。
    player.stop_immediately();
    // 2. 加载 4 个 Halloween 音乐事件（O：event:/Music/Halloween/... 串池，逐个
    //    instantiate_audio + beat 回调注册）。
    let tracks: Vec<String> = HALLOWEEN_MUSIC_EVENTS
        .iter()
        .map(|event| (*event).to_string())
        .collect();
    player.load_tracks(backend, &tracks);
    // 3. 主题音量门（O：theme==4 → 0.25；其余 0）。
    let flag = halloween_theme_flag(theme_byte);
    player.glade_theme_flag = flag;
    // 4. 绑定 FMOD 参数（O：GladeTypeIsHalloween / IsHalloween 两参数）。
    let _ = backend.set_global_parameter(GLADE_TYPE_IS_HALLOWEEN_PARAMETER, flag);
    let _ = backend.set_global_parameter(IS_HALLOWEEN_PARAMETER, flag);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_names_share_a_suffix_at_offset_nine() {
        assert!(GLADE_TYPE_IS_HALLOWEEN_PARAMETER.ends_with(IS_HALLOWEEN_PARAMETER));
        assert_eq!(
            GLADE_TYPE_IS_HALLOWEEN_PARAMETER.len() - IS_HALLOWEEN_PARAMETER.len(),
            9
        );
    }

    #[test]
    fn events_live_under_the_halloween_music_bus() {
        assert!(
            HALLOWEEN_MUSIC_EVENTS
                .iter()
                .all(|e| e.starts_with("event:/Music/Halloween/"))
        );
        assert_eq!(HALLOWEEN_MUSIC_EVENTS.len(), 4);
    }

    #[test]
    fn audio_json_is_joined_with_a_forward_slash() {
        assert_eq!(
            audio_json_path("assets/glade/olden"),
            "assets/glade/olden/audio.json"
        );
    }
}
