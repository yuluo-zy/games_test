use ash::vk;
use rhapsody::raw::{
    buffer::{Buffer, BufferDesc},
    command_buffer::VulkanCommandPool,
    init::create_headless_device,
    texture::{Texture, TextureDesc},
    texture_upload::{record_texture_regions, validate_texture_regions},
};
use std::sync::Arc;
fn desc() -> TextureDesc {
    TextureDesc::new_2d(4, 2, vk::Format::R8G8B8A8_UNORM).usage(
        vk::ImageUsageFlags::SAMPLED
            | vk::ImageUsageFlags::TRANSFER_SRC
            | vk::ImageUsageFlags::TRANSFER_DST,
    )
}
fn region(width: u32, height: u32) -> vk::BufferImageCopy {
    vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width,
            height,
            depth: 1,
        })
}
fn valid(d: &TextureDesc, size: u64, r: &[vk::BufferImageCopy]) -> bool {
    validate_texture_regions(
        d,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        r,
        vk::ImageLayout::UNDEFINED,
    )
    .is_ok()
}
#[test]
fn validates_padded_rows_and_exact_last_texel_bound() {
    let mut r = region(2, 2);
    r.buffer_row_length = 3;
    r.buffer_image_height = 4;
    assert!(valid(&desc(), 20, &[r]));
    assert!(!valid(&desc(), 19, &[r]));
    r.buffer_offset = u64::MAX - 3;
    assert!(!valid(&desc(), u64::MAX, &[r]));
    r.buffer_offset = 2;
    assert!(!valid(&desc(), 32, &[r]));
}
#[test]
fn rejects_invalid_descriptor_subresource_extent_and_stride() {
    let good = region(2, 2);
    assert!(!valid(&desc(), 32, &[]));
    for d in [
        TextureDesc {
            mip_levels: 2,
            ..desc()
        },
        TextureDesc {
            array_layers: 2,
            ..desc()
        },
        TextureDesc {
            samples: vk::SampleCountFlags::TYPE_2,
            ..desc()
        },
        TextureDesc {
            format: vk::Format::R8_UNORM,
            ..desc()
        },
        TextureDesc {
            usage: vk::ImageUsageFlags::SAMPLED,
            ..desc()
        },
    ] {
        assert!(!valid(&d, 32, &[good]));
    }
    let mut cases = vec![];
    let mut r = good;
    r.image_subresource.aspect_mask = vk::ImageAspectFlags::DEPTH;
    cases.push(r);
    let mut r = good;
    r.image_subresource.mip_level = 1;
    cases.push(r);
    let mut r = good;
    r.image_subresource.base_array_layer = 1;
    cases.push(r);
    let mut r = good;
    r.image_subresource.layer_count = 2;
    cases.push(r);
    let mut r = good;
    r.image_extent.width = 0;
    cases.push(r);
    let mut r = good;
    r.image_extent.depth = 2;
    cases.push(r);
    let mut r = good;
    r.image_offset.x = -1;
    cases.push(r);
    let mut r = good;
    r.image_offset.x = 3;
    cases.push(r);
    let mut r = good;
    r.image_offset.z = 1;
    cases.push(r);
    let mut r = good;
    r.buffer_row_length = 1;
    cases.push(r);
    let mut r = good;
    r.buffer_image_height = 1;
    cases.push(r);
    let mut r = good;
    r.buffer_row_length = u32::MAX;
    cases.push(r);
    for r in cases {
        assert!(!valid(&desc(), u64::MAX, &[r]));
    }
    assert!(
        validate_texture_regions(
            &desc(),
            32,
            vk::BufferUsageFlags::TRANSFER_DST,
            &[good],
            vk::ImageLayout::UNDEFINED
        )
        .is_err()
    );
    assert!(
        validate_texture_regions(
            &desc(),
            32,
            vk::BufferUsageFlags::TRANSFER_SRC,
            &[good],
            vk::ImageLayout::GENERAL
        )
        .is_err()
    );
    assert!(valid(
        &TextureDesc {
            format: vk::Format::R8G8B8A8_SRGB,
            ..desc()
        },
        32,
        &[good]
    ));
}
#[test]
fn actual_gpu_batched_updates_roundtrip_with_transfer_src_layout() {
    let device = match create_headless_device() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("GPU_SKIPPED: {e}");
            return;
        }
    };
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name =
        unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
    eprintln!(
        "GPU_DEVICE: {name}, vendor={}, device={}, driver={}, queue_family={}",
        properties.vendor_id, properties.device_id, properties.driver_version, device.queue_family
    );
    let texture = Arc::new(Texture::new(device.clone(), &desc()).unwrap());
    let initial = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(32, vk::BufferUsageFlags::TRANSFER_SRC),
        )
        .unwrap(),
    );
    initial.write_bytes_unsynchronized(0, &[0; 32]).unwrap();
    let staging = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(36, vk::BufferUsageFlags::TRANSFER_SRC),
        )
        .unwrap(),
    );
    let bytes: Vec<u8> = (0..36).map(|i| (i * 7 + 1) as u8).collect();
    staging.write_bytes_unsynchronized(0, &bytes).unwrap();
    let readback = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(32, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut cb = pool.allocate().unwrap();
    assert!(
        unsafe {
            record_texture_regions(
                &mut cb,
                texture.clone(),
                initial.clone(),
                &[region(4, 2)],
                vk::ImageLayout::UNDEFINED,
            )
        }
        .is_err()
    );
    cb.begin().unwrap();
    unsafe {
        record_texture_regions(
            &mut cb,
            texture.clone(),
            initial.clone(),
            &[region(4, 2)],
            vk::ImageLayout::UNDEFINED,
        )
        .unwrap();
    }
    let mut left = region(2, 2);
    left.buffer_row_length = 3;
    left.buffer_image_height = 4;
    let mut right = region(2, 2);
    right.buffer_offset = 20;
    right.image_offset.x = 2;
    unsafe {
        record_texture_regions(
            &mut cb,
            texture.clone(),
            staging.clone(),
            &[left, right],
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )
        .unwrap();
    }
    assert!(Arc::strong_count(&staging) >= 2);
    drop(staging);
    drop(initial);
    // 回拷前显式切换 TRANSFER_SRC，避免在 shader-read 布局下执行 copy。
    let range = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .level_count(1)
        .layer_count(1);
    let barrier = |old, new, src, dst| {
        vk::ImageMemoryBarrier::default()
            .old_layout(old)
            .new_layout(new)
            .src_access_mask(src)
            .dst_access_mask(dst)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(texture.handle)
            .subresource_range(range)
    };
    unsafe {
        device.device.cmd_pipeline_barrier(
            cb.handle,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[barrier(
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                vk::AccessFlags::SHADER_READ,
                vk::AccessFlags::TRANSFER_READ,
            )],
        );
        device.device.cmd_copy_image_to_buffer(
            cb.handle,
            texture.handle,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            readback.handle,
            &[region(4, 2)],
        );
        device.device.cmd_pipeline_barrier(
            cb.handle,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[barrier(
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                vk::AccessFlags::TRANSFER_READ,
                vk::AccessFlags::SHADER_READ,
            )],
        );
        device.device.cmd_pipeline_barrier(
            cb.handle,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[vk::MemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)],
            &[],
            &[],
        );
    }
    cb.keep_alive(readback.clone());
    cb.end().unwrap();
    let pending = cb.submit().unwrap();
    pending.wait(u64::MAX).unwrap();
    let mut actual = [0; 32];
    let before_read = device.garbage_stats();
    unsafe {
        readback
            .read_bytes_after_completion(0, &mut actual)
            .unwrap();
    }
    let after_read = device.garbage_stats();
    assert_eq!(before_read.idle_waits, after_read.idle_waits);
    assert_eq!(before_read.completed_epoch, after_read.completed_epoch);
    let expected = [&bytes[0..8], &bytes[20..28], &bytes[12..20], &bytes[28..36]].concat();
    assert_eq!(actual.as_slice(), expected);
    eprintln!(
        "GPU_PASSED: two regions, padded rows, correct transfer layouts, byte-exact readback"
    );
}
