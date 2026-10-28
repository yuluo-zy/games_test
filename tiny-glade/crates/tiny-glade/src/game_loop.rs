//! B main-loop event/frame consumers with an explicit macOS host adapter.
//!
//! Original anchors: new 0x1404d8100, run 0x1404d8250, main_loop
//! 0x1404d9580 (36,589-byte RNE range), monitor enumeration 0x1404e2470,
//! settings gathering 0x1404e3380. The OS contract and original jump tables are
//! in task-index/work/TG-40ff43c62ef8791c53/oschain-20261018/CONTRACT.md.
//! Source ownership here does not assert the original native GameLoop layout.
//! Full input-file/Mei ABI branches stay explicit errors or gaps,
//! rather than silently generating substitute state.

use anyhow::Context;
use std::sync::Mutex;

use bevy_app::App;
use bevy_ecs::world::World;

use country_core::render::render_config::{MonitorMatch, VideoMode};
use country_core::resources::window::{AvailableMonitor, AvailableMonitors};
use winit::event_loop::EventLoopWindowTarget;

use crate::egui_backend::EguiBackend;
use crate::render_loop::RenderMode;
use crate::setup::FromWinit;

/// Only an unwinding host can return a panic here. The normal launch installs
/// panic_reporter::setup_panic_hook, which reports and aborts before unwinding;
/// this boundary neither overrides that hook nor promises orderly panic exit.
fn catch_host_callback<T>(label: &str, operation: impl FnOnce() -> anyhow::Result<T>) -> anyhow::Result<T> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation))
        .unwrap_or_else(|payload| {
            let message = payload.downcast_ref::<String>().map(String::as_str)
                .or_else(|| payload.downcast_ref::<&'static str>().copied())
                .unwrap_or("non-string panic payload");
            Err(anyhow::anyhow!("{label} panicked: {message}"))
        })
}

/// Main-thread host lifecycle. An exit request is terminal even when GPU
/// cleanup needs another attempt; queued OS callbacks must not use that GPU.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum LoopPhase {
    #[default]
    Running,
    Stopping,
    Stopped,
}

#[derive(Default)]
pub(crate) struct LoopLifecycle {
    phase: std::cell::Cell<LoopPhase>,
    cpu_cleanup_started: std::cell::Cell<bool>,
    first_failure: std::cell::RefCell<Option<anyhow::Error>>,
}

impl LoopLifecycle {
    pub(crate) fn is_running(&self) -> bool {
        self.phase.get() == LoopPhase::Running
    }

    /// Preserve the original error object, including its typed source chain.
    pub(crate) fn record_failure(&self, error: anyhow::Error) {
        self.request_stop();
        let mut first = self.first_failure.borrow_mut();
        if first.is_none() {
            log::error!("event-loop failure: {error:#}");
            *first = Some(error);
        } else {
            log::error!("event-loop cleanup after original failure: {error:#}");
        }
    }

    pub(crate) fn take_failure(&self) -> Option<anyhow::Error> {
        self.first_failure.borrow_mut().take()
    }

    fn request_stop(&self) {
        if self.is_running() {
            self.phase.set(LoopPhase::Stopping);
        }
    }

    /// Called before dispatching any event, not just RedrawRequested. Returned
    /// errors always stop further callbacks. A panic can take this path only
    /// in hosts whose panic hook permits unwinding, including the test runner.
    pub(crate) fn dispatch(&self, operation: impl FnOnce() -> anyhow::Result<()>) -> anyhow::Result<()> {
        if !self.is_running() {
            return Ok(());
        }
        let result = catch_host_callback("event-loop callback", operation);
        if result.is_err() {
            self.request_stop();
        }
        result
    }

    /// CPU owners are consumed once. A failed renderer shutdown can be retried
    /// by LoopExiting/Drop without rerunning screenshot writes or save cleanup.
    fn begin_cleanup(&self) -> Option<bool> {
        self.request_stop();
        if self.phase.get() == LoopPhase::Stopped {
            None
        } else {
            Some(!self.cpu_cleanup_started.replace(true))
        }
    }

    fn renderer_stopped(&self) {
        self.phase.set(LoopPhase::Stopped);
    }
}

/// Opt-in wall-clock attribution only. Disabled runs do not sample Instant or
/// change pacing, render resolution, frame gates, or the original workload.
pub(crate) fn frame_timing_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("TG_FRAME_TIMING").is_some_and(|value| value == "1"))
}

/// Sampling interval for the per-stage log lines. `TG_FRAME_TIMING_EVERY=1`
/// logs every gameplay frame (needed for p95/p99); the default stays 30.
pub(crate) fn frame_timing_every() -> u64 {
    static EVERY: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *EVERY.get_or_init(|| {
        std::env::var("TG_FRAME_TIMING_EVERY")
            .ok()
            .and_then(|value| value.trim().parse::<u64>().ok())
            .filter(|every| *every > 0)
            .unwrap_or(30)
    })
}

/// B 0x1404de6b4..0x1404de7da. An open, throttleable menu limits
/// positive configured FPS to 60; disabled/NaN limits become 60 in that branch.
/// Profiling bypasses that menu cap. Missing menu/commander resources have
/// different behavior: no menu means no cap, no commander means not profiling.
pub fn frame_target_fps(world: &World) -> f32 {
    use country_core::{
        game_config::GameConfig, meta_commander::MetaCommander, resources::gui::EscMenuState,
    };

    let fps = world.resource::<GameConfig>().video.fps_limit;
    let menu_limits = world
        .get_resource::<EscMenuState>()
        .is_some_and(|menu| menu.is_open() && menu.can_limit_frame_rate());
    let profiling = world
        .get_resource::<MetaCommander>()
        .is_some_and(MetaCommander::is_profiling);
    apply_menu_fps_limit(fps, menu_limits, profiling)
}

/// B 0x1404de4f9..0x1404de587 tests the *presence* of MainCamera
/// (native TypeId bba451ee6a08a3ff/8d008d15febcf29d), then doubles that
/// boolean to form render's mode byte. No guessed AppLoadingState tag is used.
pub fn frame_render_mode(world: &World) -> RenderMode {
    if world.contains_resource::<country_core::systems::camera_rig::main_camera::MainCamera>() {
        RenderMode::WorldGpuProfiled
    } else {
        RenderMode::LoadingScreen
    }
}

/// B 1404ddf28..3c: absent DebugGui has the default-input behavior.
pub fn frame_egui_input_enabled(world: &World) -> bool {
    world.get_resource::<country_core::resources::gui::DebugGui>()
        .is_some_and(|debug| debug.enabled || debug.experimental_window)
}

/// B 1404dc863..971 读取当前 FrameRenderConfig.extent_0，而非另一份 WindowSize。
fn frame_waits_for_window_extent(world: &World) -> bool {
    world.resource::<country_core::render::render_config::FrameRenderConfig>()
        .extent_0.contains(&0)
}

fn apply_menu_fps_limit(fps: f32, menu_limits: bool, profiling: bool) -> f32 {
    if menu_limits && !profiling {
        if fps > 0.0 { fps.min(60.0) } else { 60.0 }
    } else {
        fps
    }
}

/// Read-only counters from the real production frame path. A gameplay frame
/// requires the Game state and MainCamera in the App actually updated/rendered.
/// All counts cover only frames whose render and host checks succeeded. The
/// submission/present fields are that successful-frame subset, not total GPU
/// operations: a failed frame may already have submitted work. They do not
/// assert nonempty scene data or scanout.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FrameProgress {
    pub completed_frames: u64,
    pub gameplay_frames: u64,
    pub gameplay_submissions: u64,
    pub gameplay_presents_queued: u64,
}

struct PendingFrameTiming {
    started: std::time::Instant,
    update: std::time::Duration,
    yielded_at: Option<std::time::Instant>,
    yielded: std::time::Duration,
}

/// The recovered update/render section of main_loop. This owns real Bevy/egui
/// state; it is not a native-layout mirror of the 0x1118-byte GameLoop.
pub struct GameFrameDriver {
    pending_timing: Option<PendingFrameTiming>,
    app: App,
    egui: EguiBackend,
    /// Whether the frame now on the render thread is a gameplay frame; its
    /// submission/present counts arrive with the next hand-over.
    in_flight_gameplay: bool,
    /// Latched at hand-overs, so the event loop never waits on the render thread.
    any_present_successful: bool,
    /// Game-world side of the hand-over (the render side's write-backs live
    /// in the renderer); rebuilt with an empty render world per App.
    extract: Option<crate::render_thread::ExtractSet>,
    limiter: crate::fps_limiter::FpsLimiter,
    progress: FrameProgress,
    /// Opt-in evidence through the existing post-UI screenshot owner. Never
    /// changes the reveal, menu state, resolution, or loading completion gate.
    startup_ui_capture: Option<std::path::PathBuf>,
    startup_ui_capture_elapsed: f32,
    startup_ui_capture_saw_reveal: bool,
    loading_render_pending_after_update: bool,
    input_consumed_this_call: bool,
    awaiting_render: bool,
    waiting_for_collect: bool,
    pub(crate) waiting_for_preparation: bool,
    pending_collect_input: Option<egui::RawInput>,
    in_flight_temporal: Option<crate::temporal_frame::TemporalCommit>,
    pub(crate) late_camera: crate::temporal_frame::LateCameraMailbox,
    late_camera_frame: Option<crate::temporal_frame::LateCameraFrame>,
}

#[derive(bevy_ecs::resource::Resource)]
struct AssetLibrarySystemsInstalled;

/// Both independent Apps receive the same registration once, before either
/// startup path can execute. The gameplay SubApp keeps it through warmup/move.
fn install_asset_library_systems(app: &mut App) {
    if app.world().contains_resource::<AssetLibrarySystemsInstalled>() {
        return;
    }
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::asset_libraries::runtime::{build_missing_vaos, rebuild_vaos};
    // 独立加载 App 可能尚无 PostUpdate；初始化缺失调度，不覆盖已有系统。
    app.init_schedule(bevy_app::PostUpdate);
    country_core::asset_libraries::runtime::add_asset_library_post_update_systems(
        app.get_schedule_mut(bevy_app::PostUpdate).expect("App 缺少 PostUpdate 调度"),
    );
    // The one explicit barrier in the Nani chain applies these commands too.
    // Loading has no Nani consumer and uses real Update schedule completion.
    app.add_systems(bevy_app::Update, (build_missing_vaos, rebuild_vaos)
        .chain_ignore_deferred()
        .before_ignore_deferred(country_core::render::nani::systems::nani_maintain_instances_ecs)
        .in_set(crate::run_game::GameSystemSet(44)));
    app.insert_resource(AssetLibrarySystemsInstalled);
}

fn update_application(app: &mut App) -> anyhow::Result<()> {
    // The asynchronous preparation already has this unwind boundary. The
    // synchronous skip-splash path must also report a real startup failure
    // before platform output, render or any completed-frame side effect.
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| app.update()))
        .map_err(|payload| {
            let message = if let Some(message) = payload.downcast_ref::<String>() {
                message.as_str()
            } else if let Some(message) = payload.downcast_ref::<&'static str>() {
                *message
            } else {
                "non-string panic payload"
            };
            anyhow::anyhow!("application startup/update panicked: {message}")
        })?;
    // Bind this requirement to the actual gameplay World, not a camera or an
    // early Splash/Game/Reload state. SubApp handoff retains the same role.
    if app.world().contains_resource::<crate::run_game::GameplayApplication>() {
        crate::gameplay_startup::require_clutter_catalog(app.world())?;
    }
    Ok(())
}

