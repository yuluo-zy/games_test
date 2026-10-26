//! Real original rocky-lake scene, Autumn water and Winter ice, through the
//! existing game App and production CoreRenderBridge. This is GPU correctness
//! evidence only; no WSI or clean frame-CPU acceptance is claimed.
//! Boot code is staged from the current original_gameplay_chain fixture. Once
//! the baseline is frozen, extract this common bootstrap into test support.
#[path = "support/performance_input.rs"]
mod surface_input;
#[path = "support/mod.rs"]
mod test_support;
#[path = "support/surface_nani_observation.rs"]
mod surface_nani_observation;
use anyhow::{Context, Result};
use clap::Parser;
use country_core::{
    resources::{
        session::{ActiveSession, GladeType},
        walls::InnerWalls,
        window::WindowSize,
    },
    systems::save::load_game::LoadGameManager,
};
use std::{path::PathBuf, time::Duration};
use test_support::BridgeTestDriver as _;
use tiny_glade::{
    app_config::{CliArgs, get_app_config},
    render_bridge::CoreRenderBridge,
    run_game::{ApplicationShaderLibrary, run_game_thread},
};

struct BootedSession {
    driver: tiny_glade::game_loop::GameFrameDriver,
    context: egui::Context,
    bridge: CoreRenderBridge,
    shaders: std::sync::Arc<country_core::asset_libraries::shader_library::AssetShaderLibrary>,
    device: std::sync::Arc<rhapsody::device::Device>,
    _boot: std::sync::MutexGuard<'static, ()>,
}

fn boot_surface_session(glade: GladeType) -> Result<BootedSession> {
    struct Diagnostics;
    impl log::Log for Diagnostics {
        fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
            metadata.level() <= log::Level::Error
        }
        fn log(&self, record: &log::Record<'_>) {
            if self.enabled(record.metadata()) {
                eprintln!("{} {}", record.target(), record.args());
            }
        }
        fn flush(&self) {}
    }
    static DIAGNOSTICS: Diagnostics = Diagnostics;
    if log::set_logger(&DIAGNOSTICS).is_ok() {
        log::set_max_level(log::LevelFilter::Error);
    }

    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        })
        .canonicalize()?;
    // Both shipped-session tests boot a whole App in this process: the account paths are process
    // global, so they are initialised once and the boots are serialised.
    static BOOT: std::sync::Mutex<()> = std::sync::Mutex::new(());
    static ACCOUNT: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();
    let boot_guard = BOOT
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if ACCOUNT.get().is_none() {
        let account = tempfile::tempdir()?;
        paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;
        let _ = ACCOUNT.set(account);
    }
    std::env::set_current_dir(&installed)?;
    utils::local_time::init_local_time();
    tiny_glade::load_language_library::load_language_library(None, None);
    let cli = CliArgs::parse_from(["gpu-surface-collect"]);
    let config = get_app_config(&cli)?;
    assert!(
        !config.dev.demo,
        "this acceptance requires the full original build"
    );
    let mut app = run_game_thread(config, &cli, WindowSize::new(1280, 720))?;
    // Choose metadata before the existing session/startup-water producers.
    // Changing only ActiveSession after boot would bind a water shader to the
    // Ice graph and would not test the real winter path.
    app.insert_resource(country_core::resources::session::NextSession::new_game(
        country_core::resources::session::SessionMeta {
            glade,
            ..Default::default()
        },
        Some(installed.join("assets/starting-builds/full/05-rocky-lake/history.json")),
    ));
    app.insert_resource(
        country_core::resources::platform::RuntimePlatform::from_detection(false, false),
    );
    tiny_glade::game_loop::register_platform_events(&mut app);
    tiny_glade::game_loop::register_settings_application_system(&mut app);
    let device = rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig {
        // Same budget as the production window path (the original allocator has no cap).
        validation: true,
        memory_budget_bytes: tiny_glade::production_renderer::UNBOUNDED_DEVICE_BUDGET_BYTES,
        ..rhapsody::device::DeviceConfig::headless()
    })
    .context("strict gameplay physical device")?;
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    let bridge = CoreRenderBridge::new(device.clone(), installed.clone(), shaders.clone())?;
    let mut final_post = None;
    country_core::startup::load_shaders::load_shaders(
        &shaders,
        bridge.shader_backend(),
        &mut final_post,
    );
    // Same shader set as ProductionRenderer::new: the world tail (ui_2d, text, vermeer,
    // painterly, blit) is part of the compiled production graph.
    country_core::startup::load_shaders::load_ui_shaders(&shaders, bridge.shader_backend());
    country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());
    struct TailShaders<'a> {
        library: &'a country_core::asset_libraries::shader_library::AssetShaderLibrary,
        backend: &'a dyn country_core::render::shader::boundary::ShaderBackend,
    }
    impl country_core::systems::render_loop::painterly::ShaderLoader for TailShaders<'_> {
        fn load_shader_into_library(&mut self, path: &'static str, name: &'static str) {
            country_core::startup::load_shaders::load_shader_into_library(
                self.library,
                self.backend,
                path,
                name,
            );
        }
    }
    country_core::systems::render_loop::painterly::startup_painterly(&mut TailShaders {
        library: &shaders,
        backend: bridge.shader_backend(),
    });
    country_core::systems::render_loop::vermeer::startup_vermeer(&mut TailShaders {
        library: &shaders,
        backend: bridge.shader_backend(),
    });
    bridge.install_asset_libraries(app.world_mut())?;
    tiny_glade::gameplay_startup::install_gameplay_device_runtime(
        &mut app,
        device.clone(),
        bridge.hosts().clone(),
        installed,
    )?;
    // The window path uses this driver to install the same asset maintenance
    // and canonical egui Context before its first App::update.
    let context = egui::Context::default();
    let mut driver = tiny_glade::game_loop::GameFrameDriver::new(app, context.clone());
    driver
        .app_mut()
        .insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(
            Duration::from_secs_f64(1.0 / 60.0),
        ));
    let mut loaded = false;
    for frame in 0..600 {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
        let world = driver.app().world();
        if frame % 60 == 0 {
            eprintln!(
                "gameplay App frame {frame}: session={}, walls={}",
                world.contains_resource::<ActiveSession>(),
                world.resource::<InnerWalls>().wall_ids().count()
            );
        }
        if world.contains_resource::<ActiveSession>()
            && world.resource::<LoadGameManager>().options.is_none()
            && world
                .get_resource::<country_core::resources::world_raster_defs::WaterRaster>()
                .is_some_and(|raster| raster.composite().iter().any(|&value| value > 127))
        {
            loaded = true;
            break;
        }
    }
    anyhow::ensure!(
        loaded,
        "shipped rocky-lake did not finish through the real gameplay schedule"
    );
    // Let actual asynchronous assets, constructor uploads and intro migration
    // settle through their normal schedules before asserting visible owners.
    for _ in 0..90 {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
    }
    Ok(BootedSession {
        driver,
        context,
        bridge,
        shaders,
        device,
        _boot: boot_guard,
    })
}

