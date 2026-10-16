//! Only the closed camera-schema leaf is included. Camera mode controllers,
//! projection adapters and the incomplete standalone camera package are excluded.
//! `main_camera` / `free_camera` hold the pieces of `MainCamera` verified against the
//! B EXE (see `R9_camera.md`); the per-mode bodies and `main_camera_update` are still open.

pub mod free_camera;
pub mod main_camera;
pub mod normal_body;

/// B `CameraRigParametersCache`（0x24B=36B；6 字段存档 schema，re-analyst 域类型证据包）。
/// 存档 Camera 节载荷 = `to_json_string<CameraRigParametersCache>` 0x1416f8470：
/// yaw/pitch/position(Vec3)/camera_rig_arm_length/pitch_offset/yaw_offset。
/// 旧 7 字段表（含 position_cache）修正：save 只写 6 字段；position_cache 仅反序列化接受。
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CameraRigParametersCache {
    pub yaw: f32,
    pub pitch: f32,
    #[serde(default)]
    pub position: glam::Vec3,
    pub camera_rig_arm_length: f32,
    pub pitch_offset: f32,
    pub yaw_offset: f32,
}
impl Default for CameraRigParametersCache {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            position: glam::Vec3::new(0.0, 5.0, 0.0),
            camera_rig_arm_length: 0.0,
            pitch_offset: 0.0,
            yaw_offset: 0.0,
        }
    }
}

/// B `default_vertical_fov_degrees` 0x140911e30 (9 bytes), returned f32 = 45.
/// Source: reconstruction/camera/schema.rs, Git 8107b475;
/// evidence/parallel-camera/B-decompile/140911e30.c.
pub fn default_vertical_fov_degrees() -> f32 {
    45.0
}

/// `CameraModeStates`（O：0x1409149d0 模式@+0x3d0 jumptable → {body, 表, (fov_current,
/// fov_target) 对}；Normal/TiltShift/Free/Walk/TopDown/Butterfly 各一 case + 模式 6 =
/// body@+0x0 矩阵块区）。crate 内 MainCamera 为不透明 0x4e0 B，六 body 区
/// @+0x100/0x1b8/0x270/0x2f0/0x390/0x448。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CameraModeStates {
    /// 模式字节（@+0x3d0）。
    pub mode: u8,
}

impl CameraModeStates {
    /// 模式字节偏移（O：@+0x3d0）。
    pub const MODE_OFFSET: usize = 0x3d0;
    /// 六 body 区偏移（O：章节目 @+0x100/0x1b8/0x270/0x2f0/0x390/0x448）。
    pub const BODY_OFFSETS: [usize; 6] = [0x100, 0x1b8, 0x270, 0x2f0, 0x390, 0x448];

    /// `current_dyn`（0x1409149d0）：模式 → `(body 区偏移, fov 槽偏移)`。
    pub fn current_dyn(&self) -> (usize, usize) {
        let body = match self.mode {
            0..=5 => Self::BODY_OFFSETS[self.mode as usize],
            // 模式 6：body = +0x0 矩阵块区。
            _ => 0,
        };
        let fov = main_camera::FOV_OFFSETS
            .get(self.mode as usize)
            .copied()
            .unwrap_or(0);
        (body, fov)
    }

    /// `current_body_mut`（行 167）：按模式取 body 区偏移（可变引用并入调用点）。
    pub fn current_body_offset(&self) -> usize {
        self.current_dyn().0
    }

    /// `current_dyn_mut`（行 155）：`current_dyn` 可变版（并入调用点）。
    pub fn current_dyn_mut(&mut self) -> (usize, usize) {
        self.current_dyn()
    }
}

/// `CameraState`（行 105，内联：相机运行时状态默认，并入 MainCamera::new）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CameraState {
    /// 快照字节（@+0x134，immediately_finish_interpolating 双 jumptable 之一，I）。
    pub snapshot_byte: u8,
    /// fov 系数（@+0xec，immediately_finish_interpolating `×0.1`，I）。
    pub fov_coefficient: f32,
}
