//! Window Lab scenes use the production graph/backend and existing draw-test
//! shaders. CPU code supplies vertices and an independent coverage reference;
//! the displayed image is produced by raster and compute passes on the GPU.
use super::fixture::{HEIGHT, WIDTH};
use ash::vk::{self, Handle};
use rhapsody::{
    bindless::{
        BindlessConfig, BindlessHeap, BindlessParameters, BindlessResource, BindlessShaderPackage,
    },
    pipeline::{
        AsyncPipelineCompiler, CompilerConfig, HotPipeline, PersistentPipelineCache, PipelinePoll,
        PipelineTicket,
    },
    raw::{
        buffer::{Buffer, BufferDesc},
        device::Device,
        dynamic_constants::DynamicConstants,
        shader::{
            DynamicRenderingFormats, PipelineLayout, RasterPipelineDesc, RasterPipelineOpts,
            ShaderModule, VulkanPipeline,
        },
        texture::{Texture, TextureDesc, TextureViewDesc},
    },
    rg::{
        attachments::{Attachment, AttachmentCache, RenderingPolicy},
        executor::PassContext,
        frame_loop::FrameLoop,
        graph::BuiltGraph,
        pass_api::{AshCommandBackend, RenderPassApi},
        vulkan::{GpuResource, ResourceSpec},
    },
    shader_compiler,
    shader_pipeline::{
        compiled::{CompiledShaderPipeline, CompiledShaderStage, ShaderStage},
        reflection::BindingLocation,
    },
};
use serde::Serialize;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::{path::Path, time::Duration};

#[path = "../../tests/draw_support/mod.rs"]
mod draw_support;

