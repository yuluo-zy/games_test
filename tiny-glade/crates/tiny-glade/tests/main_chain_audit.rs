//! Main-chain audit: the same real gameplay App, shipped initial session and
//! production device tuple as `original_gameplay_chain`, but with a recording
//! global error handler instead of the default panic.
//!
//! Bevy 0.16 reports a system whose parameter validation fails (missing
//! resource, missing event queue, ...) through the global error handler. The
//! default handler panics on the first one, which hides every later gap. This
//! test keeps running and prints each distinct `(system, error)` once, so the
//! complete list of unwired producers is visible from a single run.
//!
//! It does not weaken any system: failed systems are *reported*, never
//! replaced, and the test fails when the list is not empty.
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{Mutex, OnceLock},
    time::Duration,
};

use anyhow::{Context, Result};
use bevy_ecs::error::{BevyError, ErrorContext};
use clap::Parser;
use country_core::resources::window::WindowSize;
use tiny_glade::{
    app_config::{CliArgs, get_app_config},
    render_bridge::CoreRenderBridge,
    run_game::{ApplicationShaderLibrary, run_game_thread},
};

static ERRORS: OnceLock<Mutex<BTreeMap<String, (u32, u32)>>> = OnceLock::new();
static FRAME: Mutex<u32> = Mutex::new(0);

fn record(error: BevyError, ctx: ErrorContext) {
    let frame = *FRAME.lock().unwrap();
    let key = format!("{} :: {}", ctx.name(), error);
    let mut map = ERRORS.get_or_init(Default::default).lock().unwrap();
    let entry = map.entry(key).or_insert((frame, 0));
    entry.1 += 1;
}

#[test]
fn every_system_runs_on_the_real_gameplay_app() -> Result<()> {
    bevy_ecs::error::GLOBAL_ERROR_HANDLER
        .set(record)
        .map_err(|_| anyhow::anyhow!("global error handler already set"))?;
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        })
        .canonicalize()?;
    let frames: u32 = std::env::var("TG_AUDIT_FRAMES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(240);
    let account = tempfile::tempdir()?;
    paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;
    std::env::set_current_dir(&installed)?;
    utils::local_time::init_local_time();
    tiny_glade::load_language_library::load_language_library(None, None);
    let cli = CliArgs::parse_from(["main-chain-audit"]);
    let config = get_app_config(&cli)?;
    let mut app = run_game_thread(config, &cli, WindowSize::new(480, 270))?;
    app.insert_resource(
        country_core::resources::platform::RuntimePlatform::from_detection(false, false),
    );
    tiny_glade::game_loop::register_platform_events(&mut app);
    tiny_glade::game_loop::register_settings_application_system(&mut app);
    let device =
        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig {
            // Same unbounded budget as the production window path (the original has no cap).
            memory_budget_bytes: tiny_glade::production_renderer::UNBOUNDED_DEVICE_BUDGET_BYTES,
            ..rhapsody::device::DeviceConfig::headless()
        })
            .context("gameplay physical device")?;
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    let bridge = CoreRenderBridge::new(device.clone(), installed.clone(), shaders.clone())?;
    let mut final_post = None;
    country_core::startup::load_shaders::load_shaders(
        &shaders,
        bridge.shader_backend(),
        &mut final_post,
    );
    bridge.install_asset_libraries(app.world_mut())?;
    tiny_glade::gameplay_startup::install_gameplay_device_runtime(
        &mut app,
        device,
        bridge.hosts().clone(),
        installed,
    )?;
    let context = egui::Context::default();
    let mut driver = tiny_glade::game_loop::GameFrameDriver::new(app, context.clone());
    driver
        .app_mut()
        .insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(
            Duration::from_secs_f64(1.0 / 60.0),
        ));
    for frame in 0..frames {
        *FRAME.lock().unwrap() = frame;
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
    }
    let map = ERRORS.get_or_init(Default::default).lock().unwrap();
    for (key, (first, count)) in map.iter() {
        eprintln!("AUDIT first_frame={first} count={count} {key}");
    }
    eprintln!("AUDIT distinct failures: {}", map.len());
    anyhow::ensure!(map.is_empty(), "{} systems failed on the real App", map.len());
    Ok(())
}
