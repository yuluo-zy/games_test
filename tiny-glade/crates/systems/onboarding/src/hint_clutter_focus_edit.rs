//! hint_clutter_focus_edit（B 主基线；3 函数）。
//!
//! 证据（skeleton PDB）：make_focus_or_camera_binding_hint 0x141c3de00（L0）、
//! hint_clutter_focused_edit_condition 0x141c3d890（L42）、hint_clutter_focused_edit_hint
//! 0x141c3db80（L74）。语义与 hint_focused_edit 同族（杂物焦点编辑提示）。
//! 本模块保留独立文件结构（原源码文件），实现经 hint_focused_edit 共享核心。

pub use crate::hint_focused_edit::{
    hint_clutter_focused_edit_condition, hint_clutter_focused_edit_hint,
    make_focus_or_camera_binding_hint,
};
