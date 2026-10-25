//! Real adapter for the original Cargo.lock libfmod 2.222.6. The caller supplies
//! its initialized Studio (with banks loaded) and retains responsibility for
//! Studio shutdown. This module does not simulate playback or missing banks.
use super::music_player::{self, MusicBackend, MusicEvent, StopMode, TimelineBeat};
use crate::resources::audio_instance::PlaybackState;
use bevy_ecs::resource::Resource;
use libfmod::{EventInstance, Studio, ffi};
use std::{ffi::c_void, sync::Arc};

#[derive(Resource)]
pub struct FmodMusicBackend {
    pub studio: Option<Arc<Studio>>,
}
pub struct FmodMusicEvent {
    instance: EventInstance,
    _studio: Arc<Studio>,
}
impl Drop for FmodMusicEvent {
    fn drop(&mut self) {
        // instantiate_audio sets stop-on-drop=None (byte +0x28=2).
        // AudioInstance::drop 0x14089a280 releases and unwraps the result.
        self.instance.release().unwrap();
    }
}
impl MusicBackend for FmodMusicBackend {
    type Event = FmodMusicEvent;
    fn instantiate(&self, path: &str) -> Result<Option<Self::Event>, libfmod::Error> {
        self.studio
            .as_ref()
            .map(|studio| {
                let instance = studio.get_event(path)?.create_instance()?;
                Ok(FmodMusicEvent {
                    instance,
                    _studio: Arc::clone(studio),
                })
            })
            .transpose()
    }
    fn set_global_parameter(&self, name: &str, value: f32) -> Result<(), libfmod::Error> {
        if let Some(studio) = &self.studio {
            studio.set_parameter_by_name(name, value, false)?;
        }
        Ok(())
    }
}
impl MusicEvent for FmodMusicEvent {
    type Error = libfmod::Error;
    fn start(&mut self) -> Result<(), Self::Error> {
        self.instance.start()
    }
    fn stop(&mut self, mode: StopMode) -> Result<(), Self::Error> {
        self.instance.stop(match mode {
            StopMode::AllowFadeout => libfmod::StopMode::AllowFadeout,
            StopMode::Immediate => libfmod::StopMode::Immediate,
        })
    }
    fn set_paused(&mut self, paused: bool) -> Result<(), Self::Error> {
        self.instance.set_paused(paused)
    }
    fn paused(&self) -> Result<bool, Self::Error> {
        self.instance.get_paused()
    }
    fn playback_state(&self) -> Result<PlaybackState, Self::Error> {
        Ok(match self.instance.get_playback_state()? {
            libfmod::PlaybackState::Playing => PlaybackState::Playing,
            libfmod::PlaybackState::Sustaining => PlaybackState::Sustaining,
            libfmod::PlaybackState::Stopped => PlaybackState::Stopped,
            libfmod::PlaybackState::Starting => PlaybackState::Starting,
            libfmod::PlaybackState::Stopping => PlaybackState::Stopping,
        })
    }
    fn timeline_position(&self) -> Result<i32, Self::Error> {
        self.instance.get_timeline_position()
    }
    fn install_beat_callback(&mut self) -> Result<(), Self::Error> {
        self.instance.set_callback(
            Some(timeline_callback),
            ffi::FMOD_STUDIO_EVENT_CALLBACK_TIMELINE_BEAT,
        )
    }
}
unsafe extern "C" fn timeline_callback(
    _kind: ffi::FMOD_STUDIO_EVENT_CALLBACK_TYPE,
    _event: *mut ffi::FMOD_STUDIO_EVENTINSTANCE,
    parameters: *mut c_void,
) -> ffi::FMOD_RESULT {
    // FMOD invokes this only for TIMELINE_BEAT because registration uses 0x1000.
    // The payload belongs to FMOD and is valid for the duration of the callback.
    let properties = unsafe {
        parameters
            .cast::<ffi::FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES>()
            .as_ref()
    };
    music_player::get_timeline_beat_properties(properties.map(|p| TimelineBeat {
        bar: p.bar,
        beat: p.beat,
        tempo: p.tempo,
        time_signature_upper: p.timesignatureupper,
        time_signature_lower: p.timesignaturelower,
    }));
    ffi::FMOD_OK
}