const VERTEX_READ: u8 = 0x04;
const HOST_READ: u8 = 0x26;
const HOST_WRITE: u8 = 0x35;
const COLOR_WRITE: u8 = 0x2e;
const COMPUTE_READ: u8 = 0x1a;
const COMPUTE_WRITE: u8 = 0x32;
const TRANSFER_READ: u8 = 0x25;
const TRANSFER_WRITE: u8 = 0x34;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneKind {
    #[default]
    Fixture,
    Raster,
    Temporal,
    Transfer,
    Transient,
    ResultCache,
    MultiQueue,
    Showcase,
}
impl SceneKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Showcase => "continuous GPU showcase",
            Self::Fixture => "fixture",
            Self::Raster => "S03 GPU raster + compute postprocess",
            Self::Temporal => "S04 GPU temporal pingpong + reset",
            Self::Transfer => "S07 bounded GPU transfer + delayed readback",
            Self::Transient => "S05 GPU transient attachment chain",
            Self::ResultCache => "S06 versioned GPU result cache",
            Self::MultiQueue => "S08 graphics/compute/transfer dependency chain",
        }
    }
    pub fn parse(value: &str) -> super::fixture::Result<Self> {
        match value {
            "showcase" => Ok(Self::Showcase),
            "fixture" => Ok(Self::Fixture),
            "raster" => Ok(Self::Raster),
            "temporal" => Ok(Self::Temporal),
            "transfer" => Ok(Self::Transfer),
            "transient" => Ok(Self::Transient),
            "result" => Ok(Self::ResultCache),
            "multi-queue" => Ok(Self::MultiQueue),
            _ => Err(
                "scene must be fixture, raster, temporal, transfer, transient, result, or multi-queue".into(),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderingChoice {
    #[default]
    Auto,
    Dynamic,
}
impl RenderingChoice {
    pub fn parse(value: &str) -> super::fixture::Result<Self> {
        match value {
            "auto" => Ok(Self::Auto),
            "dynamic" => Ok(Self::Dynamic),
            _ => Err("rendering must be auto or dynamic".into()),
        }
    }
    fn dynamic(self, enabled: bool, _frame: u64) -> rhapsody::Result<bool> {
        RenderingPolicy::RequireDynamic.select_dynamic(enabled, true)
    }
    pub fn input_frame(self, frame: u64) -> u64 {
        frame
    }
}

#[derive(Default, Serialize)]
pub struct SceneReport {
    pub scene: SceneKind,
    pub rendering: RenderingChoice,
    pub bindless_enabled: bool,
    pub bindless_verified_frames: u64,
    pub bindless_capacity: usize,
    pub bindless_peak_live: usize,
    pub bindless_peak_retiring: usize,
    pub dynamic_rendering_frames: u64,
    pub attachment_cache_views: usize,
    pub gpu_program_hash: Option<String>,
    pub pipeline_cache_status: Option<String>,
    pub pipeline_cache_saved: bool,
    pub pipeline_compiled_requests: u64,
    pub pipeline_merged_requests: u64,
    pub pipeline_peak_pending: usize,
    pub pipeline_peak_request_bytes: usize,
    pub shader_rejected_edits: u64,
    pub shader_completed_rounds: u64,
    pub shader_discarded_partial_rounds: u64,
    pub shader_frame_boundary_publications: u64,
    pub shader_preserved_frames: u64,
    pub shader_reloaded_verified_frames: u64,
    pub shader_initial_implementation_hash: Option<String>,
    pub shader_published_implementation_hash: Option<String>,
    pub rasterized_frames: u64,
    pub postprocessed_frames: u64,
    pub reference_verified_frames: u64,
    pub worst_reference_mismatches: usize,
    pub worst_boundary_pixels: usize,
    pub history_verified_frames: u64,
    pub history_reset_count: u64,
    pub history_initializations: u64,
    pub history_reset_reasons: Vec<String>,
    pub injected_failed_frames: u64,
    pub failure_recovery_verified_frames: u64,
}

pub fn program_fingerprint() -> String {
    let (mut bytes, fragment) = draw_support::shaders();
    bytes.extend(fragment);
    bytes.extend(postprocess_shader());
    bytes.extend(WIDTH.to_le_bytes());
    bytes.extend(HEIGHT.to_le_bytes());
    format!("{:016x}", super::fixture::hash(&bytes))
}

fn extent() -> vk::Extent2D {
    vk::Extent2D {
        width: WIDTH,
        height: HEIGHT,
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

/// Completed rounds may restart only on the next 300-frame period, leaving
/// time for both asynchronous requests and a verified publication at the tail.
fn next_shader_round(
    phase: u8,
    round: u64,
    frame: u64,
    repeat: bool,
    limit: Option<u64>,
) -> Option<u64> {
    let next = frame / 300;
    (repeat
        && phase == 4
        && next > round
        && limit.is_none_or(|end| end.saturating_sub(frame) >= 120))
    .then_some(next)
}

fn discardable_shader_round(phase: u8, report: &SceneReport) -> rhapsody::Result<bool> {
    if report.shader_completed_rounds == 0
        || report.shader_rejected_edits < report.shader_completed_rounds
        || report.shader_frame_boundary_publications != report.shader_completed_rounds
        || report.shader_reloaded_verified_frames == 0
        || report.reference_verified_frames == 0
        || report.worst_reference_mismatches != 0
    {
        return Err(rhapsody::Error::invalid_usage(
            "minimum-run shader script needs a completed and pixel-verified round",
        ));
    }
    Ok(phase != 4)
}
fn discarded_reload_result(
    phase: u8,
    cancelled: bool,
    result: Result<(), Arc<rhapsody::Error>>,
) -> rhapsody::Result<()> {
    match result {
        Ok(()) if phase == 3 => Ok(()),
        Ok(()) => Err(rhapsody::Error::invalid_usage(
            "broken shader unexpectedly compiled at shutdown",
        )),
        Err(error)
            if phase == 1
                && matches!(error.as_ref(), rhapsody::Error::InvalidUsage { what, .. } if what == "invalid hot-reload SPIR-V") =>
        {
            Ok(())
        }
        Err(error) if cancelled && matches!(error.as_ref(), rhapsody::Error::Timeout) => Ok(()),
        Err(error) => Err(compile_error(error)),
    }
}

pub struct RasterScene {
    device: Arc<Device>,
    dynamic_raster: Option<Arc<VulkanPipeline>>,
    rendering: RenderingChoice,
    postprocess: HotPipeline,
    postprocess_layout: Arc<PipelineLayout>,
    compiler: AsyncPipelineCompiler,
    pipeline_cache: PersistentPipelineCache,
    reload_ticket: Option<PipelineTicket>,
    reload_phase: u8,
    reload_round: u64,
    alternate_shader: bool,
    attachments: Arc<Mutex<AttachmentCache>>,
    bindless: Arc<BindlessHeap>,
    raster_abi: Arc<rhapsody::bindless::BindlessShaderAbi>,
    postprocess_abi: Arc<rhapsody::bindless::BindlessShaderAbi>,
    parameters: Arc<Mutex<DynamicConstants>>,
    vertices: Arc<Buffer>,
    mesh: bool,
    performance: bool,
    vertex_token: u64,
    temporal: Option<History>,
    inject_failure: bool,
    failure_reached: Arc<AtomicBool>,
    recovery_pending: bool,
}
struct History {
    desc: TextureDesc,
    readback: Arc<Buffer>,
    readback_token: u64,
    previous_pixels: Option<Vec<u8>>,
    expected_pixels: Option<Vec<u8>>,
    initialized: bool,
}
impl RasterScene {
    pub fn new(
        frame_loop: &mut FrameLoop,
        kind: SceneKind,
        rendering: RenderingChoice,
        cache_path: &Path,
    ) -> rhapsody::Result<Self> {
        let device = frame_loop.swapchain().device.clone();
        let dynamic_enabled = device
            .caps()
            .is_enabled(rhapsody::device::Feature::DynamicRendering);
        rendering.dynamic(dynamic_enabled, 0)?;
        let pipeline_cache = PersistentPipelineCache::open(device.clone(), cache_path, 32)?;
        let compiler = AsyncPipelineCompiler::new(
            pipeline_cache.cache.clone(),
            CompilerConfig {
                workers: 1,
                max_pending: 8,
                max_request_bytes: 1 << 20,
                max_cached_requests: 16,
            },
        )?;
        let bindless = BindlessHeap::new(
            device.clone(),
            BindlessConfig {
                sampled_images: 4,
                storage_images: 64,
                storage_buffers: 64,
                samplers: 4,
            },
        )?;
        let (vertex, fragment) = draw_support::shaders();
        let original_raster = scene_program(&[
            (ShaderStage::Vertex, "vs_main", vertex),
            (ShaderStage::Fragment, "ps_main", fragment),
        ])?;
        let raster_package = BindlessShaderPackage::new(&bindless, &original_raster, 64)?;
        let vertex =
            ShaderModule::from_spirv(device.clone(), &raster_package.stages[0].spirv_bytes())?;
        let fragment =
            ShaderModule::from_spirv(device.clone(), &raster_package.stages[1].spirv_bytes())?;
        let mut raster = RasterPipelineDesc::new(vertex, raster_package.layout.clone());
        raster.fragment = Some(fragment);
        raster.fragment_entry = Some(c"ps_main".to_owned());
        raster.use_dynamic_rendering(DynamicRenderingFormats::color(vk::Format::R8G8B8A8_UNORM));
        raster.vertex_bindings = vec![vk::VertexInputBindingDescription {
            binding: 0,
            stride: 16,
            input_rate: vk::VertexInputRate::VERTEX,
        }];
        raster.vertex_attributes = vec![vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: 0,
        }];
        RasterPipelineOpts(0).apply_to(&mut raster, 1)?;
        let dynamic_raster = Some(
            compiler
                .request_raster(raster)?
                .wait(Duration::from_secs(5))
                .map_err(compile_error)?,
        );
        let original_compute = postprocess_program(false)?;
        let package = BindlessShaderPackage::new(&bindless, &original_compute, 64)?;
        let postprocess_layout = package.layout;
        let postprocess = compiler
            .request_compute_spirv(
                package.stages[0].spirv_bytes(),
                c"main".to_owned(),
                postprocess_layout.clone(),
            )?
            .wait(Duration::from_secs(5))
            .map_err(compile_error)?;
        let parameters = Arc::new(Mutex::new(DynamicConstants::new(device.clone())?));
        let vertices = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(3072, vk::BufferUsageFlags::VERTEX_BUFFER),
        )?);
        let vertex_token = frame_loop
            .graph
            .backend
            .bind_import(GpuResource::Buffer(vertices.clone()))?;
        let temporal = if kind == SceneKind::Temporal {
            let mut desc = TextureDesc::new_2d(WIDTH, HEIGHT, vk::Format::R8G8B8A8_UNORM);
            desc.usage = vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST;
            let readback = Arc::new(Buffer::new(
                device.clone(),
                &BufferDesc::new_gpu_to_cpu(
                    u64::from(WIDTH * HEIGHT * 4),
                    vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
                ),
            )?);
            let readback_token = frame_loop
                .graph
                .backend
                .bind_import(GpuResource::Buffer(readback.clone()))?;
            Some(History {
                desc,
                readback,
                readback_token,
                previous_pixels: None,
                expected_pixels: None,
                initialized: false,
            })
        } else {
            None
        };
        Ok(Self {
            attachments: Arc::new(Mutex::new(AttachmentCache::new(device.clone())?)),
            device,
            dynamic_raster,
            rendering,
            postprocess: HotPipeline::new(postprocess),
            postprocess_layout,
            compiler,
            pipeline_cache,
            reload_ticket: None,
            reload_phase: 0,
            reload_round: 0,
            alternate_shader: false,
            bindless,
            raster_abi: raster_package.abi,
            postprocess_abi: package.abi,
            parameters,
            vertices,
            mesh: false,
            performance: false,
            vertex_token,
            temporal,
            inject_failure: false,
            failure_reached: Arc::new(AtomicBool::new(false)),
            recovery_pending: false,
        })
    }

    pub fn set_performance(&mut self, enabled: bool) {
        self.performance = enabled;
    }
    pub fn history_buffer(&self) -> Option<Arc<Buffer>> {
        self.temporal.as_ref().map(|h| h.readback.clone())
    }
    pub fn history_display(&self) -> Option<(Arc<Buffer>, &[u8])> {
        self.temporal.as_ref().and_then(|h| {
            h.expected_pixels
                .as_deref()
                .map(|p| (h.readback.clone(), p))
        })
    }
    pub fn enable_mesh(&mut self) {
        self.mesh = true;
    }
    pub fn build_graph(
        &mut self,
        frame_loop: &mut FrameLoop,
        image: &Arc<Texture>,
        readback: &Arc<Buffer>,
        tokens: [u64; 3],
        frame: u64,
        first: bool,
    ) -> rhapsody::Result<BuiltGraph> {
        let frame = self.rendering.input_frame(frame);
        let triangle = draw_support::rotated_triangle((frame % 24) as f32 * 15.0_f32.to_radians());
        // RenderPassApi's negative-height viewport flips Y; compensate in the
        // vertex input, matching the existing graph animation acceptance test.
        let vertices = if self.mesh {
            subdivide_triangle(triangle)
        } else {
            triangle.to_vec()
        };
        let vertex_count = vertices.len() as u32;
        let bytes = vertices
            .into_iter()
            .flat_map(|v| [v[0], -v[1], v[2], v[3]])
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>();
        self.vertices.write_bytes(0, &bytes)?;
        let mut graph = frame_loop.begin_frame()?;
        let history = if let Some(history) = &mut self.temporal {
            let pair =
                frame_loop
                    .graph
                    .temporal_texture(&mut graph, "window.history", history.desc)?;
            history.initialized = !pair.history_valid();
            history.expected_pixels = if self.performance {
                None
            } else {
                Some(if pair.history_valid() {
                    history.previous_pixels.clone().ok_or_else(|| {
                        rhapsody::Error::invalid_usage(
                            "S04 valid history has no preceding verified frame",
                        )
                    })?
                } else {
                    vec![0; (WIDTH * HEIGHT * 4) as usize]
                })
            };
            let readback = graph.import_unowned(
                "scene.history_readback",
                ResourceSpec::Buffer(history.readback.desc).descriptor(),
                history.readback_token,
                HOST_READ,
            )?;
            if !pair.history_valid() {
                let device = self.device.clone();
                let index = pair.previous().raw().raw();
                let mut pass = graph.add_pass("S04 initialize invalid previous history");
                pass.add_resource_access(pair.previous().raw(), TRANSFER_WRITE, 0)?;
                pass.render_with_result_context(move |ctx: PassContext<'_>| {
                    let managed = ctx.managed.ok_or_else(|| {
                        rhapsody::Error::invalid_usage("scene managed resources absent")
                    })?;
                    let texture = managed
                        .texture_for(index)
                        .map_err(|e| rhapsody::Error::invalid_usage(format!("{e:?}")))?;

                    unsafe {
                        device.device.cmd_clear_color_image(
                            vk::CommandBuffer::from_raw(ctx.command.handle()),
                            texture.raw(),
                            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                            &vk::ClearColorValue { uint32: [0; 4] },
                            &[vk::ImageSubresourceRange::default()
                                .aspect_mask(vk::ImageAspectFlags::COLOR)
                                .level_count(1)
                                .layer_count(1)],
                        );
                    }
                    Ok(())
                });
            }
            Some((pair, readback, history.readback.clone()))
        } else {
            None
        };
        let vertices = graph.import_unowned(
            "scene.vertices",
            ResourceSpec::Buffer(self.vertices.desc).descriptor(),
            self.vertex_token,
            HOST_WRITE,
        )?;
        let mut color_desc = TextureDesc::new_2d(WIDTH, HEIGHT, vk::Format::R8G8B8A8_UNORM);
        color_desc.usage = vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::STORAGE;
        let color = frame_loop.graph.backend.declare(
            &mut graph,
            "scene.raster",
            ResourceSpec::Image(color_desc),
        )?;
        let output = graph.import_unowned(
            "lab.image",
            ResourceSpec::Image(image.desc).descriptor(),
            tokens[1],
            if first { 0 } else { TRANSFER_READ },
        )?;
        let readback_handle = graph.import_unowned(
            "lab.readback",
            ResourceSpec::Buffer(readback.desc).descriptor(),
            tokens[2],
            HOST_READ,
        )?;
        {
            let device = self.device.clone();
            let pipeline = self
                .dynamic_raster
                .as_ref()
                .expect("dynamic raster pipeline")
                .clone();
            let cache = self.attachments.clone();
            let heap = self.bindless.clone();
            let abi = self.raster_abi.clone();
            let parameters = self.parameters.clone();
            let (color_index, vertex_index) = (color.raw(), vertices.raw());
            let clear =
                draw_support::frame_clear_color(frame as usize).map(|v| f32::from(v) / 255.0);
            let mut pass = graph.add_pass("S03 GPU raster triangle");
            pass.add_resource_access(vertices, VERTEX_READ, 0)?;
            pass.add_resource_access(color, COLOR_WRITE, 0)?;
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                let record = || -> rhapsody::Result<()> {
                    let managed = ctx.managed.ok_or_else(|| {
                        rhapsody::Error::invalid_usage("scene managed resources absent")
                    })?;
                    let texture = managed
                        .texture_for(color_index)
                        .map_err(|e| rhapsody::Error::invalid_usage(format!("{e:?}")))?;
                    let vertices = managed
                        .buffer_for(vertex_index)
                        .map_err(|e| rhapsody::Error::invalid_usage(format!("{e:?}")))?;
                    let attachment = Attachment::color(texture.shared(), 0, Some(clear))?;
                    let prepared = cache
                        .lock()
                        .expect("attachment cache")
                        .prepare_for_rendering(
                            &[attachment],
                            extent(),
                            1,
                            RenderingPolicy::RequireDynamic,
                        )?;
                    if prepared.dynamic_formats() != pipeline.dynamic_rendering_formats() {
                        return Err(rhapsody::Error::invalid_usage(
                            "pipeline and prepared rendering formats differ",
                        ));
                    }
                    prepared.retain_for_pass(&ctx)?;

                    let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
                    let mut backend = unsafe { AshCommandBackend::new_for_device(&device) };
                    let mut api = RenderPassApi::new(&mut backend, cb, 1, false);
                    api.begin_prepared(&prepared)?;
                    api.set_default_view_and_scissor(extent());
                    api.set_stencil_ref(0);
                    let prepared_parameters =
                        BindlessParameters::new(heap.clone(), abi.clone(), &[])?
                            .finish(&mut parameters.lock().expect("scene parameter pages"))?;
                    unsafe {
                        device.device.cmd_bind_pipeline(
                            cb,
                            vk::PipelineBindPoint::GRAPHICS,
                            pipeline.handle,
                        );
                        prepared_parameters.bind_pass(&ctx, pipeline.clone())?;
                    }
                    api.bind_vertex_buffer_by_binding_point(0, vertices.raw());
                    api.draw(vertex_count, 1, 0, 0);
                    api.end_render_pass();
                    Ok(())
                };
                record()
            });
        }
        {
            let device = self.device.clone();
            let pipeline = self.postprocess.current();
            let heap = self.bindless.clone();
            let abi = self.postprocess_abi.clone();
            let parameters = self.parameters.clone();
            let (source_index, target_index) = (color.raw(), output.raw());
            let mut pass = graph.add_pass("S03 GPU color postprocess");
            pass.add_resource_access(color, COMPUTE_READ, 0)?;
            pass.add_resource_access(output, COMPUTE_WRITE, 0)?;
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                let record = || -> rhapsody::Result<()> {
                    let managed = ctx.managed.ok_or_else(|| {
                        rhapsody::Error::invalid_usage("scene managed resources absent")
                    })?;
                    let source = managed
                        .texture_for(source_index)
                        .map_err(|e| rhapsody::Error::invalid_usage(format!("{e:?}")))?
                        .shared();
                    let target = managed
                        .texture_for(target_index)
                        .map_err(|e| rhapsody::Error::invalid_usage(format!("{e:?}")))?
                        .shared();
                    let source_view =
                        Arc::new(source.image_view(&TextureViewDesc::new(&source.desc)?)?);
                    let target_view =
                        Arc::new(target.image_view(&TextureViewDesc::new(&target.desc)?)?);
                    let mut bindings = BindlessParameters::new(heap.clone(), abi.clone(), &[])?;
                    for (binding, view) in [(0, source_view), (1, target_view)] {
                        bindings.write(
                            BindingLocation { set: 0, binding },
                            0,
                            BindlessResource::StorageImage(view),
                        )?;
                    }
                    let prepared =
                        bindings.finish(&mut parameters.lock().expect("scene parameter pages"))?;
                    let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
                    unsafe {
                        device.device.cmd_bind_pipeline(
                            cb,
                            vk::PipelineBindPoint::COMPUTE,
                            pipeline.handle,
                        );
                        prepared.bind_pass(&ctx, pipeline.clone())?;
                    }
                    unsafe {
                        device.device.cmd_dispatch(cb, WIDTH / 8, HEIGHT / 4, 1);
                    }
                    Ok(())
                };
                record()
            });
        }
        {
            let device = self.device.clone();
            let image = image.clone();
            let readback = readback.clone();
            let mut pass = graph.add_pass("S03 readback postprocessed image");
            pass.add_resource_access(output, TRANSFER_READ, 0)?;
            pass.add_resource_access(readback_handle, TRANSFER_WRITE, 0)?;
            pass.render_with_context(move |ctx: PassContext<'_>| {
                unsafe {
                    device.device.cmd_copy_image_to_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        image.handle,
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        readback.handle,
                        &[copy_region()],
                    );
                }
                true
            });
        }
        graph.export(output, TRANSFER_READ)?;
        graph.export(readback_handle, HOST_READ)?;
        if let Some((pair, history_readback, target)) = history {
            let device = self.device.clone();
            let source = image.clone();
            let (current_index, previous_index) =
                (pair.current().raw().raw(), pair.previous().raw().raw());
            let mut pass = graph.add_pass("S04 GPU history pingpong and readback");
            pass.add_resource_access(output, TRANSFER_READ, 0)?;
            pass.add_resource_access(pair.current().raw(), TRANSFER_WRITE, 0)?;
            pass.add_resource_access(pair.previous().raw(), TRANSFER_READ, 0)?;
            pass.add_resource_access(history_readback, TRANSFER_WRITE, 0)?;
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                let managed = ctx.managed.ok_or_else(|| {
                    rhapsody::Error::invalid_usage("scene managed resources absent")
                })?;
                let current = managed
                    .texture_for(current_index)
                    .map_err(|e| rhapsody::Error::invalid_usage(format!("{e:?}")))?;
                let previous = managed
                    .texture_for(previous_index)
                    .map_err(|e| rhapsody::Error::invalid_usage(format!("{e:?}")))?;

                let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
                let layers = vk::ImageSubresourceLayers::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .layer_count(1);
                unsafe {
                    device.device.cmd_copy_image(
                        cb,
                        source.handle,
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        current.raw(),
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &[vk::ImageCopy::default()
                            .src_subresource(layers)
                            .dst_subresource(layers)
                            .extent(vk::Extent3D {
                                width: WIDTH,
                                height: HEIGHT,
                                depth: 1,
                            })],
                    );
                    device.device.cmd_copy_image_to_buffer(
                        cb,
                        previous.raw(),
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        target.handle,
                        &[copy_region()],
                    );
                }
                Ok(())
            });
            graph.export(history_readback, HOST_READ)?;
        }
        if std::mem::take(&mut self.inject_failure) {
            let reached = self.failure_reached.clone();
            graph
                .add_pass("S04 injected recording failure")
                .with_side_effect()
                .render_with_context(move |_| {
                    reached.store(true, Ordering::Relaxed);
                    false
                });
        }
        Ok(graph.build())
    }

    pub fn finish_execution(
        &mut self,
        result: &rhapsody::Result<rhapsody::rg::frame_loop::FrameOutcome>,
    ) {
        if result.is_err()
            && let Some(history) = &mut self.temporal
        {
            history.previous_pixels = None;
        }
    }
    pub fn reset_history(
        &mut self,
        frame_loop: &mut FrameLoop,
        report: &mut SceneReport,
        reason: &str,
    ) -> rhapsody::Result<()> {
        let Some(history) = &mut self.temporal else {
            return Ok(());
        };
        frame_loop.graph.reset_temporal("window.history")?;
        history.previous_pixels = None;
        history.expected_pixels = None;
        report.history_reset_count += 1;
        if report.history_reset_reasons.len() < 32 {
            report.history_reset_reasons.push(reason.into());
        }
        Ok(())
    }
    pub fn inject_record_failure(&mut self) {
        self.failure_reached.store(false, Ordering::Relaxed);
        self.inject_failure = true;
    }
    pub fn expected_record_failure(&self, error: &(dyn std::error::Error + 'static)) -> bool {
        expected_injected_record_failure(self.failure_reached.load(Ordering::Relaxed), error)
    }
    pub fn mark_failed_frame(&mut self, report: &mut SceneReport) {
        report.injected_failed_frames += 1;
        self.recovery_pending = true;
    }
    /// Run only before building/recording a frame. Pending or rejected edits
    /// retain the existing pipeline, including its command-held Arc owners.
    pub fn advance_pipelines(
        &mut self,
        frame: u64,
        scripted: bool,
        repeat: bool,
        frame_limit: Option<u64>,
        report: &mut SceneReport,
    ) -> rhapsody::Result<()> {
        if scripted
            && let Some(round) = next_shader_round(
                self.reload_phase,
                self.reload_round,
                frame,
                repeat,
                frame_limit,
            )
        {
            self.reload_round = round;
            self.reload_phase = 0;
        }
        let round_start = self.reload_round.saturating_mul(300);
        if let Some(ticket) = self.reload_ticket.clone() {
            match ticket.poll() {
                PipelinePoll::Pending => {}
                PipelinePoll::Failed(error)
                    if self.reload_phase == 1
                        && matches!(error.as_ref(), rhapsody::Error::InvalidUsage { what, .. } if what == "invalid hot-reload SPIR-V") =>
                {
                    let old = self.postprocess.current();
                    self.postprocess.stage(ticket);
                    if self.postprocess.publish_at_frame_boundary()
                        || !Arc::ptr_eq(&old, &self.postprocess.current())
                    {
                        return Err(rhapsody::Error::invalid_usage(
                            "broken shader replaced the working pipeline",
                        ));
                    }
                    self.reload_ticket = None;
                    self.reload_phase = 2;
                    report.shader_rejected_edits += 1;
                }
                PipelinePoll::Failed(error) => return Err(compile_error(error)),
                PipelinePoll::Ready(_) => {
                    if self.reload_phase != 3 {
                        return Err(rhapsody::Error::invalid_usage(
                            "broken shader unexpectedly compiled",
                        ));
                    }
                    self.postprocess.stage(ticket);
                    if !self.postprocess.publish_at_frame_boundary() {
                        return Err(self.postprocess.last_error().cloned().map_or_else(
                            || rhapsody::Error::invalid_usage("ready shader was not published"),
                            compile_error,
                        ));
                    }
                    self.reload_ticket = None;
                    self.reload_phase = 4;
                    self.alternate_shader = !self.alternate_shader;
                    report.shader_completed_rounds += 1;
                    report.shader_frame_boundary_publications += 1;
                    report.shader_published_implementation_hash = Some(format!(
                        "{:016x}",
                        super::fixture::hash(&postprocess_shader_with(self.alternate_shader))
                    ));
                }
            }
        }
        if scripted && self.reload_phase == 0 && frame >= round_start.saturating_add(30) {
            self.reload_ticket = Some(self.compiler.request_compute_spirv(
                b"broken shader edit".to_vec(),
                c"main".to_owned(),
                self.postprocess_layout.clone(),
            )?);
            self.reload_phase = 1;
        }
        if scripted && self.reload_phase == 2 && frame >= round_start.saturating_add(60) {
            let package = BindlessShaderPackage::new(
                &self.bindless,
                &postprocess_program(!self.alternate_shader)?,
                64,
            )?;
            if package.abi != self.postprocess_abi {
                return Err(rhapsody::Error::invalid_usage(
                    "edited scene shader changed the logical parameter ABI",
                ));
            }
            self.reload_ticket = Some(self.compiler.request_compute_spirv(
                package.stages[0].spirv_bytes(),
                c"main".to_owned(),
                self.postprocess_layout.clone(),
            )?);
            self.reload_phase = 3;
        }
        let stats = self.compiler.stats();
        report
            .shader_initial_implementation_hash
            .get_or_insert_with(|| format!("{:016x}", super::fixture::hash(&postprocess_shader())));
        report.pipeline_compiled_requests = stats.compiled;
        report.pipeline_merged_requests = stats.merged;
        report.pipeline_peak_pending = stats.peak_pending;
        report.pipeline_peak_request_bytes = stats.peak_request_bytes;
        report.pipeline_cache_status =
            Some(format!("{:?}", self.pipeline_cache.status).to_lowercase());
        Ok(())
    }
    pub fn verify_pixels(
        &self,
        frame: u64,
        output: &[u8],
        reference: &mut [u8],
        report: &mut SceneReport,
    ) -> super::fixture::Result<()> {
        verify_raster_with(
            self.rendering.input_frame(frame),
            self.alternate_shader,
            output,
            reference,
            report,
        )?;
        if self.reload_phase == 1 || self.reload_phase == 2 || self.reload_phase == 3 {
            report.shader_preserved_frames += 1;
        }
        if self.alternate_shader {
            report.shader_reloaded_verified_frames += 1;
        }
        Ok(())
    }
    pub fn ensure_shader_script_completed(&self) -> super::fixture::Result<()> {
        if self.reload_phase != 4 {
            return Err(
                "shader script did not observe rejection and a valid frame-boundary replacement"
                    .into(),
            );
        }
        Ok(())
    }
    pub fn finish_minimum_run_shader_script(
        &mut self,
        report: &mut SceneReport,
    ) -> rhapsody::Result<()> {
        if !discardable_shader_round(self.reload_phase, report)? {
            return Ok(());
        }
        // Stop scheduling at the run boundary. Queued work is cancelled by the
        // compiler's normal protocol; active work is joined at teardown without
        // publishing or presenting another frame. Its real error still escapes.
        let cancelled = self.compiler.cancel_queued_before(u64::MAX) != 0;
        if let Some(ticket) = self.reload_ticket.take() {
            discarded_reload_result(
                self.reload_phase,
                cancelled,
                ticket.wait(std::time::Duration::from_secs(10)).map(|_| ()),
            )?;
        }
        report.shader_discarded_partial_rounds = 1;
        Ok(())
    }
    pub fn save_pipeline_cache(&self, report: &mut SceneReport) -> rhapsody::Result<()> {
        self.pipeline_cache.save()?;
        report.pipeline_cache_saved = true;
        Ok(())
    }
    pub fn verify_rendering(
        &mut self,
        frame: u64,
        output: &[u8],
        report: &mut SceneReport,
    ) -> super::fixture::Result<()> {
        {
            self.bindless.collect()?;
            let stats = self.bindless.stats();
            report.bindless_enabled = true;
            report.bindless_verified_frames += 1;
            report.bindless_capacity = stats.capacity;
            report.bindless_peak_live = report.bindless_peak_live.max(stats.peak_live);
            report.bindless_peak_retiring = report.bindless_peak_retiring.max(stats.peak_retiring);
        }
        report.dynamic_rendering_frames += 1;
        report.attachment_cache_views = self
            .attachments
            .lock()
            .expect("attachment cache")
            .view_count();
        let _ = (frame, output);
        Ok(())
    }
    pub fn verify_history(
        &mut self,
        output: &[u8],
        report: &mut SceneReport,
    ) -> super::fixture::Result<()> {
        let Some(history) = &mut self.temporal else {
            return Ok(());
        };
        let mut pixels = vec![0; output.len()];
        unsafe {
            history
                .readback
                .read_bytes_after_completion(0, &mut pixels)?;
        }
        if Some(&pixels) != history.expected_pixels.as_ref() {
            return Err("S04 GPU previous history differs from the preceding verified GPU frame or reset clear".into());
        }
        report.history_verified_frames += 1;
        if history.initialized {
            report.history_initializations += 1;
        }
        history.previous_pixels = Some(output.to_vec());
        if std::mem::take(&mut self.recovery_pending) {
            report.failure_recovery_verified_frames += 1;
        }
        Ok(())
    }
}

