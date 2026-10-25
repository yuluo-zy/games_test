//! Original clearing lifecycle: complete per-session producer below, plus
//! the canonical unload/despawn owners. Gameplay registers init_clearing_complete
//! on every OnEnter(Game), without run_once.
use crate::resources::session::ActiveSession;
use super::StartupAssetHost;

pub const CLEARING_UNLOAD_SNAPSHOT: &str = "snapshot:/Loading";
pub const BILLBOARD_MESH: (&str, &str) = (
    "assets/meshes/distant_billboard.json",
    "distant_billboard",
);
pub const ICE_CRACKS_MESH: (&str, &str) = ("assets/meshes/ice_cracks.json", "ice_cracks");
pub const ICE_CRACKS_SHADER: (&str, &str) = ("/water/ice_cracks.raster.hlsl", "ice_cracks");

/// O：容量 = `min(x, 0x44c)`。
pub const CLEARING_CAPACITY: usize = 0x44c;
/// O：卸载遍历 27 × 0xc0 花园槽，置 +0x38 = 1。
pub const UNINIT_GARDEN_SLOTS: usize = 27;
pub const UNINIT_GARDEN_SLOT_SIZE: usize = 0xc0;
pub const UNINIT_GARDEN_FLAG_OFFSET: usize = 0x38;
/// O：init 的 spawn 记录数。
pub const CLEARING_SPAWN_RECORDS: usize = 2;

/// 无条件 billboard；冬季追加 ice_cracks 与 shader。
pub fn clearing_asset_inventory<H: StartupAssetHost>(host: &mut H, is_winter: bool) {
    host.load_mesh(BILLBOARD_MESH.0, Some(BILLBOARD_MESH.1));
    if is_winter {
        host.load_mesh(ICE_CRACKS_MESH.0, Some(ICE_CRACKS_MESH.1));
        host.load_shader(ICE_CRACKS_SHADER.0, ICE_CRACKS_SHADER.1);
    }
    log::info!(
        target: "country_core::startup::clearing",
        "Initializing the glade."
    );
    // Asset inventory only; full spawning and theme policy are below.
}

/// 卸载快照常量；FMOD 调用由音频域消费（`game_loading_audio` 已有同一字符串）。
pub fn uninit_clearing_snapshot() -> &'static str {
    CLEARING_UNLOAD_SNAPSHOT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        meshes: Vec<(&'static str, Option<&'static str>)>,
        shaders: Vec<(&'static str, &'static str)>,
    }
    impl StartupAssetHost for Rec {
        type Mesh = u64;
        type Shader = u64;
        fn load_mesh(&mut self, path: &'static str, name: Option<&'static str>) -> u64 {
            self.meshes.push((path, name));
            1
        }
        fn load_shader(&mut self, path: &'static str, name: &'static str) -> u64 {
            self.shaders.push((path, name));
            2
        }
        fn spawn_drawable(&mut self, _: u64, _: u64) {}
    }

    #[test]
    fn winter_adds_ice_cracks_and_the_shader_while_summer_does_not() {
        let mut summer = Rec::default();
        clearing_asset_inventory(&mut summer, false);
        assert_eq!(
            summer.meshes,
            [(BILLBOARD_MESH.0, Some(BILLBOARD_MESH.1))].to_vec()
        );
        assert!(summer.shaders.is_empty());

        let mut winter = Rec::default();
        clearing_asset_inventory(&mut winter, true);
        assert_eq!(winter.meshes.len(), 2);
        assert_eq!(winter.shaders, [ICE_CRACKS_SHADER].to_vec());
        assert_eq!(CLEARING_CAPACITY, 1100);
        assert_eq!(UNINIT_GARDEN_SLOTS * UNINIT_GARDEN_SLOT_SIZE, 0x1440);
        assert_eq!(uninit_clearing_snapshot(), "snapshot:/Loading");
        assert_eq!(
            crate::systems::audio::game_loading_audio::LOADING_SNAPSHOT,
            CLEARING_UNLOAD_SNAPSHOT
        );
    }
}


/// First OnExit group, original closure140b3c930: enqueue false before closing
/// the actual save channel/file. Do not replace this with resource defaults.
pub fn begin_glade_unload(
    mut clear: bevy_ecs::prelude::EventWriter<crate::systems::mode_manager::ClearCanvasCmd>,
    mut autosave: bevy_ecs::prelude::ResMut<crate::systems::save::save_game::AutoSaveState>,
) {
    clear.write(crate::systems::mode_manager::ClearCanvasCmd(false));
    autosave.close_file();
}

