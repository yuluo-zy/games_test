use super::{
    fixture::{self, HEIGHT, Playlist, Result, WIDTH},
    metrics::{FrameSample, MAX_SAMPLES, Measurements, PerformanceReport},
    multi_queue_scene::{MultiQueueReport, MultiQueueScene, SharingChoice},
    platform,
    result_scene::{ResultScene, ResultSceneReport},
    scene::{self, RasterScene, RenderingChoice, SceneKind, SceneReport},
    transfer_scene::{TransferReport, TransferScene},
    transient_scene::{TransientReport, TransientScene},
};
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandBuffer,
        device::Device,
        instance::{InstanceBuilder, ValidationMessenger},
        swapchain::{AcquiredImageLease, Swapchain},
        texture::{Texture, TextureDesc},
    },
    rg::{
        executor::PassContext,
        frame_loop::{FrameLoop, FrameStatus},
        graph::BuiltGraph,
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use serde::Serialize;
use std::{
    collections::{BTreeSet, VecDeque},
    ffi::{CStr, CString},
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

const HOST_READ: u8 = 0x26;
const HOST_WRITE: u8 = 0x35;
const TRANSFER_READ: u8 = 0x25;
const TRANSFER_WRITE: u8 = 0x34;
const TIMEOUT: u64 = 5_000_000_000;

struct Options {
    performance: bool,
    queues: rhapsody::device::QueuePreference,
    sharing: SharingChoice,
    gpu_timing: bool,
    gpu_pass_timing: bool,
    vsync: bool,
    allocation: rhapsody::device::AllocationStrategy,
    warmup: u64,
    samples: Option<usize>,
    plan_cache: bool,
    subgraph_cache: bool,
    result_cache: bool,
    playlist: PathBuf,
    frames: Option<u64>,
    minimum_run_seconds: Option<Duration>,
    fps: u32,
    script: bool,
    wsi_stress: bool,
    report: PathBuf,
    capture: Option<PathBuf>,
    require_validation: bool,
    scene: SceneKind,
    scene_script: bool,
    rendering: RenderingChoice,
    shader_script: bool,
    repeat_script: bool,
    transient_aliasing: bool,
    transient_attachments: u32,
    transient_scale: u32,
    pipeline_cache: PathBuf,
}
impl Options {
    fn parse() -> Result<Option<Self>> {
        Self::parse_from(std::env::args().skip(1))
    }
    fn parse_from(args: impl IntoIterator<Item = String>) -> Result<Option<Self>> {
        let mut options = Self {
            performance: false,
            queues: rhapsody::device::QueuePreference::Off,
            sharing: SharingChoice::Exclusive,
            gpu_timing: false,
            gpu_pass_timing: false,
            vsync: true,
            allocation: rhapsody::device::DeviceConfig::default().allocation,
            warmup: 0,
            samples: None,
            plan_cache: true,
            subgraph_cache: true,
            result_cache: true,
            playlist: fixture::default_playlist(),
            frames: None,
            minimum_run_seconds: None,
            fps: 60,
            script: false,
            wsi_stress: false,
            report: Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/window-lab/report.json"),
            capture: None,
            require_validation: false,
            scene: SceneKind::Showcase,
            scene_script: false,
            rendering: RenderingChoice::Auto,
            shader_script: false,
            repeat_script: false,
            transient_aliasing: false,
            transient_attachments: 8,
            transient_scale: 2,
            pipeline_cache: Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/window-lab/pipeline.cache"),
        };
        let mut explicit_frames = false;
        let mut explicit_queues = false;
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--performance" => options.performance = true,
                "--queues" => {
                    explicit_queues = true;
                    options.queues = match args.next().as_deref() {
                        Some("off") => rhapsody::device::QueuePreference::Off,
                        Some("prefer") => rhapsody::device::QueuePreference::Prefer,
                        Some("require") => rhapsody::device::QueuePreference::Require,
                        _ => return Err("queues must be off, prefer, or require".into()),
                    };
                }
                "--sharing" => {
                    options.sharing =
                        SharingChoice::parse(&args.next().ok_or("missing sharing mode")?)?
                }
                "--gpu-timing" => {
                    options.gpu_timing = match args.next().as_deref() {
                        Some("on") => true,
                        Some("off") => false,
                        _ => return Err("gpu timing must be on or off".into()),
                    }
                }
                "--gpu-pass-timing" => {
                    options.gpu_pass_timing = match args.next().as_deref() {
                        Some("on") => true,
                        Some("off") => false,
                        _ => return Err("gpu pass timing must be on or off".into()),
                    };
                    options.gpu_timing |= options.gpu_pass_timing;
                }
                "--vsync" => {
                    options.vsync = match args.next().as_deref() {
                        Some("on") => true,
                        Some("off") => false,
                        _ => return Err("vsync must be on or off".into()),
                    }
                }
                "--allocation" => {
                    options.allocation = match args.next().as_deref() {
                        Some("suballocated") => rhapsody::device::AllocationStrategy::Suballocated,
                        Some("dedicated") => rhapsody::device::AllocationStrategy::Dedicated,
                        _ => return Err("allocation must be suballocated or dedicated".into()),
                    }
                }
                "--warmup" => {
                    options.warmup = args.next().ok_or("missing warmup count")?.parse()?
                }
                "--samples" => {
                    options.samples = Some(args.next().ok_or("missing sample count")?.parse()?)
                }
                "--plan-cache" | "--subgraph-cache" | "--result-cache" => {
                    let enabled = match args.next().as_deref() {
                        Some("on") => true,
                        Some("off") => false,
                        _ => return Err("cache switch must be on or off".into()),
                    };
                    match arg.as_str() {
                        "--plan-cache" => options.plan_cache = enabled,
                        "--subgraph-cache" => options.subgraph_cache = enabled,
                        _ => options.result_cache = enabled,
                    }
                }
                "--playlist" => {
                    options.playlist = args.next().ok_or("missing playlist path")?.into()
                }
                "--frames" => {
                    options.frames = Some(args.next().ok_or("missing frame count")?.parse()?);
                    explicit_frames = true;
                }
                "--min-run-seconds" => {
                    let seconds: f64 = args.next().ok_or("missing minimum run seconds")?.parse()?;
                    if !seconds.is_finite() || seconds <= 0.0 {
                        return Err("minimum run seconds must be positive and finite".into());
                    }
                    let duration = Duration::try_from_secs_f64(seconds)?;
                    if duration.is_zero() {
                        return Err("minimum run seconds rounds to zero".into());
                    }
                    options.minimum_run_seconds = Some(duration);
                }
                "--fps" => options.fps = args.next().ok_or("missing fps")?.parse()?,
                "--report" => options.report = args.next().ok_or("missing report path")?.into(),
                "--capture" => {
                    options.capture = Some(args.next().ok_or("missing capture path")?.into())
                }
                "--script" => options.script = true,
                "--wsi-stress" => options.wsi_stress = true,
                "--require-validation" => options.require_validation = true,
                "--scene" => {
                    options.scene = SceneKind::parse(&args.next().ok_or("missing scene")?)?
                }
                "--scene-script" => options.scene_script = true,
                "--shader-script" => options.shader_script = true,
                "--repeat-script" => options.repeat_script = true,
                "--transient-aliasing" => {
                    options.transient_aliasing = match args.next().as_deref() {
                        Some("on") => true,
                        Some("off") => false,
                        _ => return Err("transient aliasing must be on or off".into()),
                    }
                }
                "--transient-attachments" => {
                    options.transient_attachments =
                        args.next().ok_or("missing attachment count")?.parse()?
                }
                "--transient-scale" => {
                    options.transient_scale =
                        args.next().ok_or("missing transient scale")?.parse()?
                }
                "--pipeline-cache" => {
                    options.pipeline_cache =
                        args.next().ok_or("missing pipeline cache path")?.into()
                }
                "--rendering" => {
                    options.rendering =
                        RenderingChoice::parse(&args.next().ok_or("missing rendering mode")?)?
                }
                "--help" | "-h" => {
                    println!(
                        "Window Lab\n--gpu-timing on|off --gpu-pass-timing on|off --vsync on|off --warmup N --samples 1..10000 --plan-cache on|off --subgraph-cache on|off --result-cache on|off --allocation suballocated|dedicated --queues off|prefer|require --sharing exclusive|concurrent --scene fixture|raster|temporal|transfer|transient|result|multi-queue|showcase --transient-aliasing on|off --transient-attachments 3..64 --transient-scale 1..8 --rendering auto|dynamic --scene-script --shader-script --repeat-script --pipeline-cache FILE --playlist FILE --frames N --min-run-seconds N --fps 1..240 --script | --wsi-stress --report FILE --capture FILE.ppm --require-validation\nKeys: Space pause, N next clip, Right step, Esc quit\nPPM/P3/P6 images are supported; no video codec is required."
                    );
                    return Ok(None);
                }
                _ => return Err(format!("unknown option: {arg}").into()),
            }
        }
        if options.gpu_pass_timing && !options.gpu_timing {
            return Err("per-pass timestamps require GPU timing".into());
        }
        if options.minimum_run_seconds.is_some() && !explicit_frames {
            return Err("--min-run-seconds requires explicit finite --frames".into());
        }
        if options
            .samples
            .is_some_and(|samples| samples == 0 || samples > MAX_SAMPLES)
        {
            return Err("samples must be 1..=10000".into());
        }
        if options.performance {
            if options.scene != SceneKind::Showcase
                || options.script
                || options.wsi_stress
                || options.shader_script
                || options.require_validation
            {
                return Err("--performance requires showcase without validation or window/shader stress scripts".into());
            }
            options.warmup = if options.warmup == 0 {
                300
            } else {
                options.warmup
            };
            options.samples.get_or_insert(3000);
            options.fps = 240;
            options.gpu_timing = true;
        }
        if let Some(samples) = options.samples {
            let total = options
                .warmup
                .checked_add(samples as u64)
                .ok_or("performance frame count overflow")?;
            options.frames.get_or_insert(total);
            if options.frames.is_some_and(|frames| frames < total) {
                return Err("frames must cover warmup + samples".into());
            }
        }
        if options.fps == 0 || options.fps > 240 || options.frames == Some(0) {
            return Err("fps must be 1..=240 and frames must be positive".into());
        }
        if options.scene == SceneKind::Showcase && !explicit_queues {
            options.queues = rhapsody::device::QueuePreference::Prefer;
        }
        if options.script && options.wsi_stress {
            return Err("--script and --wsi-stress are separate modes".into());
        }
        if options.wsi_stress {
            if options.scene != SceneKind::Showcase {
                options.frames.get_or_insert(360);
            }
            if options.frames.is_some_and(|n| n < 360) {
                return Err("--wsi-stress requires at least 360 frames".into());
            }
        }
        if options.script && options.frames.is_some_and(|n| n < 180) {
            return Err("--script requires at least 180 frames".into());
        }
        if options.scene_script {
            if options.scene != SceneKind::Temporal {
                return Err("--scene-script requires --scene temporal".into());
            }
            options.frames.get_or_insert(360);
            if options.frames.is_some_and(|n| n < 180) {
                return Err("--scene-script requires at least 180 frames".into());
            }
        }
        if options.scene == SceneKind::Transfer {
            options.frames.get_or_insert(360);
            if options.frames.is_some_and(|n| n < 64) {
                return Err("--scene transfer requires at least 64 frames to verify full payload and capacity recovery".into());
            }
        }
        if options.rendering != RenderingChoice::Auto
            && !matches!(
                options.scene,
                SceneKind::Raster
                    | SceneKind::Temporal
                    | SceneKind::MultiQueue
                    | SceneKind::Showcase
            )
        {
            return Err("--rendering requires raster, temporal, or multi-queue scene".into());
        }
        if options.scene != SceneKind::MultiQueue
            && matches!(options.sharing, SharingChoice::Concurrent)
        {
            return Err("--sharing concurrent requires --scene multi-queue".into());
        }
        if options.repeat_script && !options.shader_script && !options.wsi_stress {
            return Err("--repeat-script requires --shader-script or --wsi-stress".into());
        }
        if options.shader_script {
            if options.scene != SceneKind::Raster {
                return Err("--shader-script requires raster scene".into());
            }
            options.frames.get_or_insert(360);
            if options.frames.is_some_and(|n| n < 120) {
                return Err("--shader-script requires at least 120 frames".into());
            }
        }
        if !(3..=64).contains(&options.transient_attachments)
            || !(1..=8).contains(&options.transient_scale)
        {
            return Err("transient attachments must be 3..64 and scale must be 1..8".into());
        }
        if matches!(options.scene, SceneKind::Transient | SceneKind::ResultCache) {
            options.frames.get_or_insert(360);
            if options.frames.is_some_and(|n| n < 360) {
                return Err("transient/result scenes require at least 360 frames".into());
            }
        }
        Ok(Some(options))
    }
    fn reached_limit(&self, presented: u64, elapsed: Duration) -> bool {
        self.frames.is_some_and(|frames| presented >= frames)
            && self
                .minimum_run_seconds
                .is_none_or(|minimum| elapsed >= minimum)
    }
    fn accepts_completed_run(&self, presented: u64, elapsed: Duration) -> bool {
        self.frames.is_none_or(|frames| {
            if self.minimum_run_seconds.is_some() {
                presented >= frames
            } else {
                presented == frames
            }
        }) && self
            .minimum_run_seconds
            .is_none_or(|minimum| elapsed >= minimum)
    }
    fn stop_new_wsi_actions(&self, presented: u64, elapsed: Duration) -> bool {
        self.reached_limit(presented, elapsed)
            || (self.minimum_run_seconds.is_none()
                && self.repeat_script
                && self
                    .frames
                    .is_some_and(|frames| frames.saturating_sub(presented) <= 16))
    }
    fn shader_script_frame_limit(&self) -> Option<u64> {
        if self.minimum_run_seconds.is_some() {
            None
        } else {
            self.frames
        }
    }
}

