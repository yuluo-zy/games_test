//! 默认验证加载图→原 presentation resolve→真实 WSI，使用显式加载 World 与时钟。
//! TG_VALIDATE_GAMEPLAY_WSI=1 则验证真实 GameLoop 的双 App 交接、非空游戏帧、
//! 原生 present 成功及 OS 关闭、保存线程和 GPU 退出；两种验收分别记结果。
//! TG_VALIDATE_WINDOW_LIFECYCLE=loading_close/suspended_close 则分别验证真实加载中关闭。
use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc, time::Duration};

// This executable runs the production loop, so measure its production allocator.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[path = "support/performance_workload.rs"]
mod performance_workload;

use anyhow::{Context, Result};
use country_core::{
    asset_libraries::{
        configwatch::ConfigWatch, meshwatch::MeshWatch,
        placement_data_library::AssetPlacementDataLibrary, shader_library::AssetShaderLibrary,
    },
    game_config::GameConfig,
    resources::{logo_animation::LogoAnimation, window::WindowSize},
    systems::render_loop::vermeer::VermeerState,
};
use tiny_glade::{
    game_loop::GameFrameDriver, loading_graph, production_renderer::ProductionRenderer,
    run_game::ApplicationShaderLibrary,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    // Match both production entry points before creating any worker or Vulkan instance.
    unsafe { std::env::set_var("MVK_CONFIG_USE_METAL_ARGUMENT_BUFFERS", "1"); }
    #[cfg(feature = "profile")]
    tiny_glade::frame_counters::start_cpu_profiler();
    eprintln!("WSI_ALLOCATOR {}", if cfg!(feature = "mimalloc") { "mimalloc" } else { "system" });
    if std::env::var_os("TG_FRAME_TIMING").is_some() || std::env::var_os("TG_STARTUP_VISUAL_TRACE").is_some() {
        struct TimingLogger;
        impl log::Log for TimingLogger {
            fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
                metadata.target() == "tiny_glade::frame_timing"
                    || metadata.target() == "tiny_glade::startup_visual"
                    || metadata.level() <= log::Level::Warn
            }
            fn log(&self, record: &log::Record<'_>) {
                if self.enabled(record.metadata()) { eprintln!("{} {}", record.target(), record.args()); }
            }
            fn flush(&self) {}
        }
        static LOGGER: TimingLogger = TimingLogger;
        log::set_logger(&LOGGER)?;
        log::set_max_level(log::LevelFilter::Info);
    }
    if let Some(value) = std::env::var_os("TG_VALIDATE_WINDOW_LIFECYCLE") {
        let mode = match value.to_str() {
            Some("loading_close") => WindowValidationMode::LoadingClose,
            Some("suspended_close") => WindowValidationMode::SuspendedClose,
            Some("window_cycle") => WindowValidationMode::WindowCycle,
            Some("windowed_cycle") => WindowValidationMode::WindowedCycle,
            Some("late_camera") => WindowValidationMode::LateCamera,
            _ => anyhow::bail!("未知窗口生命周期验收模式：{value:?}"),
        };
        return validate_gameplay_present(mode);
    }
    if std::env::var_os("TG_VALIDATE_GAMEPLAY_WSI").is_some_and(|value| value == "1") {
        return validate_gameplay_present(WindowValidationMode::Gameplay);
    }
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        })
        .canonicalize()
        .context("original packaged installation")?;
    std::env::set_current_dir(&installed)?;
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Tiny Glade — original loading WSI validation")
            .with_inner_size(winit::dpi::LogicalSize::new(480.0, 270.0))
            .build(&event_loop)?,
    );
    let shaders = Arc::new(AssetShaderLibrary::new());
    // This is the production device, surface, swapchain and per-App owner.
    // Initialization failure is an error; this executable never skips a GPU.
    let renderer = ProductionRenderer::new(
        window.clone(),
        installed.join("compiled-shaders"),
        shaders.clone(),
        false,
        None,
    )?;
    let mut app = bevy_app::App::new();
    {
        let world = app.world_mut();
        world.insert_resource(ApplicationShaderLibrary(shaders.clone()));
        world.insert_resource(GameConfig::default());
        renderer.bridge().install_asset_libraries(world)?;
        world.insert_resource(ConfigWatch::new(false));
        world.insert_resource(MeshWatch::new(false));
        world.insert_resource(AssetPlacementDataLibrary::new());
        world.insert_resource(country_core::render::frame_constants::FrameConstants::default());
        let size = window.inner_size();
        world.insert_resource(WindowSize::new(size.width, size.height));
        world.insert_resource(bevy_time::Time::<()>::default());
        let mut vermeer = VermeerState::default();
        vermeer.activate(true, true);
        world.insert_resource(vermeer);
        world.insert_resource(LogoAnimation::default());
        // This explicit World fixture has no App scheduler; call the same two-entry
        // producer directly before the public helper. GameLoop uses real Startup.
        country_core::startup::init_populate_minimal_startup_ssbo_library::init_populate_minimal_startup_ssbo_library(
        renderer.bridge().hosts().buffers.as_ref(),
        world.resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>(),
    );
        loading_graph::initialize_loading_resources(
            world,
            renderer.device().clone(),
            renderer.bridge().hosts(),
            &installed,
            false,
        )?;
        country_core::resources::logo_animation::startup_animated_logo(
            world,
            renderer.bridge().shader_backend(),
            &shaders,
        )?;
    }
    // The production frame driver owns App::update, asset maintenance, egui,
    // the RenderHost factory and the recovered outer render call. Keep the
    // bounded loading fixture, and flush its MEI output after asset maintenance.
    app.add_systems(bevy_app::Last, loading_graph::flush_mei_frame);
    let mut driver = GameFrameDriver::new(app, egui::Context::default());
    let mut render = renderer.into_render_thread()?;
    let wake = parking_lot::Mutex::new(event_loop.create_proxy());
    render.set_wake(Arc::new(move || { let _ = wake.lock().send_event(()); }));
    let mut frames = 0;
    let result = Rc::new(RefCell::new(None::<Result<()>>));
    let observed = result.clone();
    event_loop.run(move |event, target| {
        target.set_control_flow(ControlFlow::Poll);
        let step = match event {
            Event::AboutToWait => (|| -> Result<()> {
                driver.app_mut().world_mut().resource_mut::<bevy_time::Time<()>>()
                    .advance_by(Duration::from_secs_f64(1.0 / 60.0));
                driver.frame(Some(egui::RawInput::default()), &mut render, |_| {})?;
                // Reading diagnostic snapshots must preserve the queued frame;
                // the old synchronous control query reclaimed it here.
                assert!(render.in_flight());
                let _ = render.frame_observation();
                assert!(render.in_flight(), "diagnostic query synchronously reclaimed a frame");
                // This three-frame fixture explicitly settles for acceptance.
                driver.settle_render(&mut render)?;
                if let Some(error) = render.call(|renderer| renderer.take_error()) { return Err(error); }
                frames += 1;
                if frames == 3 {
                    let presents = render.native_present_success_count();
                    anyhow::ensure!(presents > 0,
                        "production loading graph produced no native SUCCESS/SUBOPTIMAL WSI present");
                    render.shutdown_renderer()?;
                    eprintln!("PASS: {frames} production GameFrameDriver loading/presentation frames; {presents} native SUCCESS/SUBOPTIMAL WSI presents and shutdown completed");
                    *observed.borrow_mut() = Some(Ok(()));
                    target.exit();
                }
                Ok(())
            })(),
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                Err(anyhow::anyhow!("validation window closed before present verification"))
            }
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                driver.app_mut().world_mut().insert_resource(WindowSize::new(size.width, size.height));
                render.resize([size.width, size.height])
            }
            _ => Ok(()),
        };
        if let Err(error) = step {
            *observed.borrow_mut() = Some(Err(error));
            target.exit();
        }
    })?;
    result
        .borrow_mut()
        .take()
        .context("OS event loop exited without validation result")?
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WindowValidationMode {
    Gameplay,
    LoadingClose,
    SuspendedClose,
    WindowCycle,
    WindowedCycle,
    LateCamera,
}

impl WindowValidationMode {
    fn is_window_cycle(self) -> bool { matches!(self, Self::WindowCycle | Self::WindowedCycle) }
    fn is_loading_close(self) -> bool {
        matches!(self, Self::LoadingClose | Self::SuspendedClose)
    }
}

/// 只在独立生命周期模式启动一次真实 UserEvent，Wait 模式也有超时唤醒。
/// 正常结束会立即停止并 join 此测试线程，不留脱离的定时任务。
struct WindowLifecycleWatchdog {
    stop: Option<std::sync::mpsc::Sender<()>>,
    worker: Option<std::thread::JoinHandle<()>>,
    expired: Arc<std::sync::atomic::AtomicBool>,
}

impl WindowLifecycleWatchdog {
    fn new(proxy: winit::event_loop::EventLoopProxy<()>, timeout: Duration) -> Result<Self> {
        let (stop, stopped) = std::sync::mpsc::channel();
        let expired = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let thread_expired = expired.clone();
        let worker = std::thread::Builder::new()
            .name("window-lifecycle-watchdog".into())
            .spawn(move || {
                if matches!(stopped.recv_timeout(timeout),
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout))
                {
                    thread_expired.store(true, std::sync::atomic::Ordering::Release);
                    let _ = proxy.send_event(());
                }
            })?;
        Ok(Self { stop: Some(stop), worker: Some(worker), expired })
    }
}

