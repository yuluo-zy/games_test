//! `resources::history::path_edit` — Path / Cobblestone 光栅编辑记录（WP17）。
//!
//! O（观察 JSON schema：Path 234 例，字段 brush_size/is_additive/points 全有，
//! 228 例含 blend_mode；Cobblestone1..3 同构。+ `impl$1::from` 0x140a976b0）：
//!   - `RasterEdit`：笔刷光栅编辑负载（ReplayCmd 判别 0/1/2 的 Arc 负载）。
//!   - 点集为 3D Vec3（观察 1,576/1,688 个曲线对象 3D）。
//!   - serde 手写（serialize 0x16e8340 / deserialize 0x16e8510 + visitor 族）。

use crate::typed_gap::Gap;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// 笔刷混合模式（O：观察 string tag 如 "Max"；封闭枚举 U）。
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlendMode {
    #[default]
    Max,
    Add,
    Subtract,
    /// 未证模式（开放；typed gap 用字符串保真）。
    Other(String),
}

/// 光栅编辑（O：Path/Cobblestone 命令负载；ReplayCmd 判别 0/1/2）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RasterEdit {
    /// O：`points`（Vec3 点；观察 3D/2D 混合，2D 样本 U 轴约定）。
    #[serde(default)]
    pub points: Vec<Vec3>,
    /// O：`brush_size`（f32，观察 0.8 等）。
    #[serde(default)]
    pub brush_size: f32,
    /// O：`is_additive`（bool）。
    #[serde(default)]
    pub is_additive: bool,
    /// O：`blend_mode`（可选；228/234）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    /// U：原始曲线/过滤/额外字段。
    #[serde(default)]
    pub aux: Gap,
}

impl RasterEdit {
    /// O：`from`（impl$1::from 0x140a976b0；生成端）。
    pub fn from_parts(points: Vec<Vec3>, brush_size: f32, is_additive: bool) -> Self {
        Self {
            points,
            brush_size,
            is_additive,
            blend_mode: None,
            aux: Gap,
        }
    }
}

/// 点集合迭代（O：`raster_stroke::impl$2/3::iter_points_xz` 投影点到 xz）。
pub fn iter_points_xz(points: &[Vec3]) -> impl Iterator<Item = (f32, f32)> + '_ {
    points.iter().map(|p| (p.x, p.z))
}
