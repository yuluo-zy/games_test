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

// ---------------------------------------------------------------------------
// 每帧相机驱动（B `main_camera_update` 0x140912c20、`paused_main_camera_update`
// 0x1409145a0、`camera_snapshot` 0x140911e40、`reset_camera` 0x140912a40）。
//
// 证据：expansion-b2 / parallel-camera decomp。控制流 O；逐模式 body 更新与
// 快照应用函数在原件中是跳表目标（0x142aabd04 / 0x142aabd30 / 0x142aabc88），
// 这些目标体仍有损坏，因此这里只落已核对的模式分发、rig 迭代、矩阵写回与
// 输入/宽高比/光标门；未恢复分支显式 U。
// ---------------------------------------------------------------------------

use crate::resources::{
    events::WriteSettingsFileCmd,
    game_clock::GameClock,
    gui::DebugGui,
    mouse_terrain_raycast::CursorPositionSS,
    window::WindowSize,
};
use crate::systems::camera_rig::main_camera::{
    CameraMode, MainCamera, NORMAL_PITCH_OFFSET, NORMAL_POSITION_OFFSET, NORMAL_YAW_OFFSET,
    ORBIT_ANGLE_SCALE,
};
use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};

/// 模式 body 在 `MainCamera` 内的基址（O：`MainCamera::new` / `paused` 跳表）。
pub const FREE_BODY_OFFSET: usize = 0x270;
pub const WALK_BODY_OFFSET: usize = 0x2f0;
pub const BUTTERFLY_BODY_OFFSET: usize = 0x448;

/// `camera_snapshot` 跳表（0x142aabc88）实读条目数：27。
pub const SNAPSHOT_MODE_SLOTS: u32 = 27;

/// 原件更新系统读取的输入面。字段在原件里来自输入/设置资源；当前适配层
/// 用单一资源承载已核对的语义，逐字段偏移映射登记在 delivery。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct MainCameraUpdateInput {
    /// 新会话标志（原 +0x18）：光标未离开 (0,0) 时跳过一次更新。
    pub new_session_pending: bool,
    /// 鼠标深度字节（原 → `MainCamera +0x344`）。
    pub mouse_depth_byte: u8,
    /// 自由相机开关（原 +0x168）。
    pub free_camera_enabled: bool,
    /// 非自由相机时用于分发的动作模式（原 +0x10）。
    pub action_mode: u8,
}

/// 相机快照插值状态（原 `CameraSnapshot`：mode u32@+0x18、value f32@+0x1c）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CameraSnapshot {
    pub mode: u32,
    pub value: f32,
    pub active: bool,
}

/// O：跳表实读 27 项，越界在原件会读相邻数据；本实现拒绝。
pub fn snapshot_mode_is_valid(mode: u32) -> bool {
    mode < SNAPSHOT_MODE_SLOTS
}

/// 模式 rig 迭代并把分叉 `CameraTransform` 写回 body。
fn update_rig_body(camera: &mut MainCamera, mode: CameraMode, dt: f32) -> Option<(Vec3, Quat)> {
    let rigs = camera.startup_rigs.clone()?;
    let mut guard = rigs.lock().ok()?;
    let (base, transform) = match mode.0 {
        2 => (FREE_BODY_OFFSET, guard.free.update(dt)),
        3 => (WALK_BODY_OFFSET, guard.walk.update(dt)),
        5 => (BUTTERFLY_BODY_OFFSET, guard.butterfly.update(dt)),
        _ => return None,
    };
    let position = Vec3::new(
        transform.position.x,
        transform.position.y,
        transform.position.z,
    );
    let rotation = Quat::from_xyzw(
        transform.rotation.v.x,
        transform.rotation.v.y,
        transform.rotation.v.z,
        transform.rotation.s,
    );
    drop(guard);
    camera.write_rig_transform(base, position, rotation);
    camera.update_frame_matrices(position, rotation);
    Some((position, rotation))
}