// Called only after the old frame settles and before this World is extracted.
// A rejected producer range is retried in place; no new App update is needed.
fn admit_producer_uploads(
    world: &mut bevy_ecs::world::World,
    device: &dyn country_core::render::ssbo::rhapsody_boundary::GraphicsDevice,
) -> rhapsody::Result<()> {
    if let Some(mut library) = world.get_resource_mut::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>() {
        library.flush_pending_uploads(device)?;
    }
    if let Some(mut gpu) = world.get_resource_mut::<country_core::render::nani::gpu_world::GpuWorld>() {
        if gpu.instance_uploads_pending() { gpu.flush_instance_buffers(device)?; }
    }
    Ok(())
}

impl GameFrameDriver {
    pub fn new(mut app: App, context: egui::Context) -> Self {
        // GUI consumers and the backend must see the same Context identity.
        app.world_mut()
            .insert_resource(country_core::resources::gui::EguiContextResource(
                context.clone(),
            ));
        install_asset_library_systems(&mut app);
        app.insert_resource(country_core::systems::backend::prepare_frame_constants::DeferredTemporalHistory);
        Self {
            pending_timing: None,
            app,
            egui: EguiBackend::new(context),
            in_flight_gameplay: false,
            any_present_successful: false,
            extract: None,
            limiter: crate::fps_limiter::FpsLimiter::new(),
            progress: FrameProgress::default(),
            startup_ui_capture: std::env::var_os("TG_STARTUP_UI_CAPTURE")
                .filter(|path| !path.is_empty()).map(std::path::PathBuf::from),
            startup_ui_capture_elapsed: 0.0,
            startup_ui_capture_saw_reveal: false,
            loading_render_pending_after_update: false,
            input_consumed_this_call: false,
            awaiting_render: false,
            waiting_for_collect: false,
            waiting_for_preparation: false,
            pending_collect_input: None,
            in_flight_temporal: None,
            late_camera: Default::default(),
            late_camera_frame: None,
        }
    }

    pub fn app(&self) -> &App {
        &self.app
    }
    pub fn app_mut(&mut self) -> &mut App {
        &mut self.app
    }

    /// A native present has succeeded, as of the last settled frame.
    pub fn any_present_successful(&self) -> bool {
        self.any_present_successful
    }

    pub(crate) fn awaiting_render(&self) -> bool { self.awaiting_render }

    pub fn progress(&self) -> FrameProgress {
        self.progress
    }

    /// Wait for the frame on the render thread (if any) and apply its results
    /// to the game world: screenshot bookkeeping, progress counts, errors.
    /// Start a fresh render world and hand-over for the current App (first
    /// frame, and after an App switch: entity ids belong to one World).
    pub fn reset_render_exchange(
        &mut self,
        render: &mut crate::production_renderer::ProductionRenderThread,
    ) {
        self.waiting_for_preparation = false;
        self.waiting_for_collect = false;
        self.app.insert_resource(country_core::systems::backend::prepare_frame_constants::DeferredTemporalHistory);
        let exchange = crate::render_extract::production_exchange();
        let side = render.reclaim();
        side.world = World::new();
        render.set_write_back(exchange.write_back);
        self.extract = Some(exchange.extract);
    }

    pub fn settle_render(
        &mut self,
        render: &mut crate::production_renderer::ProductionRenderThread,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(!render.failed(), "render worker failed; no frame feedback remains");
        let feedback = render.apply_feedback(self.app.world_mut());
        self.any_present_successful |= render.any_present_successful();
        if let Some(candidate) = self.in_flight_temporal.take() {
            candidate.finish(self.app.world_mut(), feedback.history_submitted, feedback.temporal_candidate);
        }
        if let Some(error) = feedback.error {
            return Err(error);
        }
        if feedback.rendered && self.in_flight_gameplay {
            if feedback.submitted {
                self.progress.gameplay_submissions =
                    self.progress.gameplay_submissions.wrapping_add(1);
            }
            if feedback.present_queued {
                self.progress.gameplay_presents_queued =
                    self.progress.gameplay_presents_queued.wrapping_add(1);
            }
        }
        Ok(())
    }

    fn update_before_render(
        &mut self,
        input: Option<egui::RawInput>,
        publish_platform_output: impl FnOnce(egui::PlatformOutput),
    ) -> anyhow::Result<()> {
        self.input_consumed_this_call = false;
        if self.loading_render_pending_after_update || self.awaiting_render {
            return Ok(());
        }
        let has_gui_input = input.is_some();
        let consumed_input = self.late_camera.mark();
        if let Some(input) = input {
            self.egui.context().begin_frame(input);
        }
        {
            puffin::profile_scope!("App update");
            update_application(&mut self.app)?;
        }
        self.late_camera_frame = crate::temporal_frame::LateCameraFrame::capture(
            self.app.world(), &self.late_camera, consumed_input);
        self.input_consumed_this_call = true;
        if has_gui_input {
            publish_platform_output(self.egui.end_frame());
        }
        Ok(())
    }

    fn update_if_admitted(
        &mut self,
        input: Option<egui::RawInput>,
        publish_platform_output: impl FnOnce(egui::PlatformOutput),
    ) -> anyhow::Result<bool> {
        if !self.awaiting_render && !self.loading_render_pending_after_update {
            let input = match (self.pending_collect_input.take(), input) {
                (Some(mut pending), Some(newer)) => { pending.append(newer); Some(pending) }
                (pending, newer) => pending.or(newer),
            };
            // No Collect admission: producers never wait on the render
            // thread's read lease, so Update runs alongside shared-pool Collect.
            self.waiting_for_collect = false;
            self.update_before_render(input, publish_platform_output)?;
        } else {
            self.update_before_render(input, publish_platform_output)?;
        }
        Ok(true)
    }

    /// B begin_frame@0x1404de115 → App::update@0x1404de349 →
    /// end_frame@0x1404de3d2 → platform output send@0x1404de464 →
    /// render@0x1404de5bc → first-frame log@0x1404de640 → limiter@0x1404de7da.
    ///
    /// input=None follows the original no-egui branch: update/render still run,
    /// without a begin/end pair or a platform message. OS events and Steam
    /// callbacks must be resolved by the full host loop.
    pub fn frame(
        &mut self,
        input: Option<egui::RawInput>,
        render: &mut crate::production_renderer::ProductionRenderThread,
        publish_platform_output: impl FnOnce(egui::PlatformOutput),
    ) -> anyhow::Result<()> {
        let call_started = frame_timing_enabled().then(std::time::Instant::now);
        if let Some(now) = call_started {
            let timing = self.pending_timing.get_or_insert(PendingFrameTiming {
                started: now, update: std::time::Duration::ZERO, yielded_at: None, yielded: std::time::Duration::ZERO,
            });
            if let Some(yielded_at) = timing.yielded_at.take() {
                timing.yielded += now.duration_since(yielded_at);
            }
        }
        let timing_start = self.pending_timing.as_ref().map(|timing| timing.started);
        // Completion/error publication is independent of Update admission.
        // A failed worker must be observed before entering producer systems.
        if render.in_flight() { let _ = render.poll(); }
        // Update no longer waits for preparation admission: producers hand
        // writes to the packet's read lease without blocking, so simulation
        // proceeds during a cold pipeline compile (render_upload_boundary).
        self.waiting_for_preparation = false;
        // Runs while the render thread executes and presents the previous frame.
        if !self.update_if_admitted(input, publish_platform_output)? {
            if let Some(timing) = &mut self.pending_timing { timing.yielded_at = Some(std::time::Instant::now()); }
            return Ok(());
        }
        if self.input_consumed_this_call && let (Some(started), Some(timing)) = (call_started, &mut self.pending_timing) {
            timing.update += started.elapsed();
        }
        let mode = frame_render_mode(self.app.world());
        // Startup has now refreshed the actual saved-session list. Freeze its
        // optional background choice here and let CPU decoding finish off-thread
        // before the first loading graph advances the reveal or renders Logo.
        if mode == RenderMode::LoadingScreen
            && !crate::loading_graph::poll_background_preparation(self.app.world_mut())?
        {
            self.loading_render_pending_after_update = true;
            if let Some(timing) = &mut self.pending_timing { timing.yielded_at = Some(std::time::Instant::now()); }
            return Ok(());
        }
        self.loading_render_pending_after_update = false;
        if !render.poll() {
            self.awaiting_render = true;
            if let Some(timing) = &mut self.pending_timing { timing.yielded_at = Some(std::time::Instant::now()); }
            return Ok(());
        }
        self.awaiting_render = false;
        let gameplay_frame = mode == RenderMode::WorldGpuProfiled
            && self.app.world().get_resource::<bevy_state::state::State<
                country_core::app_loading_state::AppLoadingState,
            >>().is_some_and(|state| {
                *state.get() == country_core::app_loading_state::AppLoadingState::Game
            });
        if gameplay_frame && self.startup_ui_capture.is_some() {
            if let Some(state) = self.app.world().get_resource::<country_core::systems::render_loop::vermeer::VermeerState>() {
                if state.is_active() {
                    self.startup_ui_capture_saw_reveal = true;
                    self.startup_ui_capture_elapsed = 0.0;
                } else if self.startup_ui_capture_saw_reveal && state.progress >= 1.0 && !state.hold {
                    self.startup_ui_capture_elapsed += self.app.world()
                        .get_resource::<bevy_time::Time>().map_or(0.0, |clock| clock.delta_secs().min(1.0 / 30.0));
                }
            }
        }
        if self.startup_ui_capture.is_some() && self.startup_ui_capture_elapsed >= 0.5 {
            let path = self.startup_ui_capture.take().expect("checked capture path");
            log::info!("startup UI capture queued after real reveal completion: {}", path.display());
            crate::screenshot::gather_screenshot_requests(self.app.world_mut(), [
                crate::screenshot::ScreenshotRequest {
                    purpose: crate::screenshot::ScreenshotPurpose::FinalFrame,
                    dimensions: None,
                    sink: crate::screenshot::ScreenshotSink::File { path, jpeg_quality: 95 },
                    completion: Some(std::sync::Arc::new(|result| match result {
                        Ok(size) => log::info!("startup UI capture completed: {size:?}"),
                        Err(error) => log::error!("startup UI capture failed: {error}"),
                    })),
                },
            ])?;
        }
        // Hand-over: wait for the previous frame, apply its results and
        // write-backs to the game world, extract this frame into the render
        // world and start it (declare → present all run on the render thread).
        let handover_start = timing_start.map(|_| std::time::Instant::now());
        self.settle_render(render)?;
        let handover_time = handover_start.map(|start| start.elapsed());
        let extract_start = timing_start.map(|_| std::time::Instant::now());
        if !render.poll_owned_uploads()? {
            crate::frame_counters::record_upload_pending_yield();
            self.awaiting_render = true;
            return Ok(());
        }
        if !crate::loading_graph::poll_mei_uploads(self.app.world_mut())? {
            crate::frame_counters::record_upload_pending_yield();
            self.awaiting_render = true;
            return Ok(());
        }
        let pending_uploads = {
            let device = self.app.world().get_resource::<country_core::render::device::GraphicsDeviceResource>().cloned();
            if let Some(device) = device {
                admit_producer_uploads(self.app.world_mut(), device.device())
            } else { Ok(()) }
        };
        match pending_uploads {
            Err(rhapsody::Error::Exhausted(_)) => {
                crate::frame_counters::record_upload_pending_yield();
                self.awaiting_render = true;
                return Ok(());
            }
            result => result?,
        }
        if self.extract.is_none() {
            self.reset_render_exchange(render);
        }
        self.in_flight_temporal = crate::temporal_frame::TemporalCommit::prepare(self.app.world_mut());
        let side = render.reclaim();
        self.extract
            .as_mut()
            .expect("installed above")
            .run(self.app.world_mut(), &mut side.world);
        crate::temporal_frame::LateCameraFrame::attach(
            self.late_camera_frame.as_ref(), self.app.world(), &mut side.world);
        render.queue_frame(self.app.world(), mode, self.egui.hand_over());
        let extract_time = extract_start.map(|start| start.elapsed());
        self.in_flight_gameplay = gameplay_frame;
        render.start();
        self.progress.completed_frames = self.progress.completed_frames.wrapping_add(1);
        if gameplay_frame {
            self.progress.gameplay_frames = self.progress.gameplay_frames.wrapping_add(1);
        }
        // Temporal history is committed when this frame's image-writing
        // receipt returns, including when a later presentation step fails.
        crate::main::log_time_to_first_frame();
        let limiter_start = timing_start.map(|_| std::time::Instant::now());
        let target_fps = frame_target_fps(self.app.world());
        self.limiter.frame(target_fps);
        if let Some(start) = timing_start {
            let timing = self.pending_timing.take().expect("timing enabled for this frame");
            let limiter_time = limiter_start.expect("timing enabled").elapsed();
            let total = start.elapsed();
            let n = if gameplay_frame { self.progress.gameplay_frames } else { self.progress.completed_frames };
            if n <= 3 || n % frame_timing_every() == 0 || total.as_secs_f64() >= 1.0 {
                log::info!(target: "tiny_glade::frame_timing",
                    "frame n={n} gameplay={gameplay_frame} completed={} update_ms={:.3} yield_wait_ms={:.3} handover_wait_ms={:.3} extract_ms={:.3} limiter_ms={:.3} total_ms={:.3} target_fps={target_fps}",
                    self.progress.completed_frames,
                    timing.update.as_secs_f64() * 1000.0,
                    timing.yielded.as_secs_f64() * 1000.0,
                    handover_time.expect("timing enabled").as_secs_f64() * 1000.0,
                    extract_time.expect("timing enabled").as_secs_f64() * 1000.0,
                    limiter_time.as_secs_f64() * 1000.0, total.as_secs_f64() * 1000.0);
                if let Some(extract) = &self.extract {
                    log::info!(target: "tiny_glade::frame_timing", "extract n={n} {:?}", extract.stats());
                }
            }
        }
        Ok(())
    }
}

