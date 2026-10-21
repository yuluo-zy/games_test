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

#[path = "application_programs.rs"]
mod application_programs;

/// Bounded GPU wait: past this a frame is treated as a hung device. A software rasterizer
/// (a CPU-type physical device such as lavapipe) needs 7–11 s for one world frame, so its bound
/// is raised; real GPUs keep the 10 s bound. Set once when the device is created.
const GPU_TIMEOUT_NS: u64 = 10_000_000_000;
const SOFTWARE_GPU_TIMEOUT_NS: u64 = 180_000_000_000;
static GPU_WAIT_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(GPU_TIMEOUT_NS);
fn timeout_ns() -> u64 {
    GPU_WAIT_NS.load(std::sync::atomic::Ordering::Relaxed)
}
fn set_gpu_wait_for(device: &rhapsody::device::Device) {
    // SAFETY: the instance and physical device belong to this live device.
    let properties = unsafe {
        device
            .instance()
            .raw_instance()
            .get_physical_device_properties(device.physical_device())
    };
    let software = properties.device_type == vk::PhysicalDeviceType::CPU;
    GPU_WAIT_NS.store(
        if software { SOFTWARE_GPU_TIMEOUT_NS } else { GPU_TIMEOUT_NS },
        std::sync::atomic::Ordering::Relaxed,
    );
}

/// 管线预热的等待上限（上限而非等待目标：编译完成即返回）。软件光栅化设备（lavapipe）
/// 编译整套世界管线要数十秒，原先的 10 秒会让首帧前预热超时。
pub(crate) const PIPELINE_PREWARM_TIMEOUT: Duration = Duration::from_secs(180);

/// Original loader closure 0x1400d7440 must successfully render the gameplay
/// App once without a swapchain before publishing it.
/// Its explicit world and final_post dimensions are both 32x32; scene/window
/// resources and internal TAA/TGSR history consumers retain their own inputs.
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

// B async-loader 1400d7a51 passes packed u32x2 0x2000000020 without a
// swapchain. compile_main uses it for both world targets and final_post's R9.
// This is an explicit render argument, not a window/FrameRenderConfig change.
const GAMEPLAY_WARMUP_EXTENT: [u32; 2] = [32, 32];

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
                let extent = world_extent(world, None, Some(GAMEPLAY_WARMUP_EXTENT))?;
                let id = build_full_world_stage(
                    frame, world, lists, extent, Some(GAMEPLAY_WARMUP_EXTENT), None,
                )?;
                let display = display_extent(world, Some(GAMEPLAY_WARMUP_EXTENT))?;
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
            build_full_world_stage(frame, world, &lists, extent, None, None)
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
    // 原窗口链在 compile 成功后、execute 前 drain_finished_frames；只处理真实已完成帧。
    // 新 compiled 仍持有自己的声明和回调，未完成的旧帧也不会在此提前释放。
    if let Err(error) = bridge.collect_completed_frames() {
        drop(compiled);
        crate::screenshot::shared_texture_capture::cancel_unsubmitted(world, capture_graph_id);
        return Err(anyhow::Error::from(error).context("collect completed gameplay frames"));
    }
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

const DIAGNOSTIC_RETAIN_WSI_ENV: &str = "TG_DIAGNOSTIC_RETAIN_WSI";

/// Explicit startup diagnostics only: no environment lookup or formatting on
/// later frames after the once-initialized disabled branch.
pub(crate) fn startup_visual_trace_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("TG_STARTUP_VISUAL_TRACE").is_some_and(|value| value == "1"))
}

