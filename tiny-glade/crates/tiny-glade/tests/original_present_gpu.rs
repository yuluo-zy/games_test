//! Real final presentation consumer: resize while its completed source retains the old extent.
//! Read back every output pixel for single-sample/MSAA, growth/shrink and both orientations.
#[path = "support/mod.rs"]
mod test_support;
use ash::vk::{self, Handle};
use country_core::{
    asset_libraries::shader_library::AssetShaderLibrary,
    render::pass_builder::framebuffer_binding::AttachmentTexture,
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use test_support::BridgeTestDriver as _;
use test_support::ShaderTestDriver as _;
use tiny_glade::{egui_backend::EguiBackend, egui_gpu::EguiGpu, render_bridge::CoreRenderBridge};

#[test]
fn resized_present_covers_larger_and_smaller_targets_without_black_borders() {
    let shader_root = std::env::var_os("TG_COMPILED_SHADERS")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../../../ljxsj_92385/Tiny Glade/compiled-shaders")
        });
    let device =
        match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|value| value == "1"),
                    "strict original egui GPU initialization failed: {error}"
                );
                eprintln!("SKIP original egui GPU initialization: {error}");
                return;
            }
        };
    let shaders = Arc::new(AssetShaderLibrary::new());
    let mut bridge = CoreRenderBridge::new(
        device.clone(),
        shader_root
            .parent()
            .expect("compiled shader directory parent"),
        shaders.clone(),
    )
    .unwrap();
    country_core::startup::load_shaders::load_shader_into_library(
        &shaders,
        bridge.shader_backend(),
        "/egui.raster.hlsl",
        "egui",
    );
    country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());
    for samples in [1, 4] {
        for extent in [[64, 64], [97, 83], [31, 23], [194, 166], [64, 64]] {
            for flip in [0, 1] {
                verify_present(&device, &shaders, &mut bridge, samples, extent, flip);
            }
        }
    }
}