/// Runtime options belong to the recovered host, not to B's native layout.
#[derive(Debug, Default)]
pub struct GameLoopOptions {
    pub compiled_shader_root: std::path::PathBuf,
    /// Bounded native-window smoke run; absent in B and explicitly diagnostic.
    pub frame_limit: Option<u64>,
    pub input_recording: Option<crate::app_config::InputRecordingOpts>,
}

/// B's two cursor counters are recovered from 0x1404da7xx and 0x1404dae4f.
/// Input positions are accepted only after all preceding requests are confirmed.
#[derive(Debug, Default, Clone, Copy)]
pub struct CursorRequestClient {
    sent: u64,
    confirmed: u64,
}

impl CursorRequestClient {
    pub fn next_request_id(&mut self) -> u64 {
        self.sent = self.sent.wrapping_add(1);
        self.sent
    }
    pub fn on_confirmation(&mut self, id: u64) {
        self.confirmed = self.confirmed.max(id);
    }
    pub fn is_fully_confirmed(&self) -> bool {
        self.sent <= self.confirmed
    }
}

/// Owns the actual app/egui/WSI consumer. This is a source-level owner, not an
/// invented mirror of B's 0x1118-byte GameLoop. The macOS OS-thread adapter is
/// kept in event_loop; B's recovered event and frame semantics are methods here.
pub struct GameLoop {
    pub startup_instant: Option<std::time::Instant>,
    pub(crate) driver: GameFrameDriver,
    pub(crate) window: std::sync::Arc<winit::window::Window>,
    pub(crate) egui_io: crate::event_loop::EguiEventIo,
    /// Resident renderer; frame packets and results cross the thread boundary.
    /// Native window adaptation remains on this event-loop thread.
    pub(crate) render: crate::production_renderer::ProductionRenderThread,
    pub(crate) options: GameLoopOptions,
    input_recording: crate::input_recording_host::InputRecordingHost,
    pub(crate) modifiers: winit::keyboard::ModifiersState,
    pub(crate) cursor_requests: CursorRequestClient,
    pub(crate) capture_anchor: Option<winit::dpi::PhysicalPosition<f64>>,
    pub(crate) capture_needs_initial_event: bool,
    pub(crate) capture_uses_raw_motion: bool,
    pub(crate) visibility_suppressions: u32,
    // OS mode and last size are independent: Standard does not reset size.
    pub(crate) custom_cursor_mode: bool,
    pub(crate) custom_cursor_size_px: f32,
    // Strong actual-device lease; the Created event alone owns no memory.
    custom_cursor_buffer: Option<CustomCursorUpload>,
    pub(crate) frame_count: u64,
    pub(crate) suspended: bool,
    pub(crate) window_shown: bool,
    pub(crate) background_poll_deadline: Option<std::time::Instant>,
    pub(crate) lifecycle: std::rc::Rc<LoopLifecycle>,
    pub(crate) close_requested: bool,
    pub(crate) pending_os_events: std::collections::VecDeque<crate::event_loop::OsWindowEvent>,
    preparation: Option<crate::run_game::GameplayPreparation>,
    custom_cursor_events: bevy_ecs::event::EventCursor<
        country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd,
    >,
    capture_events:
        bevy_ecs::event::EventCursor<country_core::resources::mouse_capture::OsCursorCaptureCmd>,
    visibility_events:
        bevy_ecs::event::EventCursor<country_core::resources::mouse_capture::OsCursorVisibilityCmd>,
    window_events: bevy_ecs::event::EventCursor<country_core::resources::window::OsWindowCmd>,
    settings_events: bevy_ecs::event::EventCursor<crate::app_config::WriteSettingsFileCmd>,
    pending_windowed_resolution: Option<[u32; 2]>,
}

/// The pending CPU words belong to one admitted cursor owner. Unwritten words
/// remain undefined until their actual producer supplies them.
struct CustomCursorUpload {
    buffer: rhapsody::resource::ManagedBuffer,
    writer: std::sync::Arc<dyn country_core::render::ssbo::rhapsody_boundary::MappedUploadWriter>,
    pending: [u8; 16],
    dirty: u8,
}
impl CustomCursorUpload {
    fn write(&mut self, offset: usize, bytes: &[u8]) -> anyhow::Result<()> {
        self.pending[offset..offset + bytes.len()].copy_from_slice(bytes);
        self.dirty |= (((1u16 << (bytes.len() / 4)) - 1) << (offset / 4)) as u8;
        self.flush()
    }
    fn flush(&mut self) -> anyhow::Result<()> {
        while self.dirty != 0 {
            let first = self.dirty.trailing_zeros();
            let count = (self.dirty >> first).trailing_ones();
            let mask = (((1u16 << count) - 1) << first) as u8;
            let range = first as usize * 4..(first + count) as usize * 4;
            match self.writer.write(range.start, &self.pending[range]) {
                Ok(()) => self.dirty &= !mask,
                Err(rhapsody::Error::Exhausted(_) | rhapsody::Error::Timeout) => break,
                Err(error) => return Err(error.into()),
            }
        }
        Ok(())
    }
}

/// Event owners installed by the OS host before either App can run startup.
/// Headless acceptance uses the same registration rather than a reduced App.
pub fn register_platform_events(app: &mut App) {
    use country_core::resources::{mouse_capture::*, window::*};
    app.add_event::<OsCursorCaptureCmd>()
        .add_event::<OsCursorVisibilityCmd>()
        .add_event::<CustomCursorSsboCreatedCmd>()
        .add_event::<CursorCaptureCmd>()
        .add_event::<OsWindowCmd>()
        .add_event::<WindowFocusEvent>()
        .add_event::<country_core::resources::events::ShutdownGameCmd>();
}

impl GameLoop {
    pub(crate) fn render_ready_for_redraw(&mut self) -> bool {
        if self.driver.waiting_for_preparation {
            self.render.poll() || self.render.update_allowed()
        } else if self.driver.awaiting_render() || self.driver.waiting_for_collect {
            self.render.poll()
        } else {
            true
        }
    }

