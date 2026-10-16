//! B `resources/audio_resources.rs`: the FMOD audio core state — AudioInstance
//! model, bank table and the AudioCore (FMOD Studio) producer.
//!
//! O-level evidence (chapter audio_resources.rs.md, audio-misc DIGEST §2.1 +
//! ALGORITHMS §2.1):
//! - `AudioInstance` discriminant u64@+0: 0=Disabled / 1=Active / 2=Err(anyhow).
//!   Active layout: `{disc@0, *mut FMOD EventInstance@+8, name String cap-first
//!   @+0x10{cap,ptr,len}, state u8@+0x28 (init 2)}` (0x140a54440 write order).
//! - `start` 0x140a57240: disc==1 → EventInstance::start.
//! - `start_if_not_playing` 0x140a57300: disc&1 → get_playback_state → bit mask
//!   0xb ({Playing,Sustaining,Starting} skip) → start.
//! - `stop` 0x140a57470: disc==1 → stop(0)=STOP_IMMEDIATE; `stop_on_drop`
//!   0x140a57540: Drop when disc==1 → stop.
//! - `is_playing` 0x140a57550: get_playback_state == Playing.
//! - `set_volume` 0x140a575a0 / `set_parameter` 0x140a57670 (PDB name pause
//!   错位) / `set_position` 0x140a57da0 (set_3d_attributes with
//!   forward=(0,0,-1)/up=(0,1,0)/velocity=0, constants 0x142ad0460).
//! - `Bank::load` 0x140a53640: loaded&1 early-return; Studio::load_bank_file
//!   (path@+0x10, flags 0); failure panics "Error loading FMOD bank file:
//!   {path} {err}" (0x142ad25b0); success loaded=1 + handle@+8.
//! - `AudioCore::new` 0x140a54a40: Studio::create → get_core_system →
//!   set_software_format → initialize → scan `assets/audio/Desktop` .bank →
//!   HashMap<String, Bank> → each load + load_sample_data.
//!
//! FMOD Studio/EventInstance calls are injected through the [`EventInstance`]
//! trait (same pattern as `audio_instance::LoopInstance`); the real binding is
//! the game's libfmod.

use crate::resources::audio_instance::{
    AudioLoopPlayer, LoopInstance, PlaybackState, should_start_if_not_playing,
};
use crate::utils::audio_utils::{AudioLoop, LoopEntry};
use bevy_ecs::prelude::Resource;
use std::collections::HashMap;

/// FMOD EventInstance handle interface (injected; real binding via libfmod).
pub trait EventInstance {
    type Error;
    fn start(&mut self) -> Result<(), Self::Error>;
    fn stop(&mut self, mode: i32) -> Result<(), Self::Error>;
    fn get_playback_state(&self) -> Result<PlaybackState, Self::Error>;
    fn set_volume(&mut self, volume: f32) -> Result<(), Self::Error>;
    fn set_parameter_by_name(&mut self, name: &str, value: f32) -> Result<(), Self::Error>;
    fn has_parameter(&self, name: &str) -> Result<bool, Self::Error>;
    fn set_3d_attributes(
        &mut self,
        position: [f32; 3],
        forward: [f32; 3],
        up: [f32; 3],
        velocity: [f32; 3],
    ) -> Result<(), Self::Error>;
    fn get_timeline_position(&self) -> Result<i32, Self::Error>;
    fn pause(&mut self, paused: bool) -> Result<(), Self::Error>;
}

/// AudioInstance. Native discriminant u64@+0: 0=Disabled, 1=Active, 2=Err.
/// This crate models it as a Rust enum; the Active arm mirrors the native
/// layout ({event@+8, name@+0x10, state@+0x28}).
#[derive(Debug)]
pub enum AudioInstance<E: EventInstance> {
    Disabled,
    Active {
        event: E,
        name: String,
        state: u8,
    },
    Err(String),
}

impl<E: EventInstance> AudioInstance<E> {
    pub fn active(event: E, name: impl Into<String>) -> Self {
        AudioInstance::Active {
            event,
            name: name.into(),
            state: 2,
        }
    }

    /// `start` (0x140a57240): only the Active arm forwards to the FMOD instance.
    pub fn start(&mut self) -> Result<(), E::Error> {
        if let AudioInstance::Active { event, .. } = self {
            event.start()?;
        }
        Ok(())
    }