#[derive(Default, Serialize)]
struct Report {
    multi_queue_scene: Option<MultiQueueReport>,
    actual_queues: Vec<PhysicalQueueReport>,
    class_queue_indices: [usize; 3],
    vsync_requested: bool,
    present_mode: String,
    gpu_timing: Option<rhapsody::rg::profiling::GpuTimingStats>,
    /// Latest bounded per-pass command spans, distinct from outer command
    /// spans. These intervals can overlap and are not additive frame latency.
    gpu_pass_timings: VecDeque<PassGpuTimingReport>,
    gpu_pass_timings_dropped: u64,
    queue_lock_waits:
        [Option<rhapsody::sync::timeline::QueueLockStats>; rhapsody::sync::MAX_QUEUES],
    reclamation: rhapsody::raw::graveyard::GraveyardStats,
    cache_diagnostics: Vec<CacheDomainReport>,
    application_gpu_memory: Option<GpuMemoryReport>,
    performance: Option<PerformanceReport>,
    plan_cache_enabled: bool,
    subgraph_cache_enabled: bool,
    result_cache_enabled: bool,
    schema_version: u32,
    milestone: String,
    platform: String,
    window_backend: String,
    status: String,
    command: Vec<String>,
    git_revision_at_launch: Option<String>,
    git_dirty_at_launch: Option<bool>,
    device: String,
    driver_version_raw: u32,
    api_version_raw: u32,
    instance_api_version_raw: u32,
    physical_api_version_raw: u32,
    enabled_device_extensions: Vec<String>,
    validation_enabled: bool,
    performance_mode: bool,
    validation_errors: Vec<String>,
    validation_warnings: Vec<String>,
    validation_error_count: usize,
    validation_warning_count: usize,
    fixture_hash: String,
    requested_frames: Option<u64>,
    presented_frames: u64,
    pixel_verified_frames: u64,
    source_hash: String,
    readback_hash: String,
    visited_clips: BTreeSet<String>,
    resize_events: u64,
    recreate_events: u64,
    minimized_iterations: u64,
    script_completed: bool,
    presentation_shutdown_complete: bool,
    retained_wsi_generations: u64,
    wsi_stress: Option<WsiStressReport>,
    scene: SceneReport,
    showcase: Option<super::showcase::ShowcaseReport>,
    transfer: Option<TransferReport>,
    transient: Option<TransientReport>,
    result_cache_scene: Option<ResultSceneReport>,
    pause_actions: u64,
    next_actions: u64,
    step_actions: u64,
    cpu_frame_ms_p50: f64,
    cpu_frame_ms_p95: f64,
    cpu_frame_ms_p99: f64,
    elapsed_seconds: f64,
    play_seconds: f64,
    minimum_run_seconds: Option<f64>,
    initialization_seconds: f64,
    first_present_seconds: Option<f64>,
    script_drain_seconds: f64,
    repeat_script: bool,
    gpu_time_ms: Option<f64>,
    peak_gpu_bytes: Option<u64>,
    memory_budget_usage: Option<BudgetReport>,
    stability_samples: VecDeque<StabilitySample>,
    stability_samples_dropped: u64,
    teardown_memory: Option<TeardownMemoryReport>,
    window_visual_check: String,
    limitations: Vec<String>,
    failure: Option<String>,
}
#[derive(Serialize)]
struct PhysicalQueueReport {
    family: u32,
    index: u32,
}

const STABILITY_SAMPLE_LIMIT: usize = 128;
#[derive(Serialize)]
struct StabilitySample {
    presented_frames: u64,
    elapsed_seconds: f64,
    phase: &'static str,
    graph_resources: usize,
    graph_commands: usize,
    graph_command_pools: usize,
    pending_graph_frames: usize,
    named_resources: usize,
    registered_budget_bytes: u64,
    known_cpu_bytes: u64,
    application_vk_memory_bytes: Option<u64>,
    native_memory_blocks: Option<usize>,
}
#[derive(Serialize)]
struct CacheCauseReport {
    reason: rhapsody::rg::cache::CacheReason,
    misses: u64,
    invalidations: u64,
}
#[derive(Serialize)]
struct PassGpuTimingReport {
    #[serde(flatten)]
    sample: rhapsody::rg::profiling::GpuCommandTiming,
    pass_name: String,
}
#[derive(Serialize)]
struct CacheDomainReport {
    domain: &'static str,
    causes: Vec<CacheCauseReport>,
}
#[derive(Serialize)]
struct TeardownMemoryReport {
    scope: &'static str,
    registered_allocations: usize,
    registered_resource_bytes: u64,
    known_cpu_bytes: u64,
    registered_budget_bytes: u64,
}
impl TeardownMemoryReport {
    fn released(&self) -> bool {
        self.registered_allocations == 0
            && self.registered_resource_bytes == 0
            && self.known_cpu_bytes == 0
            && self.registered_budget_bytes == 0
    }
}
impl Report {
    fn sample_stability(&mut self, lab: &Lab, elapsed: Duration, phase: &'static str) {
        self.queue_lock_waits = lab.device.timelines.queue_lock_stats();
        self.reclamation = lab.device.garbage_stats();
        if phase == "before_lab_drop" {
            self.cache_diagnostics = [
                (
                    "whole_plan",
                    lab.frame_loop.graph.chain.plan_cache_stats().diagnostics,
                ),
                (
                    "subgraph_plan",
                    lab.frame_loop
                        .graph
                        .chain
                        .subgraph_cache_stats()
                        .diagnostics,
                ),
                (
                    "content_result",
                    lab.frame_loop.graph.result_cache_stats().diagnostics,
                ),
            ]
            .into_iter()
            .map(|(domain, diagnostics)| CacheDomainReport {
                domain,
                causes: rhapsody::rg::cache::CacheReason::ALL
                    .into_iter()
                    .map(|reason| CacheCauseReport {
                        reason,
                        misses: diagnostics.misses(reason),
                        invalidations: diagnostics.invalidations(reason),
                    })
                    .collect(),
            })
            .collect();
        }
        if self.stability_samples.len() == STABILITY_SAMPLE_LIMIT {
            self.stability_samples.pop_front();
            self.stability_samples_dropped += 1;
        }
        let memory = self
            .application_gpu_memory
            .as_ref()
            .map(|sample| sample.current);
        self.stability_samples.push_back(StabilitySample {
            presented_frames: self.presented_frames,
            elapsed_seconds: elapsed.as_secs_f64(),
            phase,
            graph_resources: lab.frame_loop.graph.resource_count(),
            graph_commands: lab.frame_loop.graph.command_count(),
            graph_command_pools: lab.frame_loop.graph.backend.command_pool_count(),
            pending_graph_frames: lab.frame_loop.graph.pending_frame_count(),
            named_resources: lab.frame_loop.graph.named_resource_count(),
            registered_budget_bytes: lab.device.allocator_stats.budget_usage().total(),
            known_cpu_bytes: lab.device.allocator_stats.cpu_memory_usage().total(),
            application_vk_memory_bytes: memory.map(|sample| sample.application_vk_memory_bytes),
            native_memory_blocks: memory.map(|sample| sample.native_memory_blocks),
        });
    }
}

