//! `tiny_glade::gui::gui_debug` — developer/debug window.
//!
//! fresh Ghidra shared 2026-10-17：`do_gui_debug` `0x140147e00`（1,620 B）、
//! `do_freecam_save` `0x1401485c0`（1,081 B）、`date_spinner` `0x140148b40`
//! （352 B）、`do_camera_snapshot_ui` `0x140148cb0`（2,542 B）。PDB 同文件
//! 另有 `render_gpu_allocator_gui`、`do_tod_save_load_ui`、
//! `do_tod_sequence_entry`。
//!
//! `date_spinner` is recovered: the body allocates a `DragValue` closure and
//! calls `egui::widgets::drag_value::impl$1::ui`; `.rdata` at `0x1429d3768`
//! carries the one-shot `0.1` speed (`0x3fb999999999999a`). The spinner's range
//! constants are loaded from `0x1429d3710` (a formatter/range descriptor) and are
//! not resolved to numbers, so the port keeps the recovered speed only.
//!
//! The other three functions are inlined egui window bodies; the exact widget
//! tree and `DebugGui` field accesses are not closed (see divergences).

use egui::Ui;

/// B `date_spinner` (`0x140148b40`).
///
/// `ui.add(egui::DragValue::new(value).speed(0.1))`; the original applies a
/// date-range descriptor whose numeric bounds are not recovered.
pub fn date_spinner(ui: &mut Ui, value: &mut u32) -> egui::Response {
    ui.add(egui::DragValue::new(value).speed(0.1))
}

/// B `do_gui_debug` (`0x140147e00`) — 未实现-需重做.
pub fn do_gui_debug() {
    todo!(
        "未实现-需重做: do_gui_debug 0x140147e00 —— egui::Window 体与 \
         DebugGui/gpu allocator/render_gpu_allocator_gui 子面板未逐语句还原"
    );
}

/// B `do_freecam_save` (`0x1401485c0`) — 未实现-需重做.
pub fn do_freecam_save() {
    todo!(
        "未实现-需重做: do_freecam_save 0x1401485c0 —— Button/WidgetText/输入快照 \
         保存流程依赖未落库的 DebugGui 字段"
    );
}

/// B `do_camera_snapshot_ui` (`0x140148cb0`) — 未实现-需重做.
pub fn do_camera_snapshot_ui() {
    todo!(
        "未实现-需重做: do_camera_snapshot_ui 0x140148cb0 —— horizontal_with_main_wrap \
         与 label 组合的相机快照面板未还原"
    );
}
