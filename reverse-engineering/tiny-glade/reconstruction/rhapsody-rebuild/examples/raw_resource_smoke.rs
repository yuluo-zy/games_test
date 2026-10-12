//! 真正调用驱动验证资源子系统；不声称验证原作画面或资源池策略。
use ash::vk;
use rhapsody::raw::{
    buffer::{Buffer, BufferDesc},
    framebuffer::Framebuffer,
    init::create_headless_device,
    render_pass::RenderPass,
    sampler::Sampler,
    texture::{Texture, TextureDesc, TextureViewDesc},
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
    let desc = BufferDesc::new_cpu_to_gpu(256, vk::BufferUsageFlags::TRANSFER_SRC);
    let buffer = Buffer::new(Arc::clone(&device), &desc)?;
    let input: Vec<_> = (0..128).map(|i| (i * 17) as u8).collect();
    buffer.write_bytes(64, &input)?;
    let mut readback = vec![0; input.len()];
    buffer.read_bytes(64, &mut readback)?;
    assert_eq!(input, readback);
    assert!(buffer.write_bytes(255, &[1, 2]).is_err());
    assert!(
        Buffer::new(
            Arc::clone(&device),
            &BufferDesc::new_cpu_to_gpu(0, vk::BufferUsageFlags::TRANSFER_SRC)
        )
        .is_err()
    );

    let desc = TextureDesc::new_2d(16, 16, vk::Format::R8G8B8A8_UNORM).color_attachment();
    let texture = Arc::new(Texture::new(Arc::clone(&device), &desc)?);
    let view = Arc::new(texture.image_view(&TextureViewDesc::new(&desc))?);
    let mut invalid = TextureViewDesc::new(&desc);
    invalid.level_count = 2;
    assert!(texture.image_view(&invalid).is_err());
    let attachments = [vk::AttachmentDescription::default()
        .format(desc.format)
        .samples(vk::SampleCountFlags::TYPE_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
    let references = [vk::AttachmentReference::default()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
    let subpasses = [vk::SubpassDescription::default()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(&references)];
    let info = vk::RenderPassCreateInfo::default()
        .attachments(&attachments)
        .subpasses(&subpasses);
    let render_pass = Arc::new(RenderPass::new(Arc::clone(&device), &info)?);
    let framebuffer = Framebuffer::new(
        Arc::clone(&render_pass),
        vec![Arc::clone(&view)],
        desc.extent_2d(),
        1,
    )?;
    let info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::NEAREST)
        .min_filter(vk::Filter::NEAREST)
        .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
        .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE);
    let sampler = Sampler::new(Arc::clone(&device), &info)?;
    drop(sampler);
    drop(framebuffer);
    drop(render_pass);
    drop(view);
    drop(texture);
    drop(buffer);
    device.wait_idle()?;
    println!(
        "{}",
        json!({"status":"passed", "device":name, "host_readback_bytes":input.len(),
        "created_and_destroyed":["buffer","image","image_view","render_pass","framebuffer","sampler"],
        "invalid_ranges_rejected":true, "swapchain_tested":false, "gpu_rendering_tested":false})
    );
    Ok(())
}