    /// `start_if_not_playing` (0x140a57300): bit mask 0xb skips
    /// {Playing, Sustaining, Starting}.
    pub fn start_if_not_playing(&mut self) -> Result<(), E::Error> {
        if let AudioInstance::Active { event, .. } = self {
            let state = event.get_playback_state()?;
            if should_start_if_not_playing(state as u32) {
                event.start()?;
            }
        }
        Ok(())
    }

    /// `stop` (0x140a57470): STOP_IMMEDIATE (mode 0).
    pub fn stop(&mut self) -> Result<(), E::Error> {
        if let AudioInstance::Active { event, .. } = self {
            event.stop(0)?;
        }
        Ok(())
    }

    /// `is_playing` (0x140a57550).
    pub fn is_playing(&self) -> Result<bool, E::Error> {
        if let AudioInstance::Active { event, .. } = self {
            Ok(event.get_playback_state()? == PlaybackState::Playing)
        } else {
            Ok(false)
        }
    }

    pub fn set_volume(&mut self, volume: f32) -> Result<(), E::Error> {
        if let AudioInstance::Active { event, .. } = self {
            event.set_volume(volume)?;
        }
        Ok(())
    }

    /// `set_parameter` (0x140a57670).
    pub fn set_parameter(&mut self, name: &str, value: f32) -> Result<(), E::Error> {
        if let AudioInstance::Active { event, .. } = self {
            event.set_parameter_by_name(name, value)?;
        }
        Ok(())
    }

    /// `set_position` (0x140a57da0): forward=(0,0,-1), up=(0,1,0), velocity 0.
    pub fn set_position(&mut self, position: [f32; 3]) -> Result<(), E::Error> {
        if let AudioInstance::Active { event, .. } = self {
            event.set_3d_attributes(position, [0.0, 0.0, -1.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0])?;
        }
        Ok(())
    }

    pub fn pause(&mut self, paused: bool) -> Result<(), E::Error> {
        if let AudioInstance::Active { event, .. } = self {
            event.pause(paused)?;
        }
        Ok(())
    }

    pub fn has_parameter(&self, name: &str) -> Result<bool, E::Error> {
        if let AudioInstance::Active { event, .. } = self {
            event.has_parameter(name)
        } else {
            Ok(false)
        }
    }

    pub fn get_timeline_position(&self) -> Result<i32, E::Error> {
        if let AudioInstance::Active { event, .. } = self {
            event.get_timeline_position()
        } else {
            Ok(0)
        }
    }
}

/// `stop_on_drop` (0x140a57540): the Active arm stops when dropped.
impl<E: EventInstance> Drop for AudioInstance<E> {
    fn drop(&mut self) {
        if let AudioInstance::Active { event, .. } = self {
            let _ = event.stop(0);
        }
    }
}

/// Loop-player entry adapter for [`AudioInstance`] (ring model, utils::AudioLoop).
impl<E: EventInstance> LoopEntry for AudioInstance<E> {
    type Error = E::Error;
    fn is_active(&self) -> bool {
        matches!(self, AudioInstance::Active { .. })
    }
    fn set_parameter(&mut self, name: &str, value: f32) {
        let _ = AudioInstance::set_parameter(self, name, value);
    }
    fn start(&mut self) -> Result<(), Self::Error> {
        AudioInstance::start(self)
    }
    fn stop(&mut self, _mode: crate::utils::audio_utils::StopMode) -> Result<(), Self::Error> {
        AudioInstance::stop(self)
    }
}

/// Loop-player entry adapter for [`AudioInstance`] (should-play model,
/// wall_audio_utils::AudioLoopPlayer).
impl<E: EventInstance> LoopInstance for AudioInstance<E> {
    type Error = E::Error;
    fn is_active(&self) -> bool {
        matches!(self, AudioInstance::Active { .. })
    }
    fn set_position(&mut self, position: [f32; 3]) {
        let _ = AudioInstance::set_position(self, position);
    }
    fn set_parameter(&mut self, name: &str, value: f32) {
        let _ = AudioInstance::set_parameter(self, name, value);
    }
    fn set_volume(&mut self, volume: f32) -> Result<(), Self::Error> {
        AudioInstance::set_volume(self, volume)
    }
    fn start(&mut self) -> Result<(), Self::Error> {
        AudioInstance::start(self)
    }
    fn stop(&mut self) -> Result<(), Self::Error> {
        AudioInstance::stop(self)
    }
}

