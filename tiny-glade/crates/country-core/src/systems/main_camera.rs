//! `country_core::systems::main_camera`（B 原模块路径）。
//!
//! 本文件只落 unlock-tinyglade-deps 需要的枚举；`MainCamera` 的已恢复部分在
//! `systems::camera_rig::main_camera`（历史分组），两者路径不同、类型不冲突。
//!
//! `CameraRotationPivot` fresh 证据（Ghidra shared 只读）：
//!  * serialize 0x14003AC90：6 分支跳表 0x14292C04C → 各分支
//!    `write_identifier` 指向 .rdata 0x14292BB7C 的字符串池
//!    "CameraRotationPivotCursorDepthFilteredCursorDepthCenterDepth
//!     FilteredCenterDepthCursorTerrainCenterTerrain"：
//!    0 CursorDepth / 1 FilteredCursorDepth / 2 CenterDepth /
//!    3 FilteredCenterDepth / 4 CursorTerrain / 5 CenterTerrain。
//!  * FieldVisitor 0x14003B6A0：长度 19 的两个名字
//!    "FilteredCursorDepth"/"FilteredCenterDepth" 分支与上述顺序互证。
//!  * Sequence next/previous 0x1408B99E0/0x140914840（同构家族）。
//!  * `GameSettings.camera_rotation_pivot`/`camera_zoom_pivot` 均为
//!    `Option<CameraRotationPivot>`（PDB 无 `CameraZoomPivot` 实体）。

use serde::{Deserialize, Serialize};

/// 原 B `PROMPT_ROLLERCOASTER`（0x143634cd9，经导入槽0x14362c028共享）。
/// 唯一业务写者是 tiny-glade 的 rollercoaster_update，摄影尾部只读取。
/// 原版为跨crate全局bool；AtomicBool保持同一权威槽，避免Rust可变静态数据竞争。
/// 初始化false不代表检测完成：写者的真实射线/车辆状态分支必须另外恢复和注册。
pub static PROMPT_ROLLERCOASTER: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// 原骑乘帧在普通相机处理之后，强制Walk保留垂直分量。
pub fn set_rollercoaster_walk_override(camera: &mut super::camera_rig::MainCamera) {
    camera.set_walk_modifier(true);
}

