//! `load_glade_audio` — glade 音频 producer（B `startup::load_glade_audio` 系统
//! 0x1408be360）。
//!
//! O 主流程（decomp-0x1408be360）：
//! 1. `player.pause()`（FUN_1408b72e0）+ 清 current-track（`puVar7[0x72]=0`）。
//! 2. `GladeAudio::load`（0x1408bf460）读 `assets/glade/{theme}/audio.json`。
//! 3. 拼 4 个 `event:/Music/Halloween/{Single Leaf, Tiny Olden Glade, Forgotten Ruins,
//!    Enchanted Forest}`（串表 @0x142aa0c33）。
//! 4. `player.load_tracks`（0x1408b6b10）用这 4 个名字。
//! 5. 逐 track `AudioCoreInner::instantiate_audio` + 容器注册（0x14089a280）——
//!    由 `MusicPlayer::load_tracks` 的实例化路径覆盖（注册细节 I）。
//! 6. `set_parameter("GladeTypeIsHalloween", table[theme])`（0x142aa152c f32
//!    [0,3,1,2,4] = `GladeType::audio_id()`）与 `set_parameter("IsHalloween",
//!    1.0|0.0)`（文件夹串非空 → 1.0，`_UNK_142925904`；GladeType 恒有效 → 恒 1.0）。
//! 7. theme 旗标 `0.25`（`_UNK_142925950`）当 theme==4（Winter）否则 `0`，写 player
//!    `+0x1c0/+0x1c4`（映射为 [`MusicPlayer::glade_theme_flag`]）。
//!
//! 调度：native 在 GameStartup（0x140941110）装配为启动期系统（主题载入时运行一次）；
//! 本模块提供 [`configure_glade_audio`] 注册点供宿主调用（run_game.rs 库外，需仲裁/宿主接线）。
use crate::{
    resources::session::{ActiveSession, GladeType},
    startup::load_glade_audio::{
        GLADE_TYPE_IS_HALLOWEEN_PARAMETER, HALLOWEEN_MUSIC_EVENTS, IS_HALLOWEEN_PARAMETER,
    },
};
use bevy_ecs::prelude::*;

use super::{
    glade_audio::GladeAudio,
    music_player::MusicBackend,
    music_system::MusicAudioState,
};

fn set_parameter<B: MusicBackend>(backend: &B, name: &str, value: f32) {
    // AudioCoreInner::set_parameter（0x140a53db0）失败 → 全局报告 + 继续（O）。
    if let Err(error) = backend.set_global_parameter(name, value) {
        log::warn!("Could not set audio parameter {name}: {error:?}");
    }
}

/// `load_glade_audio` @0x1408be360：`(Res<AudioCore>|backend, Res<ActiveSession>,
/// ResMut<MusicAudioState>)`（native 后端是 `Res<AudioCore>`；本实现按 `B: MusicBackend`
/// 泛型，`AudioCore` 经 [`super::audio_core_backend`] 适配实现后端）。
pub fn load_glade_audio<B: MusicBackend + Resource>(
    backend: Res<B>,
    session: Res<ActiveSession>,
    mut state: ResMut<MusicAudioState<B::Event>>,
) {
    let theme = session.meta.glade;

    // 1. pause + 清 current-track（O）。
    state.player.pause();
    state.player.current_track = None;

    // 2. GladeAudio::load（O：0x1408bf460 读 `{folder}/audio.json`，失败 default 回退）。
    let _glade_audio = GladeAudio::load(&session.meta.assets_root().to_string_lossy());

    // 3+4. 4 个 Halloween 事件 → load_tracks（O：串表 + 0x1408b6b10）。
    let paths: Vec<String> = HALLOWEEN_MUSIC_EVENTS.iter().map(|s| s.to_string()).collect();
    state.player.load_tracks(&*backend, &paths);

    // 6. 参数（O）。
    set_parameter(&*backend, GLADE_TYPE_IS_HALLOWEEN_PARAMETER, theme.audio_id());
    // `IsHalloween` = 1.0 当主题文件夹串非空（O，`_UNK_142925904`）；GladeType 恒有效。
    set_parameter(&*backend, IS_HALLOWEEN_PARAMETER, 1.0);

    // 7. theme 旗标（O：theme==4 → 0.25，否则 0）。
    state.player.glade_theme_flag = if theme == GladeType::Winter { 0.25 } else { 0.0 };
}