/// `AudioCore::instantiate_loop` descriptor (O: `{name, count=3, max=50}`,
/// 0x140a59220; the ring flags/threshold are 待取证 — library callers supply
/// their own threshold/flag policy via [`AudioLoop`]'s constructor).
#[derive(Clone, Debug)]
pub struct AudioLoopDesc {
    /// FMOD event path (e.g. `event:/SFX/Wall Draw`).
    pub name: String,
    /// Number of event instances in the loop (`count`, O: 3).
    pub count: usize,
    /// Soft cap for recycling (`max`, O: 50).
    pub max: usize,
    /// Ring stop threshold in ms (native default 待取证; 0 = toggle immediately).
    pub threshold_ms: u32,
    /// Whether the loop sets the `loop` parameter on start/stop (O: used by
    /// AudioLoop; native init default 待取证).
    pub uses_loop_parameter: bool,
    /// Whether the loop explicitly `stop`s the FMOD instance (O; default 待取证).
    pub stops_explicitly: bool,
}

impl AudioLoopDesc {
    /// Native init default shape (count=3, max=50).
    pub fn new(name: impl Into<String>) -> Self {
        AudioLoopDesc {
            name: name.into(),
            count: 3,
            max: 50,
            threshold_ms: 0,
            uses_loop_parameter: false,
            stops_explicitly: false,
        }
    }
}

/// `AudioSamplesLoadedPromise::recv` (0x140a57100): crossbeam three-channel
/// `recv_timeout(1s)`; timeout discards the payload and returns normally,
/// a disconnected channel unwraps (panics) like the original.
#[derive(Debug)]
pub struct AudioSamplesLoadedPromise<T> {
    receiver: crossbeam_channel::Receiver<T>,
}

impl<T> AudioSamplesLoadedPromise<T> {
    pub fn new(receiver: crossbeam_channel::Receiver<T>) -> Self {
        Self { receiver }
    }

    /// O: `recv_timeout(1_000_000_000 ns)`; timeout → `None` (payload dropped).
    pub fn recv(&self) -> Option<T> {
        match self.receiver.recv_timeout(std::time::Duration::from_secs(1)) {
            Ok(payload) => Some(payload),
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => None,
            Err(crossbeam_channel::RecvTimeoutError::Disconnected) => {
                panic!("called Result::unwrap() on an Err value")
            }
        }
    }
}

/// FMOD bank. Native `loaded u8@+0`, handle@+8, path String@+0x10.
#[derive(Clone, Debug)]
pub struct Bank {
    pub loaded: bool,
    pub path: String,
}

impl Bank {
    /// `load` (0x140a53640): early-return when already loaded; panics on
    /// failure with the original message.
    pub fn load<S: Studio>(&mut self, studio: &mut S) {
        if self.loaded {
            return;
        }
        if let Err(err) = studio.load_bank_file(&self.path) {
            panic!("Error loading FMOD bank file: {} {}", self.path, err);
        }
        self.loaded = true;
    }

    /// `load_sample_data` (0x140a53870).
    pub fn load_sample_data<S: Studio>(&mut self, studio: &mut S) {
        log::info!(target: "country_core::resources::audio_resources", "Loading sample data of FMOD bank {}", self.path);
        studio.load_sample_data();
    }
}

/// FMOD Studio handle interface (injected).
pub trait Studio {
    type Error: std::fmt::Display;
    /// The injected FMOD event-instance type (`AudioInstance`'s `E`).
    type Event: EventInstance<Error = Self::Error> + Send + Sync + 'static;
    fn load_bank_file(&mut self, path: &str) -> Result<(), Self::Error>;
    fn load_sample_data(&mut self);
    /// `set_vca_volume` (0x140a54810): get_vca → Vca::set_volume.
    fn set_vca_volume(&mut self, name: &str, volume: f32) -> Result<(), Self::Error>;
    /// `AudioCoreInner::update` (0x140a53a00): listener attributes →
    /// set_listener_attributes(0) → Studio::update.
    fn update(&mut self, listener: ListenerAttributes) -> Result<(), Self::Error>;
    /// `AudioCoreInner::set_parameter` (0x140a53db0): global FMOD parameter
    /// (libfmod `Studio::set_parameter_by_name` is `&self`).
    fn set_parameter_by_name(&self, name: &str, value: f32) -> Result<(), Self::Error>;
    /// `AudioCoreInner::instantiate_audio` (0x140a54440) producer: get_event →
    /// create_instance. `Ok(None)` is the disabled-studio branch.
    fn create_event_instance(
        &self,
        name: &str,
    ) -> Result<Option<Self::Event>, Self::Error>;
}

