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

/// Rust marker for the original 45 GameSystemSet discriminants. Their source
/// names are unavailable; original add_stages iterates 0..44 and chains them.
/// Retaining numeric identity avoids inventing stage names or engine internals.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GameSystemSet(pub u8);

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
    let event_loop = winit::event_loop::EventLoop::<()>::new()?;
    let (window, _exclusive_mode) =
        crate::setup::setup_window(&event_loop, &mut config.video.display);
    let window = Arc::new(window);
    let physical = window.inner_size();
    let mut app = run_game_thread(
        config,
        &cli,
        WindowSize::new(physical.width, physical.height),
    )?;
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
    crate::event_loop::run_event_loop(event_loop, window, app, options, shaders)
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
    let mut app = App::new();
    startup_event_runtime::install(&mut app);
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
    
    #[cfg(target_os = "windows")]
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
    app.add_systems(bevy_state::state::OnExit(country_core::app_loading_state::AppLoadingState::Game),crate::app_systems::walls_on_glade_unload);
    // Native140b3f550 outer chain: begin precedes the inner unordered pair.
    // Keep the independently registered full host wall-unload system above.
    app.add_systems(
        bevy_state::state::OnExit(country_core::app_loading_state::AppLoadingState::Game),
        (
            country_core::startup::clearing::begin_glade_unload,
            (
                country_core::startup::clearing::uninit_clearing::<country_core::resources::native_audio::NativeStudio>,
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
    app.add_plugins((InputPlugin, bevy_time::TimePlugin));
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
    // R10G1: 预置树系统（B save::add_save_load_systems 注册的同一 owner）在
    // OnEnter(Game) 写入该表；宿主在本文件直接注册 save 批次，因此补上资源。
    app.init_resource::<country_core::resources::tree_storage::TreeStorage>();
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
    toolbar::configure_app(&mut app);
    crate::decorator_plugin::build_startup_resources(&mut app);
    crate::wall_constructor_plugin::build(&mut app);
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
        toolbar::photomode::photomode_camera_buttons,
        country_core::systems::backend::photomode::finish_photomode_frame,
    ).chain_ignore_deferred().in_set(GameSystemSet(8)).run_if(
        bevy_state::condition::in_state(country_core::app_loading_state::AppLoadingState::Game)));
    app.init_resource::<EscMenuState>();
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
    let mut gui = Schedule::new(bevy_app::Update);
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
        (
            country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees,
            system_tree::convert_preplaced_trees::convert_preplaced_trees::<
                country_core::resources::history::MaybePendingSnapshotId,
            >,
        )
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
    #[cfg(target_os = "windows")]
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
    Ok(app)
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
            terrain_height::{TerrainHeightSsbo, TerrainHeightsData, VisualTerrainHeightSsbo},
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
    // Safe projection of the unengaged original ActionSetStates slots16/17.
    // ActionSetStates aliases MainActionSet; this is the same input owner.
    // Native startup1405a146f supplies tag2; this is a real inactive state,
    // not a substitute for the active lifecycle handled below.
    app.insert_resource(country_core::systems::timelapse_replay::TimelapseState::Inactive);
    app.init_resource::<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>();
    app.add_event::<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>();
    app.add_event::<country_core::systems::timelapse_replay::InterruptTimelapse>();
    app.add_event::<country_core::systems::timelapse_replay::TimelapseReplayFinished>();
    app.add_event::<country_core::resources::history::tree_edit::SpawnPreplacedTrees>();
    app.add_systems(bevy_app::PostUpdate, country_core::systems::timelapse_replay::cleanup_timelapse);
    // insert_terrain_resources @1409457c0 creates both actual CPU readback
    // owners: 0x80000 zero bytes with the original -2.5/24 height bounds.
    let heights = TerrainHeightsData::<TerrainHeightSsbo>::new();
    let view = WallPublicationTerrain::from_height_data(&heights);
    app.insert_resource(heights);
    app.insert_resource(TerrainHeightsData::<VisualTerrainHeightSsbo>::new());
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
    // Recovered body of PreStartup chain @140941125/1130. The full10-item
    // original chain is not yet assembled here; this follows the already
    // registered real profile producer and flushes Commands before Update19.
    app.add_systems(
        bevy_app::PreStartup,
        once_startup_walls.after(country_core::game_profile::load_game_profile),
    );
    // add_wall_events @1404939a6/3b99 and original new empty publishers.
    app.init_resource::<PublicWalls>();
    app.init_resource::<WallSpatialHash>();
    app.init_resource::<WallPublicationDirty>();
    app.init_resource::<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>();
    // DecoratorPlugin @140343120/313d/3377/3a3c; world plugin @140b685e0.
    app.init_resource::<DecoratorStorage>();
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
    // add_wall_systems @140493197/3246: adjust precedes publication inside
    // the set19 chain. Replay set40 therefore becomes visible next frame.
    app.add_systems(
        bevy_app::Update,
        (
            synchronize_wall_publication_terrain,
            adjust_wall_height,
            collect_wall_publication_dirty,
            publish_public_wall_state,
            country_core::systems::wall::delete_wall::delete_wall,
            system_wall_constructor::gpu_output::delete_constructor_output,
            spatial_hash_update,
            country_core::systems::wall::calc_wall_bend::calc_wall_bend,
            system_wall_constructor::gpu_output::clear_constructor_planks,
            system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_wall_path_segmentation,
            system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height,
        )
            .chain_ignore_deferred()
            .in_set(GameSystemSet(19)),
    );
    // add_history_systems @1409438dc/e1 and @140943981 prove sets35/40.
    app.add_systems(
        bevy_app::Update,
        (
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
        history::replay_wall::replay_wall.in_set(GameSystemSet(40)),
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
    // Resource-producing prefix of original VermeerPlugin::build @1408a8280.
    // Its shader PreStartup consumer is provided by the actual renderer; the
    // still-missing AppLoadingState OnExit schedule is not replaced with a no-op.
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
    // Original loading App @1405a4d93/95 supplies false, independently of gameplay.
    app.insert_resource(country_core::asset_libraries::configwatch::ConfigWatch::new(false));
    for stage in 0..44 {
        app.configure_sets(
            bevy_app::Update,
            GameSystemSet(stage).before(GameSystemSet(stage + 1)),
        );
    }
    app.add_plugins((InputPlugin, bevy_time::TimePlugin));
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
    Ok(app)
}

/// The original async loader transfers a SubApp, not an App (which contains a
/// process runner). A failed producer/warmup is returned to the OS owner; it
/// never becomes a synthetic "ready" application.
pub struct GameplayPreparation {
    receiver: std::sync::mpsc::Receiver<anyhow::Result<bevy_app::SubApp>>,
    pending: Option<bevy_app::SubApp>,
    cancel: Arc<std::sync::atomic::AtomicBool>,
}

impl GameplayPreparation {
    pub fn spawn(
        mut gameplay: bevy_app::SubApp,
        warm_up: impl FnOnce(&mut World) -> anyhow::Result<()> + Send + 'static,
    ) -> anyhow::Result<Self> {
        use std::sync::atomic::{AtomicBool, Ordering};
        let (sender, receiver) = std::sync::mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let cancelled = cancel.clone();
        std::thread::Builder::new().name("async load".to_owned()).spawn(move || {
            // Host diagnostic boundary only: preserve the original load/ready
            // gates, but carry a worker panic back to the owning OS thread.
            let mut phase = "validate gameplay resources";
            let mut update_index = 0u64;
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // The current partial startup must fail explicitly instead of
                // spinning forever without its original producer. AppMode and
                // LoadGameManager are actual inputs of load_game_system.
                anyhow::ensure!(gameplay.world().contains_resource::<country_core::systems::mode_manager::AppMode>(),
                    "B async load @0x1400d7440: original gameplay AppMode/startup producers are not registered");
                anyhow::ensure!(gameplay.world().contains_resource::<country_core::systems::save::load_game::LoadGameManager>(),
                    "B async load: original session/snapshot loading producer is not registered");
                loop {
                    if cancelled.load(Ordering::Acquire) { anyhow::bail!("gameplay preparation cancelled"); }
                    // @1400d76e3 and Events<LoadingGameComplete> native TypeId
                    // 4d00324fb817a8a1/5846f8d25a034525, confirmed register_event
                    // @14151ffb0. It reads both live event buffers.
                    phase = "gameplay SubApp::update";
                    gameplay.update();
                    update_index += 1;
                    phase = "read original LoadingGameComplete";
                    let events = gameplay.world().get_resource::<Events<country_core::systems::save::load_game::LoadingGameComplete>>()
                        .ok_or_else(|| anyhow::anyhow!("B async load: LoadingGameComplete event resource missing"))?;
                    if !events.is_empty() { break; }
                }
                phase = "original gameplay graph warm-up";
                warm_up(gameplay.world_mut())?;
                Ok(gameplay)
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
            let _ = sender.send(result);
        })?;
        Ok(Self {
            receiver,
            pending: None,
            cancel,
        })
    }

    /// Original main_loop polls only at the logo's waiting stage and switches
    /// only after the final stage. Early errors are surfaced immediately in
    /// this host, because incomplete producers must not become a hung splash.
    pub fn poll(&mut self, loading: &mut App) -> anyhow::Result<bool> {
        use country_core::resources::logo_animation::LogoAnimation;
        use std::sync::mpsc::TryRecvError;
        if self.pending.is_none() {
            match self.receiver.try_recv() {
                Ok(Ok(app)) => self.pending = Some(app),
                Ok(Err(error)) => return Err(error),
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => anyhow::bail!(
                    "original async gameplay loader stopped before returning its SubApp"
                ),
            }
        }
        let logo = loading.world_mut().get_resource_mut::<LogoAnimation>();
        if let Some(mut logo) = logo {
            if self.pending.is_some() && logo.is_waiting() {
                logo.finish_wait();
            }
            if !logo.is_finished() {
                return Ok(false);
            }
        }
        let Some(mut gameplay) = self.pending.take() else {
            return Ok(false);
        };
        transfer_loading_resources(loading.world_mut(), gameplay.world_mut())?;
        *loading.main_mut() = gameplay;
        Ok(true)
    }
}

impl Drop for GameplayPreparation {
    fn drop(&mut self) {
        self.cancel
            .store(true, std::sync::atomic::Ordering::Release);
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
            cancel: Arc::new(std::sync::atomic::AtomicBool::new(false)) };
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
}
