//! MainCamera owns the six original B bodies, lenses and persistent rig state.
//! Normal/TiltShift/TopDown use the restored orbit queue and soft bounds;
//! Free/Walk/Butterfly reuse the existing original Dolly0.6 drivers. The byte
//! mirror supports existing consumers; every vector and rig has a Rust owner.
//! Per-frame controls/snapshot/paused systems are in systems::main_camera.
//! B FOV table142aabb98 has six entries; subsequent qwords are depth-pivot
//! offset tables, not more modes. The old 13-mode/CameraMode::previous claim
//! was false (previous@140914ac0 belongs to CameraRotationPivot).

use bevy_ecs::resource::Resource;
use glam::{Mat4, Quat, Vec3};
use ::utils::bytes::{read_f32_le, write_f32_le, write_u64_le};

pub const MAIN_CAMERA_SIZE: usize = 0x4e0;
pub const MODE_OFFSET: usize = 0x4d0;
pub const ASPECT_OFFSET: usize = 0x4d8;
/// B main_camera_update140912e66..e74 copies Modifiers low byte into Walk.
pub const WALK_MODIFIER_OFFSET: usize = 0x344;

/// 维护的 `CameraMatrices.inverse_view` 列（O：`CameraMatrices::eye_position`
/// 0x140c96d40 读 `[base+0xc0..0x100]`；`MainCamera::eye_position` 0x140911dd0 以
/// `self+0` 转发到该矩阵块）。
pub const INVERSE_VIEW_OFFSET: usize = 0xc0;

/// B owned rotations Vec header: cap100,ptr108,len110. The earlier equal
/// cap/len=1 constructor obscured their order; normal::update reads len+10.
pub const ROTATIONS_VEC_OFFSET: usize = 0x100;
pub const ROTATIONS_LEN_OFFSET: usize = 0x110;

/// 轨道四元数角度系数（O 0x140911520：`mulss xmm2/xmm3, [0x142aab4b0]` =
/// 0x3c8efa35）。非 π/180 字面量，按位保留。
pub const ORBIT_ANGLE_SCALE: f32 = f32::from_bits(0x3c8efa35);

/// Normal body 内：位置（+0x138）、pitch（+0x150）、yaw（+0x154）、垂直 fov（+0x190）。
/// O：`store_parameters` 读这四处；`match_free_camera_to_normal`（0x14090e6f0）把 `[+0x154]` 作 yaw、
/// `[+0x150]` 作 pitch 喂给 dolly 的 `YawPitch`。
pub const NORMAL_POSITION_OFFSET: usize = 0x138;
pub const NORMAL_PITCH_OFFSET: usize = 0x150;
pub const NORMAL_YAW_OFFSET: usize = 0x154;
pub const NORMAL_FOV_OFFSET: usize = 0x190;

/// B six-qword FOV table142aabb98, relative to MainCamera+100.
pub const FOV_OFFSETS: [usize; 6] = [0x8c, 0x144, 0x1c4, 0x264, 0x31c, 0x3a4];

/// B six original body modes0..5.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CameraMode(pub u8);

impl CameraMode {
    pub const NORMAL: Self = Self(0);
    pub const TILT_SHIFT: Self = Self(1);
    pub const FREE: Self = Self(2);
    pub const WALK: Self = Self(3);
    pub const TOP_DOWN: Self = Self(4);
    pub const BUTTERFLY: Self = Self(5);

    pub fn new(byte: u8) -> Option<Self> {
        (byte < 6).then_some(Self(byte))
    }
}

/// 0x24 B 的相机参数缓存记录（O：`store_parameters` 的输出布局）。
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CameraRigParameters {
    pub yaw: f32,
    pub pitch: f32,
    pub position: Vec3,
    /// +0x14：`store_parameters` 固定写 0（I：名字取自会话存档的字段名）。
    pub camera_rig_arm_length: f32,
    pub vertical_fov_degrees: f32,
    /// +0x1c / +0x20：固定写 0（名字 U）。
    pub pitch_offset: f32,
    pub yaw_offset: f32,
}

impl CameraRigParameters {
    pub const SIZE: usize = 0x24;

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let words = [
            self.yaw,
            self.pitch,
            self.position.x,
            self.position.y,
            self.position.z,
            self.camera_rig_arm_length,
            self.vertical_fov_degrees,
            self.pitch_offset,
            self.yaw_offset,
        ];
        let mut out = [0u8; Self::SIZE];
        for (chunk, word) in out.chunks_exact_mut(4).zip(words) {
            chunk.copy_from_slice(&word.to_le_bytes());
        }
        out
    }
}

const LOG_TARGET: &str = "country_core::systems::camera_rig";

fn finite_or(value: f32, fallback: f32, what: &str) -> f32 {
    // O：`0x7f7fffff < (bits & 0x7fffffff)` 即非有限（NaN 或无穷）。
    if value.is_finite() {
        value
    } else {
        log::error!(target: LOG_TARGET, "Camera {what} invalid. Overriding param cache value.");
        fallback
    }
}

/// B `country_core::systems::main_camera::MainCameraMoved`（roof ui_edit_roof
/// 0x141a18dd0 消费的事件；观测载荷为相机位置）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct MainCameraMoved;

/// 28 B ActiveRotation 记录（O 0x140911520 机器码：`+0=0(u64)`、`+8=pitch_delta`、
/// `+0xc=yaw_delta`、`+0x10=0x80000000`、`+0x14=0x80000000`、`+0x18=-1.0`；push 入
/// `rotations Vec@+0x100`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveRotation {
    /// +0：0（qword）。
    pub pad: u64,
    /// +8：`params.pitch - camera[+0x150]`。
    pub pitch_delta: f32,
    /// +0xc：`params.yaw - camera[+0x154]`。
    pub yaw_delta: f32,
    /// +0x10：0x80000000（u32）。
    pub field_10: u32,
    /// +0x14：0x80000000（u32）。
    pub field_14: u32,
    /// +0x18：-1.0（f32）。
    pub field_18: f32,
}

impl ActiveRotation {
    /// 便捷构造（I：除 `pitch_delta`/`yaw_delta` 外的常量与 0x140911520 逐位一致）。
    pub fn new(pitch_delta: f32, yaw_delta: f32) -> Self {
        Self {
            pad: 0,
            pitch_delta,
            yaw_delta,
            field_10: 0x80000000,
            field_14: 0x80000000,
            field_18: -1.0,
        }
    }
}

/// `MainCamera` 的原始 0x4e0 字节。原版为 ECS 资源（`input_record_replay`/`control_input_replay`
/// 0x1409423d0/0x140941a30 以 `Res<MainCamera>` 调 `store_parameters`）。
///
/// crate 内新增两个已核对状态：`matrices`（原版 @+0x0 矩阵块区，`inverse_view` 写 raw
/// +0xc0）与 `rotations`（原版 `rotations Vec@+0x100`，len 镜像写 raw +0x100）。
#[derive(Resource, Clone)]
pub struct MainCamera {
    raw: Box<[u8; MAIN_CAMERA_SIZE]>,
    /// ActiveRotation 队列（O 0x140911520：rotations Vec@+0x100）。
    pub rotations: Vec<ActiveRotation>,
    /// Safe owners of the other two NormalCamera body backlogs. All scalar
    /// body state stays in the same canonical raw mirror; no pointer readback.
    orbit_backlogs: [Vec<super::normal_body::ActiveRotation>; 2],
    /// Rust owns the dolly allocations instead of storing original-process
    /// pointers in `raw`. The native constructor builds these three rigs.
    pub startup_rigs: Option<std::sync::Arc<std::sync::Mutex<StartupCameraRigs>>>,
}

/// Immutable inputs for a render-only rotation preview. No rig, queue or game
/// state can be advanced through this value.
#[derive(Clone, Copy, Debug)]
pub struct CameraRotationPreview {
    position: Vec3,
    yaw: f32,
    pitch: f32,
    kind: RotationPreviewKind,
}

#[derive(Clone, Copy, Debug)]
enum RotationPreviewKind {
    Orbit { arm: Vec3, pending_pitch: f32, min: f32, max: f32 },
    Dolly { target_pitch: f32 },
}

