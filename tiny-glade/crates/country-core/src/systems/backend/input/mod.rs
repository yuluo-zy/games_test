//! `systems::backend::input` — 后端输入链：动作增量、光标合成、录制/回放状态机与键盘映射。
//!
//! 原文件清单（inventory，B 版）：`translate_input.rs`(5)、`input_record_replay.rs`(8)、
//! `input_replay_for_trailers.rs`(3)、`reset_input_state.rs`(1)、`calculate_input_delta.rs`(2)、
//! `translate_input_to_cursor_moved.rs`(1)、`update_cursor_position.rs`(1)。
//!
//! 帧顺序（O，`input/ALGORITHMS.md` §5）：
//! `translate_input` → `translate_input_to_cursor_moved` → `calculate_input_delta` →
//! `reset_input_state`；录制/回放系统（`input_record_replay`、`control_input_replay`、
//! `advance_input_replay_frame`、`record_and_replay_input_for_trailers`）并行消费同一状态。
//!
//! crate 无 `bevy_input` 依赖（O：Cargo.toml 只含 bevy_ecs/bevy_time），原始
//! `ButtonInput<KeyCode>`/`ButtonInput<MouseButton>` 用本模块 [`button_input`] 等价抽象
//! （`pressed`/`just_pressed`/`reset_all` 语义）。
pub mod button_input;
pub mod calculate_input_delta;
pub mod input_record_replay;
pub mod input_replay_for_trailers;
pub mod reset_input_state;
pub mod translate_input;
pub mod translate_input_to_cursor_moved;
pub mod update_cursor_position;

use bevy_ecs::prelude::*;
use input_record_replay::InputReplayCommand;
use translate_input_to_cursor_moved::{InputCursorDelta, InputCursorMoved};

/// 注册本模块事件与资源（外接调度前调用）。
pub fn register_input_events(world: &mut World) {
    world.init_resource::<Events<InputCursorMoved>>();
    world.init_resource::<Events<InputCursorDelta>>();
    world.init_resource::<Events<InputReplayCommand>>();
    world.init_resource::<Events<calculate_input_delta::UiHistoryButtonPressed>>();

    world.init_resource::<input_record_replay::InputRecordReplayState>();
    world.init_resource::<input_record_replay::InputTranslationStateResource>();
    world.init_resource::<input_record_replay::InputReplayOutputState>();
    world.init_resource::<input_record_replay::ExternalRecordFlag>();
    world.init_resource::<update_cursor_position::CursorPosition>();
    world.init_resource::<button_input::KeyButtonInput>();
    world.init_resource::<button_input::MouseButtonInput>();
    world.init_resource::<translate_input::SteamInputActive>();
    world.init_resource::<input_replay_for_trailers::InputReplayForTrailers>();
}

/// 本地可运行顺序（仅接入本模块内部可自持的资源；窗口/相机/工具栏等外部资源由宿主提供）。
pub fn add_input_cpu_consumers(schedule: &mut Schedule) {
    schedule.add_systems((
        translate_input::translate_input,
        translate_input_to_cursor_moved::translate_input_to_cursor_moved,
        calculate_input_delta::calculate_input_delta,
        reset_input_state::reset_input_state,
        update_cursor_position::update_cursor_position,
        input_record_replay::input_record_replay,
        input_record_replay::control_input_replay,
        input_record_replay::advance_input_replay_frame,
        input_replay_for_trailers::record_and_replay_input_for_trailers,
        input_replay_for_trailers::do_input_replay_ui,
    ));
}
