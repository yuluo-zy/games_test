//! wall_path_segment.rs — 墙路径段（WP11 形状/轮廓几何族）。
//!
//! 源骨架 `resources/walls/wall_path_segment.rs`（B）；PDB 命名实体 `WallPathSegment`。
//! 语义 I：墙段切分中间态（分段/轮廓预备产物），含预备点数组 + 长度。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// 墙路径段（布局 U；方法面 O）。
#[derive(Clone, Debug, Default)]
pub struct WallPathSegment {
    /// 预备点/参数数据（typed gap）。
    pub gap: Gap,
}

impl WallPathSegment {
    /// O：0x1401234970 预备路径点计数。
    pub fn pt_count(&self) -> usize {
        // typed gap：点数组长度未接线。
        0
    }
    /// O：0x1401234980 3D 世界空间长度。
    pub fn length_ws(&self) -> f32 {
        // typed gap：长度值未接线。
        0.0
    }
    /// O：0x1401234ad0 2D 长度。
    pub fn length_ws_2d(&self) -> f32 {
        // typed gap。
        0.0
    }
}