/// 已核对的模式分发：Free/Walk/Butterfly 走 dolly rig；Normal/TiltShift/TopDown
/// 及复用 Normal 槽的 6..12 用 Normal body 投影。
///
/// U：Normal body 的 `update`（0x140949fe0 的 smooth/radial/loose 推进）与
/// 6..12 各槽的具体 body 仍未恢复；这里只重建矩阵，不伪造位移。
pub fn update_camera_for_mode(camera: &mut MainCamera, mode: CameraMode, dt: f32) {
    camera.set_mode_byte(mode);
    if update_rig_body(camera, mode, dt).is_none() {
        camera.update_normal_frame_matrices();
    }
}

/// 输入生产者：把 `NewSessionStartedCmd` 转成新会话光标门标志。
///
/// U：鼠标深度字节与自由相机开关的真实来源资源未恢复（原件 +0x344 写入源 /
/// +0x168）；这两个字段保持默认值，直到对应输入域给出资源。
pub fn feed_main_camera_update_input(
    mut input: ResMut<MainCameraUpdateInput>,
    mut new_sessions: EventReader<crate::resources::session::NewSessionStartedCmd>,
) {
    if new_sessions.read().next().is_some() {
        input.new_session_pending = true;
    }
}

/// O 0x140912c20。顺序：aspect → 新会话光标门 → 窗口过小时推
/// `WriteSettingsFileCmd` → 鼠标深度字节 → DebugGui 模式（6→0）/自由相机分发。
pub fn main_camera_update(
    camera: Option<ResMut<MainCamera>>,
    window: Option<Res<WindowSize>>,
    cursor: Option<Res<CursorPositionSS>>,
    gui: Option<Res<DebugGui>>,
    clock: Option<Res<GameClock>>,
    mut input: Option<ResMut<MainCameraUpdateInput>>,
    mut settings_cmd: Option<ResMut<Events<WriteSettingsFileCmd>>>,
) {
    let Some(mut camera) = camera else {
        return;
    };
    let (width, height) = window
        .as_deref()
        .map(|w| (w.width, w.height))
        .unwrap_or((0, 0));
    if height != 0 {
        camera.set_aspect_from_window(width, height);
    }
    let cursor_at_origin = cursor
        .as_deref()
        .map(|c| c.0.length_squared() <= 0.0)
        .unwrap_or(true);
    if let Some(state) = input.as_deref_mut() {
        if state.new_session_pending {
            if width > 2 && cursor_at_origin {
                return;
            }
            state.new_session_pending = false;
        }
        camera.set_mouse_depth_byte(state.mouse_depth_byte);
    }
    if width < 3 || height < 3 {
        if let Some(events) = settings_cmd.as_mut() {
            events.send(WriteSettingsFileCmd);
        }
    }
    let free_camera = input
        .as_deref()
        .map(|state| state.free_camera_enabled)
        .unwrap_or(false);
    let action_mode = input.as_deref().map(|state| state.action_mode).unwrap_or(0);
    let mode_byte = gui
        .as_deref()
        .map(|gui| gui.camera_mode.0)
        .unwrap_or(action_mode);
    // 原件：DebugGui 模式滑块 6 → 0。
    let mode_byte = if mode_byte == 6 { 0 } else { mode_byte };
    let mode = CameraMode::new(mode_byte).unwrap_or(CameraMode::NORMAL);
    let dt = clock.as_deref().map(|clock| clock.delta).unwrap_or(0.0);
    if free_camera {
        camera.set_mode_byte(CameraMode::FREE);
        if update_rig_body(&mut camera, CameraMode::FREE, dt).is_none() {
            // U: free camera 输入平移/旋转驱动未恢复。
        }
    } else {
        update_camera_for_mode(&mut camera, mode, dt);
    }
}

/// O 0x1409145a0：暂停路径不做输入/快照门，按模式字节更新并重算矩阵。
pub fn paused_main_camera_update(
    camera: Option<ResMut<MainCamera>>,
    window: Option<Res<WindowSize>>,
    gui: Option<Res<DebugGui>>,
) {
    let Some(mut camera) = camera else {
        return;
    };
    if let Some(window) = window.as_deref() {
        if window.height != 0 {
            camera.set_aspect_from_window(window.width, window.height);
        }
    }
    let mode_byte = gui
        .as_deref()
        .map(|gui| gui.camera_mode.0)
        .unwrap_or_else(|| camera.mode_byte());
    let mode = CameraMode::new(mode_byte).unwrap_or(CameraMode::NORMAL);
    update_camera_for_mode(&mut camera, mode, 0.0);
}