// Safe field serialization in the exact order of frame_setup::frame_bytes.
// Do not read Rust padding, round floats, or omit previous-camera/jitter/lighting
// inputs: all 0x1090 GPU constant bytes participate in strict paired evidence.
fn frame_constant_words(c: &country_core::render::frame_constants::FrameConstants) -> Vec<u32> {
    let mut words = Vec::<u32>::with_capacity(0x1090 / 4);
    for value in [
        c.sun_color,
        c.sky_low_color,
        c.sky_high_color,
        c.fog_tint_thickness,
        c.fake_light_color,
        c.fake_light_dir,
        c.fake_light2_color,
        c.fake_light2_dir,
    ] {
        words.extend(value.map(f32::to_bits));
    }
    words.extend([
        c.frame_counter,
        c.delta_time_seconds.to_bits(),
        c.ev_mult.to_bits(),
        c.glade_theme_id,
        c.inner_rendering_resolution[0],
        c.inner_rendering_resolution[1],
        c.game_time_sec,
        c.game_time_frac.to_bits(),
    ]);
    words.extend(c.editing_highlight_color.map(f32::to_bits));
    words.push(c.time_of_day.to_bits());
    words.extend(c.dir_to_sun.map(f32::to_bits));
    words.push(c.camera_idx);
    words.extend(c.dir_to_moon.map(f32::to_bits));
    words.push(c.render_overrides);
    words.extend([
        c.global_mip_bias.to_bits(),
        c.highlighted_transient_stair_edge_id,
        c.pad1,
        c.pad2,
    ]);
    words.extend(c.debug_tweakers.map(f32::to_bits));
    for v in &c.views {
        for m in [
            v.clip_from_view,
            v.view_from_clip,
            v.clip_from_view_jittered,
            v.view_from_clip_jittered,
            v.view_from_world,
            v.world_from_view,
            v.clip_from_world_jittered,
            v.prev_clip_from_clip,
            v.prev_clip_from_view,
            v.prev_view_from_clip,
            v.prev_view_from_world,
            v.prev_world_from_view,
        ] {
            words.extend(m.to_cols_array().map(f32::to_bits));
        }
        for v in [
            v.pixel_offset,
            v.jitter_ndc,
            v.prev_pixel_offset,
            v.prev_jitter_ndc,
        ] {
            words.extend(v.to_array().map(f32::to_bits));
        }
    }
    assert_eq!(
        words.len() * 4,
        0x1090,
        "native FrameConstants serialization"
    );
    words
}