    pub fn new(
        mut app: App,
        window: std::sync::Arc<winit::window::Window>,
        options: GameLoopOptions,
        shaders: std::sync::Arc<country_core::asset_libraries::shader_library::AssetShaderLibrary>,
    ) -> anyhow::Result<Self> {
        if !app.is_plugin_added::<bevy_input::InputPlugin>() {
            app.add_plugins(bevy_input::InputPlugin);
        }
        use country_core::resources::window::*;
        register_platform_events(&mut app);
        let size = window.inner_size();
        app.world_mut()
            .insert_resource(WindowSize::new(size.width, size.height));
        register_settings_application_system(&mut app);
        // 只在真实host启动worker；异步准备移动同一游戏App时保留订阅取消owner。
        crate::theme_fetcher_plugin::build(&mut app);
        let skip_splash = app.world().resource::<country_core::game_config::GameConfig>().dev.skip_splash;
        let preferred = app.world().resource::<country_core::game_config::GameConfig>()
            .video.graphics_device_uuid.map(|uuid| *uuid.as_bytes());
        let mut renderer = crate::production_renderer::ProductionRenderer::new(
            window.clone(), &options.compiled_shader_root, shaders, !skip_splash, preferred,
        )?;
        // 同一窗口的真实候选快照随游戏App移动到准备线程，设置页不逐帧枚举驱动。
        let mut graphics = country_core::render::AvailableGraphicsDevices::from_enumeration(
            renderer.settings_graphics_devices()?,
        );
        graphics.record_creation_success(uuid::Uuid::from_bytes(renderer.device().device_uuid()));
        app.insert_resource(graphics);
        renderer.activate_application(app.world())?;
        renderer.set_vsync(
            app.world()
                .resource::<country_core::game_config::GameConfig>()
                .video
                .vsync_enable,
        )?;
        renderer.bridge().install_asset_libraries(app.world_mut())?;
        // B add_game_startup_systems' gameplay PreStartup tuple contains the
        // full SSBO producer. It stays on this gameplay App when preparation
        // moves its SubApp; the separate loading App installs only two entries.
        crate::gameplay_startup::install_gameplay_device_runtime(
            &mut app,
            renderer.bridge().hosts().buffers.graphics_device().clone(),
            renderer.bridge().hosts().clone(),
            options
                .compiled_shader_root
                .parent()
                .context("compiled shader directory has no installation root")?
                .to_path_buf(),
        )?;
        // Original run_game_thread14059f0a9 inserts the GUI context passed
        // from the OS owner as Pending before either startup path. The public
        // host owns Context creation here; both Apps retain that one identity.
        use country_core::resources::gui::{EguiContextResource, PendingEguiContextResource};
        let shared_gui = app.world().get_resource::<PendingEguiContextResource>()
            .map(|value| value.0.clone())
            .or_else(|| app.world().get_resource::<EguiContextResource>().map(|value| value.0.clone()))
            .unwrap_or_default();
        if !app.world().contains_resource::<PendingEguiContextResource>() {
            app.insert_resource(PendingEguiContextResource(shared_gui.clone()));
        }
        let mut preparation = None;
        if !skip_splash {
            // B owns two different Apps. Their libraries share only the actual
            // graphics device; no MainCamera is removed to fake a loading mode.
            let mut loading = crate::run_game::make_loading_app(&app)?;
            loading.insert_resource(EguiContextResource(shared_gui));
            #[cfg(any(windows, feature = "native-audio"))]
            loading.insert_resource(crate::audio_host::share_core(app.world().resource::<
                country_core::resources::audio_resources::AudioCore<
                    country_core::resources::native_audio::NativeStudio,
                >,
            >()));
            install_asset_library_systems(&mut app);
            // The warm-up graph must use the gameplay App's exact physical
            // owners; capture them before its SubApp moves to the worker. The
            // renderer keeps these same owners in `other_applications` until
            // publish re-adopts them (render_bridge.rs warm-up contract).
            let owners = renderer.bridge().warmup_owners()?;
            let initialize_programs = renderer.gameplay_program_initializer()?;
            let gameplay = std::mem::take(app.main_mut());
            preparation = Some(crate::run_game::GameplayPreparation::spawn_with_initializer(
                gameplay,
                initialize_programs,
                move |world| {
                    crate::production_renderer::warm_up_gameplay(owners, world)
                },
            )?);
            app = loading;
            // The same OS command identities are registered in B's loading
            // App tuple; replacing the App must not lose their event owners.
            register_platform_events(&mut app);
            register_settings_application_system(&mut app);
            renderer.activate_application(app.world())?;
            renderer.bridge().install_asset_libraries(app.world_mut())?;
            let demo = app
                .world()
                .resource::<country_core::game_config::GameConfig>()
                .dev
                .demo;
            crate::loading_graph::install_loading_startup(
                &mut app,
                renderer.device().clone(),
                renderer.bridge().hosts().clone(),
                paths::installed().to_path_buf(),
                demo,
            )?;
            app.insert_resource(country_core::render::device::GraphicsDeviceResource::new(
                country_core::rhapsody_contract::GraphicsDevice::new(renderer.device().clone())
                    .with_buffer_host(renderer.bridge().hosts().buffers.clone()),
            ));
            use bevy_ecs::schedule::IntoScheduleConfigs;
            crate::mei_host::install_mei_frame_systems(&mut app);
            // SplashScreenPlugin @14068e570 adds its real frame producer in set36.
            app.add_systems(
                bevy_app::Update,
                toolbar::menu::splash::ui.in_set(crate::run_game::GameSystemSet(36)),
            );
            let shaders = app
                .world()
                .resource::<crate::run_game::ApplicationShaderLibrary>()
                .0
                .clone();
            let backend = renderer.bridge().hosts().shaders.clone();
            app.add_systems(bevy_app::Startup, move |world: &mut World| {
                country_core::resources::logo_animation::startup_animated_logo(
                    world, &*backend, &shaders,
                )
                .expect("startup_animated_logo");
            });
            // Both Apps share the actual initialized Studio. Only a real FMOD
            // initialization failure enters AudioCore::None inside the producer.
            #[cfg(any(windows, feature = "native-audio"))]
            app.add_systems(
                bevy_app::Update,
                (
                    country_core::resources::logo_animation::animate_logo_system::<
                        country_core::resources::native_audio::NativeStudio,
                    >,
                    country_core::systems::audio::audio_update::boot_audio_update_system::<
                        country_core::resources::native_audio::NativeStudio,
                    >,
                ),
            );
            #[cfg(not(any(windows, feature = "native-audio")))]
            app.add_systems(
                bevy_app::Update,
                country_core::resources::logo_animation::animate_logo_with_disabled_audio,
            );
        } else {
            crate::run_game::finish_skipped_splash(app.world_mut())?;
        }
        let context = app
            .world()
            .get_resource::<country_core::resources::gui::EguiContextResource>()
            .map(|r| r.0.clone())
            .unwrap_or_default();
        let egui_io = crate::event_loop::EguiEventIo::new(egui_winit::State::new(
            context.clone(),
            egui::ViewportId::ROOT,
            window.as_ref(),
            Some(window.scale_factor() as f32),
            None,
        ));
        let mut input_recording = crate::input_recording_host::InputRecordingHost::new(options.input_recording.clone());
        if skip_splash { input_recording.activate(app.world_mut()); }
        Ok(Self {
            startup_instant: Some(std::time::Instant::now()),
            driver: GameFrameDriver::new(app, context),
            window,
            egui_io,
            render: renderer.into_render_thread().context("spawn render thread")?,
            options,
            input_recording,
            modifiers: Default::default(),
            cursor_requests: Default::default(),
            capture_anchor: None,
            capture_needs_initial_event: false,
            capture_uses_raw_motion: false,
            visibility_suppressions: 0,
            // Original run_event_loop @14026e56e/583/5cd initializes
            // suppressions, saved size bits, and mode independently to zero.
            custom_cursor_mode: false,
            custom_cursor_size_px: 0.0,
            custom_cursor_buffer: None,
            frame_count: 0,
            suspended: false,
            window_shown: false,
            background_poll_deadline: None,
            lifecycle: Default::default(),
            pending_os_events: Default::default(),
            close_requested: false,
            preparation,
            custom_cursor_events: Default::default(),
            capture_events: Default::default(),
            visibility_events: Default::default(),
            window_events: Default::default(),
            settings_events: Default::default(),
            pending_windowed_resolution: None,
        })
    }

    pub fn app(&self) -> &App {
        self.driver.app()
    }
    pub fn app_mut(&mut self) -> &mut App {
        self.driver.app_mut()
    }

    pub fn frame_progress(&self) -> FrameProgress {
        self.driver.progress()
    }

    pub fn frame_observation(
        &mut self,
    ) -> Option<&crate::production_renderer::ProductionFrameObservation> {
        self.render.frame_observation()
    }

    /// 原生 present 结果映射前的成功次数；可恢复的过期结果不计入。
    pub fn native_present_success_count(&mut self) -> u64 {
        self.render.native_present_success_count()
    }

    /// One control round-trip at each measurement boundary; no per-frame messages.
    #[cfg(feature = "cpu-probe")]
    pub fn start_cpu_probe(&mut self, config: crate::render_cpu_probe::CpuProbeConfig) -> anyhow::Result<()> {
        self.render.call(move |renderer| {
            let session = crate::render_cpu_probe::CpuProbeSession::new(config)?;
            renderer.install_cpu_recorder(session);
            Ok(())
        })
    }

    #[cfg(feature = "cpu-probe")]
    pub fn finish_cpu_probe(&mut self) -> Option<crate::render_cpu_probe::CpuProbeSession> {
        self.render.call(|renderer| renderer.take_cpu_recorder())
    }

    pub fn gameplay_preparation_pending(&self) -> bool {
        self.preparation.is_some()
    }

    /// B consumes outgoing game requests before the OS event batch. Platform
    /// operations happen on the current OS thread in the macOS adapter.
    pub(crate) fn flush_game_requests(&mut self) -> anyhow::Result<()> {
        use country_core::resources::{mouse_capture::*, window::OsWindowCmd};
        let capture = self
            .capture_events
            .read(
                self.driver
                    .app
                    .world()
                    .resource::<bevy_ecs::event::Events<OsCursorCaptureCmd>>(),
            )
            .last()
            .copied();
        if let Some(command) = capture {
            let id = self.cursor_requests.next_request_id();
            self.apply_capture(command)?;
            // B drains the incoming OS batch before sending this request. Its
            // acknowledgement cannot validate older coordinates in that batch.
            self.pending_os_events
                .push_back(crate::event_loop::OsWindowEvent::CursorConfirmed(id));
        }
        let minimize_count = self
            .window_events
            .read(
                self.driver
                    .app
                    .world()
                    .resource::<bevy_ecs::event::Events<OsWindowCmd>>(),
            )
            .count();
        for _ in 0..minimize_count {
            self.window
                .set_minimized(!self.window.is_minimized().unwrap_or(false));
        }
        let visibility = self
            .visibility_events
            .read(
                self.driver
                    .app
                    .world()
                    .resource::<bevy_ecs::event::Events<OsCursorVisibilityCmd>>(),
            )
            .last()
            .copied();
        if let Some(command) = visibility {
            self.apply_visibility(command)?;
        }
        // B @1404da967 consumes visibility before Created @1404daa37.
        // Neither receipt nor replacement initializes the mapped 16 bytes.
        let created = self
            .custom_cursor_events
            .read(
                self.driver
                    .app
                    .world()
                    .resource::<bevy_ecs::event::Events<CustomCursorSsboCreatedCmd>>(),
            )
            .last()
            .copied();
        if let Some(created) = created {
            self.install_custom_cursor_buffer(created)?;
        }
        Ok(())
    }

    fn install_custom_cursor_buffer(
        &mut self,
        created: country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd,
    ) -> anyhow::Result<()> {
        // Retire the old writer before resolving the new owner's lease.
        self.custom_cursor_buffer = None;
        let lease = self.resolve_custom_cursor_buffer()?;
        anyhow::ensure!(
            lease.buffer.persistent_host_ptr() == Some(created.0.as_ptr().cast::<u8>()),
            "CustomCursor Created pointer does not match its canonical managed buffer"
        );
        self.custom_cursor_buffer = Some(lease);
        Ok(())
    }

    /// The startup event carries only a pointer; the actual long-lived owner is
    /// the canonical App library plus this same BufferHost's managed lease.
    fn resolve_custom_cursor_buffer(&mut self) -> anyhow::Result<CustomCursorUpload> {
        use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
        use country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo;
        let core_buffer = self
            .driver
            .app
            .world()
            .resource::<AssetSsboLibrary>()
            .get_buffer::<CustomCursorSsbo>()
            .ok_or_else(|| anyhow::anyhow!("CustomCursor has no canonical buffer in its World"))?;
        let (lease, writer) = self
            .render
            .mapped_buffer_writer(*core_buffer)
            .ok_or_else(|| {
                anyhow::anyhow!("CustomCursor buffer does not belong to the active BufferHost")
            })?;
        anyhow::ensure!(
            lease.size == core::mem::size_of::<CustomCursorSsbo>() as u64,
            "CustomCursor buffer has an invalid byte length"
        );
        anyhow::ensure!(
            lease.persistent_host_ptr().is_some(),
            "CustomCursor buffer is not persistently mapped"
        );
        Ok(CustomCursorUpload { buffer: lease, writer, pending: [0; 16], dirty: 0 })
    }

    /// B callback @140557e57..eb4 writes before the GameLoop confirmation
    /// filter. Only the uncaptured absolute branch writes this mapped SSBO.
    pub(crate) fn update_custom_cursor_absolute(
        &mut self,
        position: [f64; 2],
    ) -> anyhow::Result<()> {
        if self.capture_anchor.is_some() {
            return Ok(());
        }
        if let Some(buffer) = &mut self.custom_cursor_buffer {
            let data = country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo {
                pos: [position[0] as f32, position[1] as f32],
                enabled: u32::from(self.visibility_suppressions == 0),
                size_px: self.custom_cursor_size_px,
            };
            buffer.write(0, bytemuck::bytes_of(&data))?;
        }
        Ok(())
    }

