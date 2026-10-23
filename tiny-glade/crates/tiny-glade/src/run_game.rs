//! B `crates/tiny-glade/src/run_game.rs`（游戏线程装配与临时窗口）。
//!
//! 恢复依据（restorer-tinyglade-E fresh）：
//! - `run_game` `0x140593b70`：Ghidra 反编译超时；改用 ReVa 专属 worker
//!   `restorer-tinyglade-E`（`DecompInterface.decompileFunction(...,300s)`）取得
//!   3,671 行反编译（`work/restorer-tinyglade-E/0x140593b70_run_game.c`）。
//! - `run_game_thread` `0x14059d450`：同法取得 6,544 行反编译。
//! - `make_temp_window` `0x1405ae010`（369 行反编译，winit 平台胶水）。
//! - `impl$1::build` `0x14003bb80`（8 B）—— Ghidra 解析为
//!   `finish<country_core::systems::main_camera::MainCameraPlugin>`：ICF 合并的
//!   单字节 RET 空体（1,581 处引用），build 为空注册。
//! - `impl$2::build` `0x1405aeea0`（578 B）—— bevy_input 注册面。
//! - `compute_on_dedicated_thread` `0x14032d170`（111 行反编译）—— 通用线程助手。
//!
//! ## 已恢复
//!
//! ### `impl$2::build`（胶水-legitimate）
//! `KeyboardInput`、`KeyboardFocusLost`、`MouseButtonInput`、`MouseMotion`、
//! `MouseWheel` 五个事件 + `ButtonInput<KeyCode>`/`ButtonInput<MouseButton>` 两个
//! 资源 + `PreUpdate` 的 `keyboard_input_system`/`mouse_button_input_system`
//! （`InputSystem` set）。反编译中每个 `register_event<T>` 都被
//! `Components::get_resource_id` 的既有性判断包住，等价于 `add_event` 的重复注册
//! 检查。本实现转发上游 `bevy_input::InputPlugin`。
//! **差异**：上游 0.16.0 `InputPlugin` 还注册手势/gamepad 事件；按 AGENTS
//! 「直接复用依赖」保留，额外注册项不影响原消费面。
//!
//! ### `compute_on_dedicated_thread`（判等，std 转发）
//! fresh 0x14032d170：`sync_channel(1)` → 复制 name → `Builder::name(name)`
//! → `spawn(move || tx.send(f()))`；spawn 失败
//! `panic!("Thread failed to spawn: {name}: {err}")`（文案 pieces
//! `"Thread failed to spawn: "` + name + `": "` + io::Error，0x1429beeb1）；
//! 成功后 drop JoinHandle（CloseHandle）→ `rx.recv().unwrap()`。
//!
//! ## Integration boundary (2026-10-18)
//! `run_game` now calls the actual public winit window/event-loop adapter and
//! `run_game_thread` materializes the currently evidence-backed CPU startup
//! slice. The complete original plugin tuple and domain startup systems remain
//! open; no empty audio/render/terrain replacements are registered. Device-owned
//! asset libraries are initialized by the production renderer on its own device.
//! MainCamera uses its recovered `new(aspect)` constructor, not a zero Default.
//! The macOS main-thread pump differs from B's Windows two-thread ownership.

use anyhow::Context;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use country_core::{game_config::GameConfig, resources::window::WindowSize};
use std::{path::PathBuf, sync::Arc};

#[path = "startup_event_runtime.rs"]
mod startup_event_runtime;

#[path = "startup_construction_runtime.rs"]
mod startup_construction_runtime;

#[path = "history_runtime.rs"]
mod history_runtime;

/// The original type is `country_core::systems::GameSystemSet`; the host re-exports it so
/// every plugin (including those in the `system_*` crates) shares one set identity.
pub use country_core::systems::GameSystemSet;

/// Actual current-host World role, set by the gameplay App constructor and
/// retained when its SubApp moves. The independent loading App never copies
/// this marker. It is not an asset/load-completion flag.
#[derive(Clone, Resource)]
pub(crate) struct GameplayApplication;

/// Each original App constructs a separate shader library. The Arc is a Rust
/// owner for that App's library; it does not merge the two native libraries.
pub use country_core::asset_libraries::shader_library::SharedAssetShaderLibrary as ApplicationShaderLibrary;

/// B `tiny_glade::run_game::impl$2` 对应的输入插件（PDB 未暴露类型名）。
///
/// `build` 反编译见 `0x1405aeea0`；语义 = 上游 `bevy_input::InputPlugin`
/// 的核心注册面（见模块注释的差异登记）。
#[derive(Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_input::InputPlugin);
    }
}

/// B `tiny_glade::run_game::impl$1::build` `0x14003bb80`（8 B，ICF 空实现合并）。
#[derive(Default)]
pub struct EmptyPlugin;

impl Plugin for EmptyPlugin {
    fn build(&self, _app: &mut App) {}
}

/// Original plugin / extension-trait registrations whose bodies were recovered
/// from machine code in their own crates. The host only mounts them; it never
/// re-lists their systems, resources or events (one registration per system).
/// Original ordering of the plugin tuple inside run_game_thread is not recovered
/// (OPEN); every system keeps the explicit GameSystemSet recovered in its own body.
fn add_original_plugins(app: &mut App) {
    use country_core::systems::terrain::TerrainSystems;
    use system_onboarding::OnboardingSystems;
    use system_roof::RoofSystems;
    use timberframe::TimberframeSystems;
    // insert_terrain_resources @1409457c0 / add_terrain_events @140945a20 /
    // add_terrain_systems @1409455f0.
    app.insert_terrain_resources().add_terrain_events().add_terrain_systems();
    app.add_plugins((
        forest::ForestPlugin,
        system_tree::TreePlugin,
        system_clutter::ClutterPlugin,
        system_critters::CritterPlugin,
        system_critters::sheep::SheepPlugin,
        // 0x14082bd70：Update 里的 cache_prev_object_transforms（羊/鸭的 prev_model_to_world 来源）。
        country_core::components::drawable::PrevTransformPlugin,
    ));
    // add_roof_events_and_resources @1406305f0 / add_roof_systems @14062fcb0.
    app.add_roof_events_and_resources().add_roof_systems();
    app.add_timberframe(); // @140630a20
    app.add_onboarding(); // @140650d50
}

/// `tiny_glade::run_game::compute_on_dedicated_thread` `0x14032d170` —— 判等。
///
/// `sync_channel(1)` + `Builder::name(name)` + spawn + recv；spawn 失败 panic
/// 文案 `"Thread failed to spawn: {name}: {err}"`，recv 失败 `unwrap` panic。
pub fn compute_on_dedicated_thread<T, F>(name: String, f: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::Builder::new()
        .name(name.clone())
        .spawn(move || {
            let _ = tx.send(f());
        })
        .unwrap_or_else(|err| panic!("Thread failed to spawn: {name}: {err}"));
    // JoinHandle 在此 drop（原版 CloseHandle，线程分离）；结果经 channel 回传。
    rx.recv().unwrap()
}

/// Resolve the packaged installation for the explicit local host. The original
/// executable and its assets live together. TG_INSTALL_ROOT is an explicit
/// packaging adapter for a cargo-built binary, not a replacement asset set.
pub fn installation_root() -> anyhow::Result<PathBuf> {
    use anyhow::Context;
    let root = if let Some(root) = std::env::var_os("TG_INSTALL_ROOT") {
        PathBuf::from(root)
    } else {
        std::env::current_exe()?
            .parent()
            .context("Executable has no installation directory")?
            .to_path_buf()
    };
    let root = root
        .canonicalize()
        .with_context(|| format!("Resolving installation directory {}", root.display()))?;
    for relative in ["default_settings.ron", "assets", "compiled-shaders"] {
        anyhow::ensure!(
            root.join(relative).exists(),
            "Installation {} is missing {relative}; set TG_INSTALL_ROOT to the original packaged game directory",
            root.display()
        );
    }
    Ok(root)
}

/// B startup consumer path with winit's public platform API. On macOS the OS
/// pump stays on the process main thread; the original Windows game-thread
/// split is not simulated with a detached thread around main-thread-only APIs.
pub fn run_game(
    mut config: GameConfig,
    cli: crate::app_config::CliArgs,
    steam: Option<Arc<dyn country_core::resources::steam::SteamSdk>>,
) -> anyhow::Result<()> {
    let launch_display = crate::app_config::LaunchDisplayOverride::resolve(
        &cli,
        &config.video.display,
        steam.is_none(),
        std::env::var_os("TG_FULLSCREEN").is_some(),
        std::env::var("TG_WINDOW_SIZE").ok().as_deref(),
    )?;
    if let Some(policy) = &launch_display {
        policy.apply(&mut config.video.display);
        log::info!(
            "Launch display override: fullscreen={}, windowed_resolution={:?}; saved display preferences retained",
            config.video.display.fullscreen,
            config.video.display.windowed_resolution,
        );
    }
    let event_loop = winit::event_loop::EventLoop::<()>::new()?;
    let (window, _exclusive_mode) =
        crate::setup::setup_window(&event_loop, &mut config.video.display);
    let window = Arc::new(window);
    let physical = window.inner_size();
    let size = WindowSize::new(physical.width, physical.height);
    if crate::event_loop::game_thread_enabled() {
        return crate::event_loop::run_event_loop_threaded(event_loop, window, move |window| {
            let (app, options, shaders) = build_game(config, cli, steam, launch_display, size)?;
            crate::game_loop::GameLoop::new(app, window, options, shaders)
        });
    }
    let (app, options, shaders) = build_game(config, cli, steam, launch_display, size)?;
    crate::event_loop::run_event_loop(event_loop, window, app, options, shaders)
}

/// Everything between "the window exists" and "the game loop can start": the
/// `App` with its plugins, the Steam/platform resources and the loop options.
/// With `TG_GAME_THREAD=1` this runs on the game thread, because the `App` and
/// the objects built from it are `!Send`.
fn build_game(
    config: GameConfig,
    cli: crate::app_config::CliArgs,
    steam: Option<Arc<dyn country_core::resources::steam::SteamSdk>>,
    launch_display: Option<crate::app_config::LaunchDisplayOverride>,
    window: WindowSize,
) -> anyhow::Result<(
    App,
    crate::game_loop::GameLoopOptions,
    Arc<country_core::asset_libraries::shader_library::AssetShaderLibrary>,
)> {
    let mut app = run_game_thread(config, &cli, window)?;
    if let Some(policy) = launch_display {
        app.insert_resource(policy);
    }
    if let Some(client) = steam {
        app.insert_resource(
            country_core::resources::platform::RuntimePlatform::from_detection(
                false,
                client.is_steam_running_on_steam_deck(),
            ),
        );
        app.insert_resource(country_core::systems::steam::SteamClientResource {
            client: Some(client.clone()),
        });
        // add_steam_integration 0x140945320: new request list, HookScreenshots(true), then
        // register_callback::<ScreenshotRequested> whose body (0x14091ad50) pushes an empty
        // placeholder that gather_screenshot_requests turns into one capture.
        let screenshot_requests = country_core::systems::steam::SteamScreenshotRequests::default();
        client.hook_screenshots(true);
        let callback_requests = screenshot_requests.clone();
        client.register_screenshot_requested(Arc::new(move || {
            callback_requests.on_screenshot_requested()
        }));
        app.insert_resource(screenshot_requests);
        app.world_mut().resource_mut::<country_core::systems::steam::SteamOverlayOpenResources>().client = Some(client);
    } else {
        // The explicitly named standalone host has neither GFN nor Steam SDK.
        app.insert_resource(
            country_core::resources::platform::RuntimePlatform::from_detection(false, false),
        );
    }
    let frame_limit = std::env::var("TG_SMOKE_FRAMES")
        .ok()
        .map(|value| value.parse::<u64>())
        .transpose()?;
    let options = crate::game_loop::GameLoopOptions {
        compiled_shader_root: paths::installed().join("compiled-shaders"),
        frame_limit,
        input_recording: cli.to_input_recording_opts(),
    };
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    Ok((app, options, shaders))
}