/// Original eight-owner uninit140a7a5b0. AudioCore is read-only; every other
/// owner receives precisely its observed clear/flag policy.
#[allow(clippy::too_many_arguments)]
pub fn uninit_clearing<S: crate::resources::audio_resources::Studio+Send+Sync+'static>(
    mut placement: bevy_ecs::prelude::ResMut<crate::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,
    mut triggers: bevy_ecs::prelude::ResMut<crate::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,
    mut rt: bevy_ecs::prelude::ResMut<crate::resources::rt::RtWorld>,
    mut gpu: bevy_ecs::prelude::ResMut<crate::render::nani::gpu_world::GpuWorld>,
    mut water: Option<bevy_ecs::prelude::ResMut<crate::systems::audio::water_ambience_audio::WaterAudioGrid>>,
    mut forest: bevy_ecs::prelude::ResMut<crate::resources::tree_storage::InteractableForestPlacements>,
    audio: bevy_ecs::prelude::Res<crate::resources::audio_resources::AudioCore<S>>,
    mut loading: bevy_ecs::prelude::ResMut<crate::systems::audio::game_loading_audio::GameLoadingAudioState<S::Event>>,
) {
    placement.clear();
    triggers.clear();
    rt.clear_meshes();
    gpu.clear_owned_instances();
    forest.clear();
    if let Some(water)=water.as_deref_mut() {
        water.clear_activity();
    }
    loading.arm_after_unload(&audio);
}

/// Native1408b3510 query filter is established by140a04980/140b3f550,
/// rather than the standalone void() placeholder. Commands owns deferred work.
pub fn despawn_glade_entities(
    entities: bevy_ecs::prelude::Query<bevy_ecs::entity::Entity,
        bevy_ecs::prelude::Without<crate::components::KeepOnLevelUnload>>,
    mut commands: bevy_ecs::prelude::Commands,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}


// Complete B clearing producer candidate; append to startup/clearing.rs.
// Native6913B main +521B particle closure +500B forest filter map.
// The theme helpers and canonical forest types are separate owner candidates.

#[path = "native_placement_policy.rs"]
mod native_placement_policy;