impl Drop for WindowLifecycleWatchdog {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(worker) = self.worker.take() {
            if worker.join().is_err() {
                eprintln!("window lifecycle watchdog thread panicked");
            }
        }
    }
}

/// 复用真实双 App、OS 枚举和主循环；关闭模式与完整游戏呈现分别验收。
fn validate_gameplay_present(mode: WindowValidationMode) -> Result<()> {
    use clap::Parser;
    use country_core::resources::{session::ActiveSession, walls::InnerWalls};
    use country_core::systems::save::{save_game::AutoSaveState, save_thread::SaveThreadHandle};
    use tiny_glade::{
        app_config::{get_app_config, CliArgs},
        game_loop::{GameLoop, GameLoopOptions},
        production_renderer::ProductionGraphKind,
        run_game::run_game_thread,
    };

    let mut performance = PerformanceWorkload::from_environment()?;
    #[cfg(feature = "cpu-probe")]
    let mut cpu_probe = CpuProbeValidation::from_environment()?;
    #[cfg(not(feature = "cpu-probe"))]
    anyhow::ensure!(std::env::var_os("TG_CPU_PROBE_OUT").is_none(), "build with --features cpu-probe");
    let fullscreen_capture_path = std::env::var_os("TG_VALIDATE_FULLSCREEN_CAPTURE").map(PathBuf::from);
    let fullscreen_capture_result = Arc::new(std::sync::Mutex::new(None::<std::result::Result<[u32; 2], String>>));
    let mut fullscreen_capture_queued = false;
    let mut fullscreen_capture_requested = false;
    let mut fullscreen_capture_native_since = None;
    let benchmark_extent = performance.as_ref().map_or([480, 270], |_| [1280, 720]);
    let required_world_frames = std::env::var("TG_VALIDATE_WORLD_FRAMES")
        .ok().map(|value| value.parse::<u64>()).transpose()?.unwrap_or(12).max(12);
    let minimum_world_duration = Duration::from_secs(std::env::var("TG_VALIDATE_WORLD_SECONDS")
        .ok().map(|value| value.parse::<u64>()).transpose()?.unwrap_or(0));
    let timeout_seconds = std::env::var("TG_VALIDATE_TIMEOUT_SECONDS")
        .ok().map(|value| value.parse::<u64>()).transpose()?.unwrap_or(180);

    let installed = PathBuf::from(
        std::env::var_os("TG_INSTALL_ROOT").context("必须提供原版安装树 TG_INSTALL_ROOT")?,
    ).canonicalize()?;
    // 验收用独立账户，保存线程仍走真实启动与关闭路径。
    let account = tempfile::tempdir()?;
    paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;
    std::env::set_current_dir(&installed)?;
    utils::local_time::init_local_time();
    tiny_glade::load_language_library::load_language_library(None, None);
    let cli = if mode == WindowValidationMode::WindowedCycle {
        CliArgs::parse_from(["original-gameplay-wsi", "--windowed"])
    } else { CliArgs::parse_from(["original-gameplay-wsi"]) };
    let mut config = get_app_config(&cli)?;
    let launch_display = if mode == WindowValidationMode::WindowedCycle {
        let policy = tiny_glade::app_config::LaunchDisplayOverride::resolve(
            &cli, &config.video.display, true, false, None)?.context("windowed policy missing")?;
        policy.apply(&mut config.video.display);
        Some(policy)
    } else { None };
    if fullscreen_capture_path.is_some() {
        anyhow::ensure!(mode == WindowValidationMode::Gameplay && performance.is_none(),
            "fullscreen capture requires the gameplay fixture without performance stimuli");
        config.video.display.fullscreen = false;
        config.video.display.allow_exclusive_fullscreen = false;
    }
    if performance.is_some() {
        config.video.display.fullscreen = false;
        config.video.display.allow_exclusive_fullscreen = false;
        config.video.display.windowed_resolution = benchmark_extent;
        config.video.fps_limit = 0.0;
        config.video.vsync_enable = false;
    }
    if mode.is_window_cycle() || mode == WindowValidationMode::LateCamera {
        // 本模式的真实窗口从同一 windowed 配置启动，之后才验收全屏往返。
        config.video.display.fullscreen = false;
        config.video.display.allow_exclusive_fullscreen = false;
        config.video.display.windowed_resolution = [480, 270];
        // 通过原 run_game_thread 启动生产者初始化 FrameRenderConfig，
        // 不把缺少运行时设置生产者的直接资源改写当作缩放功能。
        config.video.world_resolution_scale = Some(0.75);
        config.video.upscaling_mode = country_core::render::render_config::UpscalingMode::Tgsr;
        config.video.upscaling_preset = country_core::render::render_config::UpscalingPreset::Quality;
    }
    if mode == WindowValidationMode::LateCamera {
        config.video.fps_limit = 0.0;
    }
    anyhow::ensure!(!config.dev.demo && !config.dev.skip_splash,
        "必须验证完整版的真实 loading→gameplay 交接");
    let input_recording = match (std::env::var_os("TG_VALIDATE_RECORD_INPUT"), std::env::var_os("TG_VALIDATE_REPLAY_INPUT")) {
        (Some(path), None) => Some(tiny_glade::app_config::InputRecordingOpts::Record(PathBuf::from(path))),
        (None, Some(path)) => Some(tiny_glade::app_config::InputRecordingOpts::Replay(PathBuf::from(path))),
        (None, None) => None,
        _ => anyhow::bail!("select only one input recording validation mode"),
    };
    let replay_expected = if let Some(tiny_glade::app_config::InputRecordingOpts::Replay(path)) = &input_recording {
        country_core::input::recording::load_recording(path)?.frame_count() as u64
    } else { 0 };
    let replayed = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let mut app = run_game_thread(config, &cli, WindowSize::new(benchmark_extent[0], benchmark_extent[1]))?;
    if let Some(policy) = launch_display { app.insert_resource(policy); }
    if replay_expected != 0 {
        use bevy_ecs::schedule::IntoScheduleConfigs;
        let observed = replayed.clone();
        app.add_systems(bevy_app::Update, (move |actions: bevy_ecs::prelude::Res<country_core::input::action_set::ActionSetStates>,
            time: bevy_ecs::prelude::Res<country_core::resources::time::GameTime>,
            capture: bevy_ecs::prelude::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,
            state: bevy_ecs::prelude::Res<country_core::systems::backend::input::input_record_replay::InputRecordReplayState>| {
            if let Some(frame) = state.current_frame() {
                assert_eq!(*actions, frame.actions, "live input changed replayed actions before consumers");
                assert_eq!(time.delta.to_bits(), frame.time_delta_seconds.to_bits(), "replay clock used live time");
                assert_eq!(capture.0, frame.captured_cursor_position, "live capture changed the recorded anchor");
                observed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }).after(tiny_glade::run_game::GameSystemSet(5)).before(tiny_glade::run_game::GameSystemSet(6)));
    }
    if performance.is_some() {
        app.insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)));
    }
    app.insert_resource(country_core::resources::platform::RuntimePlatform::from_detection(false, false));
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    let event_loop = EventLoop::new()?;
    let window = Arc::new(WindowBuilder::new()
        .with_title("Tiny Glade — original gameplay WSI validation")
        .with_visible(fullscreen_capture_path.is_some() || performance.is_some() || mode.is_window_cycle() || mode == WindowValidationMode::LateCamera)
        .with_inner_size(winit::dpi::PhysicalSize::new(benchmark_extent[0], benchmark_extent[1]))
        .build(&event_loop)?);
    if mode == WindowValidationMode::WindowedCycle {
        anyhow::ensure!(window.fullscreen().is_none(), "windowed validation started fullscreen");
        let size = window.inner_size();
        anyhow::ensure!([size.width, size.height] == [480, 270],
            "windowed validation did not start at the requested small size: {size:?}");
    }
    let mut game = GameLoop::new(app, window.clone(), GameLoopOptions {
        compiled_shader_root: installed.join("compiled-shaders"),
        frame_limit: None,
        input_recording: input_recording.clone(),
    }, shaders)?;
    game.set_event_loop_proxy(event_loop.create_proxy());
    // 本验收直接驱动 main_loop，先执行生产 run() 的真实监视器枚举前置步骤。
    let mut monitors = country_core::resources::window::AvailableMonitors::default();
    GameLoop::extract_available_monitors_info(&mut monitors, &event_loop);
    game.app_mut().world_mut().insert_resource(monitors);
    let result = Rc::new(RefCell::new(None::<Result<()>>));
    let observed = result.clone();
    let started = std::time::Instant::now();
    let _watchdog = Some(WindowLifecycleWatchdog::new(event_loop.create_proxy(),
        Duration::from_secs(if mode.is_loading_close() { 60 } else { timeout_seconds }))?);
    let timed_out = _watchdog.as_ref().unwrap().expired.clone();
    let mut close_completed_before_loop_exiting = false;
    let mut preparation_pending_at_close = false;
    let mut world_frames = 0u64;
    let mut world_started = None;
    let mut loading_frames = 0u64;
    let mut native_before_world = None;
    let mut previous_frame = None;
    let mut close_requested = false;
    let mut cycle = WindowCycleValidation::default();
    cycle.windowed_only = mode == WindowValidationMode::WindowedCycle;
    let mut late_camera = LateCameraValidation::default();
    let mut stress = WindowStress::from_environment()?;
    anyhow::ensure!(stress.kind.is_none() || mode == WindowValidationMode::WindowCycle,
        "window stress requires the window_cycle lifecycle");
    event_loop.run(move |event, target| {
        let redraw = matches!(&event, Event::WindowEvent { event: WindowEvent::RedrawRequested, .. });
        let resized = matches!(&event, Event::WindowEvent { event: WindowEvent::Resized(_), .. });
        let exiting = matches!(&event, Event::LoopExiting);
        let step = (|| -> Result<()> {
            // 本模式的唯一 UserEvent 来自 watchdog。必须记失败，再由真实退出事件清理。
            if matches!(&event, Event::UserEvent(())) && timed_out.load(std::sync::atomic::Ordering::Acquire) {
                anyhow::bail!("窗口生命周期验收超时：mode={mode:?}, loading={loading_frames}, world={world_frames}, close={close_requested}, preparation_pending={}",
                    game.gameplay_preparation_pending());
            }
            let mut native_world_frame = None;
            if mode.is_window_cycle() {
                cycle.observe_os_event(&event, window.id());
            }
            game.main_loop(event, target)?;
            #[cfg(target_os = "macos")]
            if resized { verify_metal_layer_geometry(&window)?; }
            if redraw {
                // Observations are published asynchronously. Polling must not
                // reclaim the next queued frame or count a cached receipt twice.
                let observation = game.frame_observation().cloned();
                if let Some(frame) = observation.as_ref().filter(|frame|
                    previous_frame != Some((frame.world, frame.receipt_frame))) {
                    match &frame.kind {
                        ProductionGraphKind::Loading => {
                            loading_frames += 1;
                            previous_frame = Some((frame.world, frame.receipt_frame));
                        },
                        ProductionGraphKind::Main { world_rendered: true, .. } => {
                            anyhow::ensure!(!game.gameplay_preparation_pending(), "游戏帧早于双 App 交接");
                            let world = game.app().world();
                            anyhow::ensure!(world.contains_resource::<ActiveSession>()
                                && world.resource::<InnerWalls>().wall_ids().count() > 0,
                                "真实游戏帧没有已加载会话及墙");
                            anyhow::ensure!(world_frames != 0 || (world.contains_resource::<SaveThreadHandle>()
                                && world.get_resource::<AutoSaveState>().is_some_and(|state| state.is_command_attached())),
                                "真实游戏 App 没有已启动且连接的保存线程");
                            anyhow::ensure!(frame.world == world.id(), "渲染了另一个 App 的 World");
                            anyhow::ensure!(frame.draw_list_lengths.iter().any(|&count| count > 0),
                                "不能把空世界 present 当作游戏验收");
                            let cycle_native_ready = frame.present_point.is_some()
                                && frame.present_generation.is_some_and(|generation| {
                                    if cycle.previous_generation == Some(generation) {
                                        frame.native_present_successes > cycle.previous_native_successes
                                    } else { frame.native_present_successes > 0 }
                                });
                            if (mode.is_window_cycle() || fullscreen_capture_path.is_some()) && !cycle_native_ready {
                                // 原生acquire允许暂时无图像/旧generation背压；保World/draw门，
                                // 此帧不推进生命周期、receipt或native成功计数。
                            } else {
                            anyhow::ensure!(frame.present_point.is_some() && frame.present_generation.is_some(),
                                "生产世界没有 WSI present ticket");
                            previous_frame = Some((frame.world, frame.receipt_frame));
                            native_before_world.get_or_insert(frame.native_present_successes.saturating_sub(1));
                            world_frames += 1;
                            world_started.get_or_insert_with(std::time::Instant::now);
                            native_world_frame = Some(frame.clone());
                            if performance.is_none() {
                            eprintln!("GAMEPLAY_WSI world_frame={world_frames} receipt={} native_successes={} draws={:?}",
                                frame.receipt_frame, frame.native_present_successes, frame.draw_list_lengths);
                            }
                            }
                        }
                        _ => anyhow::bail!("出现空世界或 diagnostic minimal 图"),
                    }
                }
                let lifecycle_loading_presented = observation.as_ref().is_some_and(|frame| {
                    matches!(&frame.kind, ProductionGraphKind::Loading)
                        && frame.present_point.is_some()
                        && frame.present_generation.is_some()
                        && frame.native_present_successes > 0
                });
                if mode.is_loading_close() && lifecycle_loading_presented && !close_requested {
                    // 确认本次真实 loading 原生 present 成功且待交接 owner 仍在。
                    // pending 不表示 worker 一定仍在执行；已发布的结果同样必须 join/释放。
                    preparation_pending_at_close = game.gameplay_preparation_pending();
                    anyhow::ensure!(preparation_pending_at_close,
                        "生命周期关闭尚未注入，真实 gameplay preparation 已经结束");
                    close_requested = true;
                    eprintln!("WINDOW_LIFECYCLE mode={mode:?} loading={loading_frames} native_successes={} preparation_pending={preparation_pending_at_close}",
                        game.native_present_success_count());
                    if mode == WindowValidationMode::SuspendedClose {
                        game.main_loop(Event::Suspended, target)?;
                    }
                    game.main_loop(Event::WindowEvent {
                        window_id: window.id(), event: WindowEvent::CloseRequested,
                    }, target)?;
                    if mode == WindowValidationMode::SuspendedClose {
                        // 不发送 Resumed，且不以 LoopExiting 的兜底 shutdown 冒充通过。
                        anyhow::ensure!(target.exiting() && !game.gameplay_preparation_pending(),
                            "暂停关闭返回时尚未完成取消、join 和退出请求");
                        close_completed_before_loop_exiting = true;
                    }
                }
                #[cfg(feature = "cpu-probe")]
                if let Some(probe) = &mut cpu_probe {
                    probe.after_frame(&mut game, world_frames, observation.as_ref())?;
                }
                if let Some(performance) = &mut performance {
                    performance.after_frame(game.app_mut().world_mut(), world_frames, observation.as_ref(), started.elapsed())?;
                }
                if mode.is_window_cycle() && let Some(frame) = native_world_frame {
                    cycle.after_frame(&mut game, &window, target, world_frames, &frame)?;
                    if cycle.complete() {
                        stress.after_frame(&mut game, &window, world_frames, &frame, cycle.os_resize_events)?;
                    }
                }
                if mode == WindowValidationMode::LateCamera {
                    late_camera.after_redraw(&mut game, &window, target, world_frames, observation.as_ref())?;
                }
                if let Some(path) = &fullscreen_capture_path {
                    if !fullscreen_capture_requested && world_frames >= 3 {
                        game.app_mut().world_mut().resource_mut::<GameConfig>().video.display.fullscreen = true;
                        game.app_mut().world_mut().send_event(tiny_glade::app_config::ApplyVideoSettingsCmd(1));
                        fullscreen_capture_requested = true;
                    }
                    if let Some(Err(error)) = &*fullscreen_capture_result.lock().unwrap() {
                        anyhow::bail!("fullscreen main-image capture failed: {error}");
                    }
                    #[cfg(target_os = "macos")]
                    let native_fullscreen = owned_window_is_native_fullscreen(&window)?;
                    #[cfg(not(target_os = "macos"))]
                    let native_fullscreen = window.fullscreen().is_some();
                    let full_width = window.current_monitor().is_some_and(|monitor| window.inner_size().width == monitor.size().width);
                    if native_fullscreen && full_width {
                        fullscreen_capture_native_since.get_or_insert_with(std::time::Instant::now);
                    } else { fullscreen_capture_native_since = None; }
                    if !fullscreen_capture_queued && world_frames >= 8 && fullscreen_capture_native_since.is_some_and(|since| since.elapsed() >= Duration::from_millis(500))
                        && observation.as_ref().is_some_and(|frame| frame.present_point.is_some()
                            && frame.output_extent == [window.inner_size().width, window.inner_size().height]) {
                        use tiny_glade::screenshot::{ScreenshotRequest, ScreenshotPurpose, ScreenshotSink};
                        #[cfg(target_os = "macos")]
                        verify_metal_layer_geometry(&window)?;
                        #[cfg(target_os = "macos")]
                        save_owned_window_image(&window, &path.with_extension("window.png"))?;
                        let result = fullscreen_capture_result.clone();
                        tiny_glade::screenshot::gather_screenshot_requests(game.app_mut().world_mut(), [ScreenshotRequest {
                            purpose: ScreenshotPurpose::FinalFrame,
                            dimensions: None,
                            sink: ScreenshotSink::File { path: path.clone(), jpeg_quality: 95 },
                            completion: Some(Arc::new(move |value| { *result.lock().unwrap() = Some(value); })),
                        }])?;
                        fullscreen_capture_queued = true;
                        eprintln!("FULLSCREEN_CAPTURE main_image={path:?} physical={:?}", window.inner_size());
                    }
                }
                if !mode.is_loading_close() && world_frames >= required_world_frames
                    && world_started.is_some_and(|start| start.elapsed() >= minimum_world_duration)
                    && (!mode.is_window_cycle() || cycle.complete())
                    && (mode != WindowValidationMode::LateCamera || late_camera.complete())
                    && stress.pending.is_none()
                    && (fullscreen_capture_path.is_none() || fullscreen_capture_result.lock().unwrap().is_some())
                    && replayed.load(std::sync::atomic::Ordering::Relaxed) >= replay_expected && !close_requested {
                    // 发送关闭窗口事件，由现有 OS→游戏→保存→GPU 关闭链处理。
                    if mode.is_window_cycle() {
                        cycle.queue_capture(&mut game, 6)?;
                    }
                    #[cfg(feature = "cpu-probe")]
                    if let Some(probe) = &mut cpu_probe { probe.finish(&mut game)?; }
                    close_requested = true;
                    game.main_loop(Event::WindowEvent {
                        window_id: window.id(), event: WindowEvent::CloseRequested,
                    }, target)?;
                }
                if mode == WindowValidationMode::LoadingClose && close_requested && target.exiting() {
                    anyhow::ensure!(!game.gameplay_preparation_pending(),
                        "加载关闭的主帧返回后 worker owner 仍在");
                    close_completed_before_loop_exiting = true;
                }
            }
            if !exiting {
                anyhow::ensure!(started.elapsed() < Duration::from_secs(timeout_seconds),
                    "真实游戏窗口验收超时：loading={loading_frames}, world={world_frames}");
            } else if observed.borrow().is_none() {
                let native = game.native_present_success_count();
                let world = game.app().world();
                if !mode.is_loading_close() {
                    anyhow::ensure!(!world.contains_resource::<SaveThreadHandle>()
                        && world.get_resource::<AutoSaveState>().is_some_and(|state| !state.is_command_attached()),
                        "关闭后保存线程 owner 或 sender 仍然存在");
                    anyhow::ensure!(loading_frames > 0 && world_frames >= required_world_frames && close_requested,
                        "窗口提前退出：loading={loading_frames}, world={world_frames}");
                    let world_native = if mode.is_window_cycle() {
                        anyhow::ensure!(cycle.complete(), "窗口往返分支没有全部完成");
                        cycle.verify_captures()?;
                        stress.verify()?;
                        for verify in &cycle.asset_proofs { verify(world)?; }
                        cycle.native_presented_frames
                    } else if fullscreen_capture_path.is_some() {
                        anyhow::ensure!(fullscreen_capture_result.lock().unwrap().as_ref().is_some_and(|value| value.is_ok()),
                            "full-screen capture did not complete");
                        // Frames above are counted only after a real native
                        // present in their own generation, across this resize.
                        world_frames
                    } else {
                        native.checked_sub(native_before_world.context("没有游戏 present 起点")?)
                            .context("swapchain generation changed during bounded validation")?
                    };
                    anyhow::ensure!(world_native >= required_world_frames, "游戏只完成 {world_native} 次原生 present 成功");
                    if mode == WindowValidationMode::LateCamera {
                        anyhow::ensure!(late_camera.complete(), "真实鼠标输入没有产生已提交的晚相机预览并正常释放");
                        eprintln!("PASS: actual late camera input previews={} motions={} consumed-through={:?}; real binding/capture/normal camera and native present",
                            late_camera.previews, late_camera.motions, late_camera.last_input);
                    }
                    if let Some(tiny_glade::app_config::InputRecordingOpts::Record(path)) = &input_recording {
                        let stream = country_core::input::recording::load_recording(path)?;
                        anyhow::ensure!(stream.frame_count() as u64 >= world_frames, "recording lost gameplay inputs");
                        anyhow::ensure!(stream == world.resource::<country_core::systems::backend::input::input_record_replay::InputRecordReplayState>().stream,
                            "shutdown did not save the complete active input stream");
                        eprintln!("PASS: desktop full input recording frames={} sync_points={} path={}", stream.frame_count(), stream.sync_point_count(), path.display());
                    }
                    if replay_expected != 0 {
                        anyhow::ensure!(replayed.load(std::sync::atomic::Ordering::Relaxed) == replay_expected,
                            "not every recorded frame reached the real input consumers exactly once");
                        eprintln!("PASS: desktop input replay exact actions/time/capture for {replay_expected} frames");
                    }
                    eprintln!("PASS: actual GameLoop loading={loading_frames}, world={world_frames}, world-native-successes={world_native}, world-seconds={:.3}; OS close/save/GPU shutdown completed", world_started.unwrap().elapsed().as_secs_f64());
                } else {
                    anyhow::ensure!(loading_frames > 0 && close_requested && preparation_pending_at_close
                        && close_completed_before_loop_exiting && !game.gameplay_preparation_pending(),
                        "生命周期关闭只依赖退出兜底：mode={mode:?}, loading={loading_frames}, world={world_frames}");
                    anyhow::ensure!(!world.contains_resource::<SaveThreadHandle>()
                        && world.get_resource::<AutoSaveState>().is_none_or(|state| !state.is_command_attached()),
                        "加载关闭后保存线程 owner 或 sender 仍然存在");
                    if mode == WindowValidationMode::LoadingClose {
                        anyhow::ensure!(world.contains_resource::<country_core::resources::shutdown::ShutdownGame>(),
                            "普通加载关闭没有经过真实 ShutdownGameCmd 的 App 消费者");
                    }
                    eprintln!("PASS: actual GameLoop window lifecycle mode={mode:?}; loading={loading_frames}, world={world_frames}; pending preparation cancelled/joined and save/GPU cleanup completed before LoopExiting");
                }
                *observed.borrow_mut() = Some(Ok(()));
            }
            Ok(())
        })();
        if let Err(error) = step {
            if observed.borrow().is_none() { *observed.borrow_mut() = Some(Err(error)); }
            target.exit();
        }
    })?;
    let outcome = result.borrow_mut().take().context("窗口事件循环没有留下游戏验收结果")?;
    // 显式延长账户临时目录至真实保存线程关闭之后。
    drop(account);
    outcome
}

