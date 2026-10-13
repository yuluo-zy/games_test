//! systems/audio/weather_audio.rs — 天气/季节音频（全局参数）。
//!
//! 反汇编证据（O）：`weather_audio_system`（行 5）签名 `(Res<AudioCore>, Res<Weather>)`
//! （bevy 系统包装 FunctionSystem<void(*)(Res<AudioCore>, Res<Weather>)>，rva 21963360）；
//! 全内联：从 `Weather` 状态经 `AudioCoreInner::set_parameter`（全局 Studio 参数）驱动
//! 天气/季节参数；参数名 U（gap）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::Res;

use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;

/// `Weather`（O 名，weather 域提供；字段 U）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct Weather(pub Gap);

/// `weather_audio_system`（行 5，O）。
pub fn weather_audio_system(core: Res<AudioCore>, _weather: Res<Weather>) {
    if let Some(inner) = core.inner() {
        // 从 Weather 状态设全局 Studio 参数（参数名 U，见 evidence）。
        let _ = inner;
    }
}
