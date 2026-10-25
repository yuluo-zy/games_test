//! Production owner for the recovered outer render loop (B 0x1404e8010).
//!
//! Declaration and physical compilation precede command execution; acquisition
//! follows the main graph; the display submission waits for both acquire and
//! actual main-graph timeline receipts. Native shader consumers are shared with
//! core. CAMetalLayer and the external swapchain attachment are host adapters.
use std::{collections::{BTreeMap, VecDeque}, path::{Path, PathBuf}, sync::Arc, time::{Duration, Instant}};

use anyhow::{Context, Result};
use ash::vk;
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::shader_library::AssetShaderLibrary,
    game_config::GameConfig,
    render::render_config::VideoSettings,
    systems::render_loop::{
        draw_passes::draw_lists::{DrawLists, build_draw_lists},
        render_frame::{Acquire, Level, RenderHost, SwapchainInfo},
    },
};
use rhapsody::{
    raw::{instance::InstanceBuilder, swapchain::{AcquiredImageLease, GraphImageLease, PresentTicket, Swapchain}},
    rg::vulkan::SessionFrameReceipt,
};

use crate::{egui_backend::EguiBackend, egui_gpu::EguiGpu,
    game_loop::GameRenderHostFactory, render_bridge::{CompiledCoreGraph, CoreRenderBridge}};

#[path = "production_resolve.rs"]
mod resolve;

const TIMEOUT_NS: u64 = 10_000_000_000;

/// Original loader closure 0x1400d7440 must successfully render the gameplay
/// App once without a swapchain before publishing it. The unresolved original
/// world consumer is a failure here too; it cannot signal a false ready App.
pub fn warm_up_gameplay(
    _device: Arc<rhapsody::device::Device>, _compiled_shader_root: PathBuf,
    _shaders: Arc<AssetShaderLibrary>, _world: &mut World,
) -> Result<()> {
    Err(world_consumer_missing().into())
}

fn world_consumer_missing() -> rhapsody::Error {
    rhapsody::Error::unsupported("original compile_main 0x140161cd0: world FramePasses consumer is not connected")
}

struct ApplicationGpu {
    bridge: CoreRenderBridge,
    egui: EguiGpu,
    shaders: Arc<AssetShaderLibrary>,
    final_post_shader: Option<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>>,
}
impl ApplicationGpu {
    fn new(device: Arc<rhapsody::device::Device>, shader_root: &Path, shaders: Arc<AssetShaderLibrary>) -> Result<Self> {
        // Original CompiledFilePath already contains "compiled-shaders/".
        // This boundary accepts that directory explicitly; pass its owning
        // installation root to the file backend once, for either App.
        let asset_root = shader_root.parent().context("compiled shader directory has no installation root")?;
        let bridge = CoreRenderBridge::new(device.clone(), asset_root, shaders.clone())?;
        let mut final_post_shader = None;
        country_core::startup::load_shaders::load_shaders(&shaders, bridge.shader_backend(), &mut final_post_shader);
        country_core::startup::load_shaders::load_ui_shaders(&shaders, bridge.shader_backend());
        country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());
        struct VermeerShaders<'a> { library: &'a AssetShaderLibrary,
            backend: &'a dyn country_core::render::shader::boundary::ShaderBackend }
        impl country_core::systems::render_loop::painterly::ShaderLoader for VermeerShaders<'_> {
            fn load_shader_into_library(&mut self, path: &'static str, name: &'static str) {
                country_core::startup::load_shaders::load_shader_into_library(self.library, self.backend, path, name);
            }
        }
        country_core::systems::render_loop::vermeer::startup_vermeer(&mut VermeerShaders {
            library: &shaders, backend: bridge.shader_backend() });
        let egui = EguiGpu::new(device, bridge.hosts().clone());
        Ok(Self { bridge, egui, shaders, final_post_shader })
    }
}