/// Input is injected through the public winit dispatcher. The real loaded
/// binding, action translator, capture protocol and camera controller must all
/// participate; no action/camera/preview resource is manufactured by this test.
#[derive(Default)]
struct LateCameraValidation {
    button: Option<winit::event::MouseButton>,
    motions: u64,
    previews: u64,
    last_receipt: Option<(bevy_ecs::world::WorldId, u64)>,
    last_input: Option<(u64, u64)>,
    release_at: Option<u64>,
    pressed_at: Option<u64>,
    recording: bool,
    initial_view: Option<glam::Mat4>,
    complete: bool,
}

impl LateCameraValidation {
    fn complete(&self) -> bool { self.complete }

    fn after_redraw(
        &mut self,
        game: &mut tiny_glade::game_loop::GameLoop,
        window: &Arc<winit::window::Window>,
        target: &winit::event_loop::EventLoopWindowTarget<()>,
        world_frames: u64,
        frame: Option<&tiny_glade::production_renderer::ProductionFrameObservation>,
    ) -> Result<()> {
        use country_core::{input::action_set::{MainActionSet, DigitalActionPhase},
            resources::mouse_capture::CursorCapturedPosition};
        use winit::event::{ElementState, MouseButton};
        if self.complete || world_frames == 0 { return Ok(()); }
        self.recording = matches!(game.app().world().resource::<country_core::systems::backend::input::input_record_replay::InputRecordReplayState>().mode,
            country_core::input::InputRecordReplayMode::Records);
        if let Some(frame) = frame.filter(|frame|
            self.last_receipt != Some((frame.world, frame.receipt_frame))) {
            self.last_receipt = Some((frame.world, frame.receipt_frame));
            if let Some((consumed, sampled, true)) = frame.late_camera_input {
                anyhow::ensure!(!self.recording, "late camera changed a recorded input frame");
                anyhow::ensure!(sampled > consumed, "晚相机没有采样新的OS输入");
                anyhow::ensure!(frame.present_point.is_some(), "晚相机帧未进入真实present");
                self.previews += 1;
                self.last_input = Some((consumed, sampled));
                eprintln!("LATE_CAMERA native receipt={} consumed={consumed} sampled={sampled}", frame.receipt_frame);
            }
        }
        // DeviceId is an opaque event identity in this explicitly injected
        // fixture; it is never passed to an OS device API.
        let device_id = unsafe { winit::event::DeviceId::dummy() };
        if let Some(at) = self.release_at {
            if world_frames > at + 2 {
                let actions = game.app().world().resource::<MainActionSet>();
                anyhow::ensure!(actions.digital[2].phase == DigitalActionPhase::Idle,
                    "真实camera_rotate_mode在释放后仍保持按下");
                anyhow::ensure!(game.app().world().resource::<CursorCapturedPosition>().0.is_none(),
                    "真实鼠标捕获在释放后仍被保留");
                self.complete = true;
            }
            return Ok(());
        }
        if self.button.is_none() {
            if game.app().world().resource::<VermeerState>().should_freeze_camera() { return Ok(()); }
            let profile = game.app().world().resource::<country_core::game_profile::GameProfile>();
            let bindings = country_core::systems::backend::input::translate_input::BindingsTable::try_from(&profile.key_bindings)?;
            let button = match bindings.slots[37].mouse.context("旋转动作缺少真实鼠标绑定")? {
                0 => MouseButton::Left, 1 => MouseButton::Right, 2 => MouseButton::Middle,
                3 => MouseButton::Back, 4 => MouseButton::Forward, other => MouseButton::Other(other),
            };
            game.main_loop(Event::WindowEvent { window_id: window.id(),
                event: WindowEvent::CursorMoved { device_id,
                    position: winit::dpi::PhysicalPosition::new(240.0, 135.0) } }, target)?;
            game.main_loop(Event::WindowEvent { window_id: window.id(),
                event: WindowEvent::MouseInput { device_id, state: ElementState::Pressed, button } }, target)?;
            self.button = Some(button);
            self.pressed_at = Some(world_frames);
            self.initial_view = Some(game.app().world().resource::<country_core::systems::camera_rig::MainCamera>().read_matrix(0xc0));
            eprintln!("LATE_CAMERA begin real binding={button:?} world_frames={world_frames}");
        }
        if self.previews > 0 || (self.recording && self.pressed_at.is_some_and(|at| world_frames >= at + 8)) {
            if self.recording {
                anyhow::ensure!(self.previews == 0, "recording enabled an unrecorded late pose");
                anyhow::ensure!(Some(game.app().world().resource::<country_core::systems::camera_rig::MainCamera>().read_matrix(0xc0)) != self.initial_view,
                    "injected recording gesture never moved the normal simulation camera");
                eprintln!("INPUT_RECORDING normal camera gesture consumed, late preview stayed frozen");
            }
            game.main_loop(Event::WindowEvent { window_id: window.id(),
                event: WindowEvent::MouseInput { device_id, state: ElementState::Released,
                    button: self.button.expect("pressed above") } }, target)?;
            self.release_at = Some(world_frames);
            return Ok(());
        }
        let anchor = game.app().world().resource::<CursorCapturedPosition>().0
            .unwrap_or(glam::Vec2::new(240.0, 135.0));
        // Events after frame() can arrive after extraction or while the next
        // prepared Update waits for the previous renderer. Keep the assertion
        // on the actual submitted observation, not on the injected count.
        for _ in 0..8 {
            game.main_loop(Event::WindowEvent { window_id: window.id(),
                event: WindowEvent::CursorMoved { device_id,
                    position: winit::dpi::PhysicalPosition::new(anchor.x as f64 + 0.5, anchor.y as f64) } }, target)?;
            // The dispatcher chooses exactly one capture protocol: Locked
            // uses raw device motion; warp capture uses CursorMoved above.
            game.main_loop(Event::DeviceEvent { device_id,
                event: winit::event::DeviceEvent::MouseMotion { delta: (0.5, 0.0) } }, target)?;
            self.motions += 1;
        }
        Ok(())
    }
}


