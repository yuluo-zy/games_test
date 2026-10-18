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

/// Rust marker for the original 45 GameSystemSet discriminants. Their source
/// names are unavailable; original add_stages iterates 0..44 and chains them.
/// Retaining numeric identity avoids inventing stage names or engine internals.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GameSystemSet(pub u8);

/// Each original App constructs a separate shader library. The Arc is a Rust
/// owner for that App's library; it does not merge the two native libraries.
#[derive(Resource, Clone)]
pub struct ApplicationShaderLibrary(
    pub Arc<country_core::asset_libraries::shader_library::AssetShaderLibrary>,
);

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
            client: Some(client),
        });
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
    app.init_resource::<EscMenuState>();
    app.init_resource::<NextEscMenuState>();
    app.init_resource::<PrevEscMenuState>();
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
        (
            country_core::resources::mouse_capture::mouse_capture,
            country_core::resources::shutdown::initiate_game_shutdown,
        ),
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
    let mut gui = Schedule::new(bevy_app::Update);
    configure_gui_consumers(app.world_mut(), &mut gui);
    // Reuse its real systems without replacing the existing Update schedule.
    gui.add_systems(country_core::resources::gui::advance_menu_state_system);
    let gui_system = move |world: &mut World| gui.run(world);
    // add_backend_systems @0x140942c80 -> in_set @0x140aad7d0 ->
    // SubApp::add_systems<Update> @0x140b67280 proves Update. The time→GUI
    // edge below is the explicit local adapter; the full original set graph
    // (including its second GameTime consumer) is not claimed complete.
    app.add_systems(
        bevy_app::Update,
        (
            country_core::systems::time_update::time_update.in_set(GameSystemSet(2)),
            gui_system,
        )
            .chain(),
    );
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
    // Original full-game registrations: weather in set36 and the complete
    // fourteen-resource frame writer in set44. ActiveSession comes from the
    // real OnEnter(Game) producer above; loading App has a different schedule.
    app.add_systems(
        bevy_app::Update,
        country_core::systems::weather_system::weather_system.in_set(GameSystemSet(36)),
    );
    app.add_systems(
        bevy_app::Update,
        country_core::systems::time_of_day::time_of_day_audio_disabled
            .in_set(GameSystemSet(36))
            .run_if(is_gameplay_active),
    );
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
    app.init_resource::<history::replay_mode::ReplayFastHoldInput>();
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
            // Original helper140335b30 ends with the changed/deleted event
            // readers and the same PublicWalls/WallSpatialHash owners.
            spatial_hash_update,
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
        )
            .chain_ignore_deferred()
            .in_set(GameSystemSet(35)),
    );
    app.add_systems(
        bevy_app::Update,
        history::replay_wall::replay_wall.in_set(GameSystemSet(40)),
    );
    app.add_systems(
        bevy_app::Update,
        (
            history::replay_pop_bucket_system,
            history::finish_replay::finish_replay,
        )
            .chain_ignore_deferred()
            .in_set(GameSystemSet(41)),
    );
}

/// Original country_core::systems::is_gameplay_active @140a90e00,22B.
/// Kept local to this source host until the shared core systems export is
/// consolidated; the condition's two resource identities are canonical.
fn is_gameplay_active(
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
    to.send_event(country_core::resources::events::WriteSettingsFileCmd);
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
        let parsed = country_core::systems::save::load_game::parse_saved_game_data(&path)
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