/// Materialize the recovered registration slice from B run_game_thread.
/// This does not assert that the still-unrecovered ~50-plugin global schedule
/// is complete. In particular it does not install empty audio/terrain/render
/// systems in place of the original producers; the renderer must insert its
/// real device-owned libraries before their consumers can run.
pub fn run_game_thread(
    config: GameConfig,
    cli: &crate::app_config::CliArgs,
    window: WindowSize,
) -> anyhow::Result<App> {
    use country_core::{
        input::{
            ActiveInputModeTracker,
            action_set::{ActionSetStates, ActionSetValues},
        },
        resources::{
            game_clock::GameClock,
            gui::{DebugGui, EscMenuState, NextEscMenuState, PrevEscMenuState, TimeOfDayScreen},
            mouse_capture::CursorCapturedPosition,
            mouse_terrain_raycast::CursorPositionSS,
            session::{SavedSessionList, SessionFlowPlugin},
        },
        systems::{
            backend::{gui::configure_gui_consumers, input::register_input_events},
            camera_rig::main_camera::MainCamera,
        },
    };
    init_ecs_task_pool();
    let mut app = App::new();
    app.insert_resource(GameplayApplication);
    startup_event_runtime::install(&mut app);
    // Original plugin/extension-trait registrations. They run right after the
    // App-level event list so that their add_event calls precede any helper that
    // would init_resource<Events<T>> (which would make add_event skip).
    add_original_plugins(&mut app);
    // Complete the original unchained set7 tuple on the existing App.
    // The other three leaves were delivered in brush-startup/runtime-registration.rs.
    app.add_systems(bevy_app::Update, (
     country_core::systems::debug::human_scale_ref::human_scale_ref,
     country_core::systems::ui_2d_follow_mouse::ui_2d_follow_mouse,
     country_core::systems::mesh_follow_terrain::mesh_follow_terrain,
    ).in_set(GameSystemSet(7)));
    // No run_if, no chain. Native Query/resource permissions are preserved.
    
    // Integration fragment for the already authoritative App; no replacement owner.
    // B run_game_thread1405a019c/19e => in_set7, immediately add_systems(Update).
    // These three leaves belong to the original unchained six-system tuple.
    app.add_event::<country_core::systems::brush_preview::OverwriteBrushPreview>();
    app.add_event::<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>();
    app.add_systems(bevy_app::Update, (
        country_core::systems::brush_preview::brush_preview,
        country_core::systems::look_at_camera::look_at_camera,
        country_core::systems::arrow_gizmo_animation::arrow_gizmo_animation,
    ).in_set(GameSystemSet(7)));
    
    #[cfg(any(windows, feature = "native-audio"))]
    app.insert_resource(crate::audio_host::AudioStartupOptions {
        asynchronous: cli.fmod_asynchronous,
    });
    // Original StatesPlugin/init_state and PostStartup Pending(Game) producer.
    use bevy_state::app::AppExtStates;
    app.add_plugins(bevy_state::app::StatesPlugin);
    app.init_state::<country_core::app_loading_state::AppLoadingState>();
    app.add_systems(
        bevy_app::PostStartup,
        |mut next: bevy_ecs::prelude::ResMut<
            bevy_state::state::NextState<country_core::app_loading_state::AppLoadingState>,
        >| {
            next.set(country_core::app_loading_state::AppLoadingState::Game);
        },
    );
    country_core::startup::startup_session::set_startup_demo_mode(config.dev.demo);
    app.insert_resource(
        country_core::asset_libraries::configwatch::ConfigWatch::new(config.dev.live_link()),
    );
    app.init_resource::<country_core::meta_commander::MetaCommander>();
    app.init_resource::<country_core::resources::glade_settings::GladeSettings>();
    // B run_game_thread@1405a194a clears EDX before the named insert@14012c910.
    // init_clearing later publishes the actual uploaded emitter prefix count.
    app.insert_resource(country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount(0));
    country_core::systems::save::add_save_load_events_and_resources(app.world_mut());
    #[cfg(not(any(windows, feature = "native-audio")))]
    crate::gameplay_startup::install_disabled_audio(&mut app);
    // B add_save_load_systems1409443b0: set43, exact ActiveSession presence
    // condition only. Runs each frame to publish real history/section changes.
    app.add_systems(bevy_app::Update,
        country_core::systems::save::save_game::save_game_system
            .in_set(GameSystemSet(43))
            .run_if(|active: Option<bevy_ecs::prelude::Res<country_core::resources::session::ActiveSession>>|active.is_some()));
    // Original host PostUpdate closure1403bf630: Reload2 -> Game1.
    // Existing OnEnter(Game) consumes the canonical NextSession and real loader.
    app.add_systems(bevy_app::PostUpdate,
        |state: bevy_ecs::prelude::Res<bevy_state::state::State<country_core::app_loading_state::AppLoadingState>>,
         mut next: bevy_ecs::prelude::ResMut<bevy_state::state::NextState<country_core::app_loading_state::AppLoadingState>>| {
            if *state.get()==country_core::app_loading_state::AppLoadingState::Reload {
                next.set(country_core::app_loading_state::AppLoadingState::Game);
            }
        });
    // Other domains retain their own OnExit consumers. This is the complete
    // original host wall/unload consumer, not a blanket despawn replacement.
    // Native140b3f550 outer chain: begin precedes the inner unordered pair.
    // Keep the independently registered full host wall-unload system above.
    app.add_systems(
        bevy_state::state::OnExit(country_core::app_loading_state::AppLoadingState::Game),
        (
            country_core::startup::clearing::begin_glade_unload,
            (
                country_core::startup::clearing::uninit_clearing::<crate::gameplay_startup::GameplayAudioStudio>,
                country_core::startup::clearing::despawn_glade_entities,
            ),
        ).chain(),
    );
    app.add_systems(
        bevy_app::PreStartup,
        country_core::game_profile::load_game_profile,
    );

    // add_stages @0x140945ed0 / enum_iterator map @0x14130f350. Pairwise
    // public Bevy edges express the same chain without recreating its Vec,
    // interner or ScheduleConfigs representation.
    for stage in 0..44 {
        app.configure_sets(
            bevy_app::Update,
            GameSystemSet(stage).before(GameSystemSet(stage + 1)),
        );
    }
    // Original first add_to_app tuple @run_game_thread decomp L1248.
    app.add_plugins((
        InputPlugin,
        bevy_time::TimePlugin,
        country_core::systems::backend::icc::IccProfilePlugin,
    ));
    // Aspect-ratio call and constructor are adjacent original instructions
    // @0x14059dba3 and @0x14059dbe3. Never use MainCamera::default's zero buffer.
    let camera = MainCamera::new(window.aspect_ratio());
    // Original run_game_thread @14059ef47..efbd copies this camera's actual
    // four matrices into RenderTemporalState, then initializes jitter/index0.
    app.insert_resource(country_core::render::RenderTemporalState::new(
        country_core::render::FrameCameraMatrices {
            projection: camera.read_matrix(0),
            inverse_projection: camera.read_matrix(0x40),
            view: camera.read_matrix(0x80),
            inverse_view: camera.read_matrix(0xc0),
        },
    ));
    app.insert_resource(camera);
    app.init_resource::<utils::camera::TaaJitterSet>();
    // These are original insert_resources defaults, not a fabricated glade.
    app.init_resource::<country_core::resources::time_of_day::TimeOfDay>();
    app.init_resource::<country_core::resources::time_of_day::TodSequence>();
    app.init_resource::<country_core::resources::weather::Weather>();
    app.init_resource::<country_core::render::atmosphere::DerivedAtmosphere>();
    // insert_resources @14094108c..10e2: actual eye (24,48,24), target0.
    app.insert_resource(
        country_core::render::directional_light::DirectionalLight::look_at(
            glam::Vec3::new(24.0, 48.0, 24.0),
            glam::Vec3::ZERO,
        ),
    );
    // add_wall_events @140493cf5..3d0c clears the complete13-byte value;
    // decorator_plugin @1403442fc..44322 supplies an empty map with real seeds.
    app.init_resource::<country_core::resources::editing_highlight::EditingHighlight>();
    // InnerWalls::new @140859f90 creates the actual owning map/id counter.
    // Saved commands later populate this same authority; no snapshot is made.
    app.insert_resource(country_core::resources::walls::InnerWalls::new());
    // TreeStorage is inserted by TreePlugin140650520 (add_original_plugins).
    app.init_resource::<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>();
    // Original full-game tuple immediately follows MainCamera insertion with
    // ImageFormationConfigs::default @1408ae7f0.
    app.init_resource::<country_core::resources::image_formation::ImageFormationConfigs>();
    // Original run_game_thread @14059dbef..dc34 creates one shared glue;
    // @1405a0100 inserts it into gameplay, and @1405a403c starts the one
    // `bvh_builder_worker` thread with that same owner. Loading has neither.
    let rt_glue = country_core::resources::rt::RtBvhBuilderGlueResource::default();
    let rt_worker = country_core::resources::rt::bvh_builder::RtBvhWorker::spawn(rt_glue.clone())
        .context("starting original bvh_builder_worker")?;
    app.init_resource::<country_core::resources::rt::RtWorld>();
    app.insert_resource(rt_glue);
    app.insert_resource(rt_worker);
    app.add_event::<country_core::resources::rt::UpdateRtWorldEvent>();
    install_history_and_wall_runtime(&mut app, config.dev.demo);
    startup_construction_runtime::install(&mut app);
    // Preserve actual domain owner constructors without mounting incomplete
    // edit/UI helpers merely to initialize their resources.
    crate::toolbar_plugin::build(&mut app);
    crate::decorator_plugin::build_startup_resources(&mut app);
    crate::wall_constructor_plugin::build(&mut app);
    // Original plugin tuple members WaterPlugin 0x1406501d0 / GladeBorderPlugin 0x140650ab0.
    crate::app_systems::WallSystems::add_wall_events(&mut app);
    app.add_plugins(system_water::WaterPlugin);
    app.add_plugins(system_glade_border::GladeBorderPlugin);
    // Original anonymous creator @0x140af2200; its registration schedule is OPEN (Startup here:
    // its Commands are applied before the first Update that reads GladeBorder).
    app.add_systems(bevy_app::Startup, country_core::resources::glade_border::setup_glade_border);
    system_color::ColorSystems::add_color(&mut app);
    app.add_event::<toolbar::ui_interaction::UiDragEvent>();
    app.add_event::<toolbar::ui_interaction::UiJustBecameInFocusEvent>();
    app.add_event::<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>();
    app.add_event::<country_core::resources::events::ModeJustChanged>();
    app.add_event::<country_core::systems::ui::resources::widget_params::PlayUiSound>();
    app.add_event::<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>();

    app.insert_resource(window);
    // Original insert_resources @140940e35/e4a: both native tag 0.
    app.init_resource::<country_core::systems::mode_manager::AppMode>();
    app.init_resource::<country_core::systems::mode_manager::PreviousAppMode>();
    // insert_resources 0x140940e90: AdjWallHeightModeCache (all-None idle state).
    app.init_resource::<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>();
    // 0x140940f7e: GladeColorCustomization (original default @1408c5600).
    app.init_resource::<country_core::resources::glade_color_customization::GladeColorCustomization>();
    app.insert_resource(DebugGui::default());
    app.insert_resource(ActiveInputModeTracker::new_default_mouse_keyboard(
        std::time::Duration::ZERO,
    ));
    app.insert_resource(
        country_core::render::render_config::FrameRenderConfig::at_startup(window, &config.video),
    );
    app.insert_resource(
        country_core::render::adaptive_render_state::AdaptiveRenderState::at_startup(
            cli.ignore_vram_budget,
        ),
    );
    app.insert_resource(config);
    app.insert_resource(ApplicationShaderLibrary(Arc::new(
        country_core::asset_libraries::shader_library::AssetShaderLibrary::new(),
    )));
    // Full-game tuple @0x14059dedf..deea supplies (false, false).
    insert_vermeer_resources(&mut app, false, false);
    // Non-GPU owners from insert_rendering_resources @0x140940190. The
    // original constructors create empty owning libraries; their later loads
    // allocate through the renderer's real device, never a synthetic handle.
    app.insert_resource(country_core::render::frame_constants::FrameConstants::default());
    // 0x1409403c3: TerrainEditingData. Its constructor stores an empty map with fresh
    // RandomState keys, `rasterized = false` at +0x40, and a qword 1 at +0x38 that the
    // recovered struct does not name (OPEN); `Default` reproduces everything else.
    app.insert_resource(country_core::resources::terrain_editing::TerrainEditingData(
        Default::default(),
    ));
    // 0x140940882: ActiveRasterStrokesStorage (six empty stroke slots).
    app.init_resource::<country_core::resources::active_raster_stroke::ActiveRasterStrokesStorage>();
    app.insert_resource(country_core::asset_libraries::mesh_library::AssetMeshLibrary::new());
    app.insert_resource(country_core::asset_libraries::vao_library::AssetVAOLibrary::new());
    app.insert_resource(
        country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary::new(),
    );
    // resources::insert_resources @0x140940eb4 writes word 1 to +0x30:
    // ran=true, paused=false; all earlier GameTime fields are zero.
    app.insert_resource(GameClock {
        ran: true,
        ..Default::default()
    });
    app.init_resource::<ActionSetValues>();
    app.init_resource::<ActionSetStates>();
    let demo = app.world().resource::<GameConfig>().dev.demo;
    app.insert_resource(country_core::startup::init_photomode_state::PhotomodeState::new(demo));
    // Gameplay can also be constructed by the asynchronous loader, without
    // build_game's platform setup. Every gameplay World owns this UI state.
    app.init_resource::<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>();
    app.init_resource::<country_core::resources::gui::PhotomodeScreen>();
    app.add_event::<country_core::resources::photo_mode::PhotoModeCmd>();
    app.add_event::<country_core::systems::backend::photomode::PhotoModeAudioParameter>();
    app.add_event::<toolbar::ui_interaction::UiClickEvent>();
    // Original toolbar::impl1::build @14068dd10..dd96: the photo tuple is
    // chain_ignore_deferred, set8, gated by AppLoadingState::Game.
    app.add_systems(bevy_app::Update, (
        country_core::systems::backend::photomode::update_photomode_visibility,
        country_core::systems::backend::photomode::process_photomode_transitions,
        toolbar::photomode::timelapse::restore_after_timelapse,
        country_core::systems::backend::photomode::finish_photomode_frame,
    ).chain_ignore_deferred().in_set(GameSystemSet(8)).run_if(
        bevy_state::condition::in_state(country_core::app_loading_state::AppLoadingState::Game)));
    app.init_resource::<EscMenuState>();
    app.init_resource::<country_core::resources::onboarding::WelcomeScreenState>();
    app.init_resource::<NextEscMenuState>();
    app.init_resource::<PrevEscMenuState>();
    app.add_event::<country_core::resources::events::EscapeMenuJustOpened>();
    app.insert_resource(country_core::systems::steam::SteamOverlayOpenResources {
        open_browser: Some(Arc::new(|url| webbrowser::open(url).map_err(|error| error.to_string()))),
        ..Default::default()
    });
    app.add_event::<country_core::resources::window::OsWindowCmd>();
    app.add_systems(bevy_app::Update,
        country_core::systems::steam::forward_browser_window_commands.after(GameSystemSet(39)));
    // run_game_thread decomp L3503..3517 clears the block covering +0xa.
    app.insert_resource(TimeOfDayScreen {
        visible: false,
        ..Default::default()
    });
    app.init_resource::<CursorCapturedPosition>();
    // resources::insert_resources @0x140940dce..0x140940dd7 passes two
    // zero floats to the named CursorPositionSS insertion @0x14153cc00.
    app.insert_resource(CursorPositionSS([0.0, 0.0].into()));
    app.init_resource::<country_core::resources::shader_refactoring_state::ShaderRefactoringState>(
    );
    register_input_events(app.world_mut());
    install_gameplay_device_input(&mut app);
    // MainCameraPlugin@1409125e0: reset precedes snapshot in set9; the
    // active snapshot then main/paused tuple is chained without deferred flush.
    app.init_resource::<country_core::systems::main_camera::CameraSnapshot>();
    app.init_resource::<country_core::resources::read_mouse_depth_value::MouseDepthValue>();
    app.insert_resource(country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::new());
    app.insert_resource(country_core::resources::mouse_terrain_raycast::CameraCursorRay::new());
    app.add_event::<country_core::systems::main_camera::MainCameraMoved>();
    app.add_event::<country_core::resources::onboarding::NotifyHintSystemCameraMoved>();
    app.add_event::<country_core::resources::window::AudioCursorMoved>();
    app.add_systems(
        bevy_app::Update,
        country_core::systems::main_camera::reset_camera
            .in_set(GameSystemSet(9))
            .before(country_core::systems::main_camera::camera_snapshot)
            .run_if(bevy_ecs::schedule::common_conditions::on_event::<
                country_core::resources::session::NewSessionStartedCmd,
            >),
    );
    app.add_systems(
        bevy_app::Update,
        (
            country_core::systems::main_camera::camera_snapshot.run_if(is_gameplay_active),
            (
                country_core::systems::main_camera::main_camera_update.run_if(is_gameplay_active),
                country_core::systems::main_camera::paused_main_camera_update
                    .run_if(bevy_ecs::schedule::common_conditions::not(is_gameplay_active)),
            ),
        ).chain_ignore_deferred().in_set(GameSystemSet(9)),
    );
    app.add_systems(
        bevy_app::Update,
        (
            country_core::resources::mouse_terrain_raycast::camera_mouse_ray,
            country_core::resources::mouse_terrain_raycast::mouse_terrain_raycast,
            (
                country_core::resources::mouse_capture::mouse_capture,
                country_core::resources::read_mouse_depth_value::read_mouse_depth_value_from_gpu,
            ),
        ).chain_ignore_deferred().in_set(GameSystemSet(10)).run_if(is_gameplay_active),
    );
    app.add_event::<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved>();
    app.add_event::<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>();
    app.add_event::<country_core::systems::backend::input::translate_input::MouseWheel>();
    app.add_event::<country_core::resources::events::ShutdownGameCmd>();
    app.add_event::<country_core::systems::save::load_game::LoadingGameComplete>();
    app.add_event::<country_core::systems::mode_manager::HistoryCommand>();
    app.add_event::<country_core::systems::wall::structure_outline::OutlineStairsCmd>();
    app.add_event::<country_core::resources::session::NewSessionStartedCmd>();
    app.add_event::<country_core::resources::session::SessionLoadedCmd>();
    app.add_event::<country_core::resources::mouse_capture::CursorCaptureCmd>();
    app.add_event::<country_core::resources::mouse_capture::OsCursorCaptureCmd>();
    app.add_event::<country_core::resources::mouse_capture::OsCursorVisibilityCmd>();
    app.add_systems(
        bevy_app::PostUpdate,
        country_core::resources::shutdown::initiate_game_shutdown,
    );
    app.add_event::<country_core::game_profile::SaveGameProfileCmd>();
    app.add_systems(
        bevy_app::Update,
        country_core::game_profile::save_game_profile
            .in_set(GameSystemSet(43))
            .run_if(bevy_ecs::schedule::common_conditions::on_event::<
                country_core::game_profile::SaveGameProfileCmd>),
    );
    // Original in_set @0x14168c0e0 chooses GameSystemSet discriminant 3.
    // No on_event guard: its value rotation is an unconditional frame step.
    app.add_systems(
        bevy_app::Update,
        country_core::systems::backend::input::reset_input_state::reset_input_state
            .in_set(GameSystemSet(3)),
    );
    SessionFlowPlugin.build(app.world_mut());
    app.add_event::<country_core::systems::save::save_game::DeleteSessionCmd>();
    app.add_event::<country_core::systems::save::save_game::SavingGameComplete>();
    app.add_event::<country_core::resources::gui::EscapeMenuJustClosed>();
    // SessionFlowPlugin::build's original Startup producer and Update event
    // consumers are kept in separate schedules (the old core helper combines
    // them and would otherwise scan the save directory every frame).
    app.add_systems(
        bevy_app::Startup,
        country_core::resources::session::refresh_session_list_system,
    );
    use bevy_ecs::schedule::Condition;
    use bevy_ecs::schedule::common_conditions::on_event;
    app.add_systems(
        bevy_app::Update,
        (
            country_core::resources::session::delete_session_system
                .run_if(on_event::<country_core::systems::save::save_game::DeleteSessionCmd>),
            country_core::resources::session::refresh_session_list_system.run_if(
                on_event::<country_core::systems::save::save_game::DeleteSessionCmd>
                    .or(on_event::<country_core::systems::save::save_game::SavingGameComplete>),
            ),
        )
            .chain(),
    );
    // Original MenuPlugin14068dee6 /14068e29e: Load36 + in_state(Game).
    // It consumes the same owning session/menu/history resources as Escape39.
    app.add_systems(
        bevy_app::Update,
        toolbar::menu::load_ui::ui
            .in_set(GameSystemSet(36))
            .run_if(bevy_state::condition::in_state(
                country_core::app_loading_state::AppLoadingState::Game,
            )),
    );
    // Original welcome141ebe130 shares Game/set36 and the canonical MEI,
    // profile and reveal owners. Its own state machine controls visibility.
    app.add_systems(
        bevy_app::Update,
        toolbar::menu::welcome::ui
            .in_set(GameSystemSet(36))
            .run_if(bevy_state::condition::in_state(
                country_core::app_loading_state::AppLoadingState::Game,
            )),
    );
    let mut gui = Schedule::new(bevy_app::Update);
    // This schedule is owned by the exclusive gui system, not by `app`, so the
    // executor policy applied to the App's schedules cannot reach it.
    apply_ecs_executor_policy_to(&mut gui);
    configure_gui_consumers(app.world_mut(), &mut gui);
    gui.add_systems(crate::gui::gui_debug::show_camera_snapshot_panel
        .after(country_core::systems::backend::gui::show_debug_controls));
    // Reuse its real systems without replacing the existing Update schedule.
    gui.add_systems(country_core::resources::gui::advance_menu_state_system);
    gui.add_systems(crate::gui::gui_escape_menu::do_gui_escape_menu
        .before(country_core::resources::gui::advance_menu_state_system));
    let gui_system = move |world: &mut World| gui.run(world);
    // Native1405a0512 calls GUI tuple in_set140611040 with stage0x27=39.
    // The separate time tuple140aad7d0 belongs to stage2; it is not the GUI.
    app.add_systems(
        bevy_app::Update,
        country_core::systems::time_update::time_update.in_set(GameSystemSet(2)),
    );
    app.add_systems(bevy_app::Update, gui_system.in_set(GameSystemSet(39)));
    // Native @14094113d builds the resource_exists<NextSession> condition;
    // @140941158 supplies OnEnter state1=Game. Its vtable @142ccf9d0 binds
    // load_game_session @14166e7d0 through run_unsafe @14173e110.
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        country_core::startup::startup_session::load_game_session.run_if(
            bevy_ecs::schedule::common_conditions::resource_exists::<
                country_core::resources::session::NextSession,
            >,
        ),
    );
    // The sequence owner is populated from this actual session's asset file;
    // an empty constructor is never used as the first TOD sample. Commands
    // from startup_session become visible across this Bevy dependency edge.
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        country_core::startup::load_glade_tod_and_weather::load_glade_tod_and_weather
            .after(country_core::startup::startup_session::load_game_session),
    );
    // Original session tuple140839b80's first descriptor140839c98 resets the
    // whole photo owner; preserve the distinct selective photo-entry method.
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        country_core::startup::init_photomode_state::init_photomode_state
            .after(country_core::startup::startup_session::load_game_session),
    );
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        country_core::startup::load_glade_grading::load_glade_grading
            .after(country_core::startup::startup_session::load_game_session),
    );
    // 原版 `update_color_library` 0x14093b530：会话主题的 colors.json 上传到调色板（OnEnter(Game)）。
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        country_core::asset_libraries::color_library::update_color_library_for_session
            .after(country_core::startup::startup_session::load_game_session),
    );
    // 原版 OnEnter(Game) 会话链（add_game_startup_systems 0x140941150，tuple 0x140839b80）第 7 成员。
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        country_core::startup::load_textures::load_glade_texture_mappings
            .after(country_core::startup::startup_session::load_game_session),
    );
    // Device-owned per-session scene producers are installed together by
    // gameplay_startup, in the original seven-item order. Do not preload bare
    // meshes here: cached names would bypass their full animation/drawable work.
    // Original batch4 typed add_systems helper140b67b50 is Startup.
    app.add_systems(
        bevy_app::Startup,
        country_core::startup::startup_time_of_day::startup_time_of_day,
    );
    // The legacy-load event is produced by load_game in Update, after OnEnter.
    // Both original readers run during Game updates; the converter must also
    // run on no-event frames to allow the deferred prefab contents to arrive.
    app.add_systems(
        bevy_app::Update,
        // convert_preplaced_trees belongs to TreePlugin140650520 (set 0x1b chain).
        country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees
            .run_if(bevy_state::condition::in_state(
                country_core::app_loading_state::AppLoadingState::Game,
            )),
    );
    // Native @1409443de supplies set12 to the original load_game wrapper:
    // helper @14174d040 uses vtable @142cd2d78 (run_unsafe @141749a10),
    // then @1409443ec supplies in_state(Game). No provisional Update chain.
    app.add_systems(
        bevy_app::Update,
        country_core::systems::save::load_game::load_game_system
            .in_set(GameSystemSet(12))
            .run_if(bevy_state::condition::in_state(
                country_core::app_loading_state::AppLoadingState::Game,
            )),
    );
    // Original14059fc33 supplies36 to typed in_set helper140524400.
    // No invented on_event/AppState gate: sticky Local retries after replay.
    app.add_systems(bevy_app::Update,
        country_core::systems::save::load_completion_checker::check_load_completion_system
            .in_set(GameSystemSet(36)));
    // Original full-game registrations: weather in set36 and the complete
    // fourteen-resource frame writer in set44. ActiveSession comes from the
    // real OnEnter(Game) producer above; loading App has a different schedule.
    app.add_systems(
        bevy_app::Update,
        country_core::systems::weather_system::weather_system.in_set(GameSystemSet(36)),
    );
    let time_of_day_without_core = country_core::systems::time_of_day::time_of_day_audio_disabled
        .in_set(GameSystemSet(36)).run_if(is_gameplay_active);
    #[cfg(any(windows, feature = "native-audio"))]
    let time_of_day_without_core = time_of_day_without_core.run_if(
        bevy_ecs::schedule::common_conditions::not(bevy_ecs::schedule::common_conditions::resource_exists::<
            country_core::resources::audio_resources::AudioCore<country_core::resources::native_audio::NativeStudio>>));
    app.add_systems(bevy_app::Update, time_of_day_without_core);
    app.add_systems(
        bevy_app::Update,
        country_core::systems::backend::prepare_frame_constants::prepare_frame_constants
            .in_set(GameSystemSet(44)),
    );
    // @14059fc70 passes44 to helper1403c7a30's original unchained tuple:
    // prepare_frame_constants, update_rt_world, upload_rt_world. No new gate
    // or ordering is inferred between its interior-lock upload and update.
    app.add_systems(
        bevy_app::Update,
        (
            country_core::resources::rt::update_rt_world,
            country_core::resources::rt::upload_rt_world_system,
        )
            .in_set(GameSystemSet(44)),
    );
    debug_assert!(app.world().contains_resource::<SavedSessionList>());
    apply_ecs_executor_policy(&mut app);
    Ok(app)
}

