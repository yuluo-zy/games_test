//! 句柄适配层（`Device::create_buffer` 等）的冒烟验证。无 Vulkan 设备时跳过；本机可用 lavapipe。
// 区间参数刻意传单元素切片，对应 update_buffer_bytes 的区间列表。
#![allow(clippy::single_range_in_vec_init)]
mod gpu_test_support;
use ash::vk;
use rhapsody::raw::{buffer::BufferDesc, device::Device, init::create_headless_device};

fn device() -> Option<std::sync::Arc<Device>> {
    gpu_test_support::initialize("device API", create_headless_device())
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
fn dynamic_attachment_views_are_cached_and_reject_foreign_devices() {
    use rhapsody::{
        raw::texture::{Texture, TextureDesc},
        rg::attachments::{Attachment, AttachmentCache, PreparedAttachments, RenderingPolicy},
    };
    use std::sync::Arc;
    let (Some(a), Some(b)) = (device(), device()) else {
        return;
    };
    let desc = TextureDesc::new_2d(8, 8, vk::Format::R8G8B8A8_UNORM).color_attachment();
    let texture = Arc::new(Texture::new(a.clone(), &desc).unwrap());
    let extent = vk::Extent2D {
        width: 8,
        height: 8,
    };
    let mut cache = AttachmentCache::new(a).unwrap();
    let first = cache
        .prepare_for_rendering(
            &[Attachment::color(texture.clone(), 0, Some([0., 0., 0., 1.])).unwrap()],
            extent,
            1,
            RenderingPolicy::RequireDynamic,
        )
        .unwrap();
    let again = cache
        .prepare_for_rendering(
            &[Attachment::color(texture.clone(), 0, Some([1., 0., 0., 1.])).unwrap()],
            extent,
            1,
            RenderingPolicy::RequireDynamic,
        )
        .unwrap();
    let (PreparedAttachments::Dynamic(first), PreparedAttachments::Dynamic(again)) =
        (&*first, &*again);
    assert!(Arc::ptr_eq(&first.views[0], &again.views[0]));
    assert_eq!(cache.view_count(), 1);
    let mut foreign = AttachmentCache::new(b).unwrap();
    assert!(
        foreign
            .prepare_for_rendering(
                &[Attachment::color(texture, 0, None).unwrap()],
                extent,
                1,
                RenderingPolicy::RequireDynamic
            )
            .is_err()
    );
    cache.clear();
    assert_eq!(cache.view_count(), 0);
    assert_ne!(first.views[0].handle, vk::ImageView::null());
}

#[test]
fn real_attachment_storage_is_budgeted_retained_and_applies_owner_backpressure() {
    use rhapsody::{
        raw::texture::{Texture, TextureDesc},
        rg::attachments::{Attachment, AttachmentCache, RenderingPolicy},
    };
    use std::sync::Arc;
    let Some(device) = device() else { return };
    let texture = Arc::new(
        Texture::new(
            device.clone(),
            &TextureDesc::new_2d(8, 8, vk::Format::R8G8B8A8_UNORM).color_attachment(),
        )
        .unwrap(),
    );
    let baseline = device.allocator_stats().cpu_memory_usage().total();
    let mut cache = AttachmentCache::with_capacity(device.clone(), 1).unwrap();
    let attachment = Attachment::color(texture.clone(), 0, Some([0.; 4])).unwrap();
    let extent = vk::Extent2D {
        width: 8,
        height: 8,
    };
    device
        .allocator_stats()
        .set_host_budget(device.allocator_stats().cpu_memory_usage().total().max(1))
        .unwrap();
    let error = cache
        .prepare_for_rendering(
            std::slice::from_ref(&attachment),
            extent,
            1,
            RenderingPolicy::RequireDynamic,
        )
        .err()
        .expect("capacity growth must be admitted before allocation");
    assert_eq!(error.kind(), rhapsody::ErrorKind::OutOfMemory);
    assert_eq!(
        device.allocator_stats().cpu_memory_usage().total(),
        baseline
    );
    assert_eq!(cache.view_count(), 0);
    device.allocator_stats().set_total_budget(u64::MAX).unwrap();
    let first = cache
        .prepare_for_rendering(
            std::slice::from_ref(&attachment),
            extent,
            1,
            RenderingPolicy::RequireDynamic,
        )
        .unwrap();
    let mut next = attachment;
    next.generation = 1;
    assert!(matches!(
        cache.prepare_for_rendering(
            std::slice::from_ref(&next),
            extent,
            1,
            RenderingPolicy::RequireDynamic
        ),
        Err(rhapsody::Error::Timeout)
    ));
    drop(first);
    let prepared = cache
        .prepare_for_rendering(&[next], extent, 1, RenderingPolicy::RequireDynamic)
        .unwrap();
    cache.clear();
    drop(cache);
    assert!(
        device.allocator_stats().cpu_memory_usage().total() > baseline,
        "the prepared Arc and actual native view must retain their storage credit"
    );
    drop(prepared);
    assert_eq!(
        device.allocator_stats().cpu_memory_usage().total(),
        baseline
    );
}

#[test]
fn texture_descriptor_validation_rejects_invalid_fields_without_a_device() {
    use rhapsody::raw::texture::{TextureDesc, TextureKind};
    let valid = TextureDesc::new_2d(16, 16, vk::Format::R8G8B8A8_UNORM).sampled();
    assert!(valid.validate().is_ok());
    for invalid in [
        TextureDesc {
            usage: vk::ImageUsageFlags::empty(),
            ..valid
        },
        TextureDesc { width: 0, ..valid },
        TextureDesc { height: 0, ..valid },
        TextureDesc { depth: 0, ..valid },
        TextureDesc {
            mip_levels: 0,
            ..valid
        },
        TextureDesc {
            array_elements: 0,
            ..valid
        },
        TextureDesc {
            kind: TextureKind::CubeArray,
            array_elements: u32::MAX,
            ..valid
        },
    ] {
        assert_eq!(
            invalid.validate().unwrap_err().kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
    }
}

#[test]
fn pass_object_retention_cannot_bypass_memory_declarations_even_when_error_is_ignored() {
    use rhapsody::rg::{
        graph::RenderGraph,
        vulkan::{AccessTable, VulkanGraphBackend, VulkanGraphSession},
    };
    let Some(device) = device() else {
        return;
    };
    let buffer = std::sync::Arc::new(
        rhapsody::raw::buffer::Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::TRANSFER_SRC),
        )
        .unwrap(),
    );
    let mut session =
        VulkanGraphSession::new(VulkanGraphBackend::new(device, AccessTable::original()).unwrap());
    let mut graph = RenderGraph::new();
    let pass = graph
        .add_pass("reject undeclared object reference")
        .with_side_effect();
    let owner = buffer.clone();
    pass.render_with_result_context(move |context| {
        // Ignoring a registration error must not make this command executable.
        let _ = context.retain_object(owner);
        Ok(())
    });
    assert_eq!(
        session.execute_frame(graph.build()).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert!(session.backend.submissions.is_empty());
    rhapsody::resource::GpuUse::visit_uses(buffer.as_ref(), &mut |cell| {
        assert!(cell.snapshot().is_empty());
        Ok(())
    })
    .unwrap();
    let mut graph = session.begin_frame().unwrap();
    graph
        .add_pass("subsequent valid recording")
        .with_side_effect()
        .render_with_result_context(|_| Ok(()));
    session.execute_frame(graph.build()).unwrap();
    session.wait_submits_done(10_000_000_000).unwrap();
}

#[test]
fn prepared_clear_requires_an_actual_attachment_write_scope() {
    use rhapsody::{
        raw::{
            command_buffer::VulkanCommandPool,
            resource_access::image_access,
            texture::{Texture, TextureDesc},
        },
        resource::{OwnershipRange, range_state::AccessScope},
        rg::{
            attachments::{Attachment, AttachmentCache, RenderingPolicy},
            pass_api::{AshCommandBackend, RenderPassApi},
        },
    };
    let Some(device) = device() else {
        return;
    };
    let texture = std::sync::Arc::new(
        Texture::new(
            device.clone(),
            &TextureDesc::new_2d(4, 4, vk::Format::R8G8B8A8_UNORM).color_attachment(),
        )
        .unwrap(),
    );
    let prepared = AttachmentCache::new(device.clone())
        .unwrap()
        .prepare_for_rendering(
            &[Attachment::color(texture.clone(), 0, Some([0.0; 4])).unwrap()],
            vk::Extent2D {
                width: 4,
                height: 4,
            },
            1,
            RenderingPolicy::Auto,
        )
        .unwrap();
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let queue = device
        .queue(rhapsody::device::QueueClass::Graphics)
        .unwrap();
    let range = OwnershipRange::Image {
        aspects: vk::ImageAspectFlags::COLOR.as_raw(),
        base_mip: 0,
        mip_count: 1,
        base_layer: 0,
        layer_count: 1,
    };
    let mut read = pool.allocate().unwrap();
    read.begin().unwrap();
    unsafe {
        image_access(
            &mut read,
            texture.clone(),
            queue,
            range,
            AccessScope {
                stages: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                access: vk::AccessFlags::COLOR_ATTACHMENT_READ,
                write: false,
            },
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        )
        .unwrap();
    }
    assert!(matches!(
        prepared.retain_for_command(&mut read),
        Err(rhapsody::Error::InvalidUsage { .. })
    ));
    drop(read);
    let mut write = pool.allocate().unwrap();
    write.begin().unwrap();
    unsafe {
        image_access(
            &mut write,
            texture.clone(),
            queue,
            range,
            AccessScope {
                stages: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                access: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                write: true,
            },
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        )
        .unwrap();
    }
    prepared.retain_for_command(&mut write).unwrap();
    let mut backend = unsafe { AshCommandBackend::new_for_device(&device) };
    let mut pass = RenderPassApi::new(&mut backend, write.handle, 1, false);
    pass.begin_prepared(&prepared).unwrap();
    pass.end_render_pass();
    write.end().unwrap();
    write.submit().unwrap().wait(10_000_000_000).unwrap();
}
