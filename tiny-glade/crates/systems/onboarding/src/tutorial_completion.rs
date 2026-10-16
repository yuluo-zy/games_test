//! tutorial_completion（0x14227c230，源码 L15）。
//!
//! 证据（skeleton PDB）：raw_vec —— 教程完成判定（相机/建造/地形进度齐 → 完成，
//! 写会话标记）。

use bevy_ecs::prelude::*;

use crate::hint_camera_progress::CameraProgress;

/// 教程完成状态（资源）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TutorialCompletionState {
    pub completed: bool,
}

/// 教程完成进度（各教学里程碑）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TutorialProgress {
    pub camera_done: bool,
    pub building_done: bool,
    pub terrain_done: bool,
}

/// tutorial_completion（0x14227c230）：里程碑齐 → 完成。
pub fn tutorial_completion(
    mut state: ResMut<TutorialCompletionState>,
    mut progress: ResMut<TutorialProgress>,
    camera: Res<CameraProgress>,
) {
    if camera.done {
        progress.camera_done = true;
    }
    if progress.camera_done && progress.building_done && progress.terrain_done {
        state.completed = true;
    }
}
