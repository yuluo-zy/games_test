//! Frame resource preparation and ordered Vulkan command recording.
use super::input::{Globals, PackedGlyphUploads, ValidatedFrame};
use super::*;
use rhapsody::{
    bindless::PreparedBindlessParameters,
    raw::{command_buffer::VulkanCommandBuffer, shader::VulkanPipeline},
    rg::attachments::PreparedAttachments,
};

#[derive(Default)]
pub(super) struct InputBufferCache {
    primitives: Option<Arc<Buffer>>,
    shapes: Option<Arc<Buffer>>,
    globals: Option<Arc<Buffer>>,
}
pub(super) struct PreparedFrame {
    pub(super) atlas: Arc<Texture>,
    pub(super) target: Arc<Texture>,
    descriptor: TextureDesc,
    primitives: Arc<Buffer>,
    shapes: Arc<Buffer>,
    globals: Arc<Buffer>,
    attachments: Arc<PreparedAttachments>,
    parameters: PreparedBindlessParameters,
    pub(super) pipeline: Arc<VulkanPipeline>,
    pub(super) readback: Arc<Buffer>,
}

// Called only after reset_completed accepted the previous submission, so mapped
// writes cannot race with GPU reads. Arc clones transfer resource ownership to commands.
fn upload_buffer(
    device: &Arc<Device>,
    slot: &mut Option<Arc<Buffer>>,
    bytes: &[u8],
    usage: vk::BufferUsageFlags,
) -> GpuResult<Arc<Buffer>> {
    let required_bytes = bytes.len().max(16) as u64;
    let buffer = match slot {
        Some(buffer) if buffer.size >= required_bytes => buffer.clone(),
        _ => {
            let capacity = required_bytes
                .checked_next_power_of_two()
                .ok_or("buffer capacity overflow")?;
            let buffer = Arc::new(Buffer::new(
                device.clone(),
                &BufferDesc::new_cpu_to_gpu(capacity, usage),
            )?);
            *slot = Some(buffer.clone());
            buffer
        }
    };
    buffer.write_bytes_unsynchronized(0, bytes)?;
    Ok(buffer)
}
impl GpuTextRenderer {
    pub(super) fn prepare_frame(
        &mut self,
        input: &ValidatedFrame<'_>,
        pipeline: Arc<VulkanPipeline>,
    ) -> GpuResult<PreparedFrame> {
        let width = input.width;
        let height = input.height;
        let atlas_desc = input.atlas;
        let batch = input.batch;
        let images = input.images;
        let length = input.readback_bytes;
        let fresh = input.atlas_is_new;
        let atlas = if fresh {
            let desc = TextureDesc::new_2d(
                atlas_desc.width,
                atlas_desc.height,
                vk::Format::R8G8B8A8_UNORM,
            )
            .usage(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED);
            Arc::new(Texture::new(self.device.clone(), &desc)?)
        } else {
            self.atlas
                .as_ref()
                .ok_or("cached atlas is absent")?
                .1
                .clone()
        };
        let globals = Globals {
            blue_noise_dimensions: [
                self.blue_noise.desc.extent().width,
                self.blue_noise.desc.extent().height,
            ],
            window_pixels: [width as f32, height as f32],
            atlas_dimensions: [atlas_desc.width, atlas_desc.height],
        }
        .encode();
        let primitive = upload_buffer(
            &self.device,
            &mut self.buffers.primitives,
            batch.primitive_bytes(),
            vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;
        let shape = upload_buffer(
            &self.device,
            &mut self.buffers.shapes,
            bytemuck::cast_slice(&batch.shape_words),
            vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;
        let uniform = upload_buffer(
            &self.device,
            &mut self.buffers.globals,
            &globals,
            vk::BufferUsageFlags::UNIFORM_BUFFER | vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;
        let desc = TextureDesc::new_2d(width, height, vk::Format::R8G8B8A8_UNORM)
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC);
        let fresh_target = self
            .target
            .as_ref()
            .is_none_or(|(extent, _)| *extent != [width, height]);
        let target = if fresh_target {
            self.attachments.clear();
            Arc::new(Texture::new(self.device.clone(), &desc)?)
        } else {
            self.target
                .as_ref()
                .ok_or("cached target is absent")?
                .1
                .clone()
        };
        let mut attachment = Attachment::color(target.clone(), 1, Some([0.; 4]))?;
        // Dynamic rendering uses the explicit transition recorded below.
        attachment.initial_layout = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
        let prepared = self.attachments.prepare_for_rendering(
            &[attachment],
            desc.extent_2d(),
            1,
            RenderingPolicy::RequireDynamic,
        )?;
        if prepared.dynamic_formats() != pipeline.dynamic_rendering_formats() {
            return Err(rhapsody::Error::invalid_usage(
                "text pipeline and attachment formats differ",
            )
            .into());
        }
        let mut parameters = BindlessParameters::new(self.heap.clone(), self.abi.clone(), &[])?;
        for (location, lease) in &self.sampler_leases {
            parameters.write_lease(*location, 0, lease.clone())?;
        }
        for (location, buffer, size) in [
            (self.bindings.globals, &uniform, globals.len() as u64),
            (
                self.bindings.primitives,
                &primitive,
                batch.primitive_bytes().len().max(16) as u64,
            ),
            (
                self.bindings.shape,
                &shape,
                (batch.shape_words.len() as u64 * 4).max(16),
            ),
        ] {
            parameters.write(
                location,
                0,
                BindlessResource::Buffer {
                    buffer: buffer.clone(),
                    offset: 0,
                    size,
                },
            )?;
        }
        for (location, image) in [
            (self.bindings.blue, &self.blue_noise),
            (self.bindings.atlas, &atlas),
        ] {
            let view = Arc::new(image.image_view(&TextureViewDesc::new(&image.desc)?)?);
            parameters.write(
                location,
                0,
                BindlessResource::SampledImage {
                    view,
                    layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                },
            )?;
        }
        for (index, image) in images.iter().enumerate() {
            let view = Arc::new(image.image_view(&TextureViewDesc::new(&image.desc)?)?);
            parameters.write(
                self.bindings.images,
                index as u32,
                BindlessResource::SampledImage {
                    view,
                    layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                },
            )?;
        }
        let shader_parameters = parameters.finish(&mut self.parameters)?;
        let readback = self.transfer_buffer(length, true)?;

        Ok(PreparedFrame {
            atlas,
            target,
            descriptor: desc,
            primitives: primitive,
            shapes: shape,
            globals: uniform,
            attachments: prepared,
            parameters: shader_parameters,
            pipeline,
            readback,
        })
    }
    /// # Safety
    /// `command` must be recording, and `frame` must have been prepared for `input`.
    /// External sampled textures obey the public submission immutability contract.
    pub(super) unsafe fn record_frame(
        &mut self,
        command: &mut VulkanCommandBuffer,
        frame: &PreparedFrame,
        uploads: &PackedGlyphUploads,
        input: &ValidatedFrame<'_>,
    ) -> GpuResult<()> {
        let images = input.images;
        let fresh = input.atlas_is_new;
        let vertex_count = input.vertex_count;
        let length = input.readback_bytes;
        let queue = self.device.queue(rhapsody::device::QueueClass::Graphics)?;
        if fresh {
            // Clear untouched texels before partial updates so linear filtering
            // at a glyph boundary never reads uninitialized atlas contents.
            unsafe {
                record_texture_zero_fill(command, frame.atlas.clone(), queue)?;
            }
        }

        if !uploads.regions.is_empty() {
            let staging = self.transfer_buffer(uploads.bytes.len() as u64, false)?;
            staging.write_bytes_unsynchronized(0, &uploads.bytes)?;
            unsafe {
                record_texture_regions(
                    command,
                    queue,
                    frame.atlas.clone(),
                    staging,
                    &uploads.regions,
                    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                )?;
            }
        }
        let sampled = AccessScope {
            stages: vk::PipelineStageFlags::VERTEX_SHADER | vk::PipelineStageFlags::FRAGMENT_SHADER,
            access: vk::AccessFlags::SHADER_READ,
            write: false,
        };
        unsafe {
            for image in [&self.blue_noise, &frame.atlas]
                .into_iter()
                .chain(images.iter())
            {
                image_access(
                    command,
                    image.clone(),
                    queue,
                    texture_range(image)?,
                    sampled,
                    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                )?;
            }
            for buffer in [&frame.globals, &frame.primitives, &frame.shapes] {
                buffer_access(command, buffer.clone(), queue, 0, buffer.size, sampled)?;
            }
            image_access(
                command,
                frame.target.clone(),
                queue,
                texture_range(&frame.target)?,
                AccessScope {
                    stages: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                    access: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                    write: true,
                },
                vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            )?;
        }
        unsafe {
            frame.record_draw(command, &self.device, vertex_count)?;
        }
        unsafe {
            frame.record_readback(command, &self.device, queue, length)?;
        }
        Ok(())
    }
}
impl PreparedFrame {
    unsafe fn record_draw(
        &self,
        command: &mut VulkanCommandBuffer,
        device: &Arc<Device>,
        vertex_count: u32,
    ) -> GpuResult<()> {
        self.attachments.retain_for_command(command)?;
        {
            let mut backend = unsafe { AshCommandBackend::new_for_device(device) };
            let mut api = RenderPassApi::new(&mut backend, command.handle, 0, false);
            api.begin_prepared(&self.attachments)?;
            api.set_default_view_and_scissor(self.descriptor.extent_2d());
            unsafe {
                self.pipeline.bind_pipeline(command.handle);
                self.parameters
                    .bind_command(command, self.pipeline.clone())?;
                device
                    .device
                    .cmd_draw(command.handle, vertex_count, 1, 0, 0);
            }
            api.end_render_pass();
        }
        Ok(())
    }
    unsafe fn record_readback(
        &self,
        command: &mut VulkanCommandBuffer,
        device: &Arc<Device>,
        queue: rhapsody::sync::QueueId,
        length: u64,
    ) -> GpuResult<()> {
        unsafe {
            image_access(
                command,
                self.target.clone(),
                queue,
                texture_range(&self.target)?,
                AccessScope {
                    stages: vk::PipelineStageFlags::TRANSFER,
                    access: vk::AccessFlags::TRANSFER_READ,
                    write: false,
                },
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            )?;
            buffer_access(
                command,
                self.readback.clone(),
                queue,
                0,
                length,
                AccessScope {
                    stages: vk::PipelineStageFlags::TRANSFER,
                    access: vk::AccessFlags::TRANSFER_WRITE,
                    write: true,
                },
            )?;
            device.device.cmd_copy_image_to_buffer(
                command.handle,
                self.target.handle,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                self.readback.handle,
                &[vk::BufferImageCopy::default()
                    .image_subresource(
                        vk::ImageSubresourceLayers::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .layer_count(1),
                    )
                    .image_extent(self.descriptor.extent())],
            );
            buffer_access(
                command,
                self.readback.clone(),
                queue,
                0,
                length,
                AccessScope {
                    stages: vk::PipelineStageFlags::HOST,
                    access: vk::AccessFlags::HOST_READ,
                    write: false,
                },
            )?;
        }

        Ok(())
    }
}
