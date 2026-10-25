//! `tiny_glade::gui::gui_error_log` — error log window.
//!
//! fresh Ghidra shared 2026-10-17：`do_gui_error_log` `0x140160410`（608 B）。
//! 门控：`[res+0x90]`/`[res+0x70]`/`[res+0x50]`/`[res+0x30]`/`[res+0x10]`
//! 的多个 Option/计数条件、`*arg1 != 0` 与 `[res+0xf7]==1`；随后
//! `egui::Window::new(<&str>)` + `show_dyn`（闭包捕获 arg0/arg2，窗口
//! 尺寸常量 0xc1a00000 = -20.0? 为布局参数）。标题与条目表在闭包体内，
//! 未还原。

/// B `do_gui_error_log` (`0x140160410`) — 未实现-需重做.
pub fn do_gui_error_log() {
    todo!("未实现-需重做: do_gui_error_log 0x140160410 —— Window::show_dyn 闭包体未还原");
}