fn verify_present(device: &Arc<rhapsody::device::Device>, shaders: &Arc<AssetShaderLibrary>, bridge: &mut CoreRenderBridge, sample_count: u8, extent: [u32; 2], flip: u8) {
    let format = if sample_count == 4 {
        vk::Format::from_raw(
            country_core::systems::render_loop::draw_passes::final_post::MSAA_COLOR_FORMAT as i32,
        )
    } else {
        vk::Format::R8G8B8A8_UNORM
    };
    let mut renderer = EguiGpu::new(bridge.hosts().clone());
    let context = egui::Context::default();
    context.tessellation_options_mut(|options| options.feathering = false);
    let mut backend = EguiBackend::new(context.clone());
    context.begin_frame(egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(64.0, 64.0),
        )),
        ..Default::default()
    });
    context
        .layer_painter(egui::LayerId::background())
        .rect_filled(
            egui::Rect::from_min_max(egui::pos2(-5.0, -5.0), egui::pos2(69.0, 69.0)),
            0.0,
            egui::Color32::RED,
        );
    // Avoid placing color discontinuities at an exact resampling tie.
    for (min, max, color) in [
        ([24.0, -5.0], [69.0, 40.0], egui::Color32::GREEN),
        ([-5.0, 40.0], [24.0, 69.0], egui::Color32::BLUE),
        ([24.0, 40.0], [69.0, 69.0], egui::Color32::WHITE),
    ] {
        context
            .layer_painter(egui::LayerId::background())
            .rect_filled(
                egui::Rect::from_min_max(egui::pos2(min[0], min[1]), egui::pos2(max[0], max[1])),
                0.0,
                color,
            );
    }
    backend.end_frame();
    let target = Arc::new(
        rhapsody::resource::Texture::new(
            device.clone(),
            &rhapsody::resource::TextureDesc::new_2d(64, 64, format)
                .sample_count(sample_count)
                .usage(
                    vk::ImageUsageFlags::SAMPLED
                        | vk::ImageUsageFlags::COLOR_ATTACHMENT
                        | vk::ImageUsageFlags::TRANSFER_SRC
                        | vk::ImageUsageFlags::TRANSFER_DST,
                ),
        )
        .unwrap(),
    );
    let resolved = Arc::new(
        rhapsody::resource::Texture::new(
            device.clone(),
            &rhapsody::resource::TextureDesc::new_2d(
                extent[0],
                extent[1],
                vk::Format::R8G8B8A8_UNORM,
            )
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC),
        )
        .unwrap(),
    );
    let byte_count = (extent[0] * extent[1] * 4) as usize;
    let readback = Arc::new(
        rhapsody::resource::Buffer::new(
            device.clone(),
            &rhapsody::resource::BufferDesc::new_gpu_to_cpu(
                byte_count as u64,
                vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
        .unwrap(),
    );
    let (_, graph) = bridge
        .build_frame(|frame| {
            let color = frame.import_native_texture(target.clone(), 0)?;
            let output = frame.import_native_buffer(readback.clone(), 0)?;
            frame.with_graph(|graph| {
                let device = device.clone();
                let target = target.clone();
                let mut clear = graph.add_pass("fixture background");
                clear.add_resource_access(color, 0x34, 0)?;
                clear.render_with_result_context(move |ctx| {
                    unsafe {
                        device.raw_device().cmd_clear_color_image(
                            vk::CommandBuffer::from_raw(ctx.command.handle()),
                            target.raw_handle(),
                            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                            &vk::ClearColorValue {
                                float32: [0.0, 0.0, 0.0, 1.0],
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
            renderer
                .draw(
                    frame,
                    &mut backend,
                    &shaders,
                    AttachmentTexture {
                        id: color.raw(),
                        extent: (64, 64),
                        format: format.as_raw() as u32,
                        sample_count,
                    },
                )
                .map_err(|error| rhapsody::Error::InvalidUsage {
                    what: "original egui consumer".into(),
                    source: Some(error.into()),
                })?;
            let read_color = frame.import_native_texture(resolved.clone(), 0)?;
            struct Lookup<'a>(&'a AssetShaderLibrary);
            impl country_core::systems::render_loop::draw_passes::d_main_support::ShaderLookup for Lookup<'_> {
                fn raster_handle_by_name(&self, name: &str) -> Option<country_core::rhapsody_contract::Handle<country_core::render::shader::RasterShaderProgram>> {
                    self.0.get_raster_handle_by_name(name).map(|h| country_core::rhapsody_contract::Handle(country_core::rhapsody_contract::ManagedHandle(h.id().raw()), core::marker::PhantomData))
                }
            }
            frame.with_raster(|host| {
                let mut scratch = bevy_ecs::world::World::new();
                let lookup = Lookup(&shaders);
                let mut new_host = || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> { Box::new(host.sibling()) };
                let mut ctx = country_core::systems::render_loop::draw_passes::d_main_support::DrawPassCtx::new(&mut scratch, Duration::ZERO, &mut new_host, &lookup);
                country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain(&mut ctx,
                    &AttachmentTexture { id: color.raw(), extent: (64,64), format: format.as_raw() as u32, sample_count },
                    &AttachmentTexture { id: read_color.raw(), extent: (extent[0],extent[1]), format: vk::Format::R8G8B8A8_UNORM.as_raw() as u32, sample_count: 1 }, flip);
                Ok(())
            })?;
            frame.with_graph(|graph| {
                let device = device.clone();
                let target = resolved.clone();
                let readback = readback.clone();
                let mut copy = graph.add_pass("verify original egui output");
                copy.add_resource_access(read_color, 0x25, 0)?;
                copy.add_resource_access(output, 0x34, 0)?;
                copy.render_with_result_context(move |ctx| {
                    unsafe {
                        device.raw_device().cmd_copy_image_to_buffer(
                            vk::CommandBuffer::from_raw(ctx.command.handle()),
                            target.raw_handle(),
                            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                            readback.raw_handle(),
                            &[vk::BufferImageCopy::default()
                                .image_subresource(
                                    vk::ImageSubresourceLayers::default()
                                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                                        .layer_count(1),
                                )
                                .image_extent(vk::Extent3D {
                                    width: extent[0],
                                    height: extent[1],
                                    depth: 1,
                                })],
                        );
                    }
                    Ok(())
                });
                graph.export(output, 0x26)?;
                Ok(())
            })
        })
        .unwrap();
    bridge
        .shader_backend()
        .prepare_test_pipelines(Duration::from_secs(10))
        .unwrap();
    let compiled = bridge.compile_built(graph).unwrap();
    let receipt = bridge.execute_after_uploads(compiled).unwrap();
    assert!(!receipt.receipts.is_empty());
    bridge.wait_submits_done(10_000_000_000).unwrap();
    let mut pixels = vec![0; byte_count];
    readback.read_bytes(0, &mut pixels).unwrap();
    if let Some(directory) = std::env::var_os("TG_PRESENT_CAPTURE_DIR") {
        std::fs::create_dir_all(&directory).unwrap();
        image::save_buffer(
            PathBuf::from(directory).join(format!(
                "present-{}x{}-msaa{sample_count}-flip{flip}.png",
                extent[0], extent[1]
            )),
            &pixels,
            extent[0],
            extent[1],
            image::ColorType::Rgba8,
        )
        .unwrap();
    }
    for (index, pixel) in pixels.chunks_exact(4).enumerate() {
        let x = index as u32 % extent[0];
        let y = index as u32 / extent[0];
        let source_x = ((x as f32 + 0.5) * 64.0 / extent[0] as f32) as u32;
        let y = if flip == 1 { extent[1] - 1 - y } else { y };
        let source_y = ((y as f32 + 0.5) * 64.0 / extent[1] as f32) as u32;
        let expected = match (source_x >= 24, source_y >= 40) {
            (false, false) => [255, 0, 0, 255],
            (true, false) => [0, 255, 0, 255],
            (false, true) => [0, 0, 255, 255],
            (true, true) => [255, 255, 255, 255],
        };
        assert_eq!(
            pixel, &expected,
            "present pixel ({x}, {y}) source=({source_x},{source_y}) samples={sample_count} target={extent:?} flip={flip}"
        );
    }
}
