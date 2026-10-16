//! `impl MusicBackend for AudioCore<S>` — SPEC §2.4 方案 A（后端收敛）。
//!
//! 原 `music_audio_system` 以 `Res<AudioCore>` 为后端（O 签名 0x140a72810）。本模块把
//! [`crate::resources::audio_resources::AudioCore`]（FMOD Studio + banks）暴露为
//! [`MusicBackend`]：`instantiate` → `AudioCoreInner::instantiate_audio`
//! （0x140a54440），`set_global_parameter` → `AudioCoreInner::set_parameter`
//! （0x140a53db0）。FMOD 生命周期保留在 `resources/audio_resources.rs`，本组只暴露 trait。
//!
//! 中央 agent（a8a2e9731c4957436）2026-10-16 仲裁批准：`EventInstance` trait 补
//! `install_beat_callback`（native.rs `set_callback(TIMELINE_BEAT)` 已存在）；本 adapter
//! 放 music 域，不改 audio_resources 其它部分。
use super::music_player::{MusicBackend, MusicEvent, StopMode};
use crate::resources::{
    audio_instance::PlaybackState,
    audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
};

/// `AudioInstance` → [`MusicEvent`] 适配。
///
/// `MusicEvent` 各操作对 Active 实例转发；Disabled/Err 不动。`stop` 恒为
/// STOP_IMMEDIATE（`AudioInstance::stop` 0x140a57470，原生实例无 AllowFadeout 分支）；
/// `paused` 语义 I（原生无直读，以 `is_playing` 取反近似）。
pub struct AudioCoreEvent<E: EventInstance>(pub AudioInstance<E>);

impl<E> MusicEvent for AudioCoreEvent<E>
where
    E: EventInstance + Send + Sync + 'static,
    E::Error: std::fmt::Debug,
{
    type Error = E::Error;

    fn start(&mut self) -> Result<(), Self::Error> {
        self.0.start()
    }

    fn stop(&mut self, _mode: StopMode) -> Result<(), Self::Error> {
        self.0.stop()
    }

    fn set_paused(&mut self, paused: bool) -> Result<(), Self::Error> {
        self.0.pause(paused)
    }

    fn paused(&self) -> Result<bool, Self::Error> {
        Ok(!self.0.is_playing()?)
    }

    fn playback_state(&self) -> Result<PlaybackState, Self::Error> {
        if let AudioInstance::Active { event, .. } = &self.0 {
            event.get_playback_state()
        } else {
            Ok(PlaybackState::Stopped)
        }
    }

    fn timeline_position(&self) -> Result<i32, Self::Error> {
        self.0.get_timeline_position()
    }

    fn install_beat_callback(&mut self) -> Result<(), Self::Error> {
        if let AudioInstance::Active { event, .. } = &mut self.0 {
            event.install_beat_callback()
        } else {
            Ok(())
        }
    }
}

impl<S> MusicBackend for AudioCore<S>
where
    S: Studio + Send + Sync + 'static,
    S::Error: std::fmt::Debug,
{
    type Event = AudioCoreEvent<S::Event>;

    fn instantiate(
        &self,
        path: &str,
    ) -> Result<Option<Self::Event>, <Self::Event as MusicEvent>::Error> {
        let instance = self.instantiate_audio(path);
        if matches!(&instance, AudioInstance::Disabled) {
            // 空 core → Disabled（O：AudioCore::instantiate_audio 0x140a55f40）。
            return Ok(None);
        }
        if matches!(&instance, AudioInstance::Err(_)) {
            // 实例化错误 → Err(anyhow)（0x140a54440）；`MusicPlayer::load_tracks` 文档
            // "creation errors panic"（0x1408b6b10），此处以 panic 表达。
            panic!("FMOD error instantiating {path}");
        }
        Ok(Some(AudioCoreEvent(instance)))
    }

    fn set_global_parameter(
        &self,
        name: &str,
        value: f32,
    ) -> Result<(), <Self::Event as MusicEvent>::Error> {
        self.set_parameter(name, value)
    }
}