#[derive(bevy_ecs::resource::Resource, Clone, Copy)]
struct StartupVisualTargets {
    graph: rhapsody::rg::GraphId,
    color: u32,
    depth: u32,
    taa: u32,
    declared: [[u32; 2]; 3],
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct StartupVisualKey {
    kind: ProductionGraphKind,
    extents: Option<[[u32; 2]; 3]>,
    output: [u32; 2],
    phase: Option<country_core::systems::render_loop::vermeer::VermeerPhase>,
    hold: Option<bool>,
    freeze: Option<bool>,
    debug_freeze: Option<bool>,
    ui_visible: Option<bool>,
    menu_tag: Option<u32>,
}

#[derive(Default)]
struct StartupVisualTraceLog {
    loading_frames: u64,
    main_frames: u64,
    last_key: Option<StartupVisualKey>,
    pending_presentation: Option<(u64, u64)>,
}

fn diagnostic_swapchain_policy(value: Option<&std::ffi::OsStr>) -> Result<rhapsody::present::SwapchainPolicy> {
    use rhapsody::present::SwapchainPolicy;
    match value {
        None => Ok(SwapchainPolicy::Strict),
        Some(value) if value == "0" => Ok(SwapchainPolicy::Strict),
        Some(value) if value == "1" => Ok(SwapchainPolicy::RetainUnproven),
        Some(_) => anyhow::bail!("{DIAGNOSTIC_RETAIN_WSI_ENV} must be unset, 0 or 1"),
    }
}

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
/// display extent used by the TGSR history). The original no-swapchain worker
/// explicitly supplies its 32x32 render extent instead. Used by final_post's
/// R9 and the UI target viewport; zero is an error, never a fallback size.
fn display_extent(world: &World, explicit: Option<[u32; 2]>) -> rhapsody::Result<[u32; 2]> {
    let extent = if let Some(extent) = explicit {
        extent
    } else {
        world
            .get_resource::<country_core::render::render_config::FrameRenderConfig>()
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage(
                    "original compile_main 0x140161cd0: missing FrameRenderConfig for the tail display extent",
                )
            })?
            .extent_8
    };
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
    display_override: Option<[u32; 2]>,
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
    // The loader supplies its original no-swapchain extent. Normal frames
    // retain the same deferred FrameRenderConfig read at this point.
    let display = display_extent(world, display_override)?;
    let tail_controls = crate::world_postprocess::resolve_world_tail_controls(world, decision)?;
    let tail = crate::world_postprocess::build_world_tail(
        frame,
        world,
        lists,
        &out,
        display,
        &tail_controls,
    )?;
    if startup_visual_trace_enabled() {
        let graph = frame.with_graph(|graph| Ok(graph.graph_id()))?;
        world.insert_resource(StartupVisualTargets {
            graph,
            color: color.resource,
            depth: out.depth.resource,
            taa: out.taa.resource,
            declared: [color.desc.extent_2d(), out.depth.desc.extent_2d(), out.taa.desc.extent_2d()],
        });
        if display_override.is_some() {
            log::info!(target: "tiny_glade::startup_visual",
                "warmup graph={graph:?} declared main={:?} depth={:?} taa={:?} final_post={display:?}; complete GPU export/wait still required",
                color.desc.extent_2d(), out.depth.desc.extent_2d(), out.taa.desc.extent_2d());
        }
    }
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
    let started = renderer.timing_start();
    let gpu = &mut renderer.egui;
    let result = renderer.bridge.build_frame(|frame| {
        frame.with_main_prelude(world, |frame, world| {
            build_full_world_stage(frame, world, lists, extent, None, Some((gpu, egui)))
        })
    });
    renderer.timing_end(TimedStage::Declare, started);
    let (output, built) = result?;
    let started = renderer.timing_start();
    let prewarm = renderer
        .bridge
        .shader_backend()
        .prewarm(PIPELINE_PREWARM_TIMEOUT);
    renderer.timing_end(TimedStage::Prewarm, started);
    if let Err(error) = prewarm {
        if let Err(cleanup) = renderer.bridge.cancel_built(built) {
            log::warn!("world pipeline prewarm cancellation: {cleanup}");
        }
        return Err(error);
    }
    let capture_graph_id = built.graph_id();
    if let Some(timing) = &mut renderer.timing { timing.graph = Some(capture_graph_id); }
    let started = renderer.timing_start();
    let result = renderer.bridge.compile_built(built);
    renderer.timing_end(TimedStage::Compile, started);
    let graph = result?;
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
    let started = renderer.timing_start();
    let result = renderer.bridge.build_frame(|frame| {
        let mut host = MinimalWorldHost { frame };
        core_world_frame::build_minimal_world_frame(&mut host, extent)
            .map_err(|error| rhapsody::Error::invalid_usage(error.to_string()))
    });
    renderer.timing_end(TimedStage::Declare, started);
    let (output, built) = result?;
    let started = renderer.timing_start();
    let prewarm = renderer
        .bridge
        .shader_backend()
        .prewarm(PIPELINE_PREWARM_TIMEOUT);
    renderer.timing_end(TimedStage::Prewarm, started);
    if let Err(error) = prewarm {
        if let Err(cleanup) = renderer.bridge.cancel_built(built) {
            log::warn!("minimal world frame prewarm cancellation: {cleanup}");
        }
        return Err(error);
    }
    let capture_graph_id = built.graph_id();
    if let Some(timing) = &mut renderer.timing { timing.graph = Some(capture_graph_id); }
    let started = renderer.timing_start();
    let result = renderer.bridge.compile_built(built);
    renderer.timing_end(TimedStage::Compile, started);
    let graph = result?;
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
    programs_deferred: bool,
}
impl ApplicationGpu {
    fn new(
        device: Arc<rhapsody::device::Device>,
        shader_root: &Path,
        shaders: Arc<AssetShaderLibrary>,
    ) -> Result<Self> {
        let mut application = Self::new_owners(device, shader_root, shaders)?;
        application.final_post_shader = Some(application_programs::register(
            &application.shaders, application.bridge.shader_backend(), &std::sync::atomic::AtomicBool::new(false))?);
        application.programs_deferred = false;
        Ok(application)
    }

    fn new_owners(
        device: Arc<rhapsody::device::Device>, shader_root: &Path, shaders: Arc<AssetShaderLibrary>,
    ) -> Result<Self> {
        // Original CompiledFilePath already contains "compiled-shaders/".
        // This boundary accepts that directory explicitly; pass its owning
        // installation root to the file backend once, for either App.
        let asset_root = shader_root
            .parent()
            .context("compiled shader directory has no installation root")?;
        let bridge = CoreRenderBridge::new(device.clone(), asset_root, shaders.clone())?;
        let egui = EguiGpu::new(device, bridge.hosts().clone());
        Ok(Self {
            bridge,
            egui,
            shaders,
            final_post_shader: None,
            programs_deferred: true,
        })
    }
}

/// Opt-in CPU wall times. Resolve is an inclusive total for the presentation
/// graph; the other graph phases below describe the main graph only.
#[derive(Default)]
struct RenderStageTiming {
    frame: u64,
    graph: Option<rhapsody::rg::GraphId>,
    receipt: Option<u64>,
    draw_lists: Duration,
    declare: Duration,
    prewarm: Duration,
    compile: Duration,
    execute: Duration,
    collect: Duration,
    present_wait: Duration,
    acquire: Duration,
    resolve: Duration,
    present: Duration,
    frame_throttle: Duration,
    queue_trim: Duration,
    queue_budget: Duration,
    observed_completion_wall_ns: u64,
    /// Main-graph recording shape: (batches, parallel batches, chunks).
    main_recording: Option<(u32, u32, u32)>,
    vao_before: Option<(usize, crate::raster_host::VaoRefreshTiming)>,
}

#[derive(Clone, Copy)]
enum TimedStage { DrawLists, Declare, Prewarm, Compile, Execute, Collect, PresentWait, Acquire, Resolve, Present, FrameThrottle, QueueTrim }