pub struct ProductionRenderer {
    bridge: CoreRenderBridge,
    egui: EguiGpu,
    swapchain: Arc<Swapchain>,
    device: Arc<rhapsody::device::Device>,
    shaders: Arc<AssetShaderLibrary>,
    final_post_shader: Option<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>>,
    other_applications: BTreeMap<usize, ApplicationGpu>,
    compiled_shader_root: PathBuf,
    pending: VecDeque<PresentTicket>,
    acquired: Option<AcquiredImageLease>,
    presentation: Option<Arc<parking_lot::Mutex<GraphImageLease>>>,
    main: Option<(u32, SessionFrameReceipt)>,
    main_output: Option<Arc<rhapsody::raw::texture::Texture>>,
    started: Instant,
    scopes: Vec<usize>,
    first_error: Option<anyhow::Error>,
    suspended: bool,
    closed: bool,
    reduced_state: u64,
    stats: rhapsody::raw::stats::RenderStats,
    last_stats: rhapsody::raw::stats::StatsSnapshot,
    // The native window and CAMetalLayer outlive every Vulkan surface owner.
    platform: PlatformSurface,
    window: Arc<winit::window::Window>,
}

impl ProductionRenderer {
    pub fn new(
        window: Arc<winit::window::Window>,
        compiled_shader_root: impl AsRef<Path>,
        shaders: Arc<AssetShaderLibrary>,
    ) -> Result<Self> {
        let builder = InstanceBuilder { application_name: std::ffi::CString::new("Tiny Glade")?,
            extensions: platform_extensions(), ..Default::default() };
        let instance = builder.build().context("create Vulkan instance")?;
        let platform = PlatformSurface::new(&window)?;
        let surface = platform.create_surface(instance.clone())?;
        let device = rhapsody::device::Device::new(instance, Some(&surface),
            &rhapsody::device::DeviceConfig {
                queues: rhapsody::device::QueueConfig { require_present: true, ..Default::default() },
                ..Default::default()
            }).context("create graphics device for native surface")?;
        let size = window.inner_size();
        let swapchain = Arc::new(unsafe { Swapchain::new(device.clone(), surface,
            vk::Extent2D { width: size.width, height: size.height }, true,
            vk::ImageUsageFlags::COLOR_ATTACHMENT) }?).clone();
        let compiled_shader_root = compiled_shader_root.as_ref().to_path_buf();
        let ApplicationGpu { bridge, egui, shaders, final_post_shader } =
            ApplicationGpu::new(device.clone(), &compiled_shader_root, shaders)?;
        let properties = unsafe { device.instance.instance.get_physical_device_properties(device.physical) };
        log::info!("Graphics device: {}", unsafe {
            std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy());
        Ok(Self { bridge, egui, swapchain, device, shaders, final_post_shader,
            other_applications: BTreeMap::new(), compiled_shader_root, pending: VecDeque::new(),
            acquired: None, presentation: None, main: None, main_output: None, started: Instant::now(),
            scopes: Vec::new(), first_error: None, suspended: false, closed: false, reduced_state: 0,
            stats: Default::default(), last_stats: Default::default(), platform, window })
    }

    pub fn device(&self) -> &Arc<rhapsody::device::Device> { &self.device }
    pub fn bridge(&self) -> &CoreRenderBridge { &self.bridge }
    pub fn bridge_mut(&mut self) -> &mut CoreRenderBridge { &mut self.bridge }
    /// B creates independent render libraries for loading and gameplay Apps,
    /// cloning only the graphics device. Preserve each App's real library and
    /// dynamic-constant owner rather than aliasing their same-numbered handles.
    pub fn activate_application(&mut self, world: &World) -> Result<()> {
        let shaders = world.get_resource::<crate::run_game::ApplicationShaderLibrary>()
            .context("App has no original shader-library owner")?.0.clone();
        if Arc::ptr_eq(&self.shaders, &shaders) {
            if let Some(completed) = world.get_resource::<crate::render_bridge::CompletedGameplayWarmup>() {
                self.bridge.accept_gameplay_warmup(completed)?;
            }
            return Ok(());
        }
        anyhow::ensure!(self.acquired.is_none() && self.presentation.is_none(),
            "cannot switch App while its WSI acquisition is live");
        let key = Arc::as_ptr(&shaders) as usize;
        let previous = Arc::as_ptr(&self.shaders) as usize;
        let mut next = match self.other_applications.remove(&key) {
            Some(next) => next,
            None => {
                anyhow::ensure!(!world.contains_resource::<country_core::asset_libraries::runtime::AssetLibraryRuntime>(),
                    "App already owns GPU libraries but its original render bridge is missing");
                ApplicationGpu::new(self.device.clone(), &self.compiled_shader_root, shaders)?
            },
        };
        std::mem::swap(&mut self.bridge, &mut next.bridge);
        std::mem::swap(&mut self.egui, &mut next.egui);
        std::mem::swap(&mut self.shaders, &mut next.shaders);
        std::mem::swap(&mut self.final_post_shader, &mut next.final_post_shader);
        self.other_applications.insert(previous, next);
        if let Some(completed) = world.get_resource::<crate::render_bridge::CompletedGameplayWarmup>() {
            self.bridge.accept_gameplay_warmup(completed)?;
        }
        self.main_output.take();
        self.main.take();
        Ok(())
    }
    pub fn final_post_shader(&self) -> Option<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>> {
        self.final_post_shader
    }
    pub fn any_present_successful(&self) -> bool { self.swapchain.any_present_successful() }
    pub fn take_error(&mut self) -> Option<anyhow::Error> { self.first_error.take() }
    pub fn resize(&mut self, extent: [u32; 2]) -> Result<()> {
        self.platform.update(&self.window)?;
        self.swapchain.resize(vk::Extent2D { width: extent[0], height: extent[1] })?;
        Ok(())
    }
    pub fn set_vsync(&mut self, enabled: bool) -> Result<()> { self.swapchain.set_vsync(enabled)?; Ok(()) }
    pub fn set_focused(&mut self, focused: bool) -> Result<()> { self.swapchain.set_focused(focused)?; Ok(()) }
    pub fn suspend(&mut self) -> Result<()> {
        self.suspended = true;
        if let Some(image) = self.acquired.take() { image.cancel(TIMEOUT_NS)?; }
        Ok(())
    }
    pub fn resume(&mut self) -> Result<()> {
        self.platform.update(&self.window)?;
        self.suspended = false;
        self.swapchain.mark_stale();
        Ok(())
    }
    fn record_error(&mut self, error: impl Into<anyhow::Error>) {
        if self.first_error.is_none() { self.first_error = Some(error.into()); }
    }
    fn collect(&mut self) -> rhapsody::Result<()> {
        self.bridge.collect_completed_frames()?;
        for app in self.other_applications.values_mut() { app.bridge.collect_completed_frames()?; }
        while let Some(ticket) = self.pending.front() {
            if !ticket.is_complete()? { break; }
            self.pending.pop_front();
        }
        self.swapchain.collect_presentations()?;
        self.device.try_collect_garbage()?;
        Ok(())
    }
    pub fn shutdown(&mut self) -> Result<()> {
        if self.closed { return Ok(()); }
        self.presentation.take();
        if let Some(image) = self.acquired.take() { image.cancel(TIMEOUT_NS)?; }
        self.bridge.wait_submits_done(TIMEOUT_NS)?;
        for app in self.other_applications.values_mut() { app.bridge.wait_submits_done(TIMEOUT_NS)?; }
        for ticket in &self.pending { ticket.wait(TIMEOUT_NS)?; }
        self.pending.clear();
        self.swapchain.shutdown(TIMEOUT_NS)?;
        self.bridge.raster_resources().clear_attachment_cache();
        self.bridge.with_session(|session| session.clear_after_completion())?;
        for app in self.other_applications.values_mut() {
            app.bridge.raster_resources().clear_attachment_cache();
            app.bridge.with_session(|session| session.clear_after_completion())?;
        }
        self.device.wait_idle_and_recycle_resources()?;
        let peak = self.device.with_global_allocator(|allocator| allocator.peak_totals());
        if peak.device_local_bytes != 0 && peak.system_bytes != 0 {
            log::info!("Peak GPU memory use (device local): {} system: {}", peak.device_local_bytes, peak.system_bytes);
        }
        self.closed = true;
        Ok(())
    }
}

impl Drop for ProductionRenderer {
    fn drop(&mut self) {
        if !self.closed {
            if let Err(error) = self.shutdown() { log::error!("graphics shutdown: {error:#}"); }
        }
    }
}

pub struct CompiledGameGraph { graph: CompiledCoreGraph, output: u32 }

pub struct ProductionRenderHost<'a> {
    renderer: &'a mut ProductionRenderer,
    world: &'a mut World,
    egui: &'a mut EguiBackend,
    video: Option<VideoSettings>,
    input_lag_reduction: bool,
    lists: DrawLists,
}

