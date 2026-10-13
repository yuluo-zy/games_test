//! ui_edit_roof_ridge_dir.rs — 脊线方向 UI 编辑（WP9）。
//!
//! PDB 骨架 `systems/roof/src/ui_edit_roof_ridge_dir.rs`。命名实体 `DoUiEditRoofRidgeDir` /
//! `RecordRidgeSwitchHistoryCmd` / `SwitchRoofRidgeDirCmd`。机器码：`ui_edit_roof_ridge_dir`
//! 0x141a386b0（line 30）、`do_ui_edit_roof_ridge_dir` 0x141a39490（line 96）、
//! `switch_roof_ridge_direction` 0x141a3a730（line 261）、`record_switch_roof_ridge_history`
//! 0x141a3ac20（line 299）。

#![allow(dead_code, unused_imports)]

use crate::resources::roofs::roof_shape::{Roof, RoofRidgeDirection};
use crate::typed_gap::Gap;

#[derive(Clone, Debug, Default)]
pub struct DoUiEditRoofRidgeDir(pub Gap);
#[derive(Clone, Debug, Default)]
pub struct RecordRidgeSwitchHistoryCmd(pub Gap);
#[derive(Clone, Debug, Default)]
pub struct SwitchRoofRidgeDirCmd(pub Gap);

/// ui_edit_roof_ridge_dir（O：0x141a386b0）。
pub fn ui_edit_roof_ridge_dir(_gizmo: Gap) {
    let _ = Gap;
}

/// do_ui_edit_roof_ridge_dir（O：0x141a39490；get_gizmo_info）。
pub fn do_ui_edit_roof_ridge_dir(_roofs: &mut [Roof]) {
    let _ = _roofs;
}

/// switch_roof_ridge_direction（O：0x141a3a730）：翻转 roof[0x3c] 脊线方向。
pub fn switch_roof_ridge_direction(roof: &mut Roof) -> RoofRidgeDirection {
    let cur = roof.0[0x3c];
    let next = cur ^ 1;
    roof.0[0x3c] = next;
    if next == RoofRidgeDirection::Length as u8 {
        RoofRidgeDirection::Length
    } else {
        RoofRidgeDirection::Width
    }
}

/// record_switch_roof_ridge_history（O：0x141a3ac20）。
pub fn record_switch_roof_ridge_history(_cmd: RecordRidgeSwitchHistoryCmd) {
    let _ = _cmd;
}
