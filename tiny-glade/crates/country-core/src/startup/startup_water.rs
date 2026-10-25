//! B startup_water @1408f1a20/3632B: real water, ice and aquatic vegetation
//! producers. Original mesh files, shader registration order, seasonal choices,
//! mesh sharing and ECS bundles are preserved. The caller supplies its actual
//! installation root, original process flag, ActiveSession.meta.glade and App
//! libraries; no independent render device, resource table or draw list is made.
use std::path::Path;

use bevy_ecs::system::Commands;

use crate::{
    asset_libraries::{
        Handle,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        meshwatch::MeshWatch,
        shader_library::AssetShaderLibrary,
    },
    components::drawable::DrawableMeshBundle,
    render::{
        mesh::Mesh,
        shader::{RasterShaderProgram, boundary::ShaderBackend, compile_shader_variants},
    },
    resources::session::GladeType,
    rhapsody_types::RasterPipelineDesc,
    systems::render_loop::draw_passes::draw_lists::{DesignatedRenderPass, DontCastShadow},
    utils::{components::transform::Transform, load_json::try_load_json_as_mesh},
};

pub const WATER_MESH: (&str, &str) = ("assets/meshes/water_plane.json", "water_plane");
pub const WATER_SHADER: (&str, &str) = ("/water_plane.raster.hlsl", "water_plane");
pub const ICE_SHADER: (&str, &str) = ("/ice_plane.raster.hlsl", "ice_plane");
pub const REFRACTION_SHADER: (&str, &str) = (
    "/water_plane_refraction_copy.raster.hlsl",
    "water_plane_refraction_copy",
);
pub const SNOW_SHADER: (&str, &str) = ("/water_snow_plane.raster.hlsl", "water_snow_plane");
pub const FLOWER_SHADER: (&str, &str) = ("/lilypad_flower.raster.hlsl", "lilypad_flower_shader");
pub const REED_SHADER: (&str, &str) = ("/reed.raster.hlsl", "reed_shader");
pub const PATH_SHADER: (&str, &str) = ("/water_x_paths.raster.hlsl", "water_x_paths");

fn install_mesh(
    root: &Path,
    (path, name): (&str, &str),
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
) -> anyhow::Result<Handle<Mesh>> {
    let path = root.join(path);
    let path = path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("water mesh path is not UTF-8: {}", path.display()))?;
    let mesh = try_load_json_as_mesh(path)?;
    // Common original load_mesh_into_library @140b1aca0 watches first, adds
    // second. Current asset maintenance subsequently derives this Mesh's VAO.
    watch.watch(&mesh);
    Ok(meshes.add(MeshAsset::new(mesh, Some(name.to_owned()))))
}

