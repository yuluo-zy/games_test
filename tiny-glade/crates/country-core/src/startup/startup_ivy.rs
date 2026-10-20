//! Original B startup_ivy @140a21a30 (0x5c8 B): ivy debug cube, three ivy meshes,
//! their instanced raster shaders and three ivy drawables.
//!
//! Machine-code order (O): cube mesh `ivy_debug_cube` -> for each of branch / leaf / flower:
//! `load_json_as_mesh` -> `load_mesh_into_library` -> `load_shader_into_library` -> `Commands::spawn_empty().insert(..)`.
//! The cube is only registered in the mesh library (never spawned). Each spawn is exactly
//! `DrawableMeshBundle` (command 0x1412dd460, component ids = mesh, shader, transform; no DontCastShadow,
//! no designated pass, no KeepOnLevelUnload).
//!
//! Flower shader: the original passes the *leaf* path (`0x142ac9820`, length 0x23) with the name
//! `instanced_ivy_flower_shader` (length 0x1b); no `instanced_ivy_flower.raster.hlsl` ships.
use std::path::Path;

use bevy_ecs::system::Commands;
use glam::Vec3;

use crate::{
    asset_libraries::{
        Handle,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        meshwatch::MeshWatch,
        shader_library::AssetShaderLibrary,
    },
    components::drawable::DrawableMeshBundle,
    geometry::cube::{CubeBox, cube_mesh},
    render::{
        mesh::Mesh,
        shader::{RasterShaderProgram, boundary::ShaderBackend},
    },
    utils::{components::transform::Transform, load_json::try_load_json_as_mesh},
};

/// `ivy_debug_cube` box ranges (O: 24 B at 0x142ac89c0 = `[-0.1, 0.1, -1.5, 1.5, -0.1, 0.1]`).
pub const DEBUG_CUBE: CubeBox = CubeBox {
    x: [-0.1, 0.1],
    y: [-1.5, 1.5],
    z: [-0.1, 0.1],
};
/// `Mesh::add_color_self` colour (O: 0x142925904 = `(1.0, 0.6, 0.0)`).
pub const DEBUG_CUBE_COLOR: Vec3 = Vec3::new(1.0, 0.6, 0.0);
pub const DEBUG_CUBE_NAME: &str = "ivy_debug_cube";

/// (mesh path, library name, shader path, shader name), in the original order.
pub const IVY_DRAWABLES: [(&str, &str, &str, &'static str); 3] = [
    (
        "assets/meshes/ivy_branch.json",
        "ivy_branch",
        "/ivy/instanced_ivy_branch.raster.hlsl",
        "instanced_ivy_branch_shader",
    ),
    (
        "assets/meshes/ivy_leaf.json",
        "ivy_leaf",
        "/ivy/instanced_ivy_leaf.raster.hlsl",
        "instanced_ivy_leaf_shader",
    ),
    (
        "assets/meshes/ivy_flower.json",
        "ivy_flower",
        "/ivy/instanced_ivy_leaf.raster.hlsl",
        "instanced_ivy_flower_shader",
    ),
];

/// Common original `load_mesh_into_library` @140b1aca0: watch first, add second.
fn load_mesh_into_library(
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
    mesh: Mesh,
    name: &str,
) -> Handle<Mesh> {
    watch.watch(&mesh);
    meshes.add(MeshAsset::new(mesh, Some(name.to_owned())))
}

pub fn startup_ivy(
    root: &Path,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    commands: &mut Commands,
) -> anyhow::Result<()> {
    let cube = cube_mesh(&DEBUG_CUBE).add_color_self(DEBUG_CUBE_COLOR);
    load_mesh_into_library(meshes, watch, cube, DEBUG_CUBE_NAME);
    for (path, name, shader_path, shader_name) in IVY_DRAWABLES {
        let full = root.join(path);
        let full = full
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("ivy mesh path is not UTF-8: {}", full.display()))?;
        let mesh = load_mesh_into_library(meshes, watch, try_load_json_as_mesh(full)?, name);
        let shader = super::load_shader_into_library(
            shader_name,
            crate::rhapsody_types::RasterPipelineDesc {
                path: shader_path.to_owned(),
                entry: String::new(),
                defines: Vec::new(),
            },
            backend,
            shaders,
        )?;
        commands.spawn(drawable(mesh, shader));
    }
    Ok(())
}

fn drawable(mesh: Handle<Mesh>, shader: Handle<RasterShaderProgram>) -> DrawableMeshBundle {
    DrawableMeshBundle {
        mesh,
        shader,
        transform: Transform::identity(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flower_reuses_the_leaf_shader_source_under_its_own_name() {
        assert_eq!(IVY_DRAWABLES[1].2, IVY_DRAWABLES[2].2);
        assert_ne!(IVY_DRAWABLES[1].3, IVY_DRAWABLES[2].3);
        assert_eq!(IVY_DRAWABLES[0].2.len(), 0x25);
        assert_eq!(IVY_DRAWABLES[1].2.len(), 0x23);
        assert_eq!(IVY_DRAWABLES[1].3.len(), 0x19);
        assert_eq!(IVY_DRAWABLES[2].3.len(), 0x1b);
    }

    #[test]
    fn debug_cube_is_registered_with_the_original_colour() {
        let cube = cube_mesh(&DEBUG_CUBE).add_color_self(DEBUG_CUBE_COLOR);
        assert!(cube.has_attribute(crate::render::mesh::ATTR_COLOR));
        assert_eq!(cube.indices.len(), 36);
    }
}
