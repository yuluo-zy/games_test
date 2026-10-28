//! Compute and raster pipeline objects.
use ash::vk;
use crate::Result;
use crate::accounting::CpuMemoryReservation;
use crate::accounting::cpu_vec_bytes;
use crate::accounting::reserve_shared_cpu_metadata;
use crate::accounting::shared_cpu_bytes;
use crate::device::Device;
use std::ffi::CStr;
use std::ffi::CString;
use std::sync::Arc;
use super::layout::BudgetedMetadata;
use super::layout::PipelineLayout;
use super::layout::ShaderModule;
use super::pipeline_cache::BudgetedKey;
use super::pipeline_cache::PipelineCache;
use super::pipeline_cache::budgeted_key_vec;
use super::pipeline_cache::budgeted_raster_key;
use super::pipeline_cache::slot_for;
use super::pipeline_key::raster_pipeline_key_matches;
use super::raster::DynamicRenderingFormats;
use super::raster::RasterPipelineDesc;
use super::raster::RasterTargetSignature;

#[derive(Clone)]
pub struct ComputePipelineDesc {
    pub shader: Arc<ShaderModule>,
    pub entry: CString,
    pub layout: Arc<PipelineLayout>,
}

impl ComputePipelineDesc {
    pub fn new(shader: Arc<ShaderModule>, layout: Arc<PipelineLayout>) -> Self {
        Self {
            shader,
            entry: c"main".to_owned(),
            layout,
        }
    }
}

pub(super) struct PipelineArrays {
    shaders: Vec<Arc<ShaderModule>>,
    entries: Vec<usize>,
    formats: Option<DynamicRenderingFormats>,
    target: Option<RasterTargetSignature>,
}

pub struct VulkanPipeline {
    retirement: Option<crate::device::RetireTicket>,
    pub(crate) last_use: Arc<crate::tracking::SyncCell>,
    pub(crate) device: Arc<Device>,
    pub(crate) handle: vk::Pipeline,
    pub layout: Arc<PipelineLayout>,
    pub point: vk::PipelineBindPoint,
    shaders: Vec<Arc<ShaderModule>>,
    shader_entries: Vec<usize>,
    dynamic_rendering: Option<DynamicRenderingFormats>,
    raster_target: Option<RasterTargetSignature>,
    _memory: CpuMemoryReservation,
}

impl VulkanPipeline {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::Pipeline {
        self.handle
    }
}