    fn reset_world_event_cursors(&mut self) {
        // EventId counters belong to each independent App's Events owner.
        self.custom_cursor_events = Default::default();
        self.capture_events = Default::default();
        self.visibility_events = Default::default();
        self.window_events = Default::default();
        self.settings_events = Default::default();
    }

    fn apply_capture(
        &mut self,
        command: country_core::resources::mouse_capture::OsCursorCaptureCmd,
    ) -> anyhow::Result<()> {
        self.driver.late_camera.invalidate();
        use country_core::resources::mouse_capture::OsCursorCaptureCmd;
        use winit::window::CursorGrabMode;
        match command {
            OsCursorCaptureCmd::Capture(position) => {
                self.capture_uses_raw_motion = if self
                    .window
                    .set_cursor_grab(CursorGrabMode::Confined)
                    .is_ok()
                {
                    false
                } else {
                    // macOS lacks Windows' confine mode. Its pinned winit
                    // Locked mode supplies relative DeviceEvent mouse motion.
                    self.window.set_cursor_grab(CursorGrabMode::Locked)?;
                    true
                };
                self.capture_anchor = Some(winit::dpi::PhysicalPosition::new(
                    position.x as f64,
                    position.y as f64,
                ));
                self.capture_needs_initial_event = true;
            }
            OsCursorCaptureCmd::Release => {
                self.window.set_cursor_grab(CursorGrabMode::None)?;
                self.capture_anchor = None;
                self.capture_uses_raw_motion = false;
            }
            OsCursorCaptureCmd::SetPosition(position) => {
                self.window
                    .set_cursor_position(winit::dpi::PhysicalPosition::new(
                        position.x as f64,
                        position.y as f64,
                    ))?;
            }
        }
        Ok(())
    }

    fn apply_visibility(
        &mut self,
        command: country_core::resources::mouse_capture::OsCursorVisibilityCmd,
    ) -> anyhow::Result<()> {
        use country_core::resources::mouse_capture::OsCursorVisibilityCmd;
        match command {
            OsCursorVisibilityCmd::Standard => {
                self.window
                    .set_cursor_visible(self.visibility_suppressions == 0);
                self.custom_cursor_mode = false;
            }
            OsCursorVisibilityCmd::Custom(size_px) => {
                if !self.custom_cursor_mode {
                    self.window.set_cursor_visible(false);
                }
                self.custom_cursor_mode = true;
                self.custom_cursor_size_px = size_px;
                if let Some(buffer) = &mut self.custom_cursor_buffer {
                    buffer.write(12, &size_px.to_ne_bytes())?;
                }
            }
            OsCursorVisibilityCmd::Suppress(origin) => {
                assert_ne!(origin.0, 0);
                let previous = self.visibility_suppressions;
                self.visibility_suppressions |= origin.0;
                if previous == 0 {
                    if self.custom_cursor_mode {
                        if let Some(buffer) = &mut self.custom_cursor_buffer {
                            buffer.write(8, &0u32.to_ne_bytes())?;
                        }
                    } else {
                        self.window.set_cursor_visible(false);
                    }
                }
            }
            OsCursorVisibilityCmd::Unsuppress(origin) => {
                assert_ne!(origin.0, 0);
                let previous = self.visibility_suppressions;
                self.visibility_suppressions &= !origin.0;
                if previous != 0 && self.visibility_suppressions == 0 {
                    if self.custom_cursor_mode {
                        if let Some(buffer) = &mut self.custom_cursor_buffer {
                            buffer.write(8, &1u32.to_ne_bytes())?;
                        }
                    } else {
                        self.window.set_cursor_visible(true);
                    }
                }
            }
        }
        Ok(())
    }

