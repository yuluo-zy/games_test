//! `systems::camera_rig::main_camera` — `MainCamera`（0x4e0 B 资源）的已核对部分。
//!
//! 全部取自 B 构建 EXE：
//!
//! | 项 | 地址 | 级别 |
//! |---|---|---|
//! | 模式字节 `+0x4d0`、宽高比 `+0x4d8`、鼠标深度字节 `+0x344` | `main_camera_update` 0x140912c20 | O |
//! | `CameraMode::previous` | 0x140914ac0 | O：`movabs rax, 0x40302010006; shr rax, cl`（`cl = mode*8`） |
//! | fov 偏移表（13 项） | 0x142aabb98 | O：逐项实读 |
//! | `target_vertical_fov_degrees` | 0x1409114d0 | O：`[self + table[mode] + 0x108]` |
//! | `immediately_set_vertical_fov_degrees` | 0x1409114f0 → 0x140c974f0 | O：`[self + table[mode] + 0x100 + 8] = [… + 4] = fov` |
//! | `store_parameters` | 0x140911710 | O |
//!
//! 未做：`MainCamera::new`（0x14090ec70，853 行反编译，六个 body 的默认值）、`main_camera_update` 的按模式
//! 跳表（0x142aabd04 / 0x142aabd30，各模式分支的反编译有损坏）、`immediately_finish_interpolating`、
//! `paused_main_camera_update`。所以 [`MainCamera`] 只是 0x4e0 字节原始缓冲加上已核对偏移的读写器。

use bevy_ecs::resource::Resource;
use glam::Vec3;

pub const MAIN_CAMERA_SIZE: usize = 0x4e0;
pub const MODE_OFFSET: usize = 0x4d0;
pub const ASPECT_OFFSET: usize = 0x4d8;
pub const MOUSE_DEPTH_OFFSET: usize = 0x344;

/// Normal body 内：位置（+0x138）、pitch（+0x150）、yaw（+0x154）、垂直 fov（+0x190）。
/// O：`store_parameters` 读这四处；`match_free_camera_to_normal`（0x14090e6f0）把 `[+0x154]` 作 yaw、
/// `[+0x150]` 作 pitch 喂给 dolly 的 `YawPitch`。
pub const NORMAL_POSITION_OFFSET: usize = 0x138;
pub const NORMAL_PITCH_OFFSET: usize = 0x150;
pub const NORMAL_YAW_OFFSET: usize = 0x154;
pub const NORMAL_FOV_OFFSET: usize = 0x190;

/// fov 偏移表（O 0x142aabb98，13 个 u64，相对 `MainCamera + 0x100` 的 body 内 fov 槽）。
pub const FOV_OFFSETS: [usize; 13] = [
    0x8c, 0x144, 0x1c4, 0x264, 0x31c, 0x3a4, 0x8, 0x20, 0x14, 0x2c, 0x8, 0x14, 0xc,
];

/// 模式字节（O：0..=12，共 13 个槽）。0..=5 是六个 body；6..=12 复用 body 的 fov 槽（名字 U）。
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
        (byte <= 12).then_some(Self(byte))
    }

    /// O 0x140914ac0：`(0x40302010006 >> (mode * 8)) as u8`，x86 的 64 位移位按低 6 位取模，
    /// 所以 `mode >= 8` 回绕到 `mode & 7`。
    pub fn previous(self) -> Self {
        const TABLE: u64 = 0x0000_0403_0201_0006;
        let shift = u32::from(self.0.wrapping_mul(8)) & 63;
        Self((TABLE >> shift) as u8)
    }
}

/// 0x24 B 的相机参数缓存记录（O：`store_parameters` 的输出布局）。
#[derive(Clone, Copy, Debug, PartialEq)]
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

/// `MainCamera` 的原始 0x4e0 字节。原版为 ECS 资源（`input_record_replay`/`control_input_replay`
/// 0x1409423d0/0x140941a30 以 `Res<MainCamera>` 调 `store_parameters`）。
#[derive(Resource, Clone)]
pub struct MainCamera {
    raw: Box<[u8; MAIN_CAMERA_SIZE]>,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            raw: Box::new([0; MAIN_CAMERA_SIZE]),
        }
    }
}

