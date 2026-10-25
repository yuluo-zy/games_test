//! TEMPORARY USER-AUTHORIZED COMPILATION STUB. Original source awaits sync.
//! This file is not recovered gameplay. Every callable entry fails before
//! creating assets, state, entities or a successful readiness result.
#![allow(unused_variables)]

use std::path::Path;
use bevy_ecs::prelude::*;
use crate::asset_libraries::{mesh_library::AssetMeshLibrary,
    meshwatch::MeshWatch, shader_library::AssetShaderLibrary};
use crate::render::shader::boundary::ShaderBackend;
use crate::{
    asset_libraries::{placement_data_library::AssetPlacementDataLibrary,
        ssbo_library::AssetSsboLibrary},
    render::ssbo::rhapsody_boundary::GraphicsDevice,
    resources::{session::ActiveSession, tree_storage::InteractableForestPlacements,
        audio_resources::Studio},
    systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount,
};
pub fn init_clearing_complete(
    root: &Path, session: &ActiveSession, device: &dyn GraphicsDevice,
    backend: &dyn ShaderBackend, meshes: &AssetMeshLibrary, watch: &MeshWatch,
    shaders: &AssetShaderLibrary, placements: &mut AssetPlacementDataLibrary,
    ssbos: &mut AssetSsboLibrary, count: &mut LoadedParticleEmitterCount,
    forest: &mut InteractableForestPlacements, commands: &mut Commands,
) -> anyhow::Result<()> {
    anyhow::bail!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: country-core/startup/clearing::init_clearing_complete");
}
pub fn init_clearing_meshes(meshes: Res<AssetMeshLibrary>) {
    panic!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: country-core/startup/clearing::init_clearing_meshes");
}
// Scheduled-only adapter signatures do not claim the missing native SystemParam
// lists; every call fails before a lifecycle side effect can be fabricated.
pub fn begin_glade_unload() {
    panic!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: country-core/startup/clearing::begin_glade_unload");
}
pub fn uninit_clearing<S: Studio + Send + Sync + 'static>() {
    panic!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: country-core/startup/clearing::uninit_clearing");
}
pub fn despawn_glade_entities() {
    panic!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: country-core/startup/clearing::despawn_glade_entities");
}
