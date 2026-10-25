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

#[cfg(any(windows, feature = "native-audio"))]
pub(crate) type GameplayAudioStudio = country_core::resources::native_audio::NativeStudio;
#[cfg(not(any(windows, feature = "native-audio")))]
pub(crate) type GameplayAudioStudio = country_core::resources::audio_resources::DisabledStudio;

/// 无 FMOD 的宿主仍执行原作加载与卸载状态机，事件实例进入真实 Disabled 分支。
#[cfg(not(any(windows, feature = "native-audio")))]
pub(crate) fn install_disabled_audio(app: &mut App) {
    use country_core::{
        resources::audio_resources::{AudioCore, DisabledEvent},
        systems::audio::game_loading_audio::{
            GameLoadingAudioState, init_game_loading_audio, loading_audio,
        },
    };
    app.insert_resource(AudioCore::<GameplayAudioStudio> {
        studio: None,
        banks: Default::default(),
    });
    app.insert_resource(GameLoadingAudioState::<DisabledEvent>::empty_after_samples());
    app.add_systems(
        bevy_app::Startup,
        init_game_loading_audio::<GameplayAudioStudio>,
    );
    app.add_systems(
        bevy_app::Update,
        loading_audio::<GameplayAudioStudio>.in_set(crate::run_game::GameSystemSet(38)),
    );
}

#[cfg(all(test, not(any(windows, feature = "native-audio"))))]
mod disabled_audio_tests {
    use super::*;
    use country_core::{
        resources::{
            audio_resources::{AudioCore, AudioInstance, DisabledEvent},
            time::GameTime,
        },
        systems::{
            audio::game_loading_audio::GameLoadingAudioState,
            save::load_game::{StartupRevealFinished, StartupRevealStarted},
        },
    };

    #[test]
    fn host_disabled_audio_keeps_startup_reveal_and_unload_lifecycle() {
        let mut app = App::new();
        app.init_resource::<GameTime>();
        app.add_event::<StartupRevealStarted>();
        app.add_event::<StartupRevealFinished>();
        install_disabled_audio(&mut app);
        assert!(
            app.world()
                .resource::<GameLoadingAudioState<DisabledEvent>>()
                .snapshot
                .is_none()
        );

        app.update();
        assert!(matches!(
            app.world()
                .resource::<GameLoadingAudioState<DisabledEvent>>()
                .snapshot,
            Some(AudioInstance::Disabled)
        ));
        let core = app.world().resource::<AudioCore<GameplayAudioStudio>>();
        assert!(core.studio.is_none());
        assert!(core.banks.is_empty());
        assert!(matches!(
            core.instantiate_audio("event:/Critters/Sheep"),
            AudioInstance::Disabled
        ));

        app.world_mut().send_event(StartupRevealStarted);
        app.world_mut().send_event(StartupRevealFinished);
        app.update();
        assert!(
            app.world()
                .resource::<GameLoadingAudioState<DisabledEvent>>()
                .snapshot
                .is_none()
        );

        // 执行卸载消费者使用的真实状态方法，随后由宿主 set38 消费计时。
        app.world_mut().resource_scope(
            |world, mut state: Mut<GameLoadingAudioState<DisabledEvent>>| {
                state.arm_after_unload(world.resource::<AudioCore<GameplayAudioStudio>>());
            },
        );
        app.world_mut().resource_mut::<GameTime>().delta = 1.0;
        app.update();
        let state = app
            .world()
            .resource::<GameLoadingAudioState<DisabledEvent>>();
        assert!(state.loading);
        assert!(matches!(state.snapshot, Some(AudioInstance::Disabled)));
        app.world_mut().resource_mut::<GameTime>().delta = 0.125;
        app.update();
        let state = app
            .world()
            .resource::<GameLoadingAudioState<DisabledEvent>>();
        assert!(!state.loading);
        assert!(state.snapshot.is_none());
        assert_eq!(state.timer, 1.125);
    }
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameplayGpuPreStartup { Inputs, Atlases, Nani }

/// Host ordering for the original per-session startup tuple. These sets only
/// express the original dependency edges; resources remain owned by this App.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameplaySessionStartup { Walls, Sky, Critters, Roof, Sheep, Clearing, Meadow }

// The ClutterMeshes loader is the Startup member of ClutterPlugin1406520f9
// (system_clutter::ClutterPlugin, mounted by run_game::add_original_plugins).
// This host no longer carries a second closure for it.
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
    #[cfg(any(windows, feature = "native-audio"))]
    crate::audio_host::install(app, &installed)?;
    crate::world_terrain::install_terrain_startup(app, hosts.shaders.clone(), installed.clone());
    crate::world_terrain::install_grass_startup(app, hosts.shaders.clone());
    install_world_visual_startups(app, hosts.shaders.clone(), installed.clone(), demo_flag(app));
    crate::world_lighting::install_deferred_startup(app, hosts.shaders.clone());
    let demo = app.world().resource::<GameConfig>().dev.demo;
    app.add_plugins(country_core::resources::gpu_lut::GpuLutPlugin { demo });
    Ok(())
}

