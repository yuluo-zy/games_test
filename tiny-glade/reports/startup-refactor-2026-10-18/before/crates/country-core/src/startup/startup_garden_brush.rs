//! B `startup_garden_brush` 0x1415f87c0: per brush asset, in order —
//! `load_json_as_mesh<&str>` → `load_mesh_into_library` →
//! `load_shader_into_library<&str>` → `Commands::spawn_empty()` + one `insert`
//! record. Two iterations; the second `insert` carries
//! `(DrawableMeshBundle, DontCastShadow)` (O, fresh disassembly 2026-10-17:
//! the two push closure_env_0 at 0x1415f8a63 / 0x1415f88eb).
//!
//! Machine-verified strings (read_memory on the B image, 0x142c7d468…):
//! - meshes: `assets/meshes/garden_stone.json` (0x142c7d468, len 31) →
//!   name `garden_stone` (0x142c7d487, len 12);
//!   `assets/meshes/lowpoly_flower.json` (0x142c7d4de, len 33) → name
//!   `garden_flower_03` (0x142c7d4ff, len 16);
//! - raster shaders: `/solid_vertex_color_garden_stone.raster.hlsl`
//!   (0x142c7d493, len 44) → program `solid_vertex_color_garden_stone`
//!   (0x142c7d4bf, len 31); `/garden_lowpoly_flower.raster.hlsl`
//!   (0x142c7d50f, len 34) → program `garden_lowpoly_flower`
//!   (0x142c7d531, len 21).
//!   (2026-10-17 correction: RESTORE.md's `garden_stone/…` /
//!   `garden_flower_03/…` shader-path prefix is **not** in the image; paths
//!   start with `/`, and the shader program names are the file stems, not the
//!   mesh names.)
//!
//! The mesh part is wired to the ECS G2 mesh-load group ([`startup_garden_brush`]).
//! The shader loads and the two spawn/insert records depend on the render
//! shader/spawn pipeline, expressed through the [`GardenBrushHost`] boundary —
//! the same decoupling as `startup_quad.rs::StartupHost`.
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

/// Raster shader programs loaded per brush asset, same order as
/// [`GARDEN_BRUSH_MESHES`]: `(path, program name)`.
/// O: read_memory 0x142c7d493/0x142c7d4bf and 0x142c7d50f/0x142c7d531.
pub const GARDEN_BRUSH_SHADERS: [(&str, &str); 2] = [
    (
        "/solid_vertex_color_garden_stone.raster.hlsl",
        "solid_vertex_color_garden_stone",
    ),
    ("/garden_lowpoly_flower.raster.hlsl", "garden_lowpoly_flower"),
];

/// Steps 1-2 of the native sequence per brush asset: `load_json_as_mesh<&str>`
/// (panicking loader, O) + `load_mesh_into_library` (`AssetMeshLibrary::add`).
/// `AssetMeshLibrary::add` uses interior mutability, so a shared `Res` suffices.
pub fn register_brush_mesh(
    meshes: &AssetMeshLibrary,
    path: &str,
    name: &str,
) -> crate::asset_libraries::Handle<crate::render::mesh::Mesh> {
    let mesh = load_json_as_mesh(path);
    meshes.add(MeshAsset::new(mesh, Some(name.to_string())))
}

/// `startup_garden_brush` (0x1415f87c0), ECS G2 mesh-load wiring: registers the
/// garden brush meshes. The shader loads and spawn records are reconstructed by
/// [`run_garden_brush`] through the [`GardenBrushHost`] render boundary.
pub fn startup_garden_brush(meshes: Res<AssetMeshLibrary>) {
    for (path, name) in GARDEN_BRUSH_MESHES {
        register_brush_mesh(&meshes, path, name);
    }
}

/// Render-pipeline boundary for the native shader-load and spawn steps:
/// `load_shader_into_library<&str>` (0x140b3fe40) and
/// `Commands::spawn_empty` (0x140d146a0) + `insert<DrawableMeshBundle>` /
/// `insert<(DrawableMeshBundle, DontCastShadow)>`. Mirrors
/// `startup_quad.rs::StartupHost`.
pub trait GardenBrushHost {
    /// The mesh handle type produced by `load_mesh_into_library`.
    type Mesh;
    /// The shader handle type produced by `load_shader_into_library`.
    type Shader;

