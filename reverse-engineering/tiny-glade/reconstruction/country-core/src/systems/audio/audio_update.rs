//! systems/audio/audio_update.rs — 每帧/启动音频更新（VCA 音量 + 听者位姿）。
//!
//! 反汇编证据（O，audio-events-analysis §2）：
//! - `audio_update_system`（0x1409c8dd0，行 11–34）：`Res<AudioCore> + Res<相机变换资源> +
//!   Res<AudioSettings>`。5×`set_vca_volume`（vca:/Master←@+0xf8、/Music←+0xfc、
//!   /Music-DawnJingle←+0xfc、/SFX←+0x100、/Amb←+0x104）→ 相机 3×3 变换算听者 → `AudioCore::update`。
//! - `boot_audio_update_system`（0x1409c9320，行 37–54）：同 VCA 段；听者为固定默认位姿。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::resources::audio_resources::{AudioCore, ListenerAttributes};
use crate::resources::audio_settings::AudioSettings;
use crate::typed_gap::Gap;

/// 相机变换资源（U：本束不交付相机域；`[+0xc0..+0xf0]` 3×3+偏移，O 反汇编证据）。
#[derive(Resource, Clone, Debug, Default)]
pub struct CameraTransformResource(pub Gap);

/// `audio_update_system`（0x1409c8dd0，O）。
pub fn audio_update_system(
    core: Res<AudioCore>,
    _camera_transform: Res<CameraTransformResource>,
    settings: Res<AudioSettings>,
) {
    if let Some(inner) = core.inner() {
        inner.set_vca_volume("vca:/Master", settings.master_volume);
        inner.set_vca_volume("vca:/Music", settings.music_volume);
        inner.set_vca_volume("vca:/Music-DawnJingle", settings.music_volume);
        inner.set_vca_volume("vca:/SFX", settings.sfx_volume);
        inner.set_vca_volume("vca:/Amb", settings.ambience_volume);
    }
    // 从相机资源 3×3+偏移变换计算 3D 听者位姿（O；此处 typed gap）。
    let listener = ListenerAttributes {
        position: glam::Vec3::ZERO,
        forward: glam::Vec3::NEG_Z,
        up: glam::Vec3::Y,
    };
    if let Some(inner) = core.inner() {
        inner.update(listener);
    }
}

/// `boot_audio_update_system`（0x1409c9320，O）：启动期同 VCA 段 + 默认听者。
pub fn boot_audio_update_system(core: Res<AudioCore>, settings: Res<AudioSettings>) {
    if let Some(inner) = core.inner() {
        inner.set_vca_volume("vca:/Master", settings.master_volume);
        inner.set_vca_volume("vca:/Music", settings.music_volume);
        inner.set_vca_volume("vca:/Music-DawnJingle", settings.music_volume);
        inner.set_vca_volume("vca:/SFX", settings.sfx_volume);
        inner.set_vca_volume("vca:/Amb", settings.ambience_volume);
    }
    let listener = ListenerAttributes {
        position: glam::Vec3::ZERO,
        forward: glam::Vec3::NEG_Z,
        up: glam::Vec3::Y,
    };
    if let Some(inner) = core.inner() {
        inner.update(listener);
    }
}
