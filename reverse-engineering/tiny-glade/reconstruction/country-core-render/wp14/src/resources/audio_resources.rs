//! resources/audio_resources.rs — 音频核心状态（FMOD 胶水的游戏自有侧）。
//!
//! PDB 命名实体（O）：`AudioCore`、`AudioCoreInner`、`AudioInstance`、
//! `ActiveAudioInstances`、`Bank`、`DelayedBankLoader`、`AudioSamplesLoaded`、
//! `AudioSamplesLoadedPromise`。
//!
//! 反汇编证据（O）：
//! - `AudioCoreInner::new_impl`（行 101，经 `AudioCore::new` 0x140a54a40 内联）：
//!   `Studio::create` → `get_core_system` → `set_software_format`（Mixer 日志）→
//!   `Studio::initialize` → 扫 `assets/audio/Desktop` 的 `.bank` → `HashMap<String, Bank>`，
//!   每银行 `Bank::load` + `Bank::load_sample_data`。
//! - `AudioCoreInner::update`（0x140a53a00）：`set_listener_attributes` + `Studio::update`。
//! - `AudioCoreInner::set_vca_volume`（0x140a54810）/ `set_parameter`（0x140a53db0）。
//! - `AudioCoreInner::instantiate_audio`（0x140a54440）。
//! - `AudioInstance` 判别：`0=Disabled{name,state}`、`1=Active{event,name,state}`、
//!   `2=Err(anyhow)`（O 反汇编；源布局 U）。方法（start/stop/pause/resume/is_paused/
//!   is_playing/get_timeline_position/stop_immediately/stop_on_drop）全部 libfmod 边界。
//! - `impl$10`（`impl AudioInstance`）：`set_volume/set_parameter/set_position/has_parameter`。
//!
//! FMOD/libfmod 为 REUSED 依赖；此处只标注调用边界，不还原 libfmod。

#![allow(dead_code, unused_imports)]

use std::path::PathBuf;
use std::sync::Arc;

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 音频核心（Resource）。`inner @+0x10 = Option<Arc<AudioCoreInner>>`（I 布局，O 反汇编）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AudioCore {
    /// @+0 tag（O 存在；语义 U）。
    pub _tag: Gap,
    /// @+0x10 内层（O：`AudioCore::update` 读 `[+0x10]`；`instantiate_audio` 判 `[+0]`）。
    pub inner: Option<Arc<AudioCoreInner>>,
}

impl AudioCore {
    /// 内层内核（None 时为空内核路径，O）。
    pub fn inner(&self) -> Option<&AudioCoreInner> {
        self.inner.as_deref()
    }
}

/// 音频内核（O 名；内部字段 U）。
#[derive(Debug, Default)]
pub struct AudioCoreInner {
    /// FMOD Studio/Core System + 银行表等（U 布局）。
    pub _gap: Gap,
    /// 银行表（O：`HashMap<String, Bank>`，`AudioCore::new` 内 `extend` 证据）。
    pub banks: hashbrown::HashMap<String, Bank>,
}

impl AudioCoreInner {
    /// `AudioCoreInner::new_impl`（行 101，O）：创建 FMOD Studio，加载 `.bank` 银行。
    pub fn new_impl() -> Self {
        // libfmod::Studio::create / get_core_system / set_software_format / initialize /
        // load_bank_file（边界，不还原）。
        Self {
            _gap: Gap,
            banks: hashbrown::HashMap::new(),
        }
    }

    /// `AudioCoreInner::update`（0x140a53a00，O）：把听者位姿推到 FMOD 并 `Studio::update`。
    ///
    /// 听者结构：`{position: Vec3, forward: Vec3, up: Vec3}`（I；`update` 校验 position/
    /// forward 单位正交，panic 断言）。
    pub fn update(&self, _listener: ListenerAttributes) {
        // libfmod::Studio::set_listener_attributes + Studio::update（边界）。
        let _ = _listener;
    }

