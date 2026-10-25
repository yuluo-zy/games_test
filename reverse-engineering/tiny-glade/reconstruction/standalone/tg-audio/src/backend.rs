//! FMOD boundary. The game-owned glue (AudioCore / AudioLoopPlayer, A §7) is
//! modelled as a trait; libfmod itself is never reimplemented.
use bevy_ecs::prelude::Resource;
use glam::Vec3;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

pub type InstanceId = u32;
pub type LoopId = u32;

/// 3D listener pose. UNKNOWN(evidence: A §2a) how the camera transform maps to it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Listener {
    pub position: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
}

/// Calls the audio systems make on the FMOD glue (A §7 VA list in each doc comment).
pub trait AudioBackend {
    /// `AudioCore::instantiate_loop` 0x140a59220, desc `{name, count, max}`.
    fn instantiate_loop(&mut self, name: &str, count: u64, max: u32) -> LoopId;
    /// `AudioCoreInner::instantiate_audio` 0x140a54440.
    fn instantiate_audio(&mut self, name: &str) -> InstanceId;
    /// `set_position` 0x140a57da0.
    fn set_position(&mut self, id: InstanceId, pos: Vec3);
    /// `set_parameter` 0x140a57670 / 0x140a53db0.
    fn set_parameter(&mut self, id: InstanceId, name: &str, value: f32);
    /// libfmod `EventInstance::set_volume`.
    fn set_volume(&mut self, id: InstanceId, volume: f32);
    /// `start_if_not_playing` 0x140a57300.
    fn start_if_not_playing(&mut self, id: InstanceId);
    /// libfmod `stop` (0x141437440), used for the PauseMenu snapshot.
    fn stop_instance(&mut self, id: InstanceId);
    /// `AudioLoopPlayer::request_play` 0x1409285a0 (position, volume, "editing_speed").
    fn loop_request_play(&mut self, l: LoopId, pos: Vec3, volume: f32, editing_speed: f32);
    /// `set_parameter` on a loop's element instance.
    fn loop_set_parameter(&mut self, l: LoopId, name: &str, value: f32);
    /// `AudioLoop::start_if_not_playing` 0x140a596b0.
    fn loop_start_if_not_playing(&mut self, l: LoopId);
    /// `AudioLoop::stop` 0x140a59430.
    fn loop_stop(&mut self, l: LoopId);
    /// `set_vca_volume` 0x140a54810.
    fn set_vca_volume(&mut self, vca: &str, volume: f32);
    /// `AudioCore::update` 0x140a53a00. `None` = boot default pose (UNKNOWN values, A §2b).
    fn update_listener(&mut self, listener: Option<Listener>);
    /// `MusicPlayer::update` 0x1408b5500 (state machine UNKNOWN beyond the gate, B §6c).
    fn music_player_update(&mut self, beat: f32);
    /// `MusicPlayer::pause` 0x1408b7490.
    fn music_player_pause(&mut self);
}

/// One logged backend call.
#[derive(Clone, Debug, PartialEq)]
pub enum Call {
    InstantiateLoop { name: String, count: u64, max: u32, id: LoopId },
    InstantiateAudio { name: String, id: InstanceId },
    SetPosition { id: InstanceId, pos: Vec3 },
    SetParameter { id: InstanceId, name: String, value: f32 },
    SetVolume { id: InstanceId, volume: f32 },
    Start { id: InstanceId },
    StopInstance { id: InstanceId },
    LoopRequestPlay { l: LoopId, pos: Vec3, volume: f32, editing_speed: f32 },
    LoopSetParameter { l: LoopId, name: String, value: f32 },
    LoopStart { l: LoopId },
    LoopStop { l: LoopId },
    SetVca { vca: String, volume: f32 },
    UpdateListener(Option<Listener>),
    MusicUpdate { beat: f32 },
    MusicPause,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Call::InstantiateLoop { name, count, max, id } => {
                write!(f, "instantiate_loop(\"{name}\", count={count}, max={max}) -> loop#{id}")
            }
            Call::InstantiateAudio { name, id } => write!(f, "instantiate_audio(\"{name}\") -> inst#{id}"),
            Call::SetPosition { id, pos } => write!(f, "inst#{id}.set_position({pos})"),
            Call::SetParameter { id, name, value } => write!(f, "inst#{id}.set_parameter(\"{name}\", {value})"),
            Call::SetVolume { id, volume } => write!(f, "inst#{id}.set_volume({volume})"),
            Call::Start { id } => write!(f, "inst#{id}.start_if_not_playing()"),
            Call::StopInstance { id } => write!(f, "inst#{id}.stop()"),
            Call::LoopRequestPlay { l, pos, volume, editing_speed } => write!(
                f,
                "loop#{l}.request_play(pos={pos}, volume={volume}, editing_speed={editing_speed})"
            ),
            Call::LoopSetParameter { l, name, value } => write!(f, "loop#{l}.set_parameter(\"{name}\", {value})"),
            Call::LoopStart { l } => write!(f, "loop#{l}.start_if_not_playing()"),
            Call::LoopStop { l } => write!(f, "loop#{l}.stop()"),
            Call::SetVca { vca, volume } => write!(f, "set_vca_volume(\"{vca}\", {volume})"),
            Call::UpdateListener(Some(l)) => write!(f, "update_listener(pos={})", l.position),
            Call::UpdateListener(None) => write!(f, "update_listener(boot default)"),
            Call::MusicUpdate { beat } => write!(f, "MusicPlayer::update(beat={beat})"),
            Call::MusicPause => write!(f, "MusicPlayer::pause()"),
        }
    }
}