fn scene_program(
    input: &[(ShaderStage, &str, Vec<u8>)],
) -> rhapsody::Result<CompiledShaderPipeline> {
    use rspirv::{
        binary::Assemble,
        dr::{Instruction, Operand},
        spirv::Op,
    };
    let stages = input
        .iter()
        .map(|(stage, entry, bytes)| {
            let mut module = rspirv::dr::load_bytes(bytes).map_err(|error| {
                rhapsody::Error::invalid_usage(format!("scene shader: {error}"))
            })?;
            // Fixture modules omit debug names. Reflection needs stable logical names;
            // adding OpName changes neither driver instructions nor resource bindings.
            let named = module
                .debug_names
                .iter()
                .filter_map(|instruction| match instruction.operands.first() {
                    Some(Operand::IdRef(id)) => Some(*id),
                    _ => None,
                })
                .collect::<std::collections::BTreeSet<_>>();
            for id in module
                .types_global_values
                .iter()
                .filter(|i| i.class.opcode == Op::Variable)
                .filter_map(|i| i.result_id)
                .chain(
                    module
                        .functions
                        .iter()
                        .filter_map(|function| function.def.as_ref().and_then(|i| i.result_id)),
                )
            {
                if !named.contains(&id) {
                    module.debug_names.push(Instruction::new(
                        Op::Name,
                        None,
                        None,
                        vec![
                            Operand::IdRef(id),
                            Operand::LiteralString(format!("scene_{id}")),
                        ],
                    ));
                }
            }
            Ok(CompiledShaderStage {
                stage: *stage,
                entry_point: (*entry).into(),
                spirv: module.assemble(),
            })
        })
        .collect::<rhapsody::Result<Vec<_>>>()?;
    let reflection = shader_compiler::reflect::generate_reflection(&stages).map_err(|error| {
        rhapsody::Error::invalid_usage(format!("scene shader reflection: {error}"))
    })?;
    Ok(CompiledShaderPipeline {
        stages,
        reflection,
        source_dependencies: vec![],
        trailing_bytes: vec![],
    })
}
fn postprocess_program(alternate: bool) -> rhapsody::Result<CompiledShaderPipeline> {
    scene_program(&[(
        ShaderStage::Compute,
        "main",
        postprocess_shader_with(alternate),
    )])
}

