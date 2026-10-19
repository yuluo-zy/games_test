//! TEMPORARY USER-AUTHORIZED COMPILATION STUB. Original source awaits sync.
//! This file is not recovered gameplay. Every callable entry fails before
//! creating assets, state, entities or a successful readiness result.
#![allow(unused_variables)]

use std::path::Path;
use bevy_ecs::prelude::*;
use crate::asset_libraries::{mesh_library::AssetMeshLibrary,
    meshwatch::MeshWatch, shader_library::AssetShaderLibrary};
use crate::render::shader::boundary::ShaderBackend;
pub fn spawn_brush_previews(
    root: &Path, backend: &dyn ShaderBackend, meshes: &AssetMeshLibrary,
    watch: &MeshWatch, shaders: &AssetShaderLibrary, commands: &mut Commands,
) -> anyhow::Result<()> {
    anyhow::bail!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: country-core/startup/spawn_brush_previews");
}
pub fn spawn_brush_previews_meshes(meshes: Res<AssetMeshLibrary>) {
    panic!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: country-core/startup/spawn_brush_previews_meshes");
}