fn run_surface(glade: GladeType, label: &str, local_pair: Option<&str>) -> Result<()> {
    use country_core::systems::render_loop::draw_passes::draw_lists::{self, DrawLists};
    let BootedSession {
        mut driver,
        context,
        mut bridge,
        shaders,
        device,
        _boot,
    } = boot_surface_session(glade)?;
    let world = driver.app_mut().world_mut();
    #[cfg(not(feature = "surface-compare-test"))]
    anyhow::ensure!(local_pair.is_none(), "same-prefix fixture requires surface-compare-test");
    #[cfg(feature = "surface-compare-test")]
    let local_modes = local_pair.map(|pair| {
        use tiny_glade::world_surface::comparison::Mode::{Inline,Collect};
        match pair {
            "inline-collect" => Ok([Inline,Collect]),
            "collect-inline" => Ok([Collect,Inline]),
            "inline-inline" => Ok([Inline,Inline]),
            "collect-collect" => Ok([Collect,Collect]),
            _ => Err(anyhow::anyhow!("unknown local surface pair {pair}")),
        }
    }).transpose()?;
    #[cfg(feature = "surface-compare-test")]
    if let Some(modes) = local_modes {
        anyhow::ensure!(std::env::var("TG_SURFACE_MODE").unwrap_or_else(|_|"collect".into())=="collect",
            "local diagnostic keeps Collect for all unobserved frames");
        world.insert_resource(tiny_glade::world_surface::comparison::SurfaceComparison::new(modes));
    }
    anyhow::ensure!(
        world.resource::<ActiveSession>().meta.glade == glade,
        "wrong session season"
    );
    let config = *world.resource::<country_core::render::render_config::FrameRenderConfig>();
    anyhow::ensure!(
        config.extent_0 == [1280, 720]
            && config.extent_8 == [1280, 720]
            && config.world_resolution_scale == 1.0,
        "surface acceptance must keep full world scale: {config:?}"
    );
    let water = world.resource::<country_core::resources::world_raster_defs::WaterRaster>();
    let wet_pixels = water
        .composite()
        .iter()
        .filter(|&&value| value > 127)
        .count();
    anyhow::ensure!(
        wet_pixels > 0,
        "lake scene has no actual water raster contents"
    );
    let raster_signature = blake3::hash(water.composite()).to_hex().to_string();
    let mut lists = DrawLists::new();
    draw_lists::build_draw_lists(world, shaders.as_ref(), &mut lists);
    anyhow::ensure!(!lists.list(15).is_empty(), "no surface draws admitted");
    let shader_name = if glade.is_winter() {
        "ice_plane"
    } else {
        "water_plane"
    };
    anyhow::ensure!(
        shaders.get_raster_handle_by_name(shader_name).is_some(),
        "missing original {shader_name}"
    );
    let list_lengths: Vec<_> = (0..draw_lists::DRAW_LIST_COUNT)
        .map(|index| lists.list(index).len())
        .collect();
    let initial_scene_input = surface_input::scene_manifest(world, &list_lengths)?;
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)?;
    bridge.accept_gameplay_warmup(
        world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>(),
    )?;
    let captured: std::sync::Arc<parking_lot::Mutex<Vec<(usize, u32, u32, Vec<u8>)>>> =
        Default::default();
    let mut commands = Vec::new();
    let mut expected_surface_draws = 0u64;
    #[cfg(feature = "surface-compare-test")]
    let mut local_observed_draws = 0u64;
    let mut final_nani_observation = None;
    let _ = country_core::render::declare_probe::take();
    let _ = country_core::systems::render_loop::draw_passes::transparency_pass::surface_support::take_surface_collect_test_stats();
    for index in 0..30 {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
        while bridge.with_session(|session| session.pending_frame_count())
            >= device.frames_in_flight() as usize
        {
            anyhow::ensure!(
                bridge.with_session(|session| session.wait_gpu_queue_trim(120_000_000_000))?,
                "GPU wait timeout"
            );
            anyhow::ensure!(
                bridge.collect_completed_frames()? > 0,
                "GPU completion was not retired"
            );
        }
        let world = driver.app_mut().world_mut();
        let actual_config = world.resource::<country_core::render::render_config::FrameRenderConfig>();
        anyhow::ensure!(actual_config == &config,
            "surface workload changed rendering resolution/config at frame {index}");
        let actual_water = world.resource::<country_core::resources::world_raster_defs::WaterRaster>();
        anyhow::ensure!(blake3::hash(actual_water.composite()).to_hex().as_str() == raster_signature,
            "static lake water raster changed at frame {index}");
        let constants = world.resource::<country_core::render::frame_constants::FrameConstants>();
        let mut frame_input = serde_json::json!({
            "frame_constant_words":frame_constant_words(constants),
            "frame_counter":constants.frame_counter,
            "game_time_sec":constants.game_time_sec,
            "game_time_frac_bits":constants.game_time_frac.to_bits(),
            "delta_time_bits":constants.delta_time_seconds.to_bits(),
            "inner_resolution":constants.inner_rendering_resolution,
            "render_overrides":constants.render_overrides,
            "time_of_day_bits":constants.time_of_day.to_bits(),
            "camera_bits":world.resource::<country_core::systems::camera_rig::MainCamera>()
                .read_matrix(0xc0).to_cols_array().map(f32::to_bits),
        });
        // Observe canonical resources without claiming to have decoded the
        // eventual native per-pass constant payload. The host consumes these
        // exact video/config fields when declaring TAA or TGSR.
        let video = &world.resource::<country_core::game_config::GameConfig>().video;
        frame_input["temporal_source_resources"] = serde_json::json!({
            "phase":"after_App_update_before_prepare",
            "upscaling_mode":video.upscaling_mode,
            "sharpen_amount_bits":video.sharpen.to_bits(),
            "bypass_clamp_source":u8::from(actual_config.flag_20),
            "native_uniform_payload_observed":false,
        });
        #[cfg(feature = "surface-compare-test")]
        if local_modes.is_some() {
            // Original resolution publication aligns the physical inner target
            // to32: nominal1280x720 at scale1 is physically1280x736. Preserve all
            // native rows and bind readback to this actual config/constants.
            let aligned=actual_config.extent_0.map(|axis|((axis+31)&!31).clamp(32,10_912));
            anyhow::ensure!(actual_config.extent_10==aligned
                && constants.inner_rendering_resolution==actual_config.extent_10
                && actual_config.world_resolution_scale.to_bits()==1.0f32.to_bits()
                && actual_config.tgsr_resolution_scale.to_bits()==1.0f32.to_bits()
                && video.upscaling_mode==country_core::render::render_config::UpscalingMode::Off,
                "local fixture resolution differs from original scale1/alignment32 path");
            frame_input["resolution_source"] = serde_json::json!({
                "nominal_extent":actual_config.extent_0,"effective_output_extent":actual_config.extent_8,
                "physical_inner_extent":actual_config.extent_10,
                "world_scale_bits":actual_config.world_resolution_scale.to_bits(),
                "tgsr_scale_bits":actual_config.tgsr_resolution_scale.to_bits(),
                "phase":"after_App_update_before_prepare","alignment":32,
            });
        }
        let mut actual_lists = DrawLists::new();
        draw_lists::build_draw_lists(world, shaders.as_ref(), &mut actual_lists);
        let actual_lengths: Vec<_> = (0..draw_lists::DRAW_LIST_COUNT)
            .map(|slot| actual_lists.list(slot).len())
            .collect();
        anyhow::ensure!(actual_lengths[15] > 0,
            "surface draw list became empty at frame {index}");
        expected_surface_draws += actual_lengths[15] as u64;
        #[cfg(feature = "surface-compare-test")]
        if local_modes.is_some() && [19,29].contains(&index) {
            local_observed_draws += actual_lengths[15] as u64;
            world.resource_mut::<tiny_glade::world_surface::comparison::SurfaceComparison>().arm((index+1) as u32)?;
        }
        frame_input["water_raster_blake3"] = serde_json::json!(raster_signature);
        frame_input["scene"] = surface_input::scene_manifest(world, &actual_lengths)?;
        if [19, 29].contains(&index) {
            let sink = captured.clone();
            tiny_glade::screenshot::gather_screenshot_requests(
                world,
                [tiny_glade::screenshot::ScreenshotRequest {
                    purpose: tiny_glade::screenshot::ScreenshotPurpose::MenuBackground,
                    dimensions: None,
                    sink: tiny_glade::screenshot::ScreenshotSink::Rgba(std::sync::Arc::new(
                        move |w, h, rgba| sink.lock().push((index, w, h, rgba)),
                    )),
                    completion: None,
                }],
            )?;
        }
        let frame =
            tiny_glade::production_renderer::prepare_headless_gameplay_frame(&mut bridge, world)?;
        // build_world publishes this observation after choosing the actual
        // color history pair. Read after this frame's declaration, not before
        // prepare (which would observe the previous frame's flag).
        let color_history_valid = world
            .get_resource::<tiny_glade::world_graph::WorldHistoryObservation>()
            .context("world graph did not publish its history observation")?
            .color_history_valid;
        frame_input["temporal_declaration_observation"] = serde_json::json!({
            "phase":"after_prepare_before_execute",
            "color_history_valid":color_history_valid,
            "native_uniform_payload_observed":false,
            "gpu_history_contents_observed":false,
            "ice_resolve_valid_observed":false,
        });
        bridge.prepare_test_execution()?;
        let frame = frame.execute(&mut bridge, world)?;
        #[cfg(feature = "surface-compare-test")]
        if local_modes.is_some() {
            world.resource_mut::<tiny_glade::world_surface::comparison::SurfaceComparison>()
                .confirm_submission(&mut bridge,&frame.receipt)?;
        }
        anyhow::ensure!(frame.receipt.stats.chunks > 0, "no actual recording tasks");
        if index == 29 {
            final_nani_observation = Some(surface_nani_observation::retain_final(
                world, &mut bridge, &frame.receipt));
        }
        world
            .resource_mut::<country_core::render::RenderTemporalState>()
            .frame_render_returned();
        tiny_glade::screenshot::finish_screenshot_capture(world)?;
        // PGO iterates a name-keyed hash table, not execution order. Preserve
        // its actual invocation counts and canonicalize only this inventory;
        // never use its table order as command-order correctness evidence.
        let mut pass_call_counts = bridge.with_session(|session| {
            session.chain.pgo.previous.iter()
                .map(|(name, _, count)| (name.to_owned(), count))
                .collect::<Vec<_>>()
        });
        pass_call_counts.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        let pass_names: Vec<_> = pass_call_counts.iter()
            .map(|(name, _)| name.clone()).collect();
        let expected_pass = if glade.is_winter() {
            "Ice plane"
        } else {
            "Water plane"
        };
        anyhow::ensure!(
            pass_names.iter().any(|name| name == expected_pass),
            "{expected_pass} never executed"
        );
        commands
            .push(serde_json::json!({"frame":index,"pass_names":pass_names,
                "pass_call_counts":pass_call_counts,
                "retained_pass_order":frame.receipt.snapshot.pass_order.clone(),
                "input":frame_input}));
        bridge.collect_completed_frames()?;
    }
    let world = driver.app_mut().world_mut();
    for _ in 0..8 {
        tiny_glade::screenshot::shared_texture_capture::finish(world)?;
    }
    tiny_glade::screenshot::shared_texture_capture::shutdown(world)?;
    bridge.wait_submits_done(120_000_000_000)?;
    // This diagnostic reuses the existing drain. It never submits copy work,
    // changes native usage/memory type, or waits beyond that completed interval.
    let final_nani_observation = final_nani_observation
        .context("final successful main-frame receipt was not observed")?
        .read_after_existing_drain();
    let collection = country_core::render::declare_probe::take();
    let surface = country_core::systems::render_loop::draw_passes::transparency_pass::surface_support::take_surface_collect_test_stats();
    let mode = std::env::var("TG_SURFACE_MODE").unwrap_or_else(|_| "collect".into());
    #[cfg(feature = "surface-compare-test")]
    let local_active = local_modes.is_some();
    #[cfg(not(feature = "surface-compare-test"))]
    let local_active = false;
    #[cfg(feature = "surface-compare-test")]
    if let Some(modes) = local_modes {
        use tiny_glade::world_surface::comparison::Mode;
        let inline_branches=modes.into_iter().filter(|m|matches!(m,Mode::Inline)).count() as u64;
        let expected_inline_calls=inline_branches*2;
        let expected_worker_calls=28+(2-inline_branches)*2;
        anyhow::ensure!(surface.inline_calls==expected_inline_calls
            && surface.inline_draws==inline_branches*local_observed_draws
            && surface.worker_calls==expected_worker_calls
            && surface.worker_draws==expected_surface_draws-local_observed_draws+(2-inline_branches)*local_observed_draws
            && surface.worker_tasks>0 && surface.owner_collect_tasks==0,
            "local fixture did not run real inline/worker loops: {surface:?}");
    }
    if !local_active && mode == "inline" {
        anyhow::ensure!(
            surface.inline_calls == 30
                && surface.inline_draws == expected_surface_draws
                && surface.worker_calls == 0
                && surface.worker_tasks == 0,
            "inline control did not execute the original owner loop: {surface:?}"
        );
    } else if !local_active {
        anyhow::ensure!(
            surface.worker_calls == 30
                && surface.worker_tasks > 0
                && surface.worker_draws == expected_surface_draws
                && surface.inline_calls == 0
                && surface.owner_collect_tasks == 0,
            "Collect did not run on actual worker threads: {surface:?}"
        );
    }
    if std::env::var("TG_FRAME_TIMING").is_ok_and(|value| value == "1") {
        anyhow::ensure!(
            collection.collect_tasks > 0,
            "no worker Collect tasks observed"
        );
    }
    let shots = captured.lock();
    anyhow::ensure!(
        shots.len() == 2,
        "expected exactly two completed captures, got {}",
        shots.len()
    );
    let captured_frames: std::collections::BTreeSet<_> =
        shots.iter().map(|(index, _, _, _)| *index).collect();
    anyhow::ensure!(
        captured_frames == [19usize, 29].into_iter().collect::<std::collections::BTreeSet<_>>(),
        "duplicate or missing capture completion"
    );
    let out = std::env::var_os("TG_SURFACE_OUT")
        .context("TG_SURFACE_OUT must preserve raw pixel evidence")?;
    let out = PathBuf::from(out).join(label);
    std::fs::create_dir_all(&out)?;
    #[cfg(feature = "surface-compare-test")]
    let local_comparison = if local_active {
        Some(world.resource::<tiny_glade::world_surface::comparison::SurfaceComparison>()
            .write_after_existing_drain(&out.join("same-prefix"))?)
    } else {None};
    let mut pixels = Vec::new();
    for (index, w, h, rgba) in shots.iter() {
        anyhow::ensure!(
            [*w, *h] == [1280, 720] && rgba.len() == (w * h * 4) as usize,
            "wrong capture dimensions"
        );
        anyhow::ensure!(
            rgba.chunks_exact(4).any(|pixel| pixel[..3] != [0, 0, 0]),
            "empty captured world"
        );
        let name = format!("frame-{:02}.rgba", index + 1);
        std::fs::write(out.join(&name), rgba)?;
        tiny_glade::screenshot::write_image_with_callback_impl(
            &out.join(format!("frame-{:02}.png", index + 1)),
            *w,
            *h,
            rgba,
            90,
            &|result| assert!(result.is_ok(), "{result:?}"),
        );
        pixels.push(serde_json::json!({"frame":index+1,"width":w,"height":h,
            "file":name,"blake3":blake3::hash(rgba).to_hex().to_string()}));
    }
    let manifest = serde_json::json!({
        "schema":if local_active {"surface-same-prefix-fixture-v1"} else {"surface-collect-correctness-v1"}, "season":label,
        "local_surface_comparison_enabled":local_active,"local_surface_pair":local_pair,
        "pass_names_semantics":"sorted PGO name inventory; pass_call_counts preserves invocations; neither field is command execution order",
        "frame_constants_semantics":"all 0x1090 bytes in native layout as 1060 u32 bit patterns; no padding reads or value overrides",
        "retained_pass_order_semantics":"actual receipt.snapshot retained graph plan order, including repeated names; not GPU wall completion order",
        "temporal_observation_coverage":{
            "observed":"canonical sharpen/upscaling/bypass source resources and declared color history validity",
            "not_observed":["native per-pass uniform payload", "ice refr/ray-length pair flags and resolve_valid", "GPU history pixel contents"],
        },
        "surface_execution_mode":mode,
        "source":std::env::var("TG_SURFACE_SOURCE").unwrap_or_else(|_| "unrecorded".into()),
        "binary_blake3":blake3::hash(&std::fs::read(std::env::current_exe()?)?).to_hex().to_string(),
        "scene":"assets/starting-builds/full/05-rocky-lake/history.json",
        "expected_original_scene_sha256":"f85cba25b4a193d517402a57df3590323f3d388b4f8181899c4b7fed7272fec3",
        "scene_blake3":blake3::hash(&std::fs::read("assets/starting-builds/full/05-rocky-lake/history.json")?).to_hex().to_string(),
        "water_raster_blake3":raster_signature,"wet_raster_pixels":wet_pixels,
        "initial_draw_list_lengths":list_lengths,"frames":30,
        "initial_scene_input":initial_scene_input,
        "fixed_step":[1,60],"world_scale":1.0,"pixels":pixels,"commands":commands,
        "final_nani_observation":final_nani_observation,
        "collect_tasks":collection.collect_tasks,"cross_pass_tasks":collection.cross_pass_tasks,
        "actual_surface_execution":{
            "inline_calls":surface.inline_calls,"inline_draws":surface.inline_draws,
            "worker_calls":surface.worker_calls,"worker_tasks":surface.worker_tasks,
            "worker_draws":surface.worker_draws,"owner_collect_tasks":surface.owner_collect_tasks,
        },
        "scope":"physical headless GPU correctness; visible surface ROI and matching-source A/B remain required"
    });
    std::fs::write(
        out.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;
    #[cfg(feature = "surface-compare-test")]
    if let Some(report) = &local_comparison {
        anyhow::ensure!(report["passes"]==true,
            "same-prefix local surface comparison failed; raw evidence and local-comparison.json retained");
    }
    let drained = bridge
        .shader_backend()
        .bindless_occupancy()
        .tables
        .map(|table| table.live);
    anyhow::ensure!(
        drained == bridge.shader_backend().retained_bindless_counts(),
        "descriptor owners leaked"
    );
    Ok(())
}

