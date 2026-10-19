//! `separateDepthStencilLayouts` is enabled when supported and drives the
//! render-graph planner's per-aspect depth/stencil layouts on a real device.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::{
    device::{Device, DeviceConfig, Feature},
    raw::{
        buffer::{Buffer, BufferDesc},
        texture::TextureDesc,
    },
    resource::OwnershipRange,
    rg::{
        executor::PassContext,
        resource::AccessRange,
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::Arc;

const TRANSFER_WRITE: u8 = 0x34;
const TRANSFER_READ: u8 = 0x25;
const DEPTH_WRITE: u8 = 0x2f;
const DEPTH_STENCIL_READ: u8 = 0x17;
const HOST: u8 = 0x26;
const SIZE: u32 = 4;
const TIMEOUT: u64 = 10_000_000_000;

fn aspect(aspect: vk::ImageAspectFlags) -> AccessRange {
    AccessRange::Texture {
        aspect_mask: aspect.as_raw(),
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    }
}

#[test]
fn separate_depth_stencil_layouts_are_enabled_when_supported_and_used_per_aspect() {
    let Some(device) = gpu_test_support::initialize(
        "separate depth stencil layouts",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let caps = device.caps();
    // Optional feature: enabled exactly when the physical device supports it.
    assert_eq!(
        caps.is_enabled(Feature::SeparateDepthStencilLayouts),
        caps.is_supported(Feature::SeparateDepthStencilLayouts)
    );
    let separate = caps.is_enabled(Feature::SeparateDepthStencilLayouts);
    eprintln!("separateDepthStencilLayouts enabled: {separate}");
    let format = vk::Format::D32_SFLOAT_S8_UINT;
    let readback = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(
                u64::from(SIZE * SIZE * 4),
                vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
        .unwrap(),
    );
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    let mut graph = session.begin_frame().unwrap();
    let image = session
        .backend
        .declare(
            &mut graph,
            "depth stencil",
            ResourceSpec::Image(TextureDesc::new_2d(SIZE, SIZE, format).usage(
                vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
                    | vk::ImageUsageFlags::TRANSFER_DST
                    | vk::ImageUsageFlags::TRANSFER_SRC,
            )),
        )
        .unwrap();
    let image_index = image.raw();
    let token = session
        .backend
        .bind_import(GpuResource::Buffer(readback.clone()))
        .unwrap();
    let out = graph
        .import_unowned(
            "out",
            ResourceSpec::Buffer(readback.desc).descriptor(),
            token,
            HOST,
        )
        .unwrap();
    let out_index = out.raw();
    let texture = graph.texture_id(image).unwrap();
    let raw = session.backend.device.clone();
    let mut clear = graph.add_pass("clear depth").with_side_effect();
    let texture = clear
        .write_texture(
            texture,
            TRANSFER_WRITE,
            aspect(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL),
        )
        .unwrap();
    clear.render_with_result_context(move |ctx: PassContext<'_>| {
        let image = ctx
            .managed
            .ok_or_else(|| rhapsody::Error::invalid_usage("missing managed resources"))?
            .texture_for(image_index)
            .map_err(|e| rhapsody::Error::invalid_usage_with("binding", e))?;
        unsafe {
            raw.device.cmd_clear_depth_stencil_image(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                image.raw(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearDepthStencilValue {
                    depth: 0.5,
                    stencil: 7,
                },
                &[vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL)
                    .level_count(1)
                    .layer_count(1)],
            );
        }
        Ok(())
    });
    // Attachment-style uses without draws: only their barriers execute.
    let mut depth = graph.add_pass("depth attachment").with_side_effect();
    let texture = depth
        .write_texture(texture, DEPTH_WRITE, aspect(vk::ImageAspectFlags::DEPTH))
        .unwrap();
    depth.render(|_| true);
    let mut stencil = graph.add_pass("stencil read").with_side_effect();
    stencil
        .read_texture(
            texture,
            DEPTH_STENCIL_READ,
            aspect(vk::ImageAspectFlags::STENCIL),
        )
        .unwrap();
    stencil.render(|_| true);
    let mut copy = graph.add_pass("copy depth").with_side_effect();
    copy.read_texture(texture, TRANSFER_READ, aspect(vk::ImageAspectFlags::DEPTH))
        .unwrap();
    copy.add_resource_access(out, TRANSFER_WRITE, 0).unwrap();
    let raw = session.backend.device.clone();
    copy.render_with_result_context(move |ctx: PassContext<'_>| {
        let managed = ctx
            .managed
            .ok_or_else(|| rhapsody::Error::invalid_usage("missing managed resources"))?;
        let image = managed
            .texture_for(image_index)
            .map_err(|e| rhapsody::Error::invalid_usage_with("binding", e))?;
        let output = managed
            .buffer_for(out_index)
            .map_err(|e| rhapsody::Error::invalid_usage_with("binding", e))?;
        unsafe {
            raw.device.cmd_copy_image_to_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                image.raw(),
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                output.raw(),
                &[vk::BufferImageCopy {
                    image_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::DEPTH,
                        mip_level: 0,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    image_extent: vk::Extent3D {
                        width: SIZE,
                        height: SIZE,
                        depth: 1,
                    },
                    ..Default::default()
                }],
            );
        }
        Ok(())
    });
    graph.export(out, HOST).unwrap();
    session.execute_frame(graph.build()).unwrap();
    let (depth_bit, stencil_bit) = (
        vk::ImageAspectFlags::DEPTH.as_raw(),
        vk::ImageAspectFlags::STENCIL.as_raw(),
    );
    let layouts = session
        .last_synchronization()
        .transitions()
        .iter()
        .filter(|transition| transition.resource == image_index)
        .filter_map(|transition| match transition.range {
            OwnershipRange::Image { aspects, .. } => Some((aspects, transition.new_layout)),
            _ => None,
        })
        .collect::<Vec<_>>();
    if separate {
        assert!(layouts.contains(&(depth_bit, vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)));
        assert!(layouts.contains(&(stencil_bit, vk::ImageLayout::STENCIL_READ_ONLY_OPTIMAL)));
        // Only the initial whole-image clear names both aspects.
        assert!(
            layouts
                .iter()
                .all(|&(aspects, layout)| aspects != depth_bit | stencil_bit
                    || layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        );
    } else {
        assert!(layouts.iter().all(|&(aspects, layout)| {
            aspects == depth_bit | stencil_bit
                && !matches!(
                    layout,
                    vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL
                        | vk::ImageLayout::STENCIL_READ_ONLY_OPTIMAL
                )
        }));
    }
    session.wait_submits_done(TIMEOUT).unwrap();
    let mut bytes = vec![0; (SIZE * SIZE * 4) as usize];
    unsafe { readback.read_bytes_after_completion(0, &mut bytes).unwrap() };
    for value in bytes.as_chunks::<4>().0 {
        assert_eq!(f32::from_le_bytes(*value), 0.5);
    }
}