impl GameRenderHostFactory for ProductionRenderer {
    type Host<'a> = ProductionRenderHost<'a>;
    fn host<'a>(&'a mut self, world: &'a mut World, egui: &'a mut EguiBackend) -> Self::Host<'a> {
        self.activate_application(world).expect("B per-App rendering library ownership");
        ProductionRenderHost { renderer: self, world, egui, video: None,
            input_lag_reduction: false, lists: DrawLists::new() }
    }
}

impl RenderHost for ProductionRenderHost<'_> {
    type Compiled = CompiledGameGraph;
    type CompileError = rhapsody::Error;
    fn puffin_scopes_on(&mut self) -> bool { puffin::are_scopes_on() }
    fn scope_begin(&mut self, name: &str) {
        let offset = puffin::ThreadProfiler::call(|profiler| profiler.begin_scope(name, file!(), ""));
        self.renderer.scopes.push(offset);
    }
    fn scope_end(&mut self) {
        if let Some(offset) = self.renderer.scopes.pop() {
            puffin::ThreadProfiler::call(|profiler| profiler.end_scope(offset));
        }
    }
    fn now(&mut self) -> Duration { self.renderer.started.elapsed() }
    fn device_ready(&mut self) -> bool {
        if self.renderer.device.is_device_lost() {
            self.renderer.record_error(rhapsody::Error::DeviceLost);
            return false;
        }
        !self.renderer.closed
    }
    fn clone_video_settings(&mut self) -> bool {
        let Some(config) = self.world.get_resource::<GameConfig>() else { return false; };
        self.video = Some(config.video.clone());
        self.input_lag_reduction = config.ui.input_lag_reduction;
        true
    }
    fn gpu_pacing_enabled(&mut self) -> bool { self.input_lag_reduction }
    fn build_draw_lists(&mut self) {
        build_draw_lists(self.world, &*self.renderer.shaders, &mut self.lists);
        self.lists.sort();
        if let Err(error) = self.renderer.bridge.raster_resources().refresh_draw_vaos(self.world, &self.lists) {
            self.renderer.record_error(error);
        }
    }
    fn swapchain_info(&mut self) -> Option<SwapchainInfo> {
        let extent = self.renderer.swapchain.desc().extent;
        (!self.renderer.suspended && extent.width > 0 && extent.height > 0).then(|| SwapchainInfo {
            format: self.renderer.swapchain.format().as_raw() as u32, extent: [extent.width, extent.height] })
    }
    fn compile_loading_graph(&mut self, swapchain: &SwapchainInfo) -> rhapsody::Result<Self::Compiled> {
        let shaders = self.renderer.shaders.clone();
        let (output, built) = self.renderer.bridge.build_frame(|frame| {
            crate::loading_graph::build_loading_frame(frame, self.world, &shaders, swapchain.extent, swapchain.format, &self.lists)
        })?;
        if let Err(error) = self.renderer.bridge.shader_backend().prewarm(Duration::from_secs(10)) {
            if let Err(cleanup) = self.renderer.bridge.cancel_built(built) {
                log::warn!("pipeline prewarm cancellation: {cleanup}");
            }
            return Err(error);
        }
        let graph = self.renderer.bridge.compile_built(built)?;
        Ok(CompiledGameGraph { graph, output: output.id })
    }
    fn compile_world_graph(&mut self, _swapchain: Option<&SwapchainInfo>, _min_extent: Option<[u32; 2]>)
        -> rhapsody::Result<Self::Compiled> {
        Err(world_consumer_missing())
    }
    fn resolution_scale(&mut self) -> Option<f32> {
        self.world.get_resource::<GameConfig>().map(|config| config.video.world_resolution_scale.unwrap_or(1.0))
    }
    fn set_resolution_scale(&mut self, scale: f32) {
        if let Some(mut config) = self.world.get_resource_mut::<GameConfig>() { config.video.world_resolution_scale = Some(scale); }
    }
    fn clear_resource_pool(&mut self) {
        if let Err(error) = self.renderer.bridge.with_session(|session| session.clear_idle_resource_pool()) {
            self.renderer.record_error(error);
        }
    }
    fn reset_temporal_tables(&mut self) {
        if let Err(error) = self.renderer.bridge.with_session(|session| session.reset_all_temporals()) {
            self.renderer.record_error(error);
        }
    }
    fn wait_idle_and_recycle_resources(&mut self) {
        if let Err(error) = self.renderer.bridge.wait_submits_done(TIMEOUT_NS)
            .and_then(|()| self.renderer.device.wait_idle_and_recycle_resources()) { self.renderer.record_error(error); }
    }
    fn set_reduced_state(&mut self, value: u64) { self.renderer.reduced_state = value; }
    fn log(&mut self, level: Level, message: &str) {
        match level { Level::Error => log::error!(target: crate::render_loop::LOG_TARGET, "{message}"),
            Level::Info => log::info!(target: crate::render_loop::LOG_TARGET, "{message}") }
    }
    fn begin_frame(&mut self, gpu_profiling: bool) {
        let result = self.renderer.bridge.with_session(|session| session.backend.configure_gpu_timing(
            gpu_profiling.then_some(Default::default())));
        if let Err(error) = result { self.renderer.record_error(error); }
    }
    fn drain_finished_frames(&mut self) {
        if let Err(error) = self.renderer.collect() { self.renderer.record_error(error); }
    }
    fn execute_main_graph(&mut self, compiled: Self::Compiled) {
        match self.renderer.bridge.execute_compiled(compiled.graph) {
            Ok(receipt) => match self.renderer.bridge.take_exported_texture(&receipt, compiled.output) {
                Ok(texture) => { self.renderer.main_output = Some(texture); self.renderer.main = Some((compiled.output, receipt)); }
                Err(error) => self.renderer.record_error(error),
            },
            Err(error) => self.renderer.record_error(error),
        }
    }
    fn acquire_swapchain_image(&mut self) -> Acquire {
        if self.renderer.first_error.is_some() { return Acquire::Unavailable; }
        let result = (|| {
            self.renderer.collect()?;
            if self.renderer.pending.len() >= self.renderer.device.frames_in_flight() as usize {
                self.renderer.pending.front().expect("nonempty").wait(TIMEOUT_NS)?;
                self.renderer.collect()?;
            }
            Swapchain::validate_swapchain_and_acquire_image(&mut self.renderer.swapchain)
        })();
        match result { Ok(Some(image)) => { self.renderer.acquired = Some(image); Acquire::Image },
            Ok(None) => Acquire::Unavailable,
            Err(error) => { self.renderer.record_error(error); Acquire::Unavailable } }
    }
    fn execute_presentation_graph(&mut self) -> rhapsody::Result<()> {
        let source = self.renderer.main_output.clone()
            .ok_or_else(|| rhapsody::Error::invalid_usage("main graph did not publish a source image"))?;
        let target = self.renderer.acquired.as_ref()
            .ok_or_else(|| rhapsody::Error::invalid_usage("presentation needs an acquired image"))?.texture()?;
        let shaders = self.renderer.shaders.clone();
        let flag = self.video.as_ref().is_some_and(|video| video.render_upside_down);
        let (_, graph) = self.renderer.bridge.build_frame(|frame| {
            resolve::build(frame, self.world, &shaders, source, target, flag)
        })?;
        if let Err(error) = self.renderer.bridge.shader_backend().prewarm(Duration::from_secs(10)) {
            self.renderer.bridge.cancel_built(graph)?;
            return Err(error);
        }
        let compiled = self.renderer.bridge.compile_built(graph)?;
        let acquired = self.renderer.acquired.take().expect("validated acquired image");
        let presentation = Arc::new(parking_lot::Mutex::new(acquired.into_graph_submission()?));
        self.renderer.presentation = Some(presentation.clone());
        self.renderer.bridge.execute_presentation_compiled(compiled, presentation)?;
        Ok(())
    }
    fn end_frame(&mut self) {
        // The shared session seals its real receipt ledger at execute_prepared;
        // WSI work remains executable until present consumes the acquisition.
        if self.renderer.first_error.is_some() {
            self.renderer.presentation.take();
            if let Some(image) = self.renderer.acquired.take() {
                if let Err(error) = image.cancel(TIMEOUT_NS) { self.renderer.record_error(error); }
            }
        }
    }
    fn present(&mut self) {
        let result = (|| {
            let image = self.renderer.presentation.take().ok_or_else(|| rhapsody::Error::invalid_usage("missing submitted presentation graph"))?;
            let ticket = image.lock().present()?;
            self.renderer.pending.push_back(ticket);
            Ok::<_, rhapsody::Error>(())
        })();
        if let Err(error) = result { self.renderer.record_error(error); }
    }
    fn finish_screenshot_capture(&mut self) {
        // The reconstructed screenshot coordinator still lacks the original
        // per-request GPU texture association. Do not advance requests to a
        // fictional completed state from this production owner.
        if crate::screenshot::has_active_screenshot_requests() {
            self.renderer.record_error(rhapsody::Error::unsupported(
                "original screenshot 0x14031ea20: request-to-GPU-texture association is not recovered"));
        }
    }
    fn snapshot_render_stats(&mut self) { self.renderer.last_stats = self.renderer.stats.snapshot_and_reset(); }
    fn refresh_memory_budget(&mut self) {
        if let Err(error) = self.renderer.device.refresh_memory_budget() { self.renderer.record_error(error); }
    }
    fn last_gpu_frame_ns(&mut self) -> Option<u64> {
        let duration = self.renderer.bridge.with_session(|session| session.last_known_frame_duration_ns());
        (duration != 0).then_some(duration)
    }
    fn wait_gpu_queue_trim(&mut self, wait: Duration) {
        let timeout = wait.as_nanos().min(u128::from(u64::MAX)) as u64;
        if let Err(error) = self.renderer.bridge.with_session(|session| session.wait_gpu_queue_trim(timeout)) {
            self.renderer.record_error(error);
        }
    }
}