#[test]
fn real_lake_water_and_ice_record_collect_and_preserve_raw_pixel_evidence() -> Result<()> {
    std::thread::Builder::new().name("tg-render".into()).spawn(|| {
        let mode = std::env::var("TG_SURFACE_MODE").unwrap_or_else(|_| "collect".into());
        anyhow::ensure!(mode == "inline" || mode == "collect", "unknown surface mode {mode}");
        country_core::systems::render_loop::draw_passes::transparency_pass::surface_support::with_surface_collect_mode_for_test(mode == "inline", || {
            run_surface(GladeType::Autumn,"water",None)?;
            run_surface(GladeType::Winter,"ice",None)
        })
    })?.join().unwrap_or_else(|panic| std::panic::resume_unwind(panic))
}

#[test]
#[cfg(feature = "surface-compare-test")]
fn real_lake_surface_same_prefix_zero_tolerance() -> Result<()> {
    let pair=std::env::var("TG_SURFACE_LOCAL_PAIR").context("TG_SURFACE_LOCAL_PAIR must explicitly select inline-inline/collect-collect/inline-collect/collect-inline")?;
    std::thread::Builder::new().name("tg-render".into()).spawn(move|| {
        country_core::systems::render_loop::draw_passes::transparency_pass::surface_support::with_surface_collect_mode_for_test(false,|| {
            // Preserve both seasons' evidence even if the first comparison fails.
            let water=run_surface(GladeType::Autumn,"water",Some(&pair));
            let ice=run_surface(GladeType::Winter,"ice",Some(&pair));
            water?;ice
        })
    })?.join().unwrap_or_else(|panic|std::panic::resume_unwind(panic))
}
