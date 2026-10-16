//! 句柄适配层（`GraphicsDevice::create_buffer` 等）的冒烟验证。无 Vulkan 设备时跳过；本机可用 lavapipe。
// 区间参数刻意传单元素切片，对应 update_buffer_bytes 的区间列表。
#![allow(clippy::single_range_in_vec_init)]
use ash::vk;
use rhapsody::raw::{buffer::BufferDesc, device::GraphicsDevice, init::create_headless_device};

fn device() -> Option<std::sync::Arc<GraphicsDevice>> {
    create_headless_device()
        .ok()
        .map(GraphicsDevice::from_device)
}

#[test]
fn create_buffer_uploads_initial_data_and_update_writes_ranges() {
    let Some(gd) = device() else { return };
    let desc = BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::STORAGE_BUFFER);
    let init: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
    let buffer = gd.create_buffer(&desc, "t", init).unwrap();
    assert_ne!(gd.get_vk_buffer(&buffer).unwrap(), vk::Buffer::null());
    // 有数据时自动带 TRANSFER_DST。
    assert!(buffer.usage.contains(vk::BufferUsageFlags::TRANSFER_DST));
    gd.update_buffer_bytes(&buffer, &[9, 9, 0, 0, 7, 7], &[0..2, 4..6], 8)
        .unwrap();
    let mut out = [0u8; 16];
    buffer.read_bytes(0, &mut out).unwrap();
    assert_eq!(&out[..8], init);
    assert_eq!(&out[8..10], &[9, 9]);
    assert_eq!(&out[12..14], &[7, 7]);
    // 越界不部分写入。
    assert!(
        gd.update_buffer_bytes(&buffer, &[1; 4], &[0..4], 14)
            .is_err()
    );
}

#[test]
fn gpu_only_buffer_goes_through_staging_and_foreign_handles_are_rejected() {
    let (Some(a), Some(b)) = (device(), device()) else {
        return;
    };
    let desc = BufferDesc::new_gpu_only(64, vk::BufferUsageFlags::STORAGE_BUFFER);
    let data = vec![0xabu8; 64];
    let buffer = a.create_buffer(&desc, "gpu", data.as_slice()).unwrap();
    assert!(b.get_vk_buffer(&buffer).is_err());
    let none = a.create_buffer(&desc, "empty", ()).unwrap();
    assert_eq!(none.size, 64);
}

#[test]
fn create_render_pass_caches_by_description_and_framebuffer_checks_ownership() {
    use rhapsody::raw::{
        render_pass::RenderPassAttachmentDesc,
        texture::{TextureDesc, TextureViewDesc},
    };
    let (Some(a), Some(b)) = (device(), device()) else {
        return;
    };
    let color = RenderPassAttachmentDesc::new(vk::Format::R8G8B8A8_UNORM).clear_input();
    let first = a.create_render_pass(&[color]).unwrap();
    let again = a.create_render_pass(&[color]).unwrap();
    assert_eq!(first.raw(), again.raw(), "相同描述必须命中 +0x188 缓存");
    let other = a
        .create_render_pass(&[RenderPassAttachmentDesc::new(vk::Format::R8G8B8A8_UNORM)])
        .unwrap();
    assert_ne!(first.raw(), other.raw(), "load 操作不同，不能共用通道");

    let desc = TextureDesc::new_2d(8, 8, vk::Format::R8G8B8A8_UNORM)
        .sampled()
        .color_attachment();
    let texture = a.create_texture(&desc, "fb", ()).unwrap();
    let view = a
        .get_vk_image_view(&texture, &TextureViewDesc::new(&desc))
        .unwrap();
    let fb = a
        .create_framebuffer(
            &first,
            vec![view.view.clone()],
            vk::Extent2D {
                width: 8,
                height: 8,
            },
            1,
        )
        .unwrap();
    assert_ne!(fb.raw(), vk::Framebuffer::null());
    // 别的设备的通道不能在本设备建帧缓冲。
    assert!(
        b.create_framebuffer(
            &first,
            vec![],
            vk::Extent2D {
                width: 8,
                height: 8
            },
            1
        )
        .is_err()
    );
}
