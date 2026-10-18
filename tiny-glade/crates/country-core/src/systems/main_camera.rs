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
