//! The original gameplay PreStartup asset chain on the existing host owners.
//! B outer chain: preset textures -> unchained input tuple -> mesh atlases ->
//! Nani. Presets are already created by CoreRenderBridge::install_asset_libraries.
//! This adapter supplies typed/current paths; it does not fabricate readiness.
use std::{path::PathBuf, sync::Arc};
use bevy_app::{App, PreStartup};
use bevy_ecs::{prelude::*, schedule::IntoScheduleConfigs};
use country_core::{
    asset_libraries::{mesh_library::AssetMeshLibrary, meshwatch::MeshWatch,
        ssbo_library::AssetSsboLibrary,
        texture_watch::TextureWatch},
    game_config::GameConfig,
    render::nani::gpu_world::GpuWorld,
    resources::{prefab::{PrefabLibrary, PrefabsModifiedCmd},
        render::mesh_atlas_library::{MeshAtlasLibrary, MeshAtlasesModifiedCmd}},
    startup::{load_prefabs::{self, PrefabWatcher}, load_mesh_atlases, startup_nani},
};
use crate::{render_host::HostTables, run_game::ApplicationShaderLibrary};

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameplayGpuPreStartup { Inputs, Atlases, Nani }

/// The actual gameplay device tuple shared by the window host and headless
/// acceptance. Asset tables must already belong to this same physical device.
/// Keeping the tuple here prevents tests from substituting a smaller startup.
pub fn install_gameplay_device_runtime(
    app: &mut App, device: Arc<rhapsody::device::Device>, hosts: HostTables,
    installed: PathBuf,
) -> anyhow::Result<()> {
    install_gameplay_gpu_startup(app, device, hosts.clone(), installed.clone())?;
    crate::menu_background_host::install(app, hosts.clone());
    crate::world_postprocess::install_session_previews(app, hosts.clone());
    crate::screenshot::shared_texture_capture::install(app);
    #[cfg(target_os = "windows")]
    crate::audio_host::install(app, &installed)?;
    crate::world_terrain::install_terrain_startup(app, hosts.shaders.clone(), installed);
    crate::world_terrain::install_grass_startup(app, hosts.shaders.clone());
    crate::world_lighting::install_deferred_startup(app, hosts.shaders.clone());
    let demo = app.world().resource::<GameConfig>().dev.demo;
    app.add_plugins(country_core::resources::gpu_lut::GpuLutPlugin { demo });
    Ok(())
}

