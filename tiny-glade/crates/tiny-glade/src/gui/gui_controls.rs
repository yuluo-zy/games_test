//! `tiny_glade::gui::gui_controls` — controls/key-binding UI.
//!
//! fresh Ghidra shared 2026-10-17：`key_bindings_sub_menu` `0x1400854a0`
//! （5,898 B）、`show_control_tweakers` `0x140088240`（883 B）。
//! `show_gui_controls` 在 `0x1402ef5e0` **不是函数起点**（fresh
//! get_functions 返回 not found），VA 待重定位。
//!
//! `key_bindings_sub_menu` 用 `lang_lib::lookup` 构建键位行，并有
//! `with_slot<closure_env$0/$1>` 实例（0x140502d00 / 0x1405020f0）；
//! `show_control_tweakers` 用 `toolbar::mei::widgets::tweaker`
//! （`TweakableFloatWithStep`/`TweakableEnum`/`TweakableBool`/`HintTracker`）
//! 与 `CameraRotationPivot`。行表与 tweaker 描述符仍在闭包环境/`.rdata`，
//! 未逐语句还原。

/// B `key_bindings_sub_menu` (`0x1400854a0`) — 未实现-需重做.
pub fn key_bindings_sub_menu() {
    todo!(
        "未实现-需重做: key_bindings_sub_menu 0x1400854a0 —— 17 个 lang_lib 文案与 \
         键位行表未还原"
    );
}

/// B `show_control_tweakers` (`0x140088240`) — 未实现-需重做.
pub fn show_control_tweakers() {
    todo!(
        "未实现-需重做: show_control_tweakers 0x140088240 —— TweakableFloatWithStep/\
         TweakableEnum/TweakableBool 描述表未还原"
    );
}

/// B `show_gui_controls` (`0x1402ef5e0`) — 未实现-需重做.
pub fn show_gui_controls() {
    todo!("未实现-需重做: show_gui_controls 0x1402ef5e0 —— 控件总面板未还原");
}