    /// `AudioCoreInner::set_vca_volume`（0x140a54810，O）：`Studio::get_vca(name)` → `Vca::set_volume(vol)`。
    pub fn set_vca_volume(&self, _name: &str, _volume: f32) {
        // libfmod 边界。
    }

    /// `AudioCoreInner::set_parameter`（0x140a53db0，O）：`Studio::set_parameter_by_name(name, value)`。
    /// 失败 → anyhow（串 `Couldn't set global FMOD paramet…`）+ 全局 panic 报告 BTreeMap。
    pub fn set_parameter(&self, _name: &str, _value: f32) {
        // libfmod 边界。
    }

    /// `AudioCoreInner::instantiate_audio`（0x140a54440，O）：`Studio::get_event(name)` →
    /// `EventDescription::create_instance()` → `AudioInstance::Active{event,name,state=2}`。
    pub fn instantiate_audio(&self, name: &str) -> Result<AudioInstance, anyhow::Error> {
        // libfmod 边界；Ok 变体装填 {event, name, state}。
        Ok(AudioInstance::Active {
            name: name.to_string(),
            state: 2,
            _event: Gap,
        })
    }
}

/// 听者位姿（I：position/forward/up 三向量，`update` 校验正交）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ListenerAttributes {
    pub position: glam::Vec3,
    pub forward: glam::Vec3,
    pub up: glam::Vec3,
}

/// 音频事件实例（O 判别 `0/1/2`；源布局 U，字段为推断）。
///
/// 反汇编（O）：`AudioCore::instantiate_audio` 无内核路径写判别 0 并保留 name；
/// `AudioCoreInner::instantiate_audio` 成功写 1、失败写 2。`start/stop/…` 方法
/// 均以判别为 1（Active）才调 libfmod。
#[derive(Debug)]
pub enum AudioInstance {
    /// 判别 0：无内核/禁用（I 语义；`{name, state}` 字段 O 布局）。
    Disabled { name: String, state: u8 },
    /// 判别 1：活动 FMOD 事件（`{event @+8, name String @+0x10, state u8 @+0x28}`，O 布局）。
    Active {
        /// FMOD EventInstance 句柄（U；libfmod 边界）。
        _event: Gap,
        /// 事件名（O；`Studio::get_event(name)`）。
        name: String,
        /// state u8 @+0x28（O 初始 2；语义 U）。
        state: u8,
    },
    /// 判别 2：构造失败（anyhow，O）。
    Err(anyhow::Error),
}

impl AudioInstance {
    /// 事件名（O）。
    pub fn name(&self) -> &str {
        match self {
            AudioInstance::Disabled { name, .. }
            | AudioInstance::Active { name, .. } => name.as_str(),
            AudioInstance::Err(_) => "",
        }
    }

    /// `AudioInstance::start`（0x140a57240，O）：Active 时 `EventInstance::start()`。
    pub fn start(&self) {
        // libfmod::EventInstance::start（边界）。
        let _ = self;
    }

    /// `AudioInstance::stop`（0x140a57470，O）：Active 时 `EventInstance::stop()`。
    pub fn stop(&self) {
        let _ = self;
    }

    /// `AudioInstance::start_if_not_playing`（0x140a57300，O）：`get_playback_state`，未播则 start。
    pub fn start_if_not_playing(&self) {
        let _ = self;
    }

    /// `AudioInstance::pause`（行 562，O）。
    pub fn pause(&self) {
        let _ = self;
    }

    /// `AudioInstance::resume`（行 581，O）。
    pub fn resume(&self) {
        let _ = self;
    }

    /// `AudioInstance::is_paused`（行 572，O）。
    pub fn is_paused(&self) -> bool {
        false
    }

    /// `AudioInstance::is_playing`（0x140a57550，O）：`get_playback_state`。
    pub fn is_playing(&self) -> bool {
        false
    }