/// 真实窗口/交换链验收状态。resize 与全屏大小必须来自真实 winit WindowEvent；
/// suspend/zero/focus 为明确注入的跨平台语义事件，不伪称 macOS 发出了零尺寸通知。
#[derive(Default)]
struct WindowCycleValidation {
    windowed_only: bool,
    requested_vsync: Option<bool>,
    control_generation: u64,
    phase: u8,
    os_resize_events: u64,
    phase_resize_events: u64,
    phase_generation: Option<u64>,
    previous_generation: Option<u64>,
    previous_native_successes: u64,
    native_presented_frames: u64,
    captures: Arc<std::sync::Mutex<Vec<(u8, std::result::Result<[u32; 2], String>)>>>,
    pixels: Arc<std::sync::Mutex<Vec<(u8, u32, u32, usize)>>>,
    asset_proofs: Vec<Box<dyn Fn(&bevy_ecs::world::World) -> Result<()>>>,
    fullscreen_retries: u8,
    initial_windowed_requested: bool,
    pending_resize: Option<(std::time::Instant, [u32; 2])>,
}

impl WindowCycleValidation {
    const REQUESTED_SIZE: [u32; 2] = [514, 354];

    fn complete(&self) -> bool { self.phase == 6 }

    fn record_asset_proof<T: country_core::render::asset_ack::AssetSnapshot>(&mut self, world: &bevy_ecs::world::World) -> Result<()> {
        use country_core::render::asset_ack::{AssetEntryKey, RenderAssetAcks};
        let key = world.resource::<T>().asset_keys().into_iter().find(|key| matches!(key, AssetEntryKey::Id(_)))
            .context("真实游戏资产库没有可验证条目")?;
        let revision = world.get_resource::<RenderAssetAcks>().and_then(|acks| acks.entry::<T>(&key))
            .context("生产提取没有发布逐条目 applied ACK")?.stages.applied;
        anyhow::ensure!(revision > 0, "资产条目尚未真正应用");
        let source = world.id();
        self.asset_proofs.push(Box::new(move |world| {
            anyhow::ensure!(world.id() == source, "资产确认验证跨越了 World");
            anyhow::ensure!(world.resource::<RenderAssetAcks>().entry_gpu_complete::<T>(&key, revision),
                "真实提交未发布逐条目 GPU 完成确认：{} {key:?} revision={revision}", std::any::type_name::<T>());
            eprintln!("WINDOW_CYCLE actual asset GPU ACK: {} {key:?} revision={revision}", std::any::type_name::<T>());
            Ok(())
        }));
        Ok(())
    }

