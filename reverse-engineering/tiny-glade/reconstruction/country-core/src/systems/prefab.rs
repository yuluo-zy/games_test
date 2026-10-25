//! `systems/prefab.rs` — prefab spawn / update / despawn / hot-reload systems.
//!
//! PDB: `PrefabPlugin::build @0x140840430`, `spawn_prefabs @0x140840610`,
//! `create_prefab_rt @0x140841550`, `create_prefab_gpu_instances @0x140841b00`,
//! `update_prefabs @0x140841d60`, `despawn_prefabs @0x140842d60`,
//! `despawn_prefab_contents @0x140843390`, `hot_reload_prefabs @0x140843740`,
//! `register_required_components @0x1408446b0`. Named entities: `PrefabPlugin`,
//! `SpawnPrefabCmd`, `DespawnPrefabCmd`, `PrefabInstanceData`, `PrefabsToDelete`.
//!
//! O: `spawn_prefabs` consumes `SpawnPrefabCmd` → `PrefabLibrary::get<PrefabName>`
//! → spawns the entity with `(PrefabInstance, PrefabInstanceData)`; `update_
//! prefabs` refreshes the GPU instance data (`update_instance_data<
//! PlacementDataSsbo>`); `despawn_prefabs` consumes `DespawnPrefabCmd` →
//! `despawn_prefab_contents()` + `EntityCommands::despawn`; `hot_reload_prefabs`
//! reloads prefab assets on watch events.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::resources::placement_data::PlacementDataSsbo;
use crate::resources::prefab::{PrefabInstance, PrefabInstanceData, PrefabLibrary};
use crate::Gap;

/// `SpawnPrefabCmd` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct SpawnPrefabCmd {
    pub _gap: Gap,
}

/// `DespawnPrefabCmd` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct DespawnPrefabCmd {
    pub _gap: Gap,
}

/// `PrefabsToDelete` (O named entity; resource).
#[derive(Resource, Clone, Debug, Default)]
pub struct PrefabsToDelete {
    pub raw: Gap,
}

/// `spawn_prefabs` @0x140840610 (O).
pub fn spawn_prefabs(
    mut _spawns: EventReader<SpawnPrefabCmd>,
    _library: Res<PrefabLibrary>,
    mut _commands: Commands,
) {
    todo!("recover spawn_prefabs (PrefabLibrary::get + insert (PrefabInstance, PrefabInstanceData))")
}

/// `update_prefabs` @0x140841d60 (O).
pub fn update_prefabs(
    mut _instances: Query<(&PrefabInstance, &mut PrefabInstanceData)>,
    mut _placement: ResMut<PlacementDataSsbo>,
) {
    todo!("recover update_prefabs (update_instance_data<PlacementDataSsbo>)")
}

/// `despawn_prefabs` @0x140842d60 (O).
pub fn despawn_prefabs(
    mut _despawns: EventReader<DespawnPrefabCmd>,
    mut _delete: ResMut<PrefabsToDelete>,
    mut _commands: Commands,
) {
    todo!("recover despawn_prefabs (despawn_prefab_contents + EntityCommands::despawn)")
}

/// `despawn_prefab_contents` @0x140843390 (O).
pub fn despawn_prefab_contents(_entity: Entity, _commands: &mut Commands) {
    todo!("recover despawn_prefab_contents")
}

/// `hot_reload_prefabs` @0x140843740 (O).
pub fn hot_reload_prefabs(
    _watch: Res<Gap>,
    _library: ResMut<PrefabLibrary>,
    _commands: Commands,
) {
    todo!("recover hot_reload_prefabs (prefab asset watch reload)")
}

/// `PrefabPlugin` (O named entity).
pub struct PrefabPlugin;

impl PrefabPlugin {
    /// `build` @0x140840430 (O).
    pub fn build(&self, _app: &mut crate::reused::bevy_app::App) {
        todo!("recover PrefabPlugin::build (systems + register_required_components)")
    }
}
