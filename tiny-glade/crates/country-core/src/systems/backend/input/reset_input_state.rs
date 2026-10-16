//! `reset_input_state.rs` — 输入状态复位（B 0x140942b00，O）。
//!
//! 证据：`reconstruction/country-core-render/wp20/analysis/0x140942b00_reset_input_state.txt`
//! 与 `chapters/systems/backend/input/reset_input_state.rs.md`（wp20 合同复核）。
//!
//! 算法（O）：
//! 1. **清零并重排**：翻译状态资源 `current @0x00..0x58` ← `buffer @0x58..0xa8`（11 qword
//!    恒等置换），并清零 `buffer`（`0x58..0xa8`）。
//! 2. **按事件双缓冲修剪**：排空两个 `Events<T>` 双 VecDeque（计数回写；本实现消费
//!    `InputCursorMoved` 与 `InputReplayCommand` 两个事件流，语义与 bevy 排空一致）。
//! 3. **`ButtonInput::reset_all<KeyCode>()`**：复位键盘按键沿状态。crate 无 bevy_input
//!    依赖，用 [`crate::systems::backend::input::button_input::KeyButtonInput`] 等价抽象
//!    （`reset_all` 清空 just-pressed 沿）。
//! 4. 保留 `InputStream` 结构（mode/index 不动）——仅供录制重来。
use bevy_ecs::prelude::*;

use super::{
    button_input::KeyButtonInput,
    input_record_replay::{InputReplayCommand, InputTranslationStateResource},
    translate_input_to_cursor_moved::InputCursorMoved,
};

/// `reset_input_state` @0x140942b00。
///
/// decomp 参数：`in_RCX` = ResMut<翻译状态资源>（22 qword 置换）、`in_RDX`/`in_R8` = 两个
/// Events 双缓冲、`in_R9` = ResMut<ButtonInput<KeyCode>>（本实现为 `KeyButtonInput`）。
pub fn reset_input_state(
    mut translation: ResMut<InputTranslationStateResource>,
    mut cursor_moved: EventReader<InputCursorMoved>,
    mut replay_commands: EventReader<InputReplayCommand>,
    mut keyboard: ResMut<KeyButtonInput>,
) {
    // 1. buffer → current 置换（O：`new[i] = old[0xb+i]`，i in 0..11），buffer 清零。
    let buffer = translation.buffer;
    translation.current = buffer;
    translation.buffer = Default::default();

    // 2. 排空事件双缓冲（O：两个 Events 各自计算未消费计数并回写）。
    let had_cursor = cursor_moved.read().next().is_some();
    let had_replay = replay_commands.read().next().is_some();

    // 3. 有任一事件 → 复位键盘按键沿（O：`reset_all<KeyCode>`）。
    if had_cursor || had_replay {
        keyboard.reset_all();
    }
}