/// Sizes the shared compute task pool before anything creates it with defaults.
/// One pool for the ECS executor (game thread) and rhapsody's parallel command
/// recording (render thread); two cores stay with those two threads. `TG_ECS_THREADS`
/// overrides the count. `get_or_init` is first-come, so tests that initialise
/// their own pool are unaffected.
fn init_ecs_task_pool() {
    use bevy_tasks::{ComputeTaskPool, TaskPoolBuilder};
    let cores = std::thread::available_parallelism().map_or(4, |n| n.get());
    let threads = std::env::var("TG_ECS_THREADS")
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|threads| *threads > 0)
        .unwrap_or_else(|| cores.saturating_sub(2).clamp(2, 8));
    ComputeTaskPool::get_or_init(|| {
        TaskPoolBuilder::new()
            .thread_name("tg-compute".into())
            .num_threads(threads)
            .build()
    });
}

/// Executor policy. The `ecs-multithreaded` feature (on by default) runs every
/// schedule on bevy's multi-threaded executor with the conflicting, unordered
/// system pairs pinned to their single-threaded order (`ecs_order`).
/// `TG_ECS_EXECUTOR=single` forces the single-threaded executor on every schedule
/// of a feature-enabled build, and `TG_SCHEDULE_AMBIGUITY=1` logs every pair of
/// conflicting systems with no declared order (the audit that decides whether the
/// feature is safe: a real run reports 525 such pairs in `Update`).
fn apply_ecs_executor_policy(app: &mut App) {
    if !ecs_policy_active() {
        #[cfg(feature = "ecs-multithreaded")]
        crate::ecs_order::pin_and_parallelize(app.world_mut());
        return;
    }
    let mut schedules = app.world_mut().resource_mut::<bevy_ecs::schedule::Schedules>();
    for (_, schedule) in schedules.iter_mut() {
        apply_ecs_executor_policy_to(schedule);
    }
}

fn ecs_policy_active() -> bool {
    ecs_single_threaded() || ecs_ambiguity_detection()
}
fn ecs_single_threaded() -> bool {
    std::env::var("TG_ECS_EXECUTOR").is_ok_and(|value| value == "single")
}
fn ecs_ambiguity_detection() -> bool {
    std::env::var_os("TG_SCHEDULE_AMBIGUITY").is_some_and(|value| value == "1")
}

/// Applies the policy to one schedule (also used for schedules the App does not own).
fn apply_ecs_executor_policy_to(schedule: &mut Schedule) {
    use bevy_ecs::schedule::{ExecutorKind, LogLevel, ScheduleBuildSettings};
    if ecs_single_threaded() {
        schedule.set_executor_kind(ExecutorKind::SingleThreaded);
    }
    if ecs_ambiguity_detection() {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Warn,
            ..Default::default()
        });
    }
}

/// Original device-input slices in the actual gameplay App's numbered sets.
fn install_gameplay_device_input(app: &mut App) {
    use country_core::systems::backend::input::{
        calculate_input_delta, input_record_replay, input_replay_for_trailers,
        mod_key, translate_input, translate_input_to_cursor_moved, update_cursor_position,
    };
    let demo = app.world().resource::<country_core::game_config::GameConfig>().dev.demo;
    app.insert_resource(translate_input::DemoModeEnabled(demo));
    // Current retained InputFrame stores derived ActionSetStates. Begin its
    // private capture policy before the original set1 replay controls/time2.
    app.add_systems(bevy_app::Update,
        input_record_replay::begin_input_frame.before(GameSystemSet(1)));
    app.add_systems(bevy_app::Update,
        (country_core::meta_commander::commander_update_system,
         (input_record_replay::control_input_replay,
          input_replay_for_trailers::record_and_replay_input_for_trailers),
         input_record_replay::advance_input_replay_frame)
            .chain_ignore_deferred().in_set(GameSystemSet(1)));
    // Original set5's raw-value recording precedes derivation. Preserve this
    // host's refactored derived-state recording instead: calculate delta first
    // so recording/replay does not consume stale states from the prior frame.
    app.add_systems(bevy_app::Update,
        (translate_input_to_cursor_moved::translate_input_to_cursor_moved,
         translate_input::translate_input,
         calculate_input_delta::calculate_input_delta,
         input_record_replay::input_record_replay,
         (mod_key::update_mod_key_state, update_cursor_position::update_cursor_position))
            .chain_ignore_deferred().in_set(GameSystemSet(5)));
}

