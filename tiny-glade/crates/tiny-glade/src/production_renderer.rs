//! Production owner for the recovered outer render loop (B 0x1404e8010).
//!
//! Declaration and physical compilation precede command execution; acquisition
//! follows the main graph; the display submission waits for both acquire and
//! actual main-graph timeline receipts. Native shader consumers are shared with
//! core. CAMetalLayer and the external swapchain attachment are host adapters.
use std::{
    collections::{BTreeMap, VecDeque},
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use ash::vk;
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::shader_library::AssetShaderLibrary,
    game_config::GameConfig,
    render::{pass_builder::rg_resources::RgResources as _, render_config::VideoSettings},
    systems::render_loop::{
        draw_passes::draw_lists::{DrawLists, build_draw_lists},
        render_frame::{Acquire, Level, RenderHost, SwapchainInfo},
        render_world as core_render_world,
        world_frame::{self as core_world_frame, MinimalWorldFrameHost, WorldFrameError},
    },
};
use rhapsody::{
    interop::InstanceBuilder, present::AcquiredImageLease, present::PresentTicket,
    present::Swapchain, rg::SessionFrameReceipt,
};

use crate::{
    egui_backend::EguiBackend,
    egui_gpu::EguiGpu,
    game_loop::GameRenderHostFactory,
    render_bridge::{CompiledCoreGraph, CoreGraphFrame, CoreRenderBridge},
};

#[path = "production_resolve.rs"]
mod resolve;

const TIMEOUT_NS: u64 = 10_000_000_000;

/// 管线预热的等待上限（上限而非等待目标：编译完成即返回）。软件光栅化设备（lavapipe）
/// 编译整套世界管线要数十秒，原先的 10 秒会让首帧前预热超时。
pub(crate) const PIPELINE_PREWARM_TIMEOUT: Duration = Duration::from_secs(180);

/// Original loader closure 0x1400d7440 must successfully render the gameplay
/// App once without a swapchain before publishing it.
///
/// Wired to `render_bridge::warm_up_gameplay_with_world_builder` with the
/// recovered W1..W54 world stage plus the compile_main tail subset
/// (`grading -> blur -> final_post -> UI subset`) and the `"msaa color out"`
/// 0x22 export. **Equivalence status**: not complete compile_main. The tail
/// steps whose hosts/producers are still U (menu background, painterly aux
/// inputs and session/feedback overlays)
/// are listed in `world_postprocess::WorldTailControls`; when their controls are
/// off this builder declares the implemented subset only. Worker warm-up has
/// no egui backend; normal main frames draw their owned backend before export.
/// LUT gallery consumes the optional capture slot; its producer remains open.
/// The MEI producer is mandatory: without the original `text_flush_system` output the tail returns
/// a named error instead of presenting a frame without UI text. This never
/// reports success unless the graph compiled, executed and exported a real
/// texture.
/// rhapsody 的预算校验要求非零；原版没有设备内存预算，取 2^48 字节表示不限。
pub const UNBOUNDED_DEVICE_BUDGET_BYTES: u64 = 1 << 48;

pub fn warm_up_gameplay(
    owners: crate::render_bridge::CoreRenderOwners,
    world: &mut World,
) -> Result<()> {
    crate::render_bridge::warm_up_gameplay_with_world_builder(
        owners,
        world,
        |frame, world, lists| {
            // Update44 already consumed the text stream. Reuse that exact
            // output in warm-up instead of destructively flushing twice.
            frame.with_main_prelude(world, |frame, world| {
                let extent = world_extent(world, None, None)?;
                let id = build_full_world_stage(frame, world, lists, extent, None)?;
                let display = display_extent(world)?;
                Ok(
                    country_core::render::pass_builder::pass_resource_access::RgTexture {
                        resource: id.texture,
                        // Native final target: `"msaa color out"` = 0x2b. The
                        // native sample_count(4) lives on the graph image and on
                        // the attachment descriptions, not on this core view.
                        desc: country_core::rhapsody_types::TextureDesc::new_2d(display, 0x2b)
                            .sampled()
                            .color_attachment(),
                    },
                )
            })
        },
    )?;
    Ok(())
}

/// One headless steady-state gameplay frame: the same declaration -> compile -> execute -> export
/// sequence as `ProductionRenderHost` (`build_draw_lists`, `compile_full_world_graph`,
/// `execute_main_graph`) without a swapchain, presentation graph or egui backend.
/// Used by multi-frame validation after `accept_gameplay_warmup`; the caller owns the App update
/// and `RenderTemporalState::frame_render_returned` exactly as `GameFrameDriver` does.
pub struct HeadlessGameplayFrame {
    pub receipt: rhapsody::rg::SessionFrameReceipt,
    pub texture: std::sync::Arc<rhapsody::resource::Texture>,
    /// Draw-list construction plus `refresh_draw_vaos`.
    pub draw_lists: Duration,
    /// Graph declaration (`build_frame`), i.e. the world stage builders.
    pub declare: Duration,
    /// Pipeline prewarm wait plus `compile_built`.
    pub compile: Duration,
    /// `execute_compiled` (recording + submit).
    pub execute: Duration,
}

pub fn render_headless_gameplay_frame(
    bridge: &mut crate::render_bridge::CoreRenderBridge,
    world: &mut World,
) -> Result<HeadlessGameplayFrame> {
    let started = std::time::Instant::now();
    let mut lists = DrawLists::new();
    build_draw_lists(world, bridge.shader_library().as_ref(), &mut lists);
    lists.sort();
    bridge.raster_resources().refresh_draw_vaos(world, &lists).context("refresh draw VAOs")?;
    let draw_lists = started.elapsed();
    let extent = world_extent(world, None, None)?;
    let started = std::time::Instant::now();
    let (output, built) = bridge.build_frame(|frame| {
        frame.with_main_prelude(world, |frame, world| {
            build_full_world_stage(frame, world, &lists, extent, None)
        })
    }).context("declare world graph")?;
    let declare = started.elapsed();
    let started = std::time::Instant::now();
    if let Err(error) = bridge.shader_backend().prewarm(PIPELINE_PREWARM_TIMEOUT) {
        if let Err(cleanup) = bridge.cancel_built(built) {
            log::warn!("headless frame prewarm cancellation: {cleanup}");
        }
        return Err(error.into());
    }
    let capture_graph_id = built.graph_id();
    let compiled = bridge.compile_built(built).context("compile world graph")?;
    let compile = started.elapsed();
    let started = std::time::Instant::now();
    let receipt = match bridge.execute_compiled(compiled) {
        Ok(receipt) => receipt,
        Err(error) => {
            crate::screenshot::shared_texture_capture::cancel_unsubmitted(world, capture_graph_id);
            return Err(anyhow::Error::from(error).context("execute world graph"));
        }
    };
    let execute = started.elapsed();
    if crate::screenshot::shared_texture_capture::installed(world) {
        crate::screenshot::shared_texture_capture::submitted(
            world, bridge, capture_graph_id, &receipt,
        )?;
    }
    let texture = bridge.take_exported_texture(&receipt, output.texture).context("take exported texture")?;
    Ok(HeadlessGameplayFrame { receipt, texture, draw_lists, declare, compile, execute })
}

