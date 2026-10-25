//! B scene consumer fixtures using packaged shaders and actual resource owners.
//! These bounded inputs do not assert that the full gameplay graph is ready.
use ash::vk::{self, Handle};
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::{
        placement_data_library::AssetPlacementDataLibrary, shader_library::AssetShaderLibrary,
        ssbo_library::AssetSsboLibrary, texture_library::AssetTextureLibrary,
    },
    render::{
        frame_constants::FrameConstants,
        pass_builder::{
            draw_stream::PassHost, framebuffer_binding::AttachmentTexture,
            pass_resource_access::RgTexture, rg_resources::RgResources,
        },
    },
    resources::{
        texture_presets::TexturePreset, world_raster::DirtyTilePublisher, world_raster_defs::*,
    },
    rhapsody_types::TextureDesc,
    systems::render_loop::draw_passes::{
        d_main_support::{DrawPassCtx, ShaderLookup},
        final_post,
    },
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tiny_glade::{
    render_bridge::{CoreGraphFrame, CoreRenderBridge},
    scene_compute,
};

struct Fixture {
    device: Arc<rhapsody::device::Device>,
    bridge: CoreRenderBridge,
    shaders: Arc<AssetShaderLibrary>,
    world: World,
}
impl Fixture {
    fn new() -> Option<Self> {
        let installed = std::env::var_os("TG_INSTALL_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
            });
        let device = match rhapsody::device::Device::new_headless(
            &rhapsody::device::DeviceConfig::headless(),
        ) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "strict original scene GPU initialization: {error}"
                );
                eprintln!("SKIP original scene GPU initialization: {error}");
                return None;
            }
        };
        let shaders = Arc::new(AssetShaderLibrary::new());
        let bridge = CoreRenderBridge::new(
            device.clone(),
            installed.canonicalize().unwrap(),
            shaders.clone(),
        )
        .unwrap();
        country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());
        let mut world = World::new();
        bridge.install_asset_libraries(&mut world).unwrap();
        let mut config_watch = country_core::asset_libraries::configwatch::ConfigWatch::new(false);
        let palette = country_core::startup::init_color_library::init_color_library(
            bridge.hosts().buffers.as_ref(),
            world.resource::<AssetSsboLibrary>(),
            &mut config_watch,
        );
        world.insert_resource(palette);
        world.insert_resource(config_watch);
        world.insert_resource(AssetPlacementDataLibrary::new());
        world.insert_resource(FrameConstants::default());
        Some(Self {
            device,
            bridge,
            shaders,
            world,
        })
    }
}

struct Shaders<'a>(&'a AssetShaderLibrary);
impl ShaderLookup for Shaders<'_> {
    fn raster_handle_by_name(
        &self,
        name: &str,
    ) -> Option<
        country_core::rhapsody_contract::Handle<country_core::render::shader::RasterShaderProgram>,
    > {
        self.0.get_raster_handle_by_name(name).map(|h| {
            country_core::rhapsody_contract::Handle(
                country_core::rhapsody_contract::ManagedHandle(h.id().raw()),
                std::marker::PhantomData,
            )
        })
    }
}

/// Read actual original output through original resolve, using an explicitly
/// transferable test target instead of changing the original preset's usage.
fn readback(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    shaders: &AssetShaderLibrary,
    device: &Arc<rhapsody::device::Device>,
    source: RgTexture,
) -> rhapsody::Result<Arc<rhapsody::raw::buffer::Buffer>> {
    let [w, h] = source.desc.extent_2d();
    let target = Arc::new(rhapsody::raw::texture::Texture::new(
        device.clone(),
        &rhapsody::raw::texture::TextureDesc::new_2d(w, h, vk::Format::R8G8B8A8_UNORM)
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC),
    )?);
    let output = Arc::new(rhapsody::raw::buffer::Buffer::new(
        device.clone(),
        &rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu(
            u64::from(w * h * 4),
            vk::BufferUsageFlags::TRANSFER_DST,
        ),
    )?);
    let target_id = frame.import_native_texture(target.clone(), 0)?;
    let output_id = frame.import_native_buffer(output.clone(), 0)?;
    frame.with_raster(|host| {
        let mut factory = || -> Box<dyn PassHost> { Box::new(host.sibling()) };
        let lookup = Shaders(shaders);
        let mut ctx = DrawPassCtx::new(world, Duration::ZERO, &mut factory, &lookup);
        final_post::resolve_to_swapchain(
            &mut ctx,
            &AttachmentTexture {
                id: source.resource,
                extent: (w, h),
                format: source.desc.format,
                sample_count: 1,
            },
            &AttachmentTexture {
                id: target_id.raw(),
                extent: (w, h),
                format: 37,
                sample_count: 1,
            },
            0,
        );
        Ok(())
    })?;
    frame.with_graph(|graph| {
        let device = device.clone();
        let output = output.clone();
        let mut pass = graph.add_pass("fixture read original scene output");
        pass.add_resource_access(target_id, 0x25, 0)?;
        pass.add_resource_access(output_id, 0x34, 0)?;
        pass.render_with_result_context(move |ctx| {
            unsafe {
                device.device.cmd_copy_image_to_buffer(
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
                            width: w,
                            height: h,
                            depth: 1,
                        })],
                );
            }
            Ok(())
        });
        graph.export(output_id, 0x26)?;
        Ok(())
    })?;
    Ok(output)
}

