//! `tiny_glade::gui::gui_tod` — time-of-day screen.
//!
//! fresh Ghidra shared 2026-10-17：`do_time_of_day_screen` `0x1405cc7f0`
//! （862 B）、`tod_circle<u64>` `0x140334130`（666 B）。PDB 同文件另有
//! `parabola`、`remap_angle_to_tod`、`remap_tod_to_angle` 与
//! `TodCircleView::{delta,ui_confirm}` 等，属同一未恢复子树。
//!
//! `do_time_of_day_screen` 构建 MEI `TemplateView` 节点树（2 个闭包）并发出
//! `WriteSettingsFileCmd`；`tod_circle` 为极坐标圆盘布局/命中例程。节点树与
//! 圆盘几何尚未逐语句还原。

/// B `do_time_of_day_screen` (`0x1405cc7f0`) — 未实现-需重做.
pub fn do_time_of_day_screen() {
    todo!(
        "未实现-需重做: do_time_of_day_screen 0x1405cc7f0 —— MEI 模板节点/\
         WriteSettingsFileCmd 发出顺序未还原"
    );
}

/// B `tod_circle<u64>` (`0x140334130`) — 未实现-需重做.
pub fn tod_circle() {
    todo!("未实现-需重做: tod_circle 0x140334130 —— 极坐标圆盘布局/命中测试未还原");
}
