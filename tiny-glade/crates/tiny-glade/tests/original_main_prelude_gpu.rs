//! B main-prefix validation with real original LUT shaders and indirect buffers.
//! This is a bounded fixture, not a rendered gameplay scene or startup-ready proof.
use ash::vk::{self, Handle};
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::{
        configwatch::ConfigWatch, placement_data_library::AssetPlacementDataLibrary,
        shader_library::AssetShaderLibrary, ssbo_library::AssetSsboLibrary,
        texture_library::AssetTextureLibrary,
    },
    name_hash::NameHash,
    render::{
        load_texture::{AlignedFile, boundary::TextureDevice, parse_compiled_image},
        nani::{
            gpu_world::{GpuWorld, GpuWorldSubset, GpuWorldSubsetKind, OwnedGpuWorldSubset},
            instance_data::SolidVertexColorInstanceData,
        },
        pass_builder::{draw_stream::PassHost, framebuffer_binding::AttachmentTexture},
    },
    resources::{gpu_lut::GpuLutTracker, texture_presets::TexturePreset},
    systems::render_loop::{
        draw_passes::{
            d_main_support::{DrawPassCtx, ShaderLookup},
            final_post,
        },
        indirect_params::{
            AtomicCounterBuffer, DispatchIndirectBuffer, DrawIndexedIndirectBuffer,
            DrawIndirectBuffer,
        },
    },
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tiny_glade::render_bridge::CoreRenderBridge;

struct Shaders<'a>(&'a AssetShaderLibrary);
impl ShaderLookup for Shaders<'_> {
    fn raster_handle_by_name(
        &self,
        name: &str,
    ) -> Option<
        country_core::rhapsody_contract::Handle<country_core::render::shader::RasterShaderProgram>,
    > {
        self.0.get_raster_handle_by_name(name).map(|handle| {
            country_core::rhapsody_contract::Handle(
                country_core::rhapsody_contract::ManagedHandle(handle.id().raw()),
                std::marker::PhantomData,
            )
        })
    }
}