fn value(seed: u8, x: usize, y: usize) -> u8 {
    seed + ((x / 8 + y / 8 * 3) % 13) as u8
}

// The original b903 resolve fragment fetches the linear UNORM source, applies
// this sRGB transfer, then writes the UNORM readback target. Compare its actual
// output rather than expecting the input byte to survive the original shader.
fn original_resolve_byte(linear: u8) -> u8 {
    let x = f32::from(linear) / 255.0;
    let encoded = if x <= 0.003_130_8 {
        12.92 * x
    } else {
        1.055 * x.powf(0.416_666_66) - 0.055
    };
    (encoded * 255.0).round() as u8
}

#[test]
fn six_live_rasters_upload_actual_mapped_tiles_and_reuse_their_contents() {
    let Some(mut fixture) = Fixture::new() else {
        return;
    };
    let buffers = fixture.bridge.hosts().buffers.clone();
    macro_rules! raster {
        ($r:ty, $s:ty, $seed:expr) => {{
            fixture
                .world
                .resource::<AssetSsboLibrary>()
                .add_new::<$s>(buffers.as_ref());
            fixture
                .world
                .insert_resource(<$r>::new(DirtyTilePublisher::unique()));
            let data: Vec<u8> = (0..512)
                .flat_map(|y| (0..512).map(move |x| value($seed, x, y)))
                .collect();
            fixture
                .world
                .resource_mut::<$r>()
                .write_composite(Some(&data));
            fixture.world.resource_scope(
                |world, mut raster: bevy_ecs::change_detection::Mut<$r>| {
                    let mut library = world.resource_mut::<AssetSsboLibrary>();
                    let staging = library.get_mut::<$s>().unwrap();
                    raster.copy_to_staging(0, bytemuck::cast_slice_mut(staging.mapped));
                },
            );
        }};
    }
    raster!(PathRaster, PathRasterStagingSsbo, 17);
    raster!(Cobblestone1Raster, Cobblestone1RasterStagingSsbo, 43);
    raster!(Cobblestone2Raster, Cobblestone2RasterStagingSsbo, 89);
    raster!(Cobblestone3Raster, Cobblestone3RasterStagingSsbo, 137);
    raster!(WaterRaster, WaterRasterStagingSsbo, 193);
    raster!(GardenRaster, GardenRasterStagingSsbo, 239);
    for frame_index in 0..2 {
        fixture.world.increment_change_tick();
        let mut outputs = Vec::new();
        let (_, built) = fixture
            .bridge
            .build_frame(|frame| {
                tiny_glade::frame_setup::with_frame_resources(
                    frame,
                    &mut fixture.world,
                    |frame, world| {
                        let uploaded = scene_compute::upload_world_rasters(frame, world)?;
                        assert!(uploaded.iter().all(|x| x.is_some() == (frame_index == 0)));
                        for slot in [1, 2, 3, 4, 0x13, 0x14] {
                            let source = world
                                .resource::<AssetTextureLibrary>()
                                .get_rg_texture(TexturePreset(slot));
                            outputs.push(readback(
                                frame,
                                world,
                                &fixture.shaders,
                                &fixture.device,
                                RgTexture {
                                    resource: source.resource as u32,
                                    desc: source.desc,
                                },
                            )?);
                        }
                        Ok(())
                    },
                )
            })
            .unwrap();
        assert!(!fixture.world.resource::<PathRaster>().published());
        assert!(fixture.world.resource::<PathRaster>().reupload_pending());
        fixture
            .bridge
            .shader_backend()
            .prewarm(Duration::from_secs(30))
            .unwrap();
        let receipt = fixture.bridge.execute_built(built).unwrap();
        assert_eq!(
            receipt
                .snapshot
                .pass_order
                .iter()
                .filter(|x| x.as_str() == "Upload cpu raster")
                .count(),
            if frame_index == 0 { 6 } else { 0 }
        );
        fixture.bridge.wait_submits_done(10_000_000_000).unwrap();
        for (output, seed) in outputs.into_iter().zip([17, 43, 89, 137, 193, 239]) {
            let mut bytes = vec![0; 512 * 512 * 4];
            output.read_bytes(0, &mut bytes).unwrap();
            for (x, y) in [(0, 0), (7, 9), (83, 137), (511, 511)] {
                let got = bytes[(y * 512 + x) * 4];
                let expected = original_resolve_byte(value(seed, x, y));
                assert!(
                    got.abs_diff(expected) <= 1,
                    "frame {frame_index} seed {seed} at {x},{y}: {got} != {expected}"
                );
            }
        }
    }
}

