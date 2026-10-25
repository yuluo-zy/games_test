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

pub fn install_gameplay_gpu_startup(
    app: &mut App, device: Arc<rhapsody::device::Device>, hosts: HostTables,
    installed: PathBuf,
) -> anyhow::Result<()> {
    anyhow::ensure!(Arc::ptr_eq(&device, hosts.buffers.graphics_device()),
        "gameplay startup belongs to a different GPU owner");
    anyhow::ensure!(app.world().contains_resource::<AssetSsboLibrary>(),
        "gameplay GPU asset libraries must be installed before startup registration");
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
    app.add_systems(PreStartup, (move |world: &mut World| {
        crate::loading_graph::attach_loading_mei(world, device.clone(), &hosts)
            .expect("gameplay canonical MEI attachment");
    }).after(GameplayGpuPreStartup::Inputs));
    Ok(())
}
