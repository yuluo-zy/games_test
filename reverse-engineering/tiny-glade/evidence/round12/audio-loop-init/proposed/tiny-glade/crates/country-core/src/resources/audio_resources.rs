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
use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;

/// Native global error-report slot `0x143634d60`: a `BTreeMap<String, ()>`
/// that `AudioCoreInner::set_parameter` (0x140a53db0) inserts the formatted
/// "Couldn't set global FMOD parameter: …" message into before logging. The
/// crate keeps the same map (a `BTreeMap<String, ()>`) so callers/tests can
/// observe the registered failures.
static GLOBAL_AUDIO_PARAM_ERRORS: Mutex<BTreeMap<String, ()>> = Mutex::new(BTreeMap::new());

/// Snapshot of the native global parameter-error report slot.
pub fn global_parameter_errors() -> BTreeMap<String, ()> {
    GLOBAL_AUDIO_PARAM_ERRORS
        .lock()
        .map(|m| m.clone())
        .unwrap_or_default()
}

/// FMOD EventInstance handle interface (injected; real binding via libfmod).
pub trait EventInstance {
    type Error: std::fmt::Debug;
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
    /// TIMELINE_BEAT 回调安装（中央 agent 2026-10-16 仲裁批准补此方法；native.rs
    /// `set_callback(TIMELINE_BEAT)` 已存在，`MusicEvent::install_beat_callback` 依赖它）。
    fn install_beat_callback(&mut self) -> Result<(), Self::Error>;
    /// `EventInstance::release`（AudioInstance Drop，0x14089a280）：FMOD 实例释放。
    /// 默认空实现保持既有 binding 编译兼容；真实 libfmod binding 覆写它，原生 Drop
    /// 对失败 unwrap（panic）。
    fn release(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
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

    /// `stop_on_drop` (0x140a57540): clear the Active arm's stop-on-drop state
    /// (state@+0x28 = 0) so the Drop impl stops the FMOD event when the instance
    /// is dropped. The initial state is 2 ("fire-and-forget": the native Drop
    /// leaves a started one-shot playing and only releases the handle).
    pub fn stop_on_drop(&mut self) {
        if let AudioInstance::Active { state, .. } = self {
            *state = 0;
        }
    }
}

/// Drop (0x14089a280). Native: set the discriminant to Disabled, then when the
/// arm was Active and its stop_on_drop state (@+0x28) != 2, `stop()` (failure is
/// dropped, not unwrapped); `release()` is then unwrapped (failure panics).
impl<E: EventInstance> Drop for AudioInstance<E> {
    fn drop(&mut self) {
        if let AudioInstance::Active { event, state, .. } = self {
            if *state != 2 {
                let _ = event.stop(0);
            }
            event.release().unwrap();
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

/// Original borrowed descriptor is {name, extra_instance_count, stop_delay_ms}.
/// Native140a59243 increments count before collecting owned instances.
#[derive(Clone, Debug)]
pub struct AudioLoopDesc {
    pub name: String,
    /// Additional ring instances; native total is count.wrapping_add(1).
    pub count: usize,
    pub threshold_ms: u32,
}
impl AudioLoopDesc {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), count: 3, threshold_ms: 50 }
    }
}

/// One-shot completion from the delayed FMOD bank loader.
///
/// `AudioCore::new` @140a557d9 constructs `std::sync::mpsc::sync_channel(1)`.
/// `recv` @140a57100 takes the receiver before blocking and unwraps disconnect.
/// The native channel deadline's nanosecond word 1_000_000_000 is `None`, not
/// a one-second timeout (the array receiver @141679f7d skips its clock check).
#[derive(Debug, Resource)]
pub struct AudioSamplesLoadedPromise<T> {
    receiver: std::sync::Mutex<Option<std::sync::mpsc::Receiver<T>>>,
}

impl<T> AudioSamplesLoadedPromise<T> {
    pub fn new(receiver: std::sync::mpsc::Receiver<T>) -> Self {
        Self { receiver: std::sync::Mutex::new(Some(receiver)) }
    }