/// O 0x140911e40：`dt = GameClock.delta * DebugGui.camera_snapshot_interpolation_speed`，
/// 按快照 mode 分发。U：27 个跳表目标的逐模式快照应用体未恢复，本实现只推进
/// 已核对的插值步长。
pub fn camera_snapshot(
    snapshot: Option<ResMut<CameraSnapshot>>,
    gui: Option<Res<DebugGui>>,
    clock: Option<Res<GameClock>>,
) {
    let Some(mut snapshot) = snapshot else {
        return;
    };
    if !snapshot_mode_is_valid(snapshot.mode) {
        log::error!(
            target: "country_core::systems::main_camera",
            "camera snapshot mode {} out of range",
            snapshot.mode
        );
        return;
    }
    let speed = gui
        .as_deref()
        .map(|gui| gui.camera_snapshot_interpolation_speed)
        .unwrap_or(1.0);
    let dt = clock.as_deref().map(|clock| clock.delta).unwrap_or(0.0);
    snapshot.value += dt * speed;
    snapshot.active = true;
}

/// O 0x140912a40：日志 `Resetting the camera.` → `MainCamera::new(旧 aspect)` →
/// 覆盖 0x4e0 缓冲 → 清参数缓存（缓存资源类型 U）→ 推 `WriteSettingsFileCmd`。
pub fn reset_camera(
    camera: Option<ResMut<MainCamera>>,
    mut settings_cmd: Option<ResMut<Events<WriteSettingsFileCmd>>>,
) {
    let Some(mut camera) = camera else {
        return;
    };
    log::info!(
        target: "country_core::systems::main_camera",
        "Resetting the camera."
    );
    let aspect = camera.aspect();
    *camera = MainCamera::new(aspect);
    if let Some(events) = settings_cmd.as_mut() {
        events.send(WriteSettingsFileCmd);
    }
}

/// O 0x14090e510：Walk 出生点 teleport。
///
/// 原件先做地形采样（FUN_1409e9640）与地面球投，再把归一化方向交给
/// `WalkCamera::teleport`。U：地形采样/球投 API 属于碰撞域，未在此恢复；
/// 调用方直接提供已求得的出生点与方向。
pub fn init_walk_camera(
    camera: &mut MainCamera,
    spawn: Vec3,
    direction: Vec3,
) -> anyhow::Result<()> {
    let rigs = camera
        .startup_rigs
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("MainCamera rigs missing"))?;
    let mut rigs = rigs
        .lock()
        .map_err(|_| anyhow::anyhow!("MainCamera rigs poisoned"))?;
    crate::systems::camera_rig::walk::teleport_rig(&mut rigs.walk, spawn, direction);
    // I：原件 teleport 只写 driver；这里立即跑一次 dt=0 以取得与后续
    // `WalkCamera::update` 相同的 final_transform。
    rigs.walk.update(0.0);
    let transform = rigs.walk.final_transform;
    let position = Vec3::new(
        transform.position.x,
        transform.position.y,
        transform.position.z,
    );
    let rotation = Quat::from_xyzw(
        transform.rotation.v.x,
        transform.rotation.v.y,
        transform.rotation.v.z,
        transform.rotation.s,
    );
    drop(rigs);
    camera.write_rig_transform(WALK_BODY_OFFSET, position, rotation);
    camera.set_mode_byte(CameraMode::WALK);
    camera.update_frame_matrices(position, rotation);
    Ok(())
}

/// Walk body 的 teleport；与 [`init_walk_camera`] 同一实现，不改变模式字节。
pub fn teleport(camera: &mut MainCamera, spawn: Vec3, direction: Vec3) -> anyhow::Result<()> {
    let mode = camera.mode_byte();
    init_walk_camera(camera, spawn, direction)?;
    camera.set_mode_byte(CameraMode::new(mode).unwrap_or(CameraMode::WALK));
    Ok(())
}

