//! B `resources/history/decorator_edit.rs` — 装饰编辑负载（章节 O；45 函数）。
//!
//! `DecoratorHistoryEdit`/`Diff`/`State`：装饰前后状态 Diff；`DecoratorHistoryState::From`
//!（preview → panic "Attempting to capture a DecoratorHistoryState of a preview"）；
//! `is_default_lantern_brightness`==`is_default_stair_width`==`x==1.0f`（0x140b3f910 ICF）；
//! `convert_before_after_states_to_changes` 0x140b3fab0（按 style 字节分变体比较浮点输出变更集）；
//! `send_state_diff_events` 0x1417bdae0（装饰 diff 应用）。回放：replay_pop_bucket 消费 +
//! restore_decorators FUN_14155ba30。
use crate::resources::color_id::ColorId;

/// 装饰历史编辑（前后状态 Diff；字段序按章节）。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecoratorHistoryEdit {
    pub id: u64,
    pub before: DecoratorHistoryState,
    pub after: DecoratorHistoryState,
}

/// 装饰历史状态。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecoratorHistoryState {
    /// 装饰 id。
    pub id: u64,
    /// style 字节（0x140b3fab0 按 style 字节分变体比较）。
    pub style: u8,
    /// 颜色/亮度等；字段序按章节。
    pub color: ColorId,
    pub lantern_brightness: f32,
    pub stair_width: f32,
}

impl DecoratorHistoryState {
    /// `DecoratorHistoryState::From`（preview panic；0x140b3f930）。
    pub fn from_preview() -> Self {
        panic!("Attempting to capture a DecoratorHistoryState of a preview")
    }
    /// `is_default_lantern_brightness`==`is_default_stair_width`==`x==1.0f`（ICF 0x140b3f910）。
    pub fn is_default_lantern_brightness(&self) -> bool {
        self.lantern_brightness == 1.0
    }
    pub fn is_default_stair_width(&self) -> bool {
        self.stair_width == 1.0
    }
}

/// 装饰历史差异（before/after；replay_pop_bucket 消费）。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecoratorHistoryDiff {
    pub before: DecoratorHistoryState,
    pub after: DecoratorHistoryState,
}
