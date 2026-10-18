//! toolbar 拍照模式：`ui_slider`（O 0x1423486d0，符号
//! `toolbar::photomode::ui_slider::ui_slider`）。
//!
//! **未实现（重建阻塞，wave3 复核）**：Ghidra decompile 39KB（`ghidra-0x1423486d0-*.c.txt`）
//! 已证系统多参（MainCamera/BrushSizes 等接线），`switch(*(byte*)(entry+0x10))`
//! ~28 case（0..0x1b），多处 "Bad instruction - Truncating control flow"。前段：
//! `divss xmm10,[0x1429cd068]`（值/总长 → 01）、`xorps xmm14, 0xffff...`（-x）。
//! callees（wave3 fresh）：`slider_val_01`/`inverse_lerp`/`just_pressed`/`pressed`/
//! `released`/`contains`/`from_center_dims`/`despawn`/`spawn<Ui2dExtent,Ui2dImage,
//! UiDrawOrder>`——滑块值/交互判定链。截断 case 体需 radare2 定点反汇编 case target；
//! `entry+0x10` 的 widget kind 枚举未落库。本文件不落近似体。

/// O 0x1423486d0 `toolbar::photomode::ui_slider::ui_slider`：拍照参数滑块。
///
/// 已解码结构（见模块注释）：`switch(entry+0x10)` 按 widget kind 分派。case 体
/// 未全部取证，本占位登记未实现。
pub fn ui_slider() {
    // 未实现（分歧见 RESTORE.md §ui_slider）。
}
