//! Opt-in diagnostic over the existing shipped-session renderer, without WSI.
use super::*;
use serde_json::json;
use tiny_glade::cpu_clock as clock;
#[cfg(feature = "cpu-probe")]
use tiny_glade::render_cpu_probe::{self as probe, CpuProbeConfig, CpuProbeSession};

// Keep report-row types explicit when feature-off runs leave this vector empty.
type Row = (
    u64, u64, u64, [u64; 9], Option<[u64; 13]>,
    Option<country_core::render::declare_probe::DeclareProbe>,
);

#[cfg(feature = "mimalloc")]
#[global_allocator]
static ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[path = "../support/performance_input.rs"]
mod input;
#[path = "../support/performance_workload.rs"]
mod workload;
#[path = "../support/performance_sparse_layout.rs"]
mod sparse_layout;
#[cfg(feature = "cpu-probe")]
mod workload_probe;
#[cfg(feature = "cpu-probe")]
mod gpu_diagnostic;
#[cfg(feature = "cpu-probe")]
#[path = "../support/performance_sparse_ranges.rs"]
mod sparse_ranges;
#[cfg(feature = "cpu-probe")]
type WallDispatchSourceRow = (u64, u64, u64, sparse_ranges::WallDispatchSnapshot);

#[test]
#[ignore = "explicit frozen-binary CPU/ETW run; requires TG_CPU_PROBE_OUT"]
fn profile_shipped_session_cpu() -> Result<()> {
    std::thread::Builder::new().name("tg-render".into()).spawn(run)?.join()
        .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
}

fn number(name: &str, default: usize) -> Result<usize> {
    std::env::var(name).ok().map(|s| s.parse().map_err(Into::into)).unwrap_or(Ok(default))
}

// Session endpoints only: keep the legacy [QPC, owner CPU, process CPU]
// envelope and publish each OS API's uncertainty window without correcting it.
fn read_cpu_envelope() -> Result<([u64; 3], [[u64; 2]; 2])> {
    let qpc = clock::wall_ticks();
    let owner_before = clock::wall_ticks();
    let owner_cpu = clock::read_thread_cpu_ns().context("owner CPU clock failed")?;
    let owner_after = clock::wall_ticks();
    let process_before = clock::wall_ticks();
    let process_cpu = clock::process_cpu_ns().context("process CPU clock failed")?;
    let process_after = clock::wall_ticks();
    Ok(([qpc, owner_cpu, process_cpu],
        [[owner_before, owner_after], [process_before, process_after]]))
}

