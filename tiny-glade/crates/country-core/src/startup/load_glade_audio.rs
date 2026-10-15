//! `load_glade_audio.rs` — glade 音频的常量（B `startup::load_glade_audio`，串池 0x142aa0c33 起，逐字节核对）。
//!
//! `GladeAudio::load`（0x1408bf460）读 `{path}/audio.json`，随后创建 4 个 Halloween 音乐事件，并绑定两个 FMOD 参数。
//! 两个参数名共享尾部串：`IsHalloween`（0x142aa0cf1）是 `GladeTypeIsHalloween`（0x142aa0ce8）从第 9 字节起的后缀，
//! 文档里的"(9)"是这个偏移，不是长度。
//!
//! 未做：`load_glade_audio`（0x1408be360）本体——按主题字节（`+0x58`）索引、创建 FMOD 实例。

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
