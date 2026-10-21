//! 默认验证加载图→原 presentation resolve→真实 WSI，使用显式加载 World 与时钟。
//! TG_VALIDATE_GAMEPLAY_WSI=1 则验证真实 GameLoop 的双 App 交接、非空游戏帧、
//! 原生 present 成功及 OS 关闭、保存线程和 GPU 退出；两种验收分别记结果。
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
    if std::env::var_os("TG_VALIDATE_GAMEPLAY_WSI").is_some_and(|value| value == "1") {
        return validate_gameplay_present();
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
    let mut frames = 0;
    let result = Rc::new(RefCell::new(None::<Result<()>>));
    let observed = result.clone();
    event_loop.run(move |event, target| {
        target.set_control_flow(ControlFlow::Poll);
        let step = match event {
            Event::AboutToWait => (|| -> Result<()> {
                driver.app_mut().world_mut().resource_mut::<bevy_time::Time<()>>()
                    .advance_by(Duration::from_secs_f64(1.0 / 60.0));
                driver.frame(Some(egui::RawInput::default()), &mut renderer, |_| {})?;
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
                renderer.resize([size.width, size.height])
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

/// 显式验收完整双 App 交接和游戏窗口；加载专用 fixture 仍由默认入口验证。
fn validate_gameplay_present() -> Result<()> {
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
    let config = get_app_config(&cli)?;
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
    let result = Rc::new(RefCell::new(None::<Result<()>>));
    let observed = result.clone();
    let started = std::time::Instant::now();
    let mut world_frames = 0u64;
    let mut loading_frames = 0u64;
    let mut native_before_world = None;
    let mut previous_receipt = None;
    let mut close_requested = false;
    event_loop.run(move |event, target| {
        let redraw = matches!(&event, Event::WindowEvent { event: WindowEvent::RedrawRequested, .. });
        let exiting = matches!(&event, Event::LoopExiting);
        let step = (|| -> Result<()> {
            let native_before_frame = game.native_present_success_count();
            let completed_before = game.frame_progress().completed_frames;
            game.main_loop(event, target)?;
            if redraw && game.frame_progress().completed_frames > completed_before {
                if let Some(frame) = game.frame_observation() {
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
                            anyhow::ensure!(frame.present_point.is_some() && frame.present_generation.is_some(),
                                "生产世界没有 WSI present ticket");
                            anyhow::ensure!(previous_receipt != Some(frame.receipt_frame), "重复统计同一游戏帧");
                            previous_receipt = Some(frame.receipt_frame);
                            native_before_world.get_or_insert(native_before_frame);
                            world_frames += 1;
                            eprintln!("GAMEPLAY_WSI world_frame={world_frames} receipt={} native_successes={} draws={:?}",
                                frame.receipt_frame, game.native_present_success_count(), frame.draw_list_lengths);
                        }
                        _ => anyhow::bail!("出现空世界或 diagnostic minimal 图"),
                    }
                }
                if world_frames >= 12 && !close_requested {
                    // 发送关闭窗口事件，由现有 OS→游戏→保存→GPU 关闭链处理。
                    close_requested = true;
                    game.main_loop(Event::WindowEvent {
                        window_id: window.id(), event: WindowEvent::CloseRequested,
                    }, target)?;
                }
            }
            if !exiting {
                anyhow::ensure!(started.elapsed() < Duration::from_secs(180),
                    "真实游戏窗口验收超时：loading={loading_frames}, world={world_frames}");
            } else if observed.borrow().is_none() {
                let native = game.native_present_success_count();
                let world = game.app().world();
                anyhow::ensure!(!world.contains_resource::<SaveThreadHandle>()
                    && world.get_resource::<AutoSaveState>().is_some_and(|state| !state.is_command_attached()),
                    "关闭后保存线程 owner 或 sender 仍然存在");
                anyhow::ensure!(loading_frames > 0 && world_frames >= 12 && close_requested,
                    "窗口提前退出：loading={loading_frames}, world={world_frames}");
                let world_native = native.checked_sub(native_before_world.context("没有游戏 present 起点")?)
                    .context("swapchain generation changed during bounded validation")?;
                anyhow::ensure!(world_native >= 12, "游戏只完成 {world_native} 次原生 present 成功");
                eprintln!("PASS: actual GameLoop loading={loading_frames}, world={world_frames}, world-native-successes={world_native}; OS close/save/GPU shutdown completed");
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