    fn queue_capture(&self, game: &mut tiny_glade::game_loop::GameLoop, tag: u8) -> Result<()> {
        use tiny_glade::screenshot::{ScreenshotRequest, ScreenshotPurpose, ScreenshotSink};
        let captures = self.captures.clone();
        let pixels = self.pixels.clone();
        tiny_glade::screenshot::gather_screenshot_requests(game.app_mut().world_mut(), [ScreenshotRequest {
            purpose: ScreenshotPurpose::FinalFrame,
            dimensions: Some([64, 48]),
            sink: ScreenshotSink::Rgba(Arc::new(move |w, h, rgba| {
                pixels.lock().unwrap().push((tag, w, h, rgba.len()));
            })),
            completion: Some(Arc::new(move |result| { captures.lock().unwrap().push((tag, result)); })),
        }])?;
        Ok(())
    }

    fn verify_captures(&self) -> Result<()> {
        let captures = self.captures.lock().unwrap();
        let pixels = self.pixels.lock().unwrap();
        for tag in 0..=6 {
            if self.windowed_only && matches!(tag, 4 | 5) { continue; }
            let outcomes = captures.iter().filter(|(id, _)| *id == tag).collect::<Vec<_>>();
            anyhow::ensure!(outcomes.len() == 1, "截图 {tag} 回调次数错误：{captures:?}");
            if tag == 6 {
                anyhow::ensure!(outcomes[0].1.is_err(), "关闭前未提交截图不能报告成功");
                anyhow::ensure!(!pixels.iter().any(|(id, ..)| *id == tag), "取消截图交付了像素");
            } else {
                anyhow::ensure!(outcomes[0].1 == Ok([64, 48]), "组合截图失败：{captures:?}");
                anyhow::ensure!(pixels.iter().filter(|(id, w, h, len)|
                    *id == tag && *w == 64 && *h == 48 && *len == 64 * 48 * 4).count() == 1,
                    "组合截图没有交付唯一真实像素缓冲：{pixels:?}");
            }
        }
        eprintln!("WINDOW_CYCLE screenshot combinations passed: six GPU captures and one shutdown cancellation, exactly once");
        Ok(())
    }

    fn observe_os_event(&mut self, event: &Event<()>, window_id: winit::window::WindowId) {
        if let Event::WindowEvent { window_id: actual, event: WindowEvent::Resized(size) } = event {
            if *actual == window_id {
                self.os_resize_events += 1;
                if self.native_presented_frames > 0 && size.width > 0 && size.height > 0 {
                    self.pending_resize = Some((std::time::Instant::now(), [size.width, size.height]));
                }
                eprintln!("WINDOW_CYCLE actual_os_resize={} extent={}x{}", self.os_resize_events, size.width, size.height);
            }
        }
    }