#[derive(Serialize)]
struct BudgetReport {
    known_cpu_live_bytes: u64,
    known_cpu_cache_bytes: u64,
    known_cpu_peak_bytes: u64,
    scope: &'static str,
    live_bytes: u64,
    in_flight_bytes: u64,
    cache_bytes: u64,
    retiring_bytes: u64,
    total_bytes: u64,
    sampled_peak_total_bytes: u64,
}
#[derive(Serialize)]
struct GpuMemoryReport {
    scope: &'static str,
    current: rhapsody::device::memory::GpuMemoryUsage,
    sampled_peak_application_vk_memory_bytes: u64,
    sampled_peak_native_memory_blocks: usize,
}
impl GpuMemoryReport {
    fn sample(device: &Device, previous: &Option<Self>) -> Self {
        let current = device.memory_usage_snapshot();
        Self {
            scope: "Application VkDeviceMemory: native suballocator blocks including free space, plus dedicated allocations. Excludes driver-internal allocations and swapchain memory. Sampled at graph submission and frame completion.",
            sampled_peak_application_vk_memory_bytes: current.application_vk_memory_bytes.max(
                previous
                    .as_ref()
                    .map_or(0, |s| s.sampled_peak_application_vk_memory_bytes),
            ),
            sampled_peak_native_memory_blocks: current.native_memory_blocks.max(
                previous
                    .as_ref()
                    .map_or(0, |s| s.sampled_peak_native_memory_blocks),
            ),
            current,
        }
    }
}
impl BudgetReport {
    fn sample(device: &Device, previous: &Option<Self>) -> Self {
        let usage = device.allocator_stats.budget_usage();
        let cpu = device.allocator_stats.cpu_memory_usage();
        Self {
            scope: "Known resource ranges/backings and reserved CPU arenas/compiler payload bounds; excludes suballocator block slack, unregistered CPU containers and opaque driver allocations. Native reservations are reported separately.",
            known_cpu_live_bytes: cpu.live,
            known_cpu_cache_bytes: cpu.cache,
            known_cpu_peak_bytes: device.allocator_stats.cpu_peak_bytes(),
            live_bytes: usage.live,
            in_flight_bytes: usage.in_flight,
            cache_bytes: usage.cache,
            retiring_bytes: usage.retiring,
            total_bytes: usage.total(),
            sampled_peak_total_bytes: usage.total().max(
                previous
                    .as_ref()
                    .map_or(0, |sample| sample.sampled_peak_total_bytes),
            ),
        }
    }
}

const STRESS_RESIZES: u64 = 100;
const STRESS_CYCLES: u64 = 20;
const STRESS_TIMEOUT: Duration = Duration::from_secs(10);
const STRESS_MINIMIZED_HOLD: Duration = Duration::from_millis(50);

