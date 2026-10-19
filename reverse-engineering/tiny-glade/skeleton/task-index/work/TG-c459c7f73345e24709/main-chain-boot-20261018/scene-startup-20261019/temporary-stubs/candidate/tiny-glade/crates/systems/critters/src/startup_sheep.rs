//! TEMPORARY USER-AUTHORIZED COMPILATION STUB. Original source awaits sync.
//! This file is not recovered gameplay. Every callable entry fails before
//! creating assets, state, entities or a successful readiness result.
#![allow(unused_variables)]

use bevy_ecs::system::Commands;
use country_core::{
    asset_libraries::{mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary},
    resources::critters::sheep::control::SheepControlSnapshot,
    startup::sheep_assets::SheepStartupAssets,
};
use crate::{audio::AudioHandle, bird_spots::BirdSpotManager};
pub fn spawn_startup_sheep<F>(
    assets: &SheepStartupAssets, meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary, birds: &mut BirdSpotManager,
    commands: &mut Commands, snapshot: Option<SheepControlSnapshot>,
    instantiate: F,
) -> Result<(), String>
where F: FnMut(&str) -> Result<AudioHandle, String> {
    Err("ORIGINAL_IMPLEMENTATION_PENDING_SYNC: system-critters/startup_sheep".to_owned())
}