/// Original B owners and recovered slices of the global schedule. This is
/// deliberately distinct from the old isolated CPU scheduling helpers.
fn install_history_and_wall_runtime(app: &mut App, demo: bool) {
    use country_core::{
        resources::{
            history::{History, ReplayBuffer},
            terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
            walls::{
                decorator::{DecoratorIdGen, DeferredDecoratorOpEvents},
                decorator_archivist::LiveDecoratorArchivist,
                decorator_collision::DecoratorPhysicsColliders,
                decorator_storage::DecoratorStorage,
                public_walls::{OnWallChanged, OnWallDeleted, PublicWalls, WallPublicationDirty},
            },
            world_raster::DirtyTilePublisher,
            world_raster_defs::{
                Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, PathRaster,
            },
        },
        systems::{
            history, mode_manager,
            wall::{
                adjust_wall_height::{
                    AdjustWallHeightCmd, WallHeightAudio, WallHeightCommandFailed,
                    adjust_wall_height,
                },
                wall_state::{
                    publish_public_wall_state::{
                        WallPublicationTerrain, collect_wall_publication_dirty, once_startup_walls,
                        publish_public_wall_state, synchronize_wall_publication_terrain,
                    },
                    wall_spatial_hash::{WallSpatialHash, spatial_hash_update},
                },
            },
        },
    };
    // insert_history_resources @140943e10..f2e; native speed write @3e87.
    app.insert_resource(History::<country_core::resources::history::MaybePendingSnapshotId>::empty_glade());
    app.insert_resource(ReplayBuffer::new(1.0));
    app.init_resource::<history::PendingHistoryCommands>();
    app.init_resource::<history::replay_mode::ReplayModeElapsedTime>();
    app.init_resource::<history::replay_mode::HoldToFastReplay>();
    app.init_resource::<mode_manager::RestoreAppTo>();
    // insert_history_resources 0x140943eec：WallEraserArchivist（RestoreAppTo 之后、PendingHistoryCommands 之前）。
    app.init_resource::<country_core::systems::wall::eraser::archivist::WallEraserArchivist>();
    // Safe projection of the unengaged original ActionSetStates slots16/17.
    // ActionSetStates aliases MainActionSet; this is the same input owner.
    // Native startup1405a146f supplies tag2; this is a real inactive state,
    // not a substitute for the active lifecycle handled below.
    app.insert_resource(country_core::systems::timelapse_replay::TimelapseState::Inactive);
    app.init_resource::<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>();
    app.add_event::<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>();
    app.add_event::<country_core::systems::timelapse_replay::InterruptTimelapse>();
    app.add_event::<country_core::systems::timelapse_replay::TimelapseReplayFinished>();
    app.add_systems(bevy_app::PostUpdate, country_core::systems::timelapse_replay::cleanup_timelapse);
    // insert_terrain_resources @1409457c0 (TerrainSystems, add_original_plugins)
    // already created both CPU readback owners; derive the wall-publication view
    // from the actual TerrainHeightSsbo owner.
    let view = WallPublicationTerrain::from_height_data(
        app.world().resource::<TerrainHeightsData<TerrainHeightSsbo>>());
    app.insert_resource(view);
    // PathsPlugin @140b68e40: one Shared Path publisher and its three
    // Secondary cobblestone publishers, not four unrelated unique streams.
    let primary = DirtyTilePublisher::shared();
    let DirtyTilePublisher::Shared(shared) = &primary else {
        unreachable!()
    };
    app.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::secondary(
        shared,
    )));
    app.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::secondary(
        shared,
    )));
    app.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::secondary(
        shared,
    )));
    app.insert_resource(PathRaster::new(primary));
    // PathsPlugin@140b68e40 owns path stroke events; ui_delete_tree (TreePlugin) reads this one.
    // OPEN: the other path stroke events of that plugin are not yet registered here.
    app.add_event::<country_core::resources::path_stroke::PathDrawEvent>();
    // PathPlugin::build @140b68e40 inserts BrushSizes right after PathRaster (0x140b691c0).
    // Value bytes: qword @142ac56d0 = {0.8f, 2.0f} then dword 0x40400000 (3.0f) — exactly
    // the core PATH/WATER/GARDEN defaults.
    app.insert_resource(country_core::resources::brush_sizes::BrushSizes {
        path: country_core::resources::brush_sizes::PATH_DEFAULT,
        water: country_core::resources::brush_sizes::WATER_DEFAULT,
        garden: country_core::resources::brush_sizes::GARDEN_DEFAULT,
    });
    // Recovered body of PreStartup chain @140941125/1130. The full10-item
    // original chain is not yet assembled here; this follows the already
    // registered real profile producer and flushes Commands before Update19.
    app.add_systems(
        bevy_app::PreStartup,
        once_startup_walls.after(country_core::game_profile::load_game_profile),
    );
    // add_wall_events @1404939a6/3b99 and original new empty publishers.
    app.init_resource::<PublicWalls>();
    // Canonical inputs/outputs of the recovered base-brick section. Preserve
    // any existing owners; this does not mount the old aggregate CPU installer.
    app.init_resource::<country_core::resources::walls::public_walls::WallEntities>();
    app.init_resource::<country_core::resources::walls::public_walls::WallPlasters>();
    app.init_resource::<country_core::resources::walls::wall_holes::WallHoles>();
    app.init_resource::<country_core::systems::wall::construct_wall_bricks::WallBrickBuildState>();
    app.add_event::<country_core::systems::wall::construct_wall_bricks::WallBricksChanged>();
    app.add_event::<country_core::resources::roofs::commands::RecalculateWallBricks>();
    app.add_event::<country_core::resources::roofs::commands::WallGableChanged>();
    app.add_event::<country_core::resources::roofs::commands::WallRoofPresenceChanged>();
    app.add_event::<country_core::resources::roofs::commands::RoofGeometryChanged>();
    app.init_resource::<WallSpatialHash>();
    app.init_resource::<WallPublicationDirty>();
    app.init_resource::<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>();
    // DecoratorPlugin @140343120/313d/3377/3a3c; world plugin @140b685e0.
    // DecoratorPlugin::build 0x140342c10, insert_resource at 0x140344044.
    app.init_resource::<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>();
    app.init_resource::<DecoratorStorage>();
    app.init_resource::<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<
        system_decorator::decorator_visual::deco_differential_state::ChimneyAssemblyParams>>();
    app.init_resource::<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<
        system_decorator::decorator_visual::deco_differential_state::DoorStairsAssemblyParams>>();
    // B DecoratorPlugin 的八成员组第六项属于原 set0x24（36），不推导组内顺序。
    // 窗灯控制与组件由装饰物生产者、逐帧系统和 Nani 玻璃共同使用。
    app.init_resource::<country_core::resources::window_lights::GlobalWindowLightsControl>();
    app.add_event::<country_core::resources::events::LightWindowCmd>();
    app.add_systems(bevy_app::Update,
        system_decorator::window_lights::window_lights.in_set(GameSystemSet(36)));
    // 仅安装原卸载函数中窗灯资源的重建片段，不代替其余 owner 的完整清理。
    app.add_systems(
        bevy_state::state::OnExit(country_core::app_loading_state::AppLoadingState::Game),
        system_decorator::window_lights::window_lights_on_glade_unload,
    );
    // Original native assembly uses this canonical event; it is not PlayClutterSoundCmd.
    app.add_event::<country_core::systems::audio::PlayDecoratorSoundCmd>();
    app.init_resource::<LiveDecoratorArchivist>();
    app.init_resource::<DecoratorIdGen>();
    app.init_resource::<DeferredDecoratorOpEvents>();
    app.add_event::<country_core::resources::walls::decorator::DecoratorOpEvent>();
    // B DecoratorPlugin: 19-member chain is assigned to GameSystemSet25.
    // Keep original replay40-produced notifications for the following frame.
    app.add_systems(
        bevy_app::Update,
        country_core::resources::walls::decorator::unleash_decorator_op_events
            .in_set(GameSystemSet(25)),
    );
    app.init_resource::<DecoratorPhysicsColliders>();
    app.init_resource::<country_core::resources::stairs::stairs_state::StairsState>();
    app.init_resource::<country_core::systems::collision::world::RaycastWorld>();
    // Actual UI constructor @140945d25..d70, followed by a derived mode view.
    let ui = country_core::systems::ui::resources::ui_state::UiState::at_startup();
    app.insert_resource(mode_manager::ModeManagerUiContext::from_ui_state(&ui));
    app.insert_resource(ui);
    app.insert_resource(country_core::resources::toolbar::Toolbar::new(demo));
    app.add_event::<history::history_process::NewReplayCommandEvent>();
    app.add_event::<history::history_process::DebugHistoryMsg>();
    app.add_event::<country_core::systems::audio::PlayReplayAudio>();
    // Original add_audio_events registers this canonical event; App owns its
    // update lifecycle as well as its storage before the clutter writer runs.
    app.add_event::<country_core::systems::audio::PlayClutterSoundCmd>();
    app.add_event::<country_core::systems::timelapse_replay::InterruptTimelapse>();
    app.add_event::<history::HistoryReplayFailed>();
    app.add_event::<history::ReplayBucketFinished>();
    app.add_event::<history::replay_wall::WallHistoryReplayAudio>();
    app.add_event::<history::replay_wall::WallHistoryRestoreColor>();
    app.add_event::<AdjustWallHeightCmd>();
    app.add_event::<WallHeightAudio>();
    app.add_event::<WallHeightCommandFailed>();
    app.add_event::<OnWallChanged>();
    app.add_event::<OnWallDeleted>();
    app.add_event::<mode_manager::RequestAppModeChange>();
    app.add_event::<mode_manager::RequestToolbarCategoryChange>();
    app.add_event::<mode_manager::RequestHistoryCommand>();
    app.add_event::<mode_manager::UiHistoryPlaySelectAnimation>();
    app.add_event::<mode_manager::ModeManagerSound>();
    app.add_event::<mode_manager::ClearCanvasCmd>();
    app.add_event::<mode_manager::AppModeChanged>();
    app.add_event::<history::finish_replay::CmdUnfreezeSheep>();
    // run_game_thread @14059fb02 selects set10 for the UI/mode tuple.
    app.add_systems(
        bevy_app::Update,
        (
            mode_manager::synchronize_mode_manager_ui_context,
            mode_manager::mode_manager::<country_core::resources::history::EditType, country_core::resources::history::MaybePendingSnapshotId>,
            country_core::systems::timelapse_replay::start_timelapse,
            mode_manager::send_mode_just_changed_event.in_set(mode_manager::ModeChangePublication),
        )
            .chain_ignore_deferred()
            .in_set(GameSystemSet(10)),
    );
    // add_wall_systems 0x140493120：set 19/28/34/44、OnExit(Game) 等墙系统注册统一由
    // `WallSystems::add_wall_systems`（app_systems.rs）承担。
    crate::app_systems::WallSystems::add_wall_systems(app);
    // add_history_systems 0x1409438a0（Update；0x140b669c0 同一 SubApp::add_systems 实例，
    // 与 SaveLoadSystems 共用，PDB 名证明 Update）。四项一起提交：
    //   A 0x1409438d0  history_start_recording.in_set(14)（0x141738630）
    //   B 0x1409438dc  (history_finish_recording, history_process, history_clear_previous_state,
    //                  replay_mode, finish_replay, signifier_replay_active)
    //                  .chain_ignore_deferred().in_set(35)（tuple 0x141737fb0，in_set 0x1409438e1..390f）
    //   C 0x140943981  十元组 replay_path<Dirt/Cobble1/2/3>、replay_water、replay_garden、
    //                  replay_terrain、replay_wall、replay_auto_clutter、replay_stairs .in_set(40)
    //   D 0x140943a04  (finish_timelapse.run_if(resource_exists<ActiveSession>)@0x14094398f,
    //                  replay_pop_bucket, finish_replay).chain_ignore_deferred().in_set(41)
    // 记录系统在此（set14 / set35 头）；其余成员见下方各自注册（C 里 replay_path/replay_water/
    // replay_garden 在本仓库尚无对应系统，见 PLUGIN-WIRING 未闭合清单）。
    // add_history_systems 尾部 0x140943c83：register_event<FinishInputRecordingCmd>
    // （InstaCreateDeco/StairPoint 由 ClutterPlugin 0x140652884/0x1406528e0 注册，
    // InputReplayCommand 由 0x1409437d8 的 backend input 注册，此处不重复）。
    app.add_event::<history::recording::FinishInputRecordingCmd>();
    app.add_systems(
        bevy_app::Update,
        history::recording::history_start_recording::<
            country_core::resources::history::MaybePendingSnapshotId,
        >
            .in_set(GameSystemSet(14)),
    );
    app.add_systems(
        bevy_app::Update,
        (
            history::recording::history_finish_recording_system::<
                country_core::resources::history::MaybePendingSnapshotId,
            >,
            history::history_process_system::<country_core::resources::history::MaybePendingSnapshotId>,
            history::history_clear_previous_state,
            history::replay_mode::replay_mode,
            history::finish_replay::finish_replay,
            history::signifier_replay_active::signifier_replay_active,
        )
            .chain_ignore_deferred()
            .in_set(GameSystemSet(35)),
    );
    app.add_systems(
        bevy_app::Update,
        (
            history::replay_wall::replay_wall,
            history::replay_auto_clutter::replay_auto_clutter,
        )
            .in_set(GameSystemSet(40)),
    );
    history_runtime::install(app);
    app.add_systems(
        bevy_app::Update,
        (
            history::replay_pop_bucket_system,
            history::finish_replay::finish_replay,
            country_core::systems::timelapse_replay::finish_timelapse.run_if(
                bevy_ecs::schedule::common_conditions::resource_exists::<country_core::resources::session::ActiveSession>,
            ),
        )
            .chain_ignore_deferred()
            .in_set(GameSystemSet(41)),
    );
}

/// Original country_core::systems::is_gameplay_active @140a90e00,22B.
/// Kept local to this source host until the shared core systems export is
/// consolidated; the condition's two resource identities are canonical.
pub(crate) fn is_gameplay_active(
    time: bevy_ecs::prelude::Res<country_core::resources::time::GameTime>,
    state: bevy_ecs::prelude::Res<
        bevy_state::state::State<country_core::app_loading_state::AppLoadingState>,
    >,
) -> bool {
    time.ran && *state.get() == country_core::app_loading_state::AppLoadingState::Game
}

fn insert_vermeer_resources(app: &mut App, enabled: bool, hold: bool) {
    use country_core::systems::render_loop::vermeer::{
        VermeerDebugControl, VermeerPhase, VermeerState,
    };
    // Original VermeerPlugin::build @1408a8280. Shader PreStartup is owned
    // by the existing renderer; session exit releases the original background.
    app.insert_resource(VermeerState {
        phase: if enabled {
            VermeerPhase::Pending
        } else {
            VermeerPhase::Disabled
        },
        hold,
        ..Default::default()
    });
    app.insert_resource(VermeerDebugControl::default());
    app.add_systems(
        bevy_state::state::OnExit(country_core::app_loading_state::AppLoadingState::Game),
        (|mut state: bevy_ecs::prelude::ResMut<VermeerState>| state.deactivate())
            .before(crate::toolbar_plugin::clear_toolbar_session),
    );
    // Original PainterlyPlugin::build@1408a63e0 owns an independent Disabled state.
    app.init_resource::<country_core::systems::render_loop::painterly::PainterlyState>();
}

