//! 默认验证加载图→原 presentation resolve→真实 WSI，使用显式加载 World 与时钟。
//! TG_VALIDATE_GAMEPLAY_WSI=1 则验证真实 GameLoop 的双 App 交接、非空游戏帧、
//! 原生 present 成功及 OS 关闭、保存线程和 GPU 退出；两种验收分别记结果。
//! TG_VALIDATE_WINDOW_LIFECYCLE=loading_close/suspended_close 则分别验证真实加载中关闭。
use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc, time::Duration};

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
    if let Some(value) = std::env::var_os("TG_VALIDATE_WINDOW_LIFECYCLE") {
        let mode = match value.to_str() {
            Some("loading_close") => WindowValidationMode::LoadingClose,
            Some("suspended_close") => WindowValidationMode::SuspendedClose,
            Some("window_cycle") => WindowValidationMode::WindowCycle,
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
    let mut renderer = ProductionRenderer::new(
        window.clone(),
        installed.join("compiled-shaders"),
        shaders.clone(),
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
                // The back half runs on the render thread; settle it to observe it.
                driver.settle_render(&mut render)?;
                let renderer = &mut render.reclaim().renderer;
                if let Some(error) = renderer.take_error() { return Err(error); }
                frames += 1;
                if frames == 3 {
                    let presents = renderer.native_present_success_count();
                    anyhow::ensure!(presents > 0,
                        "production loading graph produced no native SUCCESS/SUBOPTIMAL WSI present");
                    renderer.shutdown()?;
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
                render.reclaim().renderer.resize([size.width, size.height])
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
}

impl WindowValidationMode {
    fn is_loading_close(self) -> bool {
        matches!(self, Self::LoadingClose | Self::SuspendedClose)
    }
}

/// 只在独立生命周期模式启动一次真实 UserEvent，Wait 模式也有超时唤醒。
/// 正常结束会立即停止并 join 此测试线程，不留脱离的定时任务。
struct WindowLifecycleWatchdog {
    stop: Option<std::sync::mpsc::Sender<()>>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl WindowLifecycleWatchdog {
    fn new(proxy: winit::event_loop::EventLoopProxy<()>, timeout: Duration) -> Result<Self> {
        let (stop, stopped) = std::sync::mpsc::channel();
        let worker = std::thread::Builder::new()
            .name("window-lifecycle-watchdog".into())
            .spawn(move || {
                if matches!(stopped.recv_timeout(timeout),
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout))
                {
                    let _ = proxy.send_event(());
                }
            })?;
        Ok(Self { stop: Some(stop), worker: Some(worker) })
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

    let installed = PathBuf::from(
        std::env::var_os("TG_INSTALL_ROOT").context("必须提供原版安装树 TG_INSTALL_ROOT")?,
    ).canonicalize()?;
    // 验收用独立账户，保存线程仍走真实启动与关闭路径。
    let account = tempfile::tempdir()?;
    paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;
    std::env::set_current_dir(&installed)?;
    utils::local_time::init_local_time();
    tiny_glade::load_language_library::load_language_library(None, None);
    let cli = CliArgs::parse_from(["original-gameplay-wsi"]);
    let mut config = get_app_config(&cli)?;
    if mode == WindowValidationMode::WindowCycle {
        // 本模式的真实窗口从同一 windowed 配置启动，之后才验收全屏往返。
        config.video.display.fullscreen = false;
        config.video.display.windowed_resolution = [480, 270];
        // 通过原 run_game_thread 启动生产者初始化 FrameRenderConfig，
        // 不把缺少运行时设置生产者的直接资源改写当作缩放功能。
        config.video.world_resolution_scale = Some(0.75);
        config.video.upscaling_mode = country_core::render::render_config::UpscalingMode::Tgsr;
        config.video.upscaling_preset = country_core::render::render_config::UpscalingPreset::Quality;
    }
    anyhow::ensure!(!config.dev.demo && !config.dev.skip_splash,
        "必须验证完整版的真实 loading→gameplay 交接");
    let mut app = run_game_thread(config, &cli, WindowSize::new(480, 270))?;
    app.insert_resource(country_core::resources::platform::RuntimePlatform::from_detection(false, false));
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    let event_loop = EventLoop::new()?;
    let window = Arc::new(WindowBuilder::new()
        .with_title("Tiny Glade — original gameplay WSI validation")
        .with_visible(false)
        .with_inner_size(winit::dpi::PhysicalSize::new(480, 270))
        .build(&event_loop)?);
    let mut game = GameLoop::new(app, window.clone(), GameLoopOptions {
        compiled_shader_root: installed.join("compiled-shaders"),
        frame_limit: None,
        input_recording: None,
    }, shaders)?;
    // 本验收直接驱动 main_loop，先执行生产 run() 的真实监视器枚举前置步骤。
    let mut monitors = country_core::resources::window::AvailableMonitors::default();
    GameLoop::extract_available_monitors_info(&mut monitors, &event_loop);
    game.app_mut().world_mut().insert_resource(monitors);
    let result = Rc::new(RefCell::new(None::<Result<()>>));
    let observed = result.clone();
    let started = std::time::Instant::now();
    let _watchdog = if mode == WindowValidationMode::Gameplay {
        None
    } else {
        Some(WindowLifecycleWatchdog::new(event_loop.create_proxy(),
            Duration::from_secs(if mode == WindowValidationMode::WindowCycle { 180 } else { 60 }))?)
    };
    let mut close_completed_before_loop_exiting = false;
    let mut preparation_pending_at_close = false;
    let mut world_frames = 0u64;
    let mut loading_frames = 0u64;
    let mut native_before_world = None;
    let mut previous_receipt = None;
    let mut close_requested = false;
    let mut cycle = WindowCycleValidation::default();
    event_loop.run(move |event, target| {
        let redraw = matches!(&event, Event::WindowEvent { event: WindowEvent::RedrawRequested, .. });
        let exiting = matches!(&event, Event::LoopExiting);
        let step = (|| -> Result<()> {
            // 本模式的唯一 UserEvent 来自 watchdog。必须记失败，再由真实退出事件清理。
            if mode != WindowValidationMode::Gameplay && matches!(&event, Event::UserEvent(())) {
                anyhow::bail!("窗口生命周期验收超时：mode={mode:?}, loading={loading_frames}, world={world_frames}, close={close_requested}, preparation_pending={}",
                    game.gameplay_preparation_pending());
            }
            let mut native_world_frame = false;
            let native_before_frame = game.native_present_success_count();
            let completed_before = game.frame_progress().completed_frames;
            if mode == WindowValidationMode::WindowCycle {
                cycle.observe_os_event(&event, window.id());
            }
            game.main_loop(event, target)?;
            if redraw && game.frame_progress().completed_frames > completed_before {
                if let Some(frame) = game.frame_observation().cloned() {
                    match &frame.kind {
                        ProductionGraphKind::Loading => loading_frames += 1,
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
                                        game.native_present_success_count() > native_before_frame
                                    } else { game.native_present_success_count() > 0 }
                                });
                            if mode == WindowValidationMode::WindowCycle && !cycle_native_ready {
                                // 原生acquire允许暂时无图像/旧generation背压；保World/draw门，
                                // 此帧不推进生命周期、receipt或native成功计数。
                            } else {
                            anyhow::ensure!(frame.present_point.is_some() && frame.present_generation.is_some(),
                                "生产世界没有 WSI present ticket");
                            anyhow::ensure!(previous_receipt != Some(frame.receipt_frame), "重复统计同一游戏帧");
                            previous_receipt = Some(frame.receipt_frame);
                            native_before_world.get_or_insert(native_before_frame);
                            world_frames += 1;
                            native_world_frame = true;
                            eprintln!("GAMEPLAY_WSI world_frame={world_frames} receipt={} native_successes={} draws={:?}",
                                frame.receipt_frame, game.native_present_success_count(), frame.draw_list_lengths);
                            }
                        }
                        _ => anyhow::bail!("出现空世界或 diagnostic minimal 图"),
                    }
                }
                let lifecycle_loading_presented = game.frame_observation().is_some_and(|frame| {
                    matches!(&frame.kind, ProductionGraphKind::Loading)
                        && frame.present_point.is_some()
                        && frame.present_generation.is_some()
                }) && game.native_present_success_count() > native_before_frame;
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
                if mode == WindowValidationMode::WindowCycle && native_world_frame {
                    cycle.after_frame(&mut game, &window, target, world_frames, native_before_frame)?;
                }
                if !mode.is_loading_close() && world_frames >= 12
                    && (mode != WindowValidationMode::WindowCycle || cycle.complete()) && !close_requested {
                    // 发送关闭窗口事件，由现有 OS→游戏→保存→GPU 关闭链处理。
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
                anyhow::ensure!(started.elapsed() < Duration::from_secs(180),
                    "真实游戏窗口验收超时：loading={loading_frames}, world={world_frames}");
            } else if observed.borrow().is_none() {
                let native = game.native_present_success_count();
                let world = game.app().world();
                if !mode.is_loading_close() {
                    anyhow::ensure!(!world.contains_resource::<SaveThreadHandle>()
                        && world.get_resource::<AutoSaveState>().is_some_and(|state| !state.is_command_attached()),
                        "关闭后保存线程 owner 或 sender 仍然存在");
                    anyhow::ensure!(loading_frames > 0 && world_frames >= 12 && close_requested,
                        "窗口提前退出：loading={loading_frames}, world={world_frames}");
                    let world_native = if mode == WindowValidationMode::WindowCycle {
                        anyhow::ensure!(cycle.complete(), "窗口往返分支没有全部完成");
                        cycle.native_presented_frames
                    } else {
                        native.checked_sub(native_before_world.context("没有游戏 present 起点")?)
                            .context("swapchain generation changed during bounded validation")?
                    };
                    anyhow::ensure!(world_native >= 12, "游戏只完成 {world_native} 次原生 present 成功");
                    eprintln!("PASS: actual GameLoop loading={loading_frames}, world={world_frames}, world-native-successes={world_native}; OS close/save/GPU shutdown completed");
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


/// 真实窗口/交换链验收状态。resize 与全屏大小必须来自真实 winit WindowEvent；
/// suspend/zero/focus 为明确注入的跨平台语义事件，不伪称 macOS 发出了零尺寸通知。
#[derive(Default)]
struct WindowCycleValidation {
    phase: u8,
    os_resize_events: u64,
    phase_resize_events: u64,
    phase_generation: Option<u64>,
    previous_generation: Option<u64>,
    native_presented_frames: u64,
}

impl WindowCycleValidation {
    const REQUESTED_SIZE: [u32; 2] = [514, 354];

    fn complete(&self) -> bool { self.phase == 6 }

    fn observe_os_event(&mut self, event: &Event<()>, window_id: winit::window::WindowId) {
        if let Event::WindowEvent { window_id: actual, event: WindowEvent::Resized(size) } = event {
            if *actual == window_id {
                self.os_resize_events += 1;
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
        native_before_frame: u64,
    ) -> Result<()> {
        use country_core::render::{frame_constants::FrameConstants, render_config::{FrameRenderConfig, UpscalingPreset}};
        use tiny_glade::game_loop::PENDING_VIDEO_SETTING_FLAGS;
        let frame = game.frame_observation().context("生命周期没有真实游戏帧")?.clone();
        let generation = frame.present_generation.context("生命周期没有原生 generation")?;
        let native = game.native_present_success_count();
        anyhow::ensure!(if self.previous_generation == Some(generation) {
            native > native_before_frame
        } else { native > 0 }, "生命周期帧没有实际原生 present 成功");
        self.native_presented_frames += 1;
        self.previous_generation = Some(generation);
        match self.phase {
            0 if world_frames >= 3 => {
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
            1 if self.os_resize_events > self.phase_resize_events => {
                let extent = window.inner_size();
                if [extent.width, extent.height] != Self::REQUESTED_SIZE { return Ok(()); }
                let world = game.app().world();
                let config = world.resource::<FrameRenderConfig>();
                anyhow::ensure!(config.extent_0 == Self::REQUESTED_SIZE, "OS resize 未送达实际 FrameRenderConfig");
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
                let before = game.frame_progress();
                let counter = world.resource::<FrameConstants>().frame_counter;
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
                // 零尺寸事件通过同一主循环入口，设置尾部必须在没有 App update 时执行。
                let before = game.frame_progress();
                let counter = game.app().world().resource::<FrameConstants>().frame_counter;
                game.main_loop(Event::WindowEvent { window_id: window.id(),
                    event: WindowEvent::Resized(winit::dpi::PhysicalSize::new(0, 0)) }, target)?;
                let next_vsync = !game.app().world().resource::<GameConfig>().video.vsync_enable;
                game.app_mut().world_mut().resource_mut::<GameConfig>().video.vsync_enable = next_vsync;
                *PENDING_VIDEO_SETTING_FLAGS.lock().unwrap() |= 2;
                game.main_loop(Event::WindowEvent { window_id: window.id(), event: WindowEvent::RedrawRequested }, target)?;
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
                let world = game.app_mut().world_mut();
                world.resource_mut::<GameConfig>().video.display.fullscreen = false;
                world.send_event(tiny_glade::app_config::ApplyVideoSettingsCmd(1));
                self.phase_resize_events = self.os_resize_events;
                self.phase_generation = Some(generation);
                self.phase = 5;
                eprintln!("WINDOW_CYCLE actual fullscreen entered; returning to windowed");
            }
            5 if window.fullscreen().is_none() && self.os_resize_events > self.phase_resize_events
                && Some(generation) != self.phase_generation => {
                let size = window.inner_size();
                let config = game.app().world().resource::<FrameRenderConfig>();
                if config.extent_0 != [size.width, size.height] { return Ok(()); }
                anyhow::ensure!(config.extent_0 == Self::REQUESTED_SIZE,
                    "全屏返回没有恢复先前窗口尺寸：{config:?}");
                self.phase = 6;
                eprintln!("WINDOW_CYCLE all phases completed; generation={generation} actual_os_resizes={}", self.os_resize_events);
            }
            _ => {}
        }
        Ok(())
    }
}