#[cfg(target_os = "macos")]
use macos::PlatformSurface;

fn platform_extensions() -> Vec<std::ffi::CString> {
    #[cfg(target_os = "macos")]
    let names = ["VK_KHR_surface", "VK_EXT_metal_surface"];
    #[cfg(target_os = "windows")]
    let names = ["VK_KHR_surface", "VK_KHR_win32_surface"];
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let names: [&str; 0] = [];
    names.into_iter().map(|name| std::ffi::CString::new(name).expect("static extension")).collect()
}

#[cfg(target_os = "macos")]
mod macos {
    use super::*;
    use objc::{Encode, Encoding, Message, rc::{StrongPtr, autoreleasepool}, runtime::{Class, Object, Sel, YES}};
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    #[link(name = "QuartzCore", kind = "framework")]
    unsafe extern "C" {}
    #[repr(C)] #[derive(Clone, Copy)]
    struct Size { width: f64, height: f64 }
    unsafe impl Encode for Size { fn encode() -> Encoding { unsafe { Encoding::from_str("{CGSize=dd}") } } }
    #[repr(C)] #[derive(Clone, Copy)]
    struct Point { x: f64, y: f64 }
    #[repr(C)] #[derive(Clone, Copy)]
    struct Rect { origin: Point, size: Size }
    unsafe impl Encode for Rect { fn encode() -> Encoding { unsafe { Encoding::from_str("{CGRect={CGPoint=dd}{CGSize=dd}}") } } }
    pub struct PlatformSurface { layer: StrongPtr, view: *mut Object }
    impl PlatformSurface {
        pub fn new(window: &winit::window::Window) -> Result<Self> {
            let view = match window.window_handle()?.as_raw() {
                RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr().cast::<Object>(),
                _ => anyhow::bail!("winit did not return an AppKit view"),
            };
            let layer = autoreleasepool(|| -> Result<StrongPtr> {
                let class = Class::get("CAMetalLayer").context("QuartzCore CAMetalLayer is unavailable")?;
                let pointer: *mut Object = unsafe { class.send_message(Sel::register("new"), ()) }?;
                if pointer.is_null() { anyhow::bail!("CAMetalLayer allocation failed"); }
                let layer = unsafe { StrongPtr::new(pointer) };
                unsafe {
                    let (): () = (&*view).send_message(Sel::register("setLayer:"), (pointer,))?;
                    let (): () = (&*view).send_message(Sel::register("setWantsLayer:"), (YES,))?;
                }
                Ok(layer)
            })?;
            let surface = Self { layer, view };
            surface.update(window)?;
            Ok(surface)
        }
        pub fn update(&self, window: &winit::window::Window) -> Result<()> {
            let size = window.inner_size();
            unsafe {
                let bounds: Rect = (&*self.view).send_message(Sel::register("bounds"), ())?;
                let (): () = (&**self.layer).send_message(Sel::register("setBounds:"), (bounds,))?;
                let (): () = (&**self.layer).send_message(Sel::register("setContentsScale:"), (window.scale_factor(),))?;
                let (): () = (&**self.layer).send_message(Sel::register("setDrawableSize:"),
                    (Size { width: f64::from(size.width), height: f64::from(size.height) },))?;
            }
            Ok(())
        }
        pub fn create_surface(&self, instance: Arc<rhapsody::raw::instance::VulkanInstance>)
            -> Result<Arc<rhapsody::raw::swapchain::Surface>> {
            let loader = ash::ext::metal_surface::Instance::new(&instance.entry, &instance.instance);
            let info = vk::MetalSurfaceCreateInfoEXT::default().layer((*self.layer).cast());
            let handle = unsafe { loader.create_metal_surface(&info, None) }?;
            Ok(Arc::new(unsafe { rhapsody::raw::swapchain::Surface::from_raw(instance, handle, true) }))
        }
    }
}

#[cfg(not(target_os = "macos"))]
struct PlatformSurface;
#[cfg(not(target_os = "macos"))]
impl PlatformSurface {
    fn new(_: &winit::window::Window) -> Result<Self> { anyhow::bail!("native surface adapter is only available for macOS in this host build") }
    fn update(&self, _: &winit::window::Window) -> Result<()> { anyhow::bail!("native surface adapter is unavailable") }
    fn create_surface(&self, _: Arc<rhapsody::raw::instance::VulkanInstance>) -> Result<Arc<rhapsody::raw::swapchain::Surface>> {
        anyhow::bail!("native surface adapter is unavailable")
    }
}
