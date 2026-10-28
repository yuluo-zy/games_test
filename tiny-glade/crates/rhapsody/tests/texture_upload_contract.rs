mod gpu_test_support;
use ash::vk;
use rhapsody::device::{Device, DeviceConfig};
use rhapsody::resource::buffer::{Buffer, BufferDesc};
use rhapsody::resource::texture::{Texture, TextureDesc, TextureKind, TextureViewDesc};
use rhapsody::transfer::setup::{SETUP_WAIT_TIMEOUT_NS, SetupAccess, init_texture_data, with_setup_resources};
use rhapsody::command::texture_copy::{copy_format_params, linear_regions, plan_texture_copies};

#[test]
fn default_views_match_texture_dimensions_and_create_real_vulkan_views() {
    let descriptions = [
        (TextureKind::D1, vk::ImageViewType::TYPE_1D),
        (TextureKind::D1Array, vk::ImageViewType::TYPE_1D_ARRAY),
        (TextureKind::D2, vk::ImageViewType::TYPE_2D),
        (TextureKind::D2Array, vk::ImageViewType::TYPE_2D_ARRAY),
        (TextureKind::D3, vk::ImageViewType::TYPE_3D),
        (TextureKind::Cube, vk::ImageViewType::CUBE),
        (TextureKind::CubeArray, vk::ImageViewType::CUBE_ARRAY),
    ];
    let device = gpu_test_support::initialize(
        "default texture view dimensions",
        Device::new_headless(&DeviceConfig::headless()),
    );
    for (kind, expected) in descriptions {
        let mut desc = TextureDesc::new(
            vk::Extent3D {
                width: 4,
                height: if matches!(kind, TextureKind::D1 | TextureKind::D1Array) { 1 } else { 4 },
                depth: if kind == TextureKind::D3 { 4 } else { 1 },
            },
            kind,
            vk::Format::R8G8B8A8_UNORM,
        )
        .sampled();
        if kind.is_array() {
            desc = desc.array_elements(2).unwrap();
        }
        let view = TextureViewDesc::new(&desc).unwrap();
        assert_eq!(view.view_type, expected);
        let Some(device) = &device else {
            continue;
        };
        let texture = std::sync::Arc::new(Texture::new(device.clone(), &desc).unwrap());
        let result = texture.image_view(&view);
        if kind == TextureKind::CubeArray
            && !device
                .caps()
                .is_enabled(rhapsody::device::Feature::ImageCubeArray)
        {
            assert_eq!(
                result.err().unwrap().kind(),
                rhapsody::ErrorKind::Unsupported
            );
        } else {
            assert!(result.is_ok(), "{kind:?}: {:?}", result.err());
        }
    }
}

