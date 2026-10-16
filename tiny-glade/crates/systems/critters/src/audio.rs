//! 音频缝（critters 音频消费侧）。
//!
//! 证据：`AudioInstance::start_if_not_playing`（0x140a57300）、`set_position`
//! （0x140a57da0）、`set_volume`（0x140a575a0）、`AudioLoop::current`（0x140a593f0）、
//! `set_parameter`（0x140a57670）、`stop`（0x140a59430）、`AudioCore::instantiate_audio`
//! （0x140a55f40）——均为 country-core `resources::audio_resources` 的 FMOD 实例方法。
//! country-core 的 `AudioInstance<S>` 对 Studio 后端泛型；本模块提供 trait 缝，
//! 宿主装配层用真实 `AudioInstance<S>` 实现（blanket impl 映射各原生调用）。

use bevy_ecs::resource::Resource;
use glam::Vec3;

/// critters 消费的音频方法集（每方法 = 一条已证原生调用）。
pub trait CritterAudioSink: Send + Sync {
    /// `start_if_not_playing`（0x140a57300）：位掩码 0xb 跳过 {Playing,Sustaining,Starting}。
    fn start_if_not_playing(&mut self);
    /// `stop`（0x140a59430/0x140a57470）：STOP_IMMEDIATE。
    fn stop(&mut self);
    /// `set_position`（0x140a57da0）：forward=(0,0,-1), up=(0,1,0), velocity=0。
    fn set_position(&mut self, position: Vec3);
    /// `set_volume`（0x140a575a0）。
    fn set_volume(&mut self, volume: f32);
    /// `set_parameter`（0x140a57670）。
    fn set_parameter(&mut self, name: &str, value: f32);
}

/// 惰性实例句柄（Disabled = 未实例化；Active 持有宿主实例）。
pub enum AudioHandle {
    Disabled,
    Active(Box<dyn CritterAudioSink>),
}

impl std::fmt::Debug for AudioHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioHandle::Disabled => f.write_str("AudioHandle::Disabled"),
            AudioHandle::Active(_) => f.write_str("AudioHandle::Active"),
        }
    }
}

impl Default for AudioHandle {
    fn default() -> Self {
        AudioHandle::Disabled
    }
}

impl AudioHandle {
    pub fn start_if_not_playing(&mut self) {
        if let AudioHandle::Active(sink) = self {
            sink.start_if_not_playing();
        }
    }
    pub fn stop(&mut self) {
        if let AudioHandle::Active(sink) = self {
            sink.stop();
        }
    }
    pub fn set_position(&mut self, position: Vec3) {
        if let AudioHandle::Active(sink) = self {
            sink.set_position(position);
        }
    }
    pub fn set_volume(&mut self, volume: f32) {
        if let AudioHandle::Active(sink) = self {
            sink.set_volume(volume);
        }
    }
    pub fn set_parameter(&mut self, name: &str, value: f32) {
        if let AudioHandle::Active(sink) = self {
            sink.set_parameter(name, value);
        }
    }
    pub fn is_active(&self) -> bool {
        matches!(self, AudioHandle::Active(_))
    }
}

/// 鸭音频（Duck::new 装载三事件 event:/SFX/Ducks/{Quack,TakeOff,Land}，O core 章节）。
#[derive(Debug)]
pub struct DuckAudio {
    pub quack: AudioHandle,
    pub takeoff: AudioHandle,
    pub land: AudioHandle,
}

impl Default for DuckAudio {
    fn default() -> Self {
        Self {
            quack: AudioHandle::Disabled,
            takeoff: AudioHandle::Disabled,
            land: AudioHandle::Disabled,
        }
    }
}

impl DuckAudio {
    /// 沿轨迹移动时的循环管理（O：AudioLoop::current + start_if_not_playing +
    /// set_position + set_volume + set_parameter + stop；水面判定按 WaterAudioGrid）。
    pub fn start_loops(&mut self, position: Vec3) {
        self.quack.set_position(position);
        self.quack.start_if_not_playing();
        self.takeoff.set_position(position);
        self.land.set_position(position);
    }
    pub fn stop_loops(&mut self) {
        self.quack.stop();
        self.takeoff.stop();
        self.land.stop();
    }
}

/// 音频核心缝（`AudioCore::instantiate_audio` 0x140a55f40 语义；宿主 FMOD 装配）。
#[derive(Resource, Debug, Default)]
pub struct CritterAudioCore {
    /// 实例化回调（宿主把 `AudioCore<S>::instantiate_audio` 接入）。
    pub instantiate: Option<fn(&str) -> AudioHandle>,
}

impl CritterAudioCore {
    pub fn instantiate(&self, name: &str) -> AudioHandle {
        match self.instantiate {
            Some(f) => f(name),
            None => AudioHandle::Disabled,
        }
    }
}

/// blanket impl：把 country-core 的 `AudioInstance<S>` 映射到本缝
/// （各方法经 core 方法转发到已证原生调用）。
impl<S: country_core::resources::audio_resources::EventInstance + Send + Sync>
    CritterAudioSink for country_core::resources::audio_resources::AudioInstance<S>
{
    fn start_if_not_playing(&mut self) {
        let _ = country_core::resources::audio_resources::AudioInstance::start_if_not_playing(self);
    }
    fn stop(&mut self) {
        let _ = country_core::resources::audio_resources::AudioInstance::stop(self);
    }
    fn set_position(&mut self, position: Vec3) {
        let _ = country_core::resources::audio_resources::AudioInstance::set_position(
            self,
            [position.x, position.y, position.z],
        );
    }
    fn set_volume(&mut self, volume: f32) {
        let _ = country_core::resources::audio_resources::AudioInstance::set_volume(self, volume);
    }
    fn set_parameter(&mut self, name: &str, value: f32) {
        let _ =
            country_core::resources::audio_resources::AudioInstance::set_parameter(self, name, value);
    }
}
