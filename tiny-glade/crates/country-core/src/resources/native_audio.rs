//! The original locked libfmod 2.222.6, shared by boot and gameplay audio.
//! Native producer: AudioCore::new @140a54a40; see round12/startup evidence.
use super::audio_instance::PlaybackState;
use super::audio_resources::{AudioCore, AudioSamplesLoadedPromise, EventInstance, ListenerAttributes, Studio, VcaHandle};
use crate::systems::audio::music::music_player::{self, MusicBackend, MusicEvent, StopMode, TimelineBeat};
use bevy_ecs::resource::Resource;
use libfmod::{Attributes3d, Error, ffi};
use std::{collections::HashMap, ffi::c_void, path::Path, sync::Arc};

#[derive(Debug)]
struct StudioOwner(libfmod::Studio);
impl Drop for StudioOwner {
    fn drop(&mut self) {
        // Every live event/VCA retains this owner, so Studio release is last.
        self.0.release().unwrap();
    }
}

#[derive(Debug, Resource, Clone)]
pub struct NativeStudio {
    owner: Arc<StudioOwner>,
    banks: HashMap<String, libfmod::Bank>,
    last_bank: Option<libfmod::Bank>,
}

#[derive(Resource, Clone, Debug)]
pub struct NativeMusicBackend(pub Option<NativeStudio>);
impl MusicBackend for NativeMusicBackend {
    type Event = NativeEvent;
    fn instantiate(&self, name: &str) -> Result<Option<NativeEvent>, Error> {
        self.0.as_ref().map(|studio| studio.create_event_instance(name)).transpose().map(Option::flatten)
    }
    fn set_global_parameter(&self, name: &str, value: f32) -> Result<(), Error> {
        if let Some(studio) = &self.0 { Studio::set_parameter_by_name(studio, name, value)?; }
        Ok(())
    }
}

impl NativeStudio {
    /// Native boolean enables the Studio update thread; false selects
    /// FMOD_STUDIO_INIT_SYNCHRONOUS_UPDATE (4).
    pub fn create(asynchronous_update: bool) -> Result<Self, Error> {
        let owner = Arc::new(StudioOwner(libfmod::Studio::create()?));
        let core = owner.0.get_core_system()?;
        core.set_software_format(None, Some(libfmod::SpeakerMode::Stereo), None)?;
        let (length, count) = core.get_dsp_buffer_size().unwrap();
        let (sample_rate, _, _) = core.get_software_format().unwrap();
        log::trace!("Mixer buffer length: {length}");
        log::trace!("Mixer buffer count: {count}");
        log::debug!("Mixer latency: {} ms", length as f32 * 1000.0 / sample_rate as f32 * (count as f32 - 1.5));
        owner.0.initialize(1024, u32::from(!asynchronous_update) << 2, ffi::FMOD_INIT_3D_RIGHTHANDED, None)?;
        Ok(Self { owner, banks: HashMap::new(), last_bank: None })
    }

    pub fn raw(&self) -> &libfmod::Studio { &self.owner.0 }
}

/// Initialization errors retain the native logged AudioCore::None branch.
/// Bank/sample failures still panic through the recovered AudioCore producer.
pub fn create_audio_core(installation: &Path, asynchronous_update: bool) -> AudioCore<NativeStudio> {
    match NativeStudio::create(asynchronous_update) {
        Ok(studio) => AudioCore::new(studio, &installation.join("assets/audio/Desktop")),
        Err(error) => {
            log::error!("Failed initializing audio: {error}");
            AudioCore { studio: None, banks: HashMap::new() }
        }
    }
}

pub struct NativeAudioStartup {
    pub core: AudioCore<NativeStudio>,
    pub samples: Option<AudioSamplesLoadedPromise<()>>,
    pub load_remaining: Option<Box<dyn FnOnce() + Send + 'static>>,
}

/// The original producer returns a boot-ready owner, a delayed bank loader,
/// and the single-buffered completion promise. The host runs the loader on its
/// loading worker while the intro continues to use the same Studio.
pub fn create_audio_resources(installation: &Path, asynchronous_update: bool) -> NativeAudioStartup {
    let core = create_audio_core(installation, asynchronous_update);
    startup_from_core(core)
}

fn startup_from_core(core: AudioCore<NativeStudio>) -> NativeAudioStartup {
    let Some(mut studio) = core.studio.clone() else {
        return NativeAudioStartup { core, samples: None, load_remaining: None };
    };
    let mut banks = core.banks.clone();
    let (sender, samples) = AudioSamplesLoadedPromise::channel();
    let load_remaining = Box::new(move || {
        // Native closure @140a520b0: load every metadata bank before any
        // sample request, then sample every bank (including boot banks).
        for bank in banks.values_mut() { bank.load(&mut studio); }
        for bank in banks.values_mut() { bank.load_sample_data(&mut studio); }
        // Original @140a5242a passes deadline=None to std's array channel.
        // The capacity-one notification survives an early loader completion;
        // receiver cancellation is ignored by this producer.
        let _ = sender.send(());
    });
    NativeAudioStartup {
        core,
        samples: Some(samples),
        load_remaining: Some(load_remaining),
    }
}

