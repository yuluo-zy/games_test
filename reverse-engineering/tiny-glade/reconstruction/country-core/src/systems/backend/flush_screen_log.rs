//! flush_screen_log.rs — 屏幕日志后端（O）。
//!
//! 源 `systems/backend/flush_screen_log.rs`；实体 `LogType`。
//! `flush_error_log_to_screen`（0x1408c0d90）/ `log_error_to_screen`（0x1408c10a0）/
//! `log_warning_to_screen`（0x1408c1310）。

#![allow(dead_code)]

/// 日志类型（O：`LogType` 实体；错误/警告）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogType {
    Error,
    Warning,
}

/// `log_error_to_screen`（O 0x1408c10a0）：把错误消息写入屏幕日志队列。
pub fn log_error_to_screen(_msg: alloc::string::String) {
    log::error!("{}", _msg);
}

/// `log_warning_to_screen`（O 0x1408c1310）。
pub fn log_warning_to_screen(_msg: alloc::string::String) {
    log::warn!("{}", _msg);
}

/// `flush_error_log_to_screen`（O 0x1408c0d90）：排空队列逐条上屏。
pub fn flush_error_log_to_screen(_world: &mut bevy_ecs::world::World) {
    // U：队列位置与 UI 写入路径（typed gap）。
}
