//! B's full SSBO producer on a real App PreStartup and actual GPU owners.
//! This validates the 72-entry producer and its cursor event, not the remaining
//! mesh/prefab/startup tuple or a complete gameplay world.
use ash::vk::Handle;
use country_core::{
    asset_libraries::{shader_library::AssetShaderLibrary, ssbo_library::AssetSsboLibrary},
    resources::mouse_capture::CustomCursorSsboCreatedCmd,
    systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo,
};
use std::{path::PathBuf, sync::Arc};
use tiny_glade::render_bridge::CoreRenderBridge;

#[test]
fn original_full_ssbo_prestartup_uses_real_owners_and_one_cursor_event() {
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        });
    let device =
        match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "strict full startup GPU initialization failed: {error}"
                );
                eprintln!("SKIP full startup GPU initialization: {error}");
                return;
            }
        };
    let shaders = Arc::new(AssetShaderLibrary::new());
    let bridge =
        CoreRenderBridge::new(device.clone(), installed.canonicalize().unwrap(), shaders).unwrap();
    let mut app = bevy_app::App::new();
    bridge.install_asset_libraries(app.world_mut()).unwrap();
    app.add_event::<CustomCursorSsboCreatedCmd>();
    app.add_systems(
        bevy_app::PreStartup,
        country_core::startup::init_populate_ssbo_library::init_populate_ssbo_library,
    );
    assert!(
        app.world()
            .resource::<AssetSsboLibrary>()
            .snapshot_graph_leases()
            .is_empty()
    );
    let mut cursor_events = bevy_ecs::event::EventCursor::<CustomCursorSsboCreatedCmd>::default();
    app.update();
    let cursor = {
        let library = app.world().resource::<AssetSsboLibrary>();
        let entries = library.snapshot_graph_leases();
        assert_eq!(entries.len(), 72);
        let mut unique = std::collections::HashSet::new();
        for entry in &entries {
            let lease = bridge
                .hosts()
                .buffers
                .managed_buffer(entry.buffer())
                .unwrap();
            assert!(Arc::ptr_eq(&lease.device(), &device));
            assert_eq!(lease.size, entry.buffer().byte_size);
            assert!(lease.size > 0);
            assert!(
                unique.insert(lease.raw().as_raw()),
                "canonical entries must own their actual buffers"
            );
        }
        *library.get_buffer::<CustomCursorSsbo>().unwrap()
    };
    let events: Vec<_> = cursor_events
        .read(
            app.world()
                .resource::<bevy_ecs::event::Events<CustomCursorSsboCreatedCmd>>(),
        )
        .copied()
        .collect();
    assert_eq!(events.len(), 1);
    let cursor_owner = bridge.hosts().buffers.managed_buffer(&cursor).unwrap();
    assert_eq!(cursor_owner.size, 16);
    assert_eq!(
        Some(events[0].0.as_ptr().cast::<u8>()),
        cursor_owner.persistent_host_ptr()
    );
    app.update();
    assert_eq!(
        app.world()
            .resource::<AssetSsboLibrary>()
            .snapshot_graph_leases()
            .len(),
        72
    );
    assert_eq!(
        cursor_events
            .read(
                app.world()
                    .resource::<bevy_ecs::event::Events<CustomCursorSsboCreatedCmd>>()
            )
            .count(),
        0
    );

    // Initial upload byte contents are checked by the canonical producer's
    // recording-device test. GPU_ONLY buffers are not host-mappable, even on
    // this unified-memory GPU. Their contents must be validated by actual
    // consumers; this fixture establishes physical owners and the cursor map.
}