/// 原骑乘帧尾写完整四矩阵。镜头来自当前body，不使用固定摄影参数，
/// 也不在这里重复推进Dolly或镜头平滑。
pub fn write_rollercoaster_camera_pose(
    camera: &mut super::camera_rig::MainCamera,
    position: glam::Vec3,
    rotation: glam::Quat,
) {
    let lens = 0x100 + super::camera_rig::main_camera::FOV_OFFSETS[camera.mode_byte() as usize];
    let projection = ::utils::camera::CameraProjectionParams {
        aspect: camera.aspect(),
        near: camera.read_f32(lens),
        vertical_fov_degrees: camera.read_f32(lens + 4),
    }.calc_matrices();
    let body = ::utils::camera::CameraBodyMatrices::from_position_rotation(position, rotation);
    camera.write_matrix(0, &projection.projection);
    camera.write_matrix(0x40, &projection.inverse_projection);
    camera.write_matrix(0x80, &body.view);
    camera.write_matrix(0xc0, &body.inverse_view);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum CameraRotationPivot {
    #[default]
    CursorDepth = 0,
    FilteredCursorDepth = 1,
    CenterDepth = 2,
    FilteredCenterDepth = 3,
    CursorTerrain = 4,
    CenterTerrain = 5,
}

/// DebugGui's original +0/+8 request. The GUI remains the request owner;
/// CameraSnapshot owns only stored camera parameters and interpolation state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CameraSnapshotCommand {
    #[default]
    None,
    Insert,
    Overwrite(usize),
    Remove(usize),
    Select(usize),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum CameraSnapshotInterpolation {
    #[default]
    Disabled = 0,
    Accelerating = 1,
    Slow = 2,
    Decelerating = 3,
}

/// Native +0 Vec<36B CameraRigParameters>, +18 state, +1c elapsed,
/// +20 selected index. Rust owns the vector; no native pointers are rebuilt.
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct CameraSnapshot {
    pub snapshots: Vec<super::camera_rig::main_camera::CameraRigParameters>,
    pub interpolation: CameraSnapshotInterpolation,
    pub elapsed: f32,
    pub index: usize,
}
impl CameraSnapshot {
    /// Original advances at most one state per call, including large deltas.
    pub fn advance(&mut self, step: f32) {
        use CameraSnapshotInterpolation::*;
        match self.interpolation {
            Disabled => {}
            Accelerating => {
                self.elapsed += step;
                if !(6.0 > self.elapsed) {
                    self.interpolation = Slow;
                    self.elapsed += -6.0;
                }
            }
            Slow => {
                self.elapsed += step;
                if !(2.0 > self.elapsed) {
                    self.interpolation = Decelerating;
                    self.elapsed = 0.0;
                }
            }
            Decelerating => {
                self.elapsed += step;
                if !(4.0 > self.elapsed) {
                    self.interpolation = Disabled;
                }
            }
        }
    }
    fn insert(&mut self, camera: &super::camera_rig::MainCamera) {
        let index = self.index.wrapping_add(1).min(self.snapshots.len());
        self.snapshots.insert(index, camera.store_parameters());
        self.index = index;
    }
    fn select(&mut self, index: usize, camera: &mut super::camera_rig::MainCamera) {
        self.index = index;
        if self.snapshots.is_empty() {
            return;
        }
        let parameters = self.snapshots[index];
        self.interpolation = CameraSnapshotInterpolation::Accelerating;
        self.elapsed = 0.0;
        let parameters = super::camera_rig::CameraRigParametersCache {
            yaw: parameters.yaw,
            pitch: parameters.pitch,
            position: parameters.position,
            camera_rig_arm_length: parameters.camera_rig_arm_length,
            vertical_fov_degrees: parameters.vertical_fov_degrees,
            pitch_offset: parameters.pitch_offset,
            yaw_offset: parameters.yaw_offset,
        };
        camera.load_parameters(&parameters, false);
    }
    pub fn interpolation_delta(&self, delta: f32, speed: f32) -> Option<f32> {
        use CameraSnapshotInterpolation::*;
        let base = speed * 0.1;
        let multiplier = match self.interpolation {
            Disabled => return None,
            Accelerating => base * (self.elapsed / 6.0),
            Slow => base,
            Decelerating => {
                let mut t = self.elapsed * 0.25;
                t *= t;
                t *= t;
                // Original utils::lerp(base,1,t) @140912227.
                base + (1.0 - base) * t
            }
        };
        Some(delta * multiplier)
    }
    pub fn cancel(&mut self) {
        self.interpolation = CameraSnapshotInterpolation::Disabled;
    }
}

/// Original reset_camera@140912a40: replace the whole camera at its current
/// aspect, free snapshot storage, reset state/index and emit one unit event.
/// The old elapsed slot is left intact; it is irrelevant while disabled.
pub fn reset_camera(
    mut camera: bevy_ecs::prelude::ResMut<super::camera_rig::MainCamera>,
    mut snapshot: bevy_ecs::prelude::ResMut<CameraSnapshot>,
    mut moved: bevy_ecs::prelude::EventWriter<super::camera_rig::main_camera::MainCameraMoved>,
) {
    log::trace!("Resetting camera");
    let aspect = camera.aspect();
    *camera = super::camera_rig::MainCamera::new(aspect);
    snapshot.snapshots = Vec::new();
    snapshot.interpolation = CameraSnapshotInterpolation::Disabled;
    snapshot.index = 0;
    moved.write(super::camera_rig::main_camera::MainCameraMoved);
}

/// Original camera_snapshot@140911e40, first gameplay-active member of set9.
/// Current host supplies demo from the same World configuration; B reads its
/// process-wide immutable demo flag inside NormalCamera::update.
pub fn camera_snapshot(
    actions: bevy_ecs::prelude::Res<crate::input::action_set::MainActionSet>,
    time: bevy_ecs::prelude::Res<crate::resources::time::GameTime>,
    mut camera: bevy_ecs::prelude::ResMut<super::camera_rig::MainCamera>,
    mut snapshot: bevy_ecs::prelude::ResMut<CameraSnapshot>,
    mut debug: bevy_ecs::prelude::ResMut<crate::resources::gui::DebugGui>,
    config: bevy_ecs::prelude::Res<crate::game_config::GameConfig>,
) {
    let speed = debug.camera_snapshot_interpolation_speed;
    snapshot.advance(time.delta * speed);
    if actions.digital[37].just_pressed() {
        snapshot.insert(&camera);
    } else if actions.digital[38].just_pressed() && !snapshot.snapshots.is_empty() {
        let index = snapshot.index.wrapping_add(1) % snapshot.snapshots.len();
        snapshot.select(index, &mut camera);
    } else {
        // Keyboard actions leave a pending GUI command for the next frame.
        match std::mem::take(&mut debug.camera_snapshot_command) {
            CameraSnapshotCommand::None => {}
            CameraSnapshotCommand::Insert => snapshot.insert(&camera),
            CameraSnapshotCommand::Overwrite(index) => {
                snapshot.snapshots[index] = camera.store_parameters()
            }
            CameraSnapshotCommand::Remove(index) => {
                snapshot.snapshots.remove(index);
                if snapshot.index >= index {
                    snapshot.index = snapshot.index.saturating_sub(1);
                }
            }
            CameraSnapshotCommand::Select(index) => snapshot.select(index, &mut camera),
        }
    }
    if let Some(delta) = snapshot.interpolation_delta(time.delta, speed) {
        camera.update_camera_matrices(delta, config.dev.demo);
    }
}

/// Original paused_main_camera_update@1409145a0 updates the selected body with
/// dt0 and rebuilds projection/view at the actual current window aspect.
pub fn paused_main_camera_update(
    mut camera: bevy_ecs::prelude::ResMut<super::camera_rig::MainCamera>,
    window: bevy_ecs::prelude::Res<crate::resources::window::WindowSize>,
    config: bevy_ecs::prelude::Res<crate::game_config::GameConfig>,
) {
    camera.set_aspect_from_window(window.width, window.height);
    camera.update_paused_camera_matrices(config.dev.demo);
}

#[cfg(test)]
mod original_snapshot_tests {
    use super::*;
    #[test]
    fn native_camera_interpolation_advances_one_stage_per_update() {
        let mut snapshot = CameraSnapshot {
            interpolation: CameraSnapshotInterpolation::Accelerating,
            ..Default::default()
        };
        snapshot.advance(10.0);
        assert_eq!(snapshot.interpolation, CameraSnapshotInterpolation::Slow);
        assert_eq!(snapshot.elapsed, 4.0);
        snapshot.advance(0.0);
        assert_eq!(
            snapshot.interpolation,
            CameraSnapshotInterpolation::Decelerating
        );
        assert_eq!(snapshot.elapsed, 0.0);
        snapshot.advance(4.0);
        assert_eq!(
            snapshot.interpolation,
            CameraSnapshotInterpolation::Disabled
        );
        assert_eq!(snapshot.elapsed, 4.0);
    }
    #[test]
    fn native_camera_snapshot_remove_preserves_selected_predecessor() {
        let camera = super::super::camera_rig::MainCamera::new(1.5);
        let mut snapshot = CameraSnapshot::default();
        snapshot.insert(&camera);
        snapshot.insert(&camera);
        snapshot.insert(&camera);
        assert_eq!(snapshot.index, 2);
        snapshot.snapshots.remove(1);
        if snapshot.index >= 1 {
            snapshot.index = snapshot.index.saturating_sub(1);
        }
        assert_eq!(snapshot.index, 1);
    }
}

pub use super::camera_rig::main_camera::MainCameraMoved;

#[cfg(feature = "native-physics")]
#[derive(bevy_ecs::system::SystemParam)]
pub struct CameraControlContext<'w, 's> {
    cursor: bevy_ecs::prelude::Res<'w, crate::resources::mouse_terrain_raycast::CursorPositionSS>,
    depth: bevy_ecs::prelude::Res<'w, crate::resources::read_mouse_depth_value::MouseDepthValue>,
    heights: bevy_ecs::prelude::Res<'w, crate::resources::terrain_height::TerrainHeightsData>,
    collisions: bevy_ecs::prelude::Res<'w, super::collision::world::RaycastWorld>,
    path: bevy_ecs::prelude::Res<'w, crate::resources::world_raster_defs::PathRaster>,
    time: bevy_ecs::prelude::Res<'w, crate::resources::time::GameTime>,
    window: bevy_ecs::prelude::Res<'w, crate::resources::window::WindowSize>,
    config: bevy_ecs::prelude::Res<'w, crate::game_config::GameConfig>,
    session: bevy_ecs::prelude::Res<'w, crate::resources::session::ActiveSession>,
    debug: bevy_ecs::prelude::Res<'w, crate::resources::gui::DebugGui>,
    butterflies: bevy_ecs::prelude::Query<
        'w,
        's,
        &'static crate::utils::components::transform::Transform,
        bevy_ecs::prelude::With<crate::resources::critters::Butterfly>,
    >,
    vermeer: bevy_ecs::prelude::Res<'w, super::render_loop::vermeer::VermeerState>,
}

#[cfg(feature = "native-physics")]
fn depth_anchor(
    pivot: CameraRotationPivot,
    depth: &crate::resources::read_mouse_depth_value::MouseDepthValue,
) -> glam::Vec3 {
    use CameraRotationPivot::*;
    match pivot {
        CursorDepth | CursorTerrain => depth.mouse_position,
        FilteredCursorDepth => depth.filtered_mouse_position,
        CenterDepth | CenterTerrain => depth.center_position,
        FilteredCenterDepth => depth.filtered_center_position,
    }
}

#[cfg(feature = "native-physics")]
fn rotation_anchor(
    pivot: CameraRotationPivot,
    camera: &super::camera_rig::MainCamera,
    context: &CameraControlContext,
) -> (glam::Vec3, glam::Vec3) {
    use CameraRotationPivot::*;
    use glam::Vec3;
    let cursor = context.cursor.0;
    let window = context.window.as_vec2();
    let inverse = camera.read_matrix(0x40);
    let world_ray =
        ::utils::geometry::Ray::from_mouse_ss(cursor, window, inverse, camera.read_matrix(0xc0));
    // This exact B expression differs from the ordinary screen-space ray's Y.
    let x = (cursor.x / window.x) * 2.0 - 1.0;
    let y = ((1.0 - cursor.y) / window.y) * 2.0 + 1.0;
    let xy = inverse.y_axis * y + inverse.x_axis * x;
    let near = inverse.z_axis + xy + inverse.w_axis;
    let far = inverse.z_axis * 0.01 + xy + inverse.w_axis;
    let near = (near / near.w).truncate();
    let far = (far / far.w).truncate();
    let direction = if matches!(
        pivot,
        FilteredCursorDepth | CenterDepth | FilteredCenterDepth | CenterTerrain
    ) {
        Vec3::new(-0.0, -0.0, -1.0)
    } else {
        (far - near).normalize()
    };
    let anchor = if matches!(pivot, CursorTerrain | CenterTerrain) {
        let ray = if pivot == CursorTerrain {
            world_ray
        } else {
            let direction = camera.eye_forward();
            ::utils::geometry::Ray {
                origin: camera.eye_position() + direction * 0.1,
                direction,
            }
        };
        context
            .heights
            .intersect_ray(ray, f32::MAX)
            .map_or(Vec3::ZERO, |hit| hit.position)
    } else {
        depth_anchor(pivot, &context.depth)
    };
    (anchor, direction)
}

/// Complete original main_camera_update@140912c20 (6525B), current ECS
/// parameter projection. The borrowed native-physics owner replaces the old
/// unproduced Raycasting Resource; all body state remains in MainCamera.
#[cfg(feature = "native-physics")]
pub fn main_camera_update(
    actions: bevy_ecs::prelude::Res<crate::input::action_set::MainActionSet>,
    modifiers: bevy_ecs::prelude::Res<super::backend::input::mod_key::Modifiers>,
    mut cursor_moved: bevy_ecs::prelude::EventReader<
        crate::resources::window::CursorMovedScreenSpace,
    >,
    mut capture: bevy_ecs::prelude::EventWriter<crate::resources::mouse_capture::CursorCaptureCmd>,
    mut camera: bevy_ecs::prelude::ResMut<super::camera_rig::MainCamera>,
    context: CameraControlContext,
    mut snapshots: bevy_ecs::prelude::ResMut<CameraSnapshot>,
    mut moved: bevy_ecs::prelude::EventWriter<MainCameraMoved>,
    mut hints: bevy_ecs::prelude::EventWriter<
        crate::resources::onboarding::NotifyHintSystemCameraMoved,
    >,
) {
    use super::camera_rig::main_camera::CameraMode;
    use crate::input::action_set::DigitalActionPhase;
    use crate::resources::{
        mouse_capture::CursorCaptureCmd, onboarding::NotifyHintSystemCameraMoved,
    };
    use glam::Vec3;
    camera.set_aspect_from_window(context.window.width, context.window.height);
    if snapshots.interpolation != CameraSnapshotInterpolation::Disabled {
        // Original only pan digital3 and analog zoom4 interrupt interpolation.
        if !actions.digital[3].pressed() && actions.analog[4].length_squared() <= 0.0 {
            return;
        }
        snapshots.cancel();
    }
    if actions.digital[2].pressed() || actions.digital[3].pressed() {
        moved.write(MainCameraMoved);
    }
    camera.set_walk_modifier(modifiers.flags as u8 != 0);
    let dt = if context.vermeer.should_freeze_camera() {
        0.0
    } else {
        context.time.delta
    };
    let settings = &context.config.game;
    let zoom_pivot = if actions.digital[2].pressed() {
        settings.camera_rotation_pivot
    } else {
        settings.camera_zoom_pivot
    }
    .unwrap_or_default();
    let zoom_anchor = depth_anchor(zoom_pivot, &context.depth);
    let keyboard_rotation = actions.analog[5].x;
    let keyboard_rotation_active = keyboard_rotation > 0.0 || keyboard_rotation < 0.0;
    if keyboard_rotation_active
        || actions.digital[2].phase == DigitalActionPhase::JustPressed
        || (actions.digital[2].phase == DigitalActionPhase::Repeating
            && actions.digital[2].timer_seconds <= 0.0)
    {
        let pivot = if keyboard_rotation_active {
            CameraRotationPivot::CenterTerrain
        } else {
            settings.camera_rotation_pivot.unwrap_or_default()
        };
        let (anchor, direction) = rotation_anchor(pivot, &camera, &context);
        let radius = (anchor.z * anchor.z + anchor.x * anchor.x).sqrt();
        if radius < 130.0 {
            let eye = camera.eye_position();
            camera.begin_selected_rotation(anchor, direction, eye);
            if !keyboard_rotation_active {
                capture.write(CursorCaptureCmd(false));
            }
        }
    } else if actions.digital[2].pressed() {
        // Same reader is used later for pan; read().last drains the full queue.
        if let Some(delta) = cursor_moved.read().last().copied() {
            let mut yaw = ((delta.0.x * -1.2) * 0.25) * settings.camera_rotation_sensitivity;
            let mut pitch = (delta.0.y * -0.25) * settings.camera_rotation_sensitivity;
            if yaw.is_finite() && pitch.is_finite() {
                if settings.camera_invert_x {
                    yaw = -yaw;
                }
                if settings.camera_invert_y {
                    pitch = -pitch;
                }
                camera.rotate_selected(yaw, pitch);
            }
        }
    }
    if keyboard_rotation_active && camera.mode_byte() == 0 {
        let yaw = keyboard_rotation * 0.25 * settings.camera_rotation_sensitivity;
        if yaw.is_finite() {
            camera.rotate_selected(yaw, 0.0);
        }
        hints.write(NotifyHintSystemCameraMoved { tag: 2, value: 0 });
    }
    if actions.digital[2].just_released() {
        capture.write(CursorCaptureCmd(true));
        camera.finish_selected_rotation();
    }
    let mode = camera.mode_byte();
    let mut movement = Vec3::ZERO;
    if actions.digital[3].pressed() {
        if let Some(delta) = cursor_moved.read().last().copied() {
            movement =
                Vec3::new(delta.0.x, 0.0, delta.0.y) * -0.04 * settings.camera_movement_speed;
            if matches!(mode, 1 | 4) {
                movement *= 0.5;
            }
        }
    }
    let factor = match mode {
        1 => 0.25,
        2 => 2.0,
        4 => 0.2,
        _ => 1.0,
    };
    let pan = actions.analog[2];
    movement.x += pan.x * factor * settings.camera_movement_speed;
    movement.y += actions.analog[3].x * factor * settings.camera_movement_speed;
    movement.z += pan.y * factor * settings.camera_movement_speed;
    if movement != Vec3::ZERO && movement.is_finite() {
        camera.translate_selected(
            movement,
            dt,
            &context.heights,
            &context.collisions,
            context.config.dev.demo,
        );
        hints.write(NotifyHintSystemCameraMoved { tag: 0, value: 0 });
    }
    let zoom = actions.analog[4].x;
    if zoom > 0.0 || zoom < 0.0 {
        match camera.mode_byte() {
            0 | 1 | 4 => {
                camera.zoom_selected(zoom * -0.5, zoom_anchor);
                hints.write(NotifyHintSystemCameraMoved {
                    tag: 1,
                    value: zoom.abs().to_bits(),
                });
            }
            5 => camera.advance_butterfly(zoom),
            _ => {}
        }
    }
    let followed = if context.debug.camera_mode == CameraMode::BUTTERFLY {
        if let Some(transform) = context
            .butterflies
            .iter()
            .nth(camera.butterfly_selected_index())
        {
            camera.follow_butterfly(transform);
            true
        } else {
            false
        }
    } else {
        false
    };
    if !followed {
        camera.set_mode_byte(context.debug.camera_mode);
    }
    let before = camera.eye_position();
    camera.update_camera_matrices(dt, context.config.dev.demo);
    // Original constraint updates body state after writing this frame matrices.
    camera.constrain_selected(
        &context.heights,
        &context.collisions,
        &context.path,
        context.session.is_winter(),
    );
    if actions.digital[2].pressed() && (before - camera.eye_position()).length() > dt * 0.2 {
        hints.write(NotifyHintSystemCameraMoved { tag: 2, value: 0 });
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod original_camera_ecs_tests {
    use super::*;
    use crate::{
        input::action_set::MainActionSet,
        resources::{
            gui::DebugGui,
            mouse_capture::{CursorCaptureCmd, CursorCapturedPosition},
            mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS, camera_mouse_ray},
            read_mouse_depth_value::MouseDepthValue,
            session::{ActiveSession, SessionId, SessionMeta},
            terrain_height::TerrainHeightsData,
            window::{AudioCursorMoved, CursorMovedScreenSpace, WindowSize},
            world_raster::DirtyTilePublisher,
            world_raster_defs::PathRaster,
        },
        systems::{
            backend::input::mod_key::Modifiers, camera_rig::MainCamera,
            collision::world::RaycastWorld, render_loop::vermeer::VermeerState,
        },
    };
    use bevy_ecs::prelude::*;
    #[test]
    fn actual_normal_camera_ecs_input_produces_the_updated_world_ray() {
        // Original camera/metadata fixture, and the real constructors of each
        // CPU owner. No GPU mapping, fake collider or missing optional resource.
        let params: crate::systems::camera_rig::CameraRigParametersCache = serde_json::from_str(
            include_str!("../../tests/fixtures/original-intro-camera-b.json"),
        )
        .unwrap();
        let mut camera = MainCamera::new(4.0 / 3.0);
        camera.load_parameters(&params, false);
        camera.finish_loaded_normal_interpolation(false).unwrap();
        let before = camera.eye_position();
        let directory = tempfile::tempdir().unwrap();
        let meta = directory.path().join("meta.json");
        std::fs::write(&meta, br#"{"state":{"glade":"Autumn"},"version":1}"#).unwrap();
        let session = ActiveSession::new(
            SessionId::new(),
            SessionMeta::try_load_from_path(meta).unwrap(),
        );
        let mut app = bevy_app::App::new();
        app.insert_resource(camera);
        app.insert_resource(session);
        app.insert_resource(WindowSize::new(1024, 768));
        app.insert_resource(CursorPositionSS([512.0, 384.0].into()));
        app.insert_resource(CameraCursorRay::new());
        app.insert_resource(MouseDepthValue::new());
        app.insert_resource(TerrainHeightsData::<
            crate::resources::terrain_height::TerrainHeightSsbo,
        >::new());
        app.insert_resource(RaycastWorld::new());
        app.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        app.insert_resource(crate::game_config::GameConfig::default());
        app.insert_resource(DebugGui::default());
        app.insert_resource(VermeerState::default());
        app.insert_resource(CameraSnapshot::default());
        app.insert_resource(Modifiers::default());
        app.insert_resource(CursorCapturedPosition::default());
        let mut time = crate::resources::time::GameTime::default();
        time.delta = 1.0 / 60.0;
        app.insert_resource(time);
        let mut actions = MainActionSet::default();
        actions.analog[2] = glam::Vec2::new(20.0, 10.0);
        app.insert_resource(actions);
        app.add_event::<CursorMovedScreenSpace>();
        app.add_event::<CursorCaptureCmd>();
        app.add_event::<MainCameraMoved>();
        app.add_event::<crate::resources::onboarding::NotifyHintSystemCameraMoved>();
        app.add_event::<AudioCursorMoved>();
        app.add_systems(
            bevy_app::Update,
            (main_camera_update, camera_mouse_ray).chain_ignore_deferred(),
        );
        app.update();
        // The original default Vermeer owner is Pending/freeze_camera=true:
        // input queues translation while dt=0 preserves this frame's pose.
        assert_eq!(app.world().resource::<MainCamera>().eye_position(), before);
        let queued = app.world().resource::<MainCamera>().read_vec3(0x144);
        assert!(queued.is_finite() && queued.length_squared() > 0.0);
        // End the real reveal lifecycle; the queued input now advances through
        // the same controller/body and produces a changed canonical world ray.
        app.world_mut().resource_mut::<VermeerState>().deactivate();
        app.update();
        let camera = app.world().resource::<MainCamera>();
        assert_ne!(camera.eye_position(), before);
        let ray = app.world().resource::<CameraCursorRay>();
        assert!(ray.origin.is_finite() && ray.dir.is_finite());
        assert!((ray.dir.length() - 1.0).abs() < 1.0e-5);
        let expected = ::utils::geometry::Ray::from_mouse_ss(
            [512.0, 384.0].into(),
            [1024.0, 768.0].into(),
            camera.read_matrix(0x40),
            camera.read_matrix(0xc0),
        );
        assert_eq!(ray.origin, expected.origin);
        assert_eq!(ray.dir, expected.direction);
        assert_eq!(app.world().resource::<Events<AudioCursorMoved>>().len(), 2);
        let hints = app
            .world()
            .resource::<Events<crate::resources::onboarding::NotifyHintSystemCameraMoved>>();
        let mut reader = hints.get_cursor();
        assert!(reader.read(hints).any(|hint| hint.tag == 0));
    }
}