fn install_shader(
    (path, name): (&str, &'static str),
    replace: bool,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
) -> anyhow::Result<Handle<RasterShaderProgram>> {
    let desc = RasterPipelineDesc {
        path: path.to_owned(),
        entry: String::new(),
        defines: Vec::new(),
    };
    let variants = compile_shader_variants(&desc, backend)?;
    // Original normal @140b3fe40 passes false. Replace-previous @140b405e0
    // passes r8b=1 at140b4085e to the same AssetShaderLibrary.add_raster.
    Ok(shaders.add_raster(
        RasterShaderProgram {
            name,
            desc,
            variants,
        },
        replace,
    ))
}

fn drawable(mesh: Handle<Mesh>, shader: Handle<RasterShaderProgram>) -> DrawableMeshBundle {
    DrawableMeshBundle {
        mesh,
        shader,
        transform: Transform::identity(),
    }
}

/// Native Res<ActiveSession> contributes only its glade byte (@+0x58), and
/// global143634d90 is the early-return flag. `demo` is the explicit same-process
/// flag used by startup_terrain; callers must also publish it to the existing
/// WATER_PLANES_DISABLED callback flag. Original required file/compile unwraps
/// become contextual Results in this host adaptation, never an empty mesh.
#[allow(clippy::too_many_arguments)]
pub fn startup_water(
    root: &Path,
    demo: bool,
    glade: GladeType,
    backend: &dyn ShaderBackend,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
    shaders: &AssetShaderLibrary,
    commands: &mut Commands,
) -> anyhow::Result<()> {
    if demo {
        return Ok(());
    }

    let water = install_mesh(root, WATER_MESH, meshes, watch)?;
    let shader = install_shader(
        if glade.is_winter() {
            ICE_SHADER
        } else {
            WATER_SHADER
        },
        false,
        shaders,
        backend,
    )?;
    // @1408f1bc8 -> command1412dc470's PDB tuple includes DontCastShadow
    // and DesignatedRenderPass. Both actual surface branches use list15.
    commands.spawn((
        drawable(water, shader),
        DontCastShadow,
        DesignatedRenderPass(15),
    ));
    let shader = install_shader(REFRACTION_SHADER, false, shaders, backend)?;
    commands.spawn((
        drawable(water, shader),
        DontCastShadow,
        DesignatedRenderPass(14),
    ));

    if glade.is_winter() {
        let shader = install_shader(SNOW_SHADER, false, shaders, backend)?;
        // command1412d9de0 contains DontCastShadow but no designated pass.
        commands.spawn((drawable(water, shader), DontCastShadow));
    }

    // @1408f1e2c: wrapping_sub(3)<2 selects Autumn and Winter together.
    // These seasons intentionally publish reed code under lilypad_shader.
    let fallen_leaves = glade.is_autumn() || glade.is_winter();
    let shader = install_shader(
        (
            if fallen_leaves {
                "/reed.raster.hlsl"
            } else {
                "/lilypad.raster.hlsl"
            },
            "lilypad_shader",
        ),
        true,
        shaders,
        backend,
    )?;
    let mesh = install_mesh(
        root,
        (
            if fallen_leaves {
                "assets/meshes/fallen_leaves.json"
            } else {
                "assets/meshes/lilypad.json"
            },
            "lilypad_or_fallen_leaves",
        ),
        meshes,
        watch,
    )?;
    // command1412dd460's PDB payload is exactly DrawableMeshBundle. No
    // guessed no-shadow, designated pass or persistence marker is added.
    commands.spawn(drawable(mesh, shader));

    let shader = install_shader(FLOWER_SHADER, false, shaders, backend)?;
    let mesh = install_mesh(
        root,
        (
            if glade.is_winter() {
                "assets/meshes/water_grass_winter.json"
            } else {
                "assets/meshes/lilypad_flower.json"
            },
            "lilypad_flower",
        ),
        meshes,
        watch,
    )?;
    commands.spawn(drawable(mesh, shader));

    let shader = install_shader(REED_SHADER, false, shaders, backend)?;
    let mesh = install_mesh(
        root,
        (
            if glade.is_autumn() {
                "assets/meshes/reed_autumn.json"
            } else {
                "assets/meshes/reed.json"
            },
            "reed",
        ),
        meshes,
        watch,
    )?;
    commands.spawn(drawable(mesh, shader));

    let shader = install_shader(PATH_SHADER, false, shaders, backend)?;
    let mesh = install_mesh(
        root,
        ("assets/meshes/water_stones.json", "water_stones"),
        meshes,
        watch,
    )?;
    commands.spawn(drawable(mesh, shader));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::test_device::TestDevice;
    use bevy_ecs::world::{CommandQueue, World};

    fn original_assets() -> std::path::PathBuf {
        std::env::var_os("TG_INSTALL_ROOT")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../ljxsj_92385/Tiny Glade"))
    }

    #[test]
    fn shipped_water_startup_populates_real_seasonal_meshes_and_surface_entities() {
        for glade in GladeType::ALL {
            let meshes = AssetMeshLibrary::new();
            let watch = MeshWatch::new(false);
            let shaders = AssetShaderLibrary::new();
            let mut world = World::new();
            let mut queue = CommandQueue::default();
            startup_water(
                &original_assets(),
                false,
                glade,
                &TestDevice::default(),
                &meshes,
                &watch,
                &shaders,
                &mut Commands::new(&mut queue, &world),
            )
            .unwrap();
            queue.apply(&mut world);
            let water = meshes.get_handle_by_name("water_plane");
            assert!(!meshes.get(water).unwrap().indices.is_empty());
            let mut query = world.query::<(
                &Handle<Mesh>,
                &Handle<RasterShaderProgram>,
                &Transform,
                Option<&DontCastShadow>,
                Option<&DesignatedRenderPass>,
            )>();
            let rows: Vec<_> = query
                .iter(&world)
                .map(|(mesh, shader, transform, no_shadow, pass)| {
                    assert_eq!(*transform, Transform::identity());
                    assert!(!meshes.get(*mesh).unwrap().indices.is_empty());
                    (
                        *mesh,
                        shaders.get_raster(*shader).unwrap().name,
                        no_shadow.is_some(),
                        pass.map(|p| p.0),
                    )
                })
                .collect();
            assert_eq!(rows.len(), if glade.is_winter() { 7 } else { 6 });
            assert!(rows.contains(&(
                water,
                if glade.is_winter() {
                    "ice_plane"
                } else {
                    "water_plane"
                },
                true,
                Some(15)
            )));
            assert!(rows.contains(&(water, "water_plane_refraction_copy", true, Some(14))));
            assert_eq!(rows.iter().filter(|r| r.3.is_some()).count(), 2);
            assert_eq!(
                rows.iter().filter(|r| r.2).count(),
                if glade.is_winter() { 3 } else { 2 }
            );
            if glade.is_winter() {
                assert!(rows.contains(&(water, "water_snow_plane", true, None)));
            }
            for (name, file) in [
                (
                    "lilypad_or_fallen_leaves",
                    if glade.is_autumn() || glade.is_winter() {
                        "fallen_leaves.json"
                    } else {
                        "lilypad.json"
                    },
                ),
                (
                    "lilypad_flower",
                    if glade.is_winter() {
                        "water_grass_winter.json"
                    } else {
                        "lilypad_flower.json"
                    },
                ),
                (
                    "reed",
                    if glade.is_autumn() {
                        "reed_autumn.json"
                    } else {
                        "reed.json"
                    },
                ),
                ("water_stones", "water_stones.json"),
            ] {
                let mesh = meshes.get_by_name(name).unwrap();
                assert!(mesh.source_file.as_ref().unwrap().path.ends_with(file));
            }
            let lily = shaders.get_raster_handle_by_name("lilypad_shader").unwrap();
            assert_eq!(
                shaders.get_raster(lily).unwrap().desc.path,
                if glade.is_autumn() || glade.is_winter() {
                    "/reed.raster.hlsl"
                } else {
                    "/lilypad.raster.hlsl"
                }
            );
        }
    }

    #[test]
    fn original_demo_branch_returns_before_any_required_file_or_shader() {
        let meshes = AssetMeshLibrary::new();
        let watch = MeshWatch::new(false);
        let shaders = AssetShaderLibrary::new();
        let world = World::new();
        let mut queue = CommandQueue::default();
        startup_water(
            Path::new("/missing-required-assets"),
            true,
            GladeType::Winter,
            &TestDevice::default(),
            &meshes,
            &watch,
            &shaders,
            &mut Commands::new(&mut queue, &world),
        )
        .unwrap();
        assert!(meshes.try_get_handle_by_name("water_plane").is_none());
        assert!(shaders.get_raster_handle_by_name("ice_plane").is_none());
        assert!(queue.is_empty());
    }

    #[test]
    fn autumn_replaces_the_lilypad_shader_without_rewriting_older_entity_handles() {
        let meshes = AssetMeshLibrary::new();
        let watch = MeshWatch::new(false);
        let shaders = AssetShaderLibrary::new();
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        startup_water(
            &original_assets(),
            false,
            GladeType::Summer,
            &TestDevice::default(),
            &meshes,
            &watch,
            &shaders,
            &mut Commands::new(&mut queue, &world),
        )
        .unwrap();
        queue.apply(&mut world);
        let previous = shaders.get_raster_handle_by_name("lilypad_shader").unwrap();
        startup_water(
            &original_assets(),
            false,
            GladeType::Autumn,
            &TestDevice::default(),
            &meshes,
            &watch,
            &shaders,
            &mut Commands::new(&mut queue, &world),
        )
        .unwrap();
        queue.apply(&mut world);
        let current = shaders.get_raster_handle_by_name("lilypad_shader").unwrap();
        assert_ne!(previous, current);
        assert_eq!(
            shaders.get_raster(previous).unwrap().desc.path,
            "/lilypad.raster.hlsl"
        );
        assert_eq!(
            shaders.get_raster(current).unwrap().desc.path,
            "/reed.raster.hlsl"
        );
        let mut query = world.query::<&Handle<RasterShaderProgram>>();
        assert_eq!(query.iter(&world).filter(|h| **h == previous).count(), 1);
        assert_eq!(query.iter(&world).filter(|h| **h == current).count(), 1);
    }
}