/// 3D listener attributes passed to Studio::update (O: camera rotation+position
/// in audio_update, fixed default in boot_audio_update).
#[derive(Clone, Copy, Debug)]
pub struct ListenerAttributes {
    pub position: [f32; 3],
    pub forward: [f32; 3],
    pub up: [f32; 3],
    pub velocity: [f32; 3],
}

/// AudioCore — the FMOD Studio core. Native: core handle@+0 (empty → Disabled
/// variant), banks HashMap<String, Bank>, and the inner volume/parameter state.
/// `AudioCore::new` (0x140a54a40) creates the Studio, initializes it and scans
/// `assets/audio/Desktop` `.bank` files, loading each bank and its sample data.
#[derive(Debug)]
pub struct AudioCore<S> {
    pub studio: Option<S>,
    pub banks: HashMap<String, Bank>,
}

impl<S: Send + Sync + 'static> Resource for AudioCore<S> {}

impl<S: Studio> AudioCore<S> {
    /// `new` (0x140a54a40). `desktop_dir` is `assets/audio/Desktop` in the
    /// original (scanned for `.bank` files).
    pub fn new(studio: S, desktop_dir: &std::path::Path) -> Self {
        let mut banks = HashMap::new();
        if let Ok(entries) = std::fs::read_dir(desktop_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == std::ffi::OsStr::new("bank")) {
                    let path_str = path.to_string_lossy().into_owned();
                    let bank = Bank {
                        loaded: false,
                        path: path_str.clone(),
                    };
                    banks.insert(path_str, bank);
                }
            }
        }
        let mut core = AudioCore {
            studio: Some(studio),
            banks,
        };
        // Load each bank immediately (O: scan → each load + load_sample_data).
        for bank in core.banks.values_mut() {
            if let Some(s) = &mut core.studio {
                bank.load(s);
                bank.load_sample_data(s);
            }
        }
        core
    }

    pub fn instantiate(&mut self) -> Option<&mut S> {
        self.studio.as_mut()
    }

    /// `set_vca_volume` (0x140a54810).
    pub fn set_vca_volume(&mut self, name: &str, volume: f32) -> Result<(), S::Error> {
        if let Some(studio) = &mut self.studio {
            studio.set_vca_volume(name, volume)?;
        }
        Ok(())
    }

    /// `AudioCoreInner::update` (0x140a53a00).
    pub fn update(&mut self, listener: ListenerAttributes) -> Result<(), S::Error> {
        if let Some(studio) = &mut self.studio {
            studio.update(listener)?;
        }
        Ok(())
    }

    /// `instantiate_audio` (0x140a54440): the core is empty → Disabled variant.
    pub fn is_connected(&self) -> bool {
        self.studio.is_some()
    }

    /// `AudioCore::instantiate_audio` (0x140a55f40): non-empty core → Inner
    /// producer (Active); empty → Disabled.
    pub fn instantiate_audio(&self, name: &str) -> AudioInstance<S::Event> {
        match &self.studio {
            Some(studio) => match studio.create_event_instance(name) {
                Ok(Some(event)) => AudioInstance::active(event, name),
                Ok(None) => AudioInstance::Disabled,
                Err(err) => AudioInstance::Err(err.to_string()),
            },
            None => AudioInstance::Disabled,
        }
    }

    /// `AudioCore::instantiate_loop` (0x140a59220): build `desc.count` instances
    /// for the ring loop player (tree/water/roof/... frame-end start_if_not_playing/stop).
    pub fn instantiate_loop(&self, desc: &AudioLoopDesc) -> AudioLoop<AudioInstance<S::Event>> {
        let entries = (0..desc.count)
            .map(|_| self.instantiate_audio(&desc.name))
            .collect();
        AudioLoop::new(entries, desc.threshold_ms, desc.uses_loop_parameter, desc.stops_explicitly)
    }

    /// `AudioCore::instantiate_loop` (0x140a59220) as an [`AudioLoopPlayer`]
    /// (request_play + frame-end should-play model; wall/building/stairs init).
    pub fn instantiate_loop_player(
        &self,
        desc: &AudioLoopDesc,
    ) -> AudioLoopPlayer<AudioInstance<S::Event>> {
        let instances = (0..desc.count)
            .map(|_| self.instantiate_audio(&desc.name))
            .collect();
        AudioLoopPlayer::new(instances)
    }

    /// `AudioCoreInner::set_parameter` (0x140a53db0): global FMOD parameter.
    pub fn set_parameter(&self, name: &str, value: f32) -> Result<(), S::Error> {
        if let Some(studio) = &self.studio {
            studio.set_parameter_by_name(name, value)?;
        }
        Ok(())
    }

    /// `AudioCoreInner::set_master_volume` (0x140a54810 via VCA): Master VCA.
    pub fn set_master_volume(&mut self, volume: f32) -> Result<(), S::Error> {
        self.set_vca_volume("vca:/Master", volume)
    }

    /// `AudioCoreInner::set_music_volume`: Music VCA.
    pub fn set_music_volume(&mut self, volume: f32) -> Result<(), S::Error> {
        self.set_vca_volume("vca:/Music", volume)
    }

    /// `AudioCoreInner::set_dawn_jingle_volume`: Music-DawnJingle VCA.
    pub fn set_dawn_jingle_volume(&mut self, volume: f32) -> Result<(), S::Error> {
        self.set_vca_volume("vca:/Music-DawnJingle", volume)
    }

    /// `AudioCoreInner::set_sfx_volume`: SFX VCA.
    pub fn set_sfx_volume(&mut self, volume: f32) -> Result<(), S::Error> {
        self.set_vca_volume("vca:/SFX", volume)
    }

    /// `AudioCoreInner::set_ambience_volume`: Amb VCA.
    pub fn set_ambience_volume(&mut self, volume: f32) -> Result<(), S::Error> {
        self.set_vca_volume("vca:/Amb", volume)
    }

    /// `AudioCore::visualize_sounds` (0x140a55ff0): debug visualization — the
    /// native iterates active instances, reads their positions and draws 3D
    /// markers (thread_local access). The marker rendering is render-domain;
    /// this library surface exposes the instance positions for the host debug
    /// drawer.
    pub fn visualize_sounds(&self) -> Vec<[f32; 3]> {
        // The FMOD EventInstance 3D position read requires an injected
        // getter; the native reads positions out of the instances directly.
        // 待取证: exact per-instance position access — returns an empty list
        // here, the host renderer supplies the draw.
        Vec::new()
    }
}