fn demo_flag(app: &App) -> bool {
    app.world().resource::<GameConfig>().dev.demo
}

/// The remaining original "register shaders and spawn entities" producers, installed on the actual gameplay App with its
/// real mesh / shader owners (`AssetMeshLibrary`, `MeshWatch`, `ApplicationShaderLibrary`, `Commands`). Schedules come from
/// the original registration chain (see each function's module docs for addresses):
///
/// * Startup (once per App; `Plugin::build` tail-calls `SubApp::add_systems<Startup, _>` whose `Schedules::entry` clone
///   0x1416a1c20 uses the label vtable 0x142c8c420 = "Startup"): `startup_clouds` 0x14085b8b0, `startup_stars` 0x140903ca0,
///   `startup_billboard_forest` 0x140ae8080, `startup_loading_screen` 0x140a1a1e0, and `startup::debug::debug` 0x1408efd40
///   (member of the Startup tuple 0x14099b8b0 registered by `add_game_startup_systems` @14094117f).
/// * OnEnter(Game) (members of the tuples 0x140839b80 / 0x141736760 that `add_game_startup_systems` chains into
///   `add_systems<OnEnter<AppLoadingState>>` @14094115d): `load_snow_particles` 0x140ae7d90, `startup_water` 0x1408f1a20,
///   `startup_ivy` 0x140a21a30. They read `ActiveSession`, hence run after `load_game_session`.
pub fn install_world_visual_startups(
    app: &mut App,
    backend: Arc<crate::render_host::ShaderHost>,
    installed: PathBuf,
    demo: bool,
) {
    use country_core::systems::render_loop::draw_passes::draw_shaders::{
        billboard_forest::startup_billboard_forest,
        clouds::startup_clouds,
        startup_quad::{WorldStartupHost, load_snow_particles, startup_loading_screen},
        stars::startup_stars,
    };
    use country_core::resources::session::ActiveSession;
    macro_rules! quad_startup {
        ($schedule:expr, $producer:expr) => {{
            let backend = backend.clone();
            app.add_systems($schedule,
                move |meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
                      shaders: Res<ApplicationShaderLibrary>, mut commands: Commands| {
                    let mut host = WorldStartupHost {
                        meshes: &meshes, watch: &watch, shaders: &shaders.0,
                        backend: backend.as_ref(), commands: &mut commands,
                    };
                    $producer(&mut host);
                });
        }};
    }
    quad_startup!(bevy_app::Startup, startup_clouds);
    quad_startup!(bevy_app::Startup, startup_stars);
    quad_startup!(bevy_app::Startup, startup_billboard_forest);
    quad_startup!(bevy_app::Startup, startup_loading_screen);

    // `LoadingScreenProgress(Gap(0.0, 0.0))`: the loading-screen Plugin::build 0x140a1a190 ends with
    // `App::insert_resource` 0x14152e960 with xmm1 = xmm2 = 0 right after its two `add_systems`. The draw callback
    // (env41) unwraps this resource. The second registered system (condition-gated updater) is not recovered here.
    app.insert_resource(country_core::systems::render_loop::draw_passes::draw_callbacks_a::LoadingScreenProgressView {
        f0: 0.0, f4: 0.0,
    });
    let debug_root = installed.clone();
    let debug_backend = backend.clone();
    app.add_systems(bevy_app::Startup,
        move |meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
              shaders: Res<ApplicationShaderLibrary>, mut commands: Commands| {
            country_core::startup::debug::debug(
                &debug_root, &meshes, &watch, &shaders.0, debug_backend.as_ref(), &mut commands,
            ).expect("original startup::debug::debug");
        });

    let enter_game = bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game);
    let after_session = country_core::startup::startup_session::load_game_session;
    let snow_backend = backend.clone();
    app.add_systems(enter_game.clone(),
        (move |session: Res<ActiveSession>, meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
               shaders: Res<ApplicationShaderLibrary>, mut commands: Commands| {
            let mut host = WorldStartupHost {
                meshes: &meshes, watch: &watch, shaders: &shaders.0,
                backend: snow_backend.as_ref(), commands: &mut commands,
            };
            load_snow_particles(session.meta.glade, &mut host);
        }).after(after_session));
    if demo {
        // Original early-return flag (global 143634d90) also disables the water-plane draw callback.
        country_core::systems::render_loop::draw_passes::draw_callbacks_b::WATER_PLANES_DISABLED
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
    let water_root = installed.clone();
    let water_backend = backend.clone();
    app.add_systems(enter_game.clone(),
        (move |session: Res<ActiveSession>, meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
               shaders: Res<ApplicationShaderLibrary>, mut commands: Commands| {
            country_core::startup::startup_water::startup_water(
                &water_root, demo, session.meta.glade, water_backend.as_ref(),
                &meshes, &watch, &shaders.0, &mut commands,
            ).expect("original startup_water");
        }).after(after_session));
    // The ivy draw callbacks (instanced_ivy_branch/leaf/flower) read IvyStorage & co. Original
    // `IvySystems::build` -> `insert_ivy_resources` 0x1409440c0 publishes them; init_resource is idempotent, so a
    // later full IvySystems mount keeps whatever it installs first.
    country_core::systems::ivy::registration::insert_ivy_resources(app.world_mut());
    let ivy_backend = backend;
    app.add_systems(enter_game,
        (move |meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
               shaders: Res<ApplicationShaderLibrary>, mut commands: Commands| {
            country_core::startup::startup_ivy::startup_ivy(
                &installed, &meshes, &watch, &shaders.0, ivy_backend.as_ref(), &mut commands,
            ).expect("original startup_ivy");
        }).after(after_session));
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
    // ForestPlugin (InteractableForestPlacements) is mounted by run_game::add_original_plugins;
    // run_game_thread owns these remaining actual resources.
    app.init_resource::<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>();
    app.init_resource::<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>();
    app.init_resource::<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>();
    app.add_event::<country_core::systems::prefab::SpawnPrefabCmd>();
    app.add_event::<country_core::systems::prefab::DespawnPrefabCmd>();
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
               audio: Res<country_core::resources::audio_resources::AudioCore<GameplayAudioStudio>>,
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
    // BirdSpotManager is inserted by CritterPlugin14067d440 (run_game::add_original_plugins).
    let sheep_root = installed.clone();
    let sheep_backend = hosts.shaders.clone();
    let sheep_device = hosts.buffers.clone();
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        (move |meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
            shaders: Res<ApplicationShaderLibrary>, ssbos: Res<AssetSsboLibrary>,
            audio: Res<country_core::resources::audio_resources::AudioCore<GameplayAudioStudio>>,
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
                    // AudioInstance implements Drop, so its fields cannot be moved out
                    // in a by-value match; inspect it by reference and move the whole value.
                    let instance = audio.instantiate_audio(name);
                    match &instance {
                        AudioInstance::Disabled => Ok(system_critters::audio::AudioHandle::Disabled),
                        AudioInstance::Err(error) => Err(error.clone()),
                        AudioInstance::Active { .. } =>
                            Ok(system_critters::audio::AudioHandle::Active(Box::new(instance))),
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
        crate::loading_graph::initialize_common_assets(world, &input_hosts, &input_root, demo, false)
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
