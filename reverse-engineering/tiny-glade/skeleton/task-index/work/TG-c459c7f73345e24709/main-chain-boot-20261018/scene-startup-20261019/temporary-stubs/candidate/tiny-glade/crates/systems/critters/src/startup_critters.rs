//! TEMPORARY USER-AUTHORIZED COMPILATION STUB. Original source awaits sync.
//! This file is not recovered gameplay. Every callable entry fails before
//! creating assets, state, entities or a successful readiness result.
#![allow(unused_variables)]

use std::path::Path;
use bevy_ecs::system::Commands;
use country_core::{
    asset_libraries::{mesh_library::AssetMeshLibrary, meshwatch::MeshWatch,
        shader_library::AssetShaderLibrary},
    render::shader::boundary::ShaderBackend,
    resources::{audio_resources::{AudioCore, Studio}, glade_settings::GladeSettings,
        session::ActiveSession},
};
pub fn startup_critters<S: Studio>(
    meshes: &AssetMeshLibrary, shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend, watch: &MeshWatch, root: &Path,
    audio: &AudioCore<S>, settings: &GladeSettings, session: &ActiveSession,
    commands: &mut Commands,
) {
    panic!("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: system-critters/startup_critters");
}