// ---------------------------------------------------------------------------------------------
// Original compile_main 0x140161cd0 wiring (G2 world-frame round).
//
// O: the closure creates `main color` (TextureDesc::new_2d(extent, 0x7a) then
// 0x140bd5460 `mov [desc+0x1c], 6` = mip_levels(6), name 0x14295d6a8 len 10)
// and `main depth` (new_2d(extent, 0x82), name 0x14295d798 len 10), calls
// render_world 0x1404ec110, then exports the final texture for the
// presentation closure (import_unowned access 0x22). The typed W1..W54
// consumers recovered in world_graph.rs are the production body; the 63-seam
// `FramePasses` trait has no production host yet and is not used here.
// ---------------------------------------------------------------------------------------------

/// `production_resolve` imports the main source at access 0x22.
const WORLD_EXPORT_ACCESS: u8 = 0x22;

/// Explicit diagnostic switch for the minimal (clear+export) world frame.
/// Without it, a missing world producer is a hard error:
/// presenting an empty world as a successful frame would hide the real gap.
pub const ALLOW_MINIMAL_WORLD_FRAME_ENV: &str = "TG_ALLOW_MINIMAL_WORLD_FRAME";

fn minimal_world_frame_allowed() -> bool {
    std::env::var(ALLOW_MINIMAL_WORLD_FRAME_ENV).as_deref() == Ok("1")
}

/// Original world dimensions.
///
/// Contract §5.1 M1 (I): `dims = min_extent.unwrap_or(world_dims)` — the OOM
/// retry value is an **override**, not a floor. `world_dims` is the inner
/// rendering resolution: base window extent times `world_resolution_scale`.
/// `prepare_frame_constants` treats `extent_10` as `inner_rendering_resolution`
/// and `global_mip_bias = extent_10/extent_8` (prepare_frame_constants.rs:205,
/// 214); `FrameRenderConfig::at_startup` copies the window dims without applying
/// the scale (render_config.rs:49-54), so the scale is consumed here. Reading
/// `GameConfig.video.world_resolution_scale` is what makes
/// `set_resolution_scale` (production_renderer.rs) actually shrink the OOM
/// retry.
fn world_extent(
    world: &World,
    swapchain: Option<&SwapchainInfo>,
    min_extent: Option<[u32; 2]>,
) -> rhapsody::Result<[u32; 2]> {
    if let Some(min) = min_extent {
        if min.contains(&0) {
            return Err(rhapsody::Error::invalid_usage(
                "original compile_main 0x140161cd0: min_extent contains zero",
            ));
        }
        return Ok(min);
    }
    let config = world.get_resource::<country_core::render::render_config::FrameRenderConfig>();
    let base = world
        .get_resource::<country_core::resources::window::WindowSize>()
        .map(|window| window.as_dims())
        .or_else(|| config.map(|config| config.extent_10))
        .or_else(|| swapchain.map(|info| info.extent))
        .ok_or_else(|| {
            rhapsody::Error::invalid_usage(
                "original compile_main 0x140161cd0: missing WindowSize/FrameRenderConfig producer and no swapchain extent",
            )
        })?;
    // I (evidence item): the native inner-resolution update that applies
    // world_resolution_scale to extent_10 is not recovered. Apply it at the
    // point of use so the OOM retry is not a no-op.
    let scale = world
        .get_resource::<GameConfig>()
        .and_then(|config| config.video.world_resolution_scale)
        .or_else(|| config.map(|config| config.world_resolution_scale))
        .unwrap_or(1.0);
    let extent = [
        (base[0] as f32 * scale) as u32,
        (base[1] as f32 * scale) as u32,
    ];
    if extent.contains(&0) {
        return Err(rhapsody::Error::invalid_usage(
            "original compile_main 0x140161cd0: world extent is zero after world_resolution_scale",
        ));
    }
    Ok(extent)
}

/// Inputs read by the unconditional main prelude. World-only inputs are read
/// by their actual selected consumer after `select_main_world`, so menu reuse
/// and hidden-world frames do not require a MainCamera or ActiveSession.
fn missing_main_producer(world: &World) -> Option<&'static str> {
    use country_core::{
        render::{frame_constants::FrameConstants, nani::gpu_world::GpuWorld},
        resources::gpu_lut::GpuLutTracker,
    };
    let checks: [(&'static str, bool); 4] = [
        ("GpuWorld", world.contains_resource::<GpuWorld>()),
        ("GpuLutTracker", world.contains_resource::<GpuLutTracker>()),
        (
            "FrameConstants",
            world.contains_resource::<FrameConstants>(),
        ),
        (
            "FrameRenderConfig",
            world.contains_resource::<country_core::render::render_config::FrameRenderConfig>(),
        ),
    ];
    checks
        .iter()
        .find(|(_, present)| !present)
        .map(|(name, _)| *name)
}

fn world_color_target(
    frame: &mut CoreGraphFrame<'_>, extent: [u32; 2],
) -> rhapsody::Result<country_core::render::pass_builder::pass_resource_access::RgTexture> {
    frame.with_compute(|compute| Ok(compute.create_raw_texture(
        core_world_frame::MAIN_COLOR_NAME, &core_world_frame::main_color_desc(extent))))
}

fn world_depth_target(
    frame: &mut CoreGraphFrame<'_>, extent: [u32; 2],
) -> rhapsody::Result<country_core::render::pass_builder::pass_resource_access::RgTexture> {
    frame.with_compute(|compute| Ok(compute.create_raw_texture(
        core_world_frame::MAIN_DEPTH_NAME, &core_world_frame::main_depth_desc(extent))))
}

/// Native `plVar39[6]` target extent (I: `FrameRenderConfig.extent_8`, the same
/// display extent used by the TGSR history). Used by `final_post`'s `r9` and by
/// the UI passes' viewport. Zero is a hard error, never a fallback size.
fn display_extent(world: &World) -> rhapsody::Result<[u32; 2]> {
    let extent = world
        .get_resource::<country_core::render::render_config::FrameRenderConfig>()
        .ok_or_else(|| {
            rhapsody::Error::invalid_usage(
                "original compile_main 0x140161cd0: missing FrameRenderConfig for the tail display extent",
            )
        })?
        .extent_8;
    if extent.contains(&0) {
        return Err(rhapsody::Error::invalid_usage(
            "original compile_main 0x140161cd0: display extent contains zero",
        ));
    }
    Ok(extent)
}

struct BuiltMainStage {
    texture: u32,
    kind: ProductionGraphKind,
}