impl MainCamera {
    pub fn bytes(&self) -> &[u8; MAIN_CAMERA_SIZE] {
        &self.raw
    }

    pub fn read_f32(&self, offset: usize) -> f32 {
        f32::from_le_bytes(self.raw[offset..offset + 4].try_into().unwrap())
    }

    pub fn write_f32(&mut self, offset: usize, value: f32) {
        self.raw[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
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

    /// O 0x1409114d0。模式字节 > 12 时越界 panic（原版表只有 13 项）。
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

    /// O 0x140911520：从会话相机记录（`CameraRigParametersCache` 6 字段，无 fov）恢复。
    /// 章节目：mode=0（Normal）、轨道四元数（π/180）、+0x118=0、
    /// push 28 B ActiveRotation 入 rotations Vec@+0x100、flag 时 +0x194 = 记录分量。
    /// crate 内 MainCamera 为不透明字节缓冲：恢复 Normal body 的位置/yaw/pitch + 模式字节=
    /// Normal；fov 用默认 45（fov 不在 6 字段存档内，`serde_default_vertical_fov_degrees`）；
    /// yaw/pitch 直写（与 `store_parameters` 捕获同单位，保证存档往返）；π/180 轨道四元数
    /// 与 ActiveRotation 队列（rotations Vec@+0x100）依赖 body 子结构，待取证。
    pub fn load_parameters(&mut self, params: &crate::systems::camera_rig::CameraRigParametersCache) {
        self.set_mode_byte(CameraMode::NORMAL);
        self.write_f32(NORMAL_YAW_OFFSET, params.yaw);
        self.write_f32(NORMAL_PITCH_OFFSET, params.pitch);
        self.write_f32(NORMAL_POSITION_OFFSET, params.position.x);
        self.write_f32(NORMAL_POSITION_OFFSET + 4, params.position.y);
        self.write_f32(NORMAL_POSITION_OFFSET + 8, params.position.z);
        self.write_f32(NORMAL_FOV_OFFSET, 45.0);
    }

    /// O 0x140911dd0（12 B 转发器）：`eye_position` → `utils::camera` 位置提取。
    /// crate 内 MainCamera 为不透明缓冲，从 Normal body 位置构造 [`CameraMatrices`]
    /// （非 Normal body 的位置读取待取证）。
    pub fn eye_position(&self) -> Vec3 {
        let position = Vec3::new(
            self.read_f32(NORMAL_POSITION_OFFSET),
            self.read_f32(NORMAL_POSITION_OFFSET + 4),
            self.read_f32(NORMAL_POSITION_OFFSET + 8),
        );
        position
    }

    /// O 0x140911df0（12 B 转发器）：`eye_forward`（`eye_direction` 同址）→
    /// `utils::camera` 前向提取。由 Normal body 的 yaw/pitch 组方向。
    pub fn eye_forward(&self) -> Vec3 {
        self.normal_matrices().eye_direction()
    }

    /// O 0x140911e10（12 B 转发器）：`xz_projective_rotation` → `utils::camera`
    /// XZ 投影旋转。
    pub fn xz_projective_rotation(&self) -> glam::Quat {
        self.normal_matrices().xz_projective_rotation()
    }

    /// 从 Normal body 的位置/yaw/pitch 构造共享 [`utils::camera::CameraMatrices`]
    /// （转发器辅助，I）。`xz_projective_rotation` 只依赖 `inverse_view`，projection 用
    /// 单位矩阵占位（不参与该旋转）。
    fn normal_matrices(&self) -> utils::camera::CameraMatrices {
        let position = self.eye_position();
        let yaw = self.read_f32(NORMAL_YAW_OFFSET);
        let pitch = self.read_f32(NORMAL_PITCH_OFFSET);
        let rotation = glam::Quat::from_rotation_y(yaw) * glam::Quat::from_rotation_x(pitch);
        let body = utils::camera::CameraBodyMatrices::from_position_rotation(position, rotation);
        utils::camera::CameraMatrices {
            projection: glam::Mat4::IDENTITY,
            inverse_view: body.inverse_view,
        }
    }

    /// `MainCamera::reset`（行 428，内联，I：重建默认装配；系统层 reset_camera 调用）。
    /// crate 内重建 = 清缓冲 + 模式=Normal + fov=默认 45。
    pub fn reset(&mut self) {
        self.raw = Box::new([0; MAIN_CAMERA_SIZE]);
        self.set_mode_byte(CameraMode::NORMAL);
        self.write_f32(NORMAL_FOV_OFFSET, 45.0);
        self.write_f32(NORMAL_POSITION_OFFSET + 4, 5.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previous_follows_the_packed_table_including_the_wraparound() {
        let table = [6, 0, 1, 2, 3, 4, 0, 0, 6, 0, 1, 2, 3];
        for (mode, expected) in table.into_iter().enumerate() {
            assert_eq!(
                CameraMode(mode as u8).previous(),
                CameraMode(expected),
                "mode {mode}"
            );
        }
        assert_eq!(CameraMode::NORMAL.previous(), CameraMode(6));
    }

    #[test]
    fn mode_range_is_thirteen_slots() {
        assert!(CameraMode::new(12).is_some());
        assert!(CameraMode::new(13).is_none());
        assert_eq!(FOV_OFFSETS.len(), 13);
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
        camera.set_mode_byte(CameraMode(10));
        assert_eq!(camera.target_vertical_fov_degrees(), 0.0);
        camera.immediately_set_vertical_fov_degrees(33.0);
        assert_eq!(camera.read_f32(0x100 + 0x8 + 8), 33.0);
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
    fn load_parameters_restores_normal_body_and_round_trips() {
        // O：0x140911520 恢复 Normal body + 模式字节；yaw/pitch 直写（与 store_parameters
        // 捕获同单位）；fov 用默认 45（不在 6 字段存档）。
        let params = crate::systems::camera_rig::CameraRigParametersCache {
            yaw: 90.0,
            pitch: -20.0,
            position: Vec3::new(1.0, 2.0, 3.0),
            camera_rig_arm_length: 0.0,
            pitch_offset: 0.0,
            yaw_offset: 0.0,
        };
        let mut camera = MainCamera::default();
        camera.load_parameters(&params);
        assert_eq!(camera.mode_byte(), CameraMode::NORMAL.0);
        assert_eq!(camera.read_f32(NORMAL_YAW_OFFSET), 90.0);
        assert_eq!(camera.read_f32(NORMAL_PITCH_OFFSET), -20.0);
        assert_eq!(camera.read_f32(NORMAL_POSITION_OFFSET), 1.0);
        assert_eq!(camera.read_f32(NORMAL_POSITION_OFFSET + 4), 2.0);
        assert_eq!(camera.read_f32(NORMAL_POSITION_OFFSET + 8), 3.0);
        assert_eq!(camera.read_f32(NORMAL_FOV_OFFSET), 45.0);
        // store → load 往返保持（fov 不存 6 字段存档，回 45）。
        let stored = camera.store_parameters();
        assert_eq!(stored.yaw, 90.0);
        assert_eq!(stored.pitch, -20.0);
        assert_eq!(stored.position, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn forwarders_read_normal_body_position_and_direction() {
        let mut camera = MainCamera::default();
        camera.write_f32(NORMAL_POSITION_OFFSET, 3.0);
        camera.write_f32(NORMAL_POSITION_OFFSET + 4, 4.0);
        camera.write_f32(NORMAL_POSITION_OFFSET + 8, 5.0);
        camera.write_f32(NORMAL_YAW_OFFSET, 0.0);
        camera.write_f32(NORMAL_PITCH_OFFSET, 0.0);
        assert_eq!(camera.eye_position(), Vec3::new(3.0, 4.0, 5.0));
        // yaw=0 pitch=0 → 朝 -Z。
        assert!((camera.eye_forward() - Vec3::new(0.0, 0.0, -1.0)).length() < 1e-5);
        let _ = camera.xz_projective_rotation();
        let _ = camera.reset();
        assert_eq!(camera.mode_byte(), CameraMode::NORMAL.0);
        assert_eq!(camera.eye_position(), Vec3::new(0.0, 5.0, 0.0));
    }
}
