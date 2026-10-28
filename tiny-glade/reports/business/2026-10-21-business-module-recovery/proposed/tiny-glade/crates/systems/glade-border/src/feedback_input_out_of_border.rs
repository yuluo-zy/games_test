//! feedback_input_out_of_border.rs — 越界输入反馈（B 行 12，PDB inlinee，无独立 VA；
//! 宿主 `run_unsafe<FunctionSystem<feedback_input_out_of_border,..>>` 0x141be2280，
//! 2026-10-17 fresh 反编译重建 + wave3 落库音效写面）。
//!
//! 内联系统体（O，0x141be2280 disasm）：
//! ```
//! has_feedback = feedback.read().last().is_some();      // EventReader 记账
//! if has_feedback {
//!     if state[+0x158] == 0 { sounds.write(PlayUiSound 变体 #4); }   // 上升沿一次
//!     state[+0x158] = 1;
//! } else {
//!     state[+0x158] = 0;
//! }
//! state[+0x130] = change_tick;   // last_run（bevy 胶水）
//! ```
//! `state[+0x158]` = `Local<InputOfOutBorderCache>`（1 字节 bool）——上一帧是否有越界反馈；
//! 音效在「反馈事件首次出现」的上升沿播放一次，连续越界帧不再重复。
//!
//! ## wave3 落库
//! 音效写面**已判等**：`PlayUiSound` 变体 #4（coregap-1 落库
//! 业务载荷 24B；另加 Bevy EventId 8B，事件步长 0x20B）。disasm
//! 0x141be2280 行 92-93：`*(lVar7 + lVar11*0x20) = 4`（判别值 4）、
//! `*(lVar7 + 0x18 + lVar11*0x20) = uVar6`（`uVar6 = *(lVar4+0x40)` =
//! EventWriter 计数，bevy 胶水）。crate `EventWriter::write` 自动补实例 id；
//! `PlayUiSound::variant4()` 不包含实例 id；EventId 由 Bevy 另行管理。

use bevy_ecs::prelude::*;

use country_core::resources::glade_border::FeedbackInputOutOfBorder;
use country_core::systems::ui::resources::widget_params::PlayUiSound;

/// 越界反馈去抖缓存（O：宿主 `state+0x158` 字节 = 上一帧是否有未读越界反馈）。
#[derive(Default)]
pub struct InputOfOutBorderCache {
    /// O：上一帧系统运行时有未读 `FeedbackInputOutOfBorder`（读末条事件后置 true/false）。
    pub had_feedback_last_run: bool,
}

/// `feedback_input_out_of_border`（O 面：`EventReader::read().last()` + 上升沿去抖；
/// wave3 落库 `PlayUiSound::variant4` 写面）。
pub fn feedback_input_out_of_border(
    mut feedback: EventReader<FeedbackInputOutOfBorder>,
    mut sounds: EventWriter<PlayUiSound>,
    mut cache: Local<InputOfOutBorderCache>,
) {
    let has_feedback = feedback.read().last().is_some();
    if has_feedback {
        // O：宿主 `[state+0x158] == 0`（上一帧无反馈）才写音效——上升沿触发一次。
        if !cache.had_feedback_last_run {
            // O：写 `PlayUiSound` 变体 #4（判别值 4 @+0，事件步长 0x20B，id @+0x18）。
            // id = 宿主 EventWriter 计数（Bevy 另行记账），不进入业务载荷。
            sounds.write(PlayUiSound::variant4());
        }
        cache.had_feedback_last_run = true;
    } else {
        cache.had_feedback_last_run = false;
    }
}
