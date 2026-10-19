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
use crate::render_loop::{FramePacer, RenderHost, RenderMode};
use crate::setup::FromWinit;

/// Builds the existing core RenderHost against the world *after* App::update.
/// The host can borrow that exact World and the EguiBackend for this frame.
/// Implementations must supply the actual frame/pass/WSI adapter; this trait
/// does not declare the missing production adapter implemented.
pub trait GameRenderHostFactory {
    type Host<'a>: RenderHost<CompileError = rhapsody::Error>
    where
        Self: 'a;

    fn host<'a>(&'a mut self, world: &'a mut World, egui: &'a mut EguiBackend) -> Self::Host<'a>;

    /// Return an actual host-side error before the frame's temporal/log/limiter
    /// effects. Graph errors retain their typed cause through `render`.
    fn finish_render(&mut self) -> anyhow::Result<()>;
    fn last_frame_submitted(&self) -> bool;
    /// A successfully queued present is not a proof of scanout completion.
    fn last_frame_present_queued(&self) -> bool;
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

/// The recovered update/render section of main_loop. This owns real Bevy/egui
/// state; it is not a native-layout mirror of the 0x1118-byte GameLoop.
pub struct GameFrameDriver {
    app: App,
    egui: EguiBackend,
    pacer: FramePacer,
    limiter: crate::fps_limiter::FpsLimiter,
    progress: FrameProgress,
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

impl GameFrameDriver {
    pub fn new(mut app: App, context: egui::Context) -> Self {
        // GUI consumers and the backend must see the same Context identity.
        app.world_mut()
            .insert_resource(country_core::resources::gui::EguiContextResource(
                context.clone(),
            ));
        install_asset_library_systems(&mut app);
        Self {
            app,
            egui: EguiBackend::new(context),
            pacer: FramePacer::default(),
            limiter: crate::fps_limiter::FpsLimiter::new(),
            progress: FrameProgress::default(),
        }
    }

    pub fn app(&self) -> &App {
        &self.app
    }
    pub fn app_mut(&mut self) -> &mut App {
        &mut self.app
    }

    pub fn progress(&self) -> FrameProgress {
        self.progress
    }

