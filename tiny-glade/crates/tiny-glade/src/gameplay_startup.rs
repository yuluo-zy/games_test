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

/// Host ordering for the original per-session startup tuple. These sets only
/// express the original dependency edges; resources remain owned by this App.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameplaySessionStartup { Walls, Sky, Critters, Roof, Sheep, Clearing, Meadow }

/// Registration identity only. Asset completion is the actual canonical
/// ClutterMeshes resource inserted by the original Startup producer.
#[derive(Resource)]
struct ClutterStartupRegistered;

fn install_clutter_startup(app: &mut App, installed: PathBuf) {
    if app.world().contains_resource::<ClutterStartupRegistered>() {
        return;
    }
    // B ClutterPlugin1406520f9 -> Schedules::add_systems140ed9480:
    // loader is one of three Startup members, not part of its Update chain.
    // The two subscription members are separate unresolved producers.
    app.add_systems(bevy_app::Startup,
        move |meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
              config: Res<GameConfig>, mut commands: Commands| {
            let catalog = system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes_from(
                &installed, &meshes, &watch, config.dev.demo);
            commands.insert_resource(catalog);
        });
    app.insert_resource(ClutterStartupRegistered);
}

pub(crate) fn require_clutter_catalog(world: &World) -> anyhow::Result<()> {
    anyhow::ensure!(world.contains_resource::<country_core::resources::clutter_meshes::ClutterMeshes>(),
        "gameplay Startup did not publish canonical ClutterMeshes");
    Ok(())
}

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

#[cfg(test)]
mod clutter_startup_tests {
    use super::*;
    use country_core::resources::clutter_meshes::ClutterMeshes;

    fn startup_app() -> App {
        let mut app = App::new();
        let mut config = GameConfig::default();
        config.dev.demo = false;
        app.insert_resource(config);
        app.insert_resource(AssetMeshLibrary::new());
        app.insert_resource(MeshWatch::new(false));
        app
    }

    #[test]
    fn clutter_loader_registration_is_once_per_app_and_only_in_startup() {
        for _ in 0..2 {
            let mut app = startup_app();
            install_clutter_startup(&mut app, PathBuf::from("not-loaded-at-registration"));
            install_clutter_startup(&mut app, PathBuf::from("not-loaded-at-registration"));
            assert_eq!(app.get_schedule(bevy_app::Startup).unwrap().graph().systems()
                .filter(|(_, system, _)| system.name().contains("install_clutter_startup")).count(), 1);
            assert_eq!(app.get_schedule(bevy_app::Update).unwrap().graph().systems()
                .filter(|(_, system, _)| system.name().contains("install_clutter_startup")).count(), 0);
            assert!(app.world().contains_resource::<ClutterStartupRegistered>());
            assert!(!app.world().contains_resource::<ClutterMeshes>());
            assert!(require_clutter_catalog(app.world()).is_err());
        }
    }

    #[test]
    fn a_missing_required_asset_never_publishes_the_canonical_catalog() {
        let mut app = startup_app();
        let missing = std::env::temp_dir().join(format!(
            "tiny-glade-missing-clutter-{}-{}", std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        assert!(!missing.exists());
        install_clutter_startup(&mut app, missing);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| app.update()));
        assert!(result.is_err());
        assert!(!app.world().contains_resource::<ClutterMeshes>());
        assert!(require_clutter_catalog(app.world()).is_err());
    }

    #[test]
    #[ignore = "requires the original B installation via TG_INSTALL_ROOT; no fabricated catalog"]
    fn genuine_clutter_startup_loads_once_from_the_explicit_installation() {
        let installed = crate::run_game::installation_root().unwrap();
        let mut app = startup_app();
        install_clutter_startup(&mut app, installed.clone());
        app.update();
        require_clutter_catalog(app.world()).unwrap();
        assert_eq!(app.world().resource::<AssetMeshLibrary>().mesh_entries.len(), 61);
        let catalog = app.world().resource::<ClutterMeshes>();
        assert_eq!(catalog.window_flowerbeds().len(), 6);
        let first = catalog.window_flowerbeds()[0].mesh.unwrap();
        let mesh = app.world().resource::<AssetMeshLibrary>().get(first).unwrap();
        assert!(std::path::Path::new(&mesh.source_file.as_ref().unwrap().path).starts_with(&installed));
        app.update();
        assert_eq!(app.world().resource::<AssetMeshLibrary>().mesh_entries.len(), 61);
        assert_eq!(app.world().resource::<ClutterMeshes>().window_flowerbeds()[0].mesh, Some(first));
    }
}