pub fn install_gameplay_gpu_startup(
    app: &mut App, device: Arc<rhapsody::device::Device>, hosts: HostTables,
    installed: PathBuf,
) -> anyhow::Result<()> {
    anyhow::ensure!(Arc::ptr_eq(&device, hosts.buffers.graphics_device()),
        "gameplay startup belongs to a different GPU owner");
    anyhow::ensure!(app.world().contains_resource::<AssetSsboLibrary>(),
        "gameplay GPU asset libraries must be installed before startup registration");
    // Original run_game_thread1405a0237..31a: clear_canvas is Update set33,
    // guarded solely by on_event<ClearCanvasCmd>. OnExit's false command must
    // execute during Reload before the next session, without an in_state(Game).
    country_core::systems::clear_canvas::register_clear_canvas_resources(
        app.world_mut(), hosts.textures.clone());
    app.add_systems(bevy_app::Update,
        country_core::systems::clear_canvas::clear_canvas
            .in_set(crate::run_game::GameSystemSet(33))
            .run_if(bevy_ecs::schedule::common_conditions::on_event::<
                country_core::systems::mode_manager::ClearCanvasCmd>));
    crate::mei_host::install_mei_frame_systems(app);
    app.init_resource::<country_core::systems::render_loop::lut_gallery::LutGalleryState>();
    // MenuPlugin14068de50:14068e29e passes0x24 to in_set; the original
    // lut_gallery::ui wrapper140ff9df0 is guarded by in_state(Game).
    app.add_systems(bevy_app::Update, toolbar::photomode::lut_gallery::ui
        .in_set(crate::run_game::GameSystemSet(36))
        .run_if(bevy_state::condition::in_state(
            country_core::app_loading_state::AppLoadingState::Game)));
    let config = app.world().resource::<GameConfig>();
    let demo = config.dev.demo;
    let watching = config.dev.live_link();
    app.init_resource::<PrefabLibrary>();
    app.init_resource::<MeshAtlasLibrary>();
    if !app.world().contains_resource::<GpuWorld>() {
        app.insert_resource(GpuWorld::new(hosts.buffers.as_ref()));
    }
    if !app.world().contains_resource::<MeshWatch>() {
        app.insert_resource(MeshWatch::new(watching));
    }
    if !app.world().contains_resource::<TextureWatch>() {
        app.insert_resource(TextureWatch::new(watching));
    }
    if watching && !app.world().contains_resource::<PrefabWatcher>() {
        app.insert_resource(PrefabWatcher::try_new(true)?);
    }
    app.add_event::<PrefabsModifiedCmd>();
    app.add_event::<MeshAtlasesModifiedCmd>();
    // Original clearing is a per-session producer. AssetPlacements and forest
    // records are cleared by OnExit(Game), then rebuilt for the new theme.
    app.init_resource::<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>();
    let clearing_root = installed.clone();
    let clearing_backend = hosts.shaders.clone();
    let clearing_device = hosts.buffers.clone();
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        (move |session: Res<country_core::resources::session::ActiveSession>,
               meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
               shaders: Res<ApplicationShaderLibrary>,
               mut placements: ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,
               mut ssbos: ResMut<AssetSsboLibrary>,
               mut count: ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,
               mut forest: ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,
               mut commands: Commands| {
            country_core::startup::clearing::init_clearing_complete(
                &clearing_root, &session, clearing_device.as_ref(), clearing_backend.as_ref(),
                &meshes, &watch, &shaders.0, &mut placements, &mut ssbos,
                &mut count, &mut forest, &mut commands,
            ).expect("original complete per-session clearing startup");
        }).after(country_core::startup::startup_session::load_game_session),
    );
    let meadow_root = installed.clone();
    let meadow_backend = hosts.shaders.clone();
    let meadow_device = hosts.buffers.clone();
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        (move |session: Res<country_core::resources::session::ActiveSession>,
               meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
               shaders: Res<ApplicationShaderLibrary>,
               mut placements: ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,
               mut commands: Commands| {
            country_core::startup::startup_meadow::init_meadow(
                &meadow_root, &session, meadow_device.as_ref(), meadow_backend.as_ref(),
                &meshes, &watch, &shaders.0, &mut placements, &mut commands,
            ).expect("original meadow placement/drawable startup");
        }).after(country_core::startup::startup_session::load_game_session),
    );
    app.configure_sets(PreStartup, (
        GameplayGpuPreStartup::Inputs, GameplayGpuPreStartup::Atlases,
        GameplayGpuPreStartup::Nani,
    ).chain());
    app.add_systems(PreStartup,
        country_core::startup::init_populate_ssbo_library::init_populate_ssbo_library
            .in_set(GameplayGpuPreStartup::Inputs));
    let input_root = installed.clone();
    let input_hosts = hosts.clone();
    app.add_systems(PreStartup, (move |world: &mut World| {
        crate::loading_graph::initialize_common_assets(world, &input_hosts, &input_root, demo)
            .expect("original gameplay palette/textures/fonts startup");
        crate::loading_graph::initialize_grading_assets(world, &input_hosts, &input_root)
            .expect("original gameplay grading LUT startup");
    }).in_set(GameplayGpuPreStartup::Inputs));
    let prefab_root = installed.join("assets/prefabs");
    app.add_systems(PreStartup,
        (move |mut prefabs: ResMut<PrefabLibrary>, mut watcher: Option<ResMut<PrefabWatcher>>| {
            load_prefabs::load_prefabs_with_watcher(&prefab_root, &mut prefabs, watcher.as_deref_mut())
                .expect("original load_prefabs startup");
        }).in_set(GameplayGpuPreStartup::Inputs));
    app.add_systems(PreStartup,
        (move |prefabs: Res<PrefabLibrary>, mut atlases: ResMut<MeshAtlasLibrary>,
               mut meshes: ResMut<AssetMeshLibrary>, mut watcher: Option<ResMut<PrefabWatcher>>| {
            load_mesh_atlases::load_mesh_atlases(&installed, &prefabs, &mut atlases, &mut meshes,
                watcher.as_deref_mut(), demo).expect("original load_mesh_atlases startup");
        }).in_set(GameplayGpuPreStartup::Atlases));
    let backend = hosts.shaders.clone();
    let buffers = hosts.buffers.clone();
    app.add_systems(PreStartup,
        (move |library: Res<ApplicationShaderLibrary>, ssbos: Res<AssetSsboLibrary>,
               atlases: Res<MeshAtlasLibrary>, mut gpu: ResMut<GpuWorld>, mut commands: Commands| {
            startup_nani::startup_nani(buffers.as_ref(), backend.as_ref(), &library.0,
                &ssbos, &atlases, &mut gpu, &mut commands);
        }).in_set(GameplayGpuPreStartup::Nani));
    // Current MEI host attaches to the full canonical text buffers; allocation
    // remains the original72-entry producer's responsibility.
    let gallery_backend = hosts.shaders.clone();
    app.add_systems(PreStartup, (move |world: &mut World| {
        crate::loading_graph::attach_loading_mei(world, device.clone(), &hosts)
            .expect("gameplay canonical MEI attachment");
    }).after(GameplayGpuPreStartup::Inputs));
    app.add_systems(PreStartup,
        move |library: Res<ApplicationShaderLibrary>| {
            use country_core::systems::render_loop::lut_gallery;
            country_core::startup::load_shaders::load_shader_into_library(
                &library.0, gallery_backend.as_ref(),
                lut_gallery::SHADER_URI, lut_gallery::SHADER_NAME);
        });
    Ok(())
}