    /// B begin_frame@0x1404de115 → App::update@0x1404de349 →
    /// end_frame@0x1404de3d2 → platform output send@0x1404de464 →
    /// render@0x1404de5bc → first-frame log@0x1404de640 → limiter@0x1404de7da.
    ///
    /// input=None follows the original no-egui branch: update/render still run,
    /// without a begin/end pair or a platform message. OS events and Steam
    /// callbacks must be resolved by the full host loop.
    pub fn frame<F: GameRenderHostFactory>(
        &mut self,
        input: Option<egui::RawInput>,
        factory: &mut F,
        publish_platform_output: impl FnOnce(egui::PlatformOutput),
    ) -> anyhow::Result<()> {
        let has_gui_input = input.is_some();
        if let Some(input) = input {
            self.egui.context().begin_frame(input);
        }
        {
            puffin::profile_scope!("App update");
            update_application(&mut self.app)?;
        }
        if has_gui_input {
            publish_platform_output(self.egui.end_frame());
        }
        let mode = frame_render_mode(self.app.world());
        let gameplay_frame = mode == RenderMode::WorldGpuProfiled
            && self.app.world().get_resource::<bevy_state::state::State<
                country_core::app_loading_state::AppLoadingState,
            >>().is_some_and(|state| {
                *state.get() == country_core::app_loading_state::AppLoadingState::Game
            });
        let render_result = {
            let mut host = factory.host(self.app.world_mut(), &mut self.egui);
            crate::render_loop::render(&mut host, &mut self.pacer, mode)
        };
        // Drain the adapter even when the graph returns an error, preserving the
        // graph's original cause while still completing host-side error cleanup.
        let host_result = factory.finish_render();
        render_result?;
        host_result?;
        self.progress.completed_frames = self.progress.completed_frames.wrapping_add(1);
        if gameplay_frame {
            self.progress.gameplay_frames = self.progress.gameplay_frames.wrapping_add(1);
            if factory.last_frame_submitted() {
                self.progress.gameplay_submissions =
                    self.progress.gameplay_submissions.wrapping_add(1);
            }
            if factory.last_frame_present_queued() {
                self.progress.gameplay_presents_queued =
                    self.progress.gameplay_presents_queued.wrapping_add(1);
            }
        }
        // Original @1404de63a increments only after render returns. The
        // independent loading World intentionally has no gameplay temporal.
        if let Some(mut temporal) = self
            .app
            .world_mut()
            .get_resource_mut::<country_core::render::RenderTemporalState>()
        {
            temporal.frame_render_returned();
        }
        crate::main::log_time_to_first_frame();
        self.limiter.frame(frame_target_fps(self.app.world()));
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
    pub(crate) renderer: crate::production_renderer::ProductionRenderer,
    pub(crate) options: GameLoopOptions,
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
    custom_cursor_buffer: Option<rhapsody::resource::ManagedBuffer>,
    pub(crate) frame_count: u64,
    pub(crate) suspended: bool,
    pub(crate) shutdown_done: bool,
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
    pub fn new(
        mut app: App,
        window: std::sync::Arc<winit::window::Window>,
        options: GameLoopOptions,
        shaders: std::sync::Arc<country_core::asset_libraries::shader_library::AssetShaderLibrary>,
    ) -> anyhow::Result<Self> {
        // Input stream's native serialization is not represented by the current
        // first-frame-only RecordingFile; never silently record an incomplete file.
        anyhow::ensure!(
            options.input_recording.is_none(),
            "Original full input-stream recording/replay schema is not recovered"
        );
        if !app.is_plugin_added::<bevy_input::InputPlugin>() {
            app.add_plugins(bevy_input::InputPlugin);
        }
        use country_core::resources::window::*;
        register_platform_events(&mut app);
        let size = window.inner_size();
        app.world_mut()
            .insert_resource(WindowSize::new(size.width, size.height));
        register_settings_application_system(&mut app);
        let mut renderer = crate::production_renderer::ProductionRenderer::new(
            window.clone(),
            &options.compiled_shader_root,
            shaders,
        )?;
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
        if !app
            .world()
            .resource::<country_core::game_config::GameConfig>()
            .dev
            .skip_splash
        {
            // B owns two different Apps. Their libraries share only the actual
            // graphics device; no MainCamera is removed to fake a loading mode.
            let mut loading = crate::run_game::make_loading_app(&app)?;
            loading.insert_resource(EguiContextResource(shared_gui));
            #[cfg(windows)]
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
            let gameplay = std::mem::take(app.main_mut());
            preparation = Some(crate::run_game::GameplayPreparation::spawn(
                gameplay,
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
            #[cfg(windows)]
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
            #[cfg(target_os = "macos")]
            app.add_systems(
                bevy_app::Update,
                country_core::resources::logo_animation::animate_logo_with_disabled_audio,
            );
            #[cfg(not(any(windows, target_os = "macos")))]
            anyhow::bail!(
                "AnimatedLogo requires a real FMOD AudioCore startup backend on this target"
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
        Ok(Self {
            startup_instant: Some(std::time::Instant::now()),
            driver: GameFrameDriver::new(app, context),
            window,
            egui_io,
            renderer,
            options,
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
            shutdown_done: false,
            pending_os_events: Default::default(),
            preparation,
            custom_cursor_events: Default::default(),
            capture_events: Default::default(),
            visibility_events: Default::default(),
            window_events: Default::default(),
            settings_events: Default::default(),
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
        &self,
    ) -> Option<&crate::production_renderer::ProductionFrameObservation> {
        self.renderer.frame_observation()
    }

    /// Nonblocking query of the exact current present ticket. None is unknown,
    /// including a ticket already reclaimed by normal frame maintenance.
    pub fn frame_present_complete(&self) -> rhapsody::Result<Option<bool>> {
        self.renderer.frame_present_complete()
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
            lease.persistent_host_ptr() == Some(created.0.as_ptr().cast::<u8>()),
            "CustomCursor Created pointer does not match its canonical managed buffer"
        );
        self.custom_cursor_buffer = Some(lease);
        Ok(())
    }

    /// The startup event carries only a pointer; the actual long-lived owner is
    /// the canonical App library plus this same BufferHost's managed lease.
    fn resolve_custom_cursor_buffer(&self) -> anyhow::Result<rhapsody::resource::ManagedBuffer> {
        use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
        use country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo;
        let core_buffer = self
            .driver
            .app
            .world()
            .resource::<AssetSsboLibrary>()
            .get_buffer::<CustomCursorSsbo>()
            .ok_or_else(|| anyhow::anyhow!("CustomCursor has no canonical buffer in its World"))?;
        let lease = self
            .renderer
            .bridge()
            .hosts()
            .buffers
            .managed_buffer(core_buffer)
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
        Ok(lease)
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
        if let Some(buffer) = &self.custom_cursor_buffer {
            let data = country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo {
                pos: [position[0] as f32, position[1] as f32],
                enabled: u32::from(self.visibility_suppressions == 0),
                size_px: self.custom_cursor_size_px,
            };
            // Current Rhapsody API waits only for this buffer's actual in-flight
            // uses, retaining its non-coherent flush and host-access tracking.
            buffer.write_bytes(0, bytemuck::bytes_of(&data))?;
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
                if let Some(buffer) = &self.custom_cursor_buffer {
                    buffer.write_bytes(12, &size_px.to_ne_bytes())?;
                }
            }
            OsCursorVisibilityCmd::Suppress(origin) => {
                assert_ne!(origin.0, 0);
                let previous = self.visibility_suppressions;
                self.visibility_suppressions |= origin.0;
                if previous == 0 {
                    if self.custom_cursor_mode {
                        if let Some(buffer) = &self.custom_cursor_buffer {
                            buffer.write_bytes(8, &0u32.to_ne_bytes())?;
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
                        if let Some(buffer) = &self.custom_cursor_buffer {
                            buffer.write_bytes(8, &1u32.to_ne_bytes())?;
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
    pub(crate) fn frame(&mut self, target: &EventLoopWindowTarget<()>) -> anyhow::Result<bool> {
        if let Some(preparation) = &mut self.preparation {
            // No OS callback can run while poll owns this thread. Withdraw the
            // old writer before poll can swap Worlds; a failed/pending poll
            // cannot accidentally write through another App's pointer.
            let old_cursor_lease = self.custom_cursor_buffer.take();
            if preparation.poll(self.driver.app_mut())? {
                drop(old_cursor_lease);
                self.reset_world_event_cursors();
                self.renderer
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
        // B @1404dc952..971 skips both simulation and rendering while either
        // dimension is zero. Keep input edges for the next actual update.
        let window = self
            .driver
            .app
            .world()
            .resource::<country_core::resources::window::WindowSize>();
        if window.width == 0 || window.height == 0 {
            // Main-thread host adaptation: an actual OS close may stop a zero-
            // sized window without simulating, rendering or inventing the game
            // shutdown marker. Sender disconnect still joins the real saver.
            if os_close_requested {
                return Ok(false);
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            return Ok(true);
        }
        // B 1404ddf28..3c: real OS egui input is consumed only while the
        // debug or experimental window is active. Other frames still begin
        // and end egui with RawInput::default (1404ddf49 -> 140c2c770).
        let input = if frame_egui_input_enabled(self.driver.app.world()) {
            self.egui_io.take_egui_input(&self.window)
        } else {
            egui::RawInput::default()
        };
        let egui_io = &self.egui_io;
        let window = &self.window;
        self.driver
            .frame(Some(input), &mut self.renderer, |output| {
                egui_io.handle_platform_output(window, output);
            })?;
        // 本帧消费完成后清除按下和释放边沿，保留仍按住的按钮。
        if let Some(mut keys) = self.driver.app.world_mut().get_resource_mut::<country_core::systems::backend::input::button_input::KeyButtonInput>() {
            keys.clear();
        }
        if let Some(mut mouse) = self.driver.app.world_mut().get_resource_mut::<country_core::systems::backend::input::button_input::MouseButtonInput>() {
            mouse.clear();
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
            let config = &self
                .driver
                .app
                .world()
                .resource::<country_core::game_config::GameConfig>()
                .video;
            if flags & 2 != 0 {
                self.renderer.set_vsync(config.vsync_enable)?;
            }
            if flags & 5 != 0 {
                crate::event_loop::apply_display_config(&self.window, &config.display)?;
            }
            self.driver
                .app
                .world_mut()
                .send_event(crate::app_config::WriteSettingsFileCmd);
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
    /// Fullscreen restoration and native input serialization remain separately
    /// documented original branches; no synthetic file format is emitted here.
    pub(crate) fn shut_down(&mut self) -> anyhow::Result<()> {
        if self.shutdown_done {
            return Ok(());
        }
        // A worker can still be declaring/executing the gameplay warm-up with
        // these exact GPU owners. Stop and join it before touching renderer
        // lifetime, and do not abandon GPU cleanup when it reports an error.
        let preparation_result = self
            .preparation
            .take()
            .map_or(Ok(()), |mut preparation| preparation.cancel_and_join());
        let save_result = country_core::systems::save::save_thread::shutdown_save_thread(
            self.driver.app_mut().world_mut(),
        );
        self.custom_cursor_buffer = None;
        self.window
            .set_cursor_grab(winit::window::CursorGrabMode::None)
            .ok();
        self.window.set_cursor_visible(true);
        let renderer_result = self.renderer.shutdown();
        let mut failure = preparation_result.err();
        for (phase, result) in [("save thread", save_result), ("GPU", renderer_result)] {
            if let Err(error) = result {
                if failure.is_some() {
                    log::error!("Additional {phase} shutdown failure: {error:#}");
                } else {
                    failure = Some(error);
                }
            }
        }
        if let Some(error) = failure {
            Err(error)
        } else {
            self.shutdown_done = true;
            Ok(())
        }
    }

    /// B 0x1404e2470: enumerate current monitors and sorted public video modes.
    pub fn extract_available_monitors_info(
        available: &mut AvailableMonitors,
        event_loop: &EventLoopWindowTarget<()>,
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
        if !self.shutdown_done {
            if let Err(error) = self.shut_down() {
                log::error!("game-loop shutdown: {error:#}");
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
