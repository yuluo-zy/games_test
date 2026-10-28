//! B `startup_ivy` 0x140a21a30: loads the ivy branch/leaf/flower meshes into
//! the asset mesh library.
//!
//! Machine-verified mesh strings (read_memory on the B image):
//! - named meshes (FUN_140b1aca0): `ivy_debug_cube` (0x142ac9723, len 14),
//!   `ivy_branch` (0x142ac9798, len 10), `ivy_leaf` (0x142ac9818, len 8),
//!   `ivy_flower` (0x142ac9898, len 10);
//! - file meshes (FUN_140b0eeb0): `assets/meshes/ivy_branch.json`
//!   (0x142ac9731, len 29), `assets/meshes/ivy_leaf.json` (0x142ac97e2,
//!   len 27), `assets/meshes/ivy_flower.json` (0x142ac985c, len 29).
//!
//! The shader registrations (instanced_ivy_branch / instanced_ivy_leaf raster
//! programs) and the three 0x78-byte spawn records (FUN_1412dd460) depend on
//! the render shader/spawn pipeline, which this crate has not restored (待接线).
//! `ivy_debug_cube` has no JSON geometry (a generated cube) — 待取证.
use crate::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};
use crate::utils::load_json::load_json_as_mesh;
use bevy_ecs::prelude::*;

/// Named meshes registered by `startup_ivy` (O: names at 0x142ac9798 etc.).
pub const IVY_MESHES: [(&str, &str); 3] = [
    ("assets/meshes/ivy_branch.json", "ivy_branch"),
    ("assets/meshes/ivy_leaf.json", "ivy_leaf"),
    ("assets/meshes/ivy_flower.json", "ivy_flower"),
];

/// `startup_ivy` (0x140a21a30): register the ivy meshes under their names.
/// `AssetMeshLibrary::add` uses interior mutability, so a shared `Res` suffices.
pub fn startup_ivy(meshes: Res<AssetMeshLibrary>) {
    for (path, name) in IVY_MESHES {
        let mesh = load_json_as_mesh(path);
        meshes.add(MeshAsset::new(mesh, Some(name.to_string())));
    }
}
