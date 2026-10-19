//! 主题音频加载的公共实现。主题编号、Halloween 开关和音乐音量分别处理。
//! 具体主题曲目复用 GladeAudio 配置，避免所有主题都使用 Halloween 曲库。

use crate::{
    resources::session::{GladeType, SessionMeta},
    systems::audio::music::{
        glade_audio::GladeAudio,
        music_player::{MusicBackend, MusicEvent, MusicPlayer, MusicInstance, StopMode},
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

/// 兼容依赖这个路径工具的 GladeAudio 读取器。
pub fn audio_json_path(glade_dir: &str) -> String {
    Path::new(glade_dir)
        .join("audio.json")
        .to_string_lossy()
        .into_owned()
}

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

/// 类型明确的主题入口；曲目替换会清理旧播放列表，重复调用不会积累实例。
/// 只有显式使用 halloween 目录时开启节日参数；正常冬季仍是普通主题。
pub fn load_theme_audio<E, B>(
    player: &mut MusicPlayer<E>,
    backend: &B,
    glade_dir: &Path,
    theme: GladeType,
) -> anyhow::Result<()>
where
    E: MusicEvent,
    B: MusicBackend<Event = E>,
{
    let is_halloween = glade_dir
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case("halloween"));
    let description = match GladeAudio::try_load_path(glade_dir) {
        Ok(description) => description,
        Err(error)
            if is_halloween
                && error.chain().any(|cause| {
                    cause
                        .downcast_ref::<std::io::Error>()
                        .is_some_and(|io| io.kind() == std::io::ErrorKind::NotFound)
                }) =>
        {
            GladeAudio::default()
        }
        Err(error) => return Err(error),
    };
    if is_halloween && description.music.is_empty() {
        player.load_tracks(backend, &HALLOWEEN_MUSIC_EVENTS);
    } else {
        player.load_tracks(backend, &description.music);
    }
    player.glade_theme_flag = halloween_theme_flag(theme.native_tag());
    set_parameter(backend, GLADE_TYPE_PARAMETER, theme.audio_id());
    set_parameter(
        backend,
        IS_HALLOWEEN_PARAMETER,
        if is_halloween { 1.0 } else { 0.0 },
    );
    Ok(())
}

/// 兼容字节形式的宿主调用；无效主题会在产生音频副作用前失败。
pub fn load_glade_audio<E, B>(
    player: &mut MusicPlayer<E>,
    backend: &B,
    glade_dir: &Path,
    theme_byte: u8,
) -> anyhow::Result<()>
where
    E: MusicEvent,
    B: MusicBackend<Event = E>,
{
    let theme = GladeType::ALL
        .get(theme_byte as usize)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("无效的音频主题编号：{theme_byte}"))?;
    load_theme_audio(player, backend, glade_dir, theme)
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