impl VulkanPipeline {
    pub fn create_compute_pipeline(
        cache: &PipelineCache,
        desc: ComputePipelineDesc,
    ) -> Result<Arc<Self>> {
        if cache.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if !Arc::ptr_eq(&cache.device, &desc.layout.device)
            || !Arc::ptr_eq(&cache.device, &desc.shader.device)
        {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let shader_entry = crate::pipeline::shader_interface::validate(
            &desc.shader.interfaces,
            &desc.entry,
            vk::ShaderStageFlags::COMPUTE,
            &desc.layout,
        )?;
        use ash::vk::Handle;
        let shader = desc.shader.handle.as_raw();
        let layout = desc.layout.handle.as_raw();
        let slot = slot_for(
            &cache.compute,
            |key| key.0 == shader && key.1 == layout && key.2.as_slice() == desc.entry.as_bytes(),
            || {
                let key = budgeted_key_vec(&cache.device.memory.stats, desc.entry.as_bytes())?;
                Ok(BudgetedKey {
                    value: (shader, layout, key.value),
                    memory: key.memory,
                })
            },
            cache.capacity,
            &cache.device.memory.stats,
        )?;
        let mut created = slot.lock();
        if let Some(pipeline) = created.pipeline.as_ref() {
            return Ok(pipeline.clone());
        }
        let count = 1;
        let mut memory = reserve_shared_cpu_metadata::<Self>(
            &cache.device.memory.stats,
            pipeline_array_bytes(count, count, 0, 0)?,
        )?;
        let mut shaders = try_vec(count)?;
        shaders.push(desc.shader.clone());
        let mut shader_entries = try_vec(count)?;
        shader_entries.push(shader_entry);
        memory.resize(
            shared_cpu_bytes::<Self>()?
                .checked_add(pipeline_array_bytes(
                    shaders.capacity(),
                    shader_entries.capacity(),
                    0,
                    0,
                )?)
                .ok_or(crate::Error::OutOfHostMemory)?,
        )?;
        let metadata = BudgetedMetadata {
            data: PipelineArrays {
                shaders,
                entries: shader_entries,
                formats: None,
                target: None,
            },
            memory,
        };
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(desc.shader.handle)
            .name(&desc.entry);
        let info = vk::ComputePipelineCreateInfo::default()
            .stage(stage)
            .layout(desc.layout.handle);
        let sync_storage =
            crate::tracking::SyncCell::reserve_arc_storage(&cache.device.memory.stats)?;
        let retirement = cache.device.reserve_retirement()?;
        let _cache_use = cache.lock.read();
        let result = unsafe {
            cache
                .device
                .core
                .device
                .create_compute_pipelines(cache.handle, &[info], None)
        };
        let handle = finish_pipeline(&cache.device, result)?;
        let pipeline = Arc::new(Self {
            retirement: Some(retirement),
            last_use: crate::tracking::SyncCell::for_device(cache.device.generation())
                .into_reserved_arc(sync_storage),
            device: cache.device.clone(),
            handle,
            layout: desc.layout,
            point: vk::PipelineBindPoint::COMPUTE,
            shaders: metadata.data.shaders,
            shader_entries: metadata.data.entries,
            dynamic_rendering: metadata.data.formats,
            raster_target: metadata.data.target,
            _memory: metadata.memory,
        });
        created.pipeline = Some(pipeline.clone());
        Ok(pipeline)
    }
    pub fn create_raster_pipeline(
        cache: &PipelineCache,
        desc: RasterPipelineDesc,
    ) -> Result<Arc<Self>> {
        if cache.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if !Arc::ptr_eq(&cache.device, &desc.layout.device)
            || !Arc::ptr_eq(&cache.device, &desc.vertex.device)
            || desc
                .fragment
                .as_ref()
                .is_some_and(|s| !Arc::ptr_eq(&cache.device, &s.device))
        {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let vertex_entry = crate::pipeline::shader_interface::validate(
            &desc.vertex.interfaces,
            &desc.entry,
            vk::ShaderStageFlags::VERTEX,
            &desc.layout,
        )?;
        let fragment_entry = desc
            .fragment
            .as_ref()
            .map(|fragment| {
                crate::pipeline::shader_interface::validate(
                    &fragment.interfaces,
                    desc.fragment_entry.as_deref().unwrap_or(&desc.entry),
                    vk::ShaderStageFlags::FRAGMENT,
                    &desc.layout,
                )
            })
            .transpose()?;
        {
            let formats = desc.dynamic_rendering.as_ref().ok_or_else(|| {
                crate::Error::invalid_usage("raster pipeline requires dynamic attachment formats")
            })?;
            if !cache
                .device
                .caps()
                .is_enabled(crate::device::Feature::DynamicRendering)
            {
                return Err(crate::Error::unsupported(
                    "dynamicRendering feature was not enabled",
                ));
            }
            let properties = cache.device.properties();
            if properties
                .api_version
                .min(cache.device.core.instance.api_version)
                < vk::API_VERSION_1_3
                && !cache
                    .device
                    .core
                    .enabled_extensions
                    .iter()
                    .any(|extension| extension == "VK_KHR_dynamic_rendering")
            {
                return Err(crate::Error::unsupported(
                    "dynamic rendering needs effective Vulkan 1.3 or enabled VK_KHR_dynamic_rendering",
                ));
            }
            let limits = properties.limits;
            formats.validate(limits.max_color_attachments)?;
            if formats.colors.len() != desc.blend_attachments.len() {
                return Err(crate::Error::invalid_usage(
                    "dynamic rendering color formats and blend attachments differ",
                ));
            }
            if (desc.depth_test || desc.depth_write) && formats.depth == vk::Format::UNDEFINED {
                return Err(crate::Error::invalid_usage(
                    "depth-enabled dynamic pipeline has no depth format",
                ));
            }
            if desc.stencil_test && formats.stencil == vk::Format::UNDEFINED {
                return Err(crate::Error::invalid_usage(
                    "stencil-enabled dynamic pipeline has no stencil format",
                ));
            }
            if desc.samples.as_raw().count_ones() != 1
                || (!formats.colors.is_empty()
                    && !limits
                        .framebuffer_color_sample_counts
                        .contains(desc.samples))
                || (formats.depth != vk::Format::UNDEFINED
                    && !limits
                        .framebuffer_depth_sample_counts
                        .contains(desc.samples))
                || (formats.stencil != vk::Format::UNDEFINED
                    && !limits
                        .framebuffer_stencil_sample_counts
                        .contains(desc.samples))
            {
                return Err(crate::Error::unsupported(
                    "dynamic rendering sample count exceeds framebuffer limits",
                ));
            }
        }
        let slot = slot_for(
            &cache.raster,
            |key| raster_pipeline_key_matches(&desc, key),
            || budgeted_raster_key(&cache.device.memory.stats, &desc),
            cache.capacity,
            &cache.device.memory.stats,
        )?;
        let mut created = slot.lock();
        if let Some(pipeline) = created.pipeline.as_ref() {
            return Ok(pipeline.clone());
        }
        let count = 1 + usize::from(desc.fragment.is_some());
        let formats = desc
            .dynamic_rendering
            .as_ref()
            .expect("validated dynamic formats");
        let mut memory = reserve_shared_cpu_metadata::<Self>(
            &cache.device.memory.stats,
            pipeline_array_bytes(
                count,
                count,
                formats.colors.capacity(),
                formats.colors.len(),
            )?,
        )?;
        let mut shaders = try_vec(count)?;
        shaders.push(desc.vertex.clone());
        let mut shader_entries = try_vec(count)?;
        shader_entries.push(vertex_entry);
        let mut stages = smallvec::SmallVec::<[vk::PipelineShaderStageCreateInfo<'_>; 2]>::new();
        stages.push(stage(
            desc.vertex.handle,
            vk::ShaderStageFlags::VERTEX,
            &desc.entry,
        ));
        if let Some(fragment) = &desc.fragment {
            stages.push(stage(
                fragment.handle,
                vk::ShaderStageFlags::FRAGMENT,
                desc.fragment_entry.as_deref().unwrap_or(&desc.entry),
            ));
            shaders.push(fragment.clone());
            shader_entries.push(fragment_entry.expect("validated fragment entry"));
        }
        // The public target signature owns a distinct color Vec. Construct and
        // charge both actual allocations before entering Vulkan.
        let mut colors = try_vec(formats.colors.len())?;
        colors.extend_from_slice(&formats.colors);
        let target_formats = DynamicRenderingFormats {
            colors,
            depth: formats.depth,
            stencil: formats.stencil,
            view_mask: formats.view_mask,
        };
        memory.resize(
            shared_cpu_bytes::<Self>()?
                .checked_add(pipeline_array_bytes(
                    shaders.capacity(),
                    shader_entries.capacity(),
                    formats.colors.capacity(),
                    target_formats.colors.capacity(),
                )?)
                .ok_or(crate::Error::OutOfHostMemory)?,
        )?;
        let raster_target = Some(RasterTargetSignature::Dynamic {
            formats: target_formats,
            samples: desc.samples,
        });
        let metadata = BudgetedMetadata {
            data: PipelineArrays {
                shaders,
                entries: shader_entries,
                formats: desc.dynamic_rendering,
                target: raster_target,
            },
            memory,
        };
        let vertex = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&desc.vertex_bindings)
            .vertex_attribute_descriptions(&desc.vertex_attributes);
        let assembly = vk::PipelineInputAssemblyStateCreateInfo::default().topology(desc.topology);
        let viewport = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(desc.polygon_mode)
            .cull_mode(desc.cull_mode)
            .front_face(desc.front_face)
            .line_width(1.0);
        let multisample =
            vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(desc.samples);
        let depth = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(desc.depth_test)
            .depth_write_enable(desc.depth_write)
            .depth_compare_op(desc.depth_compare)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(desc.stencil_test)
            .front(desc.stencil)
            .back(desc.stencil)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0);
        let blend =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&desc.blend_attachments);
        let line_width_dynamic = cache.device.core.capabilities.wide_lines;
        let mut dynamic_values: smallvec::SmallVec<[vk::DynamicState; 4]> = smallvec::smallvec![
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::SCISSOR,
            vk::DynamicState::STENCIL_REFERENCE,
        ];
        if line_width_dynamic {
            dynamic_values.push(vk::DynamicState::LINE_WIDTH);
        }
        let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_values);
        let mut info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex)
            .input_assembly_state(&assembly)
            .viewport_state(&viewport)
            .rasterization_state(&raster)
            .multisample_state(&multisample)
            .depth_stencil_state(&depth)
            .color_blend_state(&blend)
            .dynamic_state(&dynamic)
            .layout(desc.layout.handle);
        let mut rendering = vk::PipelineRenderingCreateInfo::default();
        if let Some(formats) = &metadata.data.formats {
            rendering = rendering
                .color_attachment_formats(&formats.colors)
                .depth_attachment_format(formats.depth)
                .stencil_attachment_format(formats.stencil)
                .view_mask(formats.view_mask);
            info = info.push_next(&mut rendering);
        }
        let sync_storage =
            crate::tracking::SyncCell::reserve_arc_storage(&cache.device.memory.stats)?;
        let retirement = cache.device.reserve_retirement()?;
        let _cache_use = cache.lock.read();
        let result = unsafe {
            cache
                .device
                .core
                .device
                .create_graphics_pipelines(cache.handle, &[info], None)
        };
        let handle = finish_pipeline(&cache.device, result)?;
        let pipeline = Arc::new(Self {
            retirement: Some(retirement),
            last_use: crate::tracking::SyncCell::for_device(cache.device.generation())
                .into_reserved_arc(sync_storage),
            device: cache.device.clone(),
            handle,
            layout: desc.layout,
            point: vk::PipelineBindPoint::GRAPHICS,
            shaders: metadata.data.shaders,
            shader_entries: metadata.data.entries,
            raster_target: metadata.data.target,
            dynamic_rendering: metadata.data.formats,
            _memory: metadata.memory,
        });
        created.pipeline = Some(pipeline.clone());
        Ok(pipeline)
    }
    /// # Safety
    /// command 必须处于正确录制状态，上层提交链应保持本对象至 GPU 完成。
    pub unsafe fn bind_pipeline(&self, command: vk::CommandBuffer) {
        unsafe {
            self.device
                .core
                .device
                .cmd_bind_pipeline(command, self.point, self.handle)
        }
    }
    pub fn shader_count(&self) -> usize {
        self.shaders.len()
    }
    pub fn raster_target_signature(&self) -> Option<&RasterTargetSignature> {
        self.raster_target.as_ref()
    }
    pub(crate) fn has_same_shader_interface(&self, other: &Self) -> bool {
        self.shaders.len() == other.shaders.len()
            && self
                .shaders
                .iter()
                .zip(&self.shader_entries)
                .zip(other.shaders.iter().zip(&other.shader_entries))
                .all(|((shader, &entry), (old, &old_entry))| {
                    shader.interfaces[entry].same_interface(&old.interfaces[old_entry])
                })
    }
    pub fn dynamic_rendering_formats(&self) -> Option<&DynamicRenderingFormats> {
        self.dynamic_rendering.as_ref()
    }
}