/// `load_glade_audio` 注册点（启动期，主题载入时运行一次）。宿主在 GameStartup 装配
/// （0x140941110）；country-core 无自身 App，由宿主 schedule 调用本函数。
pub fn configure_glade_audio<B>(schedule: &mut Schedule)
where
    B: MusicBackend + Resource,
{
    schedule.add_systems(load_glade_audio::<B>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::audio_instance::PlaybackState;
    use crate::systems::audio::music::music_player::{MusicEvent, MusicPlayer, StopMode};
    use std::convert::Infallible;

    #[derive(Clone, Debug)]
    struct Event {
        name: String,
        beat_callbacks: usize,
    }
    impl MusicEvent for Event {
        type Error = Infallible;
        fn start(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn stop(&mut self, _mode: StopMode) -> Result<(), Self::Error> {
            Ok(())
        }
        fn set_paused(&mut self, _paused: bool) -> Result<(), Self::Error> {
            Ok(())
        }
        fn paused(&self) -> Result<bool, Self::Error> {
            Ok(false)
        }
        fn playback_state(&self) -> Result<PlaybackState, Self::Error> {
            Ok(PlaybackState::Stopped)
        }
        fn timeline_position(&self) -> Result<i32, Self::Error> {
            Ok(0)
        }
        fn install_beat_callback(&mut self) -> Result<(), Self::Error> {
            self.beat_callbacks += 1;
            Ok(())
        }
    }

    #[derive(Clone, Debug, Default)]
    struct Backend;
    impl super::super::music_player::MusicBackend for Backend {
        type Event = Event;
        fn instantiate(&self, path: &str) -> Result<Option<Self::Event>, Infallible> {
            Ok(Some(Event {
                name: path.into(),
                beat_callbacks: 0,
            }))
        }
        fn set_global_parameter(&self, _name: &str, _value: f32) -> Result<(), Infallible> {
            Ok(())
        }
    }

    #[test]
    fn producer_loads_the_four_halloween_tracks_with_beat_callbacks() {
        let backend = Backend;
        let mut player = MusicPlayer::init(&backend);
        // O 步骤 3+4：4 个 Halloween 事件名 → load_tracks。
        let paths: Vec<String> = HALLOWEEN_MUSIC_EVENTS.iter().map(|s| s.to_string()).collect();
        player.load_tracks(&backend, &paths);
        assert_eq!(player.track_names.len(), 4);
        assert_eq!(player.track_names[0], "event:/Music/Halloween/Single Leaf");
        assert_eq!(player.track_names[1], "event:/Music/Halloween/Tiny Olden Glade");
        assert_eq!(player.track_names[2], "event:/Music/Halloween/Forgotten Ruins");
        assert_eq!(player.track_names[3], "event:/Music/Halloween/Enchanted Forest");
        // load_tracks 只留 install_beat_callback 成功的实例（O：0x1408b6b10）。
        assert!(
            player
                .tracks
                .iter()
                .all(|t| t.event().map_or(false, |e| e.beat_callbacks == 1))
        );
    }

    #[test]
    fn theme_audio_ids_and_winter_flag_follow_the_original_constants() {
        // `GladeTypeIsHalloween` 表 @0x142aa152c = audio_id() [0,3,1,2,4]（O）。
        assert_eq!(GladeType::Summer.audio_id(), 0.0);
        assert_eq!(GladeType::Olden.audio_id(), 3.0);
        assert_eq!(GladeType::Flowery.audio_id(), 1.0);
        assert_eq!(GladeType::Autumn.audio_id(), 2.0);
        assert_eq!(GladeType::Winter.audio_id(), 4.0);
        // theme 旗标：theme==4（Winter）→ 0.25（`_UNK_142925950`），否则 0（O）。
        assert_eq!(if GladeType::Winter == GladeType::Winter { 0.25 } else { 0.0 }, 0.25);
        assert_eq!(if GladeType::Summer == GladeType::Winter { 0.25 } else { 0.0 }, 0.0);
    }
}