pub struct ProductionRenderer {
    bridge: CoreRenderBridge,
    egui: EguiGpu,
    swapchain: Arc<Swapchain>,
    device: Arc<rhapsody::device::Device>,
    shaders: Arc<AssetShaderLibrary>,
    final_post_shader: Option<
        country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,
    >,
    programs_deferred: bool,
    other_applications: BTreeMap<usize, ApplicationGpu>,
    timing: Option<RenderStageTiming>,
    timing_gameplay_count: u64,
    /// Identity (raster owner address) of the bridge whose session has GPU timing
    /// configured; the loading and gameplay Apps own different sessions.
    gpu_timing_bridge: Option<usize>,
    gpu_span_window: crate::frame_counters::GpuSpanWindow,
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
    visual_trace: Option<StartupVisualTraceLog>,
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
        Self::new_with_shader_initialization(window, compiled_shader_root.as_ref(), shaders, false)
    }

    pub(crate) fn new_for_async_gameplay_preparation(
        window: Arc<winit::window::Window>, compiled_shader_root: impl AsRef<Path>, shaders: Arc<AssetShaderLibrary>,
    ) -> Result<Self> {
        Self::new_with_shader_initialization(window, compiled_shader_root.as_ref(), shaders, true)
    }

    fn new_with_shader_initialization(
        window: Arc<winit::window::Window>, compiled_shader_root: &Path, shaders: Arc<AssetShaderLibrary>, defer: bool,
    ) -> Result<Self> {
        let policy = diagnostic_swapchain_policy(std::env::var_os(DIAGNOSTIC_RETAIN_WSI_ENV).as_deref())?;
        if policy == rhapsody::present::SwapchainPolicy::RetainUnproven {
            log::warn!(
                "{DIAGNOSTIC_RETAIN_WSI_ENV}=1: diagnostic WSI RetainUnproven enabled; real presents remain enabled, but final presentation completion may be unproven and shutdown may return an error while retaining GPU owners; this is not strict WSI acceptance"
            );
        }
        let builder = InstanceBuilder {
            application_name: std::ffi::CString::new("Tiny Glade")?,
            extensions: platform_extensions(&window)?,
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
        set_gpu_wait_for(&device);
        let size = window.inner_size();
        let swapchain = Arc::new(unsafe {
            Swapchain::new_with_policy(
                device.clone(),
                surface,
                vk::Extent2D {
                    width: size.width,
                    height: size.height,
                },
                true,
                vk::ImageUsageFlags::COLOR_ATTACHMENT,
                policy,
            )
        }?)
        .clone();
        #[cfg(target_os = "macos")]
        platform.trace_color_space(&swapchain)?;
        let compiled_shader_root = compiled_shader_root.to_path_buf();
        let ApplicationGpu {
            bridge,
            egui,
            shaders,
            final_post_shader,
            programs_deferred,
        } = if defer { ApplicationGpu::new_owners(device.clone(), &compiled_shader_root, shaders)? }
            else { ApplicationGpu::new(device.clone(), &compiled_shader_root, shaders)? };
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
            programs_deferred,
            other_applications: BTreeMap::new(),
            timing: None,
            timing_gameplay_count: 0,
            gpu_timing_bridge: None,
            gpu_span_window: Default::default(),
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
            visual_trace: startup_visual_trace_enabled().then(StartupVisualTraceLog::default),
            suspended: false,
            closed: false,
            reduced_state: 0,
            stats: Default::default(),
            last_stats: Default::default(),
            platform,
            window,
        })
    }

    fn trace_main_export(
        &mut self, world: &World, graph: rhapsody::rg::GraphId,
        receipt: &SessionFrameReceipt, kind: ProductionGraphKind,
        output: &rhapsody::resource::Texture, lists: &DrawLists,
    ) {
        use country_core::systems::render_loop::vermeer::{VermeerState, VermeerDebugControl};
        let Some(trace) = self.visual_trace.as_mut() else { return; };
        let targets = world.get_resource::<StartupVisualTargets>().copied()
            .filter(|targets| targets.graph == graph);
        let ui = world.get_resource::<crate::world_postprocess::StartupUiVisualTrace>().copied()
            .filter(|ui| ui.graph == graph);
        let vermeer = world.get_resource::<VermeerState>();
        let key = StartupVisualKey {
            kind,
            extents: targets.map(|targets| targets.declared),
            output: [output.desc.width, output.desc.height],
            phase: vermeer.map(|state| state.phase),
            hold: vermeer.map(|state| state.hold),
            freeze: vermeer.map(|state| state.freeze_camera),
            debug_freeze: world.get_resource::<VermeerDebugControl>().map(|control| control.freeze),
            ui_visible: if kind == ProductionGraphKind::Loading { Some(true) } else { ui.map(|ui| ui.ui_visible) },
            menu_tag: world.get_resource::<country_core::resources::gui::EscMenuState>().map(|menu| menu.view().tag),
        };
        let count = if kind == ProductionGraphKind::Loading {
            trace.loading_frames = trace.loading_frames.saturating_add(1);
            trace.loading_frames
        } else {
            trace.main_frames = trace.main_frames.saturating_add(1);
            trace.main_frames
        };
        let changed = trace.last_key != Some(key);
        trace.last_key = Some(key);
        if count > 3 && count % 30 != 0 && !changed { return; }
        trace.pending_presentation = Some((receipt.frame, count));
        // Resolve actual allocations from this receipt's physical mapping;
        // unavailable/culled inputs remain None, never copied from declarations.
        let physical = self.bridge.with_session(|session| targets.map(|targets| {
            [targets.color, targets.depth, targets.taa].map(|resource| {
                let token = receipt.snapshot.physical_mapping.iter()
                    .find(|mapping| mapping.logical == resource)?.token?;
                match session.backend.resources.get(&token)? {
                    rhapsody::rg::GpuResource::Image(texture) => Some([texture.desc.width, texture.desc.height]),
                    _ => None,
                }
            })
        }));
        let mei_available = world.get_resource::<crate::loading_graph::LoadingGpuResources>()
            .and_then(|gpu| gpu.mei.output()).map(|text| text.prim_count);
        let mei_drawn = if kind == ProductionGraphKind::Loading { mei_available }
            else { ui.and_then(|ui| ui.mei_drawn_prims) };
        let dt = world.get_resource::<bevy_time::Time>().map(|time| time.delta_secs());
        let config = world.get_resource::<country_core::render::render_config::FrameRenderConfig>();
        log::info!(target: "tiny_glade::startup_visual",
            "main-export n={count} receipt={} graph={graph:?} kind={kind:?} actual_main_depth_taa={physical:?} declared={:?} actual_tail={:?}/samples{} window={:?} config_extents={:?} scale={:?} game_scale={:?} tgsr_scale={:?} ui_visible={:?} ui_elements={:?} mei_available={mei_available:?} mei_drawn={mei_drawn:?} vermeer_phase={:?} hold={:?} freeze_camera={:?} debug_freeze={:?} vermeer_frame={:?} progress={:?} dt={dt:?} capped_dt={:?} menu={:?} logo={:?} lists={:?}",
            receipt.frame, targets.map(|targets| targets.declared), key.output, output.desc.sample_count,
            world.get_resource::<country_core::resources::window::WindowSize>().map(|window| window.as_dims()),
            config.map(|config| [config.extent_0, config.extent_8, config.extent_10]),
            config.map(|config| config.world_resolution_scale),
            world.get_resource::<GameConfig>().map(|config| config.video.world_resolution_scale),
            config.map(|config| config.tgsr_resolution_scale),
            key.ui_visible, ui.and_then(|ui| ui.ui_elements), key.phase, key.hold, key.freeze, key.debug_freeze,
            vermeer.map(|state| state.frame_count), vermeer.map(|state| state.progress),
            dt.map(|dt| if dt < 1.0 / 30.0 { dt } else { 1.0 / 30.0 }), key.menu_tag,
            world.get_resource::<country_core::resources::logo_animation::LogoAnimation>().map(|logo| logo.progress),
            std::array::from_fn::<_, 21, _>(|i| lists.list(i).len()));
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

    pub(crate) fn gameplay_program_initializer(&self) -> Result<
        impl FnOnce(&mut World, &std::sync::atomic::AtomicBool) -> Result<()> + Send + 'static,
    > {
        anyhow::ensure!(self.programs_deferred, "gameplay shader programs were not deferred");
        let library = self.shaders.clone();
        let backend = self.bridge.hosts().shaders.clone();
        Ok(move |world: &mut World, cancelled: &std::sync::atomic::AtomicBool| application_programs::initialize(world, library, backend, cancelled))
    }

    fn finish_deferred_program_registration(&mut self, world: &World) -> Result<()> {
        if !self.programs_deferred { return Ok(()); }
        let registration = world.get_resource::<application_programs::RegisteredApplicationPrograms>()
            .context("gameplay shader programs are still pending their actual worker initialization")?;
        anyhow::ensure!(Arc::ptr_eq(&registration.library, &self.shaders)
            && Arc::ptr_eq(&registration.backend, &self.bridge.hosts().shaders),
            "gameplay shader registration returned different App/library/backend owners");
        anyhow::ensure!(self.shaders.get_raster_handle_by_name("final_post") == Some(registration.final_post),
            "gameplay shader registration final_post handle differs from its canonical library");
        self.final_post_shader = Some(registration.final_post);
        self.programs_deferred = false;
        Ok(())
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
                self.finish_deferred_program_registration(world)?;
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
        std::mem::swap(&mut self.programs_deferred, &mut next.programs_deferred);
        self.other_applications.insert(previous, next);
        if let Some(completed) =
            world.get_resource::<crate::render_bridge::CompletedGameplayWarmup>()
        {
            self.bridge.accept_gameplay_warmup(completed)?;
            self.finish_deferred_program_registration(world)?;
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
    /// 当前交换链代真实 native present 成功次数，供窗口验证使用。
    pub fn native_present_success_count(&self) -> u64 {
        self.swapchain.native_present_success_count()
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
            image.cancel(timeout_ns())?;
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
    /// `TG_GPU_TIMING=1`: enable per-pass timestamps on the main session once,
    /// then drain completed samples every frame and log a window summary.
    /// Does not feed `last_gpu_frame_ns`, so frame pacing is unchanged.
    fn report_gpu_spans(&mut self) {
        if !crate::frame_counters::gpu_timing_enabled() {
            return;
        }
        let bridge_id = std::sync::Arc::as_ptr(self.bridge.raster_resources()) as usize;
        if self.gpu_timing_bridge != Some(bridge_id) {
            self.gpu_timing_bridge = Some(bridge_id);
            let config = rhapsody::rg::GpuTimingConfig {
                pass_spans: true,
                pending_commands: 1024,
                ..Default::default()
            };
            if let Err(error) = self.bridge.with_session(|session| session.backend.configure_gpu_timing(Some(config))) {
                log::warn!(target: "tiny_glade::frame_timing", "TG_GPU_TIMING requested but unavailable: {error}");
                return;
            }
        }
        let window = &mut self.gpu_span_window;
        let polled = self.bridge.with_session(|session| {
            session.backend.poll_gpu_timings()?;
            for sample in session.backend.drain_gpu_timings() {
                window.add(sample.duration_ns, sample.pass_name);
            }
            Ok::<_, rhapsody::Error>(())
        });
        if let Err(error) = polled {
            log::warn!(target: "tiny_glade::frame_timing", "GPU timing poll failed: {error}");
            return;
        }
        if let Some(line) = self.gpu_span_window.end_frame(crate::game_loop::frame_timing_every().max(30)) {
            log::info!(target: "tiny_glade::frame_timing", "{line}");
        }
    }
    fn timing_start(&self) -> Option<Instant> { self.timing.as_ref().map(|_| Instant::now()) }
    fn timing_end(&mut self, stage: TimedStage, started: Option<Instant>) {
        let (Some(timing), Some(started)) = (&mut self.timing, started) else { return; };
        let elapsed = started.elapsed();
        let slot = match stage {
            TimedStage::DrawLists => &mut timing.draw_lists,
            TimedStage::Declare => &mut timing.declare, TimedStage::Prewarm => &mut timing.prewarm,
            TimedStage::Compile => &mut timing.compile, TimedStage::Execute => &mut timing.execute,
            TimedStage::Collect => &mut timing.collect, TimedStage::PresentWait => &mut timing.present_wait,
            TimedStage::Acquire => &mut timing.acquire, TimedStage::Resolve => &mut timing.resolve,
            TimedStage::Present => &mut timing.present, TimedStage::QueueTrim => &mut timing.queue_trim,
            TimedStage::FrameThrottle => &mut timing.frame_throttle,
        };
        *slot = slot.saturating_add(elapsed);
    }
    fn collect(&mut self) -> rhapsody::Result<()> {
        let started = self.timing_start();
        let result = self.collect_inner();
        self.timing_end(TimedStage::Collect, started);
        if self.timing.is_some() {
            let observed = self.bridge.with_session(|session| session.last_known_frame_duration_ns());
            self.timing.as_mut().unwrap().observed_completion_wall_ns = observed;
        }
        result
    }
    fn collect_inner(&mut self) -> rhapsody::Result<()> {
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
        self.bridge.end_display_frame();
        self.presentation.take();
        self.bridge.cancel_presentation()?;
        if let Some(image) = self.acquired.take() {
            image.cancel(timeout_ns())?;
        }
        self.bridge.wait_submits_done(timeout_ns())?;
        for app in self.other_applications.values_mut() {
            app.bridge.wait_submits_done(timeout_ns())?;
        }
        for ticket in &self.pending {
            ticket.wait(timeout_ns())?;
        }
        self.pending.clear();
        self.swapchain.shutdown(timeout_ns())?;
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
        // Close the display frame on every path, including failed frames.
        self.bridge.end_display_frame();
        // Drain the per-frame counters even for frames that are not logged, so
        // a logged frame never carries totals from skipped ones.
        let counters = self.timing.is_some().then(crate::frame_counters::take);
        self.report_gpu_spans();
        let every = crate::game_loop::frame_timing_every();
        if let Some(t) = self.timing.take().filter(|t| t.frame <= 3 || t.frame % every == 0) {
            let ms = |d: Duration| d.as_secs_f64() * 1000.0;
            let c = counters.unwrap_or_default();
            log::info!(target: "tiny_glade::frame_timing",
                "recording_shape gameplay_n={} main_batches_parallel_chunks={:?}",
                t.frame, t.main_recording);
            log::info!(target: "tiny_glade::frame_timing",
                "frame_counters gameplay_n={} upload_wait_n={} upload_wait_ms={:.3} upload_wait_bytes={} upload_capacity_waits={} prewarm_calls={} prewarm_tickets={} prewarm_ms={:.3}",
                t.frame, c.upload_wait_calls, c.upload_wait_ns as f64 / 1_000_000.0, c.upload_wait_bytes, c.upload_capacity_waits,
                c.prewarm_calls, c.prewarm_tickets, c.prewarm_ns as f64 / 1_000_000.0);
            // Compare only the exact App's RasterShared, after adoption. Never
            // subtract counters from another App's same-numbered VAO table.
            let raster = self.bridge.raster_resources();
            let vao = t.vao_before.filter(|(owner, _)| *owner == Arc::as_ptr(raster) as usize)
                .map(|(_, before)| raster.vao_refresh_timing().delta_since(before));
            log::info!(target: "tiny_glade::frame_timing",
                "render_stages gameplay_n={} graph={:?} receipt={:?} draw_lists_ms={:.3} declare_ms={:.3} prewarm_ms={:.3} compile_ms={:.3} record_submit_ms={:.3} collect_ms={:.3} present_ticket_wait_ms={:.3} acquire_ms={:.3} resolve_inclusive_ms={:.3} present_ms={:.3} frame_throttle_ms={:.3} queue_trim_actual_ms={:.3} queue_trim_budget_ms={:.3} observed_completion_wall_ms={:.3} submitted={} present_queued={} error={}; GPU top-level timestamp span unavailable",
                t.frame, t.graph, t.receipt, ms(t.draw_lists), ms(t.declare), ms(t.prewarm), ms(t.compile), ms(t.execute), ms(t.collect), ms(t.present_wait), ms(t.acquire), ms(t.resolve), ms(t.present), ms(t.frame_throttle), ms(t.queue_trim), ms(t.queue_budget), t.observed_completion_wall_ns as f64 / 1_000_000.0,
                self.frame_submitted, self.frame_present_queued, self.first_error.is_some());
            if let Some(vao) = vao {
                log::info!(target: "tiny_glade::frame_timing",
                    "vao_full_refresh gameplay_n={} graph={:?} receipt={:?} calls={} list_entries={} queried_entities={} unique_vaos={} cpu_wall_ms={:.3}; included inside frame phases, not GPU time",
                    t.frame, t.graph, t.receipt, vao.calls, vao.list_entries,
                    vao.queried_entities, vao.unique_vaos, vao.elapsed_ns as f64 / 1_000_000.0);
            }
        }
        self.take_error().map_or(Ok(()), Err)
    }
    fn last_frame_submitted(&self) -> bool { self.frame_submitted }
    fn last_frame_present_queued(&self) -> bool { self.frame_present_queued }
    fn host<'a>(&'a mut self, world: &'a mut World, egui: &'a mut EguiBackend) -> Self::Host<'a> {
        self.timing = if crate::game_loop::frame_timing_enabled() && world.contains_resource::<crate::run_game::GameplayApplication>() {
            self.timing_gameplay_count += 1;
            Some(RenderStageTiming { frame: self.timing_gameplay_count, ..Default::default() })
        } else { None };
        self.frame_submitted = false;
        self.frame_present_queued = false;
        self.frame_observation = None;
        if let Some(trace) = &mut self.visual_trace { trace.pending_presentation = None; }
        if let Err(error) = self.activate_application(world) {
            // Keep the full application-switch diagnostic chain. This is an
            // ownership/setup failure before world compilation, not its OOM
            // retry path; no anyhow context is discarded by downcasting it.
            self.record_error(rhapsody::Error::InvalidUsage {
                what: "B per-App rendering library ownership".into(),
                source: Some(error.into()),
            });
        } else if let Some(timing) = &mut self.timing {
            let raster = self.bridge.raster_resources();
            timing.vao_before = Some((Arc::as_ptr(raster) as usize, raster.vao_refresh_timing()));
        }
        if self.first_error.is_none() {
            // The main and presentation graphs of this frame share one in-flight
            // slot; this is where the CPU waits for the frame before last.
            let started = self.timing_start();
            if let Err(error) = self.bridge.begin_display_frame() {
                self.record_error(error);
            }
            self.timing_end(TimedStage::FrameThrottle, started);
        }
        if self.programs_deferred {
            self.record_error(rhapsody::Error::invalid_usage(
                "gameplay rendering attempted before actual base shader initialization and warmup publication"));
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
        let visual_trace = self.renderer.visual_trace.as_ref().and_then(|trace| trace.pending_presentation);
        let world = &mut *self.world;
        let (target, graph) = self.renderer.bridge.build_frame(|frame| {
            let target = frame.import_swapchain_image(acquired)?;
            if let Some((receipt, count)) = visual_trace {
                let output = target.texture();
                log::info!(target: "tiny_glade::startup_visual",
                    "resolve-declare n={count} main_receipt={receipt} actual_source={}x{}/samples{} actual_swapchain={}x{}/samples{} equal_extent={} alpha=original_resolve_shader flip={flag}",
                    source.desc.width, source.desc.height, source.desc.sample_count,
                    output.desc.width, output.desc.height, output.desc.sample_count,
                    [source.desc.width, source.desc.height] == [output.desc.width, output.desc.height]);
            }
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
            if let Err(error) = image.cancel(timeout_ns()) {
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
        let started = self.renderer.timing_start();
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
        self.renderer.timing_end(TimedStage::DrawLists, started);
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
        let started = self.renderer.timing_start();
        let result = self.renderer.bridge.build_frame(|frame| {
            crate::loading_graph::build_loading_frame(
                frame,
                self.world,
                &shaders,
                swapchain.extent,
                swapchain.format,
                &self.lists,
            )
        });
        self.renderer.timing_end(TimedStage::Declare, started);
        let (output, built) = result?;
        let started = self.renderer.timing_start();
        let prewarm = self
            .renderer
            .bridge
            .shader_backend()
            .prewarm(PIPELINE_PREWARM_TIMEOUT);
        self.renderer.timing_end(TimedStage::Prewarm, started);
        if let Err(error) = prewarm {
            if let Err(cleanup) = self.renderer.bridge.cancel_built(built) {
                log::warn!("pipeline prewarm cancellation: {cleanup}");
            }
            return Err(error);
        }
        let capture_graph_id = built.graph_id();
        if let Some(timing) = &mut self.renderer.timing { timing.graph = Some(capture_graph_id); }
        let started = self.renderer.timing_start();
        let result = self.renderer.bridge.compile_built(built);
        self.renderer.timing_end(TimedStage::Compile, started);
        let graph = result?;
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
            .wait_submits_done(timeout_ns())
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
        let started = self.renderer.timing_start();
        let result = self.renderer.bridge.execute_compiled(compiled.graph);
        self.renderer.timing_end(TimedStage::Execute, started);
        match result {
            Ok(receipt) => {
                if let Some(timing) = &mut self.renderer.timing {
                    timing.receipt = Some(receipt.frame);
                    timing.main_recording = Some((receipt.stats.batches, receipt.stats.parallel_batches, receipt.stats.chunks));
                }
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
                    self.renderer.trace_main_export(self.world, capture_graph_id, &receipt, kind, &texture, &self.lists);
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
                let started = self.renderer.timing_start();
                let waited = self.renderer
                    .pending
                    .front()
                    .expect("nonempty")
                    .wait(timeout_ns());
                self.renderer.timing_end(TimedStage::PresentWait, started);
                waited?;
                self.renderer.collect()?;
            }
            let started = self.renderer.timing_start();
            let result = Swapchain::validate_swapchain_and_acquire_image(&mut self.renderer.swapchain);
            self.renderer.timing_end(TimedStage::Acquire, started);
            result
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
        let started = self.renderer.timing_start();
        let result = self.execute_presentation_graph_inner();
        self.renderer.timing_end(TimedStage::Resolve, started);
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
        let started = self.renderer.timing_start();
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
        self.renderer.timing_end(TimedStage::Present, started);
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
                if let Err(error) = image.cancel(timeout_ns()) {
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
        if let Err(error) = self.renderer.device.refresh_memory_budget_if_due() {
            self.renderer.record_error(error);
        }
    }
    fn last_gpu_frame_ns(&mut self) -> Option<u64> {
        // Original 0x140beaae0 uses two GPU timestamps times timestampPeriod.
        // Session's last_known_frame_duration_ns is submission-to-collection
        // wall time, including CPU idle and pacing. The original top-level GPU
        // span is not recovered yet; do not feed a wall clock into its pacer.
        None
    }
    fn wait_gpu_queue_trim(&mut self, wait: Duration) {
        let started = self.renderer.timing_start();
        if let Some(timing) = &mut self.renderer.timing { timing.queue_budget = timing.queue_budget.saturating_add(wait); }
        let timeout = wait.as_nanos().min(u128::from(u64::MAX)) as u64;
        let result = self
            .renderer
            .bridge
            .with_session(|session| session.wait_gpu_queue_trim(timeout));
        self.renderer.timing_end(TimedStage::QueueTrim, started);
        if let Err(error) = result {
            self.renderer.record_error(error);
        }
    }
}

#[cfg(test)]
mod world_extent_tests {
    use super::{diagnostic_swapchain_policy, display_extent, world_extent, GAMEPLAY_WARMUP_EXTENT};
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
    fn warmup_world_and_tail_extents_override_dimensions_without_mutating_scene_inputs() {
        for (window, scale) in [([1280, 720], Some(0.5)), ([3840, 2160], Some(2.0)),
            ([0, 0], Some(0.0)), ([480, 270], Some(f32::NAN))]
        {
            let world = test_world(window, scale);
            let before = *world.resource::<FrameRenderConfig>();
            assert_eq!(world_extent(&world, None, Some(GAMEPLAY_WARMUP_EXTENT)).unwrap(), [32, 32]);
            assert_eq!(display_extent(&world, Some(GAMEPLAY_WARMUP_EXTENT)).unwrap(), [32, 32]);
            assert_eq!(world.resource::<WindowSize>().as_dims(), window);
            let after = world.resource::<FrameRenderConfig>();
            assert_eq!((after.extent_0, after.extent_8, after.extent_10),
                (before.extent_0, before.extent_8, before.extent_10));
            assert_eq!(after.world_resolution_scale.to_bits(), before.world_resolution_scale.to_bits());
            assert_eq!(after.tgsr_resolution_scale.to_bits(), before.tgsr_resolution_scale.to_bits());
            assert_eq!(after.flag_20, before.flag_20);
            assert_eq!(world.resource::<GameConfig>().video.world_resolution_scale.map(f32::to_bits),
                scale.map(f32::to_bits));
        }
    }

    #[test]
    fn ordinary_frames_keep_scaled_world_unscaled_display_and_oom_override_contract() {
        let world = test_world([1280, 720], Some(0.5));
        assert_eq!(world_extent(&world, None, None).unwrap(), [640, 360]);
        assert_eq!(display_extent(&world, None).unwrap(), [1280, 720]);
        assert_eq!(world_extent(&world, None, Some([64, 64])).unwrap(), [64, 64]);
        assert_eq!(display_extent(&world, None).unwrap(), [1280, 720]);
        assert!(display_extent(&world, Some([32, 0])).is_err());
        assert!(display_extent(&World::new(), None).is_err());
    }

    #[test]
    fn zero_scale_is_a_named_error() {
        let world = test_world([1280, 720], Some(0.0));
        assert!(world_extent(&world, None, None).is_err());
    }

    #[test]
    fn diagnostic_wsi_policy_requires_explicit_opt_in_and_never_uses_acquire_only() {
        use rhapsody::present::SwapchainPolicy;
        use std::ffi::OsStr;
        assert_eq!(diagnostic_swapchain_policy(None).unwrap(), SwapchainPolicy::Strict);
        assert_eq!(diagnostic_swapchain_policy(Some(OsStr::new("0"))).unwrap(), SwapchainPolicy::Strict);
        assert_eq!(diagnostic_swapchain_policy(Some(OsStr::new("1"))).unwrap(), SwapchainPolicy::RetainUnproven);
        for value in ["", "true", "2", "01", " 1", "1 "] {
            let error = diagnostic_swapchain_policy(Some(OsStr::new(value))).unwrap_err();
            assert!(error.to_string().contains("TG_DIAGNOSTIC_RETAIN_WSI"));
        }
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStringExt;
            let invalid = std::ffi::OsString::from_wide(&[0xd800]);
            assert!(diagnostic_swapchain_policy(Some(&invalid)).is_err());
        }
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            assert!(diagnostic_swapchain_policy(Some(OsStr::from_bytes(&[0xff]))).is_err());
        }
    }
}

#[cfg(target_os = "macos")]
use macos::PlatformSurface;
#[cfg(all(unix, not(target_os = "macos")))]
use linux::PlatformSurface;

fn platform_extensions(window: &winit::window::Window) -> Result<Vec<std::ffi::CString>> {
    #[cfg(target_os = "macos")]
    let names = {
        let _ = window;
        vec!["VK_KHR_surface", "VK_EXT_metal_surface"]
    };
    #[cfg(target_os = "windows")]
    let names = {
        let _ = window;
        vec!["VK_KHR_surface", "VK_KHR_win32_surface"]
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let names = linux::surface_extensions(window)?;
    #[cfg(not(any(target_os = "macos", target_os = "windows", all(unix, not(target_os = "macos")))))]
    let names: Vec<&str> = {
        let _ = window;
        Vec::new()
    };
    Ok(names
        .into_iter()
        .map(|name| std::ffi::CString::new(name).expect("static extension"))
        .collect())
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
        /// 仅显式诊断时读取驱动已设置的 layer 色彩身份；不修改颜色空间。
        pub fn trace_color_space(&self, swapchain: &Swapchain) -> Result<()> {
            static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
            if !*ENABLED.get_or_init(|| std::env::var_os("TG_COLOR_SPACE_DIAGNOSTIC").is_some()) {
                return Ok(());
            }
            use core_foundation::base::{CFType, TCFType};
            use core_graphics::color_space::{CGColorSpace, kCGColorSpaceSRGB};
            use foreign_types::ForeignType;

            let selected = swapchain.surface_format()
                .context("color diagnostic requires a successfully created native swapchain")?;
            // colorspace 是 CAMetalLayer 的 Get 属性。layer 在调用期间强持有；
            // 按 CF Get 规则 retain，局部 RAII drop 不转移或释放 layer 的引用。
            let raw: *const std::ffi::c_void = unsafe {
                (&**self.layer).send_message(Sel::register("colorspace"), ())
            }?;
            let actual = if raw.is_null() {
                None
            } else {
                Some(unsafe { CFType::wrap_under_get_rule(raw) })
            };
            // 使用框架创建的真正 sRGB 身份并由 CFEqual 比较，不用裸指针或描述字符串猜测。
            let srgb = CGColorSpace::create_with_name(unsafe { kCGColorSpaceSRGB })
                .context("CoreGraphics cannot create the sRGB color space")?;
            let srgb_identity = unsafe { CFType::wrap_under_get_rule(srgb.as_ptr().cast()) };
            let actual_is_color_space = actual.as_ref()
                .is_some_and(|actual| actual.type_of() == CGColorSpace::type_id());
            let actual_is_srgb = actual.as_ref().is_some_and(|actual| actual == &srgb_identity);
            eprintln!("TG_COLOR_SPACE_DIAGNOSTIC format={} color_space={} layer_present={} layer_is_color_space={} layer_is_srgb={}",
                selected.format.as_raw(), selected.color_space.as_raw(), actual.is_some(),
                actual_is_color_space, actual_is_srgb);
            Ok(())
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
                ash::ext::metal_surface::Instance::new(&instance.entry, instance.raw_instance());
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

// Linux host surface (X11 via Xlib/Xcb, Wayland), built from winit's actual raw handles. The
// original ships only on Windows and macOS; this adapter exists so the real window/swapchain
// path can run on Linux (lavapipe/Mesa WSI). The native window owner is kept by the renderer.
#[cfg(all(unix, not(target_os = "macos")))]
mod linux {
    use super::*;
    use winit::raw_window_handle::{
        HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    };

    pub fn surface_extensions(window: &winit::window::Window) -> Result<Vec<&'static str>> {
        let ext = match window.display_handle()?.as_raw() {
            RawDisplayHandle::Xlib(_) => "VK_KHR_xlib_surface",
            RawDisplayHandle::Xcb(_) => "VK_KHR_xcb_surface",
            RawDisplayHandle::Wayland(_) => "VK_KHR_wayland_surface",
            other => anyhow::bail!("unsupported Linux display handle {other:?}"),
        };
        Ok(vec!["VK_KHR_surface", ext])
    }

    pub struct PlatformSurface {
        window: RawWindowHandle,
        display: RawDisplayHandle,
    }
    // SAFETY: the raw handles are plain identifiers; the renderer keeps the winit window alive
    // for as long as this value and every Vulkan surface created from it exist.
    unsafe impl Send for PlatformSurface {}
    unsafe impl Sync for PlatformSurface {}

    impl PlatformSurface {
        pub fn new(window: &winit::window::Window) -> Result<Self> {
            Ok(Self {
                window: window.window_handle()?.as_raw(),
                display: window.display_handle()?.as_raw(),
            })
        }
        pub fn update(&self, window: &winit::window::Window) -> Result<()> {
            // The swapchain tracks the client extent; never create a second surface on resize.
            anyhow::ensure!(
                window.window_handle()?.as_raw() == self.window,
                "Vulkan surface belongs to a different native window"
            );
            Ok(())
        }
        pub fn create_surface(
            &self,
            instance: Arc<rhapsody::interop::VulkanInstance>,
        ) -> Result<Arc<rhapsody::present::Surface>> {
            let handle = match (self.window, self.display) {
                (RawWindowHandle::Xlib(w), RawDisplayHandle::Xlib(d)) => {
                    let display = d.display.context("Xlib display is null")?;
                    let loader =
                        ash::khr::xlib_surface::Instance::new(&instance.entry, instance.raw_instance());
                    let info = vk::XlibSurfaceCreateInfoKHR::default()
                        .dpy(display.as_ptr().cast())
                        .window(w.window);
                    unsafe { loader.create_xlib_surface(&info, None) }
                        .context("create Vulkan Xlib surface")?
                }
                (RawWindowHandle::Xcb(w), RawDisplayHandle::Xcb(d)) => {
                    let connection = d.connection.context("XCB connection is null")?;
                    let loader =
                        ash::khr::xcb_surface::Instance::new(&instance.entry, instance.raw_instance());
                    let info = vk::XcbSurfaceCreateInfoKHR::default()
                        .connection(connection.as_ptr().cast())
                        .window(w.window.get());
                    unsafe { loader.create_xcb_surface(&info, None) }
                        .context("create Vulkan XCB surface")?
                }
                (RawWindowHandle::Wayland(w), RawDisplayHandle::Wayland(d)) => {
                    let loader = ash::khr::wayland_surface::Instance::new(
                        &instance.entry,
                        instance.raw_instance(),
                    );
                    let info = vk::WaylandSurfaceCreateInfoKHR::default()
                        .display(d.display.as_ptr().cast())
                        .surface(w.surface.as_ptr().cast());
                    unsafe { loader.create_wayland_surface(&info, None) }
                        .context("create Vulkan Wayland surface")?
                }
                _ => anyhow::bail!("winit window and display handles are of different kinds"),
            };
            Ok(Arc::new(unsafe {
                rhapsody::present::Surface::from_raw(instance, handle, true)
            }))
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows", unix)))]
struct PlatformSurface;
#[cfg(not(any(target_os = "macos", target_os = "windows", unix)))]
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