    /// The original completion can arrive before gameplay finishes loading.
    pub fn channel() -> (std::sync::mpsc::SyncSender<T>, Self) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        (sender, Self::new(receiver))
    }

    /// Consume once. Later calls have no receiver and return immediately.
    pub fn recv(&self) -> Option<T> {
        let receiver = self.receiver.lock().unwrap().take()?;
        Some(receiver.recv().unwrap())
    }
}

#[cfg(test)]
mod audio_samples_lifecycle {
    use super::AudioSamplesLoadedPromise;
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn early_completion_survives_a_late_receiver() {
        let (sender, promise) = AudioSamplesLoadedPromise::channel();
        // Join proves the sender can finish without a simultaneous receiver.
        thread::spawn(move || sender.send(37).unwrap()).join().unwrap();
        thread::sleep(Duration::from_millis(1100));
        assert_eq!(promise.recv(), Some(37));
        assert_eq!(promise.recv(), None);
    }

    #[test]
    fn receiving_waits_for_completion_beyond_the_old_false_deadline() {
        let (sender, promise) = AudioSamplesLoadedPromise::channel();
        let (started, waiting) = mpsc::channel();
        let receiver = thread::spawn(move || {
            started.send(()).unwrap();
            promise.recv()
        });
        waiting.recv().unwrap();
        thread::sleep(Duration::from_millis(1100));
        assert!(!receiver.is_finished(), "the original has no one-second timeout");
        sender.send(71).unwrap();
        assert_eq!(receiver.join().unwrap(), Some(71));
    }

    #[test]
    fn cancelled_gameplay_releases_the_completion_receiver() {
        let (sender, promise) = AudioSamplesLoadedPromise::<()>::channel();
        drop(promise);
        assert!(sender.send(()).is_err());
    }

    #[test]
    fn failed_loader_is_not_silently_accepted_as_loaded_samples() {
        let (sender, promise) = AudioSamplesLoadedPromise::<()>::channel();
        drop(sender);
        assert!(std::panic::catch_unwind(|| promise.recv()).is_err());
        // Original takes the receiver before unwrapping the receive result.
        assert_eq!(promise.recv(), None);
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

    /// `load_sample_data` (0x140a53870). Native asserts the bank is loaded first
    /// (`*loaded & 1 == 0 → unwrap_failed`), logs at trace level
    /// (`MAX_LOG_LEVEL_FILTER > 4`) "Loading sample data of FMOD bank: {path}",
    /// then calls FMOD `Bank::load_sample_data` and unwraps it.
    pub fn load_sample_data<S: Studio>(&mut self, studio: &mut S) {
        assert!(self.loaded, "Bank::load_sample_data called before the bank is loaded");
        log::trace!(target: "country_core::resources::audio_resources", "Loading sample data of FMOD bank: {}", self.path);
        studio.load_bank_sample_data(&self.path).unwrap();
    }
}

/// FMOD Studio handle interface (injected).
pub trait Studio {
    type Error: std::fmt::Display + std::fmt::Debug;
    /// The injected FMOD event-instance type (`AudioInstance`'s `E`).
    type Event: EventInstance<Error = Self::Error> + Send + Sync + 'static;
    /// The injected FMOD VCA handle (`set_vca_volume` 0x140a54810).
    type Vca: VcaHandle<Error = Self::Error>;
    fn load_bank_file(&mut self, path: &str) -> Result<(), Self::Error>;
    fn load_sample_data(&mut self) -> Result<(), Self::Error>;
    /// Sample data belongs to the selected bank, rather than whichever bank
    /// happened to be loaded most recently. Legacy injected bindings can keep
    /// their single-bank operation; the production FMOD owner overrides this.
    fn load_bank_sample_data(&mut self, _path: &str) -> Result<(), Self::Error> {
        self.load_sample_data()
    }
    /// `get_vca` (0x140a54810): resolve the VCA by name; the caller logs and
    /// continues on failure (native), then unwraps `Vca::set_volume`.
    fn get_vca(&self, name: &str) -> Result<Self::Vca, Self::Error>;
    /// `AudioCoreInner::update` (0x140a53a00): set the 3D listener (index 0).
    fn set_listener_attributes(&mut self, listener: ListenerAttributes) -> Result<(), Self::Error>;
    /// Studio::update (0x140a53a00 continuation): advance the FMOD Studio.
    fn update(&mut self) -> Result<(), Self::Error>;
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

/// The injected FMOD VCA handle (native `libfmod::Vca`).
pub trait VcaHandle {
    type Error: std::fmt::Debug;
    /// `Vca::set_volume` (0x140a54810 continuation); native unwraps the failure.
    fn set_volume(&self, volume: f32) -> Result<(), Self::Error>;
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
/// `AudioCore::new` (0x140a54a40) scans `.bank` files and loads the five boot
/// banks in order; the returned original background closure loads the rest.
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
        {
            let entries = std::fs::read_dir(desktop_dir).unwrap();
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.extension().is_some_and(|e| e == std::ffi::OsStr::new("bank")) {
                    let path_str = path.to_string_lossy().into_owned();
                    let bank = Bank {
                        loaded: false,
                        path: path_str.clone(),
                    };
                    banks.insert(entry.file_name().to_string_lossy().into_owned(), bank);
                }
            }
        }
        let mut core = AudioCore {
            studio: Some(studio),
            banks,
        };
        // Native arrays @142ad27b8: all five metadata loads precede all five
        // sample loads. Loading every theme here was an incorrect old shortcut.
        for name in BOOT_BANKS {
            let bank = core.banks.get_mut(name).expect("missing boot FMOD bank");
            bank.load(core.studio.as_mut().unwrap());
        }
        for name in BOOT_BANKS {
            let bank = core.banks.get_mut(name).expect("missing boot FMOD bank");
            bank.load_sample_data(core.studio.as_mut().unwrap());
        }
        core
    }