fn compile_error(error: Arc<rhapsody::Error>) -> rhapsody::Error {
    use rhapsody::Error;
    match error.as_ref() {
        Error::DeviceLost => Error::DeviceLost,
        Error::OutOfDeviceMemory => Error::OutOfDeviceMemory,
        Error::OutOfHostMemory => Error::OutOfHostMemory,
        Error::SurfaceOutOfDate => Error::SurfaceOutOfDate,
        Error::SurfaceLost => Error::SurfaceLost,
        Error::Timeout => Error::Timeout,
        Error::Unsupported(reason) => Error::Unsupported(reason.clone()),
        Error::Vulkan(result) => Error::Vulkan(*result),
        _ => Error::invalid_usage_with("background pipeline creation", error),
    }
}

fn expected_injected_record_failure(
    reached: bool,
    error: &(dyn std::error::Error + 'static),
) -> bool {
    reached
        && matches!(error.downcast_ref::<rhapsody::Error>(), Some(rhapsody::Error::InvalidUsage { source: Some(source), .. })
        if matches!(source.downcast_ref::<rhapsody::rg::executor::Error>(), Some(rhapsody::rg::executor::Error::Record(outcome)) if outcome.panic))
}

/// Exact channel permutation avoids implementation-dependent float rounding.
/// local_size=(8,4) divides the fixture extent, so no invocation is out of bounds.
fn postprocess_shader() -> Vec<u8> {
    postprocess_shader_with(false)
}
fn postprocess_shader_with(alternate: bool) -> Vec<u8> {
    postprocess_shader_logical(alternate)
}
fn postprocess_shader_logical(alternate: bool) -> Vec<u8> {
    use rspirv::{
        binary::Assemble,
        dr::{Builder, Operand},
        spirv::*,
    };
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(Capability::Shader);
    b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = b.type_void();
    let fn_ty = b.type_function(void, vec![]);
    let uint = b.type_int(32, 0);
    let uvec2 = b.type_vector(uint, 2);
    let uvec3 = b.type_vector(uint, 3);
    let float = b.type_float(32);
    let vec4 = b.type_vector(float, 4);
    let image_ty = b.type_image(float, Dim::Dim2D, 0, 0, 0, 2, ImageFormat::Rgba8, None);
    let image_ptr = b.type_pointer(None, StorageClass::UniformConstant, image_ty);
    let input_ptr = b.type_pointer(None, StorageClass::Input, uvec3);
    let source = b.variable(image_ptr, None, StorageClass::UniformConstant, None);
    let target = b.variable(image_ptr, None, StorageClass::UniformConstant, None);
    for (binding, image, name) in [(0, source, "source"), (1, target, "target")] {
        b.name(image, name);
        b.decorate(image, Decoration::DescriptorSet, [Operand::LiteralInt32(0)]);
        b.decorate(image, Decoration::Binding, [Operand::LiteralInt32(binding)]);
    }
    b.decorate(source, Decoration::NonWritable, []);
    b.decorate(target, Decoration::NonReadable, []);
    let global = b.variable(input_ptr, None, StorageClass::Input, None);
    b.decorate(
        global,
        Decoration::BuiltIn,
        [Operand::BuiltIn(BuiltIn::GlobalInvocationId)],
    );
    let main = b
        .begin_function(void, None, FunctionControl::NONE, fn_ty)
        .unwrap();
    b.begin_block(None).unwrap();
    let id = b.load(uvec3, None, global, None, []).unwrap();
    let xy = b.vector_shuffle(uvec2, None, id, id, [0, 1]).unwrap();
    let image = b.load(image_ty, None, source, None, []).unwrap();
    let pixel = b.image_read(vec4, None, image, xy, None, []).unwrap();
    let transformed = b
        .vector_shuffle(
            vec4,
            None,
            pixel,
            pixel,
            if alternate {
                [1, 2, 0, 3]
            } else {
                [2, 0, 1, 3]
            },
        )
        .unwrap();
    let image = b.load(image_ty, None, target, None, []).unwrap();
    b.image_write(image, xy, transformed, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::GLCompute, main, "main", [global]);
    b.execution_mode(main, ExecutionMode::LocalSize, [8, 4, 1]);
    b.module()
        .assemble()
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect()
}

#[cfg(test)]
pub fn verify_raster(
    frame: u64,
    output: &[u8],
    reference: &mut [u8],
    report: &mut SceneReport,
) -> super::fixture::Result<()> {
    verify_raster_with(frame, false, output, reference, report)
}
fn verify_raster_with(
    frame: u64,
    alternate: bool,
    output: &[u8],
    reference: &mut [u8],
    report: &mut SceneReport,
) -> super::fixture::Result<()> {
    let coverage = raster_coverage(frame);
    let transform = |p: [u8; 4]| {
        if alternate {
            [p[1], p[2], p[0], p[3]]
        } else {
            [p[2], p[0], p[1], p[3]]
        }
    };
    let fill = transform(draw_support::FILL);
    let clear = transform(draw_support::frame_clear_color(frame as usize));
    let mut mismatches = 0;
    let mut boundary = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let flags = coverage[(y * WIDTH + x) as usize];
            let inside = flags & 1 != 0;
            let near_boundary = flags & 2 != 0;
            let index = ((y * WIDTH + x) * 4) as usize;
            let expected = if inside { fill } else { clear };
            reference[index..index + 4].copy_from_slice(&expected);
            let actual = &output[index..index + 4];
            if actual != fill && actual != clear {
                return Err(
                    format!("S03 invalid GPU postprocess color at ({x},{y}): {actual:?}").into(),
                );
            }
            if near_boundary {
                boundary += 1;
            }
            if actual != expected {
                mismatches += 1;
                if !near_boundary {
                    return Err(format!(
                        "S03 CPU coverage mismatch outside rasterization boundary at ({x},{y})"
                    )
                    .into());
                }
            }
        }
    }
    report.rasterized_frames += 1;
    report.postprocessed_frames += 1;
    report.reference_verified_frames += 1;
    report.worst_reference_mismatches = report.worst_reference_mismatches.max(mismatches);
    report.worst_boundary_pixels = report.worst_boundary_pixels.max(boundary);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_lowering_uses_shared_heap_and_one_parameter_index() {
        for alternate in [false, true] {
            let input = postprocess_program(alternate).unwrap();
            let (abi, stages) =
                shader_compiler::bindless::lower_pipeline_with_limit(&input, 64).unwrap();
            assert_eq!(abi.original_push_bytes, 0);
            assert_eq!(abi.bindings.len(), 2);
            assert_eq!(abi.parameter_bytes, 8);
            let entries = rhapsody::raw::shader::reflect_spirv_entries(&stages[0].spirv).unwrap();
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].push_constants.len(), 1);
            assert!(
                entries[0]
                    .descriptors
                    .iter()
                    .all(|binding| binding.set == 0)
            );
            let module = rspirv::dr::load_words(&stages[0].spirv).unwrap();
            assert!(
                module
                    .all_inst_iter()
                    .any(|i| i.class.opcode == rspirv::spirv::Op::TypeRuntimeArray)
            );
        }
        let (vertex, fragment) = draw_support::shaders();
        let raster = scene_program(&[
            (ShaderStage::Vertex, "vs_main", vertex),
            (ShaderStage::Fragment, "ps_main", fragment),
        ])
        .unwrap();
        let (abi, stages) =
            shader_compiler::bindless::lower_pipeline_with_limit(&raster, 64).unwrap();
        assert!(abi.bindings.is_empty());
        assert_eq!(abi.parameter_bytes, 4);
        assert_eq!(stages.len(), 2);
    }

    #[test]
    fn only_dynamic_rendering_is_accepted() {
        for choice in [RenderingChoice::Auto, RenderingChoice::Dynamic] {
            assert_eq!(
                choice.dynamic(false, 0).unwrap_err().kind(),
                rhapsody::ErrorKind::Unsupported
            );
            assert!(choice.dynamic(true, 0).unwrap());
            assert_eq!(choice.input_frame(7), 7);
        }
        assert!(RenderingChoice::parse("legacy").is_err());
        assert!(RenderingChoice::parse("compare").is_err());
    }

    #[test]
    fn compute_shader_reads_and_writes_images_with_the_expected_channel_transform() {
        let bytes = postprocess_shader();
        let module = rspirv::dr::load_bytes(&bytes).expect("assembled SPIR-V");
        let instructions = module.all_inst_iter().collect::<Vec<_>>();
        assert_eq!(
            instructions
                .iter()
                .filter(|i| i.class.opcode == rspirv::spirv::Op::ImageRead)
                .count(),
            1
        );
        assert_eq!(
            instructions
                .iter()
                .filter(|i| i.class.opcode == rspirv::spirv::Op::ImageWrite)
                .count(),
            1
        );
        assert!(
            instructions
                .iter()
                .any(|i| i.class.opcode == rspirv::spirv::Op::VectorShuffle
                    && i.operands.ends_with(&[
                        rspirv::dr::Operand::LiteralInt32(2),
                        rspirv::dr::Operand::LiteralInt32(0),
                        rspirv::dr::Operand::LiteralInt32(1),
                        rspirv::dr::Operand::LiteralInt32(3)
                    ]))
        );
        if let Some(path) = std::env::var_os("RHAPSODY_SCENE_SHADER_DUMP") {
            std::fs::write(path, bytes).unwrap();
        }
    }

    #[test]
    fn reference_accepts_expected_geometry_and_rejects_blank_or_wrong_postprocess_output() {
        let mut pixels = vec![0; (WIDTH * HEIGHT * 4) as usize];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let y = y as f32 + 0.5;
                let x_center = x as f32 + 0.5;
                let left = 80.0 + (y - 45.0) * (80.0 / 90.0);
                let right = 240.0 - (y - 45.0) * (80.0 / 90.0);
                let inside = (45.0..135.0).contains(&y) && x_center >= left && x_center <= right;
                let offset = ((y.floor() as u32 * WIDTH + x) * 4) as usize;
                pixels[offset..offset + 4].copy_from_slice(if inside {
                    &[0, 255, 0, 255]
                } else {
                    &[255, 0, 0, 255]
                });
            }
        }
        let mut reference = vec![0; pixels.len()];
        let mut report = SceneReport::default();
        verify_raster(0, &pixels, &mut reference, &mut report).unwrap();
        assert_eq!(report.reference_verified_frames, 1);
        assert_eq!(report.worst_reference_mismatches, 0);
        let alternate = pixels
            .as_chunks::<4>()
            .0
            .iter()
            .flat_map(|p| [p[2], p[0], p[1], p[3]])
            .collect::<Vec<_>>();
        verify_raster_with(0, true, &alternate, &mut reference, &mut report).unwrap();
        assert!(verify_raster_with(0, false, &alternate, &mut reference, &mut report).is_err());
        let center = ((HEIGHT / 2 * WIDTH + WIDTH / 2) * 4) as usize;
        pixels[center..center + 4].copy_from_slice(&[255, 0, 0, 255]);
        assert!(verify_raster(0, &pixels, &mut reference, &mut report).is_err());
        pixels[center..center + 4].copy_from_slice(&[0, 0, 255, 255]);
        assert!(verify_raster(0, &pixels, &mut reference, &mut report).is_err());
    }

    #[test]
    fn failure_script_only_accepts_the_reached_injected_recording_error() {
        let injected = rhapsody::Error::from(rhapsody::rg::executor::Error::Record(
            rhapsody::rg::recorder::Outcome {
                panic: true,
                ..Default::default()
            },
        ));
        assert!(expected_injected_record_failure(true, &injected));
        assert!(!expected_injected_record_failure(false, &injected));
        for other in [
            rhapsody::Error::DeviceLost,
            rhapsody::Error::OutOfHostMemory,
            rhapsody::Error::invalid_usage("unrelated validation failure"),
        ] {
            assert!(!expected_injected_record_failure(true, &other));
        }
    }

    #[test]
    fn async_compile_errors_keep_actionable_device_and_memory_categories() {
        for error in [
            rhapsody::Error::DeviceLost,
            rhapsody::Error::OutOfDeviceMemory,
            rhapsody::Error::OutOfHostMemory,
            rhapsody::Error::Timeout,
            rhapsody::Error::Unsupported("feature".into()),
            rhapsody::Error::Vulkan(vk::Result::ERROR_UNKNOWN),
        ] {
            let kind = error.kind();
            assert_eq!(compile_error(Arc::new(error)).kind(), kind);
        }
        assert_ne!(postprocess_shader(), postprocess_shader_with(true));
        let changed = rspirv::dr::load_bytes(postprocess_shader_with(true)).unwrap();
        assert!(changed.all_inst_iter().any(|i| i.class.opcode
            == rspirv::spirv::Op::VectorShuffle
            && i.operands.ends_with(&[
                rspirv::dr::Operand::LiteralInt32(1),
                rspirv::dr::Operand::LiteralInt32(2),
                rspirv::dr::Operand::LiteralInt32(0),
                rspirv::dr::Operand::LiteralInt32(3)
            ])));
    }
}

