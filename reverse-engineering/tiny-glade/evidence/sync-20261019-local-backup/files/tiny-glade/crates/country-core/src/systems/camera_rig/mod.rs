//! Only the closed camera-schema leaf is included. Camera mode controllers,
//! projection adapters and the incomplete standalone camera package are excluded.
//! `main_camera` / `free_camera` hold the pieces of `MainCamera` verified against the
//! B EXE (see `R9_camera.md`); the per-mode bodies and `main_camera_update` are still open.

pub mod butterfly;
pub mod free_camera;
pub mod main_camera;
pub mod normal_body;
pub mod walk;

/// B `CameraRigParametersCache`（0x24B=36B；6 字段存档 schema，re-analyst 域类型证据包）。
/// 存档 Camera 节载荷 = `to_json_string<CameraRigParametersCache>` 0x1416f8470：
/// yaw/pitch/position(Vec3)/camera_rig_arm_length/pitch_offset/yaw_offset。
/// 旧 7 字段表（含 position_cache）修正：save 只写 6 字段；position_cache 仅反序列化接受。
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CameraRigParametersCache {
    #[serde(alias = "yaw_cache")]
    pub yaw: f32,
    #[serde(alias = "pitch_cache")]
    pub pitch: f32,
    #[serde(default, alias = "position_cache")]
    pub position: glam::Vec3,
    pub camera_rig_arm_length: f32,
    /// Native +18. Original object visitor @1415c43f9 writes45 unconditionally
    /// (Result payload starts+4), and serializer1416f8470 skips this slot.
    #[serde(skip, default = "default_vertical_fov_degrees")]
    pub vertical_fov_degrees: f32,
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
            vertical_fov_degrees: default_vertical_fov_degrees(),
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

/// `CameraModeStates`（O：0x1409149d0 模式@+0x3d0 jumptable）。
///
/// fresh 反编译（2026-10-17）：switch 六 case，每 case 写 4 元组
/// `{dyn_body_ptr, vtable, fov_abs, body_abs}` 到 `*out`：
///
/// | mode | dyn_body_ptr | vtable | fov_abs | body_abs |
/// |---|---|---|---|---|
/// | 0 Normal | +0x0（矩阵块区/自身） | anon_…_19 | +0x8c | +0x70 |
/// | 1 TiltShift | +0xb8 | anon_…_19 | +0x144 | +0x128 |
/// | 2 Free | +0x170 | anon_…_17 | +0x1c4 | +0x1a8 |
/// | 3 Walk | +0x1f0 | anon_…_16 | +0x264 | +0x248 |
/// | 4 TopDown | +0x290 | anon_…_19 | +0x31c | +0x300 |
/// | 5 Butterfly | +0x348 | anon_…_18 | +0x3a4 | +0x388 |
///
/// mode ≥ 6 无 case（输出未初始化；crate 内按 FOV_OFFSETS 兜底 fov、其余 0）。
/// `fov_abs` 与 [`main_camera::FOV_OFFSETS`] 前 6 项逐项一致。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CameraModeStates {
    /// 模式字节（@+0x3d0）。
    pub mode: u8,
}

/// 各 body 的 dyn vtable（LLVM anon 表；Ghidra 名 `anon_0555c73667d6be1dc866b3f0ae0b3f3f_<N>_llvm_…`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraBodyVtable {
    /// anon_…_19（Normal/TiltShift/TopDown）。
    A19,
    /// anon_…_17（Free）。
    A17,
    /// anon_…_16（Walk）。
    A16,
    /// anon_…_18（Butterfly）。
    A18,
}

impl CameraModeStates {
    /// 模式字节偏移（O：@+0x3d0）。
    pub const MODE_OFFSET: usize = 0x3d0;
    /// 六 body_abs（O 0x1409149d0：+0x70/0x128/0x1a8/0x248/0x300/0x388）。
    pub const BODY_OFFSETS: [usize; 6] = [0x70, 0x128, 0x1a8, 0x248, 0x300, 0x388];
    /// 六 fov_abs（O 0x1409149d0：+0x8c/0x144/0x1c4/0x264/0x31c/0x3a4）。
    pub const FOV_ABS: [usize; 6] = [0x8c, 0x144, 0x1c4, 0x264, 0x31c, 0x3a4];
    /// 六 dyn_body_ptr（O 0x1409149d0：+0x0/+0xb8/+0x170/+0x1f0/+0x290/+0x348）。
    pub const DYN_BODY_PTRS: [usize; 6] = [0x0, 0xb8, 0x170, 0x1f0, 0x290, 0x348];

