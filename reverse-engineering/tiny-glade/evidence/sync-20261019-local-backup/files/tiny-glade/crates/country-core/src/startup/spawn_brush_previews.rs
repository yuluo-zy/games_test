//! 画笔预览网格（B `spawn_brush_previews` @0x140924b40，51 KB）。
//!
//! 证据（R10 契约 §2.4，O）：batch4；输入 shader/mesh 库、路径根、Commands、
//! ShaderBackend、watch。资源按出现序：
//! * shader `/timed_terrain_gizmo.raster.hlsl` → `timed_terrain_gizmo_shader`；
//! * 文件/注册名：`brush_arrow`、`brush_circle`、`brush_circle_cross`、
//!   `brush_circle_scribble`、`brush_arrow_flat`、`brush_4_arrows`(name
//!   `brush_terrain_edit`)、`brush_2_arrows`(name `brush_flat_roof_edit`)、
//!   `brush_5_replay`(name `brush_replay`)、`hand`(name `temp_sphere`)、
//!   `dashed_circle`、`dashed_lines`(name `snapping_line`)。
//! * spawn bundle 统计：10×0x38、4×0x40、2×0x78、1×0x88。
//!
//! U：bundle 与网格的逐项映射、Transform 初值。本模块只做真实网格注册。
use crate::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};
use super::register_mesh_file;
use bevy_ecs::prelude::Res;

/// O：文件 stem 与注册名（`name` 缺省时同名）。
pub const PREVIEW_MESHES: [(&str, Option<&str>); 11] = [
    ("brush_arrow", None),
    ("brush_circle", None),
    ("brush_circle_cross", None),
    ("brush_circle_scribble", None),
    ("brush_arrow_flat", None),
    ("brush_4_arrows", Some("brush_terrain_edit")),
    ("brush_2_arrows", Some("brush_flat_roof_edit")),
    ("brush_5_replay", Some("brush_replay")),
    ("hand", Some("temp_sphere")),
    ("dashed_circle", None),
    ("dashed_lines", Some("snapping_line")),
];

/// O：唯一显式注册的 shader。
pub const PREVIEW_SHADER: (&str, &str) =
    ("/timed_terrain_gizmo.raster.hlsl", "timed_terrain_gizmo_shader");

/// O：spawn 记录按 bundle 尺寸分组。
pub const PREVIEW_SPAWN_BUNDLES: [(usize, usize); 4] = [(0x38, 10), (0x40, 4), (0x78, 2), (0x88, 1)];

pub fn spawn_brush_previews_meshes(meshes: Res<AssetMeshLibrary>) {
    for (stem, name) in PREVIEW_MESHES {
        let path = format!("assets/meshes/{stem}.json");
        if !std::path::Path::new(&path).is_file() {
            log::error!(
                target: "country_core::startup::spawn_brush_previews",
                "mesh file {path} is absent; skipping"
            );
            continue;
        }
        let mesh = crate::utils::load_json::load_json_as_mesh(&path);
        meshes.add(MeshAsset::new(
            mesh,
            Some(name.unwrap_or(stem).to_string()),
        ));
    }
    // U: 17 条 spawn 的网格/Transform/pass 逐项映射未恢复。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_table_matches_the_contract_and_bundle_counts() {
        assert_eq!(PREVIEW_MESHES.len(), 11);
        assert_eq!(PREVIEW_SHADER.1, "timed_terrain_gizmo_shader");
        let spawns: usize = PREVIEW_SPAWN_BUNDLES.iter().map(|(_, count)| count).sum();
        assert_eq!(spawns, 17);
        assert_eq!(PREVIEW_SPAWN_BUNDLES[2], (0x78, 2));
        assert_eq!(
            PREVIEW_MESHES[5],
            ("brush_4_arrows", Some("brush_terrain_edit"))
        );
    }
}