#[cfg(test)]
mod repeated_shader_script_tests {
    use super::{
        SceneReport, discardable_shader_round, discarded_reload_result, next_shader_round,
    };
    use std::sync::Arc;
    #[test]
    fn minimum_run_discards_only_a_trailing_round_after_verified_publications() {
        let mut report = SceneReport {
            shader_completed_rounds: 1,
            shader_rejected_edits: 1,
            shader_frame_boundary_publications: 1,
            shader_reloaded_verified_frames: 1,
            reference_verified_frames: 100,
            ..Default::default()
        };
        for phase in 0..4 {
            assert!(discardable_shader_round(phase, &report).unwrap());
        }
        assert!(!discardable_shader_round(4, &report).unwrap());
        report.shader_completed_rounds = 0;
        assert!(discardable_shader_round(1, &report).is_err());
        report.shader_completed_rounds = 1;
        report.reference_verified_frames = 0;
        assert!(discardable_shader_round(3, &report).is_err());
        report.reference_verified_frames = 100;
        report.worst_reference_mismatches = 1;
        assert!(discardable_shader_round(3, &report).is_err());
    }
    #[test]
    fn discarded_reload_never_swallows_real_device_or_memory_failures() {
        assert!(
            discarded_reload_result(
                1,
                false,
                Err(Arc::new(rhapsody::Error::invalid_usage(
                    "invalid hot-reload SPIR-V"
                )))
            )
            .is_ok()
        );
        assert!(discarded_reload_result(3, false, Ok(())).is_ok());
        assert!(discarded_reload_result(1, false, Ok(())).is_err());
        assert!(discarded_reload_result(3, true, Err(Arc::new(rhapsody::Error::Timeout))).is_ok());
        assert!(
            discarded_reload_result(3, false, Err(Arc::new(rhapsody::Error::Timeout))).is_err()
        );
        for error in [
            rhapsody::Error::DeviceLost,
            rhapsody::Error::OutOfHostMemory,
            rhapsody::Error::OutOfDeviceMemory,
        ] {
            let kind = error.kind();
            assert_eq!(
                discarded_reload_result(3, true, Err(Arc::new(error)))
                    .unwrap_err()
                    .kind(),
                kind
            );
        }
    }
    #[test]
    fn repeated_shader_rounds_wait_for_completion_and_advance_by_period() {
        for phase in 0..4 {
            assert_eq!(next_shader_round(phase, 0, 300, true, None), None);
        }
        assert_eq!(next_shader_round(4, 0, 299, true, None), None);
        assert_eq!(next_shader_round(4, 0, 300, true, None), Some(1));
        assert_eq!(next_shader_round(4, 1, 600, true, None), Some(2));
        assert_eq!(next_shader_round(4, 0, 980, true, None), Some(3));
        assert_eq!(next_shader_round(4, 0, 300, false, None), None);
    }
    #[test]
    fn final_frames_leave_room_for_async_replacement_and_pixel_verification() {
        assert_eq!(next_shader_round(4, 0, 300, true, Some(419)), None);
        assert_eq!(next_shader_round(4, 0, 300, true, Some(420)), Some(1));
        assert_eq!(next_shader_round(4, 0, 300, true, Some(360)), None);
        assert_eq!(next_shader_round(4, 1, 600, true, Some(600)), None);
    }
}

