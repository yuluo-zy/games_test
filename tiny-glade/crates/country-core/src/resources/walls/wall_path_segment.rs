//! 墙路径段（walls/wall_path_segment.rs）。
//!
//! 原 B `resources/walls/wall_path_segment.rs`（PDB 实体 `WallPathSegment`；
//! segmentation 族的预备段：点列 + 长度；`system_wall_constructor` 消费）。
use crate::Gap;

/// 墙路径段（布局 U；方法面 O）。
#[derive(Clone, Debug, Default)]
pub struct WallPathSegment {
    /// 预备点/参数数据（typed gap）。
    pub gap: Gap,
}

impl WallPathSegment {
    /// O：预备路径点计数（0x1401234970 链）。
    pub fn pt_count(&self) -> usize {
        0
    }
    /// O：3D 世界空间长度（0x1401234980 链）。
    pub fn length_ws(&self) -> f32 {
        0.0
    }
    /// O：2D 长度。
    pub fn length_ws_2d(&self) -> f32 {
        0.0
    }
}