#[cfg(test)]
mod audio_completion {
    use super::*;

    #[test]
    fn initialization_failure_has_no_completion_to_wait_for() {
        // Same owner returned by create_audio_core's real FMOD error arm.
        let startup = startup_from_core(AudioCore { studio: None, banks: HashMap::new() });
        assert!(startup.core.studio.is_none());
        assert!(startup.samples.is_none());
        assert!(startup.load_remaining.is_none());
    }

    #[test]
    #[ignore = "requires original shipped FMOD DLLs, banks and TG_INSTALL_ROOT"]
    fn shipped_fmod_completion_and_shared_studio_survive_delayed_receive() {
        let installed = std::env::var_os("TG_INSTALL_ROOT").expect("TG_INSTALL_ROOT");
        let mut startup = create_audio_resources(Path::new(&installed), false);
        assert!(startup.core.studio.is_some(), "original FMOD must initialize");
        let promise = startup.samples.take().expect("enabled completion");
        let loader = startup.load_remaining.take().expect("delayed loader");
        std::thread::spawn(loader).join().expect("metadata/sample loader");
        // Deliberately receive after both producer exit and the old fake timeout.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        assert_eq!(promise.recv(), Some(()));
        assert_eq!(promise.recv(), None);
        let shared = startup.core.studio.as_ref().unwrap().clone();
        drop(startup);
        for name in ["event:/Amb/Ambience", "event:/Music/mus_GladePreview", "event:/UI/Logo/Scribble"] {
            let mut event = shared.create_event_instance(name).unwrap().expect("real event");
            EventInstance::start(&mut event).unwrap();
            EventInstance::stop(&mut event, ffi::FMOD_STUDIO_STOP_IMMEDIATE).unwrap();
            EventInstance::release(&mut event).unwrap();
        }
        shared.raw().update().unwrap();
        // Event release and the last Arc release exercise actual FMOD teardown.
        drop(shared);
    }
}

#[derive(Debug)]
pub struct NativeEvent {
    instance: Option<libfmod::EventInstance>,
    _owner: Arc<StudioOwner>,
}
impl NativeEvent {
    fn raw(&self) -> &libfmod::EventInstance { self.instance.as_ref().expect("released FMOD event") }
    fn release_once(&mut self) -> Result<(), Error> {
        if let Some(instance) = self.instance.take() { instance.release()?; }
        Ok(())
    }
}
impl Drop for NativeEvent {
    fn drop(&mut self) { self.release_once().unwrap(); }
}

