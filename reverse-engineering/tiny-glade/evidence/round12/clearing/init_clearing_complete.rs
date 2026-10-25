//! Complete B clearing producer candidate; append to startup/clearing.rs.
//! Native6913B main +521B particle closure +500B forest filter map.
//! The theme helpers and canonical forest types are separate owner candidates.

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