    /// `current_dyn`（0x1409149d0）：模式 → `{dyn_body_ptr, vtable, fov_abs, body_abs}`。
    pub fn current_dyn(&self) -> CurrentDyn {
        match self.mode {
            0 => CurrentDyn::new(0x0, CameraBodyVtable::A19, 0x8c, 0x70),
            1 => CurrentDyn::new(0xb8, CameraBodyVtable::A19, 0x144, 0x128),
            2 => CurrentDyn::new(0x170, CameraBodyVtable::A17, 0x1c4, 0x1a8),
            3 => CurrentDyn::new(0x1f0, CameraBodyVtable::A16, 0x264, 0x248),
            4 => CurrentDyn::new(0x290, CameraBodyVtable::A19, 0x31c, 0x300),
            5 => CurrentDyn::new(0x348, CameraBodyVtable::A18, 0x3a4, 0x388),
            // mode ≥ 6：反编译无 case（输出保持初始/未定义）；fov 仍由 13 项表兜底。
            _ => CurrentDyn {
                dyn_body_ptr: 0,
                vtable: CameraBodyVtable::A19,
                fov_abs: main_camera::FOV_OFFSETS
                    .get(self.mode as usize)
                    .copied()
                    .unwrap_or(0),
                body_abs: 0,
            },
        }
    }

    /// `current_body_mut`（行 167）：按模式取 body_abs（可变引用并入调用点）。
    pub fn current_body_offset(&self) -> usize {
        self.current_dyn().body_abs
    }

    /// `current_dyn_mut`（行 155）：`current_dyn` 可变版（并入调用点）。
    pub fn current_dyn_mut(&mut self) -> CurrentDyn {
        self.current_dyn()
    }
}

/// `current_dyn` 的 4 元输出（O 0x1409149d0：`*out = lVar2; out[1] = ppuVar1; out[2] = fov_abs;
/// out[3] = body_abs`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CurrentDyn {
    /// dyn body 指针偏移（mode 0 = +0x0 矩阵块区）。
    pub dyn_body_ptr: usize,
    /// body 的 vtable（LLVM anon 表）。
    pub vtable: CameraBodyVtable,
    /// fov 槽绝对偏移。
    pub fov_abs: usize,
    /// body 绝对偏移。
    pub body_abs: usize,
}

impl CurrentDyn {
    pub const fn new(
        dyn_body_ptr: usize,
        vtable: CameraBodyVtable,
        fov_abs: usize,
        body_abs: usize,
    ) -> Self {
        Self {
            dyn_body_ptr,
            vtable,
            fov_abs,
            body_abs,
        }
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

#[cfg(test)]
mod legacy_asset_tests {
    #[test]
    fn original_intro_camera_aliases_preserve_pose_and_canonical_serialization() {
        let text = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/original-intro-camera-b.json"
        ));
        let cache: super::CameraRigParametersCache = serde_json::from_str(text).unwrap();
        assert_eq!(core::mem::size_of_val(&cache), 0x24);
        assert_eq!(cache.vertical_fov_degrees, 45.0);
        assert_eq!(cache.yaw, -56.79069137573242_f32);
        assert_eq!(cache.pitch, -32.4960823059082_f32);
        assert_eq!(
            cache.position.to_array(),
            [
                -21.3394775390625_f32,
                19.812301635742188_f32,
                11.120874404907227_f32
            ]
        );
        let value = serde_json::to_value(cache).unwrap();
        assert!(
            value.get("yaw").is_some()
                && value.get("pitch").is_some()
                && value.get("position").is_some()
        );
        assert!(
            value.get("yaw_cache").is_none()
                && value.get("pitch_cache").is_none()
                && value.get("position_cache").is_none()
        );
    }
}
