//! startup/startup_roof.rs — 屋顶资产注册（WP9）。
//!
//! PDB 骨架 `startup/startup_roof.rs`（line 14）。机器码 `startup_roof` 0x1414547d0。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// startup_roof（O：0x1414547d0）。4×：`load_json_as_mesh` + `load_mesh_into_library` +
/// `load_shader_into_library` → `spawn_empty` + 插入 `DrawableMeshBundle`
/// （Transform {t=0, r=identity, s=1}）。→ 注册屋顶 4 个网格资产（屋瓦/spire/支撑梁/旗布，
/// U 名称）。
pub fn startup_roof(_commands: Gap, _mesh_library: Gap) {
    let _ = _commands;
    let _ = _mesh_library;
}