impl CameraRotationPreview {
    /// Apply only the new input delta to the already evaluated pose. Pending
    /// motion, zoom, translation and smoothing remain in their simulation owner.
    pub fn apply(self, camera: &mut MainCamera, yaw_delta: f32, pitch_delta: f32) -> bool {
        if !yaw_delta.is_finite() || !pitch_delta.is_finite() {
            return false;
        }
        let (position, rotation) = match self.kind {
            RotationPreviewKind::Orbit { arm, pending_pitch, min, max } => {
                let base = self.pitch + pending_pitch;
                let wanted = -(pitch_delta + base);
                let bounded = wanted.min(max).max(min);
                let pitch = self.pitch + (-bounded - base);
                let rotation = |yaw: f32, pitch: f32| {
                    Quat::from_rotation_y(yaw * ORBIT_ANGLE_SCALE)
                        * Quat::from_rotation_x(pitch * ORBIT_ANGLE_SCALE)
                };
                let previous = rotation(self.yaw, self.pitch);
                let current = rotation(self.yaw + yaw_delta, pitch);
                (self.position + current * arm - previous * arm, current)
            }
            RotationPreviewKind::Dolly { target_pitch } => {
                // The canonical Free/Walk controller scales input by 0.5
                // before Dolly's [-90, 90] degree clamp.
                let pitch_delta = (target_pitch + pitch_delta * 0.5).clamp(-90.0, 90.0) - target_pitch;
                let rotation = Quat::from_euler(glam::EulerRot::YXZ,
                    (self.yaw + yaw_delta * 0.5).to_radians(),
                    (self.pitch + pitch_delta).to_radians(), 0.0);
                (self.position, rotation)
            }
        };
        if !position.is_finite() || !rotation.is_finite() {
            return false;
        }
        let matrices = utils::camera::CameraBodyMatrices::from_position_rotation(position, rotation);
        camera.write_matrix(0x80, &matrices.view);
        camera.write_matrix(0xc0, &matrices.inverse_view);
        true
    }
}

/// Dolly is the original locked dependency. Only the game's driver composition
/// is recovered here; its smoothing and rig evaluation are reused unchanged.
#[derive(Debug)]
pub struct StartupCameraRigs {
    pub free: dolly::prelude::CameraRig,
    pub walk: dolly::prelude::CameraRig,
    pub butterfly: dolly::prelude::CameraRig,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            raw: Box::new([0; MAIN_CAMERA_SIZE]),
            rotations: Vec::new(),
            orbit_backlogs: std::array::from_fn(|_| Vec::new()),
            startup_rigs: None,
        }
    }
}

impl MainCamera {
    /// Capture on the simulation owner after its normal update. The orbit
    /// anchor and pending pitch come from that same selected body; Free/Walk
    /// copy the real driver's target clamp without sharing its mutable owner.
    pub fn rotation_preview(&self) -> Option<CameraRotationPreview> {
        let position = self.eye_position();
        let (yaw, pitch, kind) = match self.mode_byte() {
            mode @ (0 | 1 | 4) => {
                let base = match mode { 0 => 0x100, 1 => 0x1b8, _ => 0x390 };
                if u32::from_le_bytes(self.raw[base + 0x18..base + 0x1c].try_into().unwrap()) == 0 {
                    return None;
                }
                let backlog: f32 = match mode {
                    0 => self.rotations.iter().map(|r| r.pitch_delta).sum(),
                    1 => self.orbit_backlogs[0].iter().map(|r| r.pitch_delta).sum(),
                    _ => self.orbit_backlogs[1].iter().map(|r| r.pitch_delta).sum(),
                };
                let direction = self.read_vec3(base + 0x2c);
                (self.read_f32(base + 0x54), self.read_f32(base + 0x50), RotationPreviewKind::Orbit {
                    arm: -direction * self.read_f32(base + 0x1c),
                    pending_pitch: backlog + self.read_f32(base + 0x24),
                    min: self.read_f32(base + 0x58), max: self.read_f32(base + 0x5c),
                })
            }
            mode @ (2 | 3) => {
                let rigs = self.startup_rigs.as_ref()?.lock().ok()?;
                let rig = if mode == 2 { &rigs.free } else { &rigs.walk };
                let target_pitch = rig.driver::<dolly::drivers::YawPitch>().pitch_degrees;
                let (_, rotation, _) = self.read_matrix(0xc0).to_scale_rotation_translation();
                let (yaw, pitch, _) = rotation.to_euler(glam::EulerRot::YXZ);
                (yaw.to_degrees(), pitch.to_degrees(), RotationPreviewKind::Dolly { target_pitch })
            }
            _ => return None,
        };
        if !position.is_finite() || !yaw.is_finite() || !pitch.is_finite() { return None; }
        match kind {
            RotationPreviewKind::Orbit { arm, pending_pitch, min, max }
                if !arm.is_finite() || !pending_pitch.is_finite()
                    || !min.is_finite() || !max.is_finite() || min > max => return None,
            RotationPreviewKind::Dolly { target_pitch } if !target_pitch.is_finite() => return None,
            _ => {},
        }
        Some(CameraRotationPreview { position, yaw, pitch, kind })
    }

