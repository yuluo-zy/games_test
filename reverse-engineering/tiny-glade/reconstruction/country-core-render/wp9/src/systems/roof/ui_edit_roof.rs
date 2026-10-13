//! ui_edit_roof.rs — 屋顶 UI 编辑系统（WP9）。
//!
//! PDB 骨架 `systems/roof/src/ui_edit_roof.rs`。命名实体 `DoUiEditRoof` /
//! `OriginalWallHeight` / `RoofParamsCacheResource` / `RoofShapeEditAmount`。
//! 机器码：`ui_edit_roof` 0x141a18dd0（line 49）、`do_ui_edit_roof` 0x141a19a90（line 104）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::roofs::roof_shape::Roof;
use crate::typed_gap::Gap;

/// UI 编辑请求（命名实体，布局 U）。
#[derive(Clone, Debug, Default)]
pub struct DoUiEditRoof(pub Gap);
/// 原墙高缓存（命名实体，U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct OriginalWallHeight(pub Gap);
/// 屋顶参数缓存（命名实体，U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RoofParamsCacheResource(pub Gap);
/// 屋顶形状编辑量（命名实体，U）。
#[derive(Clone, Debug, Default)]
pub struct RoofShapeEditAmount(pub Gap);

/// ui_edit_roof（O：0x141a18dd0，line 49）。
pub fn ui_edit_roof(_ui: Gap, _roofs: &mut [Roof], _cache: &RoofParamsCacheResource) {
    let _ = _cache;
    let _ = _ui;
    let _ = _roofs;
}

/// do_ui_edit_roof（O：0x141a19a90，line 104）：按 RoofType 分派 + update_wall_height_if_needed。
pub fn do_ui_edit_roof(_request: DoUiEditRoof, _roofs: &mut [Roof]) {
    let _ = _request;
    let _ = _roofs;
}
