//! `systems/ui_camera_mode_gizmos.rs` — camera-mode gizmo signifiers drawn for
//! small windows.
//!
//! PDB: `ui_camera_mode_gizmos @0x1409527a0` (948 B) + `closure$0 @0x140952b60`.
//!
//! O: when the active mode is a playback/timelapse state (`mode == 2` or
//! `mode == 10` with a byte flag) and the camera-mode byte @+0x4d0 is NOT the
//! free camera (`(byte & 6) != 2`): for a window < 3 px, register the widget
//! via `UiState::add`, and when the camera mode is one of the tool modes push
//! a `UiAppendSignifier {tag,size,pos,fov}` through `AppendOnlyVec`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::reused::append_only_vec::AppendOnlyVec;
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::signifiers::UiAppendSignifier;
use crate::Gap;

/// `ui_camera_mode_gizmos` @0x1409527a0 (O).
pub fn ui_camera_mode_gizmos(
    mut _ui_state: ResMut<UiState>,
    _window: Res<Gap>,
    _camera: Res<Gap>,
    _mode: Res<Gap>,
    _signifiers: Res<AppendOnlyVec<UiAppendSignifier>>,
) {
    todo!("recover ui_camera_mode_gizmos (mode gate + UiState::add + AppendOnlyVec signifier)")
}