    fn after_frame(
        &mut self,
        game: &mut tiny_glade::game_loop::GameLoop,
        window: &Arc<winit::window::Window>,
        target: &winit::event_loop::EventLoopWindowTarget<()>,
        world_frames: u64,
        frame: &tiny_glade::production_renderer::ProductionFrameObservation,
    ) -> Result<()> {
        use country_core::render::{frame_constants::FrameConstants, render_config::{FrameRenderConfig, UpscalingPreset}};
        use tiny_glade::game_loop::PENDING_VIDEO_SETTING_FLAGS;
        if self.windowed_only {
            anyhow::ensure!(window.fullscreen().is_none(), "windowed-only validation entered fullscreen");
            anyhow::ensure!(game.app().world().get_resource::<tiny_glade::app_config::LaunchDisplayOverride>()
                .is_some_and(|policy| policy.force_windowed), "windowed launch policy missing after handoff");
        }
        let generation = frame.present_generation.context("生命周期没有原生 generation")?;
        let native = frame.native_present_successes;
        anyhow::ensure!(if self.previous_generation == Some(generation) {
            native > self.previous_native_successes
        } else { native > 0 }, "生命周期帧没有实际原生 present 成功");
        self.native_presented_frames += 1;
        self.previous_generation = Some(generation);
        self.previous_native_successes = native;
        if let Some((started, extent)) = self.pending_resize {
            let latency = started.elapsed();
            anyhow::ensure!(latency < Duration::from_secs(2),
                "resize first correctly sized presentation delayed: expected={extent:?} output={:?} latency={latency:?}", frame.output_extent);
            if frame.output_extent == extent {
                eprintln!("WINDOW_RESIZE_READY extent={extent:?} latency_ms={:.3} native={native} generation={generation}", latency.as_secs_f64() * 1000.0);
                self.pending_resize = None;
            }
        }
        match self.phase {
            0 if world_frames >= 3 && window.fullscreen().is_some() => {
                anyhow::ensure!(!self.windowed_only, "windowed-only validation started fullscreen");
                if !self.initial_windowed_requested {
                    self.initial_windowed_requested = true;
                    window.set_fullscreen(None);
                }
            }
            0 if world_frames >= 3 => {
                #[cfg(target_os = "macos")]
                if !self.windowed_only { exercise_fullscreen_failure_callback(window)?; }
                use country_core::asset_libraries::{mesh_library::AssetMeshLibrary, ssbo_library::AssetSsboLibrary,
                    vao_library::AssetVAOLibrary, texture_library::AssetTextureLibrary,
                    placement_data_library::AssetPlacementDataLibrary};
                self.record_asset_proof::<AssetMeshLibrary>(game.app().world())?;
                self.record_asset_proof::<AssetSsboLibrary>(game.app().world())?;
                self.record_asset_proof::<AssetVAOLibrary>(game.app().world())?;
                self.record_asset_proof::<AssetTextureLibrary>(game.app().world())?;
                self.record_asset_proof::<AssetPlacementDataLibrary>(game.app().world())?;
                self.queue_capture(game, 0)?;
                {
                    let mut config = game.app_mut().world_mut().resource_mut::<GameConfig>();
                    config.video.display.windowed_resolution = Self::REQUESTED_SIZE;
                }
                self.phase_resize_events = self.os_resize_events;
                self.phase_generation = Some(generation);
                // 不自行投递 Resized：必须等 OS 确认真实 drawable 尺寸。
                let immediate = window.request_inner_size(winit::dpi::PhysicalSize::new(
                    Self::REQUESTED_SIZE[0], Self::REQUESTED_SIZE[1]));
                eprintln!("WINDOW_CYCLE requested_size={:?} immediate={immediate:?}", Self::REQUESTED_SIZE);
                self.phase = 1;
            }
            1 if self.os_resize_events > self.phase_resize_events
                && Some(generation) != self.phase_generation => {
                let extent = window.inner_size();
                if [extent.width, extent.height] != Self::REQUESTED_SIZE { return Ok(()); }
                let world = game.app().world();
                let config = world.resource::<FrameRenderConfig>();
                anyhow::ensure!(config.extent_0 == Self::REQUESTED_SIZE, "OS resize 未送达实际 FrameRenderConfig");
                anyhow::ensure!(frame.output_extent == Self::REQUESTED_SIZE,
                    "最终合成图没有覆盖完整窗口：{:?}", frame.output_extent);
                // 原版截断 world extent，TGSR inner extent向上对齐32，再夹[32,10912]。
                anyhow::ensure!(config.extent_8 == [385, 265] && config.extent_10 == [288, 192],
                    "原版两级缩放尺寸不符：{config:?}");
                anyhow::ensure!(config.world_resolution_scale.to_bits() == 0.75_f32.to_bits()
                    && config.tgsr_resolution_scale.to_bits() == UpscalingPreset::Quality.tgsr_resolution_scale().to_bits(),
                    "渲染比例没有消费当前视频设置：{config:?}");
                anyhow::ensure!(world.resource::<FrameConstants>().inner_rendering_resolution == config.extent_10,
                    "GPU 帧常量与真实内部渲染尺寸不一致");
                anyhow::ensure!(Some(generation) != self.phase_generation, "resize 沿用了旧交换链 generation");
                eprintln!("WINDOW_CYCLE resized generation={generation} config={config:?}");
                self.queue_capture(game, 1)?;
                let before = game.frame_progress();
                let counter = game.app().world().resource::<FrameConstants>().frame_counter;
                game.main_loop(Event::Suspended, target)?;
                game.main_loop(Event::WindowEvent { window_id: window.id(), event: WindowEvent::RedrawRequested }, target)?;
                anyhow::ensure!(game.frame_progress() == before
                    && game.app().world().resource::<FrameConstants>().frame_counter == counter,
                    "暂停期间执行了 App update 或渲染");
                game.main_loop(Event::Resumed, target)?;
                self.phase_generation = Some(generation);
                self.phase = 2;
            }
            2 if Some(generation) != self.phase_generation => {
                self.queue_capture(game, 2)?;
                // 零尺寸事件通过同一主循环入口，设置尾部必须在没有 App update 时执行。
                let before = game.frame_progress();
                let counter = game.app().world().resource::<FrameConstants>().frame_counter;
                game.main_loop(Event::WindowEvent { window_id: window.id(),
                    event: WindowEvent::Resized(winit::dpi::PhysicalSize::new(0, 0)) }, target)?;
                let next_vsync = !game.app().world().resource::<GameConfig>().video.vsync_enable;
                if self.windowed_only {
                    let before = game.app().world().resource::<GameConfig>().video.clone();
                    game.app_mut().world_mut().resource_mut::<tiny_glade::gui::settings::LastAppliedVideoSettings>()
                        .populate_empty_from(&before);
                    self.requested_vsync = Some(next_vsync);
                }
                game.app_mut().world_mut().resource_mut::<GameConfig>().video.vsync_enable = next_vsync;
                *PENDING_VIDEO_SETTING_FLAGS.lock().unwrap() |= 2;
                game.main_loop(Event::WindowEvent { window_id: window.id(), event: WindowEvent::RedrawRequested }, target)?;
                if self.windowed_only {
                    self.control_generation = game.window_control_progress().0;
                    anyhow::ensure!(game.app().world().resource::<tiny_glade::gui::settings::LastAppliedVideoSettings>()
                        .0.as_ref().context("missing applied snapshot")?.vsync_enable != next_vsync,
                        "未等待 worker 确认就更新了 LastApplied");
                }
                anyhow::ensure!(game.frame_progress() == before
                    && game.app().world().resource::<FrameConstants>().frame_counter == counter,
                    "零尺寸期间执行了 App update 或渲染");
                anyhow::ensure!(*PENDING_VIDEO_SETTING_FLAGS.lock().unwrap() == 0,
                    "零尺寸跳过了视频设置尾部");
                let settings: GameConfig = ron::from_str(&std::fs::read_to_string(tiny_glade::app_config::settings_file_path())?)?;
                anyhow::ensure!(settings.video.vsync_enable == next_vsync, "零尺寸设置没有实际写盘");
                let size = window.inner_size();
                game.main_loop(Event::WindowEvent { window_id: window.id(), event: WindowEvent::Resized(size) }, target)?;
                game.main_loop(Event::WindowEvent { window_id: window.id(), event: WindowEvent::Focused(false) }, target)?;
                game.main_loop(Event::WindowEvent { window_id: window.id(), event: WindowEvent::Focused(true) }, target)?;
                self.phase_generation = Some(generation);
                self.phase = 3;
                eprintln!("WINDOW_CYCLE suspend/resume and zero/settings branches completed; generation={generation}");
            }
            3 if Some(generation) != self.phase_generation => {
                if self.windowed_only {
                    let wanted = self.requested_vsync.context("missing requested vsync")?;
                    let (_, applied) = game.window_control_progress();
                    let snapshot = game.app().world().resource::<tiny_glade::gui::settings::LastAppliedVideoSettings>();
                    if applied < self.control_generation || snapshot.0.as_ref().is_none_or(|video| video.vsync_enable != wanted) {
                        return Ok(());
                    }
                    anyhow::ensure!(window.fullscreen().is_none(), "windowed-only mode entered fullscreen");
                    self.queue_capture(game, 3)?;
                    self.phase = 6;
                    eprintln!("WINDOWED_CYCLE vsync confirmed generation={} actual_os_resizes={}", self.control_generation, self.os_resize_events);
                    return Ok(());
                }
                self.queue_capture(game, 3)?;
                let world = game.app_mut().world_mut();
                world.resource_mut::<GameConfig>().video.display.fullscreen = true;
                world.send_event(tiny_glade::app_config::ApplyVideoSettingsCmd(1));
                self.phase_resize_events = self.os_resize_events;
                self.phase_generation = Some(generation);
                self.phase = 4;
            }
            4 if window.fullscreen().is_some() && self.os_resize_events > self.phase_resize_events
                && Some(generation) != self.phase_generation => {
                let actual = window.inner_size();
                if game.app().world().resource::<FrameRenderConfig>().extent_0 != [actual.width, actual.height] {
                    return Ok(());
                }
                self.queue_capture(game, 4)?;
                let world = game.app_mut().world_mut();
                world.resource_mut::<GameConfig>().video.display.fullscreen = false;
                world.send_event(tiny_glade::app_config::ApplyVideoSettingsCmd(1));
                self.phase_resize_events = self.os_resize_events;
                self.phase_generation = Some(generation);
                self.phase = 5;
                eprintln!("WINDOW_CYCLE actual fullscreen entered; returning to windowed");
            }
            4 if window.fullscreen().is_none() && self.os_resize_events > self.phase_resize_events
                && Some(generation) != self.phase_generation
                && [window.inner_size().width, window.inner_size().height] == Self::REQUESTED_SIZE => {
                // AppKit may reject a Space transition during another native
                // animation. Recovery must return; the actual enter/exit
                // assertions above remain mandatory on the bounded retry.
                anyhow::ensure!(self.fullscreen_retries < 2, "OS repeatedly rejected fullscreen entry");
                self.fullscreen_retries += 1;
                self.phase_resize_events = self.os_resize_events;
                self.phase_generation = Some(generation);
                game.app_mut().world_mut().send_event(tiny_glade::app_config::ApplyVideoSettingsCmd(1));
                eprintln!("WINDOW_CYCLE recovered rejected fullscreen transition; retry={}", self.fullscreen_retries);
            }
            5 if window.fullscreen().is_none() && self.os_resize_events > self.phase_resize_events
                && Some(generation) != self.phase_generation => {
                let size = window.inner_size();
                let config = game.app().world().resource::<FrameRenderConfig>();
                if config.extent_0 != [size.width, size.height] { return Ok(()); }
                anyhow::ensure!(config.extent_0 == Self::REQUESTED_SIZE,
                    "全屏返回没有恢复先前窗口尺寸：{config:?}");
                anyhow::ensure!(frame.output_extent == Self::REQUESTED_SIZE,
                    "全屏返回后的最终合成图没有覆盖完整窗口：{:?}", frame.output_extent);
                self.queue_capture(game, 5)?;
                self.phase = 6;
                eprintln!("WINDOW_CYCLE all phases completed; generation={generation} actual_os_resizes={}", self.os_resize_events);
            }
            _ => {}
        }
        Ok(())
    }
}