fn build_full_world_stage(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    lists: &DrawLists,
    extent: [u32; 2],
    egui: Option<(&mut EguiGpu, &mut EguiBackend)>,
) -> rhapsody::Result<BuiltMainStage> {
    let color = world_color_target(frame, extent)?;
    let decision = crate::world_graph::select_main_world(world)?;
    let controls = crate::world_graph::WorldGraphControls {
        // O: *0x14362c0d8 is a process debug byte initialised to zero; no
        // recovered producer exists, so the load-time value is used.
        freeze_occlusion_culling: false,
        postprocess: crate::world_postprocess::PostProcessControls {
            // O: *0x14362c080 (calc_dof multires) and *0x143634d50 (history
            // disable) are load-time zero debug globals without producers.
            multires_dof: false,
            disable_temporal_history: false,
        },
    };
    let debug_rt = crate::gui::gui_debug::RENDER_OVERRIDE_SHOW_DEBUG_RT_VIEW
        .load(std::sync::atomic::Ordering::Relaxed);
    let out = if decision.render_world && !debug_rt {
        let depth = world_depth_target(frame, extent)?;
        crate::world_graph::build_world(frame, world, lists, extent, depth, color, controls)?
    } else {
        if decision.render_world {
            // Original 140162840 selects the recovered Primary RT algorithm,
            // not a clear operation and not an empty stand-in world.
            crate::scene_compute::with_env(frame, world, |env| {
                let pass = country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug(env, &color);
                env.record(pass);
                Ok(())
            })?;
        }
        // Both original no-world branches use these real independent dummies.
        let depth = world_depth_target(frame, extent)?;
        let coc = frame.with_compute(|compute| Ok(compute.create_raw_texture("coc",
            &country_core::rhapsody_types::TextureDesc::new_2d([1, 1], 9))))?;
        crate::world_postprocess::WorldPostOutputs { depth, coc, taa: color }
    };
    // Native compile_main 0x140161cd0 continues after render_world with
    // grading -> blur -> final_post -> UI, then exports the MSAA target at 0x22.
    let display = display_extent(world)?;
    let tail_controls = crate::world_postprocess::resolve_world_tail_controls(world, decision)?;
    let tail = crate::world_postprocess::build_world_tail(
        frame,
        world,
        lists,
        &out,
        display,
        &tail_controls,
    )?;
    // T21 (B compile_main :1363): the main-frame owner supplies Some, while
    // the loading frame and the worker warm-up have no EguiBackend. Keep this
    // after T20 loading_screen_pass and before the final 0x22 export; all draws
    // target the same 4x MSAA image and the existing per-App GPU namespaces.
    {
        if let Some((gpu, backend)) = egui {
            let shaders = frame.shader_library();
            let target =
                country_core::systems::render_loop::draw_passes::final_post::msaa_color_desc(
                    (display[0], display[1]),
                    tail.msaa.resource,
                );
            gpu.draw(frame, backend, shaders.as_ref(), target)
                .map_err(|error| {
                    rhapsody::Error::invalid_usage(format!(
                        "original compile_main tail EguiBackend::draw: {error:#}"
                    ))
                })?;
        }
    }
    crate::screenshot::shared_texture_capture::opportunity(frame, world, 6, tail.msaa, 4)?;
    frame.with_graph(|graph| {
        let id = graph.resource_handle(tail.msaa.resource)?;
        graph.export(id, WORLD_EXPORT_ACCESS)?;
        Ok(())
    })?;
    Ok(BuiltMainStage {
        texture: tail.msaa.resource,
        kind: ProductionGraphKind::Main {
            world_rendered: decision.render_world && !debug_rt,
            primary_rt: decision.render_world && debug_rt,
        },
    })
}

fn compile_full_world_graph(
    renderer: &mut ProductionRenderer,
    world: &mut World,
    lists: &DrawLists,
    extent: [u32; 2],
    egui: &mut EguiBackend,
) -> rhapsody::Result<CompiledGameGraph> {
    // Original `text_flush_system` runs in Update44. Compile and OOM retry
    // reuse that uploaded stream without consuming the text renderer again.
    let gpu = &mut renderer.egui;
    let (output, built) = renderer.bridge.build_frame(|frame| {
        frame.with_main_prelude(world, |frame, world| {
            build_full_world_stage(frame, world, lists, extent, Some((gpu, egui)))
        })
    })?;
    if let Err(error) = renderer
        .bridge
        .shader_backend()
        .prewarm(PIPELINE_PREWARM_TIMEOUT)
    {
        if let Err(cleanup) = renderer.bridge.cancel_built(built) {
            log::warn!("world pipeline prewarm cancellation: {cleanup}");
        }
        return Err(error);
    }
    let capture_graph_id = built.graph_id();
    let graph = renderer.bridge.compile_built(built)?;
    Ok(CompiledGameGraph {
        capture_graph_id,
        graph,
        output: output.texture,
        kind: output.kind,
        expected: ExportExpectation {
            format: country_core::systems::render_loop::draw_passes::final_post::MSAA_COLOR_FORMAT,
            sample_count:
                country_core::systems::render_loop::draw_passes::final_post::MSAA_SAMPLE_COUNT,
        },
    })
}

/// Minimal world-frame host over the real enclosing graph. `Clear image`
/// (0x1409dde90) is a real transfer pass; no shader or substitute renderer is
/// installed.
struct MinimalWorldHost<'a, 'b> {
    frame: &'a mut CoreGraphFrame<'b>,
}

fn backend_error(seam: &'static str) -> impl FnOnce(rhapsody::Error) -> WorldFrameError {
    move |error| WorldFrameError::backend(seam, error)
}

impl MinimalWorldFrameHost for MinimalWorldHost<'_, '_> {
    fn create_color_target(
        &mut self,
        name: &str,
        desc: &country_core::rhapsody_types::TextureDesc,
    ) -> Result<core_render_world::RgTexture, WorldFrameError> {
        // The graph creation uses the full country-core description, including
        // the native mip_levels(6); the returned render_world view only carries
        // id/format/extent because downstream consumers use `id`.
        let texture = self
            .frame
            .with_compute(|compute| Ok(compute.create_raw_texture(name, desc)))
            .map_err(backend_error("create_color_target"))?;
        Ok(core_render_world::RgTexture {
            id: texture.resource,
            desc: core_render_world::TextureDesc::new_2d(
                [desc.extent[0], desc.extent[1]],
                desc.format,
            ),
        })
    }

    fn clear_color_target(
        &mut self,
        color: &core_render_world::RgTexture,
    ) -> Result<(), WorldFrameError> {
        self.frame
            .with_compute(|compute| {
                country_core::render::rg_helpers::clear_rg_color_image(compute, color.id);
                Ok(())
            })
            .map_err(backend_error("clear_color_target"))
    }

    fn export_texture(
        &mut self,
        color: &core_render_world::RgTexture,
        access: u8,
    ) -> Result<(), WorldFrameError> {
        self.frame
            .with_graph(|graph| {
                let id = graph.resource_handle(color.id)?;
                graph.export(id, access)?;
                Ok(())
            })
            .map_err(backend_error("export_texture"))
    }
}

fn compile_minimal_world_graph(
    renderer: &mut ProductionRenderer,
    extent: [u32; 2],
) -> rhapsody::Result<CompiledGameGraph> {
    let (output, built) = renderer.bridge.build_frame(|frame| {
        let mut host = MinimalWorldHost { frame };
        core_world_frame::build_minimal_world_frame(&mut host, extent)
            .map_err(|error| rhapsody::Error::invalid_usage(error.to_string()))
    })?;
    if let Err(error) = renderer
        .bridge
        .shader_backend()
        .prewarm(PIPELINE_PREWARM_TIMEOUT)
    {
        if let Err(cleanup) = renderer.bridge.cancel_built(built) {
            log::warn!("minimal world frame prewarm cancellation: {cleanup}");
        }
        return Err(error);
    }
    let capture_graph_id = built.graph_id();
    let graph = renderer.bridge.compile_built(built)?;
    Ok(CompiledGameGraph {
        capture_graph_id,
        graph,
        output: output.id,
        kind: ProductionGraphKind::DiagnosticMinimal,
        expected: ExportExpectation {
            format: core_render_world::formats::COLOR_HDR,
            sample_count: 1,
        },
    })
}

