//! Actual original-egui-shader consumer test. The black clear and red rectangle
//! are bounded validation inputs, not a replacement game scene or loading graph.
use ash::vk::{self, Handle};
use country_core::{
    asset_libraries::shader_library::AssetShaderLibrary,
    render::pass_builder::framebuffer_binding::AttachmentTexture,
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tiny_glade::{egui_backend::EguiBackend, egui_gpu::EguiGpu, render_bridge::CoreRenderBridge};

#[test]
fn original_egui_shader_draws_real_vertex_index_and_font_uploads_to_gpu() {
    verify_original_egui(1);
}

#[test]
fn original_egui_shader_draws_to_the_gameplay_four_sample_target() {
    verify_original_egui(4);
}

fn verify_original_egui(sample_count: u8) {
    let format = if sample_count == 4 {
        vk::Format::from_raw(
            country_core::systems::render_loop::draw_passes::final_post::MSAA_COLOR_FORMAT as i32,
        )
    } else {
        vk::Format::R8G8B8A8_UNORM
    };
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
    let mut renderer = EguiGpu::new(device.clone(), bridge.hosts().clone());
    let context = egui::Context::default();
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
            egui::Rect::from_min_max(egui::pos2(16.0, 16.0), egui::pos2(48.0, 48.0)),
            0.0,
            egui::Color32::RED,
        );
    backend.end_frame();
    let target = Arc::new(
        rhapsody::resource::Texture::new(
            device.clone(),
            &rhapsody::resource::TextureDesc::new_2d(64, 64, format)
                .sample_count(sample_count)
                .usage(
                    vk::ImageUsageFlags::COLOR_ATTACHMENT
                        | vk::ImageUsageFlags::TRANSFER_SRC
                        | vk::ImageUsageFlags::TRANSFER_DST,
                ),
        )
        .unwrap(),
    );
    let resolved = if sample_count == 4 {
        Arc::new(
            rhapsody::resource::Texture::new(
                device.clone(),
                &rhapsody::resource::TextureDesc::new_2d(64, 64, format)
                    .usage(vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST),
            )
            .unwrap(),
        )
    } else {
        target.clone()
    };
    let readback = Arc::new(
        rhapsody::resource::Buffer::new(
            device.clone(),
            &rhapsody::resource::BufferDesc::new_gpu_to_cpu(
                64 * 64 * 4,
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
            let read_color = if sample_count == 4 {
                let destination = frame.import_native_texture(resolved.clone(), 0)?;
                frame.with_graph(|graph| {
                    let device = device.clone();
                    let target = target.clone();
                    let resolved = resolved.clone();
                    let mut pass = graph.add_pass("read back multisampled original egui");
                    pass.add_resource_access(color, 0x25, 0)?;
                    pass.add_resource_access(destination, 0x34, 0)?;
                    pass.render_with_result_context(move |ctx| {
                        let layers = vk::ImageSubresourceLayers::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .layer_count(1);
                        unsafe {
                            device.raw_device().cmd_resolve_image(
                                vk::CommandBuffer::from_raw(ctx.command.handle()),
                                target.raw_handle(),
                                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                                resolved.raw_handle(),
                                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                                &[vk::ImageResolve::default()
                                    .src_subresource(layers)
                                    .dst_subresource(layers)
                                    .extent(vk::Extent3D {
                                        width: 64,
                                        height: 64,
                                        depth: 1,
                                    })],
                            );
                        }
                        Ok(())
                    });
                    Ok(())
                })?;
                destination
            } else {
                color
            };
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
                                    width: 64,
                                    height: 64,
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
        .prewarm(Duration::from_secs(10))
        .unwrap();
    let compiled = bridge.compile_built(graph).unwrap();
    let receipt = bridge.execute_compiled(compiled).unwrap();
    assert!(!receipt.receipts.is_empty());
    bridge.wait_submits_done(10_000_000_000).unwrap();
    let mut pixels = vec![0; 64 * 64 * 4];
    readback.read_bytes(0, &mut pixels).unwrap();
    assert_eq!(
        &pixels[(32 * 64 + 32) * 4..][..4],
        &[255, 0, 0, 255],
        "center must come from the original shader's indexed draw"
    );
    assert_eq!(&pixels[..4], &[0, 0, 0, 255]);
}