#[allow(clippy::too_many_arguments)]
pub fn init_clearing_complete(
    installed: &std::path::Path,
    session: &crate::resources::session::ActiveSession,
    device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
    backend: &dyn crate::render::shader::boundary::ShaderBackend,
    meshes: &crate::asset_libraries::mesh_library::AssetMeshLibrary,
    watch: &crate::asset_libraries::meshwatch::MeshWatch,
    shaders: &crate::asset_libraries::shader_library::AssetShaderLibrary,
    placements: &mut crate::asset_libraries::placement_data_library::AssetPlacementDataLibrary,
    ssbos: &mut crate::asset_libraries::ssbo_library::AssetSsboLibrary,
    emitter_count: &mut crate::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount,
    forest: &mut crate::resources::tree_storage::InteractableForestPlacements,
    commands: &mut bevy_ecs::prelude::Commands,
) -> anyhow::Result<()> {
    use crate::{
        asset_libraries::mesh_library::MeshAsset,
        components::drawable::DrawableMeshBundle,
        resources::{
            preplaced_autoclutter::PreplacedAutoClutter, render::mesh_atlas_library::PrefabName,
            session::GladeType, tree_storage::ForestTree,
        },
        systems::render_loop::compute_passes::leaf_particles::ParticleEmitterSsbo,
        utils::components::transform::Transform,
    };
    use anyhow::Context;

    fn load_mesh(
        installed: &std::path::Path,
        relative: &str,
        name: &str,
        meshes: &crate::asset_libraries::mesh_library::AssetMeshLibrary,
        watch: &crate::asset_libraries::meshwatch::MeshWatch,
    ) -> anyhow::Result<crate::asset_libraries::Handle<crate::render::mesh::Mesh>> {
        let path = installed.join(relative);
        let mesh = crate::utils::load_json::try_load_json_as_mesh(
            path.to_str().context("clearing mesh path is not UTF-8")?,
        )?;
        watch.watch(&mesh);
        Ok(meshes.add(MeshAsset::new(mesh, Some(name.to_owned()))))
    }

    log::info!(target: "country_core::startup::clearing", "Initializing the glade.");
    if session.meta.glade == GladeType::Winter {
        let mesh = load_mesh(installed, BILLBOARD_ICE_PATH, "ice_cracks", meshes, watch)?;
        let shader = super::load_shaders::load_shader_into_library(
            shaders,
            backend,
            "/water/ice_cracks.raster.hlsl",
            "ice_cracks",
        );
        commands.spawn(DrawableMeshBundle {
            mesh,
            shader,
            transform: Transform::identity(),
        });
    }

    // The original Olden helper returns the whole resource. Other themes
    // remove it; Flowery's additional scene entities are created first.
    match session.meta.glade {
        GladeType::Olden => {
            let clutter = super::glade_themes::startup_olden(
                installed, placements, device, session, commands, watch, meshes, shaders,
            )?;
            commands.insert_resource(clutter);
        }
        GladeType::Flowery => {
            super::glade_themes::startup_flowery(
                installed, placements, device, session, commands, watch, meshes, shaders,
            )?;
            commands.remove_resource::<PreplacedAutoClutter>();
        }
        _ => {
            commands.remove_resource::<PreplacedAutoClutter>();
        }
    }
    let glade_root = if session.meta.glade == GladeType::Olden {
        installed.join("assets/glade/olden")
    } else {
        installed.join(session.assets_root())
    };
    let groups = super::spawn_preplaced_trees::load_tree_groups(glade_root.join("trees.json"))
        .context("deserialize trees")?;
    let mut emitters = Vec::<ParticleEmitterSsbo>::new();
    // Release every previous Arc before replacing the new session records.
    forest.clear();
    for group in groups {
        if group.prefab == "clearing/tree/bush" {
            continue;
        }
        let path = relative_path::RelativePath::new(&group.placements).to_path(&glade_root);
        let (records, _handle) = placements.load(device, path);
        let prefab_name = PrefabName::new(&group.prefab);
        forest.records.extend(records.iter().filter_map(|record| {
            native_placement_policy::is_interactable_forest_placement(record.position).then(|| {
                ForestTree {
                    prefab_name: prefab_name.clone(),
                    placement: *record,
                    entity: None,
                    billboard_expansion_scale: group.billboard_expansion_scale,
                }
            })
        }));
        if let Some(emitter_mesh) = group.particle_emitter {
            // Native RelativePath::to_path joins assets/meshes directly; do
            // not invent a suffix or emit from the prefab's visible mesh.
            let path = relative_path::RelativePath::new(&emitter_mesh)
                .to_path(installed.join("assets/meshes"));
            let mesh = crate::utils::load_json::try_load_json_as_mesh(
                path.to_str()
                    .context("particle emitter path is not UTF-8")?,
            )?;
            for record in &records {
                for &vertex in mesh.positions_without_indices() {
                    if let Some(words) = native_placement_policy::particle_emitter_for_vertex(
                        record.position,
                        record.scale,
                        record.rotation_angle,
                        vertex,
                    ) {
                        emitters.push(ParticleEmitterSsbo { words });
                    }
                }
            }
        }
    }
    // Upload the complete native vector. Only the published count is capped;
    // silently truncating the upload would discard the original error branch.
    if let Some(storage) = ssbos.get_mut::<ParticleEmitterSsbo>() {
        if !emitters.is_empty() {
            let buffer = &storage.buffer;
            let bytes: &[u8] = bytemuck::cast_slice(&emitters);
            anyhow::ensure!(
                bytes.len() as u64 <= buffer.byte_size,
                "particle emitter buffer is too small"
            );
            device.update_buffer_bytes(buffer, bytes, &[0..bytes.len()], 0)?;
        }
    }
    emitter_count.0 = emitters.len().min(1100) as u64;
    // The common shader is generated from the actual original format pieces:
    // ["/clearing/", ".raster.hlsl"] and ["", "_shader"].
    let shader = super::load_shaders::load_shader_into_library(
        shaders,
        backend,
        "/clearing/distant_billboard.raster.hlsl",
        "distant_billboard_shader",
    );
    let mesh = load_mesh(
        installed,
        "assets/meshes/distant_billboard.json",
        "distant_billboard",
        meshes,
        watch,
    )?;
    commands.spawn(DrawableMeshBundle {
        mesh,
        shader,
        transform: Transform::identity(),
    });
    Ok(())
}

const BILLBOARD_ICE_PATH: &str = "assets/meshes/ice_cracks.json";
