//! 共享支撑类型：游戏自有 Transform（WP6 已恢复 40B 布局）+ GameTime。
//! 数学复用 glam。

#![allow(dead_code)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;
use glam::{Quat, Vec3};

/// 游戏自有 Transform（`utils::components::transform`，WP6 `compute_affine` 反编译 O）：
/// `{translation @+0x0, rotation @+0xC, scale @+0x1C}` = 40B。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

/// 游戏时间资源（O：反编译均读 `*Res<Time> + 0x28` = f32 delta_seconds）。
#[derive(Resource, Debug)]
pub struct GameTime {
    /// `+0x28` 帧 delta 秒（O）。
    pub delta_seconds: f32,
}

impl Default for GameTime {
    fn default() -> Self {
        GameTime { delta_seconds: 0.016 }
    }
}
