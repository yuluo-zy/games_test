//! Bounded offscreen text rendering over the shared rhapsody resource and pipeline APIs.
mod frame;
mod input;
use crate::renderer::{AtlasDescriptor, PendingGlyphUpload, RenderBatch};
use crate::validation::{RenderLimits, validate_batch};
use ash::vk;
use input::{Globals, PackedGlyphUploads};
use rhapsody::{
    bindless::BindlessConfig, bindless::BindlessHeap, bindless::BindlessLease,
    bindless::BindlessParameters, bindless::BindlessResource, bindless::BindlessShaderAbi,
    bindless::BindlessShaderPackage, command::VulkanCommandPool, command::buffer_access,
    command::image_access, command::record_texture_regions, command::record_texture_zero_fill,
    device::Device, pipeline::AsyncPipelineCompiler, pipeline::CompilerConfig,
    pipeline::CompilerStats, pipeline::DynamicRenderingFormats, pipeline::PersistentPipelineCache,
    pipeline::PipelineCache, pipeline::PipelineTicket, pipeline::RasterPipelineDesc,
    pipeline::ShaderModule, pipeline::get_vk_blend_mode, resource::Buffer, resource::BufferDesc,
    resource::DynamicConstants, resource::OwnershipRange, resource::Sampler, resource::Texture,
    resource::TextureDesc, resource::TextureViewDesc, resource::range_state::AccessScope,
    rg::attachments::Attachment, rg::attachments::AttachmentCache,
    rg::attachments::RenderingPolicy, rg::pass_api::AshCommandBackend, rg::pass_api::RenderPassApi,
};
use shader_pipeline::{
    compiled::CompiledShaderPipeline, compiled::ShaderStage, reflection::BindingKind,
    reflection::BindingLocation, reflection::ShaderReflection,
};
use std::{ffi::CString, path::PathBuf, sync::Arc, time::Duration};
/// GPU operations preserve CPU validation and rhapsody errors as their original error objects.
pub type GpuResult<T> = Result<T, Box<dyn std::error::Error>>;
/// Per-frame draw/upload totals and cumulative snapshots of this adapter's resource counters.
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuTextStats {
    pub primitives: usize,
    pub upload_regions: usize,
    pub upload_bytes: usize,
    pub draw_calls: u32,
    pub submits: u32,
    /// Completed pool resets since adapter creation.
    pub command_pool_resets: u64,
    /// Pipeline requests actually compiled by this adapter's compiler.
    pub compiled_pipeline_requests: u64,
    pub dynamic_rendering: bool,
    /// Live slots in the heap, including other consumers when the heap is shared.
    pub bindless_live_slots: usize,
    /// Cumulative parameter-storage backing page creations.
    pub dynamic_constant_page_creations: u64,
    /// Cumulative staging/readback buffer creations; warm frames reuse these.
    pub transfer_buffer_creations: u64,
}
/// Completed offscreen image. `rgba` holds tightly packed, top-to-bottom RGBA8 pixels.
/// No submission is left pending when this value is returned.
pub struct RenderedText {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub stats: GpuTextStats,
}
#[derive(Clone, Copy)]
struct TextBindings {
    globals: BindingLocation,
    primitives: BindingLocation,
    shape: BindingLocation,
    atlas: BindingLocation,
    blue: BindingLocation,
    images: BindingLocation,
    samplers: [BindingLocation; 2],
}
impl TextBindings {
    fn resolve(reflection: &ShaderReflection) -> GpuResult<Self> {
        let sets = reflection.effective_descriptor_sets();
        let slot = |name: &str, kind: BindingKind| -> GpuResult<BindingLocation> {
            let location = reflection.binding_by_name(name).ok_or_else(|| {
                rhapsody::Error::invalid_usage(format!("text shader binding absent: {name}"))
            })?;
            let actual = sets
                .get(location.set as usize)
                .and_then(|set| set.get(&location.binding))
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(format!(
                        "text shader binding location absent: {name}"
                    ))
                })?;
            if actual.kind != kind {
                return Err(rhapsody::Error::invalid_usage(format!(
                    "text shader binding kind differs: {name}"
                ))
                .into());
            }
            Ok(location)
        };
        Ok(Self {
            globals: slot("$Globals", BindingKind::UniformBuffer)?,
            primitives: slot("primitives", BindingKind::StorageBuffer)?,
            shape: slot("shape_stream", BindingKind::StorageBuffer)?,
            atlas: slot("font_atlas_tex", BindingKind::SampledImage)?,
            blue: slot("blue_noise_texture", BindingKind::SampledImage)?,
            images: slot("bindless_textures_dyn", BindingKind::SampledImage)?,
            samplers: [
                slot("sampler_lnc", BindingKind::Sampler)?,
                slot("sampler_llc", BindingKind::Sampler)?,
            ],
        })
    }
}
/// Startup and per-frame limits for an offscreen adapter.
/// Constructors reject zero budgets, resolver capacity outside `1..=65536`,
/// and completion timeouts outside `(0, 300 seconds]`.
#[derive(Clone, Debug)]
pub struct GpuTextConfig {
    /// Optional device-specific pipeline cache file, read and saved during startup.
    pub pipeline_cache_path: Option<PathBuf>,
    /// Capacity used when lowering the shader’s runtime sampled-image array.
    pub max_resolver_images: u32,
    /// CPU validation and GPU allocation budgets for each submission.
    pub limits: RenderLimits,
    /// Bounded wait after submission; timeout does not imply GPU completion.
    pub completion_timeout: Duration,
}
impl Default for GpuTextConfig {
    fn default() -> Self {
        Self {
            pipeline_cache_path: None,
            max_resolver_images: 64,
            limits: RenderLimits::default(),
            completion_timeout: Duration::from_secs(30),
        }
    }
}
/// Serial offscreen renderer with reusable atlas, target, descriptors and mapped buffers.
/// External sampled textures remain caller-owned; submission methods state their
/// immutability contract. A timed-out submission retains its resources until completion.
pub struct GpuTextRenderer {
    device: Arc<Device>,
    heap: Arc<BindlessHeap>,
    abi: Arc<BindlessShaderAbi>,
    sampler_leases: Vec<(BindingLocation, Arc<BindlessLease>)>,
    parameters: DynamicConstants,
    bindings: TextBindings,
    compiler: AsyncPipelineCompiler,
    pipeline: PipelineTicket,
    disk_cache: Option<PersistentPipelineCache>,
    command_pool: Arc<VulkanCommandPool>,
    attachments: AttachmentCache,
    blue_noise: Arc<Texture>,
    atlas: Option<(AtlasDescriptor, Arc<Texture>)>,
    buffers: frame::InputBufferCache,
    target: Option<([u32; 2], Arc<Texture>)>,
    staging: Option<Arc<Buffer>>,
    readback: Option<Arc<Buffer>>,
    transfer_buffer_creations: u64,
    limits: RenderLimits,
    completion_timeout: Duration,
}
fn sampled_2d(texture: &Texture) -> bool {
    texture.desc.image_type() == vk::ImageType::TYPE_2D
        && matches!(texture.desc.array_layers(), Ok(1))
        && texture.desc.samples() == vk::SampleCountFlags::TYPE_1
        && TextureViewDesc::new(&texture.desc)
            .is_ok_and(|view| view.view_type == vk::ImageViewType::TYPE_2D)
        && texture.desc.usage.contains(vk::ImageUsageFlags::SAMPLED)
        && rhapsody::resource::vk_format_info(texture.desc.format)
            .is_some_and(|f| f.aspect == vk::ImageAspectFlags::COLOR)
}
impl GpuTextRenderer {
    /// Create an adapter with default budgets and a private bindless heap.
    /// Shader bytes must use the text globals/binding/primitive ABI; blue-noise must
    /// be a sampled, single-layer 2D color texture on `device`.
    ///
    /// # Errors
    /// Returns shader/ABI, device capability, allocation or startup compilation errors.
    pub fn new(
        device: Arc<Device>,
        compiled_bytes: &[u8],
        blue_noise: Arc<Texture>,
    ) -> GpuResult<Self> {
        Self::new_with_config(device, compiled_bytes, blue_noise, GpuTextConfig::default())
    }
    /// Create with explicit budgets and optional disk cache using a private heap.
    /// In addition to [`Self::new`] errors, invalid budgets and cache I/O fail construction.
    pub fn new_with_config(
        device: Arc<Device>,
        compiled_bytes: &[u8],
        blue_noise: Arc<Texture>,
        config: GpuTextConfig,
    ) -> GpuResult<Self> {
        let heap = BindlessHeap::new(device.clone(), BindlessConfig::default())?;
        Self::new_with_heap(device, compiled_bytes, blue_noise, heap, config)
    }
    /// Share an existing bindless heap across text and scene pipelines.
    /// The heap and blue-noise texture must belong to `device`; the shader ABI
    /// and configuration are validated before pipeline creation. The pipeline ticket
    /// wait has a five-second timeout; dropping its compiler may still wait for an
    /// in-progress driver compilation. Errors include device/ABI mismatch, invalid
    /// budgets, allocation, compilation and disk-cache I/O failures.
    pub fn new_with_heap(
        device: Arc<Device>,
        compiled_bytes: &[u8],
        blue_noise: Arc<Texture>,
        heap: Arc<BindlessHeap>,
        config: GpuTextConfig,
    ) -> GpuResult<Self> {
        if config.max_resolver_images == 0
            || config.max_resolver_images > 65_536
            || config.completion_timeout.is_zero()
            || config.completion_timeout > Duration::from_secs(300)
            || config.limits.max_target_bytes == 0
            || config.limits.max_primitives == 0
            || config.limits.max_shape_words == 0
            || config.limits.max_upload_bytes == 0
            || config.limits.max_upload_regions == 0
            || config.limits.max_atlas_bytes == 0
            || config.limits.max_coverage_checks == 0
        {
            return Err("invalid text GPU budgets, resolver capacity or completion timeout".into());
        }
        if !Arc::ptr_eq(&device, &heap.layout.device) {
            return Err(
                rhapsody::Error::invalid_usage("text heap belongs to another device").into(),
            );
        }
        if !Arc::ptr_eq(&device, &blue_noise.device) || !sampled_2d(&blue_noise) {
            return Err("blue-noise texture must be sampled and use this device".into());
        }
        let compiled = CompiledShaderPipeline::decode(compiled_bytes)?;
        let bindings = TextBindings::resolve(&compiled.reflection)?;
        Globals::validate_layout(&compiled.reflection, bindings.globals)?;
        let package = BindlessShaderPackage::new(&heap, &compiled, config.max_resolver_images)?;
        let mut sampler_leases = Vec::with_capacity(2);
        for (location, name) in bindings
            .samplers
            .into_iter()
            .zip(["sampler_lnc", "sampler_llc"])
        {
            let sampler = Arc::new(Sampler::new(
                device.clone(),
                &vk::SamplerCreateInfo::default()
                    .mag_filter(vk::Filter::LINEAR)
                    .min_filter(vk::Filter::LINEAR)
                    .mipmap_mode(if name == "sampler_lnc" {
                        vk::SamplerMipmapMode::NEAREST
                    } else {
                        vk::SamplerMipmapMode::LINEAR
                    })
                    .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE),
            )?);
            sampler_leases.push((location, heap.sampler(sampler)?));
        }
        let vertex = package
            .stages
            .iter()
            .find(|stage| stage.stage == ShaderStage::Vertex)
            .ok_or_else(|| rhapsody::Error::invalid_usage("text vertex stage is absent"))?;
        let fragment = package
            .stages
            .iter()
            .find(|stage| stage.stage == ShaderStage::Fragment)
            .ok_or_else(|| rhapsody::Error::invalid_usage("text fragment stage is absent"))?;
        let vertex_module = ShaderModule::from_spirv(device.clone(), &vertex.spirv_bytes())?;
        let fragment_module = ShaderModule::from_spirv(device.clone(), &fragment.spirv_bytes())?;
        let disk_cache = config
            .pipeline_cache_path
            .map(|path| PersistentPipelineCache::open(device.clone(), path, 32))
            .transpose()?;
        let cache = if let Some(cache) = &disk_cache {
            cache.cache.clone()
        } else {
            Arc::new(PipelineCache::new(device.clone(), &[])?)
        };
        let compiler = AsyncPipelineCompiler::new(
            cache,
            CompilerConfig {
                workers: 1,
                max_pending: 8,
                max_request_bytes: 1 << 20,
                max_cached_requests: 32,
            },
        )?;
        let mut state = RasterPipelineDesc::new(vertex_module, package.layout.clone());
        state.fragment = Some(fragment_module);
        state.fragment_entry = Some(CString::new(fragment.entry_point.as_str())?);
        state.use_dynamic_rendering(DynamicRenderingFormats::color(vk::Format::R8G8B8A8_UNORM));
        state.entry = CString::new(vertex.entry_point.as_str())?;
        state.depth_test = false;
        state.depth_write = false;
        state.cull_mode = vk::CullModeFlags::NONE;
        state.topology = vk::PrimitiveTopology::TRIANGLE_LIST;
        const TEXT_RASTER_BLEND_FLAGS: u8 = 64;
        state.blend_attachments = vec![get_vk_blend_mode(TEXT_RASTER_BLEND_FLAGS)];
        let pipeline = compiler.request_raster(state)?;
        // Fixed output format and samples are known at construction. Compile
        // before any frame is recorded, never wait inside render_offscreen.
        pipeline
            .wait(Duration::from_secs(5))
            .map_err(rhapsody::Error::from)?;
        if let Some(cache) = &disk_cache {
            cache.save()?;
        }
        let command_pool = VulkanCommandPool::new(device.clone())?;
        let parameters = DynamicConstants::new(device.clone())?;
        Ok(Self {
            attachments: AttachmentCache::new(device.clone())?,
            device,
            heap,
            abi: package.abi,
            sampler_leases,
            parameters,
            bindings,
            compiler,
            pipeline,
            disk_cache,
            command_pool,
            blue_noise,
            atlas: None,
            buffers: frame::InputBufferCache::default(),
            target: None,
            staging: None,
            readback: None,
            transfer_buffer_creations: 0,
            limits: config.limits,
            completion_timeout: config.completion_timeout,
        })
    }
    /// Snapshot compiler counters without requesting or waiting for compilation.
    pub fn pipeline_stats(&self) -> CompilerStats {
        self.compiler.stats()
    }
    /// Persist the configured disk cache; no configured cache is a successful no-op.
    /// File/driver cache-export failures propagate to the caller.
    pub fn save_pipeline_cache(&self) -> GpuResult<()> {
        if let Some(cache) = &self.disk_cache {
            cache.save()?;
        }
        Ok(())
    }
    fn transfer_buffer(&mut self, size: u64, readback: bool) -> GpuResult<Arc<Buffer>> {
        let slot = if readback {
            &mut self.readback
        } else {
            &mut self.staging
        };
        if slot.as_ref().is_none_or(|buffer| buffer.size < size) {
            let capacity = size
                .max(16)
                .checked_next_power_of_two()
                .ok_or("transfer capacity overflow")?;
            let desc = if readback {
                BufferDesc::new_gpu_to_cpu(capacity, vk::BufferUsageFlags::TRANSFER_DST)
            } else {
                BufferDesc::new_cpu_to_gpu(capacity, vk::BufferUsageFlags::TRANSFER_SRC)
            };
            *slot = Some(Arc::new(Buffer::new(self.device.clone(), &desc)?));
            self.transfer_buffer_creations = self.transfer_buffer_creations.saturating_add(1);
        }
        slot.as_ref()
            .cloned()
            .ok_or_else(|| "missing text transfer buffer".into())
    }

    /// Draw a CPU-produced batch using its owned glyph uploads and return RGBA8 pixels.
    /// Target dimensions must be nonzero and within configured/device limits.
    /// Errors and resource lifetimes follow [`Self::render_offscreen`]; the batch
    /// remains borrowed, so a failed submission does not consume CPU data.
    ///
    /// # Safety
    /// Blue-noise and resolver images must belong to this device, be shader-readable
    /// and remain immutable until actual submission completion, including after timeout.
    pub unsafe fn render_batch_offscreen(
        &mut self,
        width: u32,
        height: u32,
        batch: &RenderBatch,
        atlas: AtlasDescriptor,
        images: &[Arc<Texture>],
    ) -> GpuResult<RenderedText> {
        unsafe { self.render_offscreen(width, height, batch, atlas, &batch.glyph_uploads, images) }
    }

    /// Upload, draw and read pixels using explicitly supplied glyph updates.
    /// `uploads` replaces `batch.glyph_uploads`. A fresh GPU atlas requires pixel
    /// coverage for every referenced glyph; stale uploads and malformed batches fail
    /// before mapped writes. Successful completion publishes the reusable atlas/target.
    ///
    /// # Errors
    /// Returns CPU validation, shader/device, allocation, command or completion errors.
    /// After timeout, retain external texture contents until completion; a subsequent
    /// call refuses mapped-buffer reuse while the earlier submission is pending.
    ///
    /// # Safety
    /// Blue-noise and resolver images must be shader-readable, use this device,
    /// and remain immutable until GPU completion. After a timeout, the owner must
    /// keep external images immutable until device submission completion is known.
    pub unsafe fn render_offscreen(
        &mut self,
        width: u32,
        height: u32,
        batch: &RenderBatch,
        atlas_desc: AtlasDescriptor,
        uploads: &[PendingGlyphUpload],
        images: &[Arc<Texture>],
    ) -> GpuResult<RenderedText> {
        let input = self.validate_inputs(width, height, batch, atlas_desc, uploads, images)?;
        let pipeline = self.pipeline.try_get()?;
        // Completion is a gate before any mapped write, including after a timeout.
        self.device.try_collect_garbage()?;
        self.command_pool.reset_completed(&mut [])?;
        self.heap.collect()?;
        let packed_uploads = PackedGlyphUploads::new(input.uploads)?;
        let frame = self.prepare_frame(&input, pipeline)?;
        let mut command = self.command_pool.allocate()?;
        command.begin()?;
        unsafe {
            self.record_frame(&mut command, &frame, &packed_uploads, &input)?;
        }
        command.end()?;
        let pending = command.submit()?;
        pending.wait(self.completion_timeout.as_nanos().try_into()?)?;
        let mut rgba = vec![0; input.readback_bytes.try_into()?];
        unsafe {
            frame.readback.read_bytes_after_completion(0, &mut rgba)?;
        }
        // Publish reusable images only after successful completion and readback.
        self.target = Some(([width, height], frame.target));
        self.atlas = Some((atlas_desc, frame.atlas));
        Ok(RenderedText {
            width,
            height,
            rgba,
            stats: GpuTextStats {
                primitives: batch.primitives.len(),
                upload_regions: packed_uploads.regions.len(),
                upload_bytes: packed_uploads.bytes.len(),
                draw_calls: 1,
                submits: 1,
                command_pool_resets: self.command_pool.reset_count(),
                compiled_pipeline_requests: self.compiler.stats().compiled,
                dynamic_rendering: frame.pipeline.dynamic_rendering_formats().is_some(),
                bindless_live_slots: self.heap.stats().live,
                dynamic_constant_page_creations: self.parameters.stats().page_creations,
                transfer_buffer_creations: self.transfer_buffer_creations,
            },
        })
    }
}

fn texture_range(texture: &Texture) -> rhapsody::Result<OwnershipRange> {
    let aspect = rhapsody::resource::vk_format_info(texture.desc.format)
        .ok_or_else(|| rhapsody::Error::unsupported("text texture has no native aspect"))?
        .aspect;
    Ok(OwnershipRange::Image {
        aspects: aspect.as_raw(),
        base_mip: 0,
        mip_count: u32::from(texture.desc.mip_levels),
        base_layer: 0,
        layer_count: texture.desc.array_layers()?,
    })
}