/// Normal body 的旋转组合（供测试与需要纯函数的调用方）。
pub fn normal_body_rotation(yaw: f32, pitch: f32) -> Quat {
    Quat::from_rotation_y(yaw * ORBIT_ANGLE_SCALE)
        * Quat::from_rotation_x(pitch * ORBIT_ANGLE_SCALE)
}

/// Normal body 位置；(0,5,0) 的默认值来自 `CameraRigParametersCache::default`。
pub fn normal_body_position(camera: &MainCamera) -> Vec3 {
    Vec3::new(
        camera.read_f32(NORMAL_POSITION_OFFSET),
        camera.read_f32(NORMAL_POSITION_OFFSET + 4),
        camera.read_f32(NORMAL_POSITION_OFFSET + 8),
    )
}

/// 当前 Normal 朝向。
pub fn normal_body_yaw_pitch(camera: &MainCamera) -> (f32, f32) {
    (
        camera.read_f32(NORMAL_YAW_OFFSET),
        camera.read_f32(NORMAL_PITCH_OFFSET),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::schedule::Schedule;

    fn camera_world() -> World {
        let mut world = World::new();
        world.insert_resource(MainCamera::new(16.0 / 9.0));
        world.insert_resource(WindowSize::new(1920, 1080));
        world.insert_resource(MainCameraUpdateInput::default());
        world.insert_resource(GameClock::default());
        world
    }

    #[test]
    fn update_writes_aspect_mouse_depth_and_real_matrices() {
        let mut world = camera_world();
        world.resource_mut::<MainCameraUpdateInput>().mouse_depth_byte = 0x2a;
        let mut schedule = Schedule::new(bevy_app::Update);
        schedule.add_systems(main_camera_update);
        schedule.run(&mut world);
        let camera = world.resource::<MainCamera>();
        assert_eq!(camera.aspect(), 1920.0 / 1080.0);
        assert_eq!(camera.mouse_depth_byte(), 0x2a);
        assert_eq!(camera.mode_byte(), CameraMode::NORMAL.0);
        // Normal 模式写入了四个真实矩阵。
        assert_ne!(camera.read_matrix(0), glam::Mat4::IDENTITY);
    }

    #[test]
    fn new_session_gate_waits_for_the_first_cursor_move() {
        let mut world = camera_world();
        world.resource_mut::<MainCameraUpdateInput>().new_session_pending = true;
        world.insert_resource(CursorPositionSS(glam::Vec2::ZERO));
        let before = world.resource::<MainCamera>().bytes().to_vec();
        let mut schedule = Schedule::new(bevy_app::Update);
        schedule.add_systems(main_camera_update);
        schedule.run(&mut world);
        assert!(world
            .resource::<MainCameraUpdateInput>()
            .new_session_pending);
        assert_eq!(world.resource::<MainCamera>().bytes().to_vec(), before);

        // 光标移动后门被清除。
        world
            .resource_mut::<CursorPositionSS>()
            .0
            .x = 5.0;
        let mut schedule = Schedule::new(bevy_app::Update);
        schedule.add_systems(main_camera_update);
        schedule.run(&mut world);
        assert!(!world
            .resource::<MainCameraUpdateInput>()
            .new_session_pending);
    }

    #[test]
    fn small_windows_emit_the_settings_command() {
        let mut world = camera_world();
        world.insert_resource(WindowSize::new(2, 2));
        world.insert_resource(Events::<WriteSettingsFileCmd>::default());
        let mut schedule = Schedule::new(bevy_app::Update);
        schedule.add_systems(main_camera_update);
        schedule.run(&mut world);
        let events = world.resource::<Events<WriteSettingsFileCmd>>();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn free_mode_updates_its_rig_and_writes_body_transform() {
        let mut world = World::new();
        world.insert_resource(MainCamera::new(16.0 / 9.0));
        world.insert_resource(MainCameraUpdateInput {
            free_camera_enabled: true,
            ..Default::default()
        });
        let mut schedule = Schedule::new(bevy_app::Update);
        schedule.add_systems(main_camera_update);
        schedule.run(&mut world);
        let camera = world.resource::<MainCamera>();
        assert_eq!(camera.mode_byte(), CameraMode::FREE.0);
        // 分叉 CameraTransform rotation-first：+0x18..+0x28 = quaternion，
        // +0x28..+0x34 = position（Free body 基址 0x270）。
        assert!(camera.read_f32(FREE_BODY_OFFSET + 0x24).is_finite());
        assert!(camera.read_f32(FREE_BODY_OFFSET + 0x28).is_finite());
    }

    #[test]
    fn reset_rebuilds_the_default_camera_and_keeps_aspect() {
        let mut world = World::new();
        let mut camera = MainCamera::new(1.5);
        camera.set_mouse_depth_byte(9);
        world.insert_resource(camera);
        // O 0x140912a40 用 `MainCamera::new(旧 aspect)` 覆盖缓冲；Normal body
        // 位置来自构造器的轨道默认值，不是 (0,5,0)（后者属于 CamRig 参数默认）。
        let constructor_position = normal_body_position(&MainCamera::new(1.5));
        let mut schedule = Schedule::new(bevy_app::Update);
        schedule.add_systems(reset_camera);
        schedule.run(&mut world);
        let camera = world.resource::<MainCamera>();
        assert_eq!(camera.aspect(), 1.5);
        assert_eq!(camera.mouse_depth_byte(), 0);
        assert_eq!(normal_body_position(camera), constructor_position);
    }

    #[test]
    fn snapshot_step_is_delta_times_gui_speed_and_rejects_out_of_range_mode() {
        let mut world = World::new();
        let mut clock = GameClock::default();
        clock.delta = 0.5;
        world.insert_resource(clock);
        let gui = DebugGui {
            camera_snapshot_interpolation_speed: 2.0,
            ..DebugGui::default()
        };
        world.insert_resource(gui);
        world.insert_resource(CameraSnapshot {
            mode: 3,
            value: 0.0,
            active: false,
        });
        let mut schedule = Schedule::new(bevy_app::Update);
        schedule.add_systems(camera_snapshot);
        schedule.run(&mut world);
        let snapshot = world.resource::<CameraSnapshot>();
        assert_eq!(snapshot.value, 1.0);
        assert!(snapshot.active);

        world.resource_mut::<CameraSnapshot>().mode = SNAPSHOT_MODE_SLOTS;
        let mut schedule = Schedule::new(bevy_app::Update);
        schedule.add_systems(camera_snapshot);
        schedule.run(&mut world);
        assert_eq!(world.resource::<CameraSnapshot>().value, 1.0);
    }

    #[test]
    fn init_walk_camera_teleports_the_real_rig_and_matrix_block() {
        let mut camera = MainCamera::new(16.0 / 9.0);
        init_walk_camera(&mut camera, Vec3::new(2.0, 3.0, 4.0), Vec3::NEG_Z).unwrap();
        assert_eq!(camera.mode_byte(), CameraMode::WALK.0);
        assert!((camera.read_f32(WALK_BODY_OFFSET + 0x28) - 2.0).abs() < 1e-5);
        assert!((camera.read_f32(WALK_BODY_OFFSET + 0x2c) - 3.0).abs() < 1e-5);
        assert_ne!(camera.read_matrix(0xc0), glam::Mat4::IDENTITY);
        // Walk body 分叉变换 rotation-first：-Z 朝向 → 单位四元数。
        assert!((camera.read_f32(WALK_BODY_OFFSET + 0x18) - 0.0).abs() < 1e-5);
        assert!((camera.read_f32(WALK_BODY_OFFSET + 0x24) - 1.0).abs() < 1e-5);
        // Normal body 的 yaw/pitch 不被 walk teleport 改写。
        let (before_yaw, before_pitch) = normal_body_yaw_pitch(&MainCamera::new(16.0 / 9.0));
        let (yaw, pitch) = normal_body_yaw_pitch(&camera);
        assert_eq!((yaw, pitch), (before_yaw, before_pitch));
        assert!(normal_body_rotation(0.0, 0.0).abs_diff_eq(Quat::IDENTITY, 1e-6));
    }
}