/// B's independent loading App (run_game_thread @1405a4380 onward). This
/// copies only the original shared CPU values; GPU libraries are separately
/// created by the same device after the renderer activates this App.
pub fn make_loading_app(gameplay: &App) -> anyhow::Result<App> {
    use anyhow::Context;
    use country_core::{
        asset_libraries::{
            mesh_library::AssetMeshLibrary, placement_data_library::AssetPlacementDataLibrary,
            ssbo_library::AssetSsboLibrary, vao_library::AssetVAOLibrary,
        },
        input::{
            ActiveInputModeTracker,
            action_set::{ActionSetStates, ActionSetValues},
        },
        render::{
            adaptive_render_state::AdaptiveRenderState, frame_constants::FrameConstants,
            render_config::FrameRenderConfig,
        },
        resources::{
            game_clock::GameClock, logo_animation::LogoAnimation, platform::RuntimePlatform,
            session::SavedSessionList,
        },
    };
    let source = gameplay.world();
    let mut app = App::new();
    // The loading and gameplay windows share this per-process launch policy;
    // never let a handoff restore fullscreen or persist the temporary extent.
    if let Some(policy) = source.get_resource::<crate::app_config::LaunchDisplayOverride>() {
        app.insert_resource(policy.clone());
    }
    // Original loading App @1405a4d93/95 supplies false, independently of gameplay.
    app.insert_resource(country_core::asset_libraries::configwatch::ConfigWatch::new(false));
    for stage in 0..44 {
        app.configure_sets(
            bevy_app::Update,
            GameSystemSet(stage).before(GameSystemSet(stage + 1)),
        );
    }
    app.add_plugins((
        InputPlugin,
        bevy_time::TimePlugin,
        country_core::systems::backend::icc::IccProfilePlugin,
    ));
    app.insert_resource(
        source
            .get_resource::<GameConfig>()
            .context("loading App: original GameConfig producer missing")?
            .clone(),
    );
    app.insert_resource(
        *source
            .get_resource::<WindowSize>()
            .context("loading App: WindowSize missing")?,
    );
    app.insert_resource(
        *source
            .get_resource::<FrameRenderConfig>()
            .context("loading App: FrameRenderConfig missing")?,
    );
    app.insert_resource(
        *source
            .get_resource::<AdaptiveRenderState>()
            .context("loading App: AdaptiveRenderState missing")?,
    );
    app.insert_resource(
        *source
            .get_resource::<RuntimePlatform>()
            .context("loading App: RuntimePlatform missing")?,
    );
    app.insert_resource(
        source
            .get_resource::<ActiveInputModeTracker>()
            .context("loading App: ActiveInputModeTracker missing")?
            .clone(),
    );
    app.insert_resource(ApplicationShaderLibrary(Arc::new(
        country_core::asset_libraries::shader_library::AssetShaderLibrary::new(),
    )));
    app.insert_resource(FrameConstants::default());
    app.insert_resource(AssetMeshLibrary::new());
    app.insert_resource(AssetVAOLibrary::new());
    app.insert_resource(AssetPlacementDataLibrary::new());
    app.insert_resource(AssetSsboLibrary::new());
    // run_game_thread @1405a501e explicitly passes false, independently of
    // gameplay's live-link configuration.
    app.insert_resource(country_core::asset_libraries::meshwatch::MeshWatch::new(
        false,
    ));
    app.insert_resource(GameClock {
        ran: true,
        ..Default::default()
    });
    app.init_resource::<ActionSetValues>();
    app.init_resource::<ActionSetStates>();
    app.init_resource::<SavedSessionList>();
    // @1405a4518 /451a supply true,true to loading tuple @1400cd450.
    insert_vermeer_resources(&mut app, true, true);
    app.init_resource::<LogoAnimation>();
    app.init_resource::<country_core::resources::gui::DebugGui>();
    app.init_resource::<toolbar::MeiContext>();
    app.init_resource::<toolbar::resouces_and_components::taffy_layout::TaffyLayout>();
    app.init_resource::<toolbar::toolbar_update::ToolbarSizeMult>();
    app.init_resource::<country_core::systems::ui::resources::widget_params::UiNavState>();
    country_core::systems::backend::input::register_input_events(app.world_mut());
    app.add_event::<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved>();
    app.add_event::<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>();
    app.add_event::<country_core::systems::backend::input::translate_input::MouseWheel>();
    // These startup callbacks are separate from the game's session load. No
    // LoadingGameComplete event is manufactured for this loading World.
    app.add_systems(
        bevy_app::Startup,
        country_core::resources::session::refresh_session_list_system,
    );
    app.add_event::<country_core::resources::events::ShutdownGameCmd>();
    // Same canonical close consumer as gameplay. The current OS host must
    // observe a loading-window close before it can stop the preparation worker.
    // Original loading event registration is B1405a4901; this adapter's exact
    // PostUpdate placement is not claimed as the unrecovered native tuple.
    app.add_systems(bevy_app::PostUpdate,
        country_core::resources::shutdown::initiate_game_shutdown);
    Ok(app)
}

/// The original async loader transfers a SubApp, not an App (which contains a
/// process runner). A failed producer/warmup is returned to the OS owner; it
/// never becomes a synthetic "ready" application.
#[derive(Debug)]
pub(crate) struct GameplayPreparationCancelled;

impl std::fmt::Display for GameplayPreparationCancelled {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("gameplay preparation cancelled")
    }
}

impl std::error::Error for GameplayPreparationCancelled {}

/// A local disposal guard for the same SubApp. In particular, a failed thread
/// spawn drops its captured closure before it can return ownership to us.
struct GameplayAppOwner(Option<bevy_app::SubApp>);

impl GameplayAppOwner {
    fn new(app: bevy_app::SubApp) -> Self { Self(Some(app)) }
    fn app_mut(&mut self) -> &mut bevy_app::SubApp {
        self.0.as_mut().expect("gameplay SubApp is still owned")
    }
    fn take(&mut self) -> bevy_app::SubApp {
        self.0.take().expect("gameplay SubApp is still owned")
    }
    fn dispose(&mut self) -> anyhow::Result<()> {
        let Some(mut app) = self.0.take() else { return Ok(()); };
        country_core::systems::save::save_thread::shutdown_save_thread(app.world_mut())
    }
}

impl Drop for GameplayAppOwner {
    fn drop(&mut self) {
        if let Err(error) = self.dispose() {
            log::error!("Save shutdown before gameplay World disposal: {error:#}");
        }
    }
}

fn dispose_gameplay_app(app: bevy_app::SubApp) -> anyhow::Result<()> {
    GameplayAppOwner::new(app).dispose()
}

fn publish_prepared_app(
    sender: &std::sync::mpsc::Sender<anyhow::Result<bevy_app::SubApp>>,
    app: bevy_app::SubApp,
) -> anyhow::Result<()> {
    match sender.send(Ok(app)) {
        Ok(()) => Ok(()),
        Err(std::sync::mpsc::SendError(Ok(app))) => {
            dispose_gameplay_app(app)?;
            anyhow::bail!("gameplay owner stopped receiving its prepared App");
        }
        Err(std::sync::mpsc::SendError(Err(error))) => Err(error),
    }
}

fn keep_first_preparation_failure(
    primary: anyhow::Result<()>,
    cleanup: anyhow::Result<()>,
) -> anyhow::Result<()> {
    match (primary, cleanup) {
        (Err(primary), Err(secondary)) => {
            log::error!("Additional gameplay preparation shutdown failure: {secondary:#}");
            Err(primary)
        }
        (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
        (Ok(()), Ok(())) => Ok(()),
    }
}

pub struct GameplayPreparation {
    receiver: std::sync::mpsc::Receiver<anyhow::Result<bevy_app::SubApp>>,
    pending: Option<bevy_app::SubApp>,
    cancel: Arc<std::sync::atomic::AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl GameplayPreparation {
    pub fn spawn(
        gameplay: bevy_app::SubApp,
        warm_up: impl FnOnce(&mut World) -> anyhow::Result<()> + Send + 'static,
    ) -> anyhow::Result<Self> {
        Self::spawn_with_initializer(gameplay, |_, _| Ok(()), warm_up)
    }

    pub(crate) fn spawn_with_initializer(
        gameplay: bevy_app::SubApp,
        initialize: impl FnOnce(&mut World, &std::sync::atomic::AtomicBool) -> anyhow::Result<()> + Send + 'static,
        warm_up: impl FnOnce(&mut World) -> anyhow::Result<()> + Send + 'static,
    ) -> anyhow::Result<Self> {
        use std::sync::atomic::{AtomicBool, Ordering};
        let (sender, receiver) = std::sync::mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let cancelled = cancel.clone();
        let mut gameplay_owner = GameplayAppOwner::new(gameplay);
        let worker = std::thread::Builder::new().name("async load".to_owned()).spawn(move || {
            // Host diagnostic boundary only: preserve the original load/ready
            // gates, but carry a worker panic back to the owning OS thread.
            let mut phase = "validate gameplay resources";
            let mut update_index = 0u64;
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Borrow the owned App so an error/panic cannot drop its World
                // before its save worker has disconnected and joined.
                let gameplay = gameplay_owner.app_mut();
                // The current partial startup must fail explicitly instead of
                // spinning forever without its original producer. AppMode and
                // LoadGameManager are actual inputs of load_game_system.
                anyhow::ensure!(gameplay.world().contains_resource::<country_core::systems::mode_manager::AppMode>(),
                    "B async load @0x1400d7440: original gameplay AppMode/startup producers are not registered");
                anyhow::ensure!(gameplay.world().contains_resource::<country_core::systems::save::load_game::LoadGameManager>(),
                    "B async load: original session/snapshot loading producer is not registered");
                if cancelled.load(Ordering::Acquire) { return Err(GameplayPreparationCancelled.into()); }
                phase = "register gameplay base shader programs";
                initialize(gameplay.world_mut(), &cancelled)?;
                if cancelled.load(Ordering::Acquire) { return Err(GameplayPreparationCancelled.into()); }
                log::info!(target: "tiny_glade::startup", "gameplay preparation: original load update loop begins");
                loop {
                    if cancelled.load(Ordering::Acquire) { return Err(GameplayPreparationCancelled.into()); }
                    // @1400d76e3 and Events<LoadingGameComplete> native TypeId
                    // 4d00324fb817a8a1/5846f8d25a034525, confirmed register_event
                    // @14151ffb0. It reads both live event buffers.
                    phase = "gameplay SubApp::update";
                    if update_index == 0 {
                        log::info!(target: "tiny_glade::startup", "gameplay preparation: first SubApp update begins");
                    }
                    gameplay.update();
                    update_index += 1;
                    if update_index == 1 {
                        log::info!(target: "tiny_glade::startup", "gameplay preparation: first SubApp update returned");
                    }
                    phase = "verify actual gameplay clutter Startup";
                    crate::gameplay_startup::require_clutter_catalog(gameplay.world())?;
                    phase = "read original LoadingGameComplete";
                    let events = gameplay.world().get_resource::<Events<country_core::systems::save::load_game::LoadingGameComplete>>()
                        .ok_or_else(|| anyhow::anyhow!("B async load: LoadingGameComplete event resource missing"))?;
                    if !events.is_empty() {
                        log::info!(target: "tiny_glade::startup",
                            "gameplay preparation: LoadingGameComplete observed after {update_index} updates");
                        break;
                    }
                }
                // A close request may arrive during the final update. Do not
                // start another device submission after observing cancellation.
                if cancelled.load(Ordering::Acquire) {
                    return Err(GameplayPreparationCancelled.into());
                }
                phase = "original gameplay graph warm-up";
                log::info!(target: "tiny_glade::startup", "gameplay preparation: original GPU graph warm-up begins");
                warm_up(gameplay.world_mut())?;
                log::info!(target: "tiny_glade::startup", "gameplay preparation: original GPU graph warm-up completed");
                if cancelled.load(Ordering::Acquire) {
                    return Err(GameplayPreparationCancelled.into());
                }
                Ok(())
            }))
            .unwrap_or_else(|payload| {
                let message = if let Some(message) = payload.downcast_ref::<String>() {
                    message.as_str()
                } else if let Some(message) = payload.downcast_ref::<&'static str>() {
                    *message
                } else {
                    "non-string panic payload"
                };
                Err(anyhow::anyhow!(
                    "async gameplay loader panicked during {phase} (completed updates: {update_index}): {message}"
                ))
            });
            match result {
                Ok(()) => {
                    if let Err(error) = publish_prepared_app(&sender, gameplay_owner.take()) {
                        log::error!("Prepared gameplay App could not be published: {error:#}");
                    } else {
                        log::info!(target: "tiny_glade::startup", "gameplay preparation: prepared SubApp published");
                    }
                }
                Err(primary) => {
                    // Record the originating failure before save/world disposal:
                    // a later shutdown error must not hide the actual load phase.
                    if !primary.is::<GameplayPreparationCancelled>() {
                        log::error!(target: "tiny_glade::startup",
                            "gameplay preparation failed during {phase} after {update_index} completed updates: {primary:#}");
                    }
                    let cleanup = gameplay_owner.dispose();
                    // Cancellation is benign only when its real save cleanup
                    // succeeded; never discard a cleanup failure as cancellation.
                    let error = if primary.is::<GameplayPreparationCancelled>() {
                        match cleanup { Ok(()) => primary, Err(error) => error }
                    } else {
                        if let Err(secondary) = cleanup {
                            log::error!("Save shutdown after gameplay producer failure: {secondary:#}");
                        }
                        primary
                    };
                    let _ = sender.send(Err(error));
                }
            }
        })?;
        Ok(Self {
            receiver,
            pending: None,
            cancel,
            worker: Some(worker),
        })
    }

    fn join_worker(&mut self) -> anyhow::Result<()> {
        let Some(worker) = self.worker.take() else { return Ok(()); };
        worker.join().map_err(|payload| {
            let message = if let Some(message) = payload.downcast_ref::<String>() {
                message.as_str()
            } else if let Some(message) = payload.downcast_ref::<&'static str>() {
                *message
            } else {
                "non-string panic payload"
            };
            anyhow::anyhow!("async gameplay worker panicked while completing: {message}")
        })
    }

    /// Finish all preparation work before the renderer starts device shutdown.
    /// Cancellation is an ordinary exit request; actual load/warmup failures
    /// retain their original error and are still returned to the OS owner.
    pub fn cancel_and_join(&mut self) -> anyhow::Result<()> {
        self.cancel.store(true, std::sync::atomic::Ordering::Release);
        let joined = self.join_worker();
        // Both received and queued Worlds must stop their save thread before
        // their scene/device owners are dropped and renderer shutdown begins.
        let pending_cleanup = self.pending.take().map_or(Ok(()), dispose_gameplay_app);
        let completion = match self.receiver.try_recv() {
            Ok(Err(error)) if !error.is::<GameplayPreparationCancelled>() => Err(error),
            Ok(Ok(app)) => dispose_gameplay_app(app),
            Ok(Err(_)) | Err(_) => Ok(()),
        };
        keep_first_preparation_failure(
            keep_first_preparation_failure(completion, joined), pending_cleanup)
    }

    /// Receive actual worker completion/errors without changing Logo or App
    /// ownership. Optional background decoding must never conceal a failed
    /// mandatory gameplay producer while its loading frame is suspended.
    pub(crate) fn poll_completion(&mut self) -> anyhow::Result<()> {
        use std::sync::mpsc::TryRecvError;
        if self.pending.is_none() {
            match self.receiver.try_recv() {
                Ok(Ok(app)) => {
                    let mut app = GameplayAppOwner::new(app);
                    self.join_worker()?;
                    self.pending = Some(app.take());
                    log::info!(target: "tiny_glade::startup", "loading App: prepared gameplay SubApp received");
                }
                Ok(Err(error)) => {
                    if let Err(secondary) = self.join_worker() {
                        log::error!("Async gameplay worker also failed while stopping: {secondary:#}");
                    }
                    return Err(error);
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    self.join_worker()?;
                    anyhow::bail!("original async gameplay loader stopped before returning its SubApp");
                }
            }
        }
        Ok(())
    }

    /// Original main_loop polls only at the logo's waiting stage and switches
    /// only after the final stage. Early errors are surfaced immediately in
    /// this host, because incomplete producers must not become a hung splash.
    pub fn poll(&mut self, loading: &mut App) -> anyhow::Result<bool> {
        use country_core::resources::logo_animation::LogoAnimation;
        self.poll_completion()?;
        if !loading.world().contains_resource::<LogoAnimation>() {
            // A missing loading producer must not turn into permission to
            // bypass the logo's waiting/final-stage gates. Dispose any received
            // gameplay owner through the same save-thread shutdown boundary.
            if let Some(gameplay) = self.pending.take() {
                if let Err(secondary) = dispose_gameplay_app(gameplay) {
                    log::error!("Save shutdown after missing loading LogoAnimation: {secondary:#}");
                }
            }
            anyhow::bail!("loading App: mandatory LogoAnimation producer is missing; gameplay handoff refused");
        }
        {
            let mut logo = loading.world_mut().resource_mut::<LogoAnimation>();
            if self.pending.is_some() && logo.is_waiting() {
                // finish_wait is intentionally still called by the same native
                // gate on every waiting frame. Log only its actual transition.
                if logo.progress[country_core::resources::logo_animation::WAIT_STAGE] != 1.0 {
                    log::info!(target: "tiny_glade::startup", "loading App: prepared gameplay releases Logo wait stage");
                }
                logo.finish_wait();
            }
            if !logo.is_finished() {
                return Ok(false);
            }
        }
        let Some(gameplay) = self.pending.take() else {
            return Ok(false);
        };
        log::info!(target: "tiny_glade::startup", "loading App: Logo finished; original owner handoff begins");
        let mut gameplay = GameplayAppOwner::new(gameplay);
        transfer_loading_resources(loading.world_mut(), gameplay.app_mut().world_mut())?;
        // Normally the independent loading World owns no save worker. Still
        // close any actual owner before replacing it; all schedules are stopped.
        country_core::systems::save::save_thread::shutdown_save_thread(loading.world_mut())?;
        *loading.main_mut() = gameplay.take();
        log::info!(target: "tiny_glade::startup", "loading App: gameplay SubApp handoff completed");
        Ok(true)
    }
}

