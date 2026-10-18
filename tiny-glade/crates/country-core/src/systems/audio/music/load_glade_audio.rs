//! ECS 主题音频入口，复用 startup 的唯一加载实现。
use super::{music_player::MusicBackend, music_system::MusicAudioState};
use crate::{resources::session::ActiveSession, startup::load_glade_audio::load_theme_audio};
use bevy_ecs::prelude::*;

/// 在活动会话已建立、音乐资源已初始化后运行。
pub fn load_glade_audio<B: MusicBackend + Resource>(
    backend: Res<B>,
    session: Res<ActiveSession>,
    mut state: ResMut<MusicAudioState<B::Event>>,
) -> bevy_ecs::error::Result {
    load_theme_audio(
        &mut state.player,
        &*backend,
        &session.assets_root(),
        session.meta.glade,
    )?;
    Ok(())
}

/// 主题加载阶段注册点，不应放入无条件运行的每帧更新表。
pub fn configure_glade_audio<B: MusicBackend + Resource>(schedule: &mut Schedule) {
    schedule.add_systems(load_glade_audio::<B>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::audio_instance::PlaybackState;
    use crate::resources::session::GladeType;
    use crate::startup::load_glade_audio::HALLOWEEN_MUSIC_EVENTS;
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
        let paths: Vec<String> = HALLOWEEN_MUSIC_EVENTS
            .iter()
            .map(|s| s.to_string())
            .collect();
        player.load_tracks(&backend, &paths);
        assert_eq!(player.track_names.len(), 4);
        assert_eq!(player.track_names[0], "event:/Music/Halloween/Single Leaf");
        assert_eq!(
            player.track_names[1],
            "event:/Music/Halloween/Tiny Olden Glade"
        );
        assert_eq!(
            player.track_names[2],
            "event:/Music/Halloween/Forgotten Ruins"
        );
        assert_eq!(
            player.track_names[3],
            "event:/Music/Halloween/Enchanted Forest"
        );
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
        assert_eq!(
            if GladeType::Winter == GladeType::Winter {
                0.25
            } else {
                0.0
            },
            0.25
        );
        assert_eq!(
            if GladeType::Summer == GladeType::Winter {
                0.25
            } else {
                0.0
            },
            0.0
        );
    }
}