struct ApplicationGpu {
    bridge: CoreRenderBridge,
    egui: EguiGpu,
    shaders: Arc<AssetShaderLibrary>,
    final_post_shader: Option<
        country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,
    >,
}
impl ApplicationGpu {
    fn new(
        device: Arc<rhapsody::device::Device>,
        shader_root: &Path,
        shaders: Arc<AssetShaderLibrary>,
    ) -> Result<Self> {
        // Original CompiledFilePath already contains "compiled-shaders/".
        // This boundary accepts that directory explicitly; pass its owning
        // installation root to the file backend once, for either App.
        let asset_root = shader_root
            .parent()
            .context("compiled shader directory has no installation root")?;
        let bridge = CoreRenderBridge::new(device.clone(), asset_root, shaders.clone())?;
        let mut final_post_shader = None;
        country_core::startup::load_shaders::load_shaders(
            &shaders,
            bridge.shader_backend(),
            &mut final_post_shader,
        );
        country_core::startup::load_shaders::load_ui_shaders(&shaders, bridge.shader_backend());
        country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());
        struct VermeerShaders<'a> {
            library: &'a AssetShaderLibrary,
            backend: &'a dyn country_core::render::shader::boundary::ShaderBackend,
        }
        impl country_core::systems::render_loop::painterly::ShaderLoader for VermeerShaders<'_> {
            fn load_shader_into_library(&mut self, path: &'static str, name: &'static str) {
                country_core::startup::load_shaders::load_shader_into_library(
                    self.library,
                    self.backend,
                    path,
                    name,
                );
            }
        }
        country_core::systems::render_loop::painterly::startup_painterly(&mut VermeerShaders {
            library: &shaders,
            backend: bridge.shader_backend(),
        });
        country_core::systems::render_loop::vermeer::startup_vermeer(&mut VermeerShaders {
            library: &shaders,
            backend: bridge.shader_backend(),
        });
        let egui = EguiGpu::new(device, bridge.hosts().clone());
        Ok(Self {
            bridge,
            egui,
            shaders,
            final_post_shader,
        })
    }
}

pub struct ProductionRenderer {
    bridge: CoreRenderBridge,
    egui: EguiGpu,
    swapchain: Arc<Swapchain>,
    device: Arc<rhapsody::device::Device>,
    shaders: Arc<AssetShaderLibrary>,
    final_post_shader: Option<
        country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,
    >,
    other_applications: BTreeMap<usize, ApplicationGpu>,
    compiled_shader_root: PathBuf,
    pending: VecDeque<PresentTicket>,
    acquired: Option<AcquiredImageLease>,
    /// The swapchain image imported into the executed presentation graph.
    presentation: Option<rhapsody::rg::PresentTarget>,
    main: Option<(u32, SessionFrameReceipt)>,
    main_output: Option<Arc<rhapsody::resource::Texture>>,
    started: Instant,
    scopes: Vec<usize>,
    first_error: Option<rhapsody::Error>,
    frame_submitted: bool,
    frame_present_queued: bool,
    frame_observation: Option<ProductionFrameObservation>,
    suspended: bool,
    closed: bool,
    reduced_state: u64,
    stats: rhapsody::device::RenderStats,
    last_stats: rhapsody::device::StatsSnapshot,
    // The native window and platform backing outlive every Vulkan surface owner.
    platform: PlatformSurface,
    window: Arc<winit::window::Window>,
}

