//! systems/wall/snapping/snapping_audio.rs — 墙体吸附音频。
//!
//! 反汇编证据（O）：`snapping_audio`（0x140a6a480）`WallSnapToSegment` 瞬发（imports
//! alloc/bevy_ecs/core）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::Res;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `SnappingAudioState`（O 名；U 布局）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SnappingAudioState(pub Gap);

/// `snapping_audio`（0x140a6a480，O）。
pub fn snapping_audio(_state: Res<SnappingAudioState>) {
    // WallSnapToSegment 瞬发（libfmod 边界）。
}
