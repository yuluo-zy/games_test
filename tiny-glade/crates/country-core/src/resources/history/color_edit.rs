//! B `resources/history/color_edit.rs` — 颜色编辑负载（章节 O；48 函数，serde 族
//! serialize 0x14003c5d0 / deserialize 0x1417befe0 / visit_str 0x1417bf8a0）。
//!
//! `ColorEdit`：目标类型（Wall/Roof/Halftimber/Decorator/StairRailing/FlatRoof/
//! StairLadder/Brightness）+ from/to 颜色 + 目标 id + 附加 chimney_h_offset/
//! allow_connection。运行侧：load_game 旧版颜色 ID 升级（映射 {1→2,2→5,4→8,5→3,8→4}，
//! +0x52 ∈ {0x38..0x3a} 记录 +0x30 字段）、save_game Colors 节。
use crate::resources::color_id::ColorId;

/// 颜色编辑目标类型（章节：Wall/Roof/Halftimber/Decorator/StairRailing/FlatRoof/
/// StairLadder/Brightness）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum ColorTarget {
    #[default]
    Wall = 0,
    Roof = 1,
    Halftimber = 2,
    Decorator = 3,
    StairRailing = 4,
    FlatRoof = 5,
    StairLadder = 6,
    Brightness = 7,
}

/// 颜色编辑负载（字段序/类型待取证，字段名按章节）。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ColorEdit {
    pub target: ColorTarget,
    pub from: ColorId,
    pub to: ColorId,
    /// 目标 id（墙/屋顶/装饰 id；类型待取证）。
    pub target_id: u64,
    pub chimney_h_offset: f32,
    pub allow_connection: bool,
}