/// `DelayedBankLoader` (native line 64, inlined): defers a bank load until the
/// sample data is available. The PDB name `DelayedBankLoader::load` is 错位
/// (it actually resolves to `AudioCoreInner::update`); the real loader's
/// structure is 待取证 — modeled here as a small deferred-load wrapper.
#[derive(Clone, Debug)]
pub struct DelayedBankLoader {
    /// Banks whose sample data is still pending.
    pub pending: Vec<String>,
}

impl Default for DelayedBankLoader {
    fn default() -> Self {
        DelayedBankLoader {
            pending: Vec::new(),
        }
    }
}

impl DelayedBankLoader {
    /// Register a bank for deferred sample-data loading.
    pub fn defer(&mut self, path: impl Into<String>) {
        self.pending.push(path.into());
    }

    /// Load sample data for every deferred bank (I; native 待取证).
    pub fn load<S: Studio>(&mut self, studio: &mut S) {
        for path in &self.pending {
            if let Some(bank) = self.pending_bank(path) {
                let _ = bank;
                let _ = studio.load_sample_data();
            }
        }
        self.pending.clear();
    }

    fn pending_bank(&self, _path: &str) -> Option<()> {
        // The bank-table lookup lives in AudioCore; the loader's exact coupling
        // to `AudioCoreInner` is 待取证.
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FakeEvent {
        calls: Vec<String>,
        state: PlaybackState,
    }

    impl Default for FakeEvent {
        fn default() -> Self {
            FakeEvent {
                calls: Vec::new(),
                state: PlaybackState::Stopped,
            }
        }
    }

    impl EventInstance for FakeEvent {
        type Error = String;
        fn start(&mut self) -> Result<(), Self::Error> {
            self.calls.push("start".into());
            self.state = PlaybackState::Playing;
            Ok(())
        }
        fn stop(&mut self, _mode: i32) -> Result<(), Self::Error> {
            self.calls.push("stop".into());
            self.state = PlaybackState::Stopped;
            Ok(())
        }
        fn get_playback_state(&self) -> Result<PlaybackState, Self::Error> {
            Ok(self.state)
        }
        fn set_volume(&mut self, _v: f32) -> Result<(), Self::Error> {
            Ok(())
        }
        fn set_parameter_by_name(&mut self, _n: &str, _v: f32) -> Result<(), Self::Error> {
            Ok(())
        }
        fn has_parameter(&self, _n: &str) -> Result<bool, Self::Error> {
            Ok(true)
        }
        fn set_3d_attributes(
            &mut self,
            _p: [f32; 3],
            _f: [f32; 3],
            _u: [f32; 3],
            _v: [f32; 3],
        ) -> Result<(), Self::Error> {
            Ok(())
        }
        fn get_timeline_position(&self) -> Result<i32, Self::Error> {
            Ok(0)
        }
        fn pause(&mut self, _p: bool) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakeStudio;
    impl Studio for FakeStudio {
        type Error = String;
        type Event = FakeEvent;
        fn load_bank_file(&mut self, _p: &str) -> Result<(), Self::Error> {
            Ok(())
        }
        fn load_sample_data(&mut self) {}
        fn set_vca_volume(&mut self, _n: &str, _v: f32) -> Result<(), Self::Error> {
            Ok(())
        }
        fn update(&mut self, _l: ListenerAttributes) -> Result<(), Self::Error> {
            Ok(())
        }
        fn set_parameter_by_name(&self, _n: &str, _v: f32) -> Result<(), Self::Error> {
            Ok(())
        }
        fn create_event_instance(
            &self,
            _n: &str,
        ) -> Result<Option<Self::Event>, Self::Error> {
            Ok(Some(FakeEvent::default()))
        }
    }

    #[test]
    fn disabled_instance_never_forwards_to_fmod() {
        let mut instance: AudioInstance<FakeEvent> = AudioInstance::Disabled;
        instance.start().unwrap();
        instance.stop().unwrap();
        instance.start_if_not_playing().unwrap();
        assert!(!instance.is_playing().unwrap());
    }

    #[test]
    fn start_if_not_playing_skips_playing_sustaining_starting() {
        for state in [
            PlaybackState::Playing,
            PlaybackState::Sustaining,
            PlaybackState::Starting,
        ] {
            let mut instance = AudioInstance::active(
                FakeEvent {
                    state,
                    ..Default::default()
                },
                "e",
            );
            instance.start_if_not_playing().unwrap();
            let calls = match &instance {
                AudioInstance::Active { event, .. } => event.calls.clone(),
                _ => vec![],
            };
            assert!(calls.is_empty(), "{state:?} should skip start");
        }
        let mut stopped = AudioInstance::active(FakeEvent::default(), "e");
        stopped.start_if_not_playing().unwrap();
        let calls = match &stopped {
            AudioInstance::Active { event, .. } => event.calls.clone(),
            _ => vec![],
        };
        assert_eq!(calls, ["start"]);
    }

    #[test]
    fn bank_load_marks_loaded_after_studio_accepts() {
        let mut bank = Bank {
            loaded: false,
            path: "a.bank".into(),
        };
        bank.load(&mut FakeStudio::default());
        assert!(bank.loaded);
    }

    #[test]
    fn instantiate_audio_produces_active_instance_with_studio_event() {
        let core = AudioCore {
            studio: Some(FakeStudio::default()),
            banks: HashMap::new(),
        };
        let instance = core.instantiate_audio("event:/x");
        match &instance {
            AudioInstance::Active { name, state, .. } => {
                assert_eq!(name, "event:/x");
                assert_eq!(*state, 2);
            }
            other => panic!("expected Active, got {other:?}"),
        }
    }

    #[test]
    fn instantiate_audio_without_studio_is_disabled() {
        let core = AudioCore::<FakeStudio> {
            studio: None,
            banks: HashMap::new(),
        };
        assert!(matches!(core.instantiate_audio("event:/x"), AudioInstance::Disabled));
    }

    #[test]
    fn audio_loop_desc_native_default_shape_is_count_3_max_50() {
        let desc = AudioLoopDesc::new("event:/x");
        assert_eq!(desc.count, 3);
        assert_eq!(desc.max, 50);
    }
}