impl Drop for GameplayPreparation {
    fn drop(&mut self) {
        if let Err(error) = self.cancel_and_join() {
            log::error!("Failed to stop async gameplay preparation: {error:#}");
        }
    }
}

/// Original skip_splash=true branch1405a4194..439d. There is no loading
/// App, async preparation, warmup or Started event on this native path.
pub(crate) fn finish_skipped_splash(world: &mut World) -> anyhow::Result<()> {
    use country_core::resources::gui::PendingEguiContextResource;
    let pending = world.remove_resource::<PendingEguiContextResource>()
        .context("B skipped splash: PendingEguiContextResource missing")?;
    world.insert_resource(pending.into_egui_context_resource());
    world.send_event(country_core::systems::save::load_game::StartupRevealFinished);
    Ok(())
}

fn transfer_loading_resources(from: &mut World, to: &mut World) -> anyhow::Result<()> {
    use country_core::{
        input::ActiveInputModeTracker,
        render::{adaptive_render_state::AdaptiveRenderState, render_config::FrameRenderConfig},
        resources::{gui::EguiContextResource, window::AvailableMonitors},
        systems::render_loop::vermeer::VermeerState,
    };
    fn take<T: Resource>(from: &mut World, to: &mut World) -> anyhow::Result<()> {
        let value = from.remove_resource::<T>().ok_or_else(|| {
            anyhow::anyhow!(
                "B loading→gameplay handoff: required {} is missing",
                std::any::type_name::<T>()
            )
        })?;
        to.insert_resource(value);
        Ok(())
    }
    // Validate the native reveal-source owner/device before moving any of the
    // six required resources. A frame-local graph index cannot cross Apps.
    crate::loading_graph::validate_saved_background_handoff(from, to)?;
    // Required move list, original main_loop @0x1404d9xxx..0x1404dacxx.
    take::<GameConfig>(from, to)?;
    take::<ActiveInputModeTracker>(from, to)?;
    take::<WindowSize>(from, to)?;
    take::<AvailableMonitors>(from, to)?;
    take::<FrameRenderConfig>(from, to)?;
    take::<AdaptiveRenderState>(from, to)?;
    // main_loop1404da0b5..15a: this exact TypeId is StartupRevealStarted.
    // It precedes Vermeer.into_disable_hold, after the six real owner moves.
    to.send_event(country_core::systems::save::load_game::StartupRevealStarted);
    let vermeer = from
        .remove_resource::<VermeerState>()
        .ok_or_else(|| anyhow::anyhow!("B handoff: VermeerState missing"))?;
    if let Some(background) = from.remove_resource::<crate::loading_graph::SavedLoadingBackgroundTexture>() {
        to.insert_resource(background);
    }
    to.insert_resource(vermeer.into_disable_hold());
    take::<EguiContextResource>(from, to)?;
    Ok(())
}

/// Temporary-window platform fallback from B make_temp_window: hide the old
/// window, create a matching Tiny Glade window, and request its initial redraw.
/// Vulkan retry policy belongs to the renderer; this function does no fake
/// device reset or copied window handle.
pub fn make_temp_window(
    event_loop: &winit::event_loop::EventLoopWindowTarget<()>,
    old_window: &winit::window::Window,
) -> anyhow::Result<winit::window::Window> {
    old_window.set_visible(false);
    let mut builder = winit::window::WindowBuilder::new()
        .with_title("Tiny Glade")
        .with_inner_size(old_window.inner_size());
    if let Ok(position) = old_window.outer_position() {
        builder = builder.with_position(position);
    }
    let window = builder.build(event_loop)?;
    window.request_redraw();
    Ok(window)
}

#[cfg(test)]
mod intro_history_integration {
    use super::*;
    use country_core::{
        resources::{
            history::{History, transactions::ObservedHistoryLedger},
            time::GameTime,
            walls::{InnerWalls, WallId, public_walls::PublicWalls},
        },
        systems::{
            history::PendingHistoryCommands,
            mode_manager::{AppMode, HistoryCommand},
        },
    };

    /// Original shipped intro has 13 entries, 26 internal groups (including
    /// empty and concurrent groups), and ends with exactly these three walls.
    /// This runs the production stage registration, including real Rapier
    /// bucket flushes, with no window/device or save-thread side effects.
    #[test]
    fn shipped_intro_reaches_the_same_live_wall_authority() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../country-core/tests/fixtures/original-intro-history-b.json");
        let parsed = country_core::systems::save::load_game::parse_saved_game_data(&path, &country_core::resources::glade_settings::GladeSettings::default())
            .expect("original intro sections decode");
        assert_eq!(parsed.chain_index, 12);
        assert_eq!(parsed.edits.len(), 13);
        let mut app = App::new();
        for stage in 0..44 {
            app.configure_sets(
                bevy_app::Update,
                GameSystemSet(stage).before(GameSystemSet(stage + 1)),
            );
        }
        install_history_and_wall_runtime(&mut app, false);
        app.init_resource::<AppMode>();
        app.init_resource::<country_core::systems::mode_manager::PreviousAppMode>();
        app.init_resource::<country_core::input::action_set::MainActionSet>();
        app.init_resource::<country_core::resources::gui::TimeOfDayScreen>();
        app.init_resource::<country_core::meta_commander::MetaCommander>();
        app.add_event::<country_core::systems::save::load_game::LoadingGameComplete>();
        app.add_event::<HistoryCommand>();
        app.insert_resource(InnerWalls::new());
        app.insert_resource(GameTime {
            delta: 1.0 / 60.0,
            ran: true,
            ..Default::default()
        });
        app.insert_resource(History::<country_core::resources::history::MaybePendingSnapshotId>::from_observed(ObservedHistoryLedger {
            entries: parsed.edits,
            cursor: -1,
            undo_anchor: parsed.history_start.max(0),
            resolved_snapshots: vec![],
            observed_count_58: 13,
            mutation_nonce: 0,
        }));
        app.world_mut()
            .resource_mut::<Events<HistoryCommand>>()
            .send_batch((0..13).map(|_| HistoryCommand {
                bytes: 0x03010101u32.to_le_bytes(),
            }));
        for _ in 0..64 {
            app.update();
        }
        let world = app.world();
        assert_eq!(world.resource::<History>().ledger().cursor, 12);
        assert!(
            world
                .resource::<country_core::resources::history::ReplayBuffer>()
                .is_empty()
        );
        assert!(world.resource::<PendingHistoryCommands>().0.is_empty());
        let inner = world.resource::<InnerWalls>();
        let mut ids: Vec<_> = inner.wall_ids().map(|id| id.0).collect();
        ids.sort_unstable();
        assert_eq!(ids, [27, 28, 29]);
        for (id, height, point_count) in [
            (27, 4.158195495605469f32, 11),
            (28, 1.5993374586105347f32, 6),
            (29, 3.0318603515625f32, 4),
        ] {
            let state = inner.get(WallId(id)).unwrap();
            assert_eq!(state.height().to_bits(), height.to_bits(), "wall {id}");
            assert_eq!(state.source().curve.points.len(), point_count);
            assert!(world.resource::<PublicWalls>().get(WallId(id)).is_some());
            assert!(world.resource::<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>()
                .segments_for_wall(WallId(id)).next().is_some(), "wall {id} spatial publication");
        }
        assert_eq!(world.resource::<PublicWalls>().wall_ids().count(), 3);
        assert!(
            world
                .resource::<Events<country_core::systems::history::HistoryReplayFailed>>()
                .is_empty()
        );
    }
}

#[cfg(test)]
mod startup_handoff_integration {
    use super::*;
    use country_core::{
        input::ActiveInputModeTracker,
        render::{adaptive_render_state::AdaptiveRenderState, render_config::{FrameRenderConfig, MonitorMatch, VideoMode}},
        resources::{gui::{EguiContextResource, PendingEguiContextResource}, logo_animation::LogoAnimation,
            window::{AvailableMonitor, AvailableMonitors}},
        systems::{render_loop::vermeer::{VermeerState, VermeerPhase}, save::load_game::{StartupRevealStarted, StartupRevealFinished}},
    };

    fn events(app: &mut App) {
        app.add_event::<StartupRevealStarted>().add_event::<StartupRevealFinished>();
    }

    fn six_owners(app: &mut App) -> (usize, usize, FrameRenderConfig) {
        let mut config = GameConfig::default();
        config.video.fps_limit = 117.5;
        config.lang_override = Some("original-loading-owner".to_owned());
        let config_allocation = config.lang_override.as_ref().unwrap().as_ptr() as usize;
        let window = WindowSize::new(1431, 913);
        let frame = FrameRenderConfig {
            extent_0: [1431,913], extent_8: [1171,733], extent_10: [1001,607],
            world_resolution_scale: 0.75, tgsr_resolution_scale: 0.5, flag_20: true,
        };
        let monitors = AvailableMonitors { monitors: vec![AvailableMonitor {
            monitor: MonitorMatch { name: Some("loading-monitor".into()), physical_position: [-1431,91], physical_size: [1431,913] },
            video_modes: vec![VideoMode { resolution: [1431,913], refresh_rate_millihertz: 144_001, bit_depth: 30 }],
        }] };
        let monitor_allocation = monitors.monitors.as_ptr() as usize;
        app.insert_resource(config);
        app.insert_resource(ActiveInputModeTracker::new_default_controller(std::time::Duration::from_millis(827)));
        app.insert_resource(window);
        app.insert_resource(monitors);
        app.insert_resource(frame);
        app.insert_resource(AdaptiveRenderState::at_startup(true));
        (config_allocation, monitor_allocation, frame)
    }

    #[test]
    fn ordinary_startup_waits_for_logo_then_moves_real_owners_and_starts_reveal() {
        let mut loading = App::new();
        let (config_ptr, monitors_ptr, expected_frame) = six_owners(&mut loading);
        events(&mut loading);
        let mut logo = LogoAnimation::default();
        logo.progress[3] = 0.25;
        loading.insert_resource(logo);
        let context = egui::Context::default();
        let marker = egui::Id::new("loading-egui-owner");
        context.data_mut(|data| data.insert_temp(marker, 912_u32));
        loading.insert_resource(EguiContextResource(context.clone()));
        loading.insert_resource(VermeerState { frame_count: 71, progress: 0.37,
            timer: 8.25, target_extent: [1431,913], phase: VermeerPhase::Ready,
            hold: true, freeze_camera: false, ..Default::default() });
        let loading_shaders = Arc::new(country_core::asset_libraries::shader_library::AssetShaderLibrary::new());
        loading.insert_resource(ApplicationShaderLibrary(loading_shaders.clone()));
        let mut gameplay = App::new();
        events(&mut gameplay);
        let gameplay_shaders = Arc::new(country_core::asset_libraries::shader_library::AssetShaderLibrary::new());
        gameplay.insert_resource(ApplicationShaderLibrary(gameplay_shaders.clone()));
        gameplay.insert_resource(WindowSize::new(1,2));
        gameplay.insert_resource(GameConfig::default());
        let (sender, receiver) = std::sync::mpsc::channel();
        sender.send(Ok(std::mem::take(gameplay.main_mut()))).unwrap();
        let mut preparation = GameplayPreparation { receiver, pending: None,
            cancel: Arc::new(std::sync::atomic::AtomicBool::new(false)), worker: None };
        assert!(!preparation.poll(&mut loading).unwrap());
        assert_eq!(loading.world().resource::<LogoAnimation>().progress[3],1.0);
        assert_eq!(loading.world().resource::<GameConfig>().video.fps_limit,117.5);
        assert!(loading.world().resource::<Events<StartupRevealStarted>>().is_empty());
        loading.world_mut().resource_mut::<LogoAnimation>().progress[4]=1.0;
        assert!(preparation.poll(&mut loading).unwrap());
        let world=loading.world();
        assert_eq!(world.resource::<GameConfig>().lang_override.as_ref().unwrap().as_ptr() as usize,config_ptr);
        assert_eq!(world.resource::<GameConfig>().video.fps_limit,117.5);
        assert_eq!(*world.resource::<ActiveInputModeTracker>(),ActiveInputModeTracker::new_default_controller(std::time::Duration::from_millis(827)));
        assert_eq!(*world.resource::<WindowSize>(),WindowSize::new(1431,913));
        let monitors=world.resource::<AvailableMonitors>();
        assert_eq!(monitors.monitors.as_ptr() as usize,monitors_ptr);
        assert_eq!(monitors.monitors[0].monitor.name.as_deref(),Some("loading-monitor"));
        assert_eq!(monitors.monitors[0].video_modes[0].refresh_rate_millihertz,144_001);
        assert_eq!(*world.resource::<FrameRenderConfig>(),expected_frame);
        assert_eq!(*world.resource::<AdaptiveRenderState>(),AdaptiveRenderState::at_startup(true));
        assert_eq!(world.resource::<Events<StartupRevealStarted>>().len(),1);
        assert!(world.resource::<Events<StartupRevealFinished>>().is_empty());
        let reveal=world.resource::<VermeerState>();
        assert_eq!(reveal.frame_count,71); assert_eq!(reveal.progress,0.37);
        assert_eq!(reveal.timer,8.25); assert_eq!(reveal.target_extent,[1431,913]);
        assert_eq!(reveal.phase,VermeerPhase::Ready); assert!(!reveal.hold); assert!(!reveal.freeze_camera);
        assert_eq!(world.resource::<EguiContextResource>().0.data(|data|data.get_temp::<u32>(marker)),Some(912));
        world.resource::<EguiContextResource>().0.data_mut(|data|data.insert_temp(marker,913_u32));
        assert_eq!(context.data(|data|data.get_temp::<u32>(marker)),Some(913));
        assert!(Arc::ptr_eq(&world.resource::<ApplicationShaderLibrary>().0,&gameplay_shaders));
        assert!(!Arc::ptr_eq(&world.resource::<ApplicationShaderLibrary>().0,&loading_shaders));
    }

