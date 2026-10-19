//! TEMPORARY USER-AUTHORIZED COMPILATION STUB. Original source awaits sync.
//! This file is not recovered gameplay. Every callable entry fails before
//! creating assets, state, entities or a successful readiness result.
#![allow(unused_variables)]

use std::path::Path;
use bevy_ecs::prelude::*;
use crate::asset_libraries::{mesh_library::AssetMeshLibrary,
    meshwatch::MeshWatch, shader_library::AssetShaderLibrary};
use crate::render::shader::boundary::ShaderBackend;

pub fn startup_roof(
    meshes: &AssetMeshLibrary, shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend, watch: &MeshWatch, root: &Path,
    commands: &mut Commands,
) {
    panic!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: country-core/startup/startup_roof::startup_roof");
}

pub fn startup_roof_meshes(meshes: Res<AssetMeshLibrary>) {
    panic!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: country-core/startup/startup_roof::startup_roof_meshes");
}
