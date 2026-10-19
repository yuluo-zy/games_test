//! 真实宿主验证：纹理更新与清零回读，以及真实采样描述符租约。
//! RHAPSODY_REQUIRE_VULKAN=1 时，设备不可用会使测试失败。
use ash::vk;
use country_core::{
    asset_libraries::texture_library::{AssetTextureLibrary, TextureLibraryDevice, TextureUpdate},
    render::load_texture::boundary::{TextureDesc, TextureDevice},
    resources::texture_presets::TexturePreset,
};
use rhapsody::{
    command::buffer_access, command::image_access, resource::AccessScope, resource::Buffer,
    resource::BufferDesc, resource::OwnershipRange, resource::with_setup_command,
};
use std::sync::Arc;
use tiny_glade::render_host::TextureHost;

#[test]
fn native_texture_clear_changes_pixels_and_descriptor_owns_a_real_view() {
    let device =
        match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|value| value == "1"),
                    "GPU 初始化失败：{error}"
                );
                eprintln!("SKIP 真实资产库 GPU 验证：{error}");
                return;
            }
        };
    let host = TextureHost::new(device.clone()).with_extra_usage(vk::ImageUsageFlags::TRANSFER_SRC);
    let mut assets = AssetTextureLibrary::try_new(&host).unwrap();
    assets.create_preset_textures_with(&host, |_| {
        TextureDesc::new_2d([4, 4], 37)
            .sampled()
            .mip_levels(1)
            .array_elements(1)
    });
    let texture = assets.preset(TexturePreset(0)).unwrap().texture.clone();
    host.update_texture(
        &texture,
        &TextureUpdate {
            data: vec![0x7f; 64],
            buffer_offset: 0,
            mip_level: 0,
            extent: [4, 4, 1],
            array_layers: 1,
            access: 0x22,
        },
    )
    .unwrap();
    assets
        .try_clear_preset_texture(&host, TexturePreset(0))
        .unwrap();
    let ordinary = host
        .create_texture("ordinary", &TextureDesc::new_2d([4, 4], 37).sampled(), &[])
        .unwrap();
    let asset = assets.add(Some("test/clear".into()), ordinary.clone());
    let index = assets.handle_to_bindless_index(asset);
    let descriptors = assets.try_bindless_textures_dset(&host).unwrap();
    assert_eq!(descriptors.lease(index.0).unwrap().index().index, index.0);
    assert_eq!(descriptors.heap().unwrap().stats().live, 1);
    assert_ne!(host.get_vk_image_view(&texture, 0).unwrap().0, texture.id);

    let owner = host.get(&texture).unwrap().shared();
    let output = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(64, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let queue = device
        .queue(rhapsody::device::QueueClass::Graphics)
        .unwrap();
    // SAFETY：资源由当前设备创建，访问经 rhapsody 登记，setup 提交同步等待完成。
    with_setup_command(&device, &[], |command| unsafe {
        image_access(
            command,
            owner.clone(),
            queue,
            OwnershipRange::Image {
                aspects: vk::ImageAspectFlags::COLOR.as_raw(),
                base_mip: 0,
                mip_count: 1,
                base_layer: 0,
                layer_count: 1,
            },
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_READ,
                write: false,
            },
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        )?;
        buffer_access(
            command,
            output.clone(),
            queue,
            0,
            64,
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_WRITE,
                write: true,
            },
        )?;
        device.raw_device().cmd_copy_image_to_buffer(
            command.raw_handle(),
            owner.raw_handle(),
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            output.raw_handle(),
            &[vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_extent(vk::Extent3D {
                    width: 4,
                    height: 4,
                    depth: 1,
                })],
        );
        buffer_access(
            command,
            output.clone(),
            queue,
            0,
            64,
            AccessScope {
                stages: vk::PipelineStageFlags::HOST,
                access: vk::AccessFlags::HOST_READ,
                write: false,
            },
        )
    })
    .and_then(|setup| setup.wait(rhapsody::resource::SETUP_WAIT_TIMEOUT_NS))
    .unwrap();
    let mut pixels = [0xff; 64];
    output.read_bytes(0, &mut pixels).unwrap();
    assert_eq!(pixels, [0; 64]);
    let held_by_command = descriptors.lease(index.0).unwrap();
    let table = descriptors.clone();
    let replacement = host
        .create_texture(
            "replacement",
            &TextureDesc::new_2d([4, 4], 37).sampled(),
            &[],
        )
        .unwrap();
    let previous = assets
        .replace(
            country_core::name_hash::NameHash::compute("test/clear"),
            replacement,
        )
        .unwrap();
    host.release(&previous);
    assert!(table.lease(index.0).is_none());
    let replacement_index = assets.handle_to_bindless_index(asset);
    assert_ne!(replacement_index, index, "旧租约仍在使用时不能复用槽位");
    drop(held_by_command);
    table.heap().unwrap().collect().unwrap();
    assert_eq!(
        table.heap().unwrap().stats().live,
        1,
        "库不再积累已替换纹理的租约"
    );
    eprintln!(
        "asset-library GPU: sampled_capacity={}, live_leases={}",
        table.sampled_capacity(),
        table.heap().unwrap().stats().live
    );
}