    #[test]
    fn skipped_splash_moves_pending_context_and_finishes_without_started_or_loading_wait() {
        let mut app=App::new(); events(&mut app);
        let context=egui::Context::default();let marker=egui::Id::new("skip-context");
        context.data_mut(|data|data.insert_temp(marker,53_u32));
        app.insert_resource(PendingEguiContextResource(context.clone()));
        // No LoadingGameComplete, preparation manager, logo, or six-owner bundle
        // exists: the original skip path requires none of those objects.
        finish_skipped_splash(app.world_mut()).unwrap();
        assert!(!app.world().contains_resource::<PendingEguiContextResource>());
        assert_eq!(app.world().resource::<EguiContextResource>().0.data(|data|data.get_temp::<u32>(marker)),Some(53));
        app.world().resource::<EguiContextResource>().0.data_mut(|data|data.insert_temp(marker,54_u32));
        assert_eq!(context.data(|data|data.get_temp::<u32>(marker)),Some(54));
        assert_eq!(app.world().resource::<Events<StartupRevealFinished>>().len(),1);
        assert!(app.world().resource::<Events<StartupRevealStarted>>().is_empty());
    }

    #[test]
    fn started_is_published_after_all_six_moves_before_vermeer_and_egui_move() {
        let mut loading=App::new();six_owners(&mut loading);
        let context=egui::Context::default();
        loading.insert_resource(EguiContextResource(context));
        // Deliberately omit Vermeer: the real handoff exposes the exact native
        // mutation boundary rather than replacing failure with a ready marker.
        let mut gameplay=App::new();events(&mut gameplay);
        assert!(transfer_loading_resources(loading.world_mut(),gameplay.world_mut()).is_err());
        let from=loading.world();let to=gameplay.world();
        assert!(!from.contains_resource::<GameConfig>() && to.contains_resource::<GameConfig>());
        assert!(!from.contains_resource::<ActiveInputModeTracker>() && to.contains_resource::<ActiveInputModeTracker>());
        assert!(!from.contains_resource::<WindowSize>() && to.contains_resource::<WindowSize>());
        assert!(!from.contains_resource::<AvailableMonitors>() && to.contains_resource::<AvailableMonitors>());
        assert!(!from.contains_resource::<FrameRenderConfig>() && to.contains_resource::<FrameRenderConfig>());
        assert!(!from.contains_resource::<AdaptiveRenderState>() && to.contains_resource::<AdaptiveRenderState>());
        assert_eq!(to.resource::<Events<StartupRevealStarted>>().len(),1);
        assert!(from.contains_resource::<EguiContextResource>());
        assert!(!to.contains_resource::<EguiContextResource>());
    }

    #[test]
    fn missing_saved_background_owner_refuses_handoff_before_any_required_move() {
        use country_core::systems::render_loop::painterly::TexRef;
        let mut loading = App::new();
        six_owners(&mut loading);
        loading.insert_resource(EguiContextResource(egui::Context::default()));
        let mut vermeer = VermeerState::default();
        vermeer.activate(true, true);
        // A graph-local reference deliberately has no native managed owner.
        vermeer.background = Some(country_core::systems::render_loop::vermeer::ManagedTexture(TexRef(999)));
        loading.insert_resource(vermeer);
        let mut gameplay = App::new();
        events(&mut gameplay);
        let error = transfer_loading_resources(loading.world_mut(), gameplay.world_mut()).unwrap_err();
        assert!(error.to_string().contains("managed texture owner is missing"));
        assert!(loading.world().contains_resource::<GameConfig>());
        assert!(loading.world().contains_resource::<ActiveInputModeTracker>());
        assert!(loading.world().contains_resource::<WindowSize>());
        assert!(loading.world().contains_resource::<AvailableMonitors>());
        assert!(loading.world().contains_resource::<FrameRenderConfig>());
        assert!(loading.world().contains_resource::<AdaptiveRenderState>());
        assert!(loading.world().contains_resource::<EguiContextResource>());
        assert!(loading.world().resource::<VermeerState>().background.is_some());
        assert!(!gameplay.world().contains_resource::<GameConfig>());
        assert!(gameplay.world().resource::<Events<StartupRevealStarted>>().is_empty());
    }
}

#[cfg(test)]
mod gameplay_preparation_shutdown_tests {
    use super::*;
    use std::sync::{atomic::{AtomicBool, Ordering}, mpsc};

    fn prepared_worker(result: anyhow::Result<bevy_app::SubApp>) -> GameplayPreparation {
        let (sender, receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || { sender.send(result).unwrap(); });
        GameplayPreparation { receiver, pending: None,
            cancel: Arc::new(AtomicBool::new(false)), worker: Some(worker) }
    }

    #[test]
    fn cancellation_joins_worker_and_discards_only_typed_cancel() {
        let (sender, receiver) = mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let worker_cancel = cancel.clone();
        let stopped = Arc::new(AtomicBool::new(false));
        let worker_stopped = stopped.clone();
        let worker = std::thread::spawn(move || {
            while !worker_cancel.load(Ordering::Acquire) { std::thread::yield_now(); }
            worker_stopped.store(true, Ordering::Release);
            sender.send(Err(GameplayPreparationCancelled.into())).unwrap();
        });
        let mut preparation = GameplayPreparation { receiver, pending: None, cancel,
            worker: Some(worker) };
        preparation.cancel_and_join().unwrap();
        assert!(stopped.load(Ordering::Acquire));
        assert!(preparation.worker.is_none());
        preparation.cancel_and_join().unwrap();
    }

    #[test]
    fn cancellation_preserves_actual_producer_failure() {
        let mut preparation = prepared_worker(Err(anyhow::anyhow!("warmup producer failed")));
        let error = preparation.cancel_and_join().unwrap_err();
        assert_eq!(error.to_string(), "warmup producer failed");
        assert!(preparation.worker.is_none());
    }

    #[test]
    fn explicit_stop_releases_received_world() {
        let mut preparation = prepared_worker(Ok(bevy_app::SubApp::new()));
        preparation.pending = Some(preparation.receiver.recv().unwrap().unwrap());
        preparation.cancel_and_join().unwrap();
        assert!(preparation.pending.is_none());
        assert!(preparation.worker.is_none());
    }

    #[derive(bevy_ecs::resource::Resource)]
    struct WorldDropFlag(Arc<AtomicBool>);
    impl Drop for WorldDropFlag {
        fn drop(&mut self) { self.0.store(true, Ordering::Release); }
    }

    #[test]
    fn drop_waits_for_worker_and_releases_pending_or_queued_world() {
        for receive_before_drop in [false, true] {
            let released = Arc::new(AtomicBool::new(false));
            let stopped = Arc::new(AtomicBool::new(false));
            let worker_stopped = stopped.clone();
            let mut world = bevy_app::SubApp::new();
            world.world_mut().insert_resource(WorldDropFlag(released.clone()));
            let (sender, receiver) = mpsc::channel();
            let worker = std::thread::spawn(move || {
                let _ = sender.send(Ok(world));
                worker_stopped.store(true, Ordering::Release);
            });
            let mut preparation = GameplayPreparation { receiver, pending: None,
                cancel: Arc::new(AtomicBool::new(false)), worker: Some(worker) };
            if receive_before_drop {
                preparation.pending = Some(preparation.receiver.recv().unwrap().unwrap());
            }
            drop(preparation);
            assert!(stopped.load(Ordering::Acquire));
            assert!(released.load(Ordering::Acquire));
        }
    }

}

#[cfg(test)]
mod save_shutdown_preparation_tests {
    use super::*;
    use country_core::systems::save::{save_game::AutoSaveState,
        save_thread::{AutosaveErrorState, SaveThreadHandle}};
    use std::sync::{atomic::{AtomicBool, Ordering}, mpsc};

    #[derive(bevy_ecs::resource::Resource)]
    struct SaveStoppedBeforeWorldDrop {
        stopped: Arc<AtomicBool>,
        observed: Arc<AtomicBool>,
    }
    impl Drop for SaveStoppedBeforeWorldDrop {
        fn drop(&mut self) {
            self.observed.store(self.stopped.load(Ordering::Acquire), Ordering::Release);
        }
    }

    fn attach_save_owner(world: &mut World) -> (Arc<AtomicBool>, Arc<AtomicBool>) {
        let stopped = Arc::new(AtomicBool::new(false));
        let observed = Arc::new(AtomicBool::new(false));
        let worker_stopped = stopped.clone();
        let (command_sender, receiver) = crossbeam_channel::unbounded();
        let (error_sender, error_state) = AutosaveErrorState::channel();
        let join = std::thread::spawn(move || {
            assert!(matches!(receiver.recv_timeout(std::time::Duration::from_secs(2)),
                Err(crossbeam_channel::RecvTimeoutError::Disconnected)));
            drop(error_sender);
            worker_stopped.store(true, Ordering::Release);
        });
        let mut auto = AutoSaveState::default();
        auto.attach_command_sender(command_sender.clone());
        world.insert_resource(auto);
        world.insert_resource(SaveThreadHandle { join, command_sender, error_state });
        world.insert_resource(SaveStoppedBeforeWorldDrop {
            stopped: stopped.clone(), observed: observed.clone(),
        });
        (stopped, observed)
    }

    fn assert_saved_before_drop(flags: (Arc<AtomicBool>, Arc<AtomicBool>)) {
        assert!(flags.0.load(Ordering::Acquire), "save worker must terminate");
        assert!(flags.1.load(Ordering::Acquire), "World must drop only after save join");
    }

    #[test]
    fn shader_initializer_failure_prevents_update_warmup_and_publication() {
        use std::sync::atomic::AtomicUsize;
        let updates = Arc::new(AtomicUsize::new(0));
        let warmups = Arc::new(AtomicUsize::new(0));
        let mut app = App::new();
        app.init_resource::<country_core::systems::mode_manager::AppMode>();
        app.init_resource::<country_core::systems::save::load_game::LoadGameManager>();
        let observed = updates.clone();
        app.add_systems(bevy_app::Update, move || { observed.fetch_add(1, Ordering::Relaxed); });
        let flags = attach_save_owner(app.world_mut());
        let warmed = warmups.clone();
        let mut preparation = GameplayPreparation::spawn_with_initializer(
            std::mem::take(app.main_mut()), |_, _| Err(std::io::Error::from(std::io::ErrorKind::NotFound).into()),
            move |_| { warmed.fetch_add(1, Ordering::Relaxed); Ok(()) }).unwrap();
        preparation.join_worker().unwrap();
        let error = preparation.poll_completion().unwrap_err();
        assert_eq!(error.downcast_ref::<std::io::Error>().unwrap().kind(), std::io::ErrorKind::NotFound);
        assert_eq!(updates.load(Ordering::Relaxed), 0);
        assert_eq!(warmups.load(Ordering::Relaxed), 0);
        assert!(preparation.pending.is_none());
        assert_saved_before_drop(flags);
        preparation.cancel_and_join().unwrap();
    }

    #[test]
    fn shader_initializer_runs_once_before_first_update_without_fabricating_ready() {
        use std::sync::atomic::AtomicUsize;
        let initialized = Arc::new(AtomicUsize::new(0));
        let updates = Arc::new(AtomicUsize::new(0));
        let mut app = App::new();
        app.init_resource::<country_core::systems::mode_manager::AppMode>();
        app.init_resource::<country_core::systems::save::load_game::LoadGameManager>();
        let init_observed = initialized.clone();
        let update_observed = updates.clone();
        app.add_systems(bevy_app::Update, move || {
            assert_eq!(init_observed.load(Ordering::Acquire), 1);
            update_observed.fetch_add(1, Ordering::Relaxed);
        });
        let flags = attach_save_owner(app.world_mut());
        let initialize = initialized.clone();
        let mut preparation = GameplayPreparation::spawn_with_initializer(
            std::mem::take(app.main_mut()), move |_, _| { initialize.fetch_add(1, Ordering::Release); Ok(()) },
            |_| panic!("the genuine missing catalog must prevent warmup")).unwrap();
        preparation.join_worker().unwrap();
        assert!(preparation.poll_completion().unwrap_err().to_string().contains("ClutterMeshes"));
        assert_eq!(initialized.load(Ordering::Acquire), 1);
        assert_eq!(updates.load(Ordering::Relaxed), 1);
        assert!(preparation.pending.is_none());
        assert_saved_before_drop(flags);
        preparation.cancel_and_join().unwrap();
    }