    /// `load_mesh_into_library` (0x140b1aca0): register a JSON-loaded mesh.
    fn load_mesh_into_library(
        &mut self,
        name: &'static str,
        mesh: crate::render::mesh::Mesh,
    ) -> Self::Mesh;

    /// `load_shader_into_library<&str>` (0x140b3fe40).
    fn load_shader_into_library(
        &mut self,
        path: &'static str,
        name: &'static str,
    ) -> Self::Shader;

    /// `Commands::spawn_empty().insert(...)`: the second iteration (flower)
    /// inserts `(DrawableMeshBundle, DontCastShadow)` (O).
    fn spawn_empty_insert(
        &mut self,
        mesh: Self::Mesh,
        shader: Self::Shader,
        dont_cast_shadow: bool,
    );
}

/// Full native control flow of 0x1415f87c0, two iterations, in order:
/// `load_json_as_mesh<&str>` → `load_mesh_into_library` →
/// `load_shader_into_library<&str>` → `spawn_empty` + `insert` (2nd with
/// `DontCastShadow`). The inserted bundle is `Transform::IDENTITY` + mesh
/// handle + shader handle + a `false` flag (O bundle stores at
/// 0x1415f88f0…0x1415f8960).
///
/// `load_mesh` is injectable so tests do not read the game's brush JSON from
/// disk; the ECS system uses the real [`load_json_as_mesh`].
pub fn run_garden_brush<H: GardenBrushHost>(
    host: &mut H,
    mut load_mesh: impl FnMut(&str) -> crate::render::mesh::Mesh,
) {
    for (i, (path, name)) in GARDEN_BRUSH_MESHES.iter().enumerate() {
        let mesh = load_mesh(path);
        let mesh_handle = host.load_mesh_into_library(name, mesh);
        let (shader_path, shader_name) = GARDEN_BRUSH_SHADERS[i];
        let shader = host.load_shader_into_library(shader_path, shader_name);
        host.spawn_empty_insert(mesh_handle, shader, i == 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        meshes: Vec<(&'static str, u64)>,
        shaders: Vec<(&'static str, &'static str)>,
        casts: Vec<bool>,
        order: Vec<&'static str>,
    }

    impl GardenBrushHost for Rec {
        type Mesh = u64;
        type Shader = u64;
        fn load_mesh_into_library(&mut self, name: &'static str, mesh: crate::render::mesh::Mesh) -> u64 {
            self.order.push("mesh");
            self.meshes.push((name, mesh.indices.len() as u64));
            100 + self.meshes.len() as u64
        }
        fn load_shader_into_library(&mut self, path: &'static str, name: &'static str) -> u64 {
            self.order.push("shader");
            self.shaders.push((path, name));
            200 + self.shaders.len() as u64
        }
        fn spawn_empty_insert(&mut self, mesh: u64, shader: u64, dont_cast_shadow: bool) {
            self.order.push("spawn");
            assert!(mesh >= 101 && shader >= 201, "handles flow into the bundle");
            self.casts.push(dont_cast_shadow);
        }
    }

    #[test]
    fn two_iterations_run_mesh_shader_spawn_and_second_insert_casts_no_shadow() {
        let mut h = Rec::default();
        run_garden_brush(&mut h, |_path| crate::render::mesh::Mesh::new("brush"));
        assert_eq!(h.order, vec!["mesh", "shader", "spawn", "mesh", "shader", "spawn"]);
        assert_eq!(h.meshes[0].0, "garden_stone");
        assert_eq!(h.meshes[1].0, "garden_flower_03");
        assert_eq!(
            h.shaders,
            vec![
                ("/solid_vertex_color_garden_stone.raster.hlsl", "solid_vertex_color_garden_stone"),
                ("/garden_lowpoly_flower.raster.hlsl", "garden_lowpoly_flower"),
            ]
        );
        assert_eq!(h.casts, vec![false, true], "2nd insert carries DontCastShadow");
    }
}