#[derive(Clone, Serialize)]
struct WsiStressEvent {
    kind: &'static str,
    from_extent: [u32; 2],
    observed_extent: [u32; 2],
    requested_logical_extent: Option<[u32; 2]>,
    presented_frames_at_observation: u64,
}
#[derive(Default, Clone, Serialize)]
struct WsiStressReport {
    target_resizes: u64,
    target_minimize_restore_cycles: u64,
    resize_requests: u64,
    resize_completions: u64,
    minimize_requests: u64,
    minimize_completions: u64,
    restore_requests: u64,
    restore_completions: u64,
    completed_cycles: u64,
    completed: bool,
    completed_batches: u64,
    diagnostic_events_dropped: u64,
    events: VecDeque<WsiStressEvent>,
}
impl WsiStressReport {
    fn new() -> Self {
        Self {
            target_resizes: STRESS_RESIZES,
            target_minimize_restore_cycles: STRESS_CYCLES,
            events: VecDeque::with_capacity((STRESS_RESIZES + STRESS_CYCLES * 2) as usize),
            ..Self::default()
        }
    }
    fn record(
        &mut self,
        kind: &'static str,
        before: (u32, u32),
        after: (u32, u32),
        requested: Option<(u32, u32)>,
        presented: u64,
    ) {
        if self.events.len() == (STRESS_RESIZES + STRESS_CYCLES * 2) as usize {
            self.events.pop_front();
            self.diagnostic_events_dropped += 1;
        }
        self.events.push_back(WsiStressEvent {
            kind,
            from_extent: before.into(),
            observed_extent: after.into(),
            requested_logical_extent: requested.map(Into::into),
            presented_frames_at_observation: presented,
        });
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StressAction {
    Resize(i32, i32),
    Minimize,
    Restore,
}
#[derive(Clone, Copy)]
enum StressPhase {
    Ready {
        after_presented: u64,
    },
    Resize {
        before: (u32, u32),
        requested: (u32, u32),
        deadline: Instant,
    },
    Minimize {
        before: (u32, u32),
        deadline: Instant,
    },
    Minimized {
        restore_at: Instant,
    },
    Restore {
        deadline: Instant,
    },
    Complete,
}
struct WsiStress {
    phase: StressPhase,
    report: WsiStressReport,
    repeat: bool,
    accept_new: bool,
}
impl WsiStress {
    #[cfg(test)]
    fn new() -> Self {
        Self::with_repeat(false)
    }
    fn with_repeat(repeat: bool) -> Self {
        Self {
            phase: StressPhase::Ready { after_presented: 0 },
            report: WsiStressReport::new(),
            repeat,
            accept_new: true,
        }
    }
    fn pending_action(&self) -> bool {
        !matches!(
            self.phase,
            StressPhase::Ready { .. } | StressPhase::Complete
        )
    }
    fn finish_if_quiescent(&mut self) {
        if !self.accept_new && !self.pending_action() {
            self.phase = StressPhase::Complete;
            self.report.completed = self.report.completed_batches > 0;
        }
    }
    fn stop_new_actions(&mut self) {
        self.accept_new = false;
        self.finish_if_quiescent();
    }
    fn ensure_completed(&self) -> Result<()> {
        if self.report.completed {
            Ok(())
        } else {
            Err(format!(
                "WSI stress incomplete: {}/{} observed resizes, {}/{} confirmed minimize/restore cycles",
                self.report.resize_completions, STRESS_RESIZES,
                self.report.completed_cycles, STRESS_CYCLES,
            ).into())
        }
    }
    /// Requests and observations are separate transitions. In particular,
    /// restore is never requested before a real zero extent has been observed.
    fn advance(
        &mut self,
        now: Instant,
        extent: (u32, u32),
        presented: u64,
    ) -> Result<Option<StressAction>> {
        let drawable = extent.0 != 0 && extent.1 != 0;
        let action: Result<Option<StressAction>> = match self.phase {
            StressPhase::Ready { after_presented }
                if self.accept_new && drawable && presented >= after_presented =>
            {
                let sizes = [
                    (760, 480),
                    (900, 540),
                    (820, 600),
                    (1060, 640),
                    (800, 500),
                    (980, 580),
                ];
                let requested = sizes[(self.report.resize_requests % sizes.len() as u64) as usize];
                self.report.resize_requests += 1;
                self.phase = StressPhase::Resize {
                    before: extent,
                    requested,
                    deadline: now + STRESS_TIMEOUT,
                };
                Ok(Some(StressAction::Resize(
                    requested.0 as i32,
                    requested.1 as i32,
                )))
            }
            StressPhase::Resize {
                before,
                requested,
                deadline,
            } => {
                if drawable && extent != before {
                    self.report.resize_completions += 1;
                    self.report
                        .record("resize", before, extent, Some(requested), presented);
                    if self
                        .report
                        .resize_completions
                        .is_multiple_of(STRESS_RESIZES / STRESS_CYCLES)
                        && self.accept_new
                    {
                        self.report.minimize_requests += 1;
                        self.phase = StressPhase::Minimize {
                            before: extent,
                            deadline: now + STRESS_TIMEOUT,
                        };
                        Ok(Some(StressAction::Minimize))
                    } else {
                        self.phase = StressPhase::Ready {
                            after_presented: presented + 1,
                        };
                        Ok(None)
                    }
                } else if now >= deadline {
                    Err("WSI stress resize request produced no changed nonzero extent".into())
                } else {
                    Ok(None)
                }
            }
            StressPhase::Minimize { before, deadline } => {
                if !drawable {
                    self.report.minimize_completions += 1;
                    self.report
                        .record("minimize", before, extent, None, presented);
                    self.phase = StressPhase::Minimized {
                        restore_at: now + STRESS_MINIMIZED_HOLD,
                    };
                    Ok(None)
                } else if now >= deadline {
                    Err("WSI stress minimize request never produced a zero extent".into())
                } else {
                    Ok(None)
                }
            }
            StressPhase::Minimized { restore_at } => {
                if drawable {
                    return Err(
                        "WSI stress window restored before the scripted restore request".into(),
                    );
                }
                if now >= restore_at {
                    self.report.restore_requests += 1;
                    self.phase = StressPhase::Restore {
                        deadline: now + STRESS_TIMEOUT,
                    };
                    Ok(Some(StressAction::Restore))
                } else {
                    Ok(None)
                }
            }
            StressPhase::Restore { deadline } => {
                if drawable {
                    self.report.restore_completions += 1;
                    self.report.completed_cycles += 1;
                    self.report
                        .record("restore", (0, 0), extent, None, presented);
                    if self
                        .report
                        .resize_completions
                        .is_multiple_of(STRESS_RESIZES)
                        && self.report.completed_cycles.is_multiple_of(STRESS_CYCLES)
                    {
                        self.report.completed_batches += 1;
                        if self.repeat && self.accept_new {
                            self.phase = StressPhase::Ready {
                                after_presented: presented + 1,
                            };
                        } else {
                            self.phase = StressPhase::Complete;
                            self.report.completed = true;
                        }
                    } else {
                        self.phase = StressPhase::Ready {
                            after_presented: presented + 1,
                        };
                    }
                    Ok(None)
                } else if now >= deadline {
                    Err("WSI stress restore request never produced a nonzero extent".into())
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        };
        let action = action?;
        self.finish_if_quiescent();
        Ok(action)
    }
}

struct Lab {
    last_sample: FrameSample,
    // Field drop order preserves session/resources before device and debug messenger.
    frame_loop: FrameLoop,
    image: Arc<Texture>,
    upload: Arc<Buffer>,
    readback: Arc<Buffer>,
    tokens: [u64; 3],
    scene: Option<RasterScene>,
    showcase: Option<super::showcase::Showcase>,
    performance: bool,
    transfer: Option<TransferScene>,
    transient: Option<TransientScene>,
    result_cache: Option<ResultScene>,
    multi_queue: Option<MultiQueueScene>,
    device: Arc<Device>,
    messenger: ValidationMessenger,
}
impl Lab {
    fn new(window: &platform::Window, report: &mut Report, options: &Options) -> Result<Self> {
        let require_validation = options.require_validation;
        let scene_kind = options.scene;
        let rendering = options.rendering;
        let pipeline_cache = &options.pipeline_cache;
        let mut builder = InstanceBuilder {
            application_name: CString::new("Rhapsody Window Lab")?,
            extensions: platform::required_extensions(),
            ..Default::default()
        };
        // ValidationMessenger is also used in performance mode with the layer off;
        // debug utils is the messenger transport, not the validation layer itself.
        builder.debug_markers();
        report.validation_enabled = if options.performance {
            false
        } else {
            builder.enable_validation()?
        };
        if require_validation && !report.validation_enabled {
            return Err("Khronos validation layer is required but unavailable".into());
        }
        let instance = builder.build()?;
        report.instance_api_version_raw = instance.api_version;
        let messenger = ValidationMessenger::new(&instance)?;
        let surface = window.create_surface(instance.clone())?;
        let device = rhapsody::device::Device::new(
            instance,
            Some(&surface),
            &rhapsody::device::DeviceConfig {
                validation: report.validation_enabled,
                frames_in_flight: if options.performance {
                    3
                } else {
                    rhapsody::device::DeviceConfig::default().frames_in_flight
                },
                allocation: options.allocation,
                queues: rhapsody::device::QueueConfig {
                    async_compute: options.queues,
                    transfer: options.queues,
                    require_present: true,
                },
                ..Default::default()
            },
        )?;
        let properties = unsafe {
            device
                .instance
                .instance
                .get_physical_device_properties(device.physical)
        };
        report.device = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        report.driver_version_raw = properties.driver_version;
        report.api_version_raw = properties.api_version;
        report.physical_api_version_raw = properties.api_version;
        report.enabled_device_extensions = device.enabled_extensions.clone();
        report.actual_queues = device
            .queue_plan
            .queues
            .iter()
            .map(|queue| PhysicalQueueReport {
                family: queue.family,
                index: queue.index,
            })
            .collect();
        report.class_queue_indices =
            rhapsody::device::QueueClass::ALL.map(|class| device.queue_plan.queue(class).index());
        let (width, height) = window.extent();
        let chain = Arc::new(unsafe {
            Swapchain::new(
                device.clone(),
                surface,
                vk::Extent2D { width, height },
                options.vsync,
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_DST,
            )
        }?);
        report.vsync_requested = options.vsync;
        report.present_mode = format!("{:?}", chain.present_mode());
        let features = unsafe {
            device
                .instance
                .instance
                .get_physical_device_format_properties(device.physical, chain.format)
        };
        if !features
            .optimal_tiling_features
            .contains(vk::FormatFeatureFlags::BLIT_DST)
        {
            return Err("swapchain format does not support blitting".into());
        }
        let source_features = unsafe {
            device
                .instance
                .instance
                .get_physical_device_format_properties(device.physical, vk::Format::R8G8B8A8_UNORM)
        };
        if !source_features
            .optimal_tiling_features
            .contains(vk::FormatFeatureFlags::BLIT_SRC)
        {
            return Err("RGBA8 source does not support blitting".into());
        }
        let mut desc = TextureDesc::new_2d(WIDTH, HEIGHT, vk::Format::R8G8B8A8_UNORM);
        desc.usage = vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST;
        if matches!(
            scene_kind,
            SceneKind::Raster | SceneKind::Temporal | SceneKind::MultiQueue | SceneKind::Showcase
        ) {
            desc.usage |= vk::ImageUsageFlags::STORAGE;
        }
        let image = Arc::new(Texture::new(device.clone(), &desc)?);
        let bytes = u64::from(WIDTH * HEIGHT * 4);
        let upload = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(bytes, vk::BufferUsageFlags::TRANSFER_SRC),
        )?);
        let readback = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(
                bytes,
                vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
            ),
        )?);
        let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original())?;
        if options.gpu_timing {
            backend.configure_gpu_timing(Some(rhapsody::rg::profiling::GpuTimingConfig {
                pass_spans: options.gpu_pass_timing,
                pending_commands: 2048,
                ..Default::default()
            }))?;
        }
        let tokens = [
            backend.bind_import(GpuResource::Buffer(upload.clone()))?,
            backend.bind_import(GpuResource::Image(image.clone()))?,
            backend.bind_import(GpuResource::Buffer(readback.clone()))?,
        ];
        let session = VulkanGraphSession::new(backend);
        let mut frame_loop = FrameLoop::new(session, chain)?;
        frame_loop
            .graph
            .chain
            .set_plan_cache_enabled(options.plan_cache);
        frame_loop
            .graph
            .chain
            .set_subgraph_cache_enabled(options.subgraph_cache);
        frame_loop
            .graph
            .set_result_cache_enabled(options.result_cache);
        let scene = if matches!(scene_kind, SceneKind::Raster | SceneKind::Temporal) {
            ensure_parent(pipeline_cache)?;
            Some(RasterScene::new(
                &mut frame_loop,
                scene_kind,
                rendering,
                pipeline_cache,
            )?)
        } else {
            None
        };
        let transfer = if scene_kind == SceneKind::Transfer {
            Some(TransferScene::new(&mut frame_loop)?)
        } else {
            None
        };
        let transient = if scene_kind == SceneKind::Transient {
            Some(TransientScene::new(
                &mut frame_loop,
                options.transient_aliasing,
                options.transient_attachments,
                options.transient_scale,
            )?)
        } else {
            None
        };
        let result_cache = if scene_kind == SceneKind::ResultCache {
            let mut scene = ResultScene::new(&mut frame_loop)?;
            scene.set_cache_policy(
                options.plan_cache,
                options.subgraph_cache,
                options.result_cache,
            );
            Some(scene)
        } else {
            None
        };
        let multi_queue = if scene_kind == SceneKind::MultiQueue {
            Some(MultiQueueScene::new(
                &mut frame_loop,
                rendering,
                pipeline_cache,
                options.sharing,
            )?)
        } else {
            None
        };
        let mut showcase = if scene_kind == SceneKind::Showcase {
            Some(super::showcase::Showcase::new(
                &mut frame_loop,
                pipeline_cache,
            )?)
        } else {
            None
        };
        if let Some(scene) = &mut showcase {
            scene.set_performance(options.performance);
        }
        Ok(Self {
            performance: options.performance,
            showcase,
            last_sample: FrameSample::default(),
            frame_loop,
            image,
            upload,
            readback,
            tokens,
            scene,
            transfer,
            transient,
            result_cache,
            multi_queue,
            device,
            messenger,
        })
    }

    fn build_graph(&mut self, pixels: &[u8], first: bool, frame: u64) -> Result<BuiltGraph> {
        if let Some(scene) = &mut self.showcase {
            return scene.build_graph(
                &mut self.frame_loop,
                &self.image,
                &self.readback,
                self.tokens,
                first,
                frame,
            );
        }
        if let Some(scene) = &mut self.multi_queue {
            return Ok(scene.build_graph(
                &mut self.frame_loop,
                &self.image,
                &self.readback,
                self.tokens,
                first,
                frame,
            )?);
        }
        if let Some(scene) = &mut self.transient {
            return Ok(scene.build_graph(
                &mut self.frame_loop,
                &self.image,
                &self.readback,
                self.tokens,
                first,
                frame,
            )?);
        }
        if let Some(scene) = &mut self.result_cache {
            return Ok(scene.build_graph(
                &mut self.frame_loop,
                &self.image,
                &self.readback,
                self.tokens,
                first,
                frame,
            )?);
        }
        if let Some(transfer) = &mut self.transfer {
            transfer.tick(frame)?;
            return Ok(transfer.build_graph(
                &mut self.frame_loop,
                &self.image,
                &self.readback,
                self.tokens,
                first,
            )?);
        }
        if let Some(scene) = &mut self.scene {
            return Ok(scene.build_graph(
                &mut self.frame_loop,
                &self.image,
                &self.readback,
                self.tokens,
                frame,
                first,
            )?);
        }
        // The production FrameLoop owns both the graph and display submission.
        self.upload.write_bytes(0, pixels)?;
        let mut graph = self.frame_loop.begin_frame()?;
        let upload = graph.import_unowned(
            "lab.upload",
            ResourceSpec::Buffer(self.upload.desc).descriptor(),
            self.tokens[0],
            HOST_WRITE,
        )?;
        let image = graph.import_unowned(
            "lab.image",
            ResourceSpec::Image(self.image.desc).descriptor(),
            self.tokens[1],
            if first { 0 } else { TRANSFER_READ },
        )?;
        let readback = graph.import_unowned(
            "lab.readback",
            ResourceSpec::Buffer(self.readback.desc).descriptor(),
            self.tokens[2],
            HOST_READ,
        )?;
        {
            let device = self.device.clone();
            let source = self.upload.clone();
            let target = self.image.clone();
            let mut pass = graph.add_pass("upload fixture");
            pass.add_resource_access(upload, TRANSFER_READ, 0)?;
            pass.add_resource_access(image, TRANSFER_WRITE, 0)?;
            pass.render_with_context(move |ctx: PassContext<'_>| {
                unsafe {
                    device.device.cmd_copy_buffer_to_image(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        source.handle,
                        target.handle,
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &[copy_region()],
                    );
                }
                true
            });
        }
        {
            let device = self.device.clone();
            let source = self.image.clone();
            let target = self.readback.clone();
            let mut pass = graph.add_pass("verify fixture");
            pass.add_resource_access(image, TRANSFER_READ, 0)?;
            pass.add_resource_access(readback, TRANSFER_WRITE, 0)?;
            pass.render_with_context(move |ctx: PassContext<'_>| {
                unsafe {
                    device.device.cmd_copy_image_to_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        source.handle,
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        target.handle,
                        &[copy_region()],
                    );
                }
                true
            });
        }
        graph.export(image, TRANSFER_READ)?;
        graph.export(readback, HOST_READ)?;
        Ok(graph.build())
    }

    fn present(
        &mut self,
        built: BuiltGraph,
        pixels: &[u8],
        output: &mut [u8],
        report: &mut Report,
    ) -> Result<bool> {
        let device = self.device.clone();
        let image = self.image.clone();
        let record = move |command: &mut VulkanCommandBuffer,
                           frame: &AcquiredImageLease|
              -> rhapsody::Result<()> {
            // The display submission reads the graph's real physical image.
            // Publish that exact access through the same submission guard.
            unsafe {
                rhapsody::raw::resource_access::image_access(
                    command,
                    image.clone(),
                    rhapsody::sync::QueueId::new(0).expect("graphics queue"),
                    rhapsody::resource::OwnershipRange::Image {
                        aspects: vk::ImageAspectFlags::COLOR.as_raw(),
                        base_mip: 0,
                        mip_count: 1,
                        base_layer: 0,
                        layer_count: 1,
                    },
                    rhapsody::resource::range_state::AccessScope {
                        stages: vk::PipelineStageFlags::TRANSFER,
                        access: vk::AccessFlags::TRANSFER_READ,
                        write: false,
                    },
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                )?;
            }
            let range = vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1);
            let layers = vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1);
            let to_dst = vk::ImageMemoryBarrier::default()
                .image(frame.image())
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .subresource_range(range);
            let to_present = vk::ImageMemoryBarrier::default()
                .image(frame.image())
                .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .subresource_range(range);
            let extent = frame.extent();
            let blit = vk::ImageBlit::default()
                .src_subresource(layers)
                .dst_subresource(layers)
                .src_offsets([
                    vk::Offset3D::default(),
                    vk::Offset3D {
                        x: WIDTH as i32,
                        y: HEIGHT as i32,
                        z: 1,
                    },
                ])
                .dst_offsets([
                    vk::Offset3D::default(),
                    vk::Offset3D {
                        x: extent.width as i32,
                        y: extent.height as i32,
                        z: 1,
                    },
                ]);
            unsafe {
                device.device.cmd_pipeline_barrier(
                    command.handle,
                    vk::PipelineStageFlags::TOP_OF_PIPE,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[to_dst],
                );
                device.device.cmd_blit_image(
                    command.handle,
                    image.handle,
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    frame.image(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[blit],
                    vk::Filter::NEAREST,
                );
                device.device.cmd_pipeline_barrier(
                    command.handle,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[to_present],
                );
            }
            Ok(())
        };
        let protocol_started = Instant::now();
        let executed = self
            .frame_loop
            .execute(built, record, vk::PipelineStageFlags::TRANSFER);
        let protocol_ns = protocol_started.elapsed().as_nanos() as u64;
        if let Some(scene) = &mut self.scene {
            scene.finish_execution(&executed);
        }
        let outcome = executed?;
        if outcome.status == FrameStatus::Recreated {
            report.recreate_events += 1;
        }
        let Some((graph, present)) = outcome.receipts else {
            return Ok(false);
        };
        report.application_gpu_memory = Some(GpuMemoryReport::sample(
            &self.device,
            &report.application_gpu_memory,
        ));
        let wsi = self.frame_loop.last_cpu_timings();
        self.last_sample = FrameSample {
            acquire_ns: wsi.acquire_ns,
            display_record_ns: wsi.display_record_ns,
            display_submit_present_ns: wsi.display_submit_present_ns,
            compile_ns: graph.stats.compile_ns,
            record_ns: graph.stats.record_ns,
            submit_ns: graph.stats.submit_ns,
            protocol_ns,
            ..Default::default()
        };
        let wait_started = Instant::now();
        if !self.performance {
            self.frame_loop.graph.wait_frame(graph.frame, TIMEOUT)?;
            self.frame_loop.wait_render(present, TIMEOUT)?;
        }
        self.last_sample.completion_wait_ns = wait_started.elapsed().as_nanos() as u64;
        if let Some(stats) = self.frame_loop.graph.backend.poll_gpu_timings()? {
            report.gpu_timing = Some(stats);
            for sample in self.frame_loop.graph.backend.drain_gpu_timings() {
                if graph.receipts.iter().any(|point| {
                    point.queue.index() == sample.queue && point.value == sample.completion_value
                }) {
                    if let Some(index) = sample.pass_index {
                        if report.gpu_pass_timings.len() == 1024 {
                            report.gpu_pass_timings.pop_front();
                            report.gpu_pass_timings_dropped += 1;
                        }
                        let name = graph
                            .snapshot
                            .pass_order
                            .get(index)
                            .ok_or("GPU pass timestamp has no matching frame declaration")?;
                        report.gpu_pass_timings.push_back(PassGpuTimingReport {
                            sample,
                            pass_name: name.clone(),
                        });
                        continue;
                    }
                    let total = &mut self.last_sample.gpu_command_spans_ns[sample.queue];
                    *total = Some(total.unwrap_or(0).saturating_add(sample.duration_ns));
                }
            }
        }
        if self.performance {
            return Ok(true);
        }
        // The graph's HOST_READ export and the display submit are complete.
        let readback_started = Instant::now();
        unsafe { self.readback.read_bytes_after_completion(0, output) }?;
        self.last_sample.readback_ns = readback_started.elapsed().as_nanos() as u64;
        if self.scene.is_none()
            && self.transfer.is_none()
            && self.transient.is_none()
            && self.result_cache.is_none()
            && self.multi_queue.is_none()
            && self.showcase.is_none()
            && output != pixels
        {
            return Err("GPU readback differs from the deterministic CPU fixture".into());
        }
        Ok(true)
    }
    fn drain_messages(&self, report: &mut Report) {
        for message in self.messenger.collector.all() {
            let text = format!("{}: {}", message.id_name, message.message);
            if message
                .severity
                .contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR)
            {
                report.validation_error_count += 1;
                if report.validation_errors.len() < 32 {
                    report.validation_errors.push(text);
                }
            } else {
                report.validation_warning_count += 1;
                if report.validation_warnings.len() < 32 {
                    report.validation_warnings.push(text);
                }
            }
        }
        self.messenger.collector.clear();
    }
}
fn copy_region() -> vk::BufferImageCopy {
    vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: WIDTH,
            height: HEIGHT,
            depth: 1,
        })
}
fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn run() -> Result<()> {
    let Some(options) = Options::parse()? else {
        return Ok(());
    };
    let git = |args: &[&str]| -> Option<String> {
        let output = std::process::Command::new("git")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args(["-c", "safe.directory=D:/game"])
            .args(args)
            .output()
            .ok()?;
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
    };
    let mut report = Report { schema_version: 1, milestone: match options.scene { SceneKind::Showcase => "continuous composite GPU acceptance", SceneKind::Fixture => "M0 / baseline fixture player", SceneKind::Transfer => "M4 / S07 bounded transfer acceptance", SceneKind::Transient => "M6 / S05 transient attachment chain", SceneKind::ResultCache => "M5 / S06 versioned result cache", SceneKind::MultiQueue => "M8 / S08 physical multi-queue dependency chain", _ => "M3 / S03 raster and S04 temporal acceptance" }.into(),
        performance_mode: options.performance,
        platform: std::env::consts::OS.into(), window_backend: platform::BACKEND.into(),
        git_revision_at_launch: git(&["rev-parse", "HEAD"]),
        git_dirty_at_launch: git(&["status", "--porcelain", "--untracked-files=normal", "--", "."]).map(|s| !s.is_empty()),
        command: std::env::args().collect(), requested_frames: options.frames, repeat_script: options.repeat_script,
        minimum_run_seconds: options.minimum_run_seconds.map(|duration| duration.as_secs_f64()),
        wsi_stress: options.wsi_stress.then(WsiStressReport::new),
        scene: SceneReport { scene: options.scene, rendering: options.rendering, gpu_program_hash: matches!(options.scene, SceneKind::Raster | SceneKind::Temporal).then(scene::program_fingerprint), ..Default::default() },
        window_visual_check: "pending; offscreen readback is verified separately".into(),
        limitations: vec!["Production FrameLoop submits the offscreen graph and display work with timeline dependencies".into(),
            "Per-frame CPU fixture generation and GPU readback are correctness checks, not renderer benchmarks".into(),
            "Production presentation requires maintenance1 and waits actual presentation fences at shutdown".into(),
            "GPU command timestamps require --gpu-timing on; opaque driver pool/pipeline memory is unavailable".into()], ..Default::default() };
    let started = Instant::now();
    let result = play(&options, &mut report, started);
    report.elapsed_seconds =
        (started.elapsed().as_secs_f64() - report.script_drain_seconds).max(0.0);
    if let Err(error) = &result {
        report.failure = Some(error.to_string());
    }
    report.status = if result.is_ok() { "pass" } else { "fail" }.into();
    ensure_parent(&options.report)?;
    std::fs::write(&options.report, serde_json::to_vec_pretty(&report)?)?;
    println!(
        "Window Lab {}: {} presented / {} verified; validation errors: {}; report: {}",
        report.status,
        report.presented_frames,
        report.pixel_verified_frames,
        report.validation_error_count,
        options.report.display()
    );
    result
}

