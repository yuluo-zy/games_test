//! Actual original-intro App -> async startup -> GameFrameDriver -> world WSI.
//! This executable uses the production GameLoop, not a substitute RenderHost.
use std::{cell::RefCell, path::PathBuf, rc::Rc, time::{Duration, Instant}};
use anyhow::{Context, Result};
use clap::Parser;
use country_core::{resources::{session::ActiveSession, walls::InnerWalls, window::WindowSize},
    systems::save::load_game::LoadGameManager};
use tiny_glade::{app_config::{CliArgs, get_app_config},
    game_loop::{GameLoop, GameLoopOptions}, run_game::{ApplicationShaderLibrary, run_game_thread}};
use winit::{event::{Event, WindowEvent}, event_loop::EventLoop, window::WindowBuilder};

const WALL_TIMEOUT: Duration = Duration::from_secs(600);
const MAX_REDRAWS: u64 = 3600;
const REQUIRED_COMPLETED_WORLD_PRESENTS: u64 = 3;

fn main() -> Result<()> {
    let started = Instant::now();
    anyhow::ensure!(std::env::var("TG_ALLOW_MINIMAL_WORLD_FRAME").as_deref() != Ok("1"),
        "nonempty gameplay acceptance forbids the minimal-world diagnostic");
    let installed = std::env::var_os("TG_INSTALL_ROOT").map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../ljxsj_92385/Tiny Glade")).canonicalize()?;
    let account = tempfile::tempdir()?;
    paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;
    std::env::set_current_dir(&installed)?;
    utils::local_time::init_local_time();
    tiny_glade::load_language_library::load_language_library(None, None);
    let cli = CliArgs::parse_from(["original-gameplay-present"]);
    let config = get_app_config(&cli)?;
    anyhow::ensure!(!config.dev.demo && !config.dev.skip_splash,
        "acceptance requires full original intro and the real async startup/handoff");
    let mut app = run_game_thread(config, &cli, WindowSize::new(480, 270))?;
    app.insert_resource(country_core::resources::platform::RuntimePlatform::from_detection(false, false));
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    let event_loop = EventLoop::new()?;
    let window = std::sync::Arc::new(WindowBuilder::new()
        .with_title("Tiny Glade — actual nonempty gameplay WSI acceptance")
        .with_inner_size(winit::dpi::LogicalSize::new(480.0, 270.0))
        .with_visible(false).build(&event_loop)?);
    // GameLoop::new owns the production renderer/device, installs the exact
    // device runtime, and runs the original two-App preparation/handoff. It
    // neither removes systems nor synthesizes LoadingGameComplete/Reveal.
    let mut game = GameLoop::new(app, window, GameLoopOptions {
        compiled_shader_root: installed.join("compiled-shaders"),
        frame_limit: None, input_recording: None,
    }, shaders)?;
    game.enable_world_present_completion_tracking();
    let result = Rc::new(RefCell::new(None::<Result<()>>));
    let observed = result.clone();
    let mut initialized_monitors = false;
    let mut redraws = 0u64;
    let mut first_ready_frame = None;
    let mut completion_baseline = None;
    let mut verified = false;
    event_loop.run(move |event, target| {
        // Keep the isolated account until both Apps, audio and renderer have
        // completed shutdown; never direct the test at the player's account.
        let _account_lease = &account;
        let is_exit = matches!(&event, Event::LoopExiting);
        let is_redraw = matches!(&event, Event::WindowEvent { event: WindowEvent::RedrawRequested, .. });
        let premature_close = matches!(&event, Event::WindowEvent { event: WindowEvent::CloseRequested, .. }) && !verified;
        let step = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<()> {
            if !initialized_monitors {
                // GameLoop::run normally initializes this before its first
                // callback. Reuse the same public production OS consumer.
                game.on_os_event(tiny_glade::event_loop::OsWindowEvent::MonitorChanged, target)?;
                initialized_monitors = true;
            }
            // The real callback executes GameFrameDriver::frame, actual egui
            // platform output, compile/execute/acquire/display/present, and
            // forwards LoopExiting through the production shutdown protocol.
            game.main_loop(event, target)?;
            if is_exit { return Ok(()); }
            anyhow::ensure!(!premature_close, "window closed before gameplay acceptance");
            anyhow::ensure!(started.elapsed() < WALL_TIMEOUT,
                "gameplay WSI acceptance exceeded {:?} after {redraws} redraws", WALL_TIMEOUT);
            if !is_redraw || verified { return Ok(()); }
            redraws += 1;
            anyhow::ensure!(redraws <= MAX_REDRAWS, "intro did not reach completed nonempty WSI in {MAX_REDRAWS} redraws");
            let world = game.app().world();
            let loaded = world.contains_resource::<ActiveSession>()
                && world.get_resource::<LoadGameManager>().is_some_and(|load| load.options.is_none())
                && world.contains_resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>()
                && world.get_resource::<InnerWalls>().is_some_and(|walls| walls.wall_ids().count() == 3);
            if redraws % 60 == 0 {
                eprintln!("actual WSI redraw {redraws}: gameplay_ready={loaded}, completed_world_presents={}",
                    game.renderer().completed_world_presents());
            }
            if !loaded { return Ok(()); }
            let frame_index = world.resource::<country_core::render::RenderTemporalState>().frame_index;
            let buffers = game.renderer().bridge().hosts().buffers.clone();
            let shaders = world.resource::<ApplicationShaderLibrary>().0.clone();
            assert_nonempty_original_walls(game.app_mut().world_mut(), &shaders, &buffers);
            let initial_frame = *first_ready_frame.get_or_insert(frame_index);
            let baseline = *completion_baseline.get_or_insert(game.renderer().completed_world_presents());
            let completed = game.renderer().completed_world_presents().saturating_sub(baseline);
            if completed >= REQUIRED_COMPLETED_WORLD_PRESENTS {
                anyhow::ensure!(frame_index.wrapping_sub(initial_frame) >= REQUIRED_COMPLETED_WORLD_PRESENTS as u32,
                    "world presentation completions did not span normal gameplay frame returns");
                verified = true;
                eprintln!("PASS: original intro, three authoritative walls, actual brick/VAO owners, {completed} completed gameplay WSI presents across normal frames");
                target.exit(); // LoopExiting must still complete production shutdown.
            }
            Ok(())
        })).unwrap_or_else(|panic| {
            let text = panic.downcast_ref::<String>().map(String::as_str)
                .or_else(|| panic.downcast_ref::<&str>().copied()).unwrap_or("non-string panic");
            Err(anyhow::anyhow!("gameplay WSI acceptance panicked after {redraws} redraws: {text}"))
        });
        if let Err(error) = step {
            let mut outcome = observed.borrow_mut();
            if outcome.is_none() { *outcome = Some(Err(error)); }
            target.exit();
        } else if is_exit {
            let mut outcome = observed.borrow_mut();
            if outcome.is_none() {
                *outcome = Some(if verified { Ok(()) } else {
                    Err(anyhow::anyhow!("event loop exited before nonempty gameplay and WSI completion"))
                });
            }
        }
    })?;
    let outcome = result.borrow_mut().take().context("event loop returned without shutdown outcome")?;
    outcome
}

// NONEMPTY_HELPER_INSERTION_POINT