    #[test]
    fn cancellation_during_initializer_prevents_update_and_warmup_and_joins_save() {
        let mut app = App::new();
        app.init_resource::<country_core::systems::mode_manager::AppMode>();
        app.init_resource::<country_core::systems::save::load_game::LoadGameManager>();
        app.add_systems(bevy_app::Update, || panic!("cancelled initializer must not update"));
        let flags = attach_save_owner(app.world_mut());
        let (started_tx, started_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let mut preparation = GameplayPreparation::spawn_with_initializer(
            std::mem::take(app.main_mut()), move |_, cancelled| {
                started_tx.send(()).unwrap();
                release_rx.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
                assert!(cancelled.load(Ordering::Acquire));
                Ok(())
            }, |_| panic!("cancelled initializer must not warm up")).unwrap();
        started_rx.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        preparation.cancel.store(true, Ordering::Release);
        release_tx.send(()).unwrap();
        preparation.cancel_and_join().unwrap();
        assert!(preparation.pending.is_none());
        assert_saved_before_drop(flags);
    }

    #[test]
    fn completion_poll_surfaces_startup_failure_without_releasing_loading_logo() {
        use country_core::resources::logo_animation::LogoAnimation;
        use country_core::systems::save::load_game::StartupRevealStarted;

        let mut loading = App::new();
        let mut logo = LogoAnimation::default();
        logo.progress[3] = 0.25;
        loading.insert_resource(logo);
        loading.add_event::<StartupRevealStarted>();
        let loading_id = loading.world().id();

        // A genuinely incomplete gameplay App must fail its production entry
        // contract; no catalog/device/ready resource is fabricated for this test.
        let mut gameplay = bevy_app::SubApp::new();
        let flags = attach_save_owner(gameplay.world_mut());
        let mut preparation = GameplayPreparation::spawn(gameplay, |_| {
            panic!("missing AppMode must reject startup before warmup")
        }).unwrap();
        preparation.join_worker().unwrap();
        let error = preparation.poll_completion().unwrap_err();
        assert!(error.to_string().contains("original gameplay AppMode/startup producers"));
        assert!(preparation.pending.is_none());
        assert_eq!(loading.world().id(), loading_id);
        assert_eq!(*loading.world().resource::<LogoAnimation>(), logo);
        assert!(loading.world().resource::<Events<StartupRevealStarted>>().is_empty());
        assert_saved_before_drop(flags);
        preparation.cancel_and_join().unwrap();
    }

    #[test]
    fn completion_poll_keeps_the_original_typed_error_and_context() {
        // Exercise the same completion transport with an OS error whose typed
        // source must survive; a new summary error would lose this diagnostic.
        let error = anyhow::Error::from(std::io::Error::from_raw_os_error(2))
            .context("required gameplay asset read");
        let (sender, receiver) = mpsc::channel();
        sender.send(Err(error)).unwrap();
        let mut preparation = GameplayPreparation { receiver, pending: None,
            cancel: Arc::new(AtomicBool::new(false)), worker: None };
        let error = preparation.poll_completion().unwrap_err();
        assert_eq!(error.to_string(), "required gameplay asset read");
        assert_eq!(error.downcast_ref::<std::io::Error>().unwrap().raw_os_error(), Some(2));
        assert!(preparation.pending.is_none());
        preparation.cancel_and_join().unwrap();
    }

    #[test]
    fn completion_poll_parks_the_same_app_until_cancellation_joins_its_save_owner() {
        use country_core::resources::logo_animation::LogoAnimation;
        use country_core::systems::save::load_game::StartupRevealStarted;

        let mut loading = App::new();
        let mut logo = LogoAnimation::default();
        logo.progress[3] = 0.5;
        loading.insert_resource(logo);
        loading.add_event::<StartupRevealStarted>();
        let loading_id = loading.world().id();
        let mut gameplay = bevy_app::SubApp::new();
        let gameplay_id = gameplay.world().id();
        let flags = attach_save_owner(gameplay.world_mut());
        let (sender, receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || { sender.send(Ok(gameplay)).unwrap(); });
        let mut preparation = GameplayPreparation { receiver, pending: None,
            cancel: Arc::new(AtomicBool::new(false)), worker: Some(worker) };
        preparation.join_worker().unwrap();

        // The background wait can poll repeatedly after the producer closes
        // its channel. Reception alone must neither hand off nor dispose it.
        for _ in 0..2 {
            preparation.poll_completion().unwrap();
            let pending = preparation.pending.as_ref().unwrap();
            assert_eq!(pending.world().id(), gameplay_id);
            assert!(pending.world().contains_resource::<SaveThreadHandle>());
            assert!(!flags.0.load(Ordering::Acquire));
            assert!(!flags.1.load(Ordering::Acquire));
            assert_eq!(loading.world().id(), loading_id);
            assert_eq!(*loading.world().resource::<LogoAnimation>(), logo);
            assert!(loading.world().resource::<Events<StartupRevealStarted>>().is_empty());
        }
        preparation.cancel_and_join().unwrap();
        assert!(preparation.pending.is_none());
        assert_saved_before_drop(flags);
        preparation.cancel_and_join().unwrap();
    }

    #[test]
    fn pending_and_queued_apps_join_save_workers_before_world_drop() {
        for receive_first in [false, true] {
            let mut app = bevy_app::SubApp::new();
            let flags = attach_save_owner(app.world_mut());
            let (sender, receiver) = mpsc::channel();
            let worker = std::thread::spawn(move || { sender.send(Ok(app)).unwrap(); });
            let mut preparation = GameplayPreparation { receiver, pending: None,
                cancel: Arc::new(AtomicBool::new(false)), worker: Some(worker) };
            if receive_first {
                preparation.pending = Some(preparation.receiver.recv().unwrap().unwrap());
            }
            preparation.cancel_and_join().unwrap();
            assert_saved_before_drop(flags);
        }
    }

    #[test]
    fn producer_validation_error_stops_save_before_publishing_failure() {
        let mut app = bevy_app::SubApp::new();
        let flags = attach_save_owner(app.world_mut());
        let mut preparation = GameplayPreparation::spawn(app, |_| Ok(())).unwrap();
        let failure = preparation.receiver.recv_timeout(std::time::Duration::from_secs(2))
            .unwrap().unwrap_err();
        assert!(failure.to_string().contains("AppMode"));
        assert_saved_before_drop(flags);
        preparation.cancel_and_join().unwrap();
    }

    #[test]
    fn worker_update_panic_missing_catalog_and_cancellation_all_stop_save() {
        // This small CPU fixture has no genuine gameplay catalog. It can
        // prove rejection/cleanup, not entry into the real warm-up graph.
        for phase in ["update panic", "missing catalog", "cancel"] {
            let mut app = App::new();
            app.insert_resource(country_core::systems::mode_manager::AppMode::default());
            app.init_resource::<country_core::systems::save::load_game::LoadGameManager>();
            app.add_event::<country_core::systems::save::load_game::LoadingGameComplete>();
            app.add_systems(bevy_app::Update, move |world: &mut World| {
                assert_ne!(phase, "update panic", "injected update panic");
                world.send_event(country_core::systems::save::load_game::LoadingGameComplete::default());
            });
            let flags = attach_save_owner(app.world_mut());
            let gameplay = std::mem::take(app.main_mut());
            let mut preparation = GameplayPreparation::spawn(gameplay, move |_| {
                panic!("missing gameplay catalog must prevent warmup: {phase}");
            }).unwrap();
            if phase == "cancel" {
                // Immediate cancellation races with the observable catalog
                // failure; both must finish the same actual save owner.
                if let Err(error) = preparation.cancel_and_join() {
                    assert!(error.to_string().contains("ClutterMeshes"));
                }
            } else {
                let error = preparation.receiver.recv_timeout(std::time::Duration::from_secs(2))
                    .unwrap().unwrap_err();
                let expected = if phase == "missing catalog" { "ClutterMeshes" } else { phase };
                assert!(error.to_string().contains(expected));
                preparation.cancel_and_join().unwrap();
            }
            assert_saved_before_drop(flags);
        }
    }

    #[test]
    fn unstarted_capture_and_failed_send_both_dispose_the_actual_app() {
        for failed_send in [false, true] {
            let mut app = bevy_app::SubApp::new();
            let flags = attach_save_owner(app.world_mut());
            if failed_send {
                let (sender, receiver) = mpsc::channel();
                drop(receiver);
                assert!(publish_prepared_app(&sender, app).is_err());
            } else {
                // Same closure-capture drop as Builder::spawn returning Err;
                // do not try to exhaust process threads to test std itself.
                let owner = GameplayAppOwner::new(app);
                let unstarted = move || drop(owner);
                drop(unstarted);
            }
            assert_saved_before_drop(flags);
        }
    }

    #[test]
    fn failed_handoff_disposes_pending_save_owner_before_returning_error() {
        let mut app = bevy_app::SubApp::new();
        let flags = attach_save_owner(app.world_mut());
        let (_sender, receiver) = mpsc::channel();
        let mut preparation = GameplayPreparation { receiver, pending: Some(app),
            cancel: Arc::new(AtomicBool::new(false)), worker: None };
        let mut loading = App::new();
        let mut logo = country_core::resources::logo_animation::LogoAnimation::default();
        logo.progress[4] = 1.0;
        loading.insert_resource(logo);
        assert!(preparation.poll(&mut loading).is_err());
        assert_saved_before_drop(flags);
        preparation.cancel_and_join().unwrap();
    }

    #[test]
    fn missing_logo_refuses_handoff_and_stops_received_save_owner() {
        for receive_first in [false, true] {
            let mut app = bevy_app::SubApp::new();
            let flags = attach_save_owner(app.world_mut());
            let (sender, receiver) = mpsc::channel();
            sender.send(Ok(app)).unwrap();
            let mut preparation = GameplayPreparation { receiver, pending: None,
                cancel: Arc::new(AtomicBool::new(false)), worker: None };
            if receive_first {
                preparation.pending = Some(preparation.receiver.recv().unwrap().unwrap());
            }
            let mut loading = App::new();
            let mut config = GameConfig::default();
            config.lang_override = Some("loading-owner-retained".to_owned());
            loading.insert_resource(config);

            let error = preparation.poll(&mut loading).unwrap_err();
            assert!(error.to_string().contains("mandatory LogoAnimation"));
            assert!(preparation.pending.is_none());
            assert_eq!(loading.world().resource::<GameConfig>().lang_override.as_deref(),
                Some("loading-owner-retained"));
            assert_saved_before_drop(flags);
            preparation.cancel_and_join().unwrap();
        }
    }
}

#[cfg(test)]
mod member14_owner_stage_tests {
    use super::GameSystemSet;
    use bevy_ecs::prelude::*;
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{
        asset_libraries::{runtime::AssetLibraryRuntime, ssbo_library::AssetSsboLibrary},
        geometry::{instanced_wall::{BrickTransformSsbo, InstancedWall}, runtime::{prepare_wall_brick_assembly_owners, sync_wall_bricks, WallBrickUploadStatus}},
        render::{sparse_instance_buffer::SparseInstanceBuffer},
        resources::{walls::{InnerWalls, WallId, WallState, public_walls::{PublicWalls, WallEntities}, publication_source::TerrainHeightGrid512, wall_trigger_volume::WallTriggerVolume}, rt::UpdateRtWorldEvent},
        systems::wall::construct_wall_bricks::WallBricksChanged,
    };
    use std::sync::Arc;

    /// CPU-only range-checking boundary for these CPU ownership/state tests.
    /// It creates no physical resource, maps no memory, and submits no GPU work.
    /// Successful updates mean only that the canonical uploader requested valid
    /// nonoverlapping byte ranges inside the declared CPU fixture capacity.
    #[derive(Debug)]
    struct CpuBufferDevice(u64);

    impl country_core::render::ssbo::rhapsody_boundary::GraphicsDevice for CpuBufferDevice {
        fn create_buffer(
            &self,
            desc: &country_core::render::ssbo::rhapsody_boundary::BufferDesc,
            _name: &str,
            data: Option<&[u8]>,
        ) -> Result<country_core::render::ssbo::rhapsody_boundary::Buffer, country_core::render::ssbo::rhapsody_boundary::BackendError> {
            assert!(data.is_none_or(|bytes| bytes.len() <= desc.size));
            Ok(country_core::render::ssbo::rhapsody_boundary::Buffer {
                id: self.0,
                byte_size: desc.size as u64,
                ..Default::default()
            })
        }

        fn update_buffer_bytes(
            &self,
            buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer,
            data: &[u8],
            ranges: &[core::ops::Range<usize>],
            offset: usize,
        ) -> Result<(), country_core::render::ssbo::rhapsody_boundary::BackendError> {
            let mut previous_end = 0;
            for range in ranges {
                assert!(previous_end <= range.start);
                assert!(range.start <= range.end && range.end <= data.len());
                let end = offset.checked_add(range.end).expect("CPU fixture byte offset overflow");
                assert!((end as u64) <= buffer.byte_size);
                previous_end = range.end;
            }
            Ok(())
        }
    }

    // AssetLibraryRuntime requires the texture capability even though this
    // fixture mounts only the real wall sparse-buffer sync system. Any accidental
    // texture/descriptor use fails explicitly instead of reporting fake success.
    impl country_core::render::load_texture::boundary::TextureDevice for CpuBufferDevice {
        fn create_texture(
            &self,
            _name: &str,
            _desc: &country_core::render::load_texture::boundary::TextureDesc,
            _data: &[u8],
        ) -> Result<country_core::render::load_texture::boundary::Texture, country_core::render::load_texture::boundary::BackendError> {
            panic!("texture creation is outside this CPU wall-owner fixture")
        }
    }
    impl country_core::asset_libraries::texture_library::TextureLibraryDevice for CpuBufferDevice {
        fn create_persistent_descriptor_set(
            &self,
            _name: &str,
        ) -> Result<country_core::rhapsody_contract::DescriptorSet, country_core::render::load_texture::boundary::BackendError> {
            panic!("descriptor creation is outside this CPU wall-owner fixture")
        }
        fn update_persistent_descriptor_set(
            &self,
            _set: &country_core::rhapsody_contract::DescriptorSet,
            _updates: &[country_core::asset_libraries::texture_library::DescriptorImageUpdate],
        ) -> Result<(), country_core::render::load_texture::boundary::BackendError> {
            panic!("descriptor updates are outside this CPU wall-owner fixture")
        }
        fn get_vk_image_view(
            &self,
            _texture: &country_core::render::load_texture::boundary::Texture,
            _mip: u32,
        ) -> Result<country_core::asset_libraries::texture_library::ImageView, country_core::render::load_texture::boundary::BackendError> {
            panic!("image views are outside this CPU wall-owner fixture")
        }
        fn update_texture(
            &self,
            _texture: &country_core::render::load_texture::boundary::Texture,
            _update: &country_core::asset_libraries::texture_library::TextureUpdate,
        ) -> Result<(), country_core::render::load_texture::boundary::BackendError> {
            panic!("texture uploads are outside this CPU wall-owner fixture")
        }
    }


    #[derive(Resource, Default)]
    struct Observed { changed_counts: Vec<usize> }

    fn inspect_at25(
        public: Res<PublicWalls>, library: Res<AssetSsboLibrary>, entities: Res<WallEntities>,
        owners: Query<(&InstancedWall<BrickTransformSsbo>, Option<&WallTriggerVolume>)>,
        changed: Query<Entity, Changed<InstancedWall<BrickTransformSsbo>>>,
        successes: Res<Events<WallBricksChanged>>, rt: Res<Events<UpdateRtWorldEvent>>,
        mut observed: ResMut<Observed>,
    ) {
        let wall = WallId(0x700000031);
        assert!(public.get(wall).is_some());
        let (owner, trigger) = owners.get(entities.wall_entity(wall).unwrap()).unwrap();
        assert_eq!(owner.id, wall.0);
        assert!(owner.is_awaiting_production()); assert!(!owner.is_upload_pending());
        assert!(trigger.is_none()); assert!(successes.is_empty()); assert!(rt.is_empty());
        let storage = library.get::<BrickTransformSsbo>().unwrap();
        let lanes = [owner.allocation().unwrap(), owner.chimney_allocation().unwrap(), owner.door_stairs_allocation().unwrap()];
        for lane in lanes { assert!(storage.owns_allocation(lane)); }
        for i in 0..3 { for j in i+1..3 { assert!(!core::ptr::eq(lanes[i].handle.0.as_deref().unwrap(), lanes[j].handle.0.as_deref().unwrap())); } }
        observed.changed_counts.push(changed.iter().count());
    }

    #[test]
    fn actual_set_type_and_ordinary_edges_expose_preassembly_commands_without_false_base_success() {
        let mut app = bevy_app::App::new();
        // Same actual GameSystemSet and exact pairwise configuration as run_game_thread.
        for stage in 0..44 { app.configure_sets(bevy_app::Update, GameSystemSet(stage).before(GameSystemSet(stage+1))); }
        let mut inner = InnerWalls::new();
        let curve = country_core::resources::roofs::geometry::surface::Curve2::try_new(vec![[0.,0.],[4.,0.]], false).unwrap();
        inner.new_wall_with_id(WallId(0x700000031), WallState::new(curve, 2., 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume(&mut inner, &TerrainHeightGrid512 { encoded: vec![0u16;512*512].into(), minimum_y: 0., maximum_y: 4., sampling_fix: true });
        let storage = SparseInstanceBuffer::<BrickTransformSsbo>::try_new(&CpuBufferDevice(1)).unwrap();
        let mut library = AssetSsboLibrary::new(); library.register::<BrickTransformSsbo>(storage);
        app.insert_resource(public); app.insert_resource(library); app.init_resource::<WallEntities>();
        app.insert_resource(AssetLibraryRuntime::new(Arc::new(CpuBufferDevice(1)), Arc::new(CpuBufferDevice(2))));
        app.init_resource::<WallBrickUploadStatus>(); app.init_resource::<Observed>();
        app.add_event::<WallBricksChanged>(); app.add_event::<UpdateRtWorldEvent>();
        app.add_systems(bevy_app::Update, prepare_wall_brick_assembly_owners.in_set(GameSystemSet(19)));
        app.add_systems(bevy_app::Update, inspect_at25.in_set(GameSystemSet(25)));
        app.add_systems(bevy_app::PostUpdate, sync_wall_bricks);
        app.update(); app.update();
        assert_eq!(app.world().resource::<Observed>().changed_counts, [1,0]);
        assert!(app.world().resource::<WallBrickUploadStatus>().gpu_ready);
        let entity = app.world().resource::<WallEntities>().wall_entity(WallId(0x700000031)).unwrap();
        assert!(app.world().get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap().is_awaiting_production());
    }
}