/// Test-owned stimuli for the separate stable/effects/resize long runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WindowStressKind { Stable, Effects, Resize }
struct WindowStress {
    kind: Option<WindowStressKind>,
    pending: Option<(u64, Option<u64>, u64, [u32; 2], bool)>,
    confirmed: u64,
    last_request: u64,
}
impl WindowStress {
    fn from_environment() -> Result<Self> {
        let kind = match std::env::var("TG_VALIDATE_WINDOW_STRESS").ok().as_deref() {
            None => None,
            Some("stable") => Some(WindowStressKind::Stable),
            Some("effects") => Some(WindowStressKind::Effects),
            Some("resize") => Some(WindowStressKind::Resize),
            Some(other) => anyhow::bail!("unknown window stress: {other}"),
        };
        Ok(Self { kind, pending: None, confirmed: 0, last_request: 0 })
    }
    fn after_frame(&mut self, game: &mut tiny_glade::game_loop::GameLoop,
        window: &Arc<winit::window::Window>, frames: u64,
        frame: &tiny_glade::production_renderer::ProductionFrameObservation, os_resizes: u64,
    ) -> Result<()> {
        use country_core::render::render_config::{FrameRenderConfig, UpscalingMode};
        let Some(kind) = self.kind else { return Ok(()); };
        if let Some((requested, old_generation, old_resizes, extent, enabled)) = self.pending {
            let config = game.app().world().resource::<FrameRenderConfig>();
            let video = &game.app().world().resource::<GameConfig>().video;
            let accepted = if kind == WindowStressKind::Resize {
                os_resizes > old_resizes && frame.present_generation != old_generation
                    && frame.output_extent == extent && config.extent_0 == extent
            } else {
                frames >= requested + 3 && video.dof_enable == enabled
                    && (config.tgsr_resolution_scale < 1.0) == enabled
                    && frame.output_extent == config.extent_0
            };
            if !accepted {
                anyhow::ensure!(frames < requested + 600, "window stress never acknowledged: {kind:?}");
                return Ok(());
            }
            self.confirmed += 1;
            self.pending = None;
            eprintln!("WINDOW_STRESS confirmed={kind:?} changes={} world_frame={frames} generation={:?} extent={:?}",
                self.confirmed, frame.present_generation, frame.output_extent);
        }
        if kind == WindowStressKind::Stable || frames < self.last_request + 120 { return Ok(()); }
        let enabled = self.confirmed % 2 == 0;
        let extent = if enabled { [640, 360] } else { [480, 270] };
        if kind == WindowStressKind::Resize {
            game.app_mut().world_mut().resource_mut::<GameConfig>().video.display.windowed_resolution = extent;
            let _ = window.request_inner_size(winit::dpi::PhysicalSize::new(extent[0], extent[1]));
        } else {
            let world = game.app_mut().world_mut();
            {
                let mut config = world.resource_mut::<GameConfig>();
                config.video.dof_enable = enabled;
                config.video.upscaling_mode = if enabled { UpscalingMode::Tgsr } else { UpscalingMode::Off };
                config.video.upscaling_preset = country_core::render::render_config::UpscalingPreset::Quality;
            }
            world.send_event(tiny_glade::app_config::ApplyVideoSettingsCmd(2));
        }
        self.pending = Some((frames, frame.present_generation, os_resizes, extent, enabled));
        self.last_request = frames;
        Ok(())
    }
    fn verify(&self) -> Result<()> {
        if matches!(self.kind, Some(WindowStressKind::Effects | WindowStressKind::Resize)) {
            anyhow::ensure!(self.confirmed >= 2 && self.pending.is_none(), "window stress did not complete both directions");
        }
        if let Some(kind) = self.kind { eprintln!("WINDOW_STRESS PASS kind={kind:?} confirmed_changes={}", self.confirmed); }
        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn owned_window_is_native_fullscreen(window: &winit::window::Window) -> Result<bool> {
    use objc::{Message, runtime::{Object, Sel}};
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    let RawWindowHandle::AppKit(handle) = window.window_handle()?.as_raw() else { return Ok(false); };
    let view = handle.ns_view.as_ptr().cast::<Object>();
    let native: *mut Object = unsafe { (&*view).send_message(Sel::register("window"), ()) }?;
    let style: usize = unsafe { (&*native).send_message(Sel::register("styleMask"), ()) }?;
    // NSWindow.h: NSWindowStyleMaskFullScreen. Read only by this fixture.
    Ok(style & (1 << 14) != 0)
}

#[cfg(target_os = "macos")]
fn save_owned_window_image(window: &winit::window::Window, path: &std::path::Path) -> Result<()> {
    use core_graphics::{color_space::CGColorSpace, context::CGContext,
        geometry::{CGRect, CGPoint, CGSize}, image::CGImageAlphaInfo, window as cg_window};
    use objc::{Message, runtime::{Object, Sel}};
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    let RawWindowHandle::AppKit(handle) = window.window_handle()?.as_raw() else {
        anyhow::bail!("not an AppKit window");
    };
    let view = handle.ns_view.as_ptr().cast::<Object>();
    let native: *mut Object = unsafe { (&*view).send_message(Sel::register("window"), ()) }?;
    let number: isize = unsafe { (&*native).send_message(Sel::register("windowNumber"), ()) }?;
    anyhow::ensure!(number > 0, "owned window has no native window number");
    // Only this fixture's own window. A denied OS capture remains unavailable;
    // the independent main-image GPU capture still identifies the render stage.
    unsafe extern "C" { static CGRectNull: CGRect; }
    // core-graphics 0.23.1 shifts every list flag by one bit. The actual SDK
    // CGWindow.h defines IncludingWindow=1<<3; its crate constant selects all
    // windows except desktop elements instead of selecting this owner.
    const INCLUDING_THIS_WINDOW: u32 = 1 << 3;
    let image = cg_window::create_image(unsafe { CGRectNull }, INCLUDING_THIS_WINDOW,
        number as u32, cg_window::kCGWindowImageBoundsIgnoreFraming | cg_window::kCGWindowImageBestResolution)
        .context("CoreGraphics did not supply this owned window image")?;
    let (width, height) = (image.width(), image.height());
    let mut context = CGContext::create_bitmap_context(None, width, height, 8, width * 4,
        &CGColorSpace::create_device_rgb(), CGImageAlphaInfo::CGImageAlphaPremultipliedLast as u32);
    context.draw_image(CGRect::new(&CGPoint::new(0.0, 0.0), &CGSize::new(width as f64, height as f64)), &image);
    image::save_buffer(path, context.data(), width as u32, height as u32, image::ColorType::Rgba8)?;
    let invalid = context.data().chunks_exact(4).filter(|p|
        (p[0] == 255 && p[1] == 0 && p[2] == 255) || (p[0] == 0 && p[1] == 0 && p[2] == 0 && p[3] > 0)).count();
    eprintln!("FULLSCREEN_CAPTURE invalid_pixels={invalid} total_pixels={}", width * height);
    anyhow::ensure!(invalid * 100 < width * height, "native full-screen image still contains an old-size region / unpainted pixels: {invalid}/{}", width * height);
    eprintln!("FULLSCREEN_CAPTURE native_window={path:?} id={number} extent={width}x{height} inner={:?} outer={:?} fullscreen={:?} monitor={:?}",
        window.inner_size(), window.outer_size(), window.fullscreen(), window.current_monitor().map(|m| m.size()));
    Ok(())
}

#[cfg(target_os = "macos")]
fn verify_metal_layer_geometry(window: &winit::window::Window) -> Result<()> {
    use objc::{Encode, Encoding, Message, runtime::{Object, Sel}};
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    struct Point { x: f64, y: f64 }
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    struct Size { width: f64, height: f64 }
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    struct Rect { origin: Point, size: Size }
    unsafe impl Encode for Rect {
        fn encode() -> Encoding { unsafe { Encoding::from_str("{CGRect={CGPoint=dd}{CGSize=dd}}") } }
    }
    unsafe impl Encode for Size {
        fn encode() -> Encoding { unsafe { Encoding::from_str("{CGSize=dd}") } }
    }
    let RawWindowHandle::AppKit(handle) = window.window_handle()?.as_raw() else {
        anyhow::bail!("layer geometry requires the test-owned AppKit view");
    };
    unsafe {
        let view = &*handle.ns_view.as_ptr().cast::<Object>();
        let bounds: Rect = view.send_message(Sel::register("bounds"), ())?;
        let layer: *mut Object = view.send_message(Sel::register("layer"), ())?;
        anyhow::ensure!(!layer.is_null(), "test window has no native layer");
        let frame: Rect = (&*layer).send_message(Sel::register("frame"), ())?;
        let drawable: Size = (&*layer).send_message(Sel::register("drawableSize"), ())?;
        let physical = window.inner_size();
        let view_frame: Rect = (&*view).send_message(Sel::register("frame"), ())?;
        let parent: *mut Object = (&*view).send_message(Sel::register("superview"), ())?;
        let parent_bounds: Option<Rect> = if parent.is_null() { None } else {
            Some((&*parent).send_message(Sel::register("bounds"), ())?)
        };
        let gravity: *mut Object = (&*layer).send_message(Sel::register("contentsGravity"), ())?;
        let gravity_bytes: *const std::ffi::c_char = (&*gravity).send_message(Sel::register("UTF8String"), ())?;
        let gravity = std::ffi::CStr::from_ptr(gravity_bytes).to_string_lossy();
        let presentation: *mut Object = (&*layer).send_message(Sel::register("presentationLayer"), ())?;
        let presentation_frame: Option<Rect> = if presentation.is_null() { None } else {
            Some((&*presentation).send_message(Sel::register("frame"), ())?)
        };
        eprintln!("WINDOW_LAYER view={bounds:?} view_frame={view_frame:?} parent_bounds={parent_bounds:?} layer={frame:?} presentation_frame={presentation_frame:?} gravity={gravity} drawable={drawable:?} physical={physical:?}");
        anyhow::ensure!((frame.origin.x - bounds.origin.x).abs() < 0.5
            && (frame.origin.y - bounds.origin.y).abs() < 0.5
            && (frame.size.width - bounds.size.width).abs() < 0.5
            && (frame.size.height - bounds.size.height).abs() < 0.5,
            "native Metal layer does not cover resized view: view={bounds:?}, layer={frame:?}");
        anyhow::ensure!((drawable.width - f64::from(physical.width)).abs() < 0.5
            && (drawable.height - f64::from(physical.height)).abs() < 0.5,
            "native drawable size lags physical window: drawable={drawable:?}, physical={physical:?}");
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn exercise_fullscreen_failure_callback(window: &winit::window::Window) -> Result<()> {
    use objc::{Message, runtime::{Object, Sel}};
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    anyhow::ensure!(window.fullscreen().is_none(), "failure injection needs a windowed owner");
    let RawWindowHandle::AppKit(handle) = window.window_handle()?.as_raw() else {
        anyhow::bail!("macOS regression has no AppKit view");
    };
    // The real test-owned NSWindow and its actual winit delegate, on AppKit's
    // main thread. Invoke the observed failure selector exactly once, without
    // substituting OS events or declaring a successful fullscreen transition.
    unsafe {
        let view = &*handle.ns_view.as_ptr().cast::<Object>();
        let native: *mut Object = view.send_message(Sel::register("window"), ())?;
        anyhow::ensure!(!native.is_null(), "test view has no NSWindow");
        let delegate: *mut Object = (&*native).send_message(Sel::register("delegate"), ())?;
        anyhow::ensure!(!delegate.is_null(), "test window has no winit delegate");
        let (): () = (&*delegate).send_message(Sel::register("windowDidFailToEnterFullScreen:"), (native,))?;
    }
    anyhow::ensure!(window.fullscreen().is_none(), "failed transition retained fullscreen state");
    eprintln!("WINDOW_CYCLE injected actual AppKit fullscreen failure callback returned without deadlock");
    Ok(())
}

/// Window logging wraps the shared real ECS/raster stimulus protocol.
struct PerformanceWorkload {
    inner: performance_workload::PerformanceWorkload,
    last_frame: u64,
    announced: bool,
    diagnostics: bool,
}
impl PerformanceWorkload {
    fn from_environment() -> Result<Option<Self>> {
        Ok(performance_workload::PerformanceWorkload::from_environment()?.map(|inner| Self {
            inner, last_frame: 0, announced: false,
            diagnostics: std::env::var("TG_PERF_DIAGNOSTICS").is_ok_and(|value| value == "1"),
        }))
    }
    fn after_frame(&mut self, world: &mut bevy_ecs::world::World, frame: u64,
        observation: Option<&tiny_glade::production_renderer::ProductionFrameObservation>, elapsed: Duration) -> Result<()> {
        if frame == 0 || frame == self.last_frame { return Ok(()); }
        self.last_frame = frame;
        if self.diagnostics && let Some(observation) = observation {
            eprintln!("PERF_FRAME frame={frame} elapsed_ns={} receipt={} native_successes={}",
                elapsed.as_nanos(), observation.receipt_frame, observation.native_present_successes);
        }
        if !self.announced {
            self.announced = true;
            let config = world.resource::<country_core::render::render_config::FrameRenderConfig>();
            anyhow::ensure!(observation.context("missing actual performance frame")?.output_extent == [1280, 720]
                && config.world_resolution_scale.to_bits() == 1.0_f32.to_bits(), "benchmark did not start at its requested window/scale");
            let expected = Duration::from_secs_f64(1.0 / 60.0);
            anyhow::ensure!(matches!(world.get_resource::<bevy_time::TimeUpdateStrategy>(),
                Some(bevy_time::TimeUpdateStrategy::ManualDuration(duration)) if *duration == expected),
                "actual gameplay lost its fixed time strategy");
            let time = world.resource::<bevy_time::Time>();
            let clock = world.resource::<country_core::resources::game_clock::GameClock>();
            eprintln!("PERF_INPUT_CLOCK {}", serde_json::json!({
                "schema":2,"fixed_delta_ns":expected.as_nanos().to_string(),
                "elapsed_ns":time.elapsed().as_nanos().to_string(),"delta_ns":time.delta().as_nanos().to_string(),
                "game_frame":clock.frame_count,"game_total_bits":clock.total_secs.to_bits(),
                "game_delta_bits":clock.delta.to_bits(),"paused":clock.paused,"ran":clock.ran,
            }));
            eprintln!("PERF_WORLD_FIRST scenario={} wall_ms={:.3} world={:?}", self.inner.scenario(), elapsed.as_secs_f64()*1000., world.id());
        }
        let observation = observation.context("missing actual input observation")?;
        let delta = self.inner.after_frame(world, frame, observation.output_extent, &observation.draw_list_lengths)?;
        if delta.clones_created > 0 {
            eprintln!("PERF_WORKLOAD cloned_actual_entities={}", delta.clones_created);
        }
        if frame == 199 {
            eprintln!("PERF_INPUT_SCENE {}", serde_json::to_string(self.inner.first_scene().unwrap())?);
            if self.inner.scenario() == "many_draws" {
                eprintln!("PERF_WORKLOAD actual_draw_list_entries={}", observation.draw_list_lengths.iter().sum::<usize>());
            }
        }
        if self.diagnostics && frame % 200 == 0 {
            eprintln!("PERF_WORKLOAD {}", self.inner.summary());
            let config = world.resource::<country_core::render::render_config::FrameRenderConfig>();
            eprintln!("PERF_CONFIG frame={frame} output={}x{} world={}x{} inner={}x{} world_scale={}",
                config.extent_0[0], config.extent_0[1], config.extent_8[0], config.extent_8[1],
                config.extent_10[0], config.extent_10[1], config.world_resolution_scale);
        }
        Ok(())
    }
}

#[cfg(feature = "cpu-probe")]
struct CpuProbeValidation {
    output: PathBuf,
    config: tiny_glade::render_cpu_probe::CpuProbeConfig,
    main: Option<tiny_glade::render_cpu_probe::CpuProbeSession>,
    last: Option<(bevy_ecs::world::WorldId, u64)>,
    dumped: bool,
}

#[cfg(feature = "cpu-probe")]
impl CpuProbeValidation {
    fn from_environment() -> Result<Option<Self>> {
        let Some(output) = std::env::var_os("TG_CPU_PROBE_OUT") else { return Ok(None); };
        let number = |name, default| -> Result<usize> {
            Ok(std::env::var(name).ok().map(|value| value.parse()).transpose()?.unwrap_or(default))
        };
        let sample_frames = number("TG_CPU_PROBE_SAMPLES", 3000)?;
        Ok(Some(Self { output: PathBuf::from(output),
            config: tiny_glade::render_cpu_probe::CpuProbeConfig {
                label: std::env::var("TG_PERF_SCENARIO").unwrap_or_else(|_| "window".into()),
                warmup_frames: number("TG_CPU_PROBE_WARMUP", 600)? as u64,
                sample_frames, capacity: sample_frames.checked_add(65).context("probe capacity overflow")?,
            }, main: None, last: None, dumped: false }))
    }

    fn after_frame(&mut self, game: &mut tiny_glade::game_loop::GameLoop, frame: u64,
        observation: Option<&tiny_glade::production_renderer::ProductionFrameObservation>) -> Result<()> {
        if self.dumped || frame == 0 { return Ok(()); }
        let Some(observation) = observation.filter(|observation|
            matches!(observation.kind, tiny_glade::production_renderer::ProductionGraphKind::Main { world_rendered: true, .. })) else { return Ok(()); };
        let identity = (observation.world, observation.receipt_frame);
        if self.last == Some(identity) { return Ok(()); }
        if self.main.is_none() {
            game.start_cpu_probe(self.config.clone())?;
            self.main = Some(tiny_glade::render_cpu_probe::CpuProbeSession::new(self.config.clone())?);
        }
        if self.last.is_some_and(|previous| previous.0 != identity.0) {
            self.main.as_mut().unwrap().reset_baseline();
        }
        self.last = Some(identity);
        self.main.as_mut().unwrap().record(frame, Some(observation.receipt_frame), frame, true);
        Ok(())
    }

    fn finish(&mut self, game: &mut tiny_glade::game_loop::GameLoop) -> Result<()> {
        if self.dumped { return Ok(()); }
        let render = game.finish_cpu_probe().context("render probe was not installed")?.to_json();
        let main = self.main.as_ref().context("main probe was not installed")?.to_json();
        let valid = render["valid"] == true && main["valid"] == true;
        let report = serde_json::json!({ "schema": "windows-perf-session-v1", "render": render, "main": main });
        if let Some(parent) = self.output.parent() { std::fs::create_dir_all(parent)?; }
        std::fs::write(&self.output, serde_json::to_vec_pretty(&report)?)?;
        self.dumped = true;
        anyhow::ensure!(valid, "CPU probe incomplete/invalid; see {}", self.output.display());
        Ok(())
    }
}