/// The FMOD boundary as an ECS resource. FMOD itself is not a reusable dependency here, so the
/// systems talk to whatever `AudioBackend` the host installs (a real FMOD binding or a mock).
#[derive(Resource)]
pub struct AudioBackendRes(Box<dyn AudioBackend + Send + Sync>);

impl AudioBackendRes {
    pub fn new(backend: impl AudioBackend + Send + Sync + 'static) -> Self {
        Self(Box::new(backend))
    }
    pub fn get(&mut self) -> &mut dyn AudioBackend {
        &mut *self.0
    }
}

impl Deref for AudioBackendRes {
    type Target = dyn AudioBackend + Send + Sync;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
impl DerefMut for AudioBackendRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

#[derive(Default, Debug)]
struct RecState {
    calls: Vec<Call>,
    next_instance: InstanceId,
    next_loop: LoopId,
}

/// Mock backend that logs every call. Clones share one log, so a test keeps a handle while
/// the `AudioBackendRes` resource owns another.
#[derive(Default, Debug, Clone)]
pub struct RecordingBackend {
    state: Arc<Mutex<RecState>>,
}

impl RecordingBackend {
    pub fn new() -> Self {
        Self::default()
    }
    fn st(&self) -> std::sync::MutexGuard<'_, RecState> {
        self.state.lock().expect("recording backend log poisoned")
    }
    /// Snapshot of the log.
    pub fn calls(&self) -> Vec<Call> {
        self.st().calls.clone()
    }
    /// Drain and return the log.
    pub fn take(&self) -> Vec<Call> {
        std::mem::take(&mut self.st().calls)
    }
    fn push(&self, c: Call) {
        self.st().calls.push(c);
    }
}

impl AudioBackend for RecordingBackend {
    fn instantiate_loop(&mut self, name: &str, count: u64, max: u32) -> LoopId {
        let id = {
            let mut st = self.st();
            let id = st.next_loop;
            st.next_loop += 1;
            id
        };
        self.push(Call::InstantiateLoop { name: name.into(), count, max, id });
        id
    }
    fn instantiate_audio(&mut self, name: &str) -> InstanceId {
        let id = {
            let mut st = self.st();
            let id = st.next_instance;
            st.next_instance += 1;
            id
        };
        self.push(Call::InstantiateAudio { name: name.into(), id });
        id
    }
    fn set_position(&mut self, id: InstanceId, pos: Vec3) {
        self.push(Call::SetPosition { id, pos });
    }
    fn set_parameter(&mut self, id: InstanceId, name: &str, value: f32) {
        self.push(Call::SetParameter { id, name: name.into(), value });
    }
    fn set_volume(&mut self, id: InstanceId, volume: f32) {
        self.push(Call::SetVolume { id, volume });
    }
    fn start_if_not_playing(&mut self, id: InstanceId) {
        self.push(Call::Start { id });
    }
    fn stop_instance(&mut self, id: InstanceId) {
        self.push(Call::StopInstance { id });
    }
    fn loop_request_play(&mut self, l: LoopId, pos: Vec3, volume: f32, editing_speed: f32) {
        self.push(Call::LoopRequestPlay { l, pos, volume, editing_speed });
    }
    fn loop_set_parameter(&mut self, l: LoopId, name: &str, value: f32) {
        self.push(Call::LoopSetParameter { l, name: name.into(), value });
    }
    fn loop_start_if_not_playing(&mut self, l: LoopId) {
        self.push(Call::LoopStart { l });
    }
    fn loop_stop(&mut self, l: LoopId) {
        self.push(Call::LoopStop { l });
    }
    fn set_vca_volume(&mut self, vca: &str, volume: f32) {
        self.push(Call::SetVca { vca: vca.into(), volume });
    }
    fn update_listener(&mut self, listener: Option<Listener>) {
        self.push(Call::UpdateListener(listener));
    }
    fn music_player_update(&mut self, beat: f32) {
        self.push(Call::MusicUpdate { beat });
    }
    fn music_player_pause(&mut self) {
        self.push(Call::MusicPause);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_sequential_and_logged() {
        let mut b = RecordingBackend::new();
        assert_eq!(b.instantiate_audio("a"), 0);
        assert_eq!(b.instantiate_audio("b"), 1);
        assert_eq!(b.instantiate_loop("l", 3, 50), 0);
        b.set_position(0, Vec3::ONE);
        b.set_parameter(0, "p", 0.5);
        b.set_volume(0, 1.0);
        b.start_if_not_playing(0);
        b.stop_instance(0);
        b.loop_request_play(0, Vec3::ZERO, 1.0, 2.0);
        b.loop_set_parameter(0, "q", 1.0);
        b.loop_start_if_not_playing(0);
        b.loop_stop(0);
        b.set_vca_volume("vca:/Master", 1.0);
        b.update_listener(None);
        b.music_player_update(1.0);
        b.music_player_pause();
        let calls = b.take();
        assert_eq!(calls.len(), 16);
        assert!(b.calls().is_empty());
        // Display covers every variant.
        for c in &calls {
            assert!(!c.to_string().is_empty());
        }
        let l = Listener { position: Vec3::X, forward: Vec3::NEG_Z, up: Vec3::Y };
        assert!(Call::UpdateListener(Some(l)).to_string().contains("update_listener"));
    }
}

#[cfg(test)]
mod res_tests {
    use super::*;

    #[test]
    fn resource_forwards_to_shared_log() {
        let rec = RecordingBackend::new();
        let mut res = AudioBackendRes::new(rec.clone());
        res.set_vca_volume("vca:/Master", 0.5);
        res.get().music_player_pause();
        assert_eq!(rec.calls().len(), 2);
    }
}