pub fn install_gameplay_gpu_startup(
    app: &mut App, device: Arc<rhapsody::device::Device>, hosts: HostTables,
    installed: PathBuf,
) -> anyhow::Result<()> {
    anyhow::ensure!(Arc::ptr_eq(&device, hosts.buffers.graphics_device()),
        "gameplay startup belongs to a different GPU owner");
    anyhow::ensure!(app.world().contains_resource::<AssetSsboLibrary>(),
        "gameplay GPU asset libraries must be installed before startup registration");
    // Also cover direct full-runtime callers: this installer is exclusively
    // for the actual gameplay World, independently of its early state/camera.
    app.insert_resource(crate::run_game::GameplayApplication);
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
    // Original backend set44: maintain -> explicit ApplyDeferred ->
    // unordered mapped updates -> real dirty-buffer flush. The asset runtime
    // registers build_missing_vaos -> rebuild_vaos before maintain with
    // before_ignore_deferred, so this one barrier also applies VAO Commands.
    app.edit_schedule(bevy_app::Update, |schedule| {
        country_core::render::nani::systems::add_mapped_nani_instance_systems(
            schedule, crate::run_game::GameSystemSet(44));
    });
    let config = app.world().resource::<GameConfig>();
    let demo = config.dev.demo;
    let watching = config.dev.live_link();
    app.init_resource::<PrefabLibrary>();
    app.init_resource::<MeshAtlasLibrary>();
    // Original ForestPlugin and run_game_thread own these actual resources;
    // init preserves an already-loaded same-World owner across installers.
    app.init_resource::<country_core::resources::tree_storage::InteractableForestPlacements>();
    app.init_resource::<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>();
    app.init_resource::<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>();
    app.init_resource::<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>();
    app.add_event::<country_core::systems::prefab::SpawnPrefabCmd>();
    app.add_event::<country_core::systems::prefab::DespawnPrefabCmd>();
    app.add_event::<country_core::resources::events::ConstructTimberframeCmd>();
    // B ForestPlugin140650c50: manage -> forest animation -> handplaced
    // animation, chain_ignore_deferred in the existing set36. No World/view
    // mirror, synthetic instance or extra Commands flush is introduced.
    app.add_systems(bevy_app::Update,
        (forest::manage_tree_prefabs::manage_tree_prefabs::<
            country_core::resources::history::MaybePendingSnapshotId>,
         forest::tree_animation::forest_tree_animation,
         forest::tree_animation::handplaced_tree_animation)
            .chain_ignore_deferred().in_set(crate::run_game::GameSystemSet(36)));
    if !app.world().contains_resource::<GpuWorld>() {
        app.insert_resource(GpuWorld::new(hosts.buffers.as_ref()));
    }
    if !app.world().contains_resource::<MeshWatch>() {
        app.insert_resource(MeshWatch::new(watching));
    }
    install_clutter_startup(app, installed.clone());
    if !app.world().contains_resource::<TextureWatch>() {
        app.insert_resource(TextureWatch::new(watching));
    }
    if watching && !app.world().contains_resource::<PrefabWatcher>() {
        app.insert_resource(PrefabWatcher::try_new(true)?);
    }
    app.add_event::<PrefabsModifiedCmd>();
    app.add_event::<MeshAtlasesModifiedCmd>();
    let enter_game = bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game);
    app.configure_sets(enter_game, (
        GameplaySessionStartup::Walls, GameplaySessionStartup::Sky,
        GameplaySessionStartup::Critters, GameplaySessionStartup::Roof,
        GameplaySessionStartup::Sheep, GameplaySessionStartup::Clearing,
        GameplaySessionStartup::Meadow,
    ).chain());
    // Each producer uses this App's actual mesh/shader owners and appends its
    // original drawable components. No mesh-only startup runs ahead of it.
    macro_rules! session_drawables {
        ($set:expr, $producer:path) => {{
            let root = installed.clone();
            let backend = hosts.shaders.clone();
            app.add_systems(
                bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
                (move |meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
                       shaders: Res<ApplicationShaderLibrary>, mut commands: Commands| {
                    $producer(&meshes, &shaders.0, backend.as_ref(), &watch, &root, &mut commands);
                }).in_set($set).after(country_core::startup::startup_session::load_game_session),
            );
        }};
    }
    session_drawables!(GameplaySessionStartup::Walls,
        country_core::startup::startup_walls::glade_startup_walls);
    session_drawables!(GameplaySessionStartup::Sky,
        country_core::startup::startup_sky_and_sun::startup_sky_and_sun);
    session_drawables!(GameplaySessionStartup::Roof,
        country_core::startup::startup_roof::startup_roof);
    // Insert inside install_gameplay_gpu_startup, after the session set chain is
    // configured and before installed/hosts are moved into PreStartup closures.
    // The old mesh-only/run_once tuple was already removed centrally.
    let critter_root = installed.clone();
    let critter_backend = hosts.shaders.clone();
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        (move |session: Res<country_core::resources::session::ActiveSession>,
               settings: Res<country_core::resources::glade_settings::GladeSettings>,
               audio: Res<country_core::resources::audio_resources::AudioCore<country_core::resources::native_audio::NativeStudio>>,
               meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
               shaders: Res<ApplicationShaderLibrary>, mut commands: Commands| {
            system_critters::startup_critters::startup_critters(
                &meshes, &shaders.0, critter_backend.as_ref(), &watch, &critter_root,
                &audio, &settings, &session, &mut commands,
            );
        })
            .in_set(GameplaySessionStartup::Critters)
            .after(country_core::startup::startup_session::load_game_session),
    );
    
    // Central-only narrow insertion in install_gameplay_gpu_startup. Original
    // register order is the already declared GameplaySessionStartup::Sheep.
    app.init_resource::<system_critters::bird_spots::BirdSpotManager>();
    let sheep_root = installed.clone();
    let sheep_backend = hosts.shaders.clone();
    let sheep_device = hosts.buffers.clone();
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        (move |meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
            shaders: Res<ApplicationShaderLibrary>, ssbos: Res<AssetSsboLibrary>,
            audio: Res<country_core::resources::audio_resources::AudioCore<country_core::resources::native_audio::NativeStudio>>,
            mut birds: ResMut<system_critters::bird_spots::BirdSpotManager>,
            mut pending: Option<ResMut<country_core::resources::critters::sheep::control::PendingSheepSnapshot>>,
            mut commands: Commands| {
            let assets = country_core::startup::sheep_assets::load_sheep_startup_assets(
                &sheep_root, &meshes, &shaders.0, sheep_backend.as_ref(), &watch,
                &ssbos, sheep_device.as_ref(),
            ).expect("original complete sheep animation asset producer");
            let snapshot = pending.as_mut().and_then(|p| p.0.take());
            commands.remove_resource::<country_core::resources::critters::sheep::control::PendingSheepSnapshot>();
            system_critters::startup_sheep::spawn_startup_sheep(
                &assets, &meshes, &shaders.0, &mut birds, &mut commands, snapshot,
                |name| {
                    use country_core::resources::audio_resources::AudioInstance;
                    match audio.instantiate_audio(name) {
                        AudioInstance::Disabled => Ok(system_critters::audio::AudioHandle::Disabled),
                        AudioInstance::Err(error) => Err(error),
                        active @ AudioInstance::Active { .. } =>
                            Ok(system_critters::audio::AudioHandle::Active(Box::new(active))),
                    }
                },
            ).expect("original complete sheep ECS producer");
        }).in_set(GameplaySessionStartup::Sheep)
            .after(country_core::startup::startup_session::load_game_session),
    );
    
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
        }).in_set(GameplaySessionStartup::Clearing)
            .after(country_core::startup::startup_session::load_game_session),
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
        }).in_set(GameplaySessionStartup::Meadow)
            .after(country_core::startup::startup_session::load_game_session),
    );
    app.configure_sets(PreStartup, (
        GameplayGpuPreStartup::Inputs, GameplayGpuPreStartup::Atlases,
        GameplayGpuPreStartup::Nani,
    ).chain());
    app.add_systems(PreStartup,
        country_core::startup::init_populate_ssbo_library::init_populate_ssbo_library
            .in_set(GameplayGpuPreStartup::Inputs));
    // B original fourth Startup tuple: persistent previews once per App.
    // PreStartup's canonical vertex_color_shader must already exist.
    let preview_root = installed.clone();
    let preview_backend = hosts.shaders.clone();
    app.add_systems(bevy_app::Startup,
        move |meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
              library: Res<ApplicationShaderLibrary>, mut commands: Commands| {
            country_core::startup::spawn_brush_previews::spawn_brush_previews(
                &preview_root, preview_backend.as_ref(), &meshes, &watch,
                &library.0, &mut commands).expect("original brush-preview startup");
        });
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