    pub fn instantiate(&mut self) -> Option<&mut S> {
        self.studio.as_mut()
    }

    /// `set_vca_volume` (0x140a54810). Native: `get_vca` failure → log + continue;
    /// `Vca::set_volume` failure → unwrap (panic). Both paths return normally.
    pub fn set_vca_volume(&mut self, name: &str, volume: f32) -> Result<(), S::Error> {
        if let Some(studio) = &mut self.studio {
            match studio.get_vca(name) {
                Ok(vca) => {
                    vca.set_volume(volume).unwrap();
                }
                Err(err) => {
                    log::error!(target: "country_core::resources::audio_resources", "Failed to get VCA {}: {}", name, err);
                }
            }
        }
        Ok(())
    }

    /// `AudioCoreInner::update` (0x140a53a00). Native first asserts the listener
    /// orientation is near-orthonormal (`|forward| - 1.0 < 0.001`,
    /// `|up| - 1.0 < 0.001`, `|forward·up| < 0.001`, panics otherwise), then
    /// unwraps `set_listener_attributes`, and on a Studio::update failure logs
    /// "FMOD update error: …" and continues (the Rust result is always `Ok`).
    pub fn update(&mut self, listener: ListenerAttributes) -> Result<(), S::Error> {
        let forward = listener.forward;
        let up = listener.up;
        let fwd_len = (forward[0] * forward[0] + forward[1] * forward[1] + forward[2] * forward[2])
            .sqrt();
        let up_len = (up[0] * up[0] + up[1] * up[1] + up[2] * up[2]).sqrt();
        assert!(!(0.001 <= fwd_len - 1.0), "listener forward must be normalized");
        assert!(!(0.001 <= up_len - 1.0), "listener up must be normalized");
        let dot = forward[0] * up[0] + forward[1] * up[1] + forward[2] * up[2];
        assert!(!(0.001 <= dot.abs()), "listener forward and up must be orthogonal");
        if let Some(studio) = &mut self.studio {
            if let Err(err) = studio.set_listener_attributes(listener) {
                panic!("FMOD set_listener_attributes failed: {err}");
            }
            if let Err(err) = studio.update() {
                log::error!(target: "country_core::resources::audio_resources", "FMOD update error: {err}");
            }
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

    /// Original140a59220 collects fallible creation, then probes the first real
    /// instance's "loop" parameter. Prefix instances release on failure.
    pub fn try_instantiate_loop(&self, desc: &AudioLoopDesc) -> anyhow::Result<AudioLoop<AudioInstance<S::Event>>> {
        let entries: Vec<_> = (0..desc.count.wrapping_add(1))
            .map(|_| {
                let instance = self.instantiate_audio(&desc.name);
                if let AudioInstance::Err(error) = &instance {
                    return Err(anyhow::anyhow!("{error}"));
                }
                Ok(instance)
            }).collect::<anyhow::Result<_>>()?;
        let first = entries.first().expect("original AudioLoop first instance bounds check");
        let uses_loop_parameter = match first.has_parameter("loop") {
            Ok(found) => found,
            Err(error) => {
                log::error!("Couldn't query audio loop parameter for {}: {error:?}", desc.name);
                false
            }
        };
        Ok(AudioLoop::new(entries, desc.threshold_ms, uses_loop_parameter, false))
    }

    /// Startup callers unwrap the original Result. This adapter keeps their
    /// existing return type while retaining creation and prefix-drop failures.
    pub fn instantiate_loop(&self, desc: &AudioLoopDesc) -> AudioLoop<AudioInstance<S::Event>> {
        self.try_instantiate_loop(desc).expect("instantiate original audio loop")
    }

    /// A player owns the same fully initialized ring; no timing/flags are lost.
    pub fn instantiate_loop_player(&self, desc: &AudioLoopDesc) -> AudioLoopPlayer<AudioInstance<S::Event>> {
        AudioLoopPlayer::from_audio(self.instantiate_loop(desc))
    }

    /// `AudioCoreInner::set_parameter` (0x140a53db0): global FMOD parameter.
    /// Native on failure formats "Couldn't set global FMOD parameter: {name} …",
    /// inserts that message into the global report map (slot 0x143634d60) and
    /// logs it, then continues (no propagation). The `Err` is returned so the
    /// caller can observe it; normal callers discard it (`let _ =`).
    pub fn set_parameter(&self, name: &str, value: f32) -> Result<(), S::Error> {
        if let Some(studio) = &self.studio {
            if let Err(err) = studio.set_parameter_by_name(name, value) {
                let message =
                    format!("Couldn't set global FMOD parameter: {name} ({err:?})");
                if let Ok(mut report) = GLOBAL_AUDIO_PARAM_ERRORS.lock() {
                    report.insert(message.clone(), ());
                }
                log::error!(target: "country_core::resources::audio_resources", "{message}");
                return Err(err);
            }
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

/// Original five-entry boot-bank table, in native order.
pub const BOOT_BANKS: [&str; 5] = [
    "Master.strings.bank", "Master.bank", "Master.assets.bank", "UI.bank", "UI.assets.bank",
];

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
        /// Shared post-drop call log (the value's own `calls` is moved into
        /// `Drop`, so drop-tests observe this handle instead). Arc so FakeEvent
        /// stays Send + Sync (Studio::Event requires it).
        log: Option<std::sync::Arc<std::sync::Mutex<Vec<String>>>>,
    }

    impl Default for FakeEvent {
        fn default() -> Self {
            FakeEvent {
                calls: Vec::new(),
                state: PlaybackState::Stopped,
                log: None,
            }
        }
    }

    impl FakeEvent {
        fn with_log(log: std::sync::Arc<std::sync::Mutex<Vec<String>>>) -> Self {
            FakeEvent {
                log: Some(log),
                ..Default::default()
            }
        }

        fn record(&mut self, call: &str) {
            self.calls.push(call.into());
            if let Some(log) = &self.log {
                log.lock().unwrap().push(call.into());
            }
        }
    }

    impl EventInstance for FakeEvent {
        type Error = String;
        fn start(&mut self) -> Result<(), Self::Error> {
            self.record("start");
            self.state = PlaybackState::Playing;
            Ok(())
        }
        fn stop(&mut self, _mode: i32) -> Result<(), Self::Error> {
            self.record("stop");
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
        fn install_beat_callback(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn release(&mut self) -> Result<(), Self::Error> {
            self.record("release");
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakeStudio;
    struct FakeVca;
    impl VcaHandle for FakeVca {
        type Error = String;
        fn set_volume(&self, _v: f32) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    impl Studio for FakeStudio {
        type Error = String;
        type Event = FakeEvent;
        type Vca = FakeVca;
        fn load_bank_file(&mut self, _p: &str) -> Result<(), Self::Error> {
            Ok(())
        }
        fn load_sample_data(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn get_vca(&self, _n: &str) -> Result<Self::Vca, Self::Error> {
            Ok(FakeVca)
        }
        fn set_listener_attributes(&mut self, _l: ListenerAttributes) -> Result<(), Self::Error> {
            Ok(())
        }
        fn update(&mut self) -> Result<(), Self::Error> {
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
    #[should_panic]
    fn load_sample_data_panics_before_the_bank_is_loaded() {
        let mut bank = Bank {
            loaded: false,
            path: "a.bank".into(),
        };
        bank.load_sample_data(&mut FakeStudio::default());
    }

    #[test]
    fn load_sample_data_after_load_is_accepted() {
        let mut bank = Bank {
            loaded: false,
            path: "a.bank".into(),
        };
        bank.load(&mut FakeStudio::default());
        bank.load_sample_data(&mut FakeStudio::default());
        assert!(bank.loaded);
    }

    #[test]
    fn drop_leaves_a_fire_and_forget_one_shot_playing_but_releases_the_handle() {
        // state == 2 (initial): native Drop does not stop, but releases.
        let log = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let instance =
            AudioInstance::<FakeEvent>::active(FakeEvent::with_log(log.clone()), "e");
        drop(instance);
        assert_eq!(*log.lock().unwrap(), ["release"]);
    }

    #[test]
    fn drop_stops_when_stop_on_drop_was_requested() {
        let log = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut instance =
            AudioInstance::<FakeEvent>::active(FakeEvent::with_log(log.clone()), "e");
        instance.stop_on_drop();
        drop(instance);
        assert_eq!(*log.lock().unwrap(), ["stop", "release"]);
    }

    #[test]
    fn stop_on_drop_clears_the_state_flag() {
        let mut instance = AudioInstance::<FakeEvent>::active(FakeEvent::default(), "e");
        let before = match &instance {
            AudioInstance::Active { state, .. } => *state,
            _ => 99,
        };
        assert_eq!(before, 2);
        instance.stop_on_drop();
        let after = match &instance {
            AudioInstance::Active { state, .. } => *state,
            _ => 99,
        };
        assert_eq!(after, 0);
    }

    #[test]
    fn update_asserts_an_orthonormal_listener() {
        let mut core = AudioCore {
            studio: Some(FakeStudio::default()),
            banks: HashMap::new(),
        };
        let ok = ListenerAttributes {
            position: [0.0, 0.0, 0.0],
            forward: [0.0, 0.0, -1.0],
            up: [0.0, 1.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
        };
        core.update(ok).unwrap();
    }

    #[test]
    #[should_panic]
    fn update_rejects_a_non_unit_forward() {
        let mut core = AudioCore {
            studio: Some(FakeStudio::default()),
            banks: HashMap::new(),
        };
        core.update(ListenerAttributes {
            position: [0.0, 0.0, 0.0],
            forward: [0.0, 0.0, -2.0],
            up: [0.0, 1.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn update_rejects_non_orthogonal_axes() {
        let mut core = AudioCore {
            studio: Some(FakeStudio::default()),
            banks: HashMap::new(),
        };
        core.update(ListenerAttributes {
            position: [0.0, 0.0, 0.0],
            forward: [1.0, 0.0, 0.0],
            up: [1.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
        })
        .unwrap();
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
    fn native_disabled_loop_constructor_corpus_retains_all_instances_and_delay() {
        let core = AudioCore::<FakeStudio> { studio: None, banks: HashMap::new() };
        // Actual original constructor+iterator outputs, native-loop-init-fixtures.json.
        for (descriptor_count, actual_count) in [(0,1),(1,2),(3,4),(7,8)] {
            let mut desc = AudioLoopDesc::new("event:/SFX/Player/Walking");
            desc.count = descriptor_count;
            let ring = core.try_instantiate_loop(&desc).unwrap();
            assert_eq!(ring.entries.len(), actual_count);
            assert_eq!(ring.threshold_ms, 50);
            assert!(!ring.uses_loop_parameter);
            assert!(!ring.stops_explicitly);
        }
    }
    #[test]
    fn original_parameter_probe_survives_player_construction() {
        let core = AudioCore { studio: Some(FakeStudio), banks: HashMap::new() };
        let player = core.instantiate_loop_player(&AudioLoopDesc::new("event:/x"));
        assert_eq!(player.audio.entries.len(), 4);
        assert_eq!(player.audio.threshold_ms, 50);
        assert!(player.audio.uses_loop_parameter);
        assert!(!player.audio.stops_explicitly);
        assert!(!player.should_play);
    }

    #[test]
    fn audio_loop_desc_native_default_shape_is_extra_count_3_delay_50() {
        let desc = AudioLoopDesc::new("event:/x");
        assert_eq!(desc.count, 3);
        assert_eq!(desc.threshold_ms, 50);
    }
}

#[cfg(test)]
mod original_loop_initialization {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    struct Trace {
        creates: Vec<String>,
        probes: Vec<(usize, String)>,
        releases: Vec<usize>,
        fail_at: Option<usize>,
        probe: Option<Result<bool, &'static str>>,
    }
    struct RecordedEvent { index: usize, trace: Arc<Mutex<Trace>> }
    impl EventInstance for RecordedEvent {
        type Error = &'static str;
        fn start(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn stop(&mut self, _: i32) -> Result<(), Self::Error> { Ok(()) }
        fn get_playback_state(&self) -> Result<PlaybackState, Self::Error> { Ok(PlaybackState::Stopped) }
        fn set_volume(&mut self, _: f32) -> Result<(), Self::Error> { Ok(()) }
        fn set_parameter_by_name(&mut self, _: &str, _: f32) -> Result<(), Self::Error> { Ok(()) }
        fn has_parameter(&self, name: &str) -> Result<bool, Self::Error> {
            let mut trace = self.trace.lock().unwrap();
            trace.probes.push((self.index, name.to_owned()));
            trace.probe.unwrap_or(Ok(true))
        }
        fn set_3d_attributes(&mut self, _: [f32;3], _: [f32;3], _: [f32;3], _: [f32;3]) -> Result<(), Self::Error> { Ok(()) }
        fn get_timeline_position(&self) -> Result<i32, Self::Error> { Ok(0) }
        fn pause(&mut self, _: bool) -> Result<(), Self::Error> { Ok(()) }
        fn install_beat_callback(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn release(&mut self) -> Result<(), Self::Error> {
            self.trace.lock().unwrap().releases.push(self.index); Ok(())
        }
    }
    struct RecordedStudio(Arc<Mutex<Trace>>);
    struct RecordedVca;
    impl VcaHandle for RecordedVca {
        type Error = &'static str;
        fn set_volume(&self, _: f32) -> Result<(), Self::Error> { Ok(()) }
    }
    impl Studio for RecordedStudio {
        type Error = &'static str;
        type Event = RecordedEvent;
        type Vca = RecordedVca;
        fn load_bank_file(&mut self, _: &str) -> Result<(), Self::Error> { Ok(()) }
        fn load_sample_data(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn get_vca(&self, _: &str) -> Result<Self::Vca, Self::Error> { Ok(RecordedVca) }
        fn set_listener_attributes(&mut self, _: ListenerAttributes) -> Result<(), Self::Error> { Ok(()) }
        fn update(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn set_parameter_by_name(&self, _: &str, _: f32) -> Result<(), Self::Error> { Ok(()) }
        fn create_event_instance(&self, name: &str) -> Result<Option<Self::Event>, Self::Error> {
            let mut trace = self.0.lock().unwrap();
            let index = trace.creates.len();
            trace.creates.push(name.to_owned());
            if trace.fail_at == Some(index) { return Err("creation failure"); }
            Ok(Some(RecordedEvent { index, trace: self.0.clone() }))
        }
    }
    fn setup(trace: Trace) -> (AudioCore<RecordedStudio>, Arc<Mutex<Trace>>) {
        let trace = Arc::new(Mutex::new(trace));
        (AudioCore { studio: Some(RecordedStudio(trace.clone())), banks: HashMap::new() }, trace)
    }

    #[test]
    fn constructor_owns_four_events_probes_only_the_first_and_releases_every_event() {
        for probe in [Ok(true), Ok(false), Err("parameter lookup failure")] {
            let (core, trace) = setup(Trace { probe: Some(probe), ..Default::default() });
            let player = core.instantiate_loop_player(&AudioLoopDesc::new("event:/test"));
            assert_eq!(player.audio.entries.len(), 4);
            assert_eq!(player.audio.threshold_ms, 50);
            assert_eq!(player.audio.uses_loop_parameter, probe == Ok(true));
            assert!(!player.audio.stops_explicitly);
            assert_eq!(trace.lock().unwrap().probes, [(0, "loop".to_owned())]);
            drop(player);
            assert_eq!(trace.lock().unwrap().releases, [0,1,2,3]);
        }
    }
    #[test]
    fn every_creation_failure_releases_the_successful_prefix_without_probing() {
        for fail_at in 0..4 {
            let (core, trace) = setup(Trace { fail_at: Some(fail_at), ..Default::default() });
            assert!(core.try_instantiate_loop(&AudioLoopDesc::new("event:/test")).is_err());
            let trace = trace.lock().unwrap();
            assert_eq!(trace.creates.len(), fail_at + 1);
            assert_eq!(trace.releases, (0..fail_at).collect::<Vec<_>>());
            assert!(trace.probes.is_empty());
        }
    }
    #[test]
    fn real_startup_owners_preserve_original_event_names_ring_size_and_noloop() {
        use crate::systems::audio::{path_audio, sheep_audio, terrain_stairs_audio, decorator_audio};
        let (core, trace) = setup(Trace::default());
        let paths = path_audio::PathAudioState::new(&core);
        for ring in &paths.loops {
            assert_eq!(ring.entries.len(), 4);
            assert_eq!(ring.threshold_ms, 50);
            assert!(ring.uses_loop_parameter && ring.stops_explicitly);
        }
        let sheep = sheep_audio::SheepAudioState::new(&core);
        let stairs = terrain_stairs_audio::TerrainStairsAudioState::new(&core);
        let decorators = decorator_audio::DecoratorAudioState::new(&core);
        let trace = trace.lock().unwrap();
        assert_eq!(trace.creates.len(), (5+1+1+16)*4);
        assert_eq!(&trace.creates[20..24], ["event:/SFX/Sheep/Pet";4]);
        assert_eq!(&trace.creates[24..28], ["event:/SFX/StairDraw_LoopTrem";4]);
        assert_eq!(&trace.creates[88..92], ["event:/SFX/Cosmetics/MoveBuilding";4]);
        drop(trace);
        assert!(!sheep.loop4.stops_explicitly);
        assert!(!stairs.ring.stops_explicitly);
        assert!(!decorators.loops[15].stops_explicitly);
    }
}
