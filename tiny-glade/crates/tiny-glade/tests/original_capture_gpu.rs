//! Strict MSAA source -> original downsample/capture shaders -> canonical slots
//! -> original buffer shader -> actual mapped bytes. Red is a bounded fixture.
use ash::vk::{self, Handle};
use country_core::{
    asset_libraries::{
        placement_data_library::AssetPlacementDataLibrary, shader_library::AssetShaderLibrary,
        ssbo_library::AssetSsboLibrary, texture_library::AssetTextureLibrary,
        vao_library::AssetVAOLibrary,
    },
    game_config::GameConfig,
    render::{
        pass_builder::{compute_pass_builder::RgBuffer, pass_resource_access::RgTexture},
        render_config::FrameRenderConfig,
    },
    resources::{
        events::EscapeMenuJustOpened,
        gui::{EscMenuState, NextEscMenuState},
        window::WindowSize,
    },
    systems::render_loop::{
        compute_passes::pass_kit::PassEnv, lut_gallery::LutGalleryState,
        menu_background::MenuBackgroundResources,
    },
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tiny_glade::{
    compute_frame::ComputeUniformAllocator,
    frame_world::{FrameLibraries, FrameWorldAccess},
    render_bridge::CoreRenderBridge,
    run_game::ApplicationShaderLibrary,
};

#[test]
fn original_shared_captures_keep_msaa_pixels_and_actual_slot_owners() -> anyhow::Result<()> {
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        });
    let device =
        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless())?;
    let shaders = Arc::new(AssetShaderLibrary::new());
    let mut bridge = CoreRenderBridge::new(device.clone(), &installed, shaders.clone())?;
    let mut app = bevy_app::App::new();
    bridge.install_asset_libraries(app.world_mut())?;
    // FrameWorldAccess includes the canonical palette pointer in the common
    // shader prefix, even for a capture shader which does not sample colors.
    // Use the actual palette producer and its physical SSBO allocation.
    let mut watch = country_core::asset_libraries::configwatch::ConfigWatch::new(false);
    let palette = country_core::startup::init_color_library::init_color_library(
        bridge.hosts().buffers.as_ref(),
        app.world().resource::<AssetSsboLibrary>(),
        &mut watch,
    );
    app.insert_resource(palette);
    app.insert_resource(watch);
    let config = GameConfig::default();
    app.insert_resource(FrameRenderConfig::at_startup(
        WindowSize::new(128, 128),
        &config.video,
    ));
    app.insert_resource(config);
    app.insert_resource(ApplicationShaderLibrary(shaders.clone()));
    app.insert_resource(AssetPlacementDataLibrary::new());
    app.insert_resource(country_core::resources::time::GameTime::default());
    app.insert_resource(EscMenuState::Photomode(false));
    app.init_resource::<NextEscMenuState>();
    app.init_resource::<LutGalleryState>();
    tiny_glade::menu_background_host::install(&mut app, bridge.hosts().clone());
    tiny_glade::screenshot::shared_texture_capture::install(&mut app);
    app.world_mut().send_event(EscapeMenuJustOpened);
    app.update();
    assert!(matches!(
        app.world().resource::<NextEscMenuState>().0,
        Some(EscMenuState::Photomode(true))
    ));
    let source = Arc::new(rhapsody::resource::Texture::new(
        device.clone(),
        &rhapsody::resource::TextureDesc::new_2d(128, 128, vk::Format::R8G8B8A8_UNORM)
            .sample_count(4)
            .sampled()
            .color_attachment()
            .usage(
                vk::ImageUsageFlags::SAMPLED
                    | vk::ImageUsageFlags::COLOR_ATTACHMENT
                    | vk::ImageUsageFlags::TRANSFER_DST,
            ),
    )?);
    let readback = Arc::new(rhapsody::resource::Buffer::new(
        device.clone(),
        &rhapsody::resource::BufferDesc::new_gpu_to_cpu(
            128 * 128 * 4,
            vk::BufferUsageFlags::STORAGE_BUFFER,
        ),
    )?);
    let ((gallery_id, menu_id), built) = bridge.build_frame(|frame| {
        let color = frame.import_native_texture(source.clone(), 0)?;
        let output = frame.import_native_buffer(readback.clone(), 0)?;
        frame.with_graph(|graph| {
            let mut pass = graph.add_pass("bounded red MSAA source");
            pass.add_resource_access(color, 0x34, 0)?;
            let device = device.clone();
            let source = source.clone();
            pass.render_with_result_context(move |ctx| {
                unsafe {
                    device.raw_device().cmd_clear_color_image(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        source.raw_handle(),
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &vk::ClearColorValue {
                            float32: [1., 0., 0., 1.],
                        },
                        &[vk::ImageSubresourceRange::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .level_count(1)
                            .layer_count(1)],
                    );
                }
                Ok(())
            });
            Ok(())
        })?;
        let source = RgTexture {
            resource: color.raw(),
            desc: country_core::rhapsody_types::TextureDesc::new_2d([128, 128], 0x25),
        };
        let world = app.world_mut();
        assert_eq!(
            tiny_glade::screenshot::shared_texture_capture::opportunity(
                frame, world, 1, source, 4
            )?,
            1
        );
        assert_eq!(
            tiny_glade::screenshot::shared_texture_capture::opportunity(
                frame, world, 2, source, 4
            )?,
            1
        );
        let gallery_id = world
            .resource::<LutGalleryState>()
            .preview
            .as_ref()
            .unwrap()
            .lock()
            .as_ref()
            .unwrap()
            .0;
        let menu_id = world
            .resource::<MenuBackgroundResources>()
            .shared
            .lock()
            .as_ref()
            .unwrap()
            .0;
        assert_ne!(gallery_id, menu_id);
        let gallery = frame.hosts().textures.get_by_id(gallery_id).unwrap();
        let menu = frame.hosts().textures.get_by_id(menu_id).unwrap();
        assert_eq!((gallery.desc.width, gallery.desc.height), (51, 51));
        assert_eq!((menu.desc.width, menu.desc.height), (128, 128));
        let input_id = frame.import_native_texture(menu.shared(), 0x22)?;
        let input = RgTexture {
            resource: input_id.raw(),
            desc: country_core::rhapsody_types::TextureDesc::new_2d([128, 128], 0x25),
        };
        let backend = frame.shader_backend();
        let raster = frame.raster_resources();
        let access = FrameWorldAccess::new(
            FrameLibraries {
                shaders: &shaders,
                textures: world.resource::<AssetTextureLibrary>(),
                ssbos: world.resource::<AssetSsboLibrary>(),
                vaos: world.resource::<AssetVAOLibrary>(),
                placement: world.resource::<AssetPlacementDataLibrary>(),
                constants: raster.dynamic_constants(),
            },
            frame.compute_device(),
        )
        .with_resource_graph_factory(frame.resource_graph_factory());
        frame.with_compute(|compute| {
            let mut uniforms = ComputeUniformAllocator;
            let mut env = PassEnv::new(&access, backend.as_ref(), &mut uniforms, compute);
            let pass = env
                .try_new_pass(
                    "verify actual shared capture",
                    &"/screenshot/capture_screenshot_buffer.cs.hlsl:sample1",
                )?
                .bind_texture(0x681c_a9c9_3557_dc9c, &&input, 0x19)
                .write_rg_storage_buffer(
                    0x0f68_04da_1cec_c30e,
                    &RgBuffer {
                        resource: output.raw(),
                    },
                )
                .uniform(0xca50_2818_f423_c41d, [128u32, 128u32])
                .dispatch_2d(128, 128);
            env.record(pass);
            Ok(())
        })?;
        frame.with_graph(|graph| {
            graph.export(output, 0x26)?;
            Ok(())
        })?;
        Ok((gallery_id, menu_id))
    })?;
    bridge.shader_backend().prewarm(Duration::from_secs(10))?;
    let capture_graph_id = built.graph_id();
    let compiled = bridge.compile_built(built)?;
    let receipt = bridge.execute_compiled(compiled)?;
    tiny_glade::screenshot::shared_texture_capture::submitted(app.world_mut(), &mut bridge, capture_graph_id, &receipt)?;
    assert!(!receipt.receipts.is_empty());
    bridge.wait_submits_done(10_000_000_000)?;
    let mut pixels = vec![0u8; 128 * 128 * 4];
    readback.read_bytes(0, &mut pixels)?;
    assert_eq!(&pixels[(64 * 128 + 64) * 4..][..4], &[255, 0, 0, 255]);
    assert!(bridge.hosts().textures.get_by_id(gallery_id).is_some());
    assert!(bridge.hosts().textures.get_by_id(menu_id).is_some());
    Ok(())
}