    /// Read-only declaration view. Camera interpolation and Dolly drivers stay
    /// exclusively with the game-thread producer; rendering needs only the
    /// maintained matrices, lens values and mode stored in the scalar mirror.
    /// In particular, ordinary `Clone` also clones the shared mutable Dolly
    /// owner and the rotation queues, which is unsuitable for frame extraction.
    pub fn render_snapshot(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            rotations: Vec::new(),
            orbit_backlogs: std::array::from_fn(|_| Vec::new()),
            startup_rigs: None,
        }
    }

    /// B `MainCamera::new` @0x14090ec70. `aspect` is the original XMM1
    /// argument: run_game_thread calls WindowSize::aspect_ratio immediately
    /// before the constructor (@0x14059dba3..0x14059dbe3).
    ///
    /// The byte view is a semantic mirror: allocation pointers and ABI padding
    /// are not copied from the original process. Real driver state is retained
    /// by `startup_rigs`; the four camera matrices initially remain identity,
    /// just as the final memcpy @0x14090fc2a writes them.
    pub fn new(aspect: f32) -> Self {
        use dolly::drivers::{Arm, LookAt, Position, Rotation, Smooth, YawPitch};
        use dolly::prelude::CameraRig;

        let free = CameraRig::builder()
            .with(Position::new([0.0, 1.0, 0.0]))
            .with(YawPitch::new())
            .with(Smooth::new_position_rotation(1.0, 1.0))
            .build();
        let walk = CameraRig::builder()
            .with(Position::new([0.0, 1.0, 0.0]))
            .with(YawPitch::new())
            .build();
        let butterfly = CameraRig::builder()
            .with(Position::new([0.0, 0.0, 0.0]))
            .with(Rotation::new([0.0, 0.0, 0.0, 1.0]))
            .with(Smooth::new_position(2.0).predictive(true))
            .with(Arm::new([0.0, 1.5, 3.5]))
            .with(Smooth::new_position(4.0))
            .with(LookAt::new([0.0, 0.0, 0.0]).tracking_smoothness(4.0))
            .build();

        let mut camera = Self::default();
        for offset in [0, 0x40, 0x80, 0xc0] {
            camera.write_matrix(offset, &Mat4::IDENTITY);
        }
        // Three NormalCamera bodies, direct stores @0x14090fc2f..0x14090fdc3.
        // The radians use the exact constants from 0x142aab3d0/3d4.
        let yaw = f32::from_bits(0xbf0a_8263);
        let pitch = f32::from_bits(0xbf06_0a92);
        let orbit = Quat::from_euler(glam::EulerRot::YXZ, yaw, pitch, 0.0);
        let top_down = Quat::from_euler(glam::EulerRot::YXZ, 0.0, f32::from_bits(0xbfc9_0fdb), 0.0);
        for (base, rotation, radius, fields, loose, state, damping) in [
            (
                0x100,
                orbit,
                50.0,
                [-30.0, -31.0, 5.0, 90.0, 90.0, 150.0],
                false,
                2,
                0.15,
            ),
            (
                0x1b8,
                orbit,
                125.0,
                [-30.0, -31.0, 12.0, 80.0, 300.0, 450.0],
                true,
                3,
                0.075,
            ),
            (
                0x390,
                top_down,
                500.0,
                [-90.0, 0.0, 75.0, 90.0, 1000.0, 1200.0],
                true,
                3,
                0.15,
            ),
        ] {
            camera.write_u64(base, 0);
            camera.write_u64(base + 8, 4); // empty Vec's alignment sentinel
            camera.write_u64(base + 0x10, 0);
            let position = rotation * Vec3::new(0.0, 0.0, radius);
            for (i, value) in position.to_array().into_iter().enumerate() {
                camera.write_f32(base + 0x38 + i * 4, value);
            }
            for (i, value) in fields.into_iter().enumerate() {
                camera.write_f32(base + 0x50 + i * 4, value);
            }
            camera.write_f32(base + 0x68, 0.75);
            camera.raw[base + 0x6c] = u8::from(loose);
            camera.raw[base + 0x70..base + 0x74].copy_from_slice(&u32::to_le_bytes(state));
            camera.write_f32(base + 0x88, damping);
        }

        camera.write_initial_rig_transform(0x270, &free);
        camera.write_initial_rig_transform(0x2f0, &walk);
        camera.write_initial_rig_transform(0x448, &butterfly);
        camera.raw[0x328] = 0x11; // WalkCamera::new @0x140993fd9
        for state in [0x2a8, 0x348, 0x488] {
            camera.raw[state..state + 4].copy_from_slice(&3_u32.to_le_bytes());
        }
        camera.write_u64(0x480, u32::MAX.into());
        camera.write_f32(0x2c0, 0.25);
        camera.write_f32(0x360, 0.3);
        camera.write_f32(0x4a0, 0.15);
        // CameraLens::new calls @ed73/f23d/f5a2/f678/f79d/f85e.
        for (offset, lens) in [
            (
                0x18c,
                utils::camera::CameraLens::new(0.1, 45.0, 5.0, 90.0, false, 40.0, 10.0, 80.0),
            ),
            (
                0x244,
                utils::camera::CameraLens::new(0.1, 15.0, 5.0, 20.0, true, 960.0, 600.0, 2000.0),
            ),
            (
                0x2c4,
                utils::camera::CameraLens::new(0.1, 45.0, 5.0, 90.0, false, 40.0, 10.0, 80.0),
            ),
            (
                0x364,
                utils::camera::CameraLens::new(0.1, 75.0, 30.0, 90.0, false, 10.0, 2.0, 30.0),
            ),
            (
                0x41c,
                utils::camera::CameraLens::new(0.1, 8.0, 2.0, 10.0, false, 40.0, 20.0, 60.0),
            ),
            (
                0x4a4,
                utils::camera::CameraLens::new(0.1, 45.0, 5.0, 90.0, false, 40.0, 10.0, 80.0),
            ),
        ] {
            camera.write_initial_lens(offset, lens);
        }
        camera.raw[MODE_OFFSET] = 0;
        camera.write_f32(ASPECT_OFFSET, aspect);
        camera.startup_rigs = Some(std::sync::Arc::new(std::sync::Mutex::new(
            StartupCameraRigs {
                free,
                walk,
                butterfly,
            },
        )));
        camera
    }

    fn write_initial_rig_transform(&mut self, base: usize, rig: &dolly::prelude::CameraRig) {
        let transform = rig.final_transform;
        let q = transform.rotation;
        let p = transform.position;
        for (i, value) in [q.v.x, q.v.y, q.v.z, q.s, p.x, p.y, p.z]
            .into_iter()
            .enumerate()
        {
            self.write_f32(base + 0x18 + i * 4, value);
        }
    }

    fn write_initial_lens(&mut self, offset: usize, lens: utils::camera::CameraLens) {
        for (i, value) in [
            lens.field_0,
            lens.vertical_fov_degrees,
            lens.target_vertical_fov_degrees,
            lens.field_c,
            lens.field_10,
            lens.field_14,
            lens.field_18,
            lens.field_1c,
            lens.field_20,
            lens.field_24,
        ]
        .into_iter()
        .enumerate()
        {
            self.write_f32(offset + i * 4, value);
        }
        self.raw[offset + 0x28] = u8::from(lens.field_28);
    }

    pub fn bytes(&self) -> &[u8; MAIN_CAMERA_SIZE] {
        &self.raw
    }

    pub fn read_f32(&self, offset: usize) -> f32 {
        read_f32_le(&self.raw[..], offset)
    }

    /// Original @14090e6f0: preserve the normal camera's actual current
    /// position and yaw/pitch, replacing the existing Free Dolly rig.
    pub fn match_free_camera_to_normal(&mut self) {
        let position = Vec3::new(self.read_f32(0x138), self.read_f32(0x13c), self.read_f32(0x140));
        let free = super::free_camera::FreeCamera::match_normal(
            position, self.read_f32(0x150), self.read_f32(0x154),
        );
        self.startup_rigs.as_ref().expect("MainCamera::new owns Dolly rigs")
            .lock().unwrap().free = free.rig;
    }

    /// Original @14090e510 (473 B): place Walk at the authored spawn, above
    /// the current terrain/building surface. Collider mask includes all 17
    /// subworlds; its ray is solid with no exclusion and f32::MAX distance.
    #[cfg(feature = "native-physics")]
    pub fn init_walk_camera(
        &mut self,
        world: &crate::systems::collision::world::RaycastWorld,
        heights: &crate::resources::terrain_height::TerrainHeightsData,
    ) {
        let xz = glam::Vec2::new(-30.1927490234375, 15.641988754272461);
        let mut position = Vec3::new(xz.x, heights.sample_at_world_xz(xz.x, xz.y), xz.y);
        let hit = world.raycast_simple(position + Vec3::Y * 500.0, -Vec3::Y, 0x1ffff);
        if !hit.is_miss() { position = hit.position; }
        position.y = (position.y + 1.9).max(0.4);
        let direction = Vec3::new(0.8871124982833862, 0.0, -0.45905280113220215)
            .try_normalize().unwrap_or(Vec3::NEG_Z);
        let mut rigs = self.startup_rigs.as_ref().expect("MainCamera::new owns Dolly rigs")
            .lock().unwrap();
        super::walk::teleport_rig(&mut rigs.walk, position, direction);
    }

    pub fn write_f32(&mut self, offset: usize, value: f32) {
        write_f32_le(&mut self.raw[..], offset, value);
    }

    pub fn write_u64(&mut self, offset: usize, value: u64) {
        write_u64_le(&mut self.raw[..], offset, value);
    }

    pub fn read_matrix(&self, offset: usize) -> Mat4 {
        let mut arr = [0.0_f32; 16];
        for (i, chunk) in self.raw[offset..offset + 64].chunks_exact(4).enumerate() {
            arr[i] = f32::from_le_bytes(chunk.try_into().unwrap());
        }
        Mat4::from_cols_array(&arr)
    }

    pub fn write_matrix(&mut self, offset: usize, matrix: &Mat4) {
        for (i, word) in matrix.to_cols_array().into_iter().enumerate() {
            self.raw[offset + i * 4..offset + i * 4 + 4].copy_from_slice(&word.to_le_bytes());
        }
    }

    /// Read the maintained real projection and inverse-view matrices.
    pub fn camera_matrices(&self) -> utils::camera::CameraMatrices {
        utils::camera::CameraMatrices {
            projection: self.read_matrix(0),
            inverse_view: self.read_matrix(INVERSE_VIEW_OFFSET),
        }
    }

    /// 由宿主按帧维护 `inverse_view`（O：`main_camera_update` 每帧重建矩阵块@+0x0；
    /// 库内该更新系统未实现，此 setter 供宿主接线）。
    pub fn set_camera_matrices(&mut self, inverse_view: Mat4) {
        self.write_matrix(INVERSE_VIEW_OFFSET, &inverse_view);
    }

    /// 由 Normal body 位置/yaw/pitch 重建 `inverse_view` 并写入维护矩阵块（转发器辅助，
    /// I：projection=IDENTITY）。
    pub fn rebuild_camera_matrices_from_normal_body(&mut self) {
        // 位置直接取自 Normal body（+0x138/+0x13c/+0x140），不能经 `eye_position()`
        // 转发回维护矩阵——那会形成循环（矩阵尚未写入时读到 0）。
        let position = Vec3::new(
            self.read_f32(NORMAL_POSITION_OFFSET),
            self.read_f32(NORMAL_POSITION_OFFSET + 4),
            self.read_f32(NORMAL_POSITION_OFFSET + 8),
        );
        let yaw = self.read_f32(NORMAL_YAW_OFFSET);
        let pitch = self.read_f32(NORMAL_PITCH_OFFSET);
        let rotation = Quat::from_rotation_y(yaw * ORBIT_ANGLE_SCALE)
            * Quat::from_rotation_x(pitch * ORBIT_ANGLE_SCALE);
        let body = utils::camera::CameraBodyMatrices::from_position_rotation(position, rotation);
        self.set_camera_matrices(body.inverse_view);
    }

    pub fn mode_byte(&self) -> u8 {
        self.raw[MODE_OFFSET]
    }

    pub fn set_mode_byte(&mut self, mode: CameraMode) {
        self.raw[MODE_OFFSET] = mode.0;
    }

    pub fn aspect(&self) -> f32 {
        self.read_f32(ASPECT_OFFSET)
    }

    /// `main_camera_update` 的头一步：`aspect = width as f32 / height as f32`（0x140912c20）。
    pub fn set_aspect_from_window(&mut self, width: u32, height: u32) {
        self.write_f32(ASPECT_OFFSET, width as f32 / height as f32);
    }

    fn fov_slot(&self) -> usize {
        FOV_OFFSETS[self.mode_byte() as usize]
    }

    /// B target FOV read from the selected one of six original lenses.
    pub fn target_vertical_fov_degrees(&self) -> f32 {
        self.read_f32(self.fov_slot() + 0x108)
    }

    /// O 0x1409114f0：同时写 body 的 `+4` 与 `+8`（0x140c974f0）。
    pub fn immediately_set_vertical_fov_degrees(&mut self, fov: f32) {
        let base = self.fov_slot() + 0x100;
        self.write_f32(base + 8, fov);
        self.write_f32(base + 4, fov);
    }

    /// O 0x140911710：把 Normal body 的位置/yaw/pitch/fov 整理成 0x24 B 记录；非有限值各自替换为
    /// 默认（位置 `(0, 5, 0)`，其余 0）并记一条错误日志。
    pub fn store_parameters(&self) -> CameraRigParameters {
        let position = Vec3::new(
            self.read_f32(NORMAL_POSITION_OFFSET),
            self.read_f32(NORMAL_POSITION_OFFSET + 4),
            self.read_f32(NORMAL_POSITION_OFFSET + 8),
        );
        let position = if position.is_finite() {
            position
        } else {
            log::error!(target: LOG_TARGET, "Camera position invalid. Overriding param cache value.");
            Vec3::new(0.0, 5.0, 0.0)
        };
        let yaw = finite_or(self.read_f32(NORMAL_YAW_OFFSET), 0.0, "yaw");
        let pitch = finite_or(self.read_f32(NORMAL_PITCH_OFFSET), 0.0, "pitch");
        let vertical_fov_degrees = finite_or(
            self.read_f32(NORMAL_FOV_OFFSET),
            0.0,
            "vertical_fov_degrees",
        );
        CameraRigParameters {
            yaw,
            pitch,
            position,
            camera_rig_arm_length: 0.0,
            vertical_fov_degrees,
            pitch_offset: 0.0,
            yaw_offset: 0.0,
        }
    }

    /// 输入回放需要立即定位，不能排队等待旧镜头动画结束。
    pub fn restore_input_parameters(&mut self, parameters: &CameraRigParameters) {
        self.set_mode_byte(CameraMode::NORMAL);
        for (offset, value) in [
            (NORMAL_POSITION_OFFSET, parameters.position.x),
            (NORMAL_POSITION_OFFSET + 4, parameters.position.y),
            (NORMAL_POSITION_OFFSET + 8, parameters.position.z),
            (NORMAL_YAW_OFFSET, parameters.yaw),
            (NORMAL_PITCH_OFFSET, parameters.pitch),
            (NORMAL_FOV_OFFSET, parameters.vertical_fov_degrees),
        ] {
            self.write_f32(offset, value);
        }
        // 清除旧镜头动画尚未消费的平移和旋转，防止恢复后再次移动。
        for offset in [
            NORMAL_POSITION_OFFSET + 12,
            NORMAL_POSITION_OFFSET + 16,
            NORMAL_POSITION_OFFSET + 20,
        ] {
            self.write_f32(offset, 0.0);
        }
        self.rotations.clear();
        self.write_u64(ROTATIONS_LEN_OFFSET, 0);
        self.immediately_set_vertical_fov_degrees(parameters.vertical_fov_degrees);
        self.rebuild_camera_matrices_from_normal_body();
    }

    /// O 0x140911520 + 机器码核对：从会话相机记录恢复。流程（逐分支一致）：
    /// 1. `mode@+0x4d0 = 0`（Normal）；
    /// 2. `from_euler(YXZ, yaw·ORBIT_ANGLE_SCALE, pitch·ORBIT_ANGLE_SCALE, 0)` 算轨道四元数，
    ///    旋转 `(0,0,arm)`，结果 `(params.position - camera[+0x138/0x13c/0x140]) + rotated`
    ///    写 `+0x144/+0x148/+0x14c`（**不直写 Normal body 位置/yaw/pitch**）；
    /// 3. `+0x118 = 0`；
    /// 4. **替换** rotations Vec@+0x100（O：dealloc 旧+新 alloc，len=1），ActiveRotation
    ///    28B `{0, pitch−Δ, yaw−Δ, 0x80000000, 0x80000000, -1.0}`
    ///    with owned Vec header cap100/ptr108/len110;
    /// 5. `!ignore_snapshot` 时 `+0x194 = params.vertical_fov_degrees`（native+18）。
    pub fn load_parameters(
        &mut self,
        params: &crate::systems::camera_rig::CameraRigParametersCache,
        ignore_snapshot: bool,
    ) {
        self.set_mode_byte(CameraMode::NORMAL);
        // 轨道四元数：glam YXZ = from_rotation_y(yaw) * from_rotation_x(pitch)（0x140d4b440
        // 的 `in_DL=2`=YXZ；0.20.5 `rot_y(u)*rot_x(v)*rot_z(w)`）。
        let q = Quat::from_rotation_y(params.yaw * ORBIT_ANGLE_SCALE)
            * Quat::from_rotation_x(params.pitch * ORBIT_ANGLE_SCALE);
        let rotated = q * Vec3::new(0.0, 0.0, params.camera_rig_arm_length);
        let cam_pos = Vec3::new(
            self.read_f32(NORMAL_POSITION_OFFSET),
            self.read_f32(NORMAL_POSITION_OFFSET + 4),
            self.read_f32(NORMAL_POSITION_OFFSET + 8),
        );
        let new_pos = params.position - cam_pos + rotated;
        self.write_f32(0x144, new_pos.x);
        self.write_f32(0x148, new_pos.y);
        self.write_f32(0x14c, new_pos.z);
        self.write_u64(0x118, 0);
        // ActiveRotation 入 rotations Vec@+0x100（O 0x140911520 机器码：**替换非 push**——
        // 旧 len→`__rust_dealloc(旧ptr, 旧len·28, 4)`→新 alloc len=1/ptr/cap=1；len 恒 1）。
        let pitch_delta = params.pitch - self.read_f32(NORMAL_PITCH_OFFSET);
        let yaw_delta = params.yaw - self.read_f32(NORMAL_YAW_OFFSET);
        self.rotations = vec![ActiveRotation::new(pitch_delta, yaw_delta)];
        // len/ptr/cap 镜像写 raw +0x100/+0x108/+0x110（O：`[RSI+0x100]=1 / [RSI+0x108]=新ptr /
        // [RSI+0x110]=1`）。新 Vec 容量恒 1、无再分配，镜像保持有效。
        let rotations_len = self.rotations.len() as u64;
        let rotations_ptr = self.rotations.as_ptr() as usize as u64;
        let rotations_cap = self.rotations.capacity() as u64;
        self.write_u64(ROTATIONS_VEC_OFFSET, rotations_cap);
        self.write_u64(ROTATIONS_VEC_OFFSET + 8, rotations_ptr);
        self.write_u64(ROTATIONS_LEN_OFFSET, rotations_len);
        if !ignore_snapshot {
            self.write_f32(0x194, params.vertical_fov_degrees);
        }
    }

    /// Full body update replaces the earlier special zero-arm loading path.
    /// Original immediately_finish_interpolating uses dt100000. Loaded normal
    /// camera records, pending translations and arbitrary orbit arms are all
    /// handled by the same per-frame business algorithm.
    pub fn finish_loaded_normal_interpolation(&mut self, demo: bool) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.mode_byte() == CameraMode::NORMAL.0,
            "loaded camera must be Normal"
        );
        self.update_camera_matrices(100000.0, demo);
        Ok(())
    }

    pub(crate) fn read_vec3(&self, offset: usize) -> Vec3 {
        Vec3::new(
            self.read_f32(offset),
            self.read_f32(offset + 4),
            self.read_f32(offset + 8),
        )
    }
    fn write_vec3(&mut self, offset: usize, value: Vec3) {
        for (index, component) in value.to_array().into_iter().enumerate() {
            self.write_f32(offset + index * 4, component);
        }
    }

    /// Borrow a Normal-family body, transferring (not cloning) its owned Vec.
    /// Native cap/ptr/len are a diagnostic mirror only and are never dereferenced.
    pub(crate) fn with_normal_body<T>(
        &mut self,
        mode: CameraMode,
        op: impl FnOnce(&mut super::normal_body::NormalCameraBody) -> T,
    ) -> T {
        use super::normal_body::{ActiveRotation as Orbit, NormalCameraBody};
        let (base, slot) = match mode.0 {
            0 => (0x100, None),
            1 => (0x1b8, Some(0)),
            4 => (0x390, Some(1)),
            _ => panic!("camera mode has no Normal body"),
        };
        let rotations = if let Some(slot) = slot {
            std::mem::take(&mut self.orbit_backlogs[slot])
        } else {
            std::mem::take(&mut self.rotations)
                .into_iter()
                .map(|r| Orbit {
                    unknown_00: f32::from_bits(r.pad as u32),
                    arm_length: f32::from_bits((r.pad >> 32) as u32),
                    pitch_delta: r.pitch_delta,
                    yaw_delta: r.yaw_delta,
                    unknown_10: f32::from_bits(r.field_10),
                    unknown_14: f32::from_bits(r.field_14),
                    unknown_18: r.field_18,
                })
                .collect()
        };
        let active = (u32::from_le_bytes(self.raw[base + 0x18..base + 0x1c].try_into().unwrap())
            != 0)
            .then(|| Orbit {
                unknown_00: self.read_f32(base + 0x1c),
                arm_length: self.read_f32(base + 0x20),
                pitch_delta: self.read_f32(base + 0x24),
                yaw_delta: self.read_f32(base + 0x28),
                unknown_10: self.read_f32(base + 0x2c),
                unknown_14: self.read_f32(base + 0x30),
                unknown_18: self.read_f32(base + 0x34),
            });
        let mut body = NormalCameraBody {
            rotations,
            active,
            position: self.read_vec3(base + 0x38),
            pending_translation: self.read_vec3(base + 0x44),
            pitch: self.read_f32(base + 0x50),
            yaw: self.read_f32(base + 0x54),
            pitch_min: self.read_f32(base + 0x58),
            pitch_max: self.read_f32(base + 0x5c),
            radial_min: self.read_f32(base + 0x60),
            radial_max: self.read_f32(base + 0x64),
            terrain_clearance: self.read_f32(base + 0x68),
            loose_ground_projection: f32::from(self.raw[base + 0x6c]),
        };
        let value = op(&mut body);
        self.write_vec3(base + 0x38, body.position);
        self.write_vec3(base + 0x44, body.pending_translation);
        self.write_f32(base + 0x50, body.pitch);
        self.write_f32(base + 0x54, body.yaw);
        self.raw[base + 0x18..base + 0x1c]
            .copy_from_slice(&u32::from(body.active.is_some()).to_le_bytes());
        if let Some(r) = body.active {
            for (index, component) in [
                r.unknown_00,
                r.arm_length,
                r.pitch_delta,
                r.yaw_delta,
                r.unknown_10,
                r.unknown_14,
                r.unknown_18,
            ]
            .into_iter()
            .enumerate()
            {
                self.write_f32(base + 0x1c + index * 4, component);
            }
        }
        let (cap, ptr, len) = if let Some(slot) = slot {
            self.orbit_backlogs[slot] = body.rotations;
            let rotations = &self.orbit_backlogs[slot];
            (
                rotations.capacity(),
                rotations.as_ptr() as usize,
                rotations.len(),
            )
        } else {
            self.rotations = body
                .rotations
                .into_iter()
                .map(|r| ActiveRotation {
                    pad: u64::from(r.unknown_00.to_bits())
                        | (u64::from(r.arm_length.to_bits()) << 32),
                    pitch_delta: r.pitch_delta,
                    yaw_delta: r.yaw_delta,
                    field_10: r.unknown_10.to_bits(),
                    field_14: r.unknown_14.to_bits(),
                    field_18: r.unknown_18,
                })
                .collect();
            (
                self.rotations.capacity(),
                self.rotations.as_ptr() as usize,
                self.rotations.len(),
            )
        };
        self.write_u64(base, cap as u64);
        self.write_u64(base + 8, ptr as u64);
        self.write_u64(base + 0x10, len as u64);
        value
    }

    pub(crate) fn update_body_pose(&mut self, dt: f32, demo: bool) -> (Vec3, Quat) {
        match self.mode_byte() {
            0 | 1 | 4 => {
                self.with_normal_body(CameraMode(self.mode_byte()), |body| body.update(dt, demo))
            }
            mode @ (2 | 3 | 5) => {
                let mut update_dt = dt;
                if mode == 5 {
                    let applied = u32::from_le_bytes(self.raw[0x480..0x484].try_into().unwrap());
                    let selected = u32::from_le_bytes(self.raw[0x484..0x488].try_into().unwrap());
                    if applied != selected {
                        self.raw[0x480..0x484].copy_from_slice(&selected.to_le_bytes());
                        update_dt = 1000.0;
                    }
                }
                let owner = self
                    .startup_rigs
                    .as_ref()
                    .expect("original dolly rigs were not constructed")
                    .clone();
                let mut rigs = owner.lock().expect("camera rig mutex poisoned");
                let transform = match mode {
                    2 => rigs.free.update(update_dt),
                    3 => rigs.walk.update(update_dt),
                    5 => rigs.butterfly.update(update_dt),
                    _ => unreachable!(),
                };
                let p = transform.position;
                let q = transform.rotation;
                (
                    Vec3::new(p.x, p.y, p.z),
                    Quat::from_xyzw(q.v.x, q.v.y, q.v.z, q.s),
                )
            }
            mode => panic!("invalid original camera mode {mode}"),
        }
    }

    fn write_camera_pose_and_lens(&mut self, position: Vec3, rotation: Quat, fov: f32) {
        let lens = 0x100 + FOV_OFFSETS[self.mode_byte() as usize];
        let projection = utils::camera::CameraProjectionParams {
            aspect: self.aspect(),
            near: self.read_f32(lens),
            vertical_fov_degrees: fov,
        }
        .calc_matrices();
        let body = utils::camera::CameraBodyMatrices::from_position_rotation(position, rotation);
        self.write_matrix(0, &projection.projection);
        self.write_matrix(0x40, &projection.inverse_projection);
        self.write_matrix(0x80, &body.view);
        self.write_matrix(0xc0, &body.inverse_view);
    }
    pub(crate) fn update_camera_matrices(&mut self, dt: f32, demo: bool) {
        let (position, rotation) = self.update_body_pose(dt, demo);
        let lens = 0x100 + FOV_OFFSETS[self.mode_byte() as usize];
        let alpha = 1.0 - (dt / -0.2).exp();
        let fov = (1.0 - alpha) * self.read_f32(lens + 4) + alpha * self.read_f32(lens + 8);
        self.write_f32(lens + 4, fov);
        self.write_camera_pose_and_lens(position, rotation, fov);
    }
    /// Exact paused path includes `current + target * 0`, preserving its
    /// IEEE behavior and body.update(0) rather than inventing a frozen matrix.
    pub(crate) fn update_paused_camera_matrices(&mut self, demo: bool) {
        let (position, rotation) = self.update_body_pose(0.0, demo);
        let lens = 0x100 + FOV_OFFSETS[self.mode_byte() as usize];
        let fov = self.read_f32(lens + 4) + self.read_f32(lens + 8) * 0.0;
        self.write_f32(lens + 4, fov);
        self.write_camera_pose_and_lens(position, rotation, fov);
    }

    /// O 0x140911dd0（12 B 转发器）：`eye_position` → 维护的 `CameraMatrices`（@+0x0，
    /// `inverse_view` 平移列）。非每次从 body 重建。
    pub fn eye_position(&self) -> Vec3 {
        self.camera_matrices().eye_position()
    }

    /// O 0x140911df0（12 B 转发器）：`eye_forward`（`eye_direction` 同址）→ 维护的
    /// `CameraMatrices` 前向提取。
    pub fn eye_forward(&self) -> Vec3 {
        self.camera_matrices().eye_direction()
    }

    /// O 0x140911e10（12 B 转发器）：`xz_projective_rotation` → 维护的 `CameraMatrices`
    /// XZ 投影旋转。
    pub fn xz_projective_rotation(&self) -> glam::Quat {
        self.camera_matrices().xz_projective_rotation()
    }

    /// Reuse the real constructor at the existing window aspect.
    pub fn reset(&mut self) {
        *self = Self::new(self.aspect());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_snapshot_keeps_camera_values_without_simulation_owners() {
        let mut camera = MainCamera::new(16.0 / 9.0);
        camera.rotations.push(ActiveRotation::new(0.25, 0.5));
        let matrices = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        camera.write_matrix(INVERSE_VIEW_OFFSET, &matrices);
        let rigs = camera.startup_rigs.as_ref().unwrap();
        let owners = std::sync::Arc::strong_count(rigs);
        let snapshot = camera.render_snapshot();
        assert_eq!(std::sync::Arc::strong_count(rigs), owners);
        assert!(snapshot.startup_rigs.is_none());
        assert!(snapshot.rotations.is_empty());
        assert!(snapshot.orbit_backlogs.iter().all(Vec::is_empty));
        assert_eq!(snapshot.raw, camera.raw);
        assert_eq!(snapshot.eye_position(), Vec3::new(1.0, 2.0, 3.0));
        camera.write_matrix(INVERSE_VIEW_OFFSET, &Mat4::IDENTITY);
        assert_eq!(snapshot.eye_position(), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn orbit_preview_preserves_anchor_and_never_advances_simulation() {
        let mut camera = MainCamera::new(16.0 / 9.0);
        camera.update_camera_matrices(0.0, false);
        let eye = camera.eye_position();
        let anchor = eye + camera.eye_forward() * 10.0;
        camera.begin_selected_rotation(anchor, Vec3::NEG_Z, eye);
        let raw = camera.raw.clone();
        let preview = camera.rotation_preview().unwrap();
        let mut render = camera.render_snapshot();
        assert!(preview.apply(&mut render, 12.0, 0.0));
        assert_ne!(render.read_matrix(0xc0), camera.read_matrix(0xc0));
        let actual_anchor = render.eye_position() + render.eye_forward() * 10.0;
        assert!((actual_anchor - anchor).length() < 1e-4);
        assert_eq!(render.read_matrix(0), camera.read_matrix(0));
        assert_eq!(camera.raw, raw);
        assert!(camera.startup_rigs.is_some());
        assert!(render.startup_rigs.is_none());
    }

    #[test]
    fn free_walk_preview_keeps_position_and_driver_targets() {
        for mode in [CameraMode::FREE, CameraMode::WALK] {
            let mut camera = MainCamera::new(1.0);
            camera.set_mode_byte(mode);
            camera.update_camera_matrices(0.0, false);
            let preview = camera.rotation_preview().unwrap();
            let raw = camera.raw.clone();
            let mut render = camera.render_snapshot();
            assert!(preview.apply(&mut render, 20.0, 10.0));
            assert_eq!(render.eye_position(), camera.eye_position());
            assert_ne!(render.eye_forward(), camera.eye_forward());
            assert_eq!(camera.raw, raw);
            let rigs = camera.startup_rigs.as_ref().unwrap().lock().unwrap();
            let rig = if mode == CameraMode::FREE { &rigs.free } else { &rigs.walk };
            let target = rig.driver::<dolly::drivers::YawPitch>();
            assert_eq!(target.yaw_degrees, 0.0);
            assert_eq!(target.pitch_degrees, 0.0);
        }
    }

    #[test]
    fn original_intro_finishes_normal_camera_and_writes_four_real_matrices() {
        let params: crate::systems::camera_rig::CameraRigParametersCache = serde_json::from_str(
            include_str!("../../../tests/fixtures/original-intro-camera-b.json"),
        )
        .unwrap();
        let mut camera = MainCamera::new(16.0 / 9.0);
        camera.load_parameters(&params, false);
        camera.finish_loaded_normal_interpolation(false).unwrap();
        assert_eq!(camera.read_f32(0x190), 45.0);
        assert!(camera.rotations.is_empty());
        assert!((camera.eye_position() - params.position).length() < 0.0001);
        assert_ne!(camera.read_matrix(0), Mat4::IDENTITY);
        for pair in [(0, 0x40), (0x80, 0xc0)] {
            let product = camera.read_matrix(pair.0) * camera.read_matrix(pair.1);
            for (actual, expected) in product
                .to_cols_array()
                .into_iter()
                .zip(Mat4::IDENTITY.to_cols_array())
            {
                assert!((actual - expected).abs() < 0.0001);
            }
        }
    }

    #[test]
    fn constructor_matches_original_rne_semantic_fields_and_owns_the_rigs() {
        // Untouched B RNE 0x14090ec70, produced by the retained Unicorn
        // instruction oracle. Native allocation pointers and padding are
        // intentionally excluded; the driver owners are checked separately.
        let native = include_bytes!("../../../tests/fixtures/main-camera-new-b.bin");
        let camera = MainCamera::new(16.0 / 9.0);
        let native_f32 =
            |offset| f32::from_le_bytes(native[offset..offset + 4].try_into().unwrap());
        assert_eq!(&camera.bytes()[..0x100], &native[..0x100]);
        for lens in [0x18c, 0x244, 0x2c4, 0x364, 0x41c, 0x4a4] {
            assert_eq!(
                &camera.bytes()[lens..lens + 0x29],
                &native[lens..lens + 0x29],
                "lens {lens:#x}"
            );
        }
        for body in [0x100, 0x1b8, 0x390] {
            for offset in (body + 0x38..body + 0x44).step_by(4) {
                assert!(
                    (camera.read_f32(offset) - native_f32(offset)).abs() <= 0.0001,
                    "position {offset:#x}: {} != {}",
                    camera.read_f32(offset),
                    native_f32(offset)
                );
            }
            assert_eq!(
                &camera.bytes()[body + 0x50..body + 0x71],
                &native[body + 0x50..body + 0x71]
            );
            assert_eq!(
                camera.read_f32(body + 0x88).to_bits(),
                native_f32(body + 0x88).to_bits()
            );
        }
        for body in [0x270, 0x2f0, 0x448] {
            for offset in (body + 0x18..body + 0x34).step_by(4) {
                assert!(
                    (camera.read_f32(offset) - native_f32(offset)).abs() <= 0.00001,
                    "rig {offset:#x}: {} != {}",
                    camera.read_f32(offset),
                    native_f32(offset)
                );
            }
        }
        let rigs = camera.startup_rigs.as_ref().unwrap().lock().unwrap();
        assert_eq!(
            (
                rigs.free.drivers.len(),
                rigs.walk.drivers.len(),
                rigs.butterfly.drivers.len()
            ),
            (3, 2, 6)
        );
        assert_eq!(camera.bytes()[0x328], native[0x328]);
        assert_eq!(camera.bytes()[MODE_OFFSET], native[MODE_OFFSET]);
        assert_eq!(
            camera.read_f32(ASPECT_OFFSET).to_bits(),
            native_f32(ASPECT_OFFSET).to_bits()
        );
    }

    #[test]
    fn original_camera_modes_are_exactly_the_six_body_variants() {
        for mode in 0..6 {
            assert!(CameraMode::new(mode).is_some());
        }
        assert!(CameraMode::new(6).is_none());
        assert!(CameraMode::new(12).is_none());
        assert_eq!(FOV_OFFSETS.len(), 6);
    }

    #[test]
    fn fov_slots_follow_the_mode_byte() {
        let mut camera = MainCamera::default();
        camera.set_mode_byte(CameraMode::NORMAL);
        camera.immediately_set_vertical_fov_degrees(50.0);
        // Normal = 0x8c：基址 0x18c，当前 fov 在 +4（0x190，即 `NORMAL_FOV_OFFSET`），目标在 +8（0x194）。
        assert_eq!(camera.read_f32(NORMAL_FOV_OFFSET), 50.0);
        assert_eq!(camera.read_f32(0x194), 50.0);
        assert_eq!(camera.target_vertical_fov_degrees(), 50.0);
        camera.set_mode_byte(CameraMode::TOP_DOWN);
        camera.immediately_set_vertical_fov_degrees(8.0);
        assert_eq!(camera.read_f32(0x420), 8.0);
        assert_eq!(camera.target_vertical_fov_degrees(), 8.0);
    }

    #[test]
    fn aspect_is_width_over_height() {
        let mut camera = MainCamera::default();
        camera.set_aspect_from_window(1920, 1080);
        assert_eq!(camera.aspect(), 1920.0 / 1080.0);
    }

    #[test]
    fn store_parameters_sanitises_each_field_separately() {
        let mut camera = MainCamera::default();
        camera.write_f32(NORMAL_POSITION_OFFSET, 3.0);
        camera.write_f32(NORMAL_POSITION_OFFSET + 4, 4.0);
        camera.write_f32(NORMAL_POSITION_OFFSET + 8, 5.0);
        camera.write_f32(NORMAL_YAW_OFFSET, 90.0);
        camera.write_f32(NORMAL_PITCH_OFFSET, -20.0);
        camera.write_f32(NORMAL_FOV_OFFSET, 45.0);
        let ok = camera.store_parameters();
        assert_eq!(ok.position, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(
            (ok.yaw, ok.pitch, ok.vertical_fov_degrees),
            (90.0, -20.0, 45.0)
        );

        camera.write_f32(NORMAL_POSITION_OFFSET + 8, f32::INFINITY);
        camera.write_f32(NORMAL_YAW_OFFSET, f32::NAN);
        let bad = camera.store_parameters();
        assert_eq!(bad.position, Vec3::new(0.0, 5.0, 0.0));
        assert_eq!(bad.yaw, 0.0);
        assert_eq!(bad.pitch, -20.0);
    }

    #[test]
    fn record_layout_is_0x24_bytes_in_the_exe_order() {
        let record = CameraRigParameters {
            yaw: 1.0,
            pitch: 2.0,
            position: Vec3::new(3.0, 4.0, 5.0),
            camera_rig_arm_length: 0.0,
            vertical_fov_degrees: 6.0,
            pitch_offset: 0.0,
            yaw_offset: 0.0,
        };
        let bytes = record.to_bytes();
        let word = |i: usize| f32::from_le_bytes(bytes[i * 4..i * 4 + 4].try_into().unwrap());
        assert_eq!(bytes.len(), 0x24);
        assert_eq!(
            [word(0), word(1), word(2), word(3), word(4), word(6)],
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
        );
    }

    #[test]
    fn load_parameters_restores_the_orbit_rotation_and_active_rotation() {
        // O：0x140911520 写轨道四元数结果 +0x144/+0x148/+0x14c、+0x118=0、
        // push ActiveRotation 入 rotations、!ignore_snapshot 时 +0x194=pitch_offset；
        // **不直写** Normal body 位置/yaw/pitch。
        let params = crate::systems::camera_rig::CameraRigParametersCache {
            yaw: 90.0,
            pitch: -20.0,
            position: Vec3::new(1.0, 2.0, 3.0),
            camera_rig_arm_length: 4.0,
            vertical_fov_degrees: 45.0,
            pitch_offset: 0.5,
            yaw_offset: 0.0,
        };
        let mut camera = MainCamera::default();
        // 当前 Normal body 位置（+0x138/0x13c/0x140）、yaw(+0x154)/pitch(+0x150)。
        camera.write_f32(NORMAL_POSITION_OFFSET, 10.0);
        camera.write_f32(NORMAL_POSITION_OFFSET + 4, 20.0);
        camera.write_f32(NORMAL_POSITION_OFFSET + 8, 30.0);
        camera.write_f32(NORMAL_YAW_OFFSET, 1.0);
        camera.write_f32(NORMAL_PITCH_OFFSET, 2.0);
        camera.load_parameters(&params, true);
        // 模式字节 = Normal；位置/yaw/pitch **未被直写**。
        assert_eq!(camera.mode_byte(), CameraMode::NORMAL.0);
        assert_eq!(camera.read_f32(NORMAL_POSITION_OFFSET), 10.0);
        assert_eq!(camera.read_f32(NORMAL_YAW_OFFSET), 1.0);
        assert_eq!(camera.read_f32(NORMAL_PITCH_OFFSET), 2.0);
        // +0x118 = 0。
        assert_eq!(
            u64::from_le_bytes(camera.raw[0x118..0x120].try_into().unwrap()),
            0
        );
        // ActiveRotation 记录（28 B）：pitch/yaw Δ + 常量位。
        assert_eq!(camera.rotations.len(), 1);
        let r = camera.rotations[0];
        assert!((r.pitch_delta - (-22.0)).abs() < 1e-4, "{r:?}");
        assert!((r.yaw_delta - 89.0).abs() < 1e-4, "{r:?}");
        assert_eq!(r.pad, 0);
        assert_eq!(r.field_10, 0x80000000);
        assert_eq!(r.field_14, 0x80000000);
        assert_eq!(r.field_18, -1.0);
        // len/ptr/cap 镜像写 raw +0x100/+0x108/+0x110。
        assert_eq!(
            u64::from_le_bytes(
                camera.raw[ROTATIONS_LEN_OFFSET..ROTATIONS_LEN_OFFSET + 8]
                    .try_into()
                    .unwrap()
            ),
            1
        );
        assert_eq!(
            u64::from_le_bytes(
                camera.raw[ROTATIONS_VEC_OFFSET + 8..ROTATIONS_VEC_OFFSET + 16]
                    .try_into()
                    .unwrap()
            ),
            camera.rotations.as_ptr() as usize as u64
        );
        assert_eq!(
            u64::from_le_bytes(
                camera.raw[ROTATIONS_VEC_OFFSET..ROTATIONS_VEC_OFFSET + 8]
                    .try_into()
                    .unwrap()
            ),
            1
        );
        // ignore_snapshot=true 不写 +0x194。
        assert_eq!(camera.read_f32(0x194), 0.0);
        // ignore_snapshot=false 写 +0x194 = 原缓存+18 FOV，不是pitch_offset。
        camera.load_parameters(&params, false);
        assert_eq!(camera.read_f32(0x194), 45.0);
        // rotations Vec@+0x100 为**替换非 push**：二次 load 后 len 恒 1（机器码 dealloc 旧+新 alloc）。
        assert_eq!(camera.rotations.len(), 1);
    }

    #[test]
    fn load_parameters_orbits_with_the_machine_code_constants() {
        // O 0x140911520：`from_euler(YXZ, yaw*0x3c8efa35, pitch*0x3c8efa35, 0)`；
        // +0x144/+0x148/+0x14c = (params.position - camera.position) + q*(0,0,arm)。
        let params = crate::systems::camera_rig::CameraRigParametersCache {
            yaw: 0.0,
            pitch: 0.0,
            position: Vec3::new(1.0, 2.0, 3.0),
            camera_rig_arm_length: 0.0,
            vertical_fov_degrees: 45.0,
            pitch_offset: 0.0,
            yaw_offset: 0.0,
        };
        let mut camera = MainCamera::default();
        camera.write_f32(NORMAL_POSITION_OFFSET, 0.0);
        camera.write_f32(NORMAL_POSITION_OFFSET + 4, 0.0);
        camera.write_f32(NORMAL_POSITION_OFFSET + 8, 0.0);
        camera.load_parameters(&params, true);
        // yaw=pitch=0 → 单位四元数：rotated=(0,0,0)，+0x144/+0x148/+0x14c = params.position。
        assert!((camera.read_f32(0x144) - 1.0).abs() < 1e-5);
        assert!((camera.read_f32(0x148) - 2.0).abs() < 1e-5);
        assert!((camera.read_f32(0x14c) - 3.0).abs() < 1e-5);
    }

    #[test]
    fn forwarders_read_the_maintained_camera_matrices() {
        let mut camera = MainCamera::default();
        camera.write_f32(NORMAL_POSITION_OFFSET, 3.0);
        camera.write_f32(NORMAL_POSITION_OFFSET + 4, 4.0);
        camera.write_f32(NORMAL_POSITION_OFFSET + 8, 5.0);
        camera.write_f32(NORMAL_YAW_OFFSET, 0.0);
        camera.write_f32(NORMAL_PITCH_OFFSET, 0.0);
        camera.rebuild_camera_matrices_from_normal_body();
        assert_eq!(camera.eye_position(), Vec3::new(3.0, 4.0, 5.0));
        // yaw=0 pitch=0 → 朝 -Z。
        assert!((camera.eye_forward() - Vec3::new(0.0, 0.0, -1.0)).length() < 1e-5);
        let _ = camera.xz_projective_rotation();
        let _ = camera.reset();
        assert_eq!(camera.mode_byte(), CameraMode::NORMAL.0);
        // reset 重建维护矩阵（Normal body 位置 (0,5,0)）→ eye_position 回到 (0,5,0)。
        assert_eq!(camera.eye_position(), Vec3::ZERO);
    }
}

impl MainCamera {
    pub(crate) fn set_walk_modifier(&mut self, enabled: bool) {
        self.raw[WALK_MODIFIER_OFFSET] = u8::from(enabled);
    }
    pub(crate) fn begin_selected_rotation(&mut self, anchor: Vec3, direction: Vec3, eye: Vec3) {
        let mode = CameraMode(self.mode_byte());
        if matches!(mode.0, 0 | 1 | 4) {
            self.with_normal_body(mode, |body| body.begin_rotation(anchor, direction, eye));
        }
    }
    pub(crate) fn finish_selected_rotation(&mut self) {
        let mode = CameraMode(self.mode_byte());
        // Original release excludes all modes2..5, including TopDown4.
        if matches!(mode.0, 0 | 1) {
            self.with_normal_body(mode, |body| body.finish_rotation());
        }
    }
    pub(crate) fn rotate_selected(&mut self, yaw: f32, pitch: f32) {
        match self.mode_byte() {
            0 | 1 | 4 => {
                let mode = CameraMode(self.mode_byte());
                self.with_normal_body(mode, |body| body.rotate_yaw_pitch(yaw, pitch));
            }
            2 | 3 => {
                let owner = self
                    .startup_rigs
                    .as_ref()
                    .expect("MainCamera's real Dolly rigs are absent")
                    .clone();
                let mut rigs = owner.lock().expect("camera rig owner poisoned");
                let rig = if self.mode_byte() == 2 {
                    &mut rigs.free
                } else {
                    &mut rigs.walk
                };
                super::free_camera::rotate_rig_yaw_pitch(rig, yaw, pitch);
            }
            5 => {}
            _ => panic!("invalid camera mode"),
        }
    }
    #[cfg(feature = "native-physics")]
    pub(crate) fn translate_selected(
        &mut self,
        movement: Vec3,
        dt: f32,
        heights: &crate::resources::terrain_height::TerrainHeightsData,
        world: &crate::systems::collision::world::RaycastWorld,
        demo: bool,
    ) {
        let mode = CameraMode(self.mode_byte());
        match mode.0 {
            0 | 1 | 4 => {
                let matrices = self.camera_matrices();
                self.with_normal_body(mode, |body| body.translate_screen_space(movement, matrices));
            }
            2 | 3 => {
                let owner = self
                    .startup_rigs
                    .as_ref()
                    .expect("MainCamera's real Dolly rigs are absent")
                    .clone();
                let mut rigs = owner.lock().expect("camera rig owner poisoned");
                if mode.0 == 2 {
                    super::free_camera::translate_rig_screen_space(
                        &mut rigs.free,
                        movement,
                        dt,
                        heights,
                    );
                } else {
                    self.write_f32(0x340, dt);
                    super::walk::translate_rig_screen_space(
                        &mut rigs.walk,
                        movement,
                        dt,
                        self.raw[0x344] != 0,
                        world,
                        heights,
                        demo,
                    );
                }
            }
            5 => {}
            _ => panic!("invalid camera mode"),
        }
    }
    #[cfg(feature = "native-physics")]
    pub(crate) fn constrain_selected(
        &mut self,
        heights: &crate::resources::terrain_height::TerrainHeightsData,
        world: &crate::systems::collision::world::RaycastWorld,
        path: &crate::resources::world_raster_defs::PathRaster,
        winter: bool,
    ) {
        let mode = CameraMode(self.mode_byte());
        match mode.0 {
            0 | 1 | 4 => self.with_normal_body(mode, |body| {
                body.constrain_to_be_above_terrain(|x, z| heights.sample_at_world_xz(x, z))
            }),
            2 | 5 => {}
            3 => {
                let owner = self
                    .startup_rigs
                    .as_ref()
                    .expect("MainCamera's real Dolly rigs are absent")
                    .clone();
                let mut rigs = owner.lock().expect("camera rig owner poisoned");
                let source = super::walk::constrain_rig_to_be_above_terrain(
                    &mut rigs.walk,
                    self.read_f32(0x340),
                    self.raw[0x344] != 0,
                    world,
                    heights,
                    path,
                    winter,
                );
                match source {
                    super::walk::GroundSourceUpdate::NoHit => self.raw[0x328] = 0x11,
                    super::walk::GroundSourceUpdate::Terrain => self.raw[0x328] = 0x0b,
                    super::walk::GroundSourceUpdate::Collider(words) => {
                        for (i, word) in words.into_iter().enumerate() {
                            self.write_u64(0x328 + i * 8, word);
                        }
                    }
                }
            }
            _ => panic!("invalid camera mode"),
        }
    }
    pub(crate) fn butterfly_selected_index(&self) -> usize {
        u32::from_le_bytes(self.raw[0x484..0x488].try_into().unwrap()) as usize
    }
    pub(crate) fn advance_butterfly(&mut self, zoom: f32) {
        // Original signum→saturating f32/i32 cast→wrapping subtraction/max0.
        let sign = if zoom > 0.0 { 1 } else { -1 };
        let selected = self.butterfly_selected_index() as i32;
        let selected = selected.wrapping_sub(sign).max(0) as u32;
        self.raw[0x484..0x488].copy_from_slice(&selected.to_le_bytes());
    }
    pub(crate) fn follow_butterfly(
        &mut self,
        transform: &crate::utils::components::transform::Transform,
    ) {
        use dolly::drivers::{LookAt, Position, Rotation};
        let owner = self
            .startup_rigs
            .as_ref()
            .expect("MainCamera's real Dolly rigs are absent")
            .clone();
        let mut rigs = owner.lock().expect("camera rig owner poisoned");
        let p = transform.translation;
        let q = transform.rotation;
        rigs.butterfly.driver_mut::<Position>().position = [p.x, p.y, p.z].into();
        rigs.butterfly.driver_mut::<Rotation>().rotation = [q.x, q.y, q.z, q.w].into();
        let target = p + Vec3::Y;
        rigs.butterfly.driver_mut::<LookAt>().target = [target.x, target.y, target.z].into();
        self.set_mode_byte(CameraMode::BUTTERFLY);
    }
}

impl MainCamera {
    pub(crate) fn zoom_selected(&mut self, amount: f32, anchor: Vec3) {
        let mode = CameraMode(self.mode_byte());
        if let Some(body) = super::normal_body::ZoomBody::for_mode(mode) {
            self.with_normal_body(mode, |normal| normal.zoom(body, amount, anchor))
                .unwrap_or_else(|error| panic!("{error}"));
        }
    }
}