impl ProductionRenderer {
    pub fn new(
        window: Arc<winit::window::Window>,
        compiled_shader_root: impl AsRef<Path>,
        shaders: Arc<AssetShaderLibrary>,
    ) -> Result<Self> {
        let builder = InstanceBuilder {
            application_name: std::ffi::CString::new("Tiny Glade")?,
            extensions: platform_extensions(),
            ..Default::default()
        };
        let instance = builder.build().context("create Vulkan instance")?;
        let platform = PlatformSurface::new(&window)?;
        let surface = platform.create_surface(instance.clone())?;
        let device = rhapsody::device::Device::new(
            instance,
            Some(&surface),
            &rhapsody::device::DeviceConfig {
                // 原版 GpuAllocator::new (0x140c0d980) 没有预算参数，allocate (0x140c0da40)
                // 没有总量上限分支；真实上限只有驱动堆与 VK_EXT_memory_budget。rhapsody 的
                // 1 GiB 默认值是它自有策略，装不下全量纹理加 Nani/地形缓冲，这里取“不限”。
                memory_budget_bytes: UNBOUNDED_DEVICE_BUDGET_BYTES,
                queues: rhapsody::device::QueueConfig {
                    require_present: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .context("create graphics device for native surface")?;
        let size = window.inner_size();
        let swapchain = Arc::new(unsafe {
            Swapchain::new(
                device.clone(),
                surface,
                vk::Extent2D {
                    width: size.width,
                    height: size.height,
                },
                true,
                vk::ImageUsageFlags::COLOR_ATTACHMENT,
            )
        }?)
        .clone();
        let compiled_shader_root = compiled_shader_root.as_ref().to_path_buf();
        let ApplicationGpu {
            bridge,
            egui,
            shaders,
            final_post_shader,
        } = ApplicationGpu::new(device.clone(), &compiled_shader_root, shaders)?;
        let properties = unsafe {
            device
                .instance()
                .raw_instance()
                .get_physical_device_properties(device.physical_device())
        };
        log::info!(
            "Graphics device: {}",
            unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy()
        );
        Ok(Self {
            bridge,
            egui,
            swapchain,
            device,
            shaders,
            final_post_shader,
            other_applications: BTreeMap::new(),
            compiled_shader_root,
            pending: VecDeque::new(),
            acquired: None,
            presentation: None,
            main: None,
            main_output: None,
            started: Instant::now(),
            scopes: Vec::new(),
            first_error: None,
            frame_submitted: false,
            frame_present_queued: false,
            frame_observation: None,
            suspended: false,
            closed: false,
            reduced_state: 0,
            stats: Default::default(),
            last_stats: Default::default(),
            platform,
            window,
        })
    }

    pub fn device(&self) -> &Arc<rhapsody::device::Device> {
        &self.device
    }
    pub fn bridge(&self) -> &CoreRenderBridge {
        &self.bridge
    }
    pub fn bridge_mut(&mut self) -> &mut CoreRenderBridge {
        &mut self.bridge
    }
    /// B creates independent render libraries for loading and gameplay Apps,
    /// cloning only the graphics device. Preserve each App's real library and
    /// dynamic-constant owner rather than aliasing their same-numbered handles.
    pub fn activate_application(&mut self, world: &World) -> Result<()> {
        let shaders = world
            .get_resource::<crate::run_game::ApplicationShaderLibrary>()
            .context("App has no original shader-library owner")?
            .0
            .clone();
        if Arc::ptr_eq(&self.shaders, &shaders) {
            if let Some(completed) =
                world.get_resource::<crate::render_bridge::CompletedGameplayWarmup>()
            {
                self.bridge.accept_gameplay_warmup(completed)?;
            }
            return Ok(());
        }
        anyhow::ensure!(
            self.acquired.is_none() && self.presentation.is_none(),
            "cannot switch App while its WSI acquisition is live"
        );
        let key = Arc::as_ptr(&shaders) as usize;
        let previous = Arc::as_ptr(&self.shaders) as usize;
        let mut next = match self.other_applications.remove(&key) {
            Some(next) => next,
            None => {
                anyhow::ensure!(!world.contains_resource::<country_core::asset_libraries::runtime::AssetLibraryRuntime>(),
                    "App already owns GPU libraries but its original render bridge is missing");
                ApplicationGpu::new(self.device.clone(), &self.compiled_shader_root, shaders)?
            }
        };
        std::mem::swap(&mut self.bridge, &mut next.bridge);
        std::mem::swap(&mut self.egui, &mut next.egui);
        std::mem::swap(&mut self.shaders, &mut next.shaders);
        std::mem::swap(&mut self.final_post_shader, &mut next.final_post_shader);
        self.other_applications.insert(previous, next);
        if let Some(completed) =
            world.get_resource::<crate::render_bridge::CompletedGameplayWarmup>()
        {
            self.bridge.accept_gameplay_warmup(completed)?;
        }
        self.main_output.take();
        self.main.take();
        Ok(())
    }
    pub fn final_post_shader(
        &self,
    ) -> Option<
        country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,
    > {
        self.final_post_shader
    }
    pub fn any_present_successful(&self) -> bool {
        self.swapchain.any_present_successful()
    }
    pub fn frame_observation(&self) -> Option<&ProductionFrameObservation> {
        self.frame_observation.as_ref()
    }
    /// `None` means no current queued ticket is retained. Queue acceptance and
    /// GPU completion are never substituted for presentation completion.
    pub fn frame_present_complete(&self) -> rhapsody::Result<Option<bool>> {
        let Some(frame) = self.frame_observation.as_ref() else { return Ok(None); };
        let Some(point) = frame.present_point else { return Ok(None); };
        for ticket in &self.pending {
            if ticket.point == point && Some(ticket.generation()) == frame.present_generation {
                return ticket.is_present_complete().map(Some);
            }
        }
        Ok(None)
    }
    pub fn take_error(&mut self) -> Option<anyhow::Error> {
        self.first_error.take().map(anyhow::Error::from)
    }
    pub fn resize(&mut self, extent: [u32; 2]) -> Result<()> {
        self.platform.update(&self.window)?;
        self.swapchain.resize(vk::Extent2D {
            width: extent[0],
            height: extent[1],
        })?;
        Ok(())
    }
    pub fn set_vsync(&mut self, enabled: bool) -> Result<()> {
        self.swapchain.set_vsync(enabled)?;
        Ok(())
    }
    pub fn set_focused(&mut self, focused: bool) -> Result<()> {
        self.swapchain.set_focused(focused)?;
        Ok(())
    }
    pub fn suspend(&mut self) -> Result<()> {
        self.suspended = true;
        if let Some(image) = self.acquired.take() {
            image.cancel(TIMEOUT_NS)?;
        }
        Ok(())
    }
    pub fn resume(&mut self) -> Result<()> {
        self.platform.update(&self.window)?;
        self.suspended = false;
        self.swapchain.mark_stale();
        Ok(())
    }
    fn record_error(&mut self, error: rhapsody::Error) {
        if self.first_error.is_none() {
            self.first_error = Some(error);
        }
    }
    fn collect(&mut self) -> rhapsody::Result<()> {
        self.bridge.collect_completed_frames()?;
        for app in self.other_applications.values_mut() {
            app.bridge.collect_completed_frames()?;
        }
        while let Some(ticket) = self.pending.front() {
            if !ticket.is_complete()? {
                break;
            }
            self.pending.pop_front();
        }
        self.swapchain.collect_presentations()?;
        self.device.try_collect_garbage()?;
        Ok(())
    }
    pub fn shutdown(&mut self) -> Result<()> {
        if self.closed {
            return Ok(());
        }
        self.presentation.take();
        self.bridge.cancel_presentation()?;
        if let Some(image) = self.acquired.take() {
            image.cancel(TIMEOUT_NS)?;
        }
        self.bridge.wait_submits_done(TIMEOUT_NS)?;
        for app in self.other_applications.values_mut() {
            app.bridge.wait_submits_done(TIMEOUT_NS)?;
        }
        for ticket in &self.pending {
            ticket.wait(TIMEOUT_NS)?;
        }
        self.pending.clear();
        self.swapchain.shutdown(TIMEOUT_NS)?;
        self.bridge.raster_resources().clear_attachment_cache();
        self.bridge
            .with_session(|session| session.clear_after_completion())?;
        for app in self.other_applications.values_mut() {
            app.bridge.raster_resources().clear_attachment_cache();
            app.bridge
                .with_session(|session| session.clear_after_completion())?;
        }
        self.device.wait_idle()?;
        let peak = self
            .device
            .with_global_allocator(|allocator| allocator.peak_totals());
        if peak.device_local_bytes != 0 && peak.system_bytes != 0 {
            log::info!(
                "Peak GPU memory use (device local): {} system: {}",
                peak.device_local_bytes,
                peak.system_bytes
            );
        }
        self.closed = true;
        Ok(())
    }
}

impl Drop for ProductionRenderer {
    fn drop(&mut self) {
        if !self.closed {
            if let Err(error) = self.shutdown() {
                log::error!("graphics shutdown: {error:#}");
            }
        }
    }
}

/// What the presentation graph must receive. The native resolve shader is chosen
/// from the imported source's `sample_count` (`final_post::resolve_shader_name`),
/// so a wrong export is observable here instead of silently taking the
/// non-MSAA path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ExportExpectation {
    format: u32,
    sample_count: u8,
}

/// Diagnostic facts from this frame's actual main graph. A Main graph may
/// deliberately reuse a static menu capture without running render_world.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionGraphKind {
    Loading,
    Main { world_rendered: bool, primary_rt: bool },
    DiagnosticMinimal,
}

#[derive(Clone, Debug)]
pub struct ProductionFrameObservation {
    pub world: bevy_ecs::world::WorldId,
    pub application_shader_owner: usize,
    pub graph: rhapsody::rg::GraphId,
    pub kind: ProductionGraphKind,
    pub receipt_frame: u64,
    pub submitted: rhapsody::sync::SyncSet,
    pub draw_list_lengths: [usize; 21],
    pub output_format: u32,
    pub output_sample_count: u8,
    pub present_point: Option<rhapsody::sync::SyncPoint>,
    pub present_generation: Option<u64>,
}

pub struct CompiledGameGraph {
    capture_graph_id: rhapsody::rg::GraphId,
    graph: CompiledCoreGraph,
    output: u32,
    kind: ProductionGraphKind,
    expected: ExportExpectation,
}

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
    fn finish_render(&mut self) -> anyhow::Result<()> {
        self.take_error().map_or(Ok(()), Err)
    }
    fn last_frame_submitted(&self) -> bool { self.frame_submitted }
    fn last_frame_present_queued(&self) -> bool { self.frame_present_queued }
    fn host<'a>(&'a mut self, world: &'a mut World, egui: &'a mut EguiBackend) -> Self::Host<'a> {
        self.frame_submitted = false;
        self.frame_present_queued = false;
        self.frame_observation = None;
        if let Err(error) = self.activate_application(world) {
            // Keep the full application-switch diagnostic chain. This is an
            // ownership/setup failure before world compilation, not its OOM
            // retry path; no anyhow context is discarded by downcasting it.
            self.record_error(rhapsody::Error::InvalidUsage {
                what: "B per-App rendering library ownership".into(),
                source: Some(error.into()),
            });
        }
        ProductionRenderHost {
            renderer: self,
            world,
            egui,
            video: None,
            input_lag_reduction: false,
            lists: DrawLists::new(),
        }
    }
}

