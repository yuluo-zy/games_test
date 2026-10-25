//! ui_edit_roof_ridge_length.rs — 脊线长度 UI 编辑（WP9）。
//!
//! PDB 骨架 `systems/roof/src/ui_edit_roof_ridge_length.rs`。命名实体
//! `DoUiEditRoofRidgeLength` / `RidgeGrabParams`。机器码：`ui_edit_roof_ridge_length`
//! 0x1421a8210（line 40）、`do_ui_edit_roof_ridge_length` 0x1421a8c50（line 100）、
//! `create_ui_widget` 0x1421aa170（line 268）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

#[derive(Clone, Debug, Default)]
pub struct DoUiEditRoofRidgeLength(pub Gap);
#[derive(Clone, Debug, Default)]
pub struct RidgeGrabParams(pub Gap);

/// ui_edit_roof_ridge_length（O：0x1421a8210）。
pub fn ui_edit_roof_ridge_length(_gizmo: Gap) {
    let _ = Gap;
}
/// do_ui_edit_roof_ridge_length（O：0x1421a8c50）。
pub fn do_ui_edit_roof_ridge_length(_request: DoUiEditRoofRidgeLength) {
    let _ = _request;
}
/// create_ui_widget（O：0x1421aa170）。
pub fn create_ui_widget(_params: RidgeGrabParams) {
    let _ = _params;
}
