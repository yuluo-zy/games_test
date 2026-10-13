//! `systems/main_camera.rs` — the `MainCamera` (0x4e0 B) resource driver:
//! per-mode camera control + snapshot + reset, and the `MainCameraPlugin`.
//!
//! PDB: `camera_snapshot @0x140911e40`, `MainCameraPlugin::build @0x1409125e0`,
//! `reset_camera @0x140912a40`, `main_camera_update @0x140912c20`,
//! `paused_main_camera_update @0x1409145a0`, ron serde glue (impl$0 serialize/
//! deserialize), `next/previous`.
//! Named entities: `CameraSnapshot`, `CameraSnapshotInterpolationState`,
//! `CameraRotationPivot`, `CacheFreeCamRig`, `MainCameraMoved`,
//! `MainCameraPlugin`.
//!
//! O: `reset_camera` rebuilds the camera from `MainCamera::new` (memcpy 0x4e0)
//! and pushes a `WriteSettingsFileCmd`; `build` registers `main_camera_update`
//! (run_if NewSessionStartedCmd), `paused_main_camera_update` (run_if
//! not(GameTime+AppLoadingState)), `reset_camera`, inserts `CameraSnapshot`
//! and registers the `MainCameraMoved` event. Mode byte @+0x4d0 selects the
//! embedded body; `camera_snapshot` dispatches per mode with the interpolation
//! fov factor @+0xec.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::app_loading_state::AppLoadingState;
use crate::input::action_set::ActionSetStates;
use crate::resources::time::GameTime;
use crate::reused::bevy_app::App;
use crate::reused::bevy_state::State;
use crate::systems::camera_rig::*;
use crate::Gap;

/// `camera_snapshot` @0x140911e40 (O): per-mode snapshot update.
pub fn camera_snapshot(
    mut _camera: ResMut<MainCamera>,
    mut _snapshot: ResMut<CameraSnapshot>,
    mut _moved: EventWriter<MainCameraMoved>,
) {
    todo!("recover camera_snapshot per-mode dispatch (mode @+0x18, fov factor @+0xec)")
}

/// `main_camera_update` @0x140912c20 (O): read ActionSetStates / GameTime /
/// WindowSize / DebugGui, apply per-mode body control, write CameraMatrices
/// (0x100 B) + mouse-depth byte @+0x344; push WriteSettingsFileCmd when the
/// window is < 3 px.
pub fn main_camera_update(
    mut _camera: ResMut<MainCamera>,
    mut _snapshot: ResMut<CameraSnapshot>,
    mut _moved: EventWriter<MainCameraMoved>,
) {
    todo!("recover main_camera_update per-mode control + CameraMatrices write")
}

/// `paused_main_camera_update` @0x1409145a0 (O): when paused (game time
/// stopped), dispatch mode byte @+0x4d0 to the embedded body, compute
/// `CameraProjectionParams::calc_matrices` and copy the 0x100 B matrix block.
pub fn paused_main_camera_update(
    _states: Res<ActionSetStates>,
    _time: Res<GameTime>,
    mut _camera: ResMut<MainCamera>,
    mut _snapshot: ResMut<CameraSnapshot>,
    mut _debug_gui: ResMut<Gap>,
) {
    todo!("recover paused_main_camera_update per-mode body update + calc_matrices")
}

/// `reset_camera` @0x140912a40 (O): rebuild MainCamera + clear params cache +
/// push WriteSettingsFileCmd.
pub fn reset_camera(
    mut _camera: ResMut<MainCamera>,
    _window_size: Res<Gap>,
    mut _cmd: EventWriter<Gap>,
) {
    todo!("recover reset_camera (MainCamera::new + params clear + WriteSettingsFileCmd)")
}

/// `MainCameraPlugin` (O named entity). `build` @0x1409125e0.
pub struct MainCameraPlugin;

impl MainCameraPlugin {
    pub fn build(&self, _app: &mut App) {
        todo!("recover MainCameraPlugin::build (systems + CameraSnapshot + MainCameraMoved)")
    }
}

/// `CameraMode` bookmark helpers `next`/`previous` (O `impl$0::next`
/// @0x1407858016 / `previous` @0x1408230976; direction semantics U — typed
/// gap, no fabricated order).
pub fn next(_mode: &mut CameraMode) {
    todo!("recover camera-mode iterator next (direction U)")
}