#[derive(Debug)]
pub struct NativeVca { vca: libfmod::Vca, _owner: Arc<StudioOwner> }
impl VcaHandle for NativeVca {
    type Error = Error;
    fn set_volume(&self, volume: f32) -> Result<(), Error> { self.vca.set_volume(volume) }
}
impl Studio for NativeStudio {
    type Error = Error;
    type Event = NativeEvent;
    type Vca = NativeVca;
    fn load_bank_file(&mut self, path: &str) -> Result<(), Error> {
        let bank = self.raw().load_bank_file(path, 0u32)?;
        self.banks.insert(path.into(), bank);
        self.last_bank = Some(bank);
        Ok(())
    }
    fn load_sample_data(&mut self) -> Result<(), Error> {
        self.last_bank.expect("bank must be loaded before sample data").load_sample_data()
    }
    fn load_bank_sample_data(&mut self, path: &str) -> Result<(), Error> {
        self.banks.get(path).expect("bank must be loaded before sample data").load_sample_data()
    }
    fn get_vca(&self, name: &str) -> Result<NativeVca, Error> {
        Ok(NativeVca { vca: self.raw().get_vca(name)?, _owner: Arc::clone(&self.owner) })
    }
    fn set_listener_attributes(&mut self, listener: ListenerAttributes) -> Result<(), Error> {
        self.raw().set_listener_attributes(0, attributes(listener), None)
    }
    fn update(&mut self) -> Result<(), Error> { self.raw().update() }
    fn set_parameter_by_name(&self, name: &str, value: f32) -> Result<(), Error> {
        self.raw().set_parameter_by_name(name, value, false)
    }
    fn create_event_instance(&self, name: &str) -> Result<Option<NativeEvent>, Error> {
        Ok(Some(NativeEvent {
            instance: Some(self.raw().get_event(name)?.create_instance()?),
            _owner: Arc::clone(&self.owner),
        }))
    }
}
fn attributes(a: ListenerAttributes) -> Attributes3d {
    Attributes3d { position: a.position.into(), forward: a.forward.into(), up: a.up.into(), velocity: a.velocity.into() }
}
fn playback(state: libfmod::PlaybackState) -> PlaybackState {
    match state {
        libfmod::PlaybackState::Playing => PlaybackState::Playing,
        libfmod::PlaybackState::Sustaining => PlaybackState::Sustaining,
        libfmod::PlaybackState::Stopped => PlaybackState::Stopped,
        libfmod::PlaybackState::Starting => PlaybackState::Starting,
        libfmod::PlaybackState::Stopping => PlaybackState::Stopping,
    }
}
impl EventInstance for NativeEvent {
    type Error = Error;
    fn start(&mut self) -> Result<(), Error> { self.raw().start() }
    fn stop(&mut self, mode: i32) -> Result<(), Error> {
        self.raw().stop(match mode {
            ffi::FMOD_STUDIO_STOP_IMMEDIATE => libfmod::StopMode::Immediate,
            ffi::FMOD_STUDIO_STOP_ALLOWFADEOUT => libfmod::StopMode::AllowFadeout,
            _ => panic!("invalid FMOD stop mode {mode}"),
        })
    }
    fn get_playback_state(&self) -> Result<PlaybackState, Error> { self.raw().get_playback_state().map(playback) }
    fn set_volume(&mut self, volume: f32) -> Result<(), Error> { self.raw().set_volume(volume) }
    fn set_parameter_by_name(&mut self, name: &str, value: f32) -> Result<(), Error> { self.raw().set_parameter_by_name(name, value, false) }
    fn has_parameter(&self, name: &str) -> Result<bool, Error> {
        match self.raw().get_description()?.get_parameter_description_by_name(name) {
            Ok(_) => Ok(true),
            Err(Error::Fmod { code: ffi::FMOD_ERR_EVENT_NOTFOUND, .. }) => Ok(false),
            Err(error) => Err(error),
        }
    }
    fn set_3d_attributes(&mut self, position: [f32;3], forward: [f32;3], up: [f32;3], velocity: [f32;3]) -> Result<(), Error> {
        self.raw().set_3d_attributes(attributes(ListenerAttributes { position, forward, up, velocity }))
    }
    fn get_timeline_position(&self) -> Result<i32, Error> { self.raw().get_timeline_position() }
    fn pause(&mut self, paused: bool) -> Result<(), Error> { self.raw().set_paused(paused) }
    fn install_beat_callback(&mut self) -> Result<(), Error> {
        self.raw().set_callback(Some(timeline_callback), ffi::FMOD_STUDIO_EVENT_CALLBACK_TIMELINE_BEAT)
    }
    fn release(&mut self) -> Result<(), Error> { self.release_once() }
}
impl MusicBackend for NativeStudio {
    type Event = NativeEvent;
    fn instantiate(&self, name: &str) -> Result<Option<NativeEvent>, Error> { self.create_event_instance(name) }
    fn set_global_parameter(&self, name: &str, value: f32) -> Result<(), Error> { Studio::set_parameter_by_name(self, name, value) }
}
impl MusicEvent for NativeEvent {
    type Error = Error;
    fn start(&mut self) -> Result<(), Error> { EventInstance::start(self) }
    fn stop(&mut self, mode: StopMode) -> Result<(), Error> {
        self.raw().stop(match mode { StopMode::Immediate => libfmod::StopMode::Immediate, StopMode::AllowFadeout => libfmod::StopMode::AllowFadeout })
    }
    fn set_paused(&mut self, paused: bool) -> Result<(), Error> { self.raw().set_paused(paused) }
    fn paused(&self) -> Result<bool, Error> { self.raw().get_paused() }
    fn playback_state(&self) -> Result<PlaybackState, Error> { EventInstance::get_playback_state(self) }
    fn timeline_position(&self) -> Result<i32, Error> { self.raw().get_timeline_position() }
    fn install_beat_callback(&mut self) -> Result<(), Error> { EventInstance::install_beat_callback(self) }
}
unsafe extern "C" fn timeline_callback(_kind: ffi::FMOD_STUDIO_EVENT_CALLBACK_TYPE, _event: *mut ffi::FMOD_STUDIO_EVENTINSTANCE, parameters: *mut c_void) -> ffi::FMOD_RESULT {
    let properties = unsafe { parameters.cast::<ffi::FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES>().as_ref() };
    music_player::get_timeline_beat_properties(properties.map(|p| TimelineBeat {
        bar: p.bar, beat: p.beat, tempo: p.tempo,
        time_signature_upper: p.timesignatureupper, time_signature_lower: p.timesignaturelower,
    }));
    ffi::FMOD_OK
}
