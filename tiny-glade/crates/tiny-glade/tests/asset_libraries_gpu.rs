//! 真实宿主验证：纹理更新与清零回读，以及真实采样描述符租约。
//! RHAPSODY_REQUIRE_VULKAN=1 时，设备不可用会使测试失败。
use ash::vk;
use country_core::{
    asset_libraries::texture_library::{AssetTextureLibrary, TextureLibraryDevice, TextureUpdate},
    render::load_texture::boundary::{TextureDesc, TextureDevice},
    resources::texture_presets::TexturePreset,
};
use rhapsody::{command::buffer_access, command::image_access, resource::Buffer, resource::BufferDesc};
use rhapsody::tracking::OwnershipRange;
use rhapsody::transfer::with_setup_command;
use rhapsody::native::AccessScope;
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
            &owner.clone(),
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
            &output.clone(),
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
            &output.clone(),
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
    .and_then(|setup| setup.wait(rhapsody::transfer::SETUP_WAIT_TIMEOUT_NS))
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

/// 文件图片必须经过真实控件准备、纹理宿主、采样租约和 GPU 像素读回。
#[test]
fn mei_file_image_uses_real_upload_cache_descriptor_and_releases_host_slot() -> anyhow::Result<()> {
    use country_core::{asset_libraries::texture_library::AssetIndex, render::device::GraphicsDeviceResource, rhapsody_contract::GraphicsDevice};
    use toolbar::mei::{core::{NodeContent, NodeDrawContext, NodePrepareContext}, widgets::{DefaultNodeContent, FileImageCache, ImageContent}};
    let device = rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless())?;
    let host = Arc::new(TextureHost::new(device.clone()).with_extra_usage(vk::ImageUsageFlags::TRANSFER_SRC));
    let mut assets = AssetTextureLibrary::try_new(host.as_ref())?;
    let mut cache = FileImageCache::new(host.clone());
    let path = std::env::temp_dir().join(format!("mei-gpu-file-{}-{}.png",std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos()));
    let expected = vec![255,0,0,255,0,255,0,128];
    image::RgbaImage::from_raw(2,1,expected.clone()).unwrap().save(&path)?;
    let mut content = ImageContent::from_font_table(Arc::new(Default::default()));
    content.image = format!("file:{}",path.display());
    content.height = 32.;
    let graphics = GraphicsDeviceResource::new(GraphicsDevice::new(device.clone()));
    let mut renderer = tg_text::TextRenderer::new();
    let mut state = 0;
    let actions = Default::default();
    for frame in 0..2 {
        let mut ctx = NodePrepareContext {
            text: &mut renderer, device: &graphics, textures: &mut assets, file_images: Some(&mut cache),
            state: &mut state, actions: &actions, window_points: glam::Vec2::new(1000.,1000.),
            steam_client: None, steam_input: None, pixels_per_point: 1., cursor: glam::Vec2::ZERO, elapsed: 0.016,
        };
        let size = content.prepare_layout(&mut ctx).unwrap();
        assert_eq!(size.map(|x|x.value),[64.,32.]);
        assert_eq!(assets.len(),1);
        if frame == 0 { std::fs::remove_file(&path)?; }
    }
    content.draw(&mut NodeDrawContext {
        text: &mut renderer, center: glam::Vec2::new(32.,16.), extent: glam::Vec2::new(64.,32.),
        opacity:1.,pixels_per_point:1.,cursor:glam::Vec2::ZERO,elapsed:0.016,state:0,
    });
    assert!(!renderer.consume()?.primitives.is_empty());
    let texture = assets.texture(AssetIndex(0)).clone();
    assert_eq!(texture.format,43);
    let owner = host.get(&texture).unwrap().shared();
    let index = assets.handle_to_bindless_index(AssetIndex(0));
    let descriptors = assets.try_bindless_textures_dset(host.as_ref())?;
    assert_eq!(descriptors.lease(index.0).unwrap().index().index,index.0);
    let output = Arc::new(Buffer::new(device.clone(),&BufferDesc::new_gpu_to_cpu(8,vk::BufferUsageFlags::TRANSFER_DST))?);
    let queue = device.queue(rhapsody::device::QueueClass::Graphics)?;
    // SAFETY：源纹理、目标缓冲均由当前设备创建，访问登记后提交并等待完成。
    with_setup_command(&device,&[],|command| unsafe {
        image_access(command,&owner.clone(),queue,OwnershipRange::Image {
            aspects:vk::ImageAspectFlags::COLOR.as_raw(),base_mip:0,mip_count:1,base_layer:0,layer_count:1,
        },AccessScope { stages:vk::PipelineStageFlags::TRANSFER,access:vk::AccessFlags::TRANSFER_READ,write:false },vk::ImageLayout::TRANSFER_SRC_OPTIMAL)?;
        buffer_access(command,&output.clone(),queue,0,8,AccessScope {
            stages:vk::PipelineStageFlags::TRANSFER,access:vk::AccessFlags::TRANSFER_WRITE,write:true,
        })?;
        device.raw_device().cmd_copy_image_to_buffer(command.raw_handle(),owner.raw_handle(),vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            output.raw_handle(),&[vk::BufferImageCopy::default().image_subresource(vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR).layer_count(1)).image_extent(vk::Extent3D {width:2,height:1,depth:1})]);
        buffer_access(command,&output.clone(),queue,0,8,AccessScope {
            stages:vk::PipelineStageFlags::HOST,access:vk::AccessFlags::HOST_READ,write:false,
        })
    })?.wait(rhapsody::transfer::SETUP_WAIT_TIMEOUT_NS)?;
    let mut actual = vec![0;8];output.read_bytes(0,&mut actual)?;
    assert_eq!(actual,expected);
    let pending_frame=cache.retained_textures();
    drop(cache);
    assert!(host.get(&texture).is_some(),"待执行的MEI快照仍拥有宿主槽位");
    drop(pending_frame);
    assert!(host.get(&texture).is_none());
    assert!(descriptors.lease(index.0).is_some(),"在途图和描述符仍保留实际托管纹理");
    Ok(())
}