/// Actual shipped prefabs -> their processed mesh Arcs -> registered atlas ->
/// real Nani subsets/drawables on this App's single physical device. No session
/// loader, frame graph or gameplay-ready signal is substituted by this test.
#[test]
fn original_gameplay_assets_reach_the_same_app_atlases_and_nani_gpu_owners() {
    use country_core::{
        asset_libraries::{configwatch::ConfigWatch, mesh_library::AssetMeshLibrary},
        game_config::GameConfig,
        render::nani::{
            GpuDrivenDrawable,
            gpu_world::{GpuWorld, GpuWorldSubsetKind},
        },
        resources::{prefab::PrefabLibrary, render::mesh_atlas_library::MeshAtlasLibrary},
    };
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        });
    let device =
        match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "strict gameplay startup GPU initialization failed: {error}"
                );
                eprintln!("SKIP gameplay startup GPU initialization: {error}");
                return;
            }
        };
    let shaders = Arc::new(AssetShaderLibrary::new());
    let bridge = CoreRenderBridge::new(
        device.clone(),
        installed.canonicalize().unwrap(),
        shaders.clone(),
    )
    .unwrap();
    // Same real COMMON13 producer as ApplicationGpu::new, before gameplay
    // Startup consumes its shader names. A bare bridge owns tables only.
    let mut final_post = None;
    country_core::startup::load_shaders::load_shaders(
        &shaders,
        bridge.shader_backend(),
        &mut final_post,
    );
    let mut app = bevy_app::App::new();
    bridge.install_asset_libraries(app.world_mut()).unwrap();
    app.insert_resource(GameConfig::default());
    app.insert_resource(ConfigWatch::new(false));
    app.insert_resource(tiny_glade::run_game::ApplicationShaderLibrary(shaders));
    app.add_event::<CustomCursorSsboCreatedCmd>();
    tiny_glade::gameplay_startup::install_gameplay_gpu_startup(
        &mut app,
        device,
        bridge.hosts().clone(),
        installed.canonicalize().unwrap(),
    )
    .unwrap();
    tiny_glade::world_terrain::install_terrain_startup(
        &mut app,
        bridge.hosts().shaders.clone(),
        installed.canonicalize().unwrap(),
    );
    app.update();
    let prefabs = app.world().resource::<PrefabLibrary>();
    assert!(
        !prefabs.inner.is_empty(),
        "original prefab source must actually be loaded"
    );
    let atlases = app.world().resource::<MeshAtlasLibrary>();
    let meshes = app.world().resource::<AssetMeshLibrary>();
    let gpu = app.world().resource::<GpuWorld>();
    let mut parts = 0usize;
    for (name, prefab) in prefabs.iter() {
        let expected: Vec<_> = prefab
            .parts
            .iter()
            .filter(|p| !p.render_mesh.indices.is_empty())
            .collect();
        if expected.is_empty() {
            continue;
        }
        let actual = atlases.get_by_name(name);
        assert_eq!(actual.len(), expected.len(), "{name:?}");
        for (part, registration) in expected.into_iter().zip(actual) {
            assert_eq!(registration.subset, part.render_subset);
            let subset = atlases.get_by_world_subset(part.render_subset);
            assert_eq!(
                subset.atlas.layouts[registration.part_index as usize].count as usize,
                part.render_mesh.indices.len(),
                "{name:?}/{}",
                part.name
            );
            assert!(
                subset
                    .registered
                    .is_some_and(|handle| meshes.get(handle).is_some())
            );
            parts += 1;
        }
    }
    assert!(parts > 0);
    let mut registered = 0usize;
    for kind in GpuWorldSubsetKind::ALL {
        let atlas = atlases.get_by_world_subset(kind);
        assert_eq!(
            gpu.get_subset(kind).is_some(),
            atlas.registered.is_some(),
            "{kind:?}"
        );
        registered += usize::from(atlas.registered.is_some());
    }
    let mut query = app
        .world_mut()
        .query_filtered::<bevy_ecs::entity::Entity, bevy_ecs::query::With<
            country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,
        >>();
    let drawables = query.iter(app.world()).count();
    assert_eq!(
        drawables,
        registered + 8,
        "actual Nani and non-demo terrain Startup entities"
    );
    let terrain = app
        .world()
        .resource::<country_core::resources::rocky_terrain::TerrainEntities>();
    assert_ne!(terrain.floor, terrain.rocky);
    let floor_mesh = app
        .world()
        .get::<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>>(
            terrain.floor,
        )
        .unwrap();
    let rocky_mesh = app
        .world()
        .get::<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>>(
            terrain.rocky,
        )
        .unwrap();
    assert_eq!(
        floor_mesh, rocky_mesh,
        "original rocky draw shares actual floor mesh"
    );
    assert_eq!(
        app.world()
            .resource::<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>()
            .0,
        148794
    );
    let _ = std::mem::size_of::<GpuDrivenDrawable>(); // Bundle API, not a fabricated Component.
    assert!(
        app.world()
            .contains_resource::<tiny_glade::loading_graph::LoadingGpuResources>()
    );
    bridge
        .shader_backend()
        .prewarm(std::time::Duration::from_secs(60))
        .unwrap();
    eprintln!(
        "actual gameplay startup: {} prefabs, {parts} parts, {registered} registered Nani subsets",
        app.world().resource::<PrefabLibrary>().inner.len()
    );
}