fn play(options: &Options, report: &mut Report, launched_at: Instant) -> Result<()> {
    let playlist = Playlist::load(&options.playlist)?;
    report.fixture_hash = playlist.fingerprint.clone();
    let window = platform::Window::new()?;
    let initialization = Instant::now();
    let lab = Lab::new(&window, report, options);
    report.initialization_seconds = initialization.elapsed().as_secs_f64();
    let mut lab = lab?;
    let mut pixels = vec![0; (WIDTH * HEIGHT * 4) as usize];
    let mut output = pixels.clone();
    let mut timings = Measurements::new(options.warmup, options.samples);
    report.plan_cache_enabled = options.plan_cache;
    report.subgraph_cache_enabled = options.subgraph_cache;
    report.result_cache_enabled = options.result_cache;
    let (mut clip, mut frame) = (0usize, 0u32);
    let mut paused = false;
    let mut extent = window.extent();
    let mut script_phase = 0;
    let mut restore_at = None;
    let mut stress = options
        .wsi_stress
        .then(|| WsiStress::with_repeat(options.repeat_script));
    let mut drain_started = None;
    let mut native_drained_at = None;
    let mut play_stopped_at = None;
    let mut explicit_reset_done = false;
    let mut failed_frame_done = false;
    let mut scene_tick = 0;
    let start = Instant::now();
    let mut last_progress = Instant::now();
    let mut showcase_window_frame = 0;
    let result = (|| -> Result<()> {
        window.run(options.fps, |window, keys, redraw| -> Result<bool> {
            let elapsed = start.elapsed();
            let reached_limit = options.reached_limit(report.presented_frames, elapsed);
            if options.stop_new_wsi_actions(report.presented_frames, elapsed)
                && let Some(stress) = &mut stress {
                stress.stop_new_actions();
            }
            if reached_limit {
                play_stopped_at.get_or_insert(elapsed);
                if let Some(stress) = &mut stress { stress.stop_new_actions(); }
                if stress.as_ref().is_none_or(|stress| !stress.pending_action()) {
                    return Ok(false);
                }
                drain_started.get_or_insert_with(Instant::now);
            }
            if options.frames.is_some() && last_progress.elapsed() > Duration::from_secs(30) {
                return Err("no presented frame for 30 seconds".into());
            }
            if keys & platform::PAUSE != 0 {
                paused = !paused;
                report.pause_actions += 1;
            }
            if keys & platform::NEXT != 0 {
                clip = (clip + 1) % playlist.clips.len();
                frame = 0;
                report.next_actions += 1;
            }
            if keys & platform::STEP != 0 {
                report.step_actions += 1;
            }
            if options.script {
                match script_phase {
                    0 if report.presented_frames >= 30 => {
                        window.resize(800, 500)?;
                        script_phase = 1;
                    }
                    1 if report.presented_frames >= 60 => {
                        window.minimize();
                        restore_at = Some(Instant::now() + Duration::from_millis(250));
                        script_phase = 2;
                    }
                    2 if restore_at.is_some_and(|at| Instant::now() >= at) => {
                        window.restore();
                        script_phase = 3;
                    }
                    3 if report.presented_frames >= 120 => {
                        window.resize(1100, 680)?;
                        script_phase = 4;
                    }
                    _ => {}
                }
            }
            if !options.performance && options.scene==SceneKind::Showcase && !options.wsi_stress && !options.script {
                if restore_at.is_some_and(|at|Instant::now()>=at) { window.restore();restore_at=None; }
                let presented=report.presented_frames;
                if presented>0 && presented.is_multiple_of(180) && showcase_window_frame!=presented {
                    showcase_window_frame=presented;
                    if presented.is_multiple_of(900) {window.minimize();restore_at=Some(Instant::now()+Duration::from_millis(250));}
                    else {let (w,h)=if (presented/180).is_multiple_of(2){(960,600)}else{(1120,700)};window.resize(w,h)?;}
                }
            }
            if let Some(stress) = &mut stress
                && let Some(action) = stress.advance(
                    Instant::now(), window.extent(), report.presented_frames,
                )?
            {
                match action {
                    StressAction::Resize(width, height) => window.resize(width, height)?,
                    StressAction::Minimize => window.minimize(),
                    StressAction::Restore => window.restore(),
                }
            }
            let current_extent = window.extent();
            if current_extent != extent {
                extent = current_extent;
                lab.frame_loop.swapchain().resize(vk::Extent2D {
                    width: extent.0,
                    height: extent.1,
                })?;
                report.resize_events += 1;
                if let Some(scene) = &mut lab.showcase { scene.reset_history(&mut lab.frame_loop)?; }
                if let Some(scene) = &mut lab.scene { scene.reset_history(&mut lab.frame_loop, &mut report.scene, "window extent changed")?; }
            }
            // Drain native acknowledgements without presenting an extra frame.
            if reached_limit { return Ok(true); }
            if extent.0 == 0 || extent.1 == 0 {
                report.minimized_iterations += 1;
                return Ok(true);
            }
            // A minimized macOS window receives timer ticks but no redraws.
            // These ticks process restore/script state without submitting work.
            if !redraw {
                return Ok(true);
            }
            let frame_start = Instant::now();
            if let Some(scene) = &mut lab.scene { scene.advance_pipelines(report.presented_frames, options.shader_script, options.repeat_script, options.shader_script_frame_limit(), &mut report.scene)?; }
            if options.scene_script && report.presented_frames >= 60 && !explicit_reset_done {
                lab.scene.as_mut().expect("temporal scene").reset_history(&mut lab.frame_loop, &mut report.scene, "explicit scripted reset")?;
                explicit_reset_done = true;
            }
            let inject_failure = options.scene_script && report.presented_frames >= 90 && !failed_frame_done;
            if inject_failure { lab.scene.as_mut().expect("temporal scene").inject_record_failure(); }
            if options.scene == SceneKind::Fixture { playlist.render(clip, frame, &mut pixels); }
            let scene_frame = if matches!(options.scene, SceneKind::Transfer | SceneKind::Transient | SceneKind::ResultCache | SceneKind::Showcase) { report.presented_frames } else { scene_tick };
            let build_started = Instant::now();
            let graph = lab.build_graph(&pixels, report.presented_frames == 0, scene_frame)?;
            let build_ns = build_started.elapsed().as_nanos() as u64;
            match lab.present(graph, &pixels, &mut output, report) {
                Ok(false) => return Ok(true),
                Ok(true) if inject_failure => return Err("S04 injected graph recording failure was not observed".into()),
                Ok(true) => {},
                Err(error) if inject_failure && lab.scene.as_ref().expect("temporal scene").expected_record_failure(error.as_ref()) => {
                    let scene = lab.scene.as_mut().expect("temporal scene");
                    scene.mark_failed_frame(&mut report.scene);
                    scene.reset_history(&mut lab.frame_loop, &mut report.scene, "failed graph recording frame")?;
                    failed_frame_done = true;
                    return Ok(true);
                },
                Err(error) => return Err(error),
            }
            report.first_present_seconds.get_or_insert_with(|| launched_at.elapsed().as_secs_f64());
            let reference_started = Instant::now();
            if !options.performance {
            if matches!(options.scene, SceneKind::Raster | SceneKind::Temporal) {
                let scene = lab.scene.as_mut().expect("GPU scene");
                scene.verify_pixels(scene_frame, &output, &mut pixels, &mut report.scene)?;
                scene.verify_rendering(scene_frame, &output, &mut report.scene)?;
                scene.verify_history(&output, &mut report.scene)?;
            }
            if let Some(showcase) = &mut lab.showcase { showcase.verify(&output)?; report.showcase=Some(showcase.report.clone()); }
            if let Some(transfer) = &mut lab.transfer { transfer.verify_display(&output, &mut pixels)?; }
            if let Some(scene) = &mut lab.transient { scene.verify_display(&output, &mut pixels, &lab.frame_loop)?; }
            if let Some(scene) = &mut lab.result_cache { scene.verify_display(&output, &mut pixels)?; }
            if let Some(scene) = &mut lab.multi_queue { scene.verify_display(&output, &mut pixels, &lab.frame_loop)?; }
            }
            if options.performance && let Some(scene)=&mut lab.showcase {scene.collect_samples(&mut lab.frame_loop,false)?;}
            let reference_ns = reference_started.elapsed().as_nanos() as u64;
            if !options.performance {report.pixel_verified_frames += 1;}
            report.presented_frames += 1;
            report.memory_budget_usage = Some(BudgetReport::sample(&lab.device, &report.memory_budget_usage));
            report.application_gpu_memory = Some(GpuMemoryReport::sample(&lab.device, &report.application_gpu_memory));
            if report.presented_frames == 300 || report.presented_frames.is_multiple_of(1000) {
                report.sample_stability(&lab, start.elapsed(), "frame_complete");
            }
            if options.performance && report.presented_frames.is_multiple_of(120) && let Some(scene)=&mut lab.showcase {scene.sample(&mut lab.frame_loop,&lab.readback,scene_frame)?;}
            last_progress = Instant::now();
            report.visited_clips.insert(if options.scene == SceneKind::Fixture { playlist.clips[clip].name.clone() } else { options.scene.name().into() });
            lab.drain_messages(report);
            if report.validation_error_count > 0 {
                return Err("Vulkan validation reported errors (see report)".into());
            }
            timings.record(report.presented_frames, FrameSample { total_ns: frame_start.elapsed().as_nanos() as u64,
                build_ns, reference_ns, ..lab.last_sample });
            if report.presented_frames.is_multiple_of(15) {
                window.title(&format!("Rhapsody Window Lab | {} | {} | clip {} frame {} | {:.1} fps | validation {} | Space/N/Right/Esc",
                    if options.scene == SceneKind::Fixture { &playlist.clips[clip].name } else { options.scene.name() }, if paused { "PAUSED" } else { "PLAY" }, frame, report.presented_frames,
                    report.presented_frames as f64 / start.elapsed().as_secs_f64(), report.validation_enabled));
            }
            if !paused || keys & platform::STEP != 0 {
                scene_tick += 1;
                frame += 1;
                if frame >= playlist.clips[clip].frames {
                    clip = (clip + 1) % playlist.clips.len();
                    frame = 0;
                }
            }
            Ok(true)
        })?;
        let window_stopped_at = Instant::now();
        play_stopped_at.get_or_insert(window_stopped_at.saturating_duration_since(start));
        native_drained_at = Some(window_stopped_at);
        if let Some(stress) = &stress {
            stress.ensure_completed()?;
        }
        if let Some(transfer) = &lab.transfer {
            transfer.ensure_completed()?;
        }
        if let Some(scene) = &lab.transient {
            scene.ensure_completed()?;
        }
        if let Some(scene) = &lab.result_cache {
            scene.ensure_completed()?;
        }
        if let Some(scene) = &lab.multi_queue {
            scene.ensure_completed()?;
        }
        if options.shader_script {
            if options.minimum_run_seconds.is_some() && options.repeat_script {
                lab.scene
                    .as_mut()
                    .expect("raster scene")
                    .finish_minimum_run_shader_script(&mut report.scene)?;
            } else {
                lab.scene
                    .as_ref()
                    .expect("raster scene")
                    .ensure_shader_script_completed()?;
            }
        }
        if options.scene_script
            && (!explicit_reset_done
                || !failed_frame_done
                || report.scene.injected_failed_frames != 1
                || report.scene.failure_recovery_verified_frames != 1)
        {
            return Err(
                "S04 scene script did not complete reset, failed-frame and recovery checks".into(),
            );
        }
        if !options.accepts_completed_run(
            report.presented_frames,
            play_stopped_at.unwrap_or_else(|| start.elapsed()),
        ) {
            return Err(
                "window closed before requested frame count or minimum play duration".into(),
            );
        }
        report.script_completed = options.script
            && script_phase == 4
            && report.minimized_iterations > 0
            && report.resize_events >= 3;
        if options.script && !report.script_completed {
            return Err("window script did not finish".into());
        }
        if report.presented_frames == 0 {
            return Err("no frames presented".into());
        }
        if let Some(path) = &options.capture {
            ensure_parent(path)?;
            fixture::write_ppm(path, &output)?;
        }
        Ok(())
    })();
    report.play_seconds = play_stopped_at
        .unwrap_or_else(|| start.elapsed())
        .as_secs_f64();
    report.script_drain_seconds = drain_started.map_or(0.0, |at| {
        native_drained_at
            .unwrap_or_else(Instant::now)
            .saturating_duration_since(at)
            .as_secs_f64()
    });
    report.wsi_stress = stress.map(|stress| stress.report);
    // Collect diagnostics through teardown, while preserving the original failure.
    let collector = lab.messenger.collector.clone();
    let allocation_stats = lab.device.allocator_stats.clone();
    let retained_before = Swapchain::retained_generation_count();
    let showcase_cleanup = if let Some(scene) = &mut lab.showcase {
        let result = scene
            .shutdown(&mut lab.frame_loop)
            .map_err(|e| e.to_string());
        report.showcase = Some(scene.report.clone());
        result
    } else {
        Ok(())
    };
    let pipeline_cleanup = lab
        .scene
        .as_ref()
        .map_or(Ok(()), |scene| scene.save_pipeline_cache(&mut report.scene))
        .map_err(|e| e.to_string());
    let transfer_cleanup = lab
        .transfer
        .as_mut()
        .map_or(Ok(()), |transfer| transfer.shutdown(TIMEOUT))
        .map_err(|e| e.to_string());
    report.transfer = lab
        .transfer
        .as_ref()
        .map(|transfer| transfer.report.clone());
    let transient_cleanup = match &mut lab.transient {
        Some(scene) => scene.shutdown(&mut lab.frame_loop, lab.tokens, TIMEOUT),
        None => Ok(()),
    }
    .map_err(|error| error.to_string());
    report.transient = lab.transient.as_ref().map(|scene| scene.report.clone());
    let result_cleanup = lab
        .result_cache
        .as_mut()
        .map_or(Ok(()), |scene| scene.shutdown(TIMEOUT))
        .map_err(|error| error.to_string());
    report.result_cache_scene = lab.result_cache.as_ref().map(|scene| scene.report.clone());
    let multi_queue_cleanup = match &mut lab.multi_queue {
        Some(scene) => scene.shutdown(&mut lab.frame_loop, lab.tokens, TIMEOUT),
        None => Ok(()),
    }
    .map_err(|error| error.to_string());
    report.multi_queue_scene = lab.multi_queue.as_ref().map(|scene| scene.report.clone());
    let presentation_cleanup = lab.frame_loop.shutdown(TIMEOUT).map_err(|e| e.to_string());
    report.presentation_shutdown_complete = presentation_cleanup.is_ok();
    report.memory_budget_usage = Some(BudgetReport::sample(
        &lab.device,
        &report.memory_budget_usage,
    ));
    report.application_gpu_memory = Some(GpuMemoryReport::sample(
        &lab.device,
        &report.application_gpu_memory,
    ));
    report.sample_stability(&lab, start.elapsed(), "before_lab_drop");
    let cleanup = showcase_cleanup
        .and(pipeline_cleanup)
        .and(transfer_cleanup)
        .and(transient_cleanup)
        .and(result_cleanup)
        .and(multi_queue_cleanup)
        .and(presentation_cleanup);
    lab.drain_messages(report);
    drop(lab);
    let allocations = allocation_stats.report();
    report.teardown_memory = Some(TeardownMemoryReport {
        scope: "Registered resource ranges and budgeted CPU storage after Lab/device owners drop; excludes opaque driver and operating-system allocations.",
        registered_allocations: allocations.allocations.len(),
        registered_resource_bytes: allocations.totals.total_allocated_bytes,
        known_cpu_bytes: allocation_stats.cpu_memory_usage().total(),
        registered_budget_bytes: allocation_stats.budget_usage().total(),
    });
    report.retained_wsi_generations = Swapchain::retained_generation_count() - retained_before;
    for error in collector.errors() {
        report.validation_error_count += 1;
        if report.validation_errors.len() < 32 {
            report
                .validation_errors
                .push(format!("{}: {}", error.id_name, error.message));
        }
    }
    report.source_hash = format!("{:016x}", fixture::hash(&pixels));
    report.readback_hash = format!("{:016x}", fixture::hash(&output));
    let performance = timings.report();
    report.cpu_frame_ms_p50 = performance.total_frame.p50_ms;
    report.cpu_frame_ms_p95 = performance.total_frame.p95_ms;
    report.cpu_frame_ms_p99 = performance.total_frame.p99_ms;
    report.performance = Some(performance);
    result?;
    cleanup?;
    if !report
        .teardown_memory
        .as_ref()
        .is_some_and(TeardownMemoryReport::released)
    {
        return Err("registered resource or CPU budget remains after teardown (see report)".into());
    }
    if report.retained_wsi_generations != 0 {
        return Err("WSI teardown retained unproven generations".into());
    }
    if report.validation_error_count > 0 {
        return Err("Vulkan validation reported teardown errors".into());
    }
    Ok(())
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn repeat_script_requires_a_script_and_preserves_the_baseline_default() {
        let parse = |args: &[&str]| Options::parse_from(args.iter().map(|arg| (*arg).to_owned()));
        assert!(parse(&["--scene", "fixture", "--repeat-script"]).is_err());
        let showcase = parse(&[]).unwrap().unwrap();
        assert_eq!(showcase.scene, SceneKind::Showcase);
        assert!(!showcase.wsi_stress && !showcase.repeat_script);
        assert_eq!(showcase.frames, None);
        assert!(matches!(
            showcase.queues,
            rhapsody::device::QueuePreference::Prefer
        ));
        let original = parse(&["--scene", "fixture", "--wsi-stress"])
            .unwrap()
            .unwrap();
        assert!(!original.repeat_script);
        assert_eq!(original.frames, Some(360));
        let repeated = parse(&["--scene", "fixture", "--wsi-stress", "--repeat-script"])
            .unwrap()
            .unwrap();
        assert!(repeated.repeat_script);
        let shader = parse(&["--scene", "raster", "--shader-script", "--repeat-script"])
            .unwrap()
            .unwrap();
        assert!(shader.repeat_script);
    }

    #[test]
    fn minimum_run_requires_finite_frames_and_both_limits_before_stopping() {
        let parse = |args: &[&str]| Options::parse_from(args.iter().map(|arg| (*arg).to_owned()));
        assert!(parse(&["--min-run-seconds", "2"]).is_err());
        for invalid in ["0", "-1", "NaN", "inf", "1e-12", "1e30"] {
            assert!(parse(&["--frames", "10000", "--min-run-seconds", invalid]).is_err());
        }
        let options = parse(&[
            "--frames",
            "10000",
            "--min-run-seconds",
            "2",
            "--wsi-stress",
            "--repeat-script",
        ])
        .unwrap()
        .unwrap();
        assert!(!options.reached_limit(9999, Duration::from_secs(3)));
        assert!(!options.reached_limit(10000, Duration::from_millis(1999)));
        assert!(options.reached_limit(10000, Duration::from_secs(2)));
        assert!(options.reached_limit(10001, Duration::from_secs(2)));
        assert!(!options.stop_new_wsi_actions(10000, Duration::from_secs(1)));
        assert!(!options.stop_new_wsi_actions(9999, Duration::from_secs(3)));
        assert!(options.stop_new_wsi_actions(10001, Duration::from_secs(2)));
        assert!(options.accepts_completed_run(10001, Duration::from_secs(2)));
        assert!(!options.accepts_completed_run(10001, Duration::from_secs(1)));
        assert_eq!(options.shader_script_frame_limit(), None);
        let original = parse(&["--frames", "360", "--wsi-stress", "--repeat-script"])
            .unwrap()
            .unwrap();
        assert!(!original.reached_limit(359, Duration::from_secs(999)));
        assert!(original.reached_limit(360, Duration::ZERO));
        assert!(!original.accepts_completed_run(361, Duration::from_secs(999)));
        assert!(original.stop_new_wsi_actions(344, Duration::ZERO));
        assert_eq!(original.shader_script_frame_limit(), Some(360));
    }

    #[test]
    fn repeated_stress_counts_whole_batches_and_bounds_diagnostic_storage() {
        let mut stress = WsiStress::with_repeat(true);
        let mut now = Instant::now();
        let mut extent = (2000, 1240);
        let mut last_nonzero = extent;
        let mut presented = 0;
        for _ in 0..5000 {
            match stress.advance(now, extent, presented).unwrap() {
                Some(StressAction::Resize(width, height)) => {
                    extent = (width as u32 * 2, height as u32 * 2)
                }
                Some(StressAction::Minimize) => {
                    last_nonzero = extent;
                    extent = (0, 0);
                }
                Some(StressAction::Restore) => extent = last_nonzero,
                None => {}
            }
            if extent != (0, 0) {
                presented += 1;
            }
            now += Duration::from_millis(16);
            if stress.report.completed_batches == 2 {
                stress.stop_new_actions();
                break;
            }
        }
        stress.ensure_completed().unwrap();
        assert_eq!(stress.report.resize_requests, 200);
        assert_eq!(stress.report.resize_completions, 200);
        assert_eq!(stress.report.completed_cycles, 40);
        assert_eq!(
            stress.report.minimize_requests,
            stress.report.restore_completions
        );
        assert_eq!(stress.report.events.len(), 140);
        assert_eq!(stress.report.diagnostic_events_dropped, 140);
        assert_eq!(stress.advance(now, extent, presented).unwrap(), None);
        assert_eq!(stress.report.resize_requests, 200);
    }

    #[test]
    fn stopping_new_actions_drains_an_existing_minimize_without_presenting() {
        let mut stress = WsiStress::with_repeat(true);
        let mut now = Instant::now();
        let mut extent = (2000, 1240);
        for frame in 0..100 {
            match stress.advance(now, extent, frame).unwrap() {
                Some(StressAction::Resize(width, height)) => {
                    extent = (width as u32 * 2, height as u32 * 2)
                }
                Some(StressAction::Minimize) => break,
                Some(StressAction::Restore) => panic!("restore before minimize"),
                None => {}
            }
            now += Duration::from_millis(16);
        }
        stress.stop_new_actions();
        assert!(stress.pending_action());
        let presented = 100;
        assert_eq!(stress.advance(now, (0, 0), presented).unwrap(), None);
        now += STRESS_MINIMIZED_HOLD;
        assert_eq!(
            stress.advance(now, (0, 0), presented).unwrap(),
            Some(StressAction::Restore)
        );
        assert_eq!(stress.advance(now, extent, presented).unwrap(), None);
        assert!(!stress.pending_action());
        assert_eq!(stress.report.completed_cycles, 1);
        assert_eq!(
            stress.report.resize_requests,
            stress.report.resize_completions
        );
        assert_eq!(stress.advance(now, extent, presented).unwrap(), None);
        assert!(
            stress.ensure_completed().is_err(),
            "a partial first batch is never PASS"
        );
    }

    #[test]
    fn stress_finishes_only_after_real_extent_acknowledgements_for_all_actions() {
        let mut stress = WsiStress::new();
        let mut now = Instant::now();
        let mut extent = (2000, 1240);
        let mut last_nonzero = extent;
        let mut presented = 0;
        for _ in 0..2000 {
            let old_resizes = stress.report.resize_completions;
            let old_minimizes = stress.report.minimize_completions;
            let old_restores = stress.report.restore_completions;
            match stress.advance(now, extent, presented).unwrap() {
                Some(StressAction::Resize(width, height)) => {
                    // A request is not an acknowledgement. Simulate the native
                    // Retina-size event arriving before the following tick.
                    assert_eq!(stress.report.resize_completions, old_resizes);
                    extent = (width as u32 * 2, height as u32 * 2);
                }
                Some(StressAction::Minimize) => {
                    assert_eq!(stress.report.minimize_completions, old_minimizes);
                    last_nonzero = extent;
                    extent = (0, 0);
                }
                Some(StressAction::Restore) => {
                    assert_eq!(stress.report.restore_completions, old_restores);
                    extent = last_nonzero;
                }
                None => {}
            }
            if extent != (0, 0) {
                presented += 1;
            }
            now += Duration::from_millis(16);
            if stress.report.completed {
                break;
            }
        }
        stress.ensure_completed().unwrap();
        assert_eq!(stress.report.resize_requests, 100);
        assert_eq!(stress.report.resize_completions, 100);
        assert_eq!(stress.report.minimize_requests, 20);
        assert_eq!(stress.report.minimize_completions, 20);
        assert_eq!(stress.report.restore_requests, 20);
        assert_eq!(stress.report.restore_completions, 20);
        assert_eq!(stress.report.completed_cycles, 20);
        assert_eq!(stress.report.events.len(), 140);
        for event in &stress.report.events {
            match event.kind {
                "resize" => assert_ne!(event.from_extent, event.observed_extent),
                "minimize" => assert_eq!(event.observed_extent, [0, 0]),
                "restore" => {
                    assert_eq!(event.from_extent, [0, 0]);
                    assert_ne!(event.observed_extent, [0, 0]);
                }
                _ => panic!("unexpected stress event"),
            }
        }
    }

    #[test]
    fn ignored_native_resize_never_counts_as_completion_or_success() {
        let mut stress = WsiStress::new();
        let start = Instant::now();
        let extent = (2000, 1240);
        assert!(matches!(
            stress.advance(start, extent, 0).unwrap(),
            Some(StressAction::Resize(_, _))
        ));
        for frame in 1..10 {
            assert_eq!(
                stress
                    .advance(start + Duration::from_millis(frame), extent, frame)
                    .unwrap(),
                None
            );
        }
        assert_eq!(stress.report.resize_requests, 1);
        assert_eq!(stress.report.resize_completions, 0);
        assert!(stress.report.events.is_empty());
        assert!(stress.advance(start + STRESS_TIMEOUT, extent, 10).is_err());
        assert!(stress.ensure_completed().is_err());
    }

    #[test]
    fn restore_waits_for_zero_extent_and_requires_a_later_nonzero_acknowledgement() {
        let mut stress = WsiStress::new();
        let mut now = Instant::now();
        let mut extent = (2000, 1240);
        for presented in 0..100 {
            match stress.advance(now, extent, presented).unwrap() {
                Some(StressAction::Resize(width, height)) => {
                    extent = (width as u32 * 2, height as u32 * 2);
                }
                Some(StressAction::Minimize) => break,
                Some(StressAction::Restore) => panic!("restore before a minimize observation"),
                None => {}
            }
            now += Duration::from_millis(16);
        }
        assert_eq!(stress.report.minimize_requests, 1);
        assert_eq!(stress.report.minimize_completions, 0);
        assert_eq!(stress.advance(now, extent, 100).unwrap(), None);
        assert_eq!(stress.report.restore_requests, 0);
        now += Duration::from_millis(10);
        assert_eq!(stress.advance(now, (0, 0), 100).unwrap(), None);
        assert_eq!(stress.report.minimize_completions, 1);
        assert_eq!(stress.report.restore_requests, 0);
        now += STRESS_MINIMIZED_HOLD;
        assert_eq!(
            stress.advance(now, (0, 0), 100).unwrap(),
            Some(StressAction::Restore)
        );
        assert_eq!(stress.report.restore_completions, 0);
        assert_eq!(stress.advance(now, (0, 0), 100).unwrap(), None);
        assert_eq!(stress.report.completed_cycles, 0);
        assert_eq!(stress.advance(now, extent, 100).unwrap(), None);
        assert_eq!(stress.report.restore_completions, 1);
        assert_eq!(stress.report.completed_cycles, 1);
    }
}
