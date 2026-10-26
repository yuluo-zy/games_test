//! 主题音频加载的公共实现。主题编号、Halloween 开关和音乐音量分别处理。
//! 具体主题曲目复用 GladeAudio 配置，避免所有主题都使用 Halloween 曲库。

use crate::{
    resources::session::{GladeType, SessionMeta},
    systems::audio::music::{
        glade_audio::GladeAudio,
        music_player::{MusicBackend, MusicInstance, StopMode},
        music_system::MusicAudioState,
    },
};
use std::path::Path;

/// 缺少 Halloween 配置时使用的曲库，不作为普通主题的默认曲库。
pub const HALLOWEEN_MUSIC_EVENTS: [&str; 4] = [
    "event:/Music/Halloween/Single Leaf",
    "event:/Music/Halloween/Tiny Olden Glade",
    "event:/Music/Halloween/Forgotten Ruins",
    "event:/Music/Halloween/Enchanted Forest",
];

/// 两个相邻但独立的 FMOD 参数名，不能把它们拼成一个参数。
pub const GLADE_TYPE_PARAMETER: &str = "GladeType";
pub const IS_HALLOWEEN_PARAMETER: &str = "IsHalloween";

/// 冬季音乐标量，与主题编号和 Halloween 布尔值没有数值上的关联。
pub const fn halloween_theme_flag(theme_byte: u8) -> f32 {
    if theme_byte == GladeType::Winter as u8 {
        0.25
    } else {
        0.0
    }
}

fn set_parameter<B: MusicBackend>(backend: &B, name: &str, value: f32) {
    if let Err(error) = backend.set_global_parameter(name, value) {
        log::warn!("无法设置音频参数 {name}={value}：{error:?}");
    }
}

/// Complete original load_glade_audio@1408be360 (3927 B). Session metadata,
/// rather than the directory spelling, selects the Halloween override. Old
/// tracks and ambience stop before replacement; the stinger is fire-and-forget.
pub fn load_session_audio<B: MusicBackend>(
    state: &mut MusicAudioState<B::Event>, backend: &B,
    glade_dir: &Path, meta: &SessionMeta,
) {
    state.player.stop_immediately();
    state.new_glade_time = 0.0;
    let description = GladeAudio::try_load_path(glade_dir).unwrap_or_default();
    let halloween = meta.is_halloween_any();
    if halloween {
        state.player.load_tracks(backend, &HALLOWEEN_MUSIC_EVENTS);
    } else {
        state.player.load_tracks(backend, &description.music);
    }
    state.ambience.stop(StopMode::Immediate);
    state.ambience = MusicInstance::create(backend, &description.ambience);
    state.ambience.start();
    if let Some(path) = description.select_glade_stinger {
        let mut stinger = MusicInstance::create(backend, &path);
        stinger.start();
        // Native instance Drop calls release; FMOD retains the playing event.
    }
    let richness = halloween_theme_flag(meta.glade.native_tag());
    state.richness = richness;
    state.target_richness = richness;
    set_parameter(backend, GLADE_TYPE_PARAMETER, meta.glade.audio_id());
    set_parameter(backend, IS_HALLOWEEN_PARAMETER, if halloween { 1.0 } else { 0.0 });
}

#[cfg(test)]
#[path = "load_glade_audio_regression.rs"]
mod session_regression;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parameter_names_are_independent() {
        assert_eq!(GLADE_TYPE_PARAMETER, "GladeType");
        assert_eq!(IS_HALLOWEEN_PARAMETER, "IsHalloween");
    }
    #[test]
    fn winter_volume_is_distinct_from_theme_id() {
        assert_eq!(halloween_theme_flag(4), 0.25);
        assert_eq!(GladeType::Winter.audio_id(), 4.0);
        assert_eq!(halloween_theme_flag(0), 0.0);
    }
}
