//! terrain_stroke.rs — 地形笔画（WP8 恢复）。
//!
//! 源：B skeleton `resources/terrain_stroke.rs`。方法行号：`new`@23、`insert_or_morph_last_point`@42。
//! 反编译：`insert_or_morph_last_point`@0x140a97880（行 42）：
//!   len==0→push；len==1 且 dist(p,last)>0.001→push；len≥2 且 dist(last,prev)≥spacing→push（len<0x400=1024），
//!   否则覆盖 last（morph）。prior `tg-terrain::stroke.rs` 逐条恢复（O 原机器码案例）。
//! serde：CompactFormatter/PrettyFormatter/Value（O 名）。

#![allow(dead_code, unused_imports)]

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// 笔画最大点数（O：`insert_or_morph_last_point` 中 `uVar2 < 0x400`）。
pub const MAX_POINTS: usize = 1024;

/// 单点距离阈值（O：len==1 分支 `0.001 < dist`）。
pub const MIN_POINT_SPACING: f32 = 0.001;

/// 地形笔画（O 名；`TerrainStroke`）。布局 U → typed gap；`points: Vec<Vec3>` 为强观察（I）。
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TerrainStroke {
    /// 笔画点列（I；插入规则 O）。
    pub points: Vec<Vec3>,
    /// typed gap：{falloff, 笔画 id, height_ws, …} 字段 U。
    pub falloff: f32,
}

impl Default for TerrainStroke {
    fn default() -> Self {
        Self { points: Vec::new(), falloff: 0.4 }
    }
}

/// 插入结果（O/I：Added / Morphed / Unchanged）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointChange {
    Added,
    Morphed,
    Unchanged,
}

impl TerrainStroke {
    /// 新建（O 名；`new`@23）。
    pub fn new() -> Self {
        Self::default()
    }

    /// 插入或形变末点（O：`insert_or_morph_last_point`@0x140a97880）。
    pub fn insert_or_morph_last_point(&mut self, point: Vec3, spacing: f32) -> PointChange {
        match self.points.len() {
            0 => {
                self.points.push(point);
                PointChange::Added
            }
            1 => {
                if self.points[0].distance(point) > MIN_POINT_SPACING {
                    self.points.push(point);
                    PointChange::Added
                } else {
                    PointChange::Unchanged
                }
            }
            n => {
                let (last, prev) = (self.points[n - 1], self.points[n - 2]);
                if spacing <= last.distance(prev) {
                    if n < MAX_POINTS {
                        self.points.push(point);
                        PointChange::Added
                    } else {
                        PointChange::Unchanged
                    }
                } else {
                    self.points[n - 1] = point;
                    PointChange::Morphed
                }
            }
        }
    }
}