impl ProductionRenderHost<'_> {
    /// The resolve graph imports the acquired image as its presentation
    /// target; the session owns the acquisition from the import on.
    fn execute_presentation_graph_inner(&mut self) -> rhapsody::Result<()> {
        let source = self.renderer.main_output.clone().ok_or_else(|| {
            rhapsody::Error::invalid_usage("main graph did not publish a source image")
        })?;
        let acquired = self.renderer.acquired.take().ok_or_else(|| {
            rhapsody::Error::invalid_usage("presentation needs an acquired image")
        })?;
        let shaders = self.renderer.shaders.clone();
        let flag = self
            .video
            .as_ref()
            .is_some_and(|video| video.render_upside_down);
        let world = &mut *self.world;
        let (target, graph) = self.renderer.bridge.build_frame(|frame| {
            let target = frame.import_swapchain_image(acquired)?;
            resolve::build(frame, world, &shaders, source, &target, flag)?;
            Ok(target)
        })?;
        if let Err(error) = self
            .renderer
            .bridge
            .shader_backend()
            .prewarm(PIPELINE_PREWARM_TIMEOUT)
        {
            if let Err(cleanup) = self.renderer.bridge.cancel_built(graph) {
                log::warn!(target: crate::render_loop::LOG_TARGET,
                    "presentation prewarm cancellation: {cleanup}");
            }
            return Err(error);
        }
        let compiled = self.renderer.bridge.compile_built(graph)?;
        self.renderer.presentation = Some(target);
        self.renderer.bridge.execute_compiled(compiled)?;
        Ok(())
    }
}