fn run() -> Result<()> {
    let output = PathBuf::from(std::env::var_os("TG_CPU_PROBE_OUT").context("TG_CPU_PROBE_OUT required")?);
    let warmup = number("TG_CPU_PROBE_WARMUP", 600)?;
    let samples = number("TG_CPU_PROBE_SAMPLES", 3000)?;
    anyhow::ensure!(warmup >= 1 && samples > 0 && samples <= 100_000, "invalid measurement bounds");
    let mode = std::env::var("TG_CPU_PROBE_MODE")
        .unwrap_or_else(|_| if cfg!(feature = "cpu-probe") { "on" } else { "off" }.into());
    anyhow::ensure!(mode == "on" || mode == "off", "unknown probe mode");
    let canonical_wall_layout = match std::env::var("TG_PERF_CANONICAL_WALL_LAYOUT").as_deref() {
        Err(std::env::VarError::NotPresent) | Ok("0") => false,
        Ok("1") => true,
        _ => anyhow::bail!("TG_PERF_CANONICAL_WALL_LAYOUT must be absent, 0 or 1"),
    };
    let bill = std::env::var("TG_CPU_BILL").is_ok_and(|s| s == "1");
    anyhow::ensure!(!bill || mode == "on", "diagnostic bill requires active probe mode");
    anyhow::ensure!(cfg!(feature = "cpu-probe") || mode == "off",
        "this binary was compiled without cpu-probe; only TG_CPU_PROBE_MODE=off is supported");
    anyhow::ensure!(cfg!(feature = "cpu-probe") || !bill,
        "TG_CPU_BILL requires a binary compiled with cpu-probe");
    // Source scans allocate and hash only in this explicit diagnostic. They
    // are neither native-dispatch evidence nor clean CPU acceptance samples.
    let wall_source_trace = std::env::var("TG_WALL_DISPATCH_SOURCE_TRACE").is_ok_and(|s| s == "1");
    let wall_source_diagnostic = bill || wall_source_trace;
    anyhow::ensure!(!wall_source_diagnostic || (cfg!(feature = "cpu-probe") && mode == "on"),
        "Wall source diagnostic requires cpu-probe/on");
    let gpu_diagnostic = std::env::var("TG_GPU_DIAGNOSTIC").is_ok_and(|s| s == "1");
    let gpu_pass_spans = std::env::var("TG_GPU_PASS_SPANS").is_ok_and(|s| s == "1");
    let gpu_isolate = std::env::var("TG_GPU_ISOLATE").is_ok_and(|s| s == "1");
    let gpu_statistics = std::env::var("TG_GPU_STATS").is_ok_and(|s| s == "1");
    anyhow::ensure!(!gpu_diagnostic || !wall_source_diagnostic,
        "GPU timing and allocating Wall source diagnostics must run separately");
    anyhow::ensure!(!gpu_pass_spans || gpu_diagnostic,
        "GPU pass spans require the separate GPU diagnostic");
    anyhow::ensure!(!gpu_isolate && !gpu_statistics || gpu_pass_spans,
        "GPU pass isolation/statistics require pass spans");
    anyhow::ensure!(!gpu_diagnostic || (cfg!(feature = "cpu-probe") && mode == "on" && !bill),
        "GPU diagnostic needs cpu-probe/on and cannot be combined with management bill");
    anyhow::ensure!(std::env::var("TG_GPU_TIMING").is_err(),
        "TG_GPU_TIMING must be absent; this fixture exclusively owns diagnostic timing");
    let mut workload = workload::PerformanceWorkload::from_environment()?
        .unwrap_or(workload::PerformanceWorkload::new("static")?);
    anyhow::ensure!(workload.scenario() == "static" || warmup >= 200,
        "edit/many-draw protocol needs at least 200 warmup frames (acceptance uses 600)");
    let BootedSession { mut driver, context, mut bridge, shaders: _shaders, device, _boot } = boot_shipped_session()?;
    let canonical_brick_layout = if canonical_wall_layout {
        Some(sparse_layout::before_first_gameplay_warmup(driver.app_mut().world_mut(), &mut bridge)?)
    } else { None };
    let boot_clock = clock_input(driver.app().world());
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, driver.app_mut().world_mut())?;
    bridge.accept_gameplay_warmup(driver.app().world().resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;
    let mut lists = country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::new();
    #[cfg(feature = "cpu-probe")]
    let mut recorder = (mode == "on").then(|| CpuProbeSession::new(CpuProbeConfig {
        label: format!("{}_headless", workload.scenario()), warmup_frames: 0,
        sample_frames: samples, capacity: samples + 65,
    })).transpose()?;
    let mut rows: Vec<Row> = Vec::new();
    #[cfg(feature = "cpu-probe")]
    let mut detailed_bills: Vec<(serde_json::Value, serde_json::Value)> = Vec::new();
    #[cfg(feature = "cpu-probe")]
    let mut binding_scope_bills: Vec<tiny_glade::raster_host::BindingScopeProbeCounts> = Vec::new();
    #[cfg(feature = "cpu-probe")]
    let mut wall_dispatch_source_bills: Vec<WallDispatchSourceRow> = Vec::new();
    #[cfg(feature = "cpu-probe")]
    if wall_source_diagnostic { wall_dispatch_source_bills.try_reserve_exact(samples + 1)?; }
    #[cfg(feature = "cpu-probe")]
    if mode == "on" && bill { binding_scope_bills.try_reserve_exact(samples + 1)?; }
    #[cfg(feature = "cpu-probe")]
    if mode == "on" && bill { detailed_bills.try_reserve_exact(samples + 1)?; }
    #[cfg(feature = "cpu-probe")]
    if mode == "on" { rows.try_reserve_exact(samples + 1)?; }
    // Hold exactly one BufferHost for the entire fixed App/session. Its
    // manager token is process-local proof, never paired scene identity.
    #[cfg(feature = "cpu-probe")]
    let upload_host = bridge.compute_device().buffers.clone();
    #[cfg(feature = "cpu-probe")]
    let mut actual_workload = (mode == "on").then(|| workload_probe::WorkloadProbe::new(
        samples + 1, upload_host.probe_upload_submissions(), device.texture_upload_probe_snapshot()))
        .transpose()?;
    #[cfg(feature = "cpu-probe")]
    let mut gpu = if gpu_diagnostic {
        let frames = warmup.checked_add(samples).and_then(|n| n.checked_add(1))
            .context("GPU diagnostic frame count overflow")?;
        let capacity = number("TG_GPU_DIAGNOSTIC_CAPACITY", frames.saturating_mul(128).min(2_000_000))?;
        Some(gpu_diagnostic::GpuDiagnostic::new(
            &mut bridge, &device, frames, capacity, gpu_pass_spans, gpu_isolate, gpu_statistics)?)
    } else {
        // A clean run must not inherit a timestamp ring from renderer warmup.
        bridge.with_session(|s| s.backend.configure_gpu_timing(None))?;
        None
    };
    let mut initial_input = serde_json::Value::Null;
    let mut trajectory = None;
    let mut begin = [0u64; 3];
    let mut begin_boundary = [[0u64; 2]; 2];
    #[cfg(feature = "cpu-probe")]
    let _ = country_core::render::declare_probe::take();
    for index in 0..warmup + samples + 1 {
        #[cfg(feature = "cpu-probe")]
        let measured = index >= warmup;
        if index == warmup {
            let lengths: Vec<_> = (0..country_core::systems::render_loop::draw_passes::draw_lists::DRAW_LIST_COUNT)
                .map(|i| lists.list(i).len()).collect();
            initial_input = input::scene_manifest(driver.app_mut().world_mut(), &lengths)?;
            initial_input["clock_camera"] = clock_input(driver.app().world());
            initial_input["workload"] = workload.summary();
            if let Some(layout) = &canonical_brick_layout { initial_input["canonical_brick_layout"] = serde_json::to_value(layout)?; }
            trajectory = Some(input::TransformTrajectory::capture(driver.app_mut().world_mut())?);
            if let Some(parent) = output.parent() { std::fs::create_dir_all(parent)?; }
            if std::env::var_os("TG_CPU_PROBE_GATE").is_some() {
                std::fs::write(output.with_extension("ready"), b"ready")?;
                let deadline = std::time::Instant::now() + Duration::from_secs(120);
                while !output.with_extension("go").exists() {
                    anyhow::ensure!(std::time::Instant::now() < deadline, "ETW start handshake timed out");
                    std::thread::sleep(Duration::from_millis(1));
                }
            }
            (begin, begin_boundary) = read_cpu_envelope()?;
        }
        #[cfg(feature = "cpu-probe")]
        let active = measured && mode == "on";
        #[cfg(feature = "cpu-probe")]
        let mut points = [0u64; 9];
        #[cfg(feature = "cpu-probe")]
        if active { points[0] = clock::wall_ticks(); }
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
        #[cfg(feature = "cpu-probe")]
        if active { points[1] = clock::wall_ticks(); }
        while bridge.with_session(|s| s.pending_frame_count()) >= device.frames_in_flight() as usize {
            anyhow::ensure!(bridge.with_session(|s| s.wait_gpu_queue_trim(120_000_000_000))?, "GPU pacing timed out");
            anyhow::ensure!(bridge.collect_completed_frames()? > 0, "completed queue front not reclaimed");
        }
        #[cfg(feature = "cpu-probe")]
        if active { points[2] = clock::wall_ticks(); probe::begin_stages(); }
        let world = driver.app_mut().world_mut();
        #[cfg(feature = "cpu-probe")]
        let tick = world.resource::<country_core::render::frame_constants::FrameConstants>().frame_counter as u64;
        // This exact World has completed App::update. Capture the unmodified
        // sparse boundaries before prepare; pair only after execute succeeds.
        #[cfg(feature = "cpu-probe")]
        let wall_source = if active && wall_source_diagnostic {
            Some(sparse_ranges::wall_dispatch_snapshot(world)?)
        } else { None };
        let prepared = tiny_glade::production_renderer::prepare_headless_gameplay_frame_with_lists(&mut bridge, world, &mut lists)?;
        #[cfg(feature = "cpu-probe")]
        if active { points[3..6].copy_from_slice(&probe::take_stages().context("missing stage owner")?); }
        bridge.prepare_test_execution()?;
        #[cfg(feature = "cpu-probe")]
        if active { points[6] = clock::wall_ticks(); }
        let frame = prepared.execute(&mut bridge, world)?;
        #[cfg(feature = "cpu-probe")]
        let receipt = frame.receipt.frame;
        #[cfg(feature = "cpu-probe")]
        let workload_stats = frame.receipt.stats;
        #[cfg(feature = "cpu-probe")]
        let workload_completion_points = frame.receipt.receipts.len();
        #[cfg(feature = "cpu-probe")]
        let workload_retained_passes = frame.receipt.snapshot.pass_order.len();
        #[cfg(feature = "cpu-probe")]
        if let Some(gpu) = &mut gpu { gpu.observe(measured, tick, index as u64 + 1, &frame.receipt); }
        #[cfg(feature = "cpu-probe")]
        if active { points[7] = clock::wall_ticks(); }
        world.resource_mut::<country_core::render::RenderTemporalState>().frame_render_returned();
        let extent = frame.texture.desc.extent();
        let output_extent = [extent.width, extent.height];
        drop(frame);
        bridge.collect_completed_frames()?;
        let lengths: [usize; country_core::systems::render_loop::draw_passes::draw_lists::DRAW_LIST_COUNT] =
            std::array::from_fn(|i| lists.list(i).len());
        workload.after_frame(world, index as u64 + 1, output_extent, &lengths)?;
        if let Some(trajectory) = &mut trajectory { trajectory.observe(world, index as u64 + 1)?; }
        #[cfg(feature = "cpu-probe")]
        if let Some(actual) = &mut actual_workload {
            actual.observe(measured, tick, receipt, index as u64 + 1, workload_stats,
                workload_completion_points, workload_retained_passes, lengths,
                upload_host.probe_upload_submissions(), device.texture_upload_probe_snapshot());
        }
        #[cfg(feature = "cpu-probe")]
        if let Some(gpu) = &mut gpu { gpu.drain(&mut bridge)?; }
        #[cfg(feature = "cpu-probe")]
        if active {
            points[8] = clock::wall_ticks();
            let counts = bill.then(|| bridge.raster_resources().draw_management_counts());
            let declaration = bill.then(country_core::render::declare_probe::take);
            if bill {
                detailed_bills.push((bridge.raster_resources().constant_pool_bill(),
                    bridge.raster_resources().draw_scratch_bill()));
                binding_scope_bills.push(bridge.raster_resources().binding_scope_probe_counts());
            }
            if let Some(source) = wall_source {
                anyhow::ensure!(wall_dispatch_source_bills.len() < samples + 1,
                    "Wall source diagnostic exceeded its fixed observation capacity");
                wall_dispatch_source_bills.push((tick, receipt, index as u64 + 1, source));
            }
            rows.push((tick, receipt, index as u64 + 1, points, counts, declaration));
            recorder.as_mut().unwrap().record(tick, Some(receipt), index as u64 + 1, true);
        }
    }
    let (end, end_boundary) = read_cpu_envelope()?;
    #[cfg(feature = "cpu-probe")]
    if let Some(actual) = &mut actual_workload { actual.finish(); }
    let lengths: Vec<_> = (0..country_core::systems::render_loop::draw_passes::draw_lists::DRAW_LIST_COUNT)
        .map(|i| lists.list(i).len()).collect();
    let mut final_input = input::scene_manifest(driver.app_mut().world_mut(), &lengths)?;
    final_input["clock_camera"] = clock_input(driver.app().world());
    final_input["workload"] = workload.summary();
    if let Some(layout) = &canonical_brick_layout { final_input["canonical_brick_layout"] = serde_json::to_value(layout)?; }
    final_input["transform_trajectory"] = trajectory.as_ref().context("missing trajectory owner")?.summary();
    bridge.wait_submits_done(120_000_000_000)?;
    #[cfg(feature = "cpu-probe")]
    if let Some(gpu) = &mut gpu { gpu.drain(&mut bridge)?; }
    #[cfg(feature = "cpu-probe")]
    let gpu_report = gpu.as_ref().map(gpu_diagnostic::GpuDiagnostic::report);
    #[cfg(not(feature = "cpu-probe"))]
    let gpu_report: Option<serde_json::Value> = None;
    #[cfg(feature = "cpu-probe")]
    let actual_workload_report = actual_workload.as_ref().map(workload_probe::WorkloadProbe::report);
    #[cfg(not(feature = "cpu-probe"))]
    let actual_workload_report: Option<serde_json::Value> = None;
    #[cfg(feature = "cpu-probe")]
    let session = recorder.as_ref().map(CpuProbeSession::to_json);
    #[cfg(not(feature = "cpu-probe"))]
    let session: Option<serde_json::Value> = None;
    // Autonomous intro sheep/accessories may move during fixed-step no-edit
    // rendering. Keep both raw manifests; paired runs must match both ends.
    let input_unchanged = initial_input == final_input;
    let clock_valid = begin[0] > 0 && end[0] > begin[0]
        && end[1] >= begin[1] && end[2] >= begin[2] && clock::wall_frequency() > 0;
    #[cfg(feature = "cpu-probe")]
    let binding_scope_reuse_enabled = Some(bridge.raster_resources().binding_scope_probe_counts().algorithm_enabled);
    #[cfg(not(feature = "cpu-probe"))]
    let binding_scope_reuse_enabled: Option<bool> = None;
    #[cfg(feature = "cpu-probe")]
    let binding_scope_bill_valid = !bill || (binding_scope_bills.len() == rows.len()
        && binding_scope_bills.iter().all(|counts| counts.valid
            && Some(counts.algorithm_enabled) == binding_scope_reuse_enabled && counts.failed_callbacks == 0));
    #[cfg(not(feature = "cpu-probe"))]
    let binding_scope_bill_valid = true;
    #[cfg(feature = "cpu-probe")]
    let wall_dispatch_source_bill_valid = !wall_source_diagnostic || (
        wall_dispatch_source_bills.len() == samples + 1 && wall_dispatch_source_bills.len() == rows.len()
        && wall_dispatch_source_bills.iter().zip(&rows).all(|((tick, receipt, attempt, source), row)| {
            (*tick, *receipt, *attempt) == (row.0, row.1, row.2) && !source.native_dispatches_observed
        }));
    #[cfg(not(feature = "cpu-probe"))]
    let wall_dispatch_source_bill_valid = true;
    // All JSON construction and digest formatting happen after the endpoint.
    #[cfg(feature = "cpu-probe")]
    let wall_dispatch_source_bill = wall_source_diagnostic.then(|| json!({
        "schema":"wall-dispatch-source-bill-v1", "valid":wall_dispatch_source_bill_valid,
        "capacity":samples+1, "retained_observations":wall_dispatch_source_bills.len(), "warmup_observations":0,
        "source_phase":"after_App_update_before_prepare", "native_dispatches_observed":false,
        "scope":"Wall source ranges only; matching a successful receipt does not independently prove callbacks executed or GPU completion",
        "association":"same World tick/attempt as successful execute receipt; first measured observation is row zero, not a warmup sample",
        "observations":wall_dispatch_source_bills.iter().map(|(tick,receipt,attempt,source)| json!({
            "world_tick":tick, "receipt":receipt, "attempt":attempt, "source":source,
            "raw_ranges_blake3_hex":source.raw_ranges_blake3.iter().map(|b| format!("{b:02x}")).collect::<String>(),
        })).collect::<Vec<_>>(),
    }));
    #[cfg(not(feature = "cpu-probe"))]
    let wall_dispatch_source_bill: Option<serde_json::Value> = None;
    let valid = wall_dispatch_source_bill_valid && binding_scope_bill_valid && clock_valid && session.as_ref().is_none_or(|s| s["valid"] == true)
        && actual_workload_report.as_ref().is_none_or(|s| s["valid"] == true)
        && gpu_report.as_ref().is_none_or(|s| s["valid"] == true);
    let management_modes = bridge.raster_resources().draw_management_modes();
    #[cfg(feature = "cpu-probe")]
    anyhow::ensure!(!bill || detailed_bills.len() == rows.len(), "incomplete detailed management bill");
    let mut report = json!({
        "schema":"headless-cpu-closure-v1", "valid":valid, "scope":"App and renderer share owner thread; fixed-step real scene stimuli and autonomous animation retained; phase CPU requires ETW; no WSI acceptance",
        "process_id":std::process::id(), "thread_id":clock::current_thread_id(), "qpc_frequency":clock::wall_frequency(),
        "warmup":warmup, "samples":samples, "mode":mode, "diagnostic_bill":bill,
        "scenario":workload.scenario(), "boot_clock":boot_clock,
        "measurement_class":if gpu_diagnostic {"gpu_diagnostic"} else if bill {"management_diagnostic"} else if wall_source_diagnostic {"wall_dispatch_source_diagnostic"} else if canonical_wall_layout {"canonical_layout_diagnostic"} else {"clean_cpu"},
        "clean_cpu_acceptance_eligible":!gpu_diagnostic && !wall_source_diagnostic && !canonical_wall_layout,
        "wall_dispatch_source_bill":wall_dispatch_source_bill,
        "actual_workload":actual_workload_report,"gpu_diagnostic":gpu_report,
        "binding_scope_reuse_enabled":binding_scope_reuse_enabled,
        "binding_scope_bill_valid":binding_scope_bill_valid,
        "management_modes":{"reflection_scope":management_modes[0],
            "vao_typed_lookup":management_modes[1], "vao_wrapper_reuse":management_modes[2],
            "constant_batch":bridge.raster_resources().constant_batch_enabled()},
        "workload":workload.summary(),
        "maintenance_scope":"includes identical real ECS/raster stimuli and transform trajectory hashing on both binaries; not production render-only CPU",
        "cpu_probe_feature":cfg!(feature = "cpu-probe"), "clock_valid":clock_valid,
        "input_unchanged":input_unchanged,
        "input_policy":"fixed_step_scripted_scene; paired_before_after_trajectory_must_match",
        "allocator":if cfg!(feature="mimalloc") {"mimalloc"} else {"System"},
        "envelope":{"begin":begin,"end":end,"frames":samples+1}, "session":session,
        "boundary_uncertainty":{"schema":"cpu-envelope-brackets-v1", "clock":"qpc",
            "begin_owner":begin_boundary[0], "begin_process":begin_boundary[1],
            "end_owner":end_boundary[0], "end_process":end_boundary[1]},
        "phase_names":["app_update","pacing","draw_lists_vao","declare","compile","pending_admission","record_submit","maintenance"],
        "management_names":["reflection_queries","reflection_hits","constant_block_takes","collected_uniform_write_bytes","binding_resource_inputs","prepared_bindless_draws","vao_full_refreshes","vao_list_entries","vao_queried_entities","vao_unique_visits","vao_publications","vao_snapshot_allocations","cached_reflection_variants"],
        "frames":rows.iter().enumerate().map(|(_index,(tick,receipt,attempt,points,counts,declaration))| {
            #[allow(unused_mut)]
            let mut row = json!({"world_tick":tick,"receipt":receipt,"attempt":attempt,
                "qpc":points,"management":counts,"declaration":declaration});
            #[cfg(feature = "cpu-probe")]
            if let Some((constants, scratch)) = detailed_bills.get(_index) {
                row["constant_pool_bill"] = constants.clone();
                row["draw_scratch_bill"] = scratch.clone();
            }
            #[cfg(feature = "cpu-probe")]
            if let Some(binding) = binding_scope_bills.get(_index) {
                row["binding_scope_bill"] = json!(binding);
            }
            row
        }).collect::<Vec<_>>(),
        "input_before":initial_input,"input_after":final_input,
    });
    if let Some(layout) = canonical_brick_layout {
        report["canonical_brick_layout"] = serde_json::to_value(layout)?;
        report["canonical_layout_input_policy"] = "canonical-brick-layout-v1".into();
    }
    std::fs::write(&output, serde_json::to_vec_pretty(&report)?)?;
    anyhow::ensure!(valid, "incomplete probe or invalid clock envelope; inspect {}", output.display());
    Ok(())
}


fn clock_input(world: &bevy_ecs::world::World) -> serde_json::Value {
    let game = world.resource::<country_core::resources::game_clock::GameClock>();
    let time = world.resource::<bevy_time::Time>();
    let frame = world.resource::<country_core::render::frame_constants::FrameConstants>();
    json!({"game_frame":game.frame_count,"game_total_bits":game.total_secs.to_bits(),
        "game_delta_bits":game.delta.to_bits(),"game_paused":game.paused,"game_ran":game.ran,
        "time_elapsed_ns":time.elapsed().as_nanos().to_string(),"time_delta_ns":time.delta().as_nanos().to_string(),
        "render_frame":frame.frame_counter,"time_of_day_bits":frame.time_of_day.to_bits(),
        "camera_bits":world.resource::<country_core::systems::camera_rig::MainCamera>()
            .read_matrix(0xc0).to_cols_array().map(f32::to_bits)})
}
