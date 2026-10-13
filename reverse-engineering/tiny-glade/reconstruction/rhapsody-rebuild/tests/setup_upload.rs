//! setup 命令缓冲与纹理上传：上传 → 回拷读回 → 逐字节比较。无 Vulkan 设备时跳过。
use ash::vk;
use rhapsody::raw::{
    buffer::{Buffer, BufferDesc},
    device::VulkanDevice,
    init::create_headless_device,
    setup::{init_texture_data, update_texture, with_setup_cb},
    texture::{Texture, TextureDesc},
};
use std::sync::Arc;

fn device() -> Option<Arc<VulkanDevice>> {
    match create_headless_device() {
        Ok(d) => Some(d),
        Err(e) => {
            eprintln!("跳过：无可用 Vulkan 设备（{e:?}）");
            None
        }
    }
}
fn texture(device: &Arc<VulkanDevice>, w: u32, h: u32) -> Texture {
    let mut desc = TextureDesc::new_2d(w, h, vk::Format::R8G8B8A8_UNORM);
    desc.usage = vk::ImageUsageFlags::TRANSFER_DST
        | vk::ImageUsageFlags::TRANSFER_SRC
        | vk::ImageUsageFlags::SAMPLED;
    Texture::new(Arc::clone(device), &desc).unwrap()
}
/// 把纹理（SHADER_READ_ONLY_OPTIMAL）回拷到宿主缓冲。
fn readback(device: &Arc<VulkanDevice>, tex: &Texture, len: usize) -> Vec<u8> {
    let dst = Buffer::new(
        Arc::clone(device),
        &BufferDesc::new_gpu_to_cpu(len as u64, vk::BufferUsageFlags::TRANSFER_DST),
    )
    .unwrap();
    with_setup_cb(device, |cb| unsafe {
        let region = [vk::BufferImageCopy::default()
            .image_subresource(
                vk::ImageSubresourceLayers::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .layer_count(1),
            )
            .image_extent(tex.desc.extent)];
        device.device.cmd_copy_image_to_buffer(
            cb,
            tex.handle,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            dst.handle,
            &region,
        );
    })
    .unwrap();
    let mut out = vec![0u8; len];
    dst.read_bytes(0, &mut out).unwrap();
    out
}
fn pattern(len: usize, seed: u8) -> Vec<u8> {
    (0..len).map(|i| (i as u8).wrapping_mul(31).wrapping_add(seed)).collect()
}

#[test]
fn init_then_update_round_trips_exact_bytes() {
    let Some(device) = device() else { return };
    let tex = texture(&device, 8, 4);
    let first = pattern(8 * 4 * 4, 1);
    init_texture_data(&device, &tex, &first).unwrap();
    assert_eq!(readback(&device, &tex, first.len()), first);
    let second = pattern(8 * 4 * 4, 77);
    update_texture(&device, &tex, &second).unwrap();
    assert_eq!(readback(&device, &tex, second.len()), second);
}

#[test]
fn rejects_wrong_length_and_missing_usage() {
    let Some(device) = device() else { return };
    let tex = texture(&device, 4, 4);
    assert!(init_texture_data(&device, &tex, &[0u8; 3]).is_err());
    let mut desc = TextureDesc::new_2d(4, 4, vk::Format::R8G8B8A8_UNORM);
    desc.usage = vk::ImageUsageFlags::SAMPLED;
    let no_dst = Texture::new(Arc::clone(&device), &desc).unwrap();
    assert!(init_texture_data(&device, &no_dst, &[0u8; 64]).is_err());
}

#[test]
fn with_setup_cb_returns_callback_value_after_completion() {
    let Some(device) = device() else { return };
    assert_eq!(with_setup_cb(&device, |_| 42).unwrap(), 42);
}