    /// B begin_frame → App::update → end_frame → PlatformOutput → render.
    pub(crate) fn frame(&mut self, target: &dyn crate::event_loop::LoopTarget) -> anyhow::Result<bool> {
        if !self.lifecycle.is_running() {
            return Ok(false);
        }
        if let Some(buffer) = &mut self.custom_cursor_buffer { buffer.flush()?; }
        if self.close_requested { self.render.cancel_preparation()?; }
        if self.driver.waiting_for_preparation {
            // Surface terminal failure before the admission gate can hide it
            // and before another simulation Update observes a failed frame.
            let _ = self.render.poll();
            if !self.render.update_allowed() { return Ok(true); }
            return self.advance_frame(None);
        }
        if self.driver.awaiting_render() || self.driver.waiting_for_collect {
            if !self.render.poll() { return Ok(true); }
            // Finalize the already simulated frame before dispatching newer
            // input or swapping Apps. Neither input nor App::update runs twice.
            return self.advance_frame(None);
        }
        let background_pending = crate::loading_graph::is_background_preparation_pending(
            self.driver.app.world(),
        );
        if let Some(preparation) = &mut self.preparation {
            preparation.poll_completion()?;
        }
        // Advance loading gates without reclaiming a frame. Only a ready
        // owner handoff needs to settle the World which declared that frame.
        let handoff_ready = if let Some(preparation) = &mut self.preparation {
            !self.close_requested && !background_pending && preparation.ready_to_handoff(self.driver.app_mut())?
        } else { false };
        if handoff_ready {
            self.driver.settle_render(&mut self.render)?;
        }
        if let Some(preparation) = &mut self.preparation
            && handoff_ready
        {
            // No OS callback can run while poll owns this thread. Withdraw the
            // old writer before poll can swap Worlds; a failed/pending poll
            // cannot accidentally write through another App's pointer.
            let old_cursor_lease = self.custom_cursor_buffer.take();
            if preparation.poll(self.driver.app_mut())? {
                drop(old_cursor_lease);
                self.reset_world_event_cursors();
                self.driver.reset_render_exchange(&mut self.render);
                self.input_recording.activate(self.driver.app_mut().world_mut());
                self.render
                    .activate_application(self.driver.app().world())?;
                // The worker may have run more than two updates after PreStartup,
                // so its one-shot Created event can already have expired. Rejoin
                // the real canonical owner; do not synthesize a new notification,
                // create a second buffer, or initialize its mapped contents.
                self.custom_cursor_buffer = Some(self.resolve_custom_cursor_buffer()?);
                self.preparation = None;
            } else {
                self.custom_cursor_buffer = old_cursor_lease;
            }
        }
        // B switches Apps, snapshots the incoming OS batch, then flushes
        // outgoing requests before dispatching that captured batch. Queue state
        // belongs to the loop, so a handoff cannot discard this frame's inputs.
        let incoming_count = self.pending_os_events.len();
        if self
            .driver
            .app
            .world()
            .contains_resource::<country_core::resources::shutdown::ShutdownGame>()
        {
            return Ok(false);
        }
        self.flush_game_requests()?;
        let mut os_close_requested = false;
        for _ in 0..incoming_count {
            let event = self
                .pending_os_events
                .pop_front()
                .expect("captured OS batch must remain queued");
            os_close_requested |= matches!(&event, crate::event_loop::OsWindowEvent::CloseRequested);
            self.dispatch_os_event(event, target)?;
        }
        if background_pending
            && !crate::loading_graph::poll_background_preparation(self.driver.app.world_mut())?
        {
            // Keep pumping native close/resize events without consuming input
            // edges, updating Logo, advancing handoff, or counting a frame.
            if os_close_requested {
                return Ok(false);
            }
            self.apply_pending_video_settings()?;
            return Ok(true);
        }
        // B @1404dc952..971 skips both simulation and rendering while either
        // dimension is zero. Keep input edges for the next actual update.
        if frame_waits_for_window_extent(self.driver.app.world()) {
            // Main-thread host adaptation: an actual OS close may stop a zero-
            // sized window without simulating, rendering or inventing the game
            // shutdown marker. Sender disconnect still joins the real saver.
            if os_close_requested {
                return Ok(false);
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            // 原 dc971 跳到公共 de84d，再处理 de8c5 视频设置；否则零尺寸可能永远
            // 无法应用恢复设置。此尾段不 update App、消耗输入边沿或推进帧计数。
            self.apply_pending_video_settings()?;
            return Ok(true);
        }
        // B 1404ddf28..3c: real OS egui input is consumed only while the
        // debug or experimental window is active. Other frames still begin
        // and end egui with RawInput::default (1404ddf49 -> 140c2c770).
        let input = if self.driver.loading_render_pending_after_update {
            None
        } else if frame_egui_input_enabled(self.driver.app.world()) {
            Some(self.egui_io.take_egui_input(&self.window))
        } else {
            Some(egui::RawInput::default())
        };
        self.advance_frame(input)
    }

    fn advance_frame(&mut self, input: Option<egui::RawInput>) -> anyhow::Result<bool> {
        let egui_io = &self.egui_io;
        let window = &self.window;
        let previous_completed = self.driver.progress().completed_frames;
        self.driver
            .frame(input, &mut self.render, |output| {
                egui_io.handle_platform_output(window, output);
            })?;
        // 本帧消费完成后清除按下和释放边沿，保留仍按住的按钮。
        if self.driver.input_consumed_this_call {
            self.input_recording.input_consumed(self.driver.app.world())?;
            if let Some(mut keys) = self.driver.app.world_mut().get_resource_mut::<country_core::systems::backend::input::button_input::KeyButtonInput>() {
                keys.clear();
            }
            if let Some(mut mouse) = self.driver.app.world_mut().get_resource_mut::<country_core::systems::backend::input::button_input::MouseButtonInput>() {
                mouse.clear();
            }
        }
        if self.driver.progress().completed_frames == previous_completed {
            self.apply_pending_video_settings()?;
            return Ok(true);
        }
        self.frame_count = self.frame_count.wrapping_add(1);
        self.apply_pending_video_settings()?;
        // The original world-side state is observed on the next iteration.
        Ok(!self
            .driver
            .app
            .world()
            .contains_resource::<country_core::resources::shutdown::ShutdownGame>())
    }

    fn apply_pending_video_settings(&mut self) -> anyhow::Result<()> {
        let flags = std::mem::take(&mut *PENDING_VIDEO_SETTING_FLAGS.lock().unwrap());
        if flags != 0 {
            // A launch-only windowed request remains authoritative when an
            // in-game settings event arrives, without persisting that override.
            if let Some(policy) = self.driver.app.world()
                .get_resource::<crate::app_config::LaunchDisplayOverride>().cloned()
            {
                policy.apply(&mut self.driver.app.world_mut()
                    .resource_mut::<country_core::game_config::GameConfig>().video.display);
            }
            let config = &self
                .driver
                .app
                .world()
                .resource::<country_core::game_config::GameConfig>()
                .video;
            if flags & 2 != 0 {
                self.render.set_vsync(config.vsync_enable)?;
            }
            if flags & 5 != 0 {
                crate::event_loop::apply_display_config(&self.window, &config.display)?;
                self.pending_windowed_resolution = (!config.display.fullscreen)
                    .then_some(config.display.windowed_resolution);
            }
            self.driver
                .app
                .world_mut()
                .send_event(crate::app_config::WriteSettingsFileCmd);
            // 原main_loop4dec71..ded27在窗口/vsync应用成功后替换完整快照。
            // 不能在点击Apply或操作失败前更新，否则菜单会隐藏尚未应用的设置。
            let applied = self.driver.app.world()
                .resource::<country_core::game_config::GameConfig>().video.clone();
            self.driver.app.world_mut()
                .resource_mut::<crate::gui::settings::LastAppliedVideoSettings>().0 = Some(applied);
        }
        // AppKit fullscreen transitions are asynchronous. Resizing the content
        // while leaving fullscreen can interrupt the native transition; restore
        // the requested windowed size only once winit reports windowed state.
        if self.window.fullscreen().is_none()
            && let Some([width, height]) = self.pending_windowed_resolution.take()
        {
            let _ = self.window.request_inner_size(winit::dpi::PhysicalSize::new(width, height));
        }
        let write = self
            .settings_events
            .read(
                self.driver
                    .app
                    .world()
                    .resource::<bevy_ecs::event::Events<crate::app_config::WriteSettingsFileCmd>>(),
            )
            .last()
            .is_some();
        if write {
            let config = self
                .driver
                .app
                .world()
                .resource::<country_core::game_config::GameConfig>();
            let persisted = self.driver.app.world()
                .get_resource::<crate::app_config::LaunchDisplayOverride>()
                .map(|policy| policy.persistence_config(config));
            let config = persisted.as_ref().unwrap_or(config);
            if let Some(repro) = self
                .driver
                .app
                .world()
                .get_resource::<crate::app_config::PanicReproStateResource>()
            {
                crate::app_config::write_settings_file(config, &repro.0);
            } else {
                crate::app_config::write_game_config_to_disk(config);
            }
        }
        Ok(())
    }

    /// B shutdown waits for GPU work before render cache/device destruction.
    /// The CLI recorder saves the current complete Rust input-stream format
    /// once while its gameplay World is still alive.
    pub(crate) fn shut_down(&mut self) -> anyhow::Result<()> {
        let Some(clean_cpu) = self.lifecycle.begin_cleanup() else { return Ok(()); };
        self.pending_os_events.clear();
        let preparation_cancel = if self.render.failed() { Ok(()) } else { self.render.cancel_preparation() };
        let mut failure = preparation_cancel.err();
        if clean_cpu {
            // A worker can still be declaring/executing the gameplay warm-up with
            // these exact GPU owners. Stop and join it before touching renderer
            // lifetime, and do not abandon GPU cleanup when it reports an error.
            let preparation_result = catch_host_callback("gameplay preparation cleanup", || {
                self.preparation.take()
                    .map_or(Ok(()), |mut preparation| preparation.cancel_and_join())
            });
            let input_result = catch_host_callback("input recording cleanup", || {
                self.input_recording.finish(self.driver.app().world())
            });
            let save_result = catch_host_callback("save thread cleanup", || {
                country_core::systems::save::save_thread::shutdown_save_thread(
                    self.driver.app_mut().world_mut(),
                )
            });
            // hurry_active_screenshots_on_shutdown (0x14031eed0): every pending request gets its
            // hurry flag (+0x50) and its readback/write is completed, then the writer threads are
            // joined. It needs the live device, so it runs before renderer.shutdown(). The original
            // registers it as a system (wrappers 0x1402cc875 / 0x14044ee13; schedule not recovered).
            let screenshot_result = catch_host_callback("screenshot cleanup", || {
                let world = self.driver.app_mut().world_mut();
                if crate::screenshot::shared_texture_capture::installed(world) {
                    crate::screenshot::hurry_active_screenshots_on_shutdown(world).map_err(Into::into)
                } else {
                    Ok(())
                }
            });
            let cursor_result = catch_host_callback("cursor cleanup", || {
                self.custom_cursor_buffer = None;
                self.window
                    .set_cursor_grab(winit::window::CursorGrabMode::None)
                    .ok();
                self.window.set_cursor_visible(true);
                Ok(())
            });
            if let Err(error) = preparation_result {
                if failure.is_some() { log::error!("Additional gameplay preparation cleanup failure: {error:#}"); }
                else { failure = Some(error); }
            }
            for (phase, result) in [
                ("input recording", input_result),
                ("save thread", save_result),
                ("screenshot hurry", screenshot_result),
                ("cursor", cursor_result),
            ] {
                if let Err(error) = result {
                    if failure.is_some() {
                        log::error!("Additional {phase} shutdown failure: {error:#}");
                    } else {
                        failure = Some(error);
                    }
                }
            }
        }
        // Settle the last frame first: its error, screenshots and counts.
        if let Err(error) = catch_host_callback("final frame", || self.driver.settle_render(&mut self.render)) {
            if failure.is_some() {
                log::error!("Additional final frame failure: {error:#}");
            } else {
                failure = Some(error);
            }
        }
        match catch_host_callback("GPU shutdown", || self.render.shutdown_renderer()) {
            Ok(()) => self.lifecycle.renderer_stopped(),
            Err(error) => {
                if failure.is_some() {
                    log::error!("Additional GPU shutdown failure: {error:#}");
                } else {
                    failure = Some(error);
                }
            }
        }
        if let Some(error) = failure {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// B 0x1404e2470: enumerate current monitors and sorted public video modes.
    pub fn extract_available_monitors_info(
        available: &mut AvailableMonitors,
        event_loop: &EventLoopWindowTarget<()>,
    ) {
        Self::extract_monitors_from_target(available, event_loop)
    }

    /// Same enumeration for any loop target, including the game thread's view.
    pub(crate) fn extract_monitors_from_target(
        available: &mut AvailableMonitors,
        event_loop: &dyn crate::event_loop::LoopTarget,
    ) {
        available.clear();
        for handle in event_loop.available_monitors() {
            let monitor = MonitorMatch::from_winit(&handle);
            let mut modes: Vec<winit::monitor::VideoMode> = handle.video_modes().collect();
            modes.sort_by(|a, b| b.cmp(a));
            available.monitors.push(AvailableMonitor {
                video_modes: modes.iter().map(VideoMode::from_winit).collect(),
                monitor,
            });
        }
    }
}

impl Drop for GameLoop {
    fn drop(&mut self) {
        // Also cover a constructed loop whose OS runner returns an error or is
        // never entered: worker fields must not outlive renderer destruction.
        if self.lifecycle.phase.get() != LoopPhase::Stopped {
            if let Err(error) = self.shut_down() {
                self.lifecycle.record_failure(error);
            }
        }
    }
}

/// `gather_pending_video_setting_changes` 的队列元素（B item stride 0x10，
/// 标志字节 @+8；前 8 字节为命令载荷）。
#[derive(Debug)]
pub struct VideoSettingChange {
    /// `ApplyVideoSettingsCmd`（B 事件；字段未知，见 app_config）。
    pub command: crate::app_config::ApplyVideoSettingsCmd,
    /// `+0x8`：按位或进全局标志的字节。
    pub flags: u8,
}

/// `PendingVideoSettingChanges` 的队列段。
///
/// B 布局（0x1404e3380）：`ptr@+8`、`end(len)@+0x10`、`start@+0x18`
/// （第二段 +0x28/+0x30/+0x38，总计 `total@+0x40`）。`start` 是该段在全局
/// 序号中的起始位置。
#[derive(Debug, Default)]
pub struct PendingVideoSettingChanges {
    /// 已消费的全局序号（解引用后：`*cursor`）。
    pub consumed: usize,
    pub first: Vec<VideoSettingChange>,
    pub first_start: usize,
    pub second: Vec<VideoSettingChange>,
    pub second_start: usize,
    pub total: usize,
}

/// 全局待处理视频设置标志（B：std `Mutex<u8>` 静态；state/poison/data 三字节）。
pub static PENDING_VIDEO_SETTING_FLAGS: Mutex<u8> = Mutex::new(0);

/// `gather_pending_video_setting_changes` `0x1404e3380` —— 重建。
///
/// 控制流逐语句对应 fresh 反编译（`wrapping_*` 保留 release 语义的无符号回绕，
/// 与原机器码一致；锁的 poison→panic 与 unlock 时的 wake 由 std Mutex 提供）。
pub fn gather_pending_video_setting_changes(changes: &mut PendingVideoSettingChanges) {
    let start = changes.consumed;

    let off1 = start.saturating_sub(changes.first_start);
    let off2 = start.saturating_sub(changes.second_start);

    let len1_total = changes.first.len();
    let len2_total = changes.second.len();

    let (first, rem1) = if len1_total < off1 {
        (&[][..], 0)
    } else {
        (&changes.first[off1..], len1_total - off1)
    };
    let (second, rem2) = if len2_total < off2 {
        (&[][..], 0)
    } else {
        (&changes.second[off2..], len2_total - off2)
    };

    changes.consumed = changes.total.wrapping_sub(rem2).wrapping_sub(rem1);

    for item in first.iter().chain(second.iter()) {
        changes.consumed = changes.consumed.wrapping_add(1);
        let mut flags = PENDING_VIDEO_SETTING_FLAGS.lock().unwrap();
        *flags |= item.flags;
    }
}

/// B 0x1404e3370 registers the 0x1404e3380 event gatherer in PostUpdate.
pub fn register_settings_application_system(app: &mut App) {
    app.add_event::<crate::app_config::ApplyVideoSettingsCmd>()
        .add_event::<crate::app_config::WriteSettingsFileCmd>()
        .add_systems(bevy_app::PostUpdate, gather_video_setting_events);
}

fn gather_video_setting_events(
    mut events: bevy_ecs::event::EventReader<crate::app_config::ApplyVideoSettingsCmd>,
) {
    for event in events.read() {
        *PENDING_VIDEO_SETTING_FLAGS.lock().unwrap() |= event.0;
    }
}

#[cfg(test)]
mod frame_tests {
    use super::*;
    use country_core::{game_config::GameConfig, resources::gui::EscMenuState};

    #[test]
    fn rejected_nani_payload_retries_before_handover_without_new_writes_or_double_roll() {
        use country_core::render::{nani::gpu_world::{GpuWorld, GpuWorldSubset,
            GpuWorldSubsetKind, OwnedGpuWorldSubset, InstanceCullData, InstanceKey},
            ssbo::{StorageElement, rhapsody_boundary::{GraphicsDevice, Buffer, BufferDesc}}};
        use std::cell::{Cell, RefCell};
        #[derive(Clone, Copy, Default, bytemuck::NoUninit)]
        #[repr(C)]
        struct Payload([u32; 8]);
        impl StorageElement for Payload { const NAME: &'static str = "NaniAdmissionFixture"; }
        #[derive(Default)]
        struct Quota {
            next: Cell<u64>, attempts: Cell<usize>,
            calls: RefCell<Vec<u64>>, accepted: RefCell<Vec<(u64, Vec<u8>)>>,
        }
        impl GraphicsDevice for Quota {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> rhapsody::Result<Buffer> {
                let id = self.next.get() + 1; self.next.set(id);
                Ok(Buffer { id, byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, buffer: &Buffer, data: &[u8], ranges: &[std::ops::Range<usize>], _: usize)
                -> rhapsody::Result<()> {
                let attempt = self.attempts.get() + 1; self.attempts.set(attempt);
                self.calls.borrow_mut().push(buffer.id);
                if [2, 3, 5].contains(&attempt) { return Err(rhapsody::Error::exhausted("fixture admission full")); }
                for range in ranges { self.accepted.borrow_mut().push((buffer.id, data[range.clone()].to_vec())); }
                Ok(())
            }
        }
        let device = Quota::default();
        let kind = GpuWorldSubsetKind::Canopy;
        let mut gpu = GpuWorld::new(&device);
        gpu.assign_subset(kind, GpuWorldSubset::Owned(OwnedGpuWorldSubset::new::<Payload>(&device, kind, 8)));
        let cull = [InstanceCullData::new(glam::Vec3::ZERO, 1.0, 0),
            InstanceCullData::new(glam::Vec3::ZERO, 1.0, 1)];
        let data = [Payload([17; 8]), Payload([29; 8])];
        let mut keys = Vec::new();
        gpu.create_instances(kind, &cull, &data, &mut keys);
        country_core::render::nani::systems::nani_flush_instance_updates(&mut gpu, &device);
        assert!(gpu.instance_uploads_pending());
        assert_eq!((gpu.previous_instance_count(), gpu.total_instance_count), (0, 2));
        let first = device.calls.borrow().clone();
        assert_eq!(first.len(), 2);
        assert_ne!(first[0], first[1], "positions and typed data have independent real identities");
        let mut world = bevy_ecs::world::World::new(); world.insert_resource(gpu);
        assert!(matches!(admit_producer_uploads(&mut world, &device), Err(rhapsody::Error::Exhausted(_))));
        assert!(world.resource::<GpuWorld>().instance_uploads_pending());
        assert_eq!(world.resource::<GpuWorld>().previous_instance_count(), 0);
        admit_producer_uploads(&mut world, &device).unwrap();
        let ready = world.resource::<GpuWorld>();
        assert!(!ready.instance_uploads_pending());
        assert_eq!((ready.previous_instance_count(), ready.total_instance_count), (2, 0));
        assert_eq!(*device.calls.borrow(), [first[0], first[1], first[1], first[1]]);
        let accepted = device.accepted.borrow();
        assert_eq!(accepted.len(), 2);
        assert_eq!(accepted[0].1, country_core::render::ssbo::rhapsody_boundary::slice_as_bytes(&cull));
        assert_eq!(accepted[1].1, bytemuck::cast_slice::<_, u8>(&data));
        drop(accepted);
        admit_producer_uploads(&mut world, &device).unwrap();
        assert_eq!(device.attempts.get(), 4);
        assert_eq!(world.resource::<GpuWorld>().previous_instance_count(), 2);
        let old = world.resource::<GpuWorld>().clone();
        assert_eq!(old.owned_subset_mut(kind).main_cull_command_bound(), 2);
        {
            let mut gpu = world.resource_mut::<GpuWorld>();
            for &(id, index, _) in &keys {
                gpu.update_instance_mesh(&mut InstanceKey { id, index, kind }, 7);
            }
            country_core::render::nani::systems::nani_flush_instance_updates(&mut gpu, &device);
            assert!(gpu.instance_uploads_pending());
            assert_eq!(gpu.previous_instance_count(), 2);
        }
        assert_eq!(device.attempts.get(), 5);
        assert_eq!(device.accepted.borrow().len(), 2);
        admit_producer_uploads(&mut world, &device).unwrap();
        assert!(!world.resource::<GpuWorld>().instance_uploads_pending());
        assert_eq!(world.resource::<GpuWorld>().previous_instance_count(), 0);
        let new = world.resource::<GpuWorld>().clone();
        assert_eq!(old.owned_subset_mut(kind).main_cull_command_bound(), 2);
        assert_eq!(new.owned_subset_mut(kind).main_cull_command_bound(), 1);
        assert_eq!(*device.calls.borrow(), [first[0], first[1], first[1], first[1], first[0], first[0]]);
        let changed = [InstanceCullData::new(glam::Vec3::ZERO, 1.0, 7); 2];
        let accepted = device.accepted.borrow();
        assert_eq!(accepted.len(), 3);
        assert_eq!(accepted[2].0, first[0]);
        assert_eq!(accepted[2].1, country_core::render::ssbo::rhapsody_boundary::slice_as_bytes(&changed));
        drop(accepted);
        admit_producer_uploads(&mut world, &device).unwrap();
        assert_eq!(device.attempts.get(), 6);
        assert_eq!(world.resource::<GpuWorld>().previous_instance_count(), 0);
    }

    #[test]
    fn cursor_upload_preserves_partial_fields_and_retries_without_new_input() {
        use country_core::render::ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice, MappedUploadWriter};
        use std::{collections::VecDeque, sync::Arc};
        #[derive(Default)]
        struct Writer {
            results: Mutex<VecDeque<rhapsody::Result<()>>>,
            accepted: Mutex<Vec<(usize, Vec<u8>)>>,
        }
        impl MappedUploadWriter for Writer {
            fn write(&self, offset: usize, bytes: &[u8]) -> rhapsody::Result<()> {
                self.results.lock().unwrap().pop_front().unwrap_or(Ok(()))?;
                self.accepted.lock().unwrap().push((offset, bytes.to_vec()));
                Ok(())
            }
            fn try_read(&self, _: usize, _: usize, _: &mut dyn FnMut()) -> rhapsody::Result<bool> {
                panic!("cursor producer must not read or wait for GPU memory")
            }
        }
        let device = match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
                return;
            }
        };
        let host = crate::render_host::BufferHost::new(device);
        let handle = host.create_buffer(&BufferDesc::new(16,
            (ash::vk::BufferUsageFlags::STORAGE_BUFFER | ash::vk::BufferUsageFlags::TRANSFER_DST).as_raw(), 1),
            "cursor retry fixture", None).unwrap();
        let writer = Arc::new(Writer::default());
        let mut upload = CustomCursorUpload {
            buffer: host.managed_buffer(&handle).unwrap(), writer: writer.clone(), pending: [0; 16], dirty: 0,
        };
        // Creation itself must not initialize unknown position or visibility.
        upload.flush().unwrap();
        assert!(writer.accepted.lock().unwrap().is_empty());
        writer.results.lock().unwrap().extend([Err(rhapsody::Error::Timeout), Err(rhapsody::Error::Timeout)]);
        upload.write(12, &24f32.to_le_bytes()).unwrap();
        upload.write(8, &1u32.to_le_bytes()).unwrap();
        upload.flush().unwrap();
        assert_eq!(*writer.accepted.lock().unwrap(), [(8, [1u32.to_le_bytes(), 24f32.to_le_bytes()].concat())]);
        writer.accepted.lock().unwrap().clear();
        // Disjoint words retain only the rejected suffix after partial admission.
        writer.results.lock().unwrap().extend([Err(rhapsody::Error::Timeout), Ok(()), Err(rhapsody::Error::Timeout)]);
        upload.write(0, &7f32.to_le_bytes()).unwrap();
        upload.write(12, &32f32.to_le_bytes()).unwrap();
        assert_eq!(upload.dirty, 8);
        upload.flush().unwrap();
        assert_eq!(*writer.accepted.lock().unwrap(), [(0, 7f32.to_le_bytes().to_vec()), (12, 32f32.to_le_bytes().to_vec())]);
        writer.accepted.lock().unwrap().clear();
        writer.results.lock().unwrap().push_back(Err(rhapsody::Error::Exhausted("cursor fixture".into())));
        upload.write(0, &[9; 16]).unwrap();
        upload.flush().unwrap();
        assert_eq!(*writer.accepted.lock().unwrap(), [(0, vec![9; 16])]);
        writer.results.lock().unwrap().push_back(Err(rhapsody::Error::invalid_usage("fatal cursor fixture")));
        assert!(upload.write(8, &0u32.to_le_bytes()).is_err());
        assert_eq!(upload.dirty, 4);
        drop(upload); // An owner replacement cancels unaccepted bytes.
        assert_eq!(writer.accepted.lock().unwrap().len(), 1);
    }

    #[test]
    fn failed_callback_blocks_every_already_queued_renderer_and_input_callback() {
        let state = LoopLifecycle::default();
        let error = state.dispatch(|| Err(std::io::Error::new(
            std::io::ErrorKind::NotFound, "first actual shader path").into()))
            .unwrap_err().context("initial world preparation");
        // This is the same ordering as run(): latch the cause before shutdown.
        state.record_failure(error);
        let invoked = std::cell::Cell::new(0);
        for _queued in ["RedrawRequested", "AboutToWait", "Resumed", "Suspended",
            "Resized", "Focused", "CloseRequested", "MouseMotion"] {
            state.dispatch(|| {
                invoked.set(invoked.get() + 1);
                Err(anyhow::anyhow!("closed renderer must not be entered"))
            }).unwrap();
        }
        assert_eq!(invoked.get(), 0);
        state.record_failure(anyhow::anyhow!("secondary GPU cleanup failure"));
        let first = state.take_failure().unwrap();
        assert_eq!(first.downcast_ref::<std::io::Error>().unwrap().kind(), std::io::ErrorKind::NotFound);
        assert!(format!("{first:#}").contains("first actual shader path"));
        assert!(!format!("{first:#}").contains("secondary GPU"));
    }

    #[test]
    fn callback_panic_latches_terminal_before_cleanup_without_removing_assertions() {
        // Default test-runner hook permits unwinding. Production launch's
        // reporting/abort hook intentionally cannot reach this recovery path.
        let state = LoopLifecycle::default();
        let error = state.dispatch(|| {
            assert!(!state.is_running(), "device_ready original assertion");
            Ok(())
        }).unwrap_err();
        assert!(error.to_string().contains("device_ready original assertion"));
        assert!(!state.is_running());
        state.dispatch(|| panic!("queued redraw after panic")).unwrap();
    }

    #[test]
    fn normal_callbacks_run_until_close_and_cleanup_is_idempotent() {
        let state = LoopLifecycle::default();
        let mut callbacks = 0;
        for _ in 0..3 {
            state.dispatch(|| { callbacks += 1; Ok(()) }).unwrap();
        }
        assert_eq!(callbacks, 3);
        assert_eq!(state.begin_cleanup(), Some(true));
        assert!(!state.is_running());
        state.renderer_stopped();
        for _ in 0..3 {
            assert_eq!(state.begin_cleanup(), None);
            state.dispatch(|| { callbacks += 1; Ok(()) }).unwrap();
        }
        assert_eq!(callbacks, 3);
        assert!(state.take_failure().is_none());
    }

    #[test]
    fn cleanup_failure_allows_only_gpu_retry_never_cpu_owner_replay_or_resume() {
        let state = LoopLifecycle::default();
        assert_eq!(state.begin_cleanup(), Some(true));
        state.record_failure(anyhow::anyhow!("save failed during shutdown"));
        state.record_failure(anyhow::anyhow!("GPU wait timed out"));
        assert_eq!(state.begin_cleanup(), Some(false));
        state.dispatch(|| panic!("resume while GPU shutdown is pending")).unwrap();
        state.renderer_stopped();
        assert_eq!(state.begin_cleanup(), None);
        assert_eq!(state.take_failure().unwrap().to_string(), "save failed during shutdown");
    }

    #[test]
    fn shutdown_callback_panics_are_errors_and_do_not_replace_an_earlier_cause() {
        // This covers embedding/test hosts that unwind, not normal launch's
        // process-aborting panic reporter.
        let state = LoopLifecycle::default();
        state.record_failure(anyhow::anyhow!("original render failure"));
        let mut reached = Vec::new();
        for phase in ["preparation", "save", "screenshot", "GPU"] {
            let result = catch_host_callback(phase, || -> anyhow::Result<()> {
                reached.push(phase);
                if phase == "save" { panic!("save writer panic"); }
                Ok(())
            });
            if let Err(error) = result { state.record_failure(error); }
        }
        assert_eq!(reached, ["preparation", "save", "screenshot", "GPU"]);
        assert_eq!(state.take_failure().unwrap().to_string(), "original render failure");
    }

    #[test]
    fn zero_extent_gate_uses_the_actual_frame_config_even_when_window_size_differs() {
        use country_core::render::render_config::{FrameRenderConfig, VideoSettings};
        use country_core::resources::window::WindowSize;
        let mut world = World::new();
        world.insert_resource(WindowSize::new(0, 0));
        world.insert_resource(FrameRenderConfig::at_startup(
            WindowSize::new(1280, 720), &VideoSettings::default()));
        assert!(!frame_waits_for_window_extent(&world));
        world.resource_mut::<WindowSize>().set(1280, 720);
        for extent in [[0, 720], [1280, 0], [0, 0]] {
            world.resource_mut::<FrameRenderConfig>().extent_0 = extent;
            assert!(frame_waits_for_window_extent(&world));
        }
        world.resource_mut::<FrameRenderConfig>().extent_0 = [1280, 720];
        assert!(!frame_waits_for_window_extent(&world));
    }

    #[test]
    fn gameplay_catalog_requirement_is_independent_of_loading_state_and_camera() {
        use country_core::app_loading_state::AppLoadingState;
        for state in [AppLoadingState::Splash, AppLoadingState::Game, AppLoadingState::Reload] {
            let mut app = App::new();
            app.insert_resource(crate::run_game::GameplayApplication);
            app.insert_resource(bevy_state::state::State::new(state));
            // An empty negative fixture must fail; do not install a fake
            // catalog to pretend this small App completed real startup.
            let error = update_application(&mut app).unwrap_err();
            assert!(error.to_string().contains("ClutterMeshes"));
        }
        let mut separate_loading = App::new();
        assert!(!separate_loading.world().contains_resource::<crate::run_game::GameplayApplication>());
        update_application(&mut separate_loading).unwrap();
    }

    #[test]
    fn synchronous_application_update_returns_the_actual_panic_text() {
        let mut app = App::new();
        app.add_systems(bevy_app::Startup, || -> () { panic!("required asset failed at its actual path"); });
        let error = update_application(&mut app).unwrap_err();
        assert!(error.to_string().contains("required asset failed at its actual path"));
    }

    fn vao_test_app() -> (App, country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>) {
        use country_core::{asset_libraries::{runtime::AssetLibraryRuntime,
            mesh_library::{AssetMeshLibrary, MeshAsset}, vao_library::AssetVAOLibrary},
            render::{mesh::Mesh, test_device::TestDevice}};
        let mut app = App::new();
        let device = std::sync::Arc::new(TestDevice(1));
        app.insert_resource(AssetLibraryRuntime::new(device.clone(), device));
        let meshes = AssetMeshLibrary::new();
        let mut mesh = Mesh::new("loading-update-publication-test");
        mesh.indices = vec![0, 1, 2];
        let handle = meshes.add(MeshAsset::new(mesh, None));
        app.insert_resource(meshes);
        app.insert_resource(AssetVAOLibrary::new());
        app.world_mut().spawn(handle);
        (app, handle)
    }

    #[test]
    fn asset_registration_is_per_app_and_loading_update_applies_vao_commands() {
        use country_core::{asset_libraries::Handle, render::vao::Vao};
        // Each separate App represents an independent loading/gameplay owner.
        // No Nani systems are installed here: the loading Update end applies.
        for _ in 0..2 {
            let (mut app, _) = vao_test_app();
            install_asset_library_systems(&mut app);
            install_asset_library_systems(&mut app);
            for name in ["build_missing_vaos", "rebuild_vaos"] {
                let count = app.get_schedule(bevy_app::Update).unwrap().graph().systems()
                    .filter(|(_, system, _)| system.name().ends_with(name)).count();
                assert_eq!(count, 1, "{name}");
                let post_count = app.get_schedule(bevy_app::PostUpdate).unwrap().graph().systems()
                    .filter(|(_, system, _)| system.name().ends_with(name)).count();
                assert_eq!(post_count, 0, "{name} remained in PostUpdate");
            }
            app.update();
            let mut query = app.world_mut().query::<&Handle<Vao>>();
            assert_eq!(query.iter(app.world()).count(), 1);
        }
    }

    #[test]
    fn update_runs_once_with_retained_input_and_no_collect_admission() {
        #[derive(bevy_ecs::resource::Resource, Default)]
        struct Updates(u32);
        let (mut app, _) = vao_test_app();
        app.init_resource::<Updates>();
        app.add_systems(bevy_app::Update, |mut updates: bevy_ecs::prelude::ResMut<Updates>| updates.0 += 1);
        let mut driver = GameFrameDriver::new(app, egui::Context::default());
        // Input retained by an earlier yield is merged ahead of the newer input.
        driver.pending_collect_input = Some(egui::RawInput { events: vec![egui::Event::Text("first".into())], ..Default::default() });
        let input = egui::RawInput { events: vec![egui::Event::Text("second".into())], ..Default::default() };
        let mut outputs = 0;
        assert!(driver.update_if_admitted(Some(input), |_| outputs += 1).unwrap());
        assert_eq!(outputs, 1);
        assert_eq!(driver.app.world().resource::<Updates>().0, 1);
        assert!(!driver.waiting_for_collect);
        assert!(driver.pending_collect_input.is_none());
    }
    #[test]
    fn pending_loading_or_render_result_resumes_without_consuming_another_update_or_egui_input() {
        #[derive(bevy_ecs::resource::Resource, Default)]
        struct UpdateCount(u32);
        let (mut app, _) = vao_test_app();
        app.init_resource::<UpdateCount>();
        app.add_systems(bevy_app::Update, |mut count: bevy_ecs::prelude::ResMut<UpdateCount>| count.0 += 1);
        let mut driver = GameFrameDriver::new(app, egui::Context::default());
        let mut outputs = 0;
        driver.update_before_render(Some(egui::RawInput::default()), |_| outputs += 1).unwrap();
        assert!(driver.input_consumed_this_call);
        assert_eq!(driver.app.world().resource::<UpdateCount>().0, 1);
        driver.loading_render_pending_after_update = true;
        for _ in 0..3 {
            driver.update_before_render(Some(egui::RawInput::default()), |_| outputs += 1).unwrap();
            assert!(!driver.input_consumed_this_call, "new OS input must remain for the next update");
        }
        assert_eq!(outputs, 1, "a suspended frame owns exactly one egui begin/end");
        assert_eq!(driver.app.world().resource::<UpdateCount>().0, 1);
        assert_eq!(driver.progress().completed_frames, 0);
        driver.loading_render_pending_after_update = false;
        driver.awaiting_render = true;
        for _ in 0..3 {
            driver.update_before_render(Some(egui::RawInput::default()), |_| outputs += 1).unwrap();
            assert!(!driver.input_consumed_this_call);
        }
        assert_eq!(outputs, 1);
        assert_eq!(driver.app.world().resource::<UpdateCount>().0, 1);
        driver.awaiting_render = false;
        driver.update_before_render(None, |_| panic!("no egui input")).unwrap();
        assert!(driver.input_consumed_this_call);
        assert_eq!(driver.app.world().resource::<UpdateCount>().0, 2);
    }

    #[test]
    fn post_update_modified_queue_is_consumed_by_the_next_update_prefix() {
        use bevy_ecs::{prelude::{ResMut, Resource}, schedule::IntoScheduleConfigs};
        use country_core::asset_libraries::{Handle, mesh_library::AssetMeshLibrary,
            vao_library::AssetVAOLibrary, runtime::AssetMaintenance};
        use country_core::render::mesh::Mesh;
        #[derive(Resource)]
        struct OnePostUpdateChange { mesh: Handle<Mesh>, done: bool }
        fn modify_mesh(mut change: ResMut<OnePostUpdateChange>, mut meshes: ResMut<AssetMeshLibrary>) {
            if !change.done {
                meshes.get_mut(change.mesh).unwrap().indices = vec![0, 1, 2, 0, 2, 1];
                change.done = true;
            }
        }
        let (mut app, mesh) = vao_test_app();
        install_asset_library_systems(&mut app);
        app.insert_resource(OnePostUpdateChange { mesh, done: false });
        // Exercise the existing phase with a one-shot real modified-queue
        // producer; this is not a test of the file watcher or JSON loader.
        app.add_systems(bevy_app::PostUpdate, modify_mesh.in_set(AssetMaintenance::Reload));
        app.update();
        assert_eq!(app.world().resource::<AssetVAOLibrary>().get(mesh).unwrap().index_count, 3);
        assert_eq!(app.world().resource::<AssetMeshLibrary>().modified_meshes, vec![mesh]);
        app.update();
        assert_eq!(app.world().resource::<AssetVAOLibrary>().get(mesh).unwrap().index_count, 6);
        assert!(app.world().resource::<AssetMeshLibrary>().modified_meshes.is_empty());
    }

    #[test]
    fn menu_cap_preserves_normal_game_limits_and_handles_disabled_or_nan_limits() {
        for fps in [0.0, -0.0, -1.0, 30.0, 60.0, 144.0, f32::INFINITY, f32::NAN] {
            assert_eq!(
                apply_menu_fps_limit(fps, false, false).to_bits(),
                fps.to_bits()
            );
            assert_eq!(
                apply_menu_fps_limit(fps, true, true).to_bits(),
                fps.to_bits()
            );
            let expected = if fps > 0.0 && fps < 60.0 { fps } else { 60.0 };
            assert_eq!(apply_menu_fps_limit(fps, true, false), expected);
        }
    }

    #[test]
    fn menu_cap_reads_the_current_world_resources() {
        let mut world = World::new();
        let mut config = GameConfig::default();
        config.video.fps_limit = 144.0;
        world.insert_resource(config);
        assert_eq!(frame_target_fps(&world), 144.0);
        world.insert_resource(EscMenuState::Main);
        assert_eq!(frame_target_fps(&world), 60.0);
        world.insert_resource(EscMenuState::Closed);
        assert_eq!(frame_target_fps(&world), 144.0);
        world.resource_mut::<GameConfig>().video.fps_limit = 30.0;
        world.insert_resource(EscMenuState::Main);
        assert_eq!(frame_target_fps(&world), 30.0);
    }

    #[test]
    fn debug_window_flags_choose_actual_os_egui_input() {
        use country_core::resources::gui::DebugGui;
        let mut world = World::new();
        assert!(!frame_egui_input_enabled(&world));
        world.insert_resource(DebugGui::default());
        assert!(!frame_egui_input_enabled(&world));
        world.resource_mut::<DebugGui>().enabled = true;
        assert!(frame_egui_input_enabled(&world));
        world.resource_mut::<DebugGui>().enabled = false;
        world.resource_mut::<DebugGui>().experimental_window = true;
        assert!(frame_egui_input_enabled(&world));
        world.resource_mut::<DebugGui>().experimental_window = false;
        assert!(!frame_egui_input_enabled(&world));
    }

    #[test]
    fn camera_resource_presence_selects_the_native_frame_mode() {
        let mut world = World::new();
        assert_eq!(frame_render_mode(&world), RenderMode::LoadingScreen);
        world
            .insert_resource(country_core::systems::camera_rig::main_camera::MainCamera::default());
        assert_eq!(frame_render_mode(&world), RenderMode::WorldGpuProfiled);
        world.remove_resource::<country_core::systems::camera_rig::main_camera::MainCamera>();
        assert_eq!(frame_render_mode(&world), RenderMode::LoadingScreen);
    }
}
