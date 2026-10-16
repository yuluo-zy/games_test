//! hint_camera_progress（B 主基线；2 函数）。
//!
//! 证据（skeleton PDB）：hint_camera_progress_update 0x14227bd30（L0）、
//! hint_camera_progress_reset 0x14227c070（L147）。语义：相机操作进度条
//! （用户已完成的相机动作比例；达成 → 完成提示）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;

/// 相机进度（资源；progress 0..1）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CameraProgress {
    pub progress: f32,
    pub done: bool,
}

/// hint_camera_progress_update（0x14227bd30）：按已发生的相机动作推进进度。
pub fn hint_camera_progress_update(
    time: Res<GameTime>,
    mut state: ResMut<CameraProgress>,
    mut pan_events: EventReader<crate::hint_camera_progress::camera_moved::CameraMoved>,
) {
    let _ = &time;
    let count = pan_events.read().count() as f32;
    if count > 0.0 {
        state.progress = (state.progress + count * 0.1).min(1.0);
    }
    if state.progress >= 1.0 {
        state.done = true;
    }
}

/// hint_camera_progress_reset（0x14227c070）：重置（新会话/新 glade）。
pub fn hint_camera_progress_reset(mut state: ResMut<CameraProgress>) {
    state.progress = 0.0;
    state.done = false;
}

pub mod camera_moved {
    //! 相机动作事件（宿主相机系统写；pan/rotate/zoom）。
    use bevy_ecs::prelude::*;

    #[derive(Event, Clone, Copy, Debug, PartialEq)]
    pub struct CameraMoved {
        pub kind: CameraMoveKind,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CameraMoveKind {
        Pan,
        Rotate,
        Zoom,
    }
}
