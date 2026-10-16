//! B `startup_garden_brush` 0x1415f87c0: loads the garden brush meshes into
//! the asset mesh library.
//!
//! Machine-verified mesh strings (read_memory on the B image):
//! - file mesh (FUN_140b0eeb0): `assets/meshes/garden_stone.json`
//!   (0x142c7d468, len 31) and `assets/meshes/lowpoly_flower.json`
//!   (0x142c7d4de, len 33);
//! - named meshes (FUN_140b1aca0): `garden_stone` (0x142c7d487, len 12) and
//!   `garden_flower_03` (0x142c7d4ff, len 16).
//!
//! The two raster shaders (`garden_stone/solid_vertex_color_garden_stone.raster.hlsl`,
//! `garden_flower_03/garden_lowpoly_flower.raster.hlsl`) and the two 0x78-byte
//! spawn records (FUN_1412dd460 / FUN_1412d9de0) depend on the render
//! shader/spawn pipeline, which this crate has not restored (待接线).
use crate::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};
use crate::utils::load_json::load_json_as_mesh;
use bevy_ecs::prelude::*;

/// Named meshes registered by `startup_garden_brush` (O: names at 0x142c7d487 /
/// 0x142c7d4ff). Note the file mesh for the flower is `lowpoly_flower.json`,
/// registered under the name `garden_flower_03`.
pub const GARDEN_BRUSH_MESHES: [(&str, &str); 2] = [
    ("assets/meshes/garden_stone.json", "garden_stone"),
    ("assets/meshes/lowpoly_flower.json", "garden_flower_03"),
];

/// `startup_garden_brush` (0x1415f87c0): register the garden brush meshes.
/// `AssetMeshLibrary::add` uses interior mutability, so a shared `Res` suffices.
pub fn startup_garden_brush(meshes: Res<AssetMeshLibrary>) {
    for (path, name) in GARDEN_BRUSH_MESHES {
        let mesh = load_json_as_mesh(path);
        meshes.add(MeshAsset::new(mesh, Some(name.to_string())));
    }
}
