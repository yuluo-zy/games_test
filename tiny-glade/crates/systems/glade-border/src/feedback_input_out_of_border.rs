//! feedback_input_out_of_border.rs — 越界输入反馈（B 行 12，PDB inlinee，无独立
//! VA；re-analyst-A 契约 §1：逻辑并入调用点，本 crate 按 BASELINE 参数面落地）。
//!
//! 参数面（O，BASELINE 候选接口）：`(EventReader<FeedbackInputOutOfBorder>,
//! EventWriter<PlayUiSound>, Local<InputOfOutBorderCache>)`——读越界反馈事件 →
//! 发射 UI 声音（防抖缓存 Local，语义 I）。

use bevy_ecs::prelude::*;

use country_core::resources::glade_border::FeedbackInputOutOfBorder;
use country_core::systems::ui::resources::widget_params::PlayUiSound;

/// 越界反馈防抖缓存（I：内联体语义；字段 U）。
#[derive(Default)]
pub struct InputOfOutBorderCache {
    /// 上次反馈帧/时间（U）。
    pub last: u64,
}

/// `feedback_input_out_of_border`（O 面：`EventReader::read().last()` + 事件写入）。
pub fn feedback_input_out_of_border(
    mut feedback: EventReader<FeedbackInputOutOfBorder>,
    mut sounds: EventWriter<PlayUiSound>,
    _cache: Local<InputOfOutBorderCache>,
) {
    if feedback.read().last().is_some() {
        sounds.write(PlayUiSound);
    }
}
