//! time.rs — 游戏时间资源（WP12 facade）。
//!
//! `GameTime`（O 名；+0x58 于 bulldoze 动画定速）；`Time`（bevy_time 0.16 离线
//! 缺失 → typed-gap 门面）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 游戏时间（O：`GameTime`；delta / game_time）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct GameTime {
    pub raw: Gap,
}

impl GameTime {
    pub fn delta_seconds(&self) -> f32 {
        0.0
    }
    /// O：+0x58 observed in bulldoze animation pacing.
    pub fn game_time(&self) -> f32 {
        0.0
    }
}

/// bevy_time::Time facade（bevy_time 0.16 absent from offline registry cache）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct Time {
    pub raw: Gap,
}

impl Time {
    pub fn delta_secs(&self) -> f32 {
        0.0
    }
}
