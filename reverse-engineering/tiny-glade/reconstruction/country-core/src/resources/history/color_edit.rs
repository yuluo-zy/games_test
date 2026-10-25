//! `resources::history::color_edit` — 颜色编辑记录（WP17）。
//!
//! O（观察 JSON Color 命令嵌套 tag：Wall 53/Roof 23/Halftimber 13/Decorator 10/
//! StairRailing 7/FlatRoof 4/StairLadder 1/Brightness 3；字段
//! wall_id/roof_id/from/to/decorator_id/brightness。serde 手写 visitor 族，
//! deserialize 0x14017f0f0 区）：

use crate::typed_gap::Gap;
use serde::{Deserialize, Serialize};

/// 颜色目标枚举（O：观察 Color 嵌套 tag）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorTarget {
    Wall,
    Roof,
    #[default]
    Halftimber,
    Decorator,
    StairRailing,
    FlatRoof,
    StairLadder,
    Brightness,
    /// 未证目标（观察 open）。
    Other(Gap),
}

/// 颜色编辑记录（O：目标 + from/to + 目标 id）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ColorEdit {
    /// O：目标类型。
    #[serde(default)]
    pub target: ColorTarget,
    /// O：`from`/`to`（颜色值，观察数字或 Override 嵌套）。
    #[serde(default)]
    pub from: Gap,
    #[serde(default)]
    pub to: Gap,
    /// O：`wall_id`/`roof_id`/`decorator_id`（观察）。
    #[serde(default)]
    pub target_id: Option<u64>,
}