#[test]
fn original_main_prefix_imports_owned_world_resets_indirect_and_builds_luts_once() {
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        });
    let installed = installed.canonicalize().unwrap();
    let device =
        match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "strict original main-prefix GPU initialization failed: {error}"
                );
                eprintln!("SKIP original main-prefix GPU initialization: {error}");
                return;
            }
        };
    let shaders = Arc::new(AssetShaderLibrary::new());
    let mut bridge = CoreRenderBridge::new(device.clone(), &installed, shaders.clone()).unwrap();
    country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());
    let mut world = World::new();
    bridge.install_asset_libraries(&mut world).unwrap();
    world.insert_resource(AssetPlacementDataLibrary::new());
    world.insert_resource(country_core::render::frame_constants::FrameConstants::default());
    world.insert_resource(GpuLutTracker::new(false));
    let buffers = bridge.hosts().buffers.clone();
    let mut gpu_world = GpuWorld::new(buffers.as_ref());
    // Actual original owned-subset data type and capacity; no invented GPU ID.
    gpu_world.assign_subset(
        GpuWorldSubsetKind::SolidVertexColor,
        GpuWorldSubset::Owned(OwnedGpuWorldSubset::new::<SolidVertexColorInstanceData>(
            buffers.as_ref(),
            GpuWorldSubsetKind::SolidVertexColor,
        )),
    );
    world.insert_resource(gpu_world);
    let ssbos = world.resource::<AssetSsboLibrary>();
    // Exact four original calls within init_populate_ssbo_library@14090a057..87.
    // This fixture does not substitute for that still-unrecovered full producer.
    ssbos.add_new::<DispatchIndirectBuffer>(buffers.as_ref());
    ssbos.add_new::<DrawIndexedIndirectBuffer>(buffers.as_ref());
    ssbos.add_new::<DrawIndirectBuffer>(buffers.as_ref());
    ssbos.add_new::<AtomicCounterBuffer>(buffers.as_ref());
    for buffer in [
        ssbos.get_buffer::<DispatchIndirectBuffer>().unwrap(),
        ssbos.get_buffer::<DrawIndexedIndirectBuffer>().unwrap(),
        ssbos.get_buffer::<DrawIndirectBuffer>().unwrap(),
        ssbos.get_buffer::<AtomicCounterBuffer>().unwrap(),
    ] {
        assert_eq!(
            buffers.description(buffer).unwrap().usage.as_raw(),
            0x122,
            "indirect producers must not use generic uniform/storage usage"
        );
    }
    let mut config_watch = ConfigWatch::new(false);
    let palette = country_core::startup::init_color_library::init_color_library(
        buffers.as_ref(),
        ssbos,
        &mut config_watch,
    );
    world.insert_resource(palette);
    world.insert_resource(config_watch);
    // Supply the one real packaged disk texture referenced by the original water
    // LUT callback, selected by its native NameHash rather than a made-up image.
    let root = installed.join("compiled-assets/textures");
    let mut found = false;
    for entry in walkdir::WalkDir::new(&root) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file()
            || entry.path().extension().and_then(|x| x.to_str()) != Some("texture")
        {
            continue;
        }
        let name = entry.path().strip_prefix(&root).unwrap().with_extension("");
        let name = name.to_str().unwrap();
        if NameHash::compute(name).0 != 0x6ec9_5f7a_9d9f_0faf {
            continue;
        }
        let file = AlignedFile::read(entry.path()).unwrap();
        let parsed = parse_compiled_image(file.bytes(), entry.path());
        let texture = bridge
            .hosts()
            .textures
            .create_texture(name, &parsed.desc, parsed.data)
            .unwrap();
        world
            .resource_mut::<AssetTextureLibrary>()
            .add(Some(name.to_owned()), texture);
        found = true;
    }
    assert!(found, "the original water-LUT disk texture must be present");
    let lookup = Shaders(&shaders);
    let mut previous_generation = None;
    let mut previous_pixels = None;
    for frame_index in 0..2 {
        let mut readbacks = Vec::new();
        let (generation, built) = bridge
            .build_frame(|frame| {
                frame.with_main_prelude(&mut world, |frame, world| {
                    let generation = world
                        .resource::<GpuWorld>()
                        .owned_subset_mut(GpuWorldSubsetKind::SolidVertexColor)
                        .data()
                        .rg_handle()
                        .generation;
                    for (preset, extent) in [(0, [64, 64]), (0x18, [256, 256])] {
                        let source = world
                            .resource::<AssetTextureLibrary>()
                            .get_rg_texture(TexturePreset(preset));
                        let target = Arc::new(
                            rhapsody::resource::Texture::new(
                                device.clone(),
                                &rhapsody::resource::TextureDesc::new_2d(
                                    extent[0],
                                    extent[1],
                                    vk::Format::R8G8B8A8_UNORM,
                                )
                                .usage(
                                    vk::ImageUsageFlags::COLOR_ATTACHMENT
                                        | vk::ImageUsageFlags::TRANSFER_SRC,
                                ),
                            )
                            .unwrap(),
                        );
                        let output = Arc::new(
                            rhapsody::resource::Buffer::new(
                                device.clone(),
                                &rhapsody::resource::BufferDesc::new_gpu_to_cpu(
                                    u64::from(extent[0] * extent[1] * 4),
                                    vk::BufferUsageFlags::TRANSFER_DST,
                                ),
                            )
                            .unwrap(),
                        );
                        let target_id = frame.import_native_texture(target.clone(), 0)?;
                        let output_id = frame.import_native_buffer(output.clone(), 0)?;
                        frame.with_raster(|host| {
                            let mut make = || -> Box<dyn PassHost> { Box::new(host.sibling()) };
                            let mut ctx =
                                DrawPassCtx::new(world, Duration::ZERO, &mut make, &lookup);
                            final_post::resolve_to_swapchain(
                                &mut ctx,
                                &AttachmentTexture {
                                    id: source.resource as u32,
                                    extent: (extent[0], extent[1]),
                                    format: source.desc.format,
                                    sample_count: 1,
                                },
                                &AttachmentTexture {
                                    id: target_id.raw(),
                                    extent: (extent[0], extent[1]),
                                    format: 37,
                                    sample_count: 1,
                                },
                                0,
                            );
                            Ok(())
                        })?;
                        frame.with_graph(|graph| {
                            let target = target.clone();
                            let output = output.clone();
                            let device = device.clone();
                            let mut copy =
                                graph.add_pass("fixture readback of original LUT resolve");
                            copy.add_resource_access(target_id, 0x25, 0)?;
                            copy.add_resource_access(output_id, 0x34, 0)?;
                            copy.render_with_result_context(move |ctx| {
                                unsafe {
                                    device.raw_device().cmd_copy_image_to_buffer(
                                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                                        target.handle,
                                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                                        output.handle,
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
                            graph.export(output_id, 0x26)?;
                            Ok(())
                        })?;
                        readbacks.push((output, extent));
                    }
                    Ok(generation)
                })
            })
            .unwrap();
        let descriptors = world.resource::<GpuLutTracker>().descriptors();
        assert_eq!(descriptors.len(), 2);
        assert!(
            descriptors
                .iter()
                .all(|d| d.rendered() && d.source_hashes().len() == 1),
            "native rendered flag is set during CPU graph declaration before submission"
        );
        if let Some(previous) = previous_generation {
            assert_ne!(generation, previous);
        }
        previous_generation = Some(generation);
        bridge
            .shader_backend()
            .prewarm(Duration::from_secs(30))
            .unwrap();
        let compiled = bridge.compile_built(built).unwrap();
        let receipt = bridge.execute_compiled(compiled).unwrap();
        assert!(!receipt.receipts.is_empty());
        assert_eq!(
            receipt
                .snapshot
                .pass_order
                .iter()
                .filter(|s| s.as_str() == "LUT")
                .count(),
            if frame_index == 0 { 2 } else { 0 },
            "original LUTs are constructed once per tracker"
        );
        assert!(
            receipt
                .snapshot
                .pass_order
                .iter()
                .any(|s| s == "Reset indirect args")
        );
        bridge.wait_submits_done(10_000_000_000).unwrap();
        let mut pixels = Vec::new();
        for (output, extent) in readbacks {
            let mut image = vec![0; (extent[0] * extent[1] * 4) as usize];
            output.read_bytes(0, &mut image).unwrap();
            assert!(
                image
                    .chunks_exact(4)
                    .any(|p| p[0] != 0 || p[1] != 0 || p[2] != 0),
                "packaged LUT shader and original resolve must produce actual RGB data"
            );
            pixels.push(image);
        }
        if let Some(previous) = previous_pixels {
            assert_eq!(
                pixels, previous,
                "once-generated LUT contents persist across frames"
            );
        }
        previous_pixels = Some(pixels);
    }
}