#[test]
fn every_bc_variant_and_odd_uncompressed_mip_has_a_valid_copy_offset() {
    for code in 131..=146 {
        let expected = if (131..=134).contains(&code) || (139..=140).contains(&code) {
            8
        } else {
            16
        };
        assert_eq!(
            copy_format_params(vk::Format::from_raw(code)).unwrap(),
            (expected, vk::ImageAspectFlags::COLOR)
        );
    }
    let data = [0u8; 18];
    let regions = linear_regions(&data, [5, 3, 1], 3, 1, 1, [1, 1]).unwrap();
    let copies = plan_texture_copies(&regions, 1, vk::ImageAspectFlags::COLOR).unwrap();
    assert_eq!(
        copies
            .copies
            .iter()
            .map(|c| c.buffer_offset)
            .collect::<Vec<_>>(),
        [0, 16, 20]
    );
    assert_eq!(copies.total, 21);
    assert!(copy_format_params(vk::Format::D24_UNORM_S8_UINT).is_err());
    assert!(copy_format_params(vk::Format::G8_B8_R8_3PLANE_420_UNORM).is_err());
}
#[test]
fn cube_faces_cube_array_faces_and_odd_r8_mips_upload_every_subresource() {
    let Some(device) = gpu_test_support::initialize(
        "full texture upload",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    for (kind, elements, width, height, format, texel_bytes) in [
        (TextureKind::Cube, 1, 4, 4, vk::Format::R8G8B8A8_UNORM, 4u32),
        (
            TextureKind::CubeArray,
            2,
            4,
            4,
            vk::Format::R8G8B8A8_UNORM,
            4,
        ),
        (TextureKind::D2Array, 3, 5, 3, vk::Format::R8_UNORM, 1),
    ] {
        let mut desc = TextureDesc::new(
            vk::Extent3D {
                width,
                height,
                depth: 1,
            },
            kind,
            format,
        )
        .mip_levels(3)
        .usage(
            vk::ImageUsageFlags::SAMPLED
                | vk::ImageUsageFlags::TRANSFER_SRC
                | vk::ImageUsageFlags::TRANSFER_DST,
        );
        desc.array_elements = elements;
        let layers = desc.array_layers().unwrap();
        let mut data = Vec::new();
        for layer in 0..layers {
            for mip in 0..3 {
                let size = (width >> mip).max(1) * (height >> mip).max(1) * texel_bytes;
                data.extend((0..size).map(|i| ((layer * 31 + mip * 7 + i) % 251) as u8));
            }
        }
        let image = Texture::new_with_initial_data(device.clone(), &desc, true).unwrap();
        init_texture_data(&device, &image, &data).unwrap();
        let regions =
            linear_regions(&data, [width, height, 1], 3, layers, texel_bytes, [1, 1]).unwrap();
        assert_eq!(regions.len(), layers as usize * 3);
        let plan = plan_texture_copies(&regions, texel_bytes, vk::ImageAspectFlags::COLOR).unwrap();
        let readback = Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(plan.total, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap();
        let accesses = [
            SetupAccess::Image {
                texture: &image,
                range: None,
                scope: SetupAccess::TRANSFER_READ,
                layout: vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            },
            SetupAccess::transfer_write(&readback, 0, plan.total),
        ];
        with_setup_resources(&device, &accesses, |command| unsafe {
            device.raw_device().cmd_copy_image_to_buffer(
                command,
                image.raw_handle(),
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                readback.raw_handle(),
                &plan.copies,
            );
            let host = vk::BufferMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .buffer(readback.raw_handle())
                .size(vk::WHOLE_SIZE);
            device.raw_device().cmd_pipeline_barrier(
                command,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::HOST,
                vk::DependencyFlags::empty(),
                &[],
                &[host],
                &[],
            );
        })
        .unwrap()
        .wait(SETUP_WAIT_TIMEOUT_NS)
        .unwrap();
        let mut actual = vec![0; plan.total as usize];
        unsafe { readback.read_bytes_after_completion(0, &mut actual) }.unwrap();
        for (region, copy) in regions.iter().zip(&plan.copies) {
            let start = copy.buffer_offset as usize;
            assert_eq!(
                &actual[start..start + region.data.len()],
                region.data,
                "kind={kind:?} layer={} mip={}",
                region.array_layer,
                region.mip_level
            );
        }
    }
}

#[test]
fn recorded_uploads_share_virtual_state_and_graph_import_uses_physical_final_layout() {
    use ash::vk::Handle;
    use rhapsody::rg::vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession};
    use rhapsody::tracking::OwnershipRange;
    use rhapsody::command::resource_access::{buffer_access, image_access};
    use rhapsody::transfer::setup::with_setup_command;
    use rhapsody::command::texture_copy::{record_texture_regions, record_texture_zero_fill};
    use rhapsody::native::AccessScope;
    use std::sync::Arc;
    let Some(device) = gpu_test_support::initialize(
        "recorded upload range state",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let texture = Arc::new(
        Texture::new(
            device.clone(),
            &TextureDesc::new_2d(8, 8, vk::Format::R8G8B8A8_UNORM).usage(
                vk::ImageUsageFlags::SAMPLED
                    | vk::ImageUsageFlags::TRANSFER_DST
                    | vk::ImageUsageFlags::TRANSFER_SRC,
            ),
        )
        .unwrap(),
    );
    let staging = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(272, vk::BufferUsageFlags::TRANSFER_SRC),
        )
        .unwrap(),
    );
    let mut source = [0u8; 272];
    for pixel in source[..256].as_chunks_mut::<4>().0 {
        *pixel = [13, 31, 57, 255];
    }
    for pixel in source[256..].as_chunks_mut::<4>().0 {
        *pixel = [191, 17, 211, 255];
    }
    staging.write_bytes(0, &source).unwrap();
    let readback = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(256, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let queue = device
        .queue(rhapsody::device::QueueClass::Graphics)
        .unwrap();
    let range = OwnershipRange::Image {
        aspects: 1,
        base_mip: 0,
        mip_count: 1,
        base_layer: 0,
        layer_count: 1,
    };
    let transfer_read = AccessScope {
        stages: vk::PipelineStageFlags::TRANSFER,
        access: vk::AccessFlags::TRANSFER_READ,
        write: false,
    };
    let transfer_write = AccessScope {
        stages: vk::PipelineStageFlags::TRANSFER,
        access: vk::AccessFlags::TRANSFER_WRITE,
        write: true,
    };
    let layers = vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .layer_count(1);
    with_setup_command(&device, &[], |command| unsafe {
        record_texture_zero_fill(command, &texture, queue)?;
        record_texture_regions(
            command,
            queue,
            &texture,
            &staging,
            &[vk::BufferImageCopy::default()
                .image_subresource(layers)
                .image_extent(vk::Extent3D {
                    width: 8,
                    height: 8,
                    depth: 1,
                })],
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )?;
        record_texture_regions(
            command,
            queue,
            &texture,
            &staging,
            &[vk::BufferImageCopy::default()
                .buffer_offset(256)
                .image_subresource(layers)
                .image_offset(vk::Offset3D { x: 2, y: 3, z: 0 })
                .image_extent(vk::Extent3D {
                    width: 2,
                    height: 2,
                    depth: 1,
                })],
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )?;
        image_access(
            command,
            &texture,
            queue,
            range,
            transfer_read,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        )?;
        buffer_access(command, &readback, queue, 0, 256, transfer_write)?;
        device.raw_device().cmd_copy_image_to_buffer(
            command.raw_handle(),
            texture.raw_handle(),
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            readback.raw_handle(),
            &[vk::BufferImageCopy::default()
                .image_subresource(layers)
                .image_extent(texture.desc.extent())],
        );
        image_access(
            command,
            &texture,
            queue,
            range,
            AccessScope {
                stages: vk::PipelineStageFlags::ALL_COMMANDS,
                access: vk::AccessFlags::SHADER_READ,
                write: false,
            },
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )?;
        buffer_access(
            command,
            &readback,
            queue,
            0,
            256,
            AccessScope {
                stages: vk::PipelineStageFlags::HOST,
                access: vk::AccessFlags::HOST_READ,
                write: false,
            },
        )
    })
    .unwrap()
    .wait(rhapsody::transfer::setup::SETUP_WAIT_TIMEOUT_NS)
    .unwrap();
    let mut expected = [0u8; 256];
    for pixel in expected.as_chunks_mut::<4>().0 {
        *pixel = [13, 31, 57, 255];
    }
    for y in 3..5 {
        for x in 2..4 {
            expected[(y * 8 + x) * 4..(y * 8 + x) * 4 + 4].copy_from_slice(&[191, 17, 211, 255]);
        }
    }
    let mut actual = [0u8; 256];
    readback.read_bytes(0, &mut actual).unwrap();
    assert_eq!(actual, expected);

    let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    backend
        .bind_import_named("uploaded", (1u64 << 62) | 17, GpuResource::Image(texture.clone()))
        .unwrap();
    backend
        .bind_import_named("readback", (1u64 << 62) | 18, GpuResource::Buffer(readback.clone()))
        .unwrap();
    let mut session = VulkanGraphSession::new(backend);
    for _ in 0..3 {
        let mut graph = session.begin_frame().unwrap();
        // This obsolete scalar deliberately disagrees with physical state.
        // It may describe initialized contents but cannot rewrite their layout.
        let input = graph
            .import_unowned(
                "uploaded",
                ResourceSpec::Image(texture.desc).descriptor(),
                (1u64 << 62) | 17,
                0x34,
            )
            .unwrap();
        let output = graph
            .import_unowned(
                "readback",
                ResourceSpec::Buffer(readback.desc).descriptor(),
                (1u64 << 62) | 18,
                0x26,
            )
            .unwrap();
        let mut pass = graph.add_pass("read uploaded physical state");
        pass.add_resource_access(input, 0x25, 0).unwrap();
        pass.add_resource_access(output, 0x34, 0).unwrap();
        let image = texture.clone();
        let buffer = readback.clone();
        let device = device.clone();
        pass.render_with_result_context(move |context| {
            unsafe {
                device.raw_device().cmd_copy_image_to_buffer(
                    vk::CommandBuffer::from_raw(context.command.handle()),
                    image.raw_handle(),
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    buffer.raw_handle(),
                    &[vk::BufferImageCopy::default()
                        .image_subresource(layers)
                        .image_extent(image.desc.extent())],
                );
            }
            Ok(())
        });
        graph.export(input, 0x22).unwrap();
        graph.export(output, 0x26).unwrap();
        let receipt = session.execute_frame(graph.build()).unwrap();
        session.wait_frame(receipt.frame, 10_000_000_000).unwrap();
        readback.read_bytes(0, &mut actual).unwrap();
        assert_eq!(actual, expected);
    }
    session.wait_submits_done(10_000_000_000).unwrap();
}
