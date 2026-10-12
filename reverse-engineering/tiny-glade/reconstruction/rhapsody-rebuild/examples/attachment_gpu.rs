//! 真实驱动验证自动附件生产、清除、拷贝读回与缓存失效后的拥有权。
use ash::vk;
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        init::create_headless_device,
        texture::{Texture, TextureDesc},
    },
    rg::{
        attachments::{Attachment, AttachmentCache},
        pass_api::{AshCommandBackend, RenderPassApi},
    },
};
use serde_json::json;
use std::{ffi::CStr, sync::Arc};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = create_headless_device()?;
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
    let desc = TextureDesc::new_2d(8, 8, vk::Format::R8G8B8A8_UNORM)
        .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC);
    let image = Arc::new(Texture::new(device.clone(), &desc)?);
    let mut attachment = Attachment::color(image.clone(), 1, Some([1.0, 0.0, 0.5, 1.0]));
    attachment.initial_layout = vk::ImageLayout::UNDEFINED;
    let mut cache = AttachmentCache::new(device.clone());
    let prepared = cache.prepare(&[attachment.clone()], desc.extent_2d(), 1)?;
    let default_attachment = Attachment::color(image.clone(), 1, None);
    let default_prepared = cache.prepare(
        std::slice::from_ref(&default_attachment),
        desc.extent_2d(),
        1,
    )?;
    let begin_projections: Vec<_> = [&default_prepared, &prepared].iter().map(|prepared| {
        let info = prepared.begin_info();
        let clear_words: Vec<_> = prepared.clears.iter().flat_map(|value| unsafe { value.color.uint32 }).collect();
        json!({"extent":[info.render_area.extent.width,info.render_area.extent.height],"clear_count":info.clear_value_count,"clear_words":clear_words,"contents":0})
    }).collect();
    let reused = cache.prepare(&[attachment.clone()], desc.extent_2d(), 1)?;
    assert!(Arc::ptr_eq(&prepared.framebuffer, &reused.framebuffer));
    let mut changed_clear = attachment.clone();
    changed_clear.clear = Some(vk::ClearValue {
        color: vk::ClearColorValue {
            float32: [0.0, 1.0, 0.0, 1.0],
        },
    });
    let color_changed = cache.prepare(&[changed_clear], desc.extent_2d(), 1)?;
    assert!(Arc::ptr_eq(
        &prepared.framebuffer,
        &color_changed.framebuffer
    ));
    let cropped = cache.prepare(
        &[attachment.clone()],
        vk::Extent2D {
            width: 4,
            height: 4,
        },
        1,
    )?;
    assert!(!Arc::ptr_eq(&prepared.framebuffer, &cropped.framebuffer));
    let mut generation = attachment.clone();
    generation.generation = 2;
    let regenerated = cache.prepare(&[generation], desc.extent_2d(), 1)?;
    assert!(!Arc::ptr_eq(
        &prepared.framebuffer,
        &regenerated.framebuffer
    ));
    let before = cache.counts();
    let mut invalid = attachment.clone();
    invalid.view.base_mip_level = 1;
    assert!(cache.prepare(&[invalid], desc.extent_2d(), 1).is_err());
    assert_eq!(cache.counts(), before);
    let resized_desc = TextureDesc::new_2d(16, 8, desc.format).usage(desc.usage);
    let resized_image = Arc::new(Texture::new(device.clone(), &resized_desc)?);
    let mut resized_attachment =
        Attachment::color(resized_image.clone(), 1, Some([1.0, 0.0, 0.5, 1.0]));
    resized_attachment.initial_layout = vk::ImageLayout::UNDEFINED;
    let resized = cache.prepare(&[resized_attachment], resized_desc.extent_2d(), 1)?;
    assert!(!Arc::ptr_eq(&prepared.framebuffer, &resized.framebuffer));
    let counts = cache.counts();
    let readback = Arc::new(Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_to_cpu(256, vk::BufferUsageFlags::TRANSFER_DST),
    )?);
    let pool = VulkanCommandPool::new(device.clone())?;
    let mut command = pool.allocate()?;
    command.begin()?;
    prepared.keep_alive(&mut command);
    command.keep_alive(readback.clone());
    {
        let mut backend = unsafe { AshCommandBackend::new(&device.device) };
        let mut api = RenderPassApi::new(&mut backend, command.handle, 0, false);
        api.begin_attachments(&prepared);
        api.end_render_pass();
    }
    let range = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .level_count(1)
        .layer_count(1);
    let barrier = [vk::ImageMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
        .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
        .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image.handle)
        .subresource_range(range)];
    let copies = [vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
        .image_extent(desc.extent)];
    unsafe {
        device.device.cmd_pipeline_barrier(
            command.handle,
            vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &barrier,
        );
        device.device.cmd_copy_image_to_buffer(
            command.handle,
            image.handle,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            readback.handle,
            &copies,
        );
        let barrier = [vk::BufferMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::HOST_READ)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .buffer(readback.handle)
            .size(vk::WHOLE_SIZE)];
        device.device.cmd_pipeline_barrier(
            command.handle,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &barrier,
            &[],
        );
    }
    command.end()?;
    // 移除缓存和 CPU 产物后，尚未提交的命令仍持有整条依赖链。
    cache.invalidate_image(image.handle, 1);
    cache.clear();
    assert_eq!(cache.counts(), (0, 0, 0));
    drop(prepared);
    drop(default_prepared);
    drop(default_attachment);
    drop(reused);
    drop(color_changed);
    drop(cropped);
    drop(regenerated);
    drop(resized);
    drop(attachment);
    drop(image);
    drop(resized_image);
    let pending = command.submit()?;
    pending.wait(u64::MAX)?;
    let mut bytes = vec![0; 256];
    readback.read_bytes(0, &mut bytes)?;
    assert!(
        bytes
            .chunks_exact(4)
            .all(|p| p[0] == 255 && p[1] == 0 && (127..=128).contains(&p[2]) && p[3] == 255)
    );
    println!(
        "{}",
        json!({"status":"passed","device":name,"pixels_verified":64,"readback_bytes":256,"pixel":bytes[..4],"cache_counts_before_clear":counts,
        "same_attachment_reused":true,"clear_value_reused":true,"extent_changed":true,"image_resize_changed":true,"generation_changed":true,"failure_no_cache_mutation":true,"command_owns_after_invalidate":true,"begin_projections":begin_projections,"original_full_begin_equivalence":false})
    );
    Ok(())
}
