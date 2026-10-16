//! reset_hints（0x141c1c8d0，源码 L0）。
//!
//! 证据（skeleton PDB）：raw_vec —— 复位全部提示状态（新 glade/新会话）。

use bevy_ecs::prelude::*;

use crate::exponential_back_off::ExponentialBackOff;
use crate::hint_display_system::HintDeletionCooldowns;

/// 提示复位状态（资源）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintResetState {
    pub pending: bool,
}

/// reset_hints（0x141c1c8d0）：复位退避/冷却/进度。
pub fn reset_hints(
    mut state: ResMut<HintResetState>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut cooldowns: ResMut<HintDeletionCooldowns>,
    mut camera_progress: ResMut<crate::hint_camera_progress::CameraProgress>,
) {
    if !state.pending {
        return;
    }
    for i in 0..16 {
        back_off.reset(i);
        cooldowns.remaining[i] = 0.0;
    }
    camera_progress.progress = 0.0;
    camera_progress.done = false;
    state.pending = false;
}