impl RenderHost for ProductionRenderHost<'_> {
    type Compiled = CompiledGameGraph;
    type CompileError = rhapsody::Error;
    fn compile_error_is_out_of_memory(error: &Self::CompileError) -> bool {
        error.kind() == rhapsody::ErrorKind::OutOfMemory
    }
    fn take_render_error(&mut self) -> Option<Self::CompileError> {
        self.renderer.first_error.take()
    }
    fn abort_frame(&mut self) {
        // `take_render_error` already moved the original error out. Cleanup is
        // unconditional and must never replace that error with a second one.
        self.renderer.presentation.take();
        if let Err(error) = self.renderer.bridge.cancel_presentation() {
            log::warn!(target: crate::render_loop::LOG_TARGET, "aborted presentation cleanup: {error}");
        }
        if let Some(image) = self.renderer.acquired.take() {
            if let Err(error) = image.cancel(TIMEOUT_NS) {
                log::warn!(target: crate::render_loop::LOG_TARGET, "aborted acquisition cleanup: {error}");
            }
        }
        self.renderer.main_output.take();
        self.renderer.main.take();
    }
    fn puffin_scopes_on(&mut self) -> bool {
        puffin::are_scopes_on()
    }
    fn scope_begin(&mut self, name: &str) {
        let offset =
            puffin::ThreadProfiler::call(|profiler| profiler.begin_scope(name, file!(), ""));
        self.renderer.scopes.push(offset);
    }
    fn scope_end(&mut self) {
        if let Some(offset) = self.renderer.scopes.pop() {
            puffin::ThreadProfiler::call(|profiler| profiler.end_scope(offset));
        }
    }
    fn now(&mut self) -> Duration {
        self.renderer.started.elapsed()
    }
    fn device_ready(&mut self) -> bool {
        if self.renderer.device.is_device_lost() {
            self.renderer.record_error(rhapsody::Error::DeviceLost);
            return false;
        }
        !self.renderer.closed
    }
    fn clone_video_settings(&mut self) -> bool {
        let Some(config) = self.world.get_resource::<GameConfig>() else {
            return false;
        };
        self.video = Some(config.video.clone());
        self.input_lag_reduction = config.ui.input_lag_reduction;
        true
    }
    fn gpu_pacing_enabled(&mut self) -> bool {
        self.input_lag_reduction
    }
    fn build_draw_lists(&mut self) {
        build_draw_lists(self.world, &*self.renderer.shaders, &mut self.lists);
        self.lists.sort();
        if let Err(error) = self
            .renderer
            .bridge
            .raster_resources()
            .refresh_draw_vaos(self.world, &self.lists)
        {
            self.renderer.record_error(error);
        }
    }
    fn swapchain_info(&mut self) -> Option<SwapchainInfo> {
        let extent = self.renderer.swapchain.desc().extent;
        (!self.renderer.suspended && extent.width > 0 && extent.height > 0).then(|| SwapchainInfo {
            format: self.renderer.swapchain.format().as_raw() as u32,
            extent: [extent.width, extent.height],
        })
    }
    fn compile_loading_graph(
        &mut self,
        swapchain: &SwapchainInfo,
    ) -> rhapsody::Result<Self::Compiled> {
        let shaders = self.renderer.shaders.clone();
        let (output, built) = self.renderer.bridge.build_frame(|frame| {
            crate::loading_graph::build_loading_frame(
                frame,
                self.world,
                &shaders,
                swapchain.extent,
                swapchain.format,
                &self.lists,
            )
        })?;
        if let Err(error) = self
            .renderer
            .bridge
            .shader_backend()
            .prewarm(PIPELINE_PREWARM_TIMEOUT)
        {
            if let Err(cleanup) = self.renderer.bridge.cancel_built(built) {
                log::warn!("pipeline prewarm cancellation: {cleanup}");
            }
            return Err(error);
        }
        let capture_graph_id = built.graph_id();
        let graph = self.renderer.bridge.compile_built(built)?;
        Ok(CompiledGameGraph {
            capture_graph_id,
            graph,
            output: output.id,
            kind: ProductionGraphKind::Loading,
            // loading_graph builds its "loading screen" target as 0x2b with a
            // single sample; presentation takes the non-MSAA resolve shader.
            expected: ExportExpectation {
                format: 0x2b,
                sample_count: 1,
            },
        })
    }
    fn compile_world_graph(
        &mut self,
        swapchain: Option<&SwapchainInfo>,
        min_extent: Option<[u32; 2]>,
    ) -> rhapsody::Result<Self::Compiled> {
        let extent = world_extent(self.world, swapchain, min_extent)?;
        match missing_main_producer(self.world) {
            None => {
                compile_full_world_graph(self.renderer, self.world, &self.lists, extent, self.egui)
            }
            Some(missing) => {
                if minimal_world_frame_allowed() {
                    // Explicit diagnostic path only. The minimal closure
                    // is a real clear+export frame, not a claim that world
                    // rendering succeeded.
                    log::warn!(
                        target: crate::render_loop::LOG_TARGET,
                        "original compile_main 0x140161cd0: missing {missing} producer; diagnostic minimal world frame enabled (clear + export)"
                    );
                    compile_minimal_world_graph(self.renderer, extent)
                } else {
                    // Default: refuse to present an empty world as success.
                    Err(rhapsody::Error::unsupported(format!(
                        "original compile_main 0x140161cd0: missing {missing} producer; refusing the minimal world frame (set {ALLOW_MINIMAL_WORLD_FRAME_ENV}=1 to allow the diagnostic)"
                    )))
                }
            }
        }
    }
    fn resolution_scale(&mut self) -> Option<f32> {
        self.world
            .get_resource::<GameConfig>()
            .map(|config| config.video.world_resolution_scale.unwrap_or(1.0))
    }
    fn set_resolution_scale(&mut self, scale: f32) {
        if let Some(mut config) = self.world.get_resource_mut::<GameConfig>() {
            config.video.world_resolution_scale = Some(scale);
        }
    }
    fn clear_resource_pool(&mut self) {
        if let Err(error) = self
            .renderer
            .bridge
            .with_session(|session| session.clear_idle_resource_pool())
        {
            self.renderer.record_error(error);
        }
    }
    fn reset_temporal_tables(&mut self) {
        if let Err(error) = self
            .renderer
            .bridge
            .with_session(|session| session.reset_all_temporals())
        {
            self.renderer.record_error(error);
        }
    }
    fn wait_idle_and_recycle_resources(&mut self) {
        if let Err(error) = self
            .renderer
            .bridge
            .wait_submits_done(TIMEOUT_NS)
            .and_then(|()| self.renderer.device.wait_idle())
        {
            self.renderer.record_error(error);
        }
    }
    fn set_reduced_state(&mut self, value: u64) {
        self.renderer.reduced_state = value;
    }
    fn log(&mut self, level: Level, message: &str) {
        match level {
            Level::Error => log::error!(target: crate::render_loop::LOG_TARGET, "{message}"),
            Level::Info => log::info!(target: crate::render_loop::LOG_TARGET, "{message}"),
        }
    }
    fn begin_frame(&mut self, gpu_profiling: bool) {
        self.renderer.frame_submitted = false;
        self.renderer.frame_present_queued = false;
        let result = self.renderer.bridge.with_session(|session| {
            session
                .backend
                .configure_gpu_timing(gpu_profiling.then_some(Default::default()))
        });
        if let Err(error) = result {
            self.renderer.record_error(error);
        }
    }
    fn drain_finished_frames(&mut self) {
        if let Err(error) = self.renderer.collect() {
            self.renderer.record_error(error);
        }
    }
    fn execute_main_graph(&mut self, compiled: Self::Compiled) {
        let expected = compiled.expected;
        let kind = compiled.kind;
        let capture_graph_id = compiled.capture_graph_id;
        match self.renderer.bridge.execute_compiled(compiled.graph) {
            Ok(receipt) => {
                // This receipt proves submission even if capture bookkeeping
                // or export validation below subsequently fails. Abort must
                // preserve that fact rather than report an unsubmitted frame.
                self.renderer.frame_submitted = !receipt.receipts.is_empty();
                if crate::screenshot::shared_texture_capture::installed(self.world) {
                    if let Err(error) = crate::screenshot::shared_texture_capture::submitted(
                        self.world, &mut self.renderer.bridge, capture_graph_id, &receipt,
                    ) { self.renderer.record_error(error); return; }
                }
                match self
                .renderer
                .bridge
                .take_exported_texture(&receipt, compiled.output)
            {
                Ok(texture) => {
                    // The presentation graph imports this exact owner and picks
                    // its resolve shader from `sample_count`; a mismatched export
                    // would silently present an unresolvable/TAA image.
                    let actual = ExportExpectation {
                        format: texture.desc.format.as_raw() as u32,
                        sample_count: texture.desc.sample_count,
                    };
                    if actual != expected {
                        self.renderer.record_error(rhapsody::Error::invalid_usage(format!(
                            "main graph export differs from the declared target: got format 0x{:x} \
                             samples {}, expected 0x{:x} samples {}",
                            actual.format, actual.sample_count, expected.format, expected.sample_count,
                        )));
                        return;
                    }
                    let mut submitted = rhapsody::sync::SyncSet::EMPTY;
                    for point in &receipt.receipts { submitted.insert(*point); }
                    self.renderer.frame_observation = Some(ProductionFrameObservation {
                        world: self.world.id(),
                        application_shader_owner: Arc::as_ptr(&self.renderer.shaders) as usize,
                        graph: capture_graph_id,
                        kind,
                        receipt_frame: receipt.frame,
                        submitted,
                        draw_list_lengths: std::array::from_fn(|i| self.lists.list(i).len()),
                        output_format: actual.format,
                        output_sample_count: actual.sample_count,
                        present_point: None,
                        present_generation: None,
                    });
                    self.renderer.main_output = Some(texture);
                    self.renderer.main = Some((compiled.output, receipt));
                }
                Err(error) => self.renderer.record_error(error),
                }
            }
            Err(error) => {
                crate::screenshot::shared_texture_capture::cancel_unsubmitted(self.world, capture_graph_id);
                self.renderer.record_error(error);
            }
        }
    }
    fn acquire_swapchain_image(&mut self) -> Acquire {
        if self.renderer.first_error.is_some() {
            return Acquire::Unavailable;
        }
        let result = (|| {
            self.renderer.collect()?;
            if self.renderer.pending.len() >= self.renderer.device.frames_in_flight() as usize {
                self.renderer
                    .pending
                    .front()
                    .expect("nonempty")
                    .wait(TIMEOUT_NS)?;
                self.renderer.collect()?;
            }
            Swapchain::validate_swapchain_and_acquire_image(&mut self.renderer.swapchain)
        })();
        match result {
            Ok(Some(image)) => {
                self.renderer.acquired = Some(image);
                Acquire::Image
            }
            Ok(None) => Acquire::Unavailable,
            Err(error) => {
                self.renderer.record_error(error);
                Acquire::Unavailable
            }
        }
    }
    fn execute_presentation_graph(&mut self) -> rhapsody::Result<()> {
        let result = self.execute_presentation_graph_inner();
        if result.is_err() {
            // Whatever failed, the session returns the image (release, or
            // discard + present if a partial graph consumed the acquire).
            self.renderer.presentation.take();
            if let Err(cleanup) = self.renderer.bridge.cancel_presentation() {
                log::warn!(target: crate::render_loop::LOG_TARGET, "presentation cleanup failed: {cleanup}");
            }
        }
        result
    }
    fn present(&mut self) {
        let result = (|| {
            let target = self.renderer.presentation.take().ok_or_else(|| {
                rhapsody::Error::invalid_usage("missing executed presentation graph")
            })?;
            let ticket = self.renderer.bridge.present(&target)?;
            if let Some(frame) = self.renderer.frame_observation.as_mut() {
                frame.present_point = Some(ticket.point);
                frame.present_generation = Some(ticket.generation());
            }
            self.renderer.pending.push_back(ticket);
            self.renderer.frame_present_queued = true;
            Ok::<_, rhapsody::Error>(())
        })();
        if let Err(error) = result {
            self.renderer.record_error(error);
        }
    }
    fn end_frame(&mut self) {
        // The shared session seals its real receipt ledger at execute_prepared;
        // WSI work remains executable until present consumes the acquisition.
        if self.renderer.first_error.is_some() {
            self.renderer.presentation.take();
            if let Err(error) = self.renderer.bridge.cancel_presentation() {
                self.renderer.record_error(error);
            }
            if let Some(image) = self.renderer.acquired.take() {
                if let Err(error) = image.cancel(TIMEOUT_NS) {
                    self.renderer.record_error(error);
                }
            }
        }
    }
    fn finish_screenshot_capture(&mut self) {
        if crate::screenshot::shared_texture_capture::installed(self.world) {
            if let Err(error) = crate::screenshot::finish_screenshot_capture(self.world) {
                self.renderer.record_error(error);
            }
        }
    }
    fn snapshot_render_stats(&mut self) {
        self.renderer.last_stats = self.renderer.stats.snapshot_and_reset();
    }
    fn refresh_memory_budget(&mut self) {
        if let Err(error) = self.renderer.device.refresh_memory_budget() {
            self.renderer.record_error(error);
        }
    }
    fn last_gpu_frame_ns(&mut self) -> Option<u64> {
        let duration = self
            .renderer
            .bridge
            .with_session(|session| session.last_known_frame_duration_ns());
        (duration != 0).then_some(duration)
    }
    fn wait_gpu_queue_trim(&mut self, wait: Duration) {
        let timeout = wait.as_nanos().min(u128::from(u64::MAX)) as u64;
        if let Err(error) = self
            .renderer
            .bridge
            .with_session(|session| session.wait_gpu_queue_trim(timeout))
        {
            self.renderer.record_error(error);
        }
    }
}

