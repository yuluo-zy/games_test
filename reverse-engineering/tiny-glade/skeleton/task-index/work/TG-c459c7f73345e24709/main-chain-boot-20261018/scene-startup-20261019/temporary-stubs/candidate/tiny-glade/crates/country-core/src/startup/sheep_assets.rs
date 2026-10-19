//! TEMPORARY USER-AUTHORIZED COMPILATION STUB. Original source awaits sync.
//! This file is not recovered gameplay. Every callable entry fails before
//! creating assets, state, entities or a successful readiness result.
#![allow(unused_variables)]

use std::path::Path;
use bevy_ecs::prelude::*;
use crate::asset_libraries::{mesh_library::AssetMeshLibrary,
    meshwatch::MeshWatch, shader_library::AssetShaderLibrary};
use crate::render::shader::boundary::ShaderBackend;
use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
/// Opaque compile-time carrier only. No Default or successful constructor.
/// User source will replace this type and its real owned fields at sync.
pub struct SheepStartupAssets { pending_sync: () }
pub fn load_sheep_startup_assets(
    root: &Path, meshes: &AssetMeshLibrary, shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend, watch: &MeshWatch, ssbos: &AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) -> anyhow::Result<SheepStartupAssets> {
    anyhow::bail!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: country-core/startup/sheep_assets");
}
