//! 独立文本 GPU 适配。直接读取发行 SPIR-V；资源与管线复用 rhapsody。
//! 离屏接口每帧统一提交/等待一次，属于独立验收适配，不声称原 toolbar 的异步调度已恢复。
use crate::renderer::{AtlasDescriptor, PendingGlyphUpload, RenderBatch};
use ash::vk;
use rhapsody::{
    raw::{
        binding::{DescriptorSetBinding, bind_persistent_descriptor_set},
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        descriptor_set::VulkanDescriptorPool,
        device::VulkanDevice,
        sampler::Sampler,
        shader::{RasterPipelineDesc, ShaderLibrary, VulkanPipeline, get_vk_blend_mode},
        texture::{Texture, TextureDesc, TextureViewDesc},
        texture_upload::record_texture_regions,
    },
    rg::{
        attachments::{Attachment, AttachmentCache},
        pass_api::{AshCommandBackend, RenderPassApi},
    },
    shader_pipeline::compiled::CompiledShaderPipeline,
};
use std::{collections::BTreeMap, ffi::CString, sync::Arc};
pub type GpuResult<T> = Result<T, Box<dyn std::error::Error>>;
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuTextStats {
    pub primitives: usize,
    pub upload_regions: usize,
    pub upload_bytes: usize,
    pub draw_calls: u32,
    pub submits: u32,
}
pub struct RenderedText {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub stats: GpuTextStats,
}
/// 只接受已初始化的采样纹理；保持资源至本帧 GPU 完成。
pub struct GpuTextRenderer {
    device: Arc<VulkanDevice>,
    library: ShaderLibrary,
    attachments: AttachmentCache,
    blue_noise: Arc<Texture>,
    samplers: BTreeMap<String, Arc<Sampler>>,
    atlas: Option<(AtlasDescriptor, Arc<Texture>)>,
    buffers: [Option<Arc<Buffer>>; 3],
    target: Option<([u32; 2], Arc<Texture>)>,
}
fn sampled_2d(texture: &Texture) -> bool {
    texture.desc.image_type == vk::ImageType::TYPE_2D
        && texture.desc.array_layers == 1
        && texture.desc.samples == vk::SampleCountFlags::TYPE_1
        && texture.desc.usage.contains(vk::ImageUsageFlags::SAMPLED)
        && rhapsody::raw::format::vk_format_info(texture.desc.format)
            .is_some_and(|f| f.aspect == vk::ImageAspectFlags::COLOR)
}
impl GpuTextRenderer {
    pub fn new(
        device: Arc<VulkanDevice>,
        compiled_bytes: &[u8],
        blue_noise: Arc<Texture>,
    ) -> GpuResult<Self> {
        if !Arc::ptr_eq(&device, &blue_noise.device) || !sampled_2d(&blue_noise) {
            return Err("blue-noise texture must be sampled and use this device".into());
        }
        let compiled = CompiledShaderPipeline::decode(compiled_bytes)?;
        for (name, set, binding) in [
            ("$Globals", 0, 13),
            ("primitives", 0, 15),
            ("shape_stream", 0, 16),
            ("font_atlas_tex", 0, 17),
            ("blue_noise_texture", 0, 14),
            ("bindless_textures_dyn", 2, 0),
        ] {
            let slot = compiled
                .reflection
                .binding_by_name(name)
                .ok_or("text shader binding absent")?;
            if (slot.set, slot.binding) != (set, binding) {
                return Err("text shader binding differs from verified contract".into());
            }
        }
        let uniform = compiled
            .reflection
            .uniform_block
            .as_ref()
            .ok_or("text globals absent")?;
        if uniform.byte_size != 24
            || ![
                ("blue_noise_texture_dims", 0),
                ("window_size_pixels", 8),
                ("font_atlas_tex_dims", 16),
            ]
            .iter()
            .all(|(name, offset)| {
                uniform
                    .name_offsets
                    .iter()
                    .any(|(n, o)| n == name && o == offset)
            })
        {
            return Err("text globals layout differs from verified contract".into());
        }
        if !device
            .capabilities
            .shader_sampled_image_array_non_uniform_indexing
            || !device.capabilities.runtime_descriptor_array
            || !device.capabilities.descriptor_binding_partially_bound
            || !device.capabilities.sampled_image_update_after_bind
            || !device
                .capabilities
                .descriptor_binding_variable_descriptor_count
        {
            return Err("device lacks enabled text bindless features".into());
        }
        let mut samplers = BTreeMap::new();
        for name in ["sampler_lnc", "sampler_llc"] {
            let filter = vk::Filter::LINEAR;
            let mipmap = if name == "sampler_lnc" {
                vk::SamplerMipmapMode::NEAREST
            } else {
                vk::SamplerMipmapMode::LINEAR
            };
            let sampler = Sampler::new(
                device.clone(),
                &vk::SamplerCreateInfo::default()
                    .mag_filter(filter)
                    .min_filter(filter)
                    .mipmap_mode(mipmap)
                    .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE),
            )?;
            samplers.insert(name.to_owned(), Arc::new(sampler));
        }
        let mut library = ShaderLibrary::new(device.clone())?;
        library.register_compiled_raster(
            "tg-text-vs".into(),
            "tg-text-ps".into(),
            compiled_bytes,
        )?;
        Ok(Self {
            attachments: AttachmentCache::new(device.clone()),
            device,
            library,
            blue_noise,
            samplers,
            atlas: None,
            buffers: [None, None, None],
            target: None,
        })
    }
    fn data_buffer(
        &mut self,
        index: usize,
        bytes: &[u8],
        usage: vk::BufferUsageFlags,
    ) -> GpuResult<Arc<Buffer>> {
        let size = bytes.len().max(16) as u64;
        if self.buffers[index].as_ref().is_none_or(|b| b.size < size) {
            self.buffers[index] = Some(Arc::new(Buffer::new(
                self.device.clone(),
                &BufferDesc::new_cpu_to_gpu(size.next_power_of_two(), usage),
            )?));
        }
        let buffer = self.buffers[index].as_ref().unwrap().clone();
        // 本对象的离屏帧串行执行，前一帧已经等待完成；当前 GPU 尚未读取这些字节。
        buffer.write_bytes_unsynchronized(0, bytes)?;
        Ok(buffer)
    }
    /// 独立完整链：批量上传→真实 shader 绘制→像素读回。
    ///
    /// # Safety
    /// blue_noise/images 必须处于 SHADER_READ_ONLY_OPTIMAL，同属本设备且在调用期间不被并发修改。
    pub unsafe fn render_offscreen(
        &mut self,
        width: u32,
        height: u32,
        batch: &RenderBatch,
        atlas_desc: AtlasDescriptor,
        uploads: &[PendingGlyphUpload],
        images: &[Arc<Texture>],
    ) -> GpuResult<RenderedText> {
        if width == 0 || height == 0 || atlas_desc.width == 0 || atlas_desc.height == 0 {
            return Err("zero extent".into());
        }
        if images
            .iter()
            .any(|image| !Arc::ptr_eq(&image.device, &self.device) || !sampled_2d(image))
        {
            return Err("image resolver resource uses wrong device or usage".into());
        }
        for p in &batch.primitives {
            if p.stream != 0 && p.stream != u32::MAX {
                let start = (p.stream >> 8) as usize;
                let len = (p.stream & 255) as usize;
                let words = batch
                    .shape_words
                    .get(start..start.checked_add(len).ok_or("shape offset overflow")?)
                    .ok_or("shape stream range outside batch")?;
                let mut i = 0;
                while i < words.len() {
                    match words[i] & 255 {
                        0 => {
                            if i + 1 >= words.len() {
                                return Err("truncated fill operation".into());
                            }
                            i += 2;
                        }
                        1 => {
                            if ((words[i] >> 8) & 65535) as usize >= images.len() {
                                return Err("image index outside resolver resources".into());
                            }
                            i += 1;
                        }
                        2 => i += 1,
                        _ => break,
                    }
                }
            }
        }
        let fresh = self.atlas.as_ref().is_none_or(|(a, _)| *a != atlas_desc);
        if fresh
            && batch.primitives.iter().filter(|p| p.stream == 0).any(|p| {
                !uploads.iter().any(|u| {
                    u.generation == atlas_desc.generation
                        && p.uv_min[0] >= u.x as f32
                        && p.uv_min[1] >= u.y as f32
                        && p.uv_min[0] + p.uv_extent[0] <= (u.x as f64 + u.width as f64) as f32
                        && p.uv_min[1] + p.uv_extent[1] <= (u.y as f64 + u.height as f64) as f32
                })
            })
        {
            return Err("fresh GPU atlas lacks cached glyph pixels; reset_glyph_cache then prepare and upload again".into());
        }
        let atlas = if fresh {
            let desc = TextureDesc::new_2d(
                atlas_desc.width,
                atlas_desc.height,
                vk::Format::R8G8B8A8_UNORM,
            )
            .usage(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED);
            Arc::new(Texture::new(self.device.clone(), &desc)?)
        } else {
            self.atlas.as_ref().unwrap().1.clone()
        };
        let mut upload_data = Vec::new();
        let mut regions = Vec::new();
        for upload in uploads {
            if upload.generation != atlas_desc.generation
                || Some(upload.rgba.len())
                    != usize::try_from(upload.width)?
                        .checked_mul(usize::try_from(upload.height)?)
                        .and_then(|n| n.checked_mul(4))
            {
                return Err("stale or invalid glyph upload".into());
            }
            regions.push(
                vk::BufferImageCopy::default()
                    .buffer_offset(upload_data.len() as u64)
                    .image_subresource(
                        vk::ImageSubresourceLayers::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .layer_count(1),
                    )
                    .image_offset(vk::Offset3D {
                        x: upload.x.try_into()?,
                        y: upload.y.try_into()?,
                        z: 0,
                    })
                    .image_extent(vk::Extent3D {
                        width: upload.width,
                        height: upload.height,
                        depth: 1,
                    }),
            );
            upload_data.extend_from_slice(&upload.rgba);
        }
        let mut globals = Vec::with_capacity(24);
        for bytes in [
            self.blue_noise.desc.extent.width.to_le_bytes(),
            self.blue_noise.desc.extent.height.to_le_bytes(),
            (width as f32).to_le_bytes(),
            (height as f32).to_le_bytes(),
            atlas_desc.width.to_le_bytes(),
            atlas_desc.height.to_le_bytes(),
        ] {
            globals.extend_from_slice(&bytes);
        }
        let primitive = self.data_buffer(
            0,
            batch.primitive_bytes(),
            vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;
        let shape = self.data_buffer(
            1,
            bytemuck::cast_slice(&batch.shape_words),
            vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;
        let uniform = self.data_buffer(2, &globals, vk::BufferUsageFlags::UNIFORM_BUFFER)?;
        let desc = TextureDesc::new_2d(width, height, vk::Format::R8G8B8A8_UNORM)
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC);
        let fresh_target = self
            .target
            .as_ref()
            .is_none_or(|(extent, _)| *extent != [width, height]);
        let target = if fresh_target {
            self.attachments.clear();
            self.library.cache.invalidate();
            Arc::new(Texture::new(self.device.clone(), &desc)?)
        } else {
            self.target.as_ref().unwrap().1.clone()
        };
        let mut attachment = Attachment::color(target.clone(), 1, Some([0.; 4]));
        attachment.initial_layout = if fresh_target {
            vk::ImageLayout::UNDEFINED
        } else {
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL
        };
        let prepared = self
            .attachments
            .prepare(&[attachment], desc.extent_2d(), 1)?;
        let program =
            self.library
                .prepare_raster_program("tg-text-vs", "tg-text-ps", &self.samplers)?;
        let mut state =
            RasterPipelineDesc::new(program.vertex.module.clone(), program.layout.clone());
        state.render_pass = prepared.framebuffer.render_pass.handle;
        state.render_pass_owner = prepared.framebuffer.render_pass.clone();
        state.entry = CString::new("vs_main")?;
        state.depth_test = false;
        state.depth_write = false;
        state.cull_mode = vk::CullModeFlags::NONE;
        state.topology = vk::PrimitiveTopology::TRIANGLE_LIST;
        state.blend_attachments = vec![get_vk_blend_mode(64)];

        let pipeline = VulkanPipeline::create_raster_pipeline(
            &self.library.cache,
            program.pipeline_desc(state)?,
        )?;

        let mut sets = Vec::new();
        for (set_index, layout) in program.layout.sets.iter().enumerate() {
            if layout.plan.bindings.is_empty() {
                continue;
            }

            let pool = VulkanDescriptorPool::new(self.device.clone(), &layout.plan, 1)?;
            let mut set = pool.allocate(layout.clone())?;
            for binding in &layout.plan.bindings {
                let ty = vk::DescriptorType::from_raw(binding.descriptor_type);
                if set_index == 0 {
                    if let Some(buffer) = match binding.binding {
                        13 => Some(&uniform),
                        15 => Some(&primitive),
                        16 => Some(&shape),
                        _ => None,
                    } {
                        unsafe {
                            set.update(
                                binding.binding,
                                0,
                                &DescriptorSetBinding::Buffer {
                                    ty,
                                    info: vk::DescriptorBufferInfo::default()
                                        .buffer(buffer.handle)
                                        .range(buffer.size),
                                    owner: buffer.clone(),
                                },
                            )?;
                        }
                    } else if binding.binding == 14 || binding.binding == 17 {
                        let image = if binding.binding == 14 {
                            self.blue_noise.clone()
                        } else {
                            atlas.clone()
                        };
                        let view = Arc::new(image.image_view(&TextureViewDesc::new(&image.desc))?);
                        unsafe {
                            set.update(
                                binding.binding,
                                0,
                                &DescriptorSetBinding::Image {
                                    ty,
                                    info: vk::DescriptorImageInfo::default()
                                        .image_view(view.handle)
                                        .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL),
                                    owners: vec![view],
                                },
                            )?;
                        }
                    }
                } else if set_index == 2 && binding.binding == 0 {
                    if images.len() > binding.descriptor_count as usize {
                        return Err("too many resolver images".into());
                    }
                    for (index, image) in images.iter().enumerate() {
                        let view = Arc::new(image.image_view(&TextureViewDesc::new(&image.desc))?);
                        unsafe {
                            set.update(
                                0,
                                index as u32,
                                &DescriptorSetBinding::Image {
                                    ty,
                                    info: vk::DescriptorImageInfo::default()
                                        .image_view(view.handle)
                                        .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL),
                                    owners: vec![view],
                                },
                            )?;
                        }
                    }
                }
            }
            sets.push((set_index as u32, Arc::new(set)));
        }
        let length = u64::from(width)
            .checked_mul(u64::from(height))
            .and_then(|n| n.checked_mul(4))
            .ok_or("target byte size overflow")?;
        let readback = Arc::new(Buffer::new(
            self.device.clone(),
            &BufferDesc::new_gpu_to_cpu(length, vk::BufferUsageFlags::TRANSFER_DST),
        )?);

        let pool = VulkanCommandPool::new(self.device.clone())?;
        let mut command = pool.allocate()?;
        command.begin()?;
        prepared.keep_alive(&mut command);
        command.keep_alive(pipeline.clone());
        command.keep_alive(readback.clone());
        for (_, set) in &sets {
            command.keep_alive(set.clone());
        }
        if fresh {
            let range = vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1);
            unsafe {
                self.device.device.cmd_pipeline_barrier(
                    command.handle,
                    vk::PipelineStageFlags::TOP_OF_PIPE,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[vk::ImageMemoryBarrier::default()
                        .old_layout(vk::ImageLayout::UNDEFINED)
                        .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                        .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                        .image(atlas.handle)
                        .subresource_range(range)],
                );
                self.device.device.cmd_clear_color_image(
                    command.handle,
                    atlas.handle,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &vk::ClearColorValue { float32: [0.; 4] },
                    &[range],
                );
                self.device.device.cmd_pipeline_barrier(
                    command.handle,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::FRAGMENT_SHADER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[vk::ImageMemoryBarrier::default()
                        .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                        .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                        .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                        .dst_access_mask(vk::AccessFlags::SHADER_READ)
                        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                        .image(atlas.handle)
                        .subresource_range(range)],
                );
            }
        }
        command.keep_alive(atlas.clone());

        if !regions.is_empty() {
            let staging = Arc::new(Buffer::new(
                self.device.clone(),
                &BufferDesc::new_cpu_to_gpu(
                    upload_data.len() as u64,
                    vk::BufferUsageFlags::TRANSFER_SRC,
                ),
            )?);
            staging.write_bytes_unsynchronized(0, &upload_data)?;
            unsafe {
                record_texture_regions(
                    &mut command,
                    atlas.clone(),
                    staging,
                    &regions,
                    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                )?;
            }
        }
        {
            let mut backend = unsafe { AshCommandBackend::new(&self.device.device) };
            let mut api = RenderPassApi::new(&mut backend, command.handle, 0, false);
            api.begin_attachments(&prepared);
            api.set_default_view_and_scissor(desc.extent_2d());
            unsafe {
                pipeline.bind_pipeline(command.handle);
                for (set_index, set) in &sets {
                    bind_persistent_descriptor_set(
                        &self.device,
                        command.handle,
                        vk::PipelineBindPoint::GRAPHICS,
                        program.layout.handle,
                        *set_index,
                        &[set.handle],
                        &[],
                    );
                }
                self.device.device.cmd_draw(
                    command.handle,
                    (batch
                        .primitives
                        .len()
                        .checked_mul(6)
                        .ok_or("draw count overflow")?)
                    .try_into()?,
                    1,
                    0,
                    0,
                );
            }
            api.end_render_pass();
        }
        unsafe {
            self.device.device.cmd_pipeline_barrier(
                command.handle,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[vk::ImageMemoryBarrier::default()
                    .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                    .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
                    .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                    .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .image(target.handle)
                    .subresource_range(
                        vk::ImageSubresourceRange::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .level_count(1)
                            .layer_count(1),
                    )],
            );
            self.device.device.cmd_copy_image_to_buffer(
                command.handle,
                target.handle,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                readback.handle,
                &[vk::BufferImageCopy::default()
                    .image_subresource(
                        vk::ImageSubresourceLayers::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .layer_count(1),
                    )
                    .image_extent(desc.extent)],
            );
            self.device.device.cmd_pipeline_barrier(
                command.handle,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::HOST,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier::default()
                    .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                    .dst_access_mask(vk::AccessFlags::HOST_READ)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .buffer(readback.handle)
                    .size(length)],
                &[],
            );
        }

        command.end()?;
        let pending = command.submit()?;
        pending.wait(u64::MAX)?;
        let mut rgba = vec![0; length.try_into()?];
        unsafe {
            readback.read_bytes_after_completion(0, &mut rgba)?;
        }
        // 复用同尺寸的目标及附件，避免每帧新增 renderpass/pipeline cache 键。
        self.target = Some(([width, height], target));
        self.atlas = Some((atlas_desc, atlas));
        Ok(RenderedText {
            width,
            height,
            rgba,
            stats: GpuTextStats {
                primitives: batch.primitives.len(),
                upload_regions: regions.len(),
                upload_bytes: upload_data.len(),
                draw_calls: 1,
                submits: 1,
            },
        })
    }
}