#[cfg(test)]
mod world_extent_tests {
    use super::world_extent;
    use bevy_ecs::world::World;
    use country_core::{
        game_config::GameConfig, render::render_config::FrameRenderConfig,
        resources::window::WindowSize,
    };

    fn test_world(window: [u32; 2], scale: Option<f32>) -> World {
        let mut world = World::new();
        world.insert_resource(WindowSize::new(window[0], window[1]));
        let mut config = GameConfig::default();
        config.video.world_resolution_scale = scale;
        world.insert_resource(config);
        world.insert_resource(FrameRenderConfig {
            extent_0: window,
            extent_8: window,
            extent_10: window,
            world_resolution_scale: scale.unwrap_or(1.0),
            tgsr_resolution_scale: 1.0,
            flag_20: false,
        });
        world
    }

    #[test]
    fn extent_consumes_world_resolution_scale() {
        let world = test_world([1280, 720], Some(0.5));
        assert_eq!(world_extent(&world, None, None).unwrap(), [640, 360]);
        let world = test_world([1280, 720], None);
        assert_eq!(world_extent(&world, None, None).unwrap(), [1280, 720]);
    }

    #[test]
    fn min_extent_is_an_override_not_a_floor() {
        let world = test_world([1280, 720], Some(0.5));
        assert_eq!(
            world_extent(&world, None, Some([32, 32])).unwrap(),
            [32, 32]
        );
    }

    #[test]
    fn zero_scale_is_a_named_error() {
        let world = test_world([1280, 720], Some(0.0));
        assert!(world_extent(&world, None, None).is_err());
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
    names
        .into_iter()
        .map(|name| std::ffi::CString::new(name).expect("static extension"))
        .collect()
}

#[cfg(target_os = "macos")]
mod macos {
    use super::*;
    use objc::{
        Encode, Encoding, Message,
        rc::{StrongPtr, autoreleasepool},
        runtime::{Class, Object, Sel, YES},
    };
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    #[link(name = "QuartzCore", kind = "framework")]
    unsafe extern "C" {}
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Size {
        width: f64,
        height: f64,
    }
    unsafe impl Encode for Size {
        fn encode() -> Encoding {
            unsafe { Encoding::from_str("{CGSize=dd}") }
        }
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Rect {
        origin: Point,
        size: Size,
    }
    unsafe impl Encode for Rect {
        fn encode() -> Encoding {
            unsafe { Encoding::from_str("{CGRect={CGPoint=dd}{CGSize=dd}}") }
        }
    }
    pub struct PlatformSurface {
        layer: StrongPtr,
        view: *mut Object,
    }
    impl PlatformSurface {
        pub fn new(window: &winit::window::Window) -> Result<Self> {
            let view = match window.window_handle()?.as_raw() {
                RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr().cast::<Object>(),
                _ => anyhow::bail!("winit did not return an AppKit view"),
            };
            let layer = autoreleasepool(|| -> Result<StrongPtr> {
                let class =
                    Class::get("CAMetalLayer").context("QuartzCore CAMetalLayer is unavailable")?;
                let pointer: *mut Object = unsafe { class.send_message(Sel::register("new"), ()) }?;
                if pointer.is_null() {
                    anyhow::bail!("CAMetalLayer allocation failed");
                }
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
                let (): () =
                    (&**self.layer).send_message(Sel::register("setBounds:"), (bounds,))?;
                let (): () = (&**self.layer)
                    .send_message(Sel::register("setContentsScale:"), (window.scale_factor(),))?;
                let (): () = (&**self.layer).send_message(
                    Sel::register("setDrawableSize:"),
                    (Size {
                        width: f64::from(size.width),
                        height: f64::from(size.height),
                    },),
                )?;
            }
            Ok(())
        }
        pub fn create_surface(
            &self,
            instance: Arc<rhapsody::interop::VulkanInstance>,
        ) -> Result<Arc<rhapsody::present::Surface>> {
            let loader =
                ash::ext::metal_surface::Instance::new(&instance.entry, &instance.instance);
            let info = vk::MetalSurfaceCreateInfoEXT::default().layer((*self.layer).cast());
            let handle = unsafe { loader.create_metal_surface(&info, None) }?;
            Ok(Arc::new(unsafe {
                rhapsody::present::Surface::from_raw(instance, handle, true)
            }))
        }
    }
}

// Host platform integration. This uses winit's actual Win32 handles and the
// existing Surface owner, including its native-window lifetime on failed WSI teardown.
#[cfg(target_os = "windows")]
struct PlatformSurface {
    hwnd: vk::HWND,
    hinstance: vk::HINSTANCE,
    window: Arc<winit::window::Window>,
}
#[cfg(target_os = "windows")]
impl PlatformSurface {
    fn native_handles(window: &winit::window::Window) -> Result<(vk::HWND, vk::HINSTANCE)> {
        use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
        let handle = match window.window_handle()?.as_raw() {
            RawWindowHandle::Win32(handle) => handle,
            _ => anyhow::bail!("winit did not return a Win32 window handle"),
        };
        Ok((
            handle.hwnd.get(),
            handle
                .hinstance
                .context("winit Win32 window has no module instance")?
                .get(),
        ))
    }

    fn new(window: &Arc<winit::window::Window>) -> Result<Self> {
        let (hwnd, hinstance) = Self::native_handles(window)?;
        Ok(Self {
            hwnd,
            hinstance,
            window: window.clone(),
        })
    }

    fn update(&self, window: &winit::window::Window) -> Result<()> {
        // Win32 owns the client extent. ProductionRenderer::resize updates the
        // swapchain separately; never create a second surface on resize/resume.
        let (hwnd, hinstance) = Self::native_handles(window)?;
        anyhow::ensure!(
            self.hwnd == hwnd && self.hinstance == hinstance,
            "Vulkan surface belongs to a different Win32 window"
        );
        Ok(())
    }

    fn create_surface(
        &self,
        instance: Arc<rhapsody::interop::VulkanInstance>,
    ) -> Result<Arc<rhapsody::present::Surface>> {
        // Keep both the native Win32 identity and its actual window owner.
        // Surface destruction releases this lease after vkDestroySurfaceKHR;
        // failed swapchain teardown retains the same lease with the Surface.
        unsafe {
            rhapsody::present::Surface::from_win32_retained(
                instance,
                self.hinstance,
                self.hwnd,
                self.window.clone(),
            )
        }
        .context("create Vulkan Win32 surface")
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
struct PlatformSurface;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
impl PlatformSurface {
    fn new(_: &winit::window::Window) -> Result<Self> {
        anyhow::bail!("native surface adapter is only available for macOS in this host build")
    }
    fn update(&self, _: &winit::window::Window) -> Result<()> {
        anyhow::bail!("native surface adapter is unavailable")
    }
    fn create_surface(
        &self,
        _: Arc<rhapsody::interop::VulkanInstance>,
    ) -> Result<Arc<rhapsody::present::Surface>> {
        anyhow::bail!("native surface adapter is unavailable")
    }
}