/// This also exposes an overly strict static graph that treats the shader's
/// disabled history sample as an unconditional contents read. No fake history
/// clear or blanket initialization exemption is allowed to make it pass.
#[test]
fn reprojection_first_frame_uses_false_validity_and_copies_real_depth() {
    let Some(mut fixture) = Fixture::new() else {
        return;
    };
    let mut outputs = Vec::new();
    let (_, built) = fixture
        .bridge
        .build_frame(|frame| {
            tiny_glade::frame_setup::with_frame_resources(
                frame,
                &mut fixture.world,
                |frame, world| {
                    let mut textures = Vec::new();
                    for (format, clear) in [
                        (vk::Format::R32_SFLOAT, [1.0, 0.0, 0.0, 0.0]),
                        (vk::Format::R16G16B16A16_SFLOAT, [0.0; 4]),
                    ] {
                        let texture = Arc::new(rhapsody::raw::texture::Texture::new(
                            fixture.device.clone(),
                            &rhapsody::raw::texture::TextureDesc::new_2d(16, 16, format).usage(
                                vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
                            ),
                        )?);
                        let id = frame.import_native_texture(texture.clone(), 0)?;
                        frame.with_graph(|graph| {
                            let device = fixture.device.clone();
                            let mut pass = graph.add_pass("fixture actual scene input");
                            pass.add_resource_access(id, 0x34, 0)?;
                            pass.render_with_result_context(move |ctx| {
                                unsafe {
                                    device.device.cmd_clear_color_image(
                                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                                        texture.handle,
                                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                                        &vk::ClearColorValue { float32: clear },
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
                        textures.push(RgTexture {
                            resource: id.raw(),
                            desc: TextureDesc::new_2d([16, 16], format.as_raw() as u32).sampled(),
                        });
                    }
                    let previous = frame.with_compute(|compute| {
                        compute
                            .get_or_create_temporal_texture(
                                "reprojection.prev_depth",
                                &TextureDesc::new_2d([16, 16], 100).sampled().storage(),
                            )
                            .map_err(|error| {
                                rhapsody::Error::invalid_usage(format!(
                                    "history allocation: {error}"
                                ))
                            })
                    })?;
                    assert_eq!(previous.flag, 0);
                    let map = scene_compute::SceneComputeHost::new(frame, world)
                        .calculate_reprojection_map(&textures[0], &textures[1], &previous)?;
                    frame.with_graph(|graph| {
                        let id = graph.resource_handle(map.resource)?;
                        graph.export(id, 0x19)?;
                        Ok(())
                    })?;
                    outputs.push(readback(
                        frame,
                        world,
                        &fixture.shaders,
                        &fixture.device,
                        previous.texture,
                    )?);
                    Ok(())
                },
            )
        })
        .unwrap();
    fixture
        .bridge
        .shader_backend()
        .prewarm(Duration::from_secs(30))
        .unwrap();
    let receipt = fixture.bridge.execute_built(built).unwrap();
    assert!(
        receipt
            .snapshot
            .pass_order
            .iter()
            .any(|x| x == "Reprojection map")
    );
    assert!(
        receipt
            .snapshot
            .pass_order
            .iter()
            .any(|x| x == "Copy depth")
    );
    fixture.bridge.wait_submits_done(10_000_000_000).unwrap();
    let mut bytes = vec![0; 16 * 16 * 4];
    outputs[0].read_bytes(0, &mut bytes).unwrap();
    assert!(
        bytes.chunks_exact(4).all(|p| p[0] == 255),
        "actual depth copy must reach original resolve/readback"
    );
}