// Triangular grid preserves interpolated color and the independent coverage
// reference while exercising 64 rotating primitives instead of one triangle.
fn subdivide_triangle(v: [[f32; 4]; 3]) -> Vec<[f32; 4]> {
    let point = |i: u32, j: u32| {
        std::array::from_fn(|k| {
            v[0][k] + (v[1][k] - v[0][k]) * i as f32 / 8.0 + (v[2][k] - v[0][k]) * j as f32 / 8.0
        })
    };
    let mut out = Vec::with_capacity(192);
    for i in 0..8 {
        for j in 0..8 - i {
            out.extend([point(i, j), point(i + 1, j), point(i, j + 1)]);
            if i + j < 7 {
                out.extend([point(i + 1, j), point(i + 1, j + 1), point(i, j + 1)]);
            }
        }
    }
    out
}

// Geometry repeats every 24 angles. Cache only the independent CPU coverage,
// never GPU pixels; every frame still compares all actual output pixels.
fn raster_coverage(frame: u64) -> &'static [u8] {
    static CACHE: [std::sync::OnceLock<Vec<u8>>; 24] = [const { std::sync::OnceLock::new() }; 24];
    CACHE[(frame % 24) as usize].get_or_init(|| {
        let triangle = draw_support::rotated_triangle((frame % 24) as f32 * 15.0_f32.to_radians());
        let points = triangle.map(|v| {
            [
                (v[0] + 1.0) * 0.5 * WIDTH as f32,
                (v[1] + 1.0) * 0.5 * HEIGHT as f32,
            ]
        });
        let edge = |a: [f32; 2], z: [f32; 2], p: [f32; 2]| {
            (z[0] - a[0]) * (p[1] - a[1]) - (z[1] - a[1]) * (p[0] - a[0])
        };
        let area = edge(points[0], points[1], points[2]);
        let mut coverage = vec![0; (WIDTH * HEIGHT) as usize];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let p = [x as f32 + 0.5, y as f32 + 0.5];
                let edges = [
                    (points[0], points[1]),
                    (points[1], points[2]),
                    (points[2], points[0]),
                ];
                let inside = edges.iter().all(|&(a, z)| edge(a, z, p) * area >= 0.0);
                let near_boundary = edges.iter().any(|&(a, z)| {
                    let d = [z[0] - a[0], z[1] - a[1]];
                    let t = (((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1])
                        / (d[0] * d[0] + d[1] * d[1]))
                        .clamp(0.0, 1.0);
                    (p[0] - a[0] - t * d[0]).powi(2) + (p[1] - a[1] - t * d[1]).powi(2)
                        <= 0.75 * 0.75
                });
                coverage[(y * WIDTH + x) as usize] =
                    u8::from(inside) | (u8::from(near_boundary) << 1);
            }
        }
        coverage
    })
}