pub(super) fn try_vec<T>(capacity: usize) -> Result<Vec<T>> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(capacity)
        .map_err(|_| crate::Error::OutOfHostMemory)?;
    Ok(values)
}

pub(super) fn pipeline_array_bytes(
    shaders: usize,
    entries: usize,
    formats: usize,
    target_formats: usize,
) -> Result<u64> {
    cpu_vec_bytes::<Arc<ShaderModule>>(shaders)?
        .checked_add(cpu_vec_bytes::<usize>(entries)?)
        .and_then(|bytes| bytes.checked_add(cpu_vec_bytes::<vk::Format>(formats).ok()?))
        .and_then(|bytes| bytes.checked_add(cpu_vec_bytes::<vk::Format>(target_formats).ok()?))
        .ok_or(crate::Error::OutOfHostMemory)
}

pub(super) fn stage<'a>(
    module: vk::ShaderModule,
    kind: vk::ShaderStageFlags,
    entry: &'a CStr,
) -> vk::PipelineShaderStageCreateInfo<'a> {
    vk::PipelineShaderStageCreateInfo::default()
        .module(module)
        .stage(kind)
        .name(entry)
}

pub(super) fn finish_pipeline(
    device: &Device,
    result: Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)>,
) -> Result<vk::Pipeline> {
    match result {
        Ok(values) => values
            .first()
            .copied()
            .ok_or_else(|| crate::Error::invalid_usage("着色器资源、接口或设备不匹配")),
        Err((values, error)) => {
            if error == vk::Result::ERROR_DEVICE_LOST {
                device.queues.timelines.mark_device_lost();
            }
            for handle in values {
                unsafe { device.core.device.destroy_pipeline(handle, None) }
            }
            Err(error.into())
        }
    }
}

impl Drop for VulkanPipeline {
    fn drop(&mut self) {
        self.device.retire_reserved(
            self.last_use.snapshot(),
            crate::device::graveyard::Doomed::Pipeline(self.handle),
            self.retirement.take().expect("pipeline retirement node"),
        );
    }
}

impl crate::tracking::GpuUse for VulkanPipeline {
    fn visit_uses(&self, visitor: &mut crate::tracking::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;
        self.layout.visit_uses(visitor)?;

        Ok(())
    }
}