    /// `AudioInstance::get_timeline_position`（行 603，O）。
    pub fn get_timeline_position(&self) -> u64 {
        0
    }

    /// `AudioInstance::stop_immediately`（行 544，O）。
    pub fn stop_immediately(&self) {
        let _ = self;
    }

    /// `AudioInstance::set_volume`（impl$10，0x140a575a0，O）。
    pub fn set_volume(&self, _volume: f32) {
        // libfmod::EventInstance::set_volume（边界）。
    }

    /// `AudioInstance::set_parameter`（impl$10，0x140a57670，O）：`EventInstance::set_parameter_by_name`。
    pub fn set_parameter(&self, _name: &str, _value: f32) {
        // libfmod 边界；失败串 `Failed to set parameter named … on`。
    }

    /// `AudioInstance::set_position`（impl$10，0x140a57da0，O）：`set_3d_attributes`
    /// （forward=(0,0,-1)、up=(0,1,0)、vel=0）。
    pub fn set_position(&self, _position: glam::Vec3) {
        // libfmod::EventInstance::set_3d_attributes（边界）。
    }

    /// `AudioInstance::has_parameter`（impl$10，行 621，O）。
    pub fn has_parameter(&self, _name: &str) -> bool {
        false
    }
}

impl Drop for AudioInstance {
    /// `stop_on_drop`（0x140a57500 区段，O）：Active 时停止 FMOD 事件。
    fn drop(&mut self) {
        let _ = self;
    }
}

/// 活动实例集合（O 名；terrain_stairs 经 `Vec::retain(|i| i.is_playing())` 清理）。
#[derive(Debug, Default)]
pub struct ActiveAudioInstances(pub Vec<AudioInstance>);

/// FMOD 银行（O 名；`Bank::load` 0x140a53640：`Studio::load_bank_file(path)`）。
#[derive(Debug)]
pub struct Bank {
    /// 已加载标志 @+0（O）。
    pub loaded: bool,
    /// 银行路径 @+0x10（O：`PathBuf`，日志 `std::path::impl$44::fmt`）。
    pub path: PathBuf,
    /// FMOD Bank 句柄（U）。
    pub _bank: Gap,
}

impl Bank {
    /// `Bank::load`（0x140a53640，O）：`Studio::load_bank_file(path)`，成功置 `loaded`。
    pub fn load(&mut self) {
        // libfmod::Studio::load_bank_file（边界）。
        self.loaded = true;
    }

    /// `Bank::load_sample_data`（0x140a53870，O）：`Bank::load_sample_data()`。
    pub fn load_sample_data(&mut self) {
        // libfmod::Bank::load_sample_data（边界）。
    }
}

/// 延迟银行加载器（O 名；`load` 行 64）。
#[derive(Debug, Default)]
pub struct DelayedBankLoader(pub Gap);

impl DelayedBankLoader {
    /// `DelayedBankLoader::load`（行 64，O）：异步加载银行到 `AudioCoreInner`。
    pub fn load(&self) {
        let _ = self;
    }
}

/// 银行样例加载完成信号（O 名；跨线程 mpmc）。
#[derive(Debug)]
pub struct AudioSamplesLoaded(pub Gap);

/// 样例加载 promise（O；`recv` 0x140a57100：array/list/zero Channel 分派）。
#[derive(Resource, Debug, Default)]
pub struct AudioSamplesLoadedPromise(pub Gap);

impl AudioSamplesLoadedPromise {
    /// `recv`（0x140a57100，O）：阻塞等样例加载完成。
    pub fn recv(&self) {
        // std::sync::mpmc（list/array/zero Channel）边界。
        let _ = self;
    }
}

/// `fmod_to_anyhow`（行 680，O）：把 libfmod 错误码转 anyhow。
pub fn fmod_to_anyhow(_error: Gap) -> anyhow::Error {
    anyhow::anyhow!("fmod error")
}
