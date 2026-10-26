//! B scene consumer fixtures using packaged shaders and actual resource owners.
//! These bounded inputs do not assert that the full gameplay graph is ready.
#[path = "support/mod.rs"]
mod test_support;
use test_support::ShaderTestDriver as _;
use test_support::BridgeTestDriver as _;
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
) -> rhapsody::Result<Arc<rhapsody::resource::Buffer>> {
    let [w, h] = source.desc.extent_2d();
    let target = Arc::new(rhapsody::resource::Texture::new(
        device.clone(),
        &rhapsody::resource::TextureDesc::new_2d(w, h, vk::Format::R8G8B8A8_UNORM)
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC),
    )?);
    let output = Arc::new(rhapsody::resource::Buffer::new(
        device.clone(),
        &rhapsody::resource::BufferDesc::new_gpu_to_cpu(
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
                device.raw_device().cmd_copy_image_to_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    target.raw_handle(),
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    output.raw_handle(),
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
            .prepare_test_pipelines(Duration::from_secs(30))
            .unwrap();
        let receipt = fixture.bridge.execute_test_graph(built).unwrap();
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
                        let texture = Arc::new(rhapsody::resource::Texture::new(
                            fixture.device.clone(),
                            &rhapsody::resource::TextureDesc::new_2d(16, 16, format).usage(
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
                                    device.raw_device().cmd_clear_color_image(
                                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                                        texture.raw_handle(),
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
        .prepare_test_pipelines(Duration::from_secs(30))
        .unwrap();
    let receipt = fixture.bridge.execute_test_graph(built).unwrap();
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

/// Bounded original W1 -> W2/W12 consumer proof. W3..W11 are not synthesized.
/// Constant nonzero rasters expose accidental pruning of their uploaded red
/// channel and loss of the water temporary's live G channel on the third pass.
#[test]
fn live_water_and_garden_masks_consume_uploads_keep_channels_and_skip_clean_frames() {
    use bevy_ecs::change_detection::DetectChanges;
    let Some(mut fixture) = Fixture::new() else {
        return;
    };
    let buffers = fixture.bridge.hosts().buffers.clone();
    macro_rules! constant_raster {
        ($r:ty, $s:ty, $value:expr) => {{
            fixture
                .world
                .resource::<AssetSsboLibrary>()
                .add_new::<$s>(buffers.as_ref());
            fixture
                .world
                .insert_resource(<$r>::new(DirtyTilePublisher::unique()));
            fixture
                .world
                .resource_mut::<$r>()
                .write_composite(Some(&vec![$value; 512 * 512]));
            fixture.world.resource_scope(
                |world, mut raster: bevy_ecs::change_detection::Mut<$r>| {
                    let mut library = world.resource_mut::<AssetSsboLibrary>();
                    let staging = library.get_mut::<$s>().unwrap();
                    raster.copy_to_staging(0, bytemuck::cast_slice_mut(staging.mapped));
                },
            );
        }};
    }
    constant_raster!(WaterRaster, WaterRasterStagingSsbo, 73);
    constant_raster!(GardenRaster, GardenRasterStagingSsbo, 149);
    let mut last_changed = None;
    for frame_index in 0..2 {
        fixture.world.increment_change_tick();
        let tick = fixture.world.read_change_tick();
        let mut outputs = Vec::new();
        let (_, built) = fixture
            .bridge
            .build_frame(|frame| {
                tiny_glade::frame_setup::with_frame_resources(
                    frame,
                    &mut fixture.world,
                    |frame, world| {
                        for slot in [0x13, 0x14] {
                            let result = scene_compute::upload_raster_staging_to_texture(
                                frame, world, slot,
                            )?;
                            assert_eq!(result.is_some(), frame_index == 0);
                        }
                        let water = scene_compute::water_rasterize_from_cpu(frame, world)?;
                        let garden = scene_compute::garden_rasterize_from_cpu(frame, world)?;
                        assert_eq!(water, (frame_index == 0).then_some(tick.get()));
                        assert_eq!(garden, (frame_index == 0).then_some(tick.get()));
                        for slot in [0x13, 0x14] {
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
        let changed = [
            fixture
                .world
                .get_resource_ref::<WaterRaster>()
                .unwrap()
                .last_changed(),
            fixture
                .world
                .get_resource_ref::<GardenRaster>()
                .unwrap()
                .last_changed(),
        ];
        if let Some(previous) = last_changed {
            assert_eq!(
                changed, previous,
                "clean reads must not mark the source resources changed"
            );
        } else {
            assert_eq!(changed, [tick; 2]);
            last_changed = Some(changed);
        }
        assert!(!fixture.world.resource::<WaterRaster>().reupload_pending());
        assert!(!fixture.world.resource::<GardenRaster>().reupload_pending());
        assert!(fixture.world.resource::<WaterRaster>().dirty().is_empty());
        assert!(fixture.world.resource::<GardenRaster>().dirty().is_empty());
        fixture
            .bridge
            .shader_backend()
            .prepare_test_pipelines(Duration::from_secs(30))
            .unwrap();
        let receipt = fixture.bridge.execute_test_graph(built).unwrap();
        for (name, initial_count) in [
            ("Upload cpu raster", 2),
            ("Blur water mask", 4),
            ("Blur garden mask", 2),
        ] {
            let count = receipt
                .snapshot
                .pass_order
                .iter()
                .filter(|x| x.as_str() == name)
                .count();
            assert_eq!(
                count,
                if frame_index == 0 { initial_count } else { 0 },
                "original pass {name}"
            );
        }
        fixture.bridge.wait_submits_done(10_000_000_000).unwrap();
        for (kind, (output, seed)) in outputs.into_iter().zip([73, 149]).enumerate() {
            let mut bytes = vec![0; 512 * 512 * 4];
            output.read_bytes(0, &mut bytes).unwrap();
            let expected = original_resolve_byte(seed);
            for (x, y) in [(0, 0), (7, 9), (83, 137), (511, 511)] {
                let rgb = &bytes[(y * 512 + x) * 4..(y * 512 + x) * 4 + 3];
                assert!(
                    rgb[0].abs_diff(expected) <= 1,
                    "preserved red frame{frame_index} kind{kind} {x},{y}: {rgb:?} expected{expected}"
                );
                assert!(
                    rgb[1].abs_diff(expected) <= 2,
                    "original blur G frame{frame_index} kind{kind} {x},{y}: {rgb:?} expected{expected}"
                );
                if kind == 0 {
                    assert!(
                        rgb[2].abs_diff(expected) <= 2,
                        "original water B frame{frame_index} {x},{y}: {rgb:?} expected{expected}"
                    );
                } else {
                    assert_eq!(rgb[2], 0, "garden must retain its original blue channel");
                }
            }
        }
    }
}

/// Actual installed shader backend, scheduled in Startup rather than invoked
/// during renderer construction. This checks producer ownership and admission,
/// not that the full deferred scene or W35..W43 has executed.
#[test]
fn original_deferred_shader_owner_is_populated_in_gameplay_startup() {
    use country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources;
    let Some(fixture) = Fixture::new() else {
        return;
    };
    let mut app = bevy_app::App::new();
    app.insert_resource(tiny_glade::run_game::ApplicationShaderLibrary(
        fixture.shaders.clone(),
    ));
    tiny_glade::world_lighting::install_deferred_startup(
        &mut app,
        fixture.bridge.hosts().shaders.clone(),
    );
    assert!(
        app.world()
            .resource::<DeferredRasterResources>()
            .shaders
            .iter()
            .all(Option::is_none)
    );
    app.add_systems(
        bevy_app::PreStartup,
        |resources: bevy_ecs::prelude::Res<DeferredRasterResources>| {
            assert!(
                resources.shaders.iter().all(Option::is_none),
                "original producer belongs to Startup"
            );
        },
    );
    app.update();
    let handles = app.world().resource::<DeferredRasterResources>().shaders;
    for (index, _, name) in country_core::startup::startup_deferred_raster::SHADERS {
        let handle = handles[index].expect("Startup must populate each original deferred material");
        let shader = fixture
            .shaders
            .get_raster(handle)
            .expect("same App library owns shader");
        assert_eq!(shader.name, name);
    }
    assert!(handles[4].is_none());
    fixture
        .bridge
        .shader_backend()
        .prepare_test_pipelines(Duration::from_secs(30))
        .unwrap();
    app.update();
    assert_eq!(
        app.world().resource::<DeferredRasterResources>().shaders,
        handles,
        "Startup must not reload material shaders in Update"
    );
}

/// Actual original W3 shaders and real dynamic BDA/NormalDebug resources.
/// Uses Fixture's original preset producer (tag1 zero initialization, including
/// depth preset10). No extra input clear, fake camera or ActiveStrokes is added.
/// This checks original gate/tick/owner/actual submission/readback, not a full
/// terrain image oracle: no unproven expected pixel/noise value is asserted.
#[test]
fn original_terrain_strokes_empty_clean_then_event_odd_xz_use_the_same_owner() {
    use bevy_ecs::{change_detection::DetectChanges, event::Events};
    use country_core::{
        render::normals_debug::NormalDebugSsbo, resources::terrain_editing::TerrainEditingData,
        systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd,
    };
    let Some(mut fixture) = Fixture::new() else {
        return;
    };
    assert!(
        fixture.device.capabilities().buffer_device_address,
        "actual original terrain shader needs BDA"
    );
    let buffers = fixture.bridge.hosts().buffers.clone();
    fixture
        .world
        .resource::<AssetSsboLibrary>()
        .add_new::<NormalDebugSsbo>(buffers.as_ref());
    fixture.world.insert_resource(TerrainEditingData(
        tg_terrain::stroke::TerrainEditingData::default(),
    ));
    fixture
        .world
        .init_resource::<Events<RecalculateHeightMapCmd>>();
    assert!(!fixture.world.resource::<TerrainEditingData>().0.rasterized);
    assert!(
        fixture
            .world
            .resource::<TerrainEditingData>()
            .strokes
            .is_empty()
    );

    for index in 0..3 {
        if index == 2 {
            // Real stroke API, odd XYZ points with deliberately distinct Y/Z.
            // The original uploader must copy XZ; Y is not a terrain coordinate.
            let mut data = fixture.world.resource_mut::<TerrainEditingData>();
            assert!(data.0.rasterized, "previous W3 owns the same true flag");
            let id = data.new_stroke([-6.0, 77.0, -4.0]).unwrap().unwrap();
            let stroke = data.strokes.get_mut(&id).unwrap();
            stroke.points.extend([[0.0, 88.0, 1.0], [6.0, 99.0, 4.0]]);
            assert_eq!(stroke.points.len(), 3);
            assert!(
                data.0.rasterized,
                "editing does not invent a separate reset instead of the original event"
            );
            drop(data);
            fixture
                .world
                .resource_mut::<Events<RecalculateHeightMapCmd>>()
                .send(RecalculateHeightMapCmd);
        }
        fixture.world.increment_change_tick();
        let tick = fixture.world.read_change_tick();
        let mut output = None;
        let (recorded, built) = fixture
            .bridge
            .build_frame(|frame| {
                tiny_glade::frame_setup::with_frame_resources(
                    frame,
                    &mut fixture.world,
                    |frame, world| {
                        // Immutable inspection must not itself mark the resource changed.
                        let flag = world.resource::<TerrainEditingData>().0.rasterized;
                        assert_eq!(flag, index != 0);
                        let recorded =
                            tiny_glade::world_terrain::rasterize_terrain_stroke(frame, world)?;
                        assert_eq!(recorded, if index == 1 { 0 } else { 4 });
                        assert!(world.resource::<TerrainEditingData>().0.rasterized);
                        let t = world
                            .resource::<AssetTextureLibrary>()
                            .get_rg_texture(TexturePreset(9));
                        output = Some(readback(
                            frame,
                            world,
                            &fixture.shaders,
                            &fixture.device,
                            RgTexture {
                                resource: t.resource as u32,
                                desc: t.desc,
                            },
                        )?);
                        Ok(recorded)
                    },
                )
            })
            .unwrap();
        assert_eq!(recorded, if index == 1 { 0 } else { 4 });
        assert_eq!(
            fixture
                .world
                .get_resource_ref::<TerrainEditingData>()
                .unwrap()
                .last_changed(),
            tick,
            "native W3 marks changed even on a clean skip"
        );
        assert!(fixture.world.resource::<TerrainEditingData>().0.rasterized);
        assert_eq!(
            fixture.world.resource::<TerrainEditingData>().strokes.len(),
            usize::from(index == 2)
        );
        assert_eq!(
            fixture.world.resource::<TerrainEditingData>().next_index,
            u32::from(index == 2)
        );
        fixture
            .bridge
            .shader_backend()
            .prepare_test_pipelines(Duration::from_secs(30))
            .unwrap();
        let receipt = fixture.bridge.execute_test_graph(built).unwrap();
        assert!(
            !receipt.receipts.is_empty(),
            "real Vulkan submissions are mandatory"
        );
        for (name, n) in [
            ("Rasterize Terrain Stroke", 1),
            ("Blur Terrain Stroke", 2),
            ("Terrain Derivative", 1),
        ] {
            assert_eq!(
                receipt
                    .snapshot
                    .pass_order
                    .iter()
                    .filter(|p| p.as_str() == name)
                    .count(),
                if index == 1 { 0 } else { n },
                "frame{index} original {name}"
            );
        }
        fixture.bridge.wait_submits_done(10_000_000_000).unwrap();
        assert_eq!(
            fixture
                .bridge
                .with_session(|session| session.frame_import_count()),
            0,
            "same-session BDA and preset imports retire after actual completion"
        );
        let mut bytes = vec![0; 512 * 512 * 4];
        output.unwrap().read_bytes(0, &mut bytes).unwrap();
        // Source preset9 is native R16_UNORM, so readback values have a finite
        // representable range. No guessed terrain/noise image is compared.
        let checksum: u64 = bytes.iter().map(|v| u64::from(*v)).sum();
        eprintln!(
            "original W3 actual frame{index}: passes={recorded}, resolved checksum={checksum}"
        );
    }
}

/// Actual subranges produced by original DoF; unwritten coordinate partitions
/// are deliberately not read or asserted. The copy remains in the same graph.
fn read_dof_buffer_range(
    frame: &mut CoreGraphFrame<'_>,
    device: &Arc<rhapsody::device::Device>,
    source: country_core::render::pass_builder::compute_pass_builder::RgBuffer,
    offset: u64,
    size: u64,
) -> rhapsody::Result<Arc<rhapsody::resource::Buffer>> {
    use rhapsody::rg::AccessRange;
    let output = Arc::new(rhapsody::resource::Buffer::new(
        device.clone(),
        &rhapsody::resource::BufferDesc::new_gpu_to_cpu(size, vk::BufferUsageFlags::TRANSFER_DST),
    )?);
    let output_id = frame.import_native_buffer(output.clone(), 0)?;
    frame.with_graph(|graph| {
        let source_id = graph.resource_handle(source.resource)?;
        let mut pass = graph.add_pass("fixture read original DoF tile range");
        pass.add_resource_access_range(source_id, 0x25, 0, AccessRange::Buffer { offset, size })?;
        pass.add_resource_access_range(
            output_id,
            0x34,
            0,
            AccessRange::Buffer { offset: 0, size },
        )?;
        let device = device.clone();
        pass.render_with_result_context(move |ctx| {
            let managed = ctx.managed.ok_or_else(|| {
                rhapsody::Error::invalid_usage("DoF readback needs the frame's resources")
            })?;
            let source = managed
                .buffer_for(source_id.raw())
                .map_err(|e| rhapsody::Error::invalid_usage_with("DoF readback source", e))?;
            let target = managed
                .buffer_for(output_id.raw())
                .map_err(|e| rhapsody::Error::invalid_usage_with("DoF readback target", e))?;
            unsafe {
                device.raw_device().cmd_copy_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    source.raw_handle(),
                    target.raw_handle(),
                    &[vk::BufferCopy {
                        src_offset: offset,
                        dst_offset: 0,
                        size,
                    }],
                );
            }
            Ok(())
        });
        graph.export(output_id, 0x26)?;
        Ok(())
    })?;
    Ok(output)
}

fn dof_read_words(buffer: &rhapsody::resource::Buffer, word_count: usize) -> Vec<u32> {
    let mut bytes = vec![0; word_count * 4];
    buffer.read_bytes(0, &mut bytes).unwrap();
    bytes
        .chunks_exact(4)
        .map(|w| u32::from_le_bytes(w.try_into().unwrap()))
        .collect()
}

/// Bounded W51 fixture, not a substitute gameplay scene. CoC=0 is a known
/// original shader branch: all32 full-resolution tiles enter slot2, all8
/// coarser tiles enter clear-halfres slot6. Both GPU lists prove that the
/// original padded32B offsets table was uploaded and read, not just recorded.
#[test]
fn original_multires_dof_reads_uploaded_tile_offsets_and_dispatches_its_real_partitions() {
    use country_core::{
        render::load_texture::{AlignedFile, boundary::TextureDevice, parse_compiled_image},
        resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo,
        systems::render_loop::compute_passes::dof::{DofConfig, DofPrepared},
    };
    use std::collections::BTreeSet;
    let Some(mut fixture) = Fixture::new() else {
        return;
    };
    fixture
        .world
        .resource_mut::<FrameConstants>()
        .inner_rendering_resolution = [64, 32];
    let buffers = fixture.bridge.hosts().buffers.clone();
    fixture
        .world
        .resource::<AssetSsboLibrary>()
        .add_new::<SpatialResolveOffsetsSsbo>(buffers.as_ref());
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        });
    let path = installed.join("compiled-assets/textures/blue_noise.texture");
    let file = AlignedFile::read(&path).unwrap();
    let parsed = parse_compiled_image(file.bytes(), &path);
    let texture = fixture
        .bridge
        .hosts()
        .textures
        .create_texture("blue_noise", &parsed.desc, parsed.data)
        .unwrap();
    fixture
        .world
        .resource_mut::<AssetTextureLibrary>()
        .add(Some("blue_noise".to_owned()), texture);

    let (readbacks, built) = fixture
        .bridge
        .build_frame(|frame| {
            tiny_glade::frame_setup::with_frame_resources(
                frame,
                &mut fixture.world,
                |frame, world| {
                    let mut inputs = Vec::new();
                    for (dims, format, clear) in [
                        (
                            [64, 32],
                            vk::Format::B10G11R11_UFLOAT_PACK32,
                            [0.25, 0.5, 0.75, 1.0],
                        ),
                        ([64, 32], vk::Format::R32_SFLOAT, [1.0, 0.0, 0.0, 0.0]),
                        ([64, 32], vk::Format::R16_SFLOAT, [0.0; 4]),
                        ([8, 4], vk::Format::R16G16_SFLOAT, [0.0; 4]),
                    ] {
                        let texture = Arc::new(rhapsody::resource::Texture::new(
                            fixture.device.clone(),
                            &rhapsody::resource::TextureDesc::new_2d(dims[0], dims[1], format)
                                .usage(
                                    vk::ImageUsageFlags::SAMPLED
                                        | vk::ImageUsageFlags::TRANSFER_DST,
                                ),
                        )?);
                        let id = frame.import_native_texture(texture.clone(), 0)?;
                        frame.with_graph(|graph| {
                            let device = fixture.device.clone();
                            let mut pass = graph.add_pass("fixture original DoF input");
                            pass.add_resource_access(id, 0x34, 0)?;
                            pass.render_with_result_context(move |ctx| {
                                unsafe {
                                    device.raw_device().cmd_clear_color_image(
                                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                                        texture.raw_handle(),
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
                        inputs.push(RgTexture {
                            resource: id.raw(),
                            desc: TextureDesc::new_2d(dims, format.as_raw() as u32).sampled(),
                        });
                    }
                    let prepared = DofPrepared {
                        coc: inputs[2],
                        coc_tiles: inputs[3],
                    };
                    let out = scene_compute::SceneComputeHost::new(frame, world).calc_dof(
                        &inputs[0],
                        &inputs[1],
                        &prepared,
                        &DofConfig {
                            bokeh_anisotropy: 0.0,
                        },
                        true,
                        false,
                    )?;
                    let tiles = out
                        .tile_buffers
                        .expect("native multires allocates three real graph buffers");
                    let offsets =
                        read_dof_buffer_range(frame, &fixture.device, tiles.offsets, 0, 32)?;
                    let dispatch =
                        read_dof_buffer_range(frame, &fixture.device, tiles.dispatch, 0, 7 * 16)?;
                    let full_coords = read_dof_buffer_range(
                        frame,
                        &fixture.device,
                        tiles.coordinates,
                        64 * 8,
                        32 * 8,
                    )?;
                    let coarse_coords = read_dof_buffer_range(
                        frame,
                        &fixture.device,
                        tiles.coordinates,
                        120 * 8,
                        8 * 8,
                    )?;
                    let pixels =
                        readback(frame, world, &fixture.shaders, &fixture.device, out.color)?;
                    Ok([offsets, dispatch, full_coords, coarse_coords, pixels])
                },
            )
        })
        .unwrap();
    fixture
        .bridge
        .shader_backend()
        .prepare_test_pipelines(Duration::from_secs(60))
        .unwrap();
    let compiled = fixture.bridge.compile_built(built).unwrap();
    for name in [
        "DoF init buffers",
        "DoF analyze",
        "DoF clear halfres",
        "DoF halfres",
        "DoF halfres all",
        "DoF Fullres all lod0",
        "DoF Fullres all lod1",
        "DoF Fullres",
        "DoF upsample",
    ] {
        assert_eq!(
            compiled
                .snapshot()
                .pass_order
                .iter()
                .filter(|p| p.as_str() == name)
                .count(),
            1,
            "the real DoF initializer/partition must survive graph compilation: {name}"
        );
    }
    let receipt = fixture.bridge.execute_after_uploads(compiled).unwrap();
    assert!(!receipt.receipts.is_empty());
    fixture.bridge.wait_submits_done(10_000_000_000).unwrap();
    assert_eq!(
        dof_read_words(&readbacks[0], 8),
        [0, 32, 64, 96, 104, 112, 120, 0]
    );
    let commands = dof_read_words(&readbacks[1], 28);
    for (slot, command) in commands.chunks_exact(4).enumerate() {
        assert_eq!(
            command,
            [
                if slot == 2 {
                    32
                } else if slot == 6 {
                    8
                } else {
                    0
                },
                1,
                1,
                0
            ],
            "native dispatch slot {slot}"
        );
    }
    let coords = |words: Vec<u32>| {
        words
            .chunks_exact(2)
            .map(|p| (p[0], p[1]))
            .collect::<BTreeSet<_>>()
    };
    assert_eq!(
        coords(dof_read_words(&readbacks[2], 64)),
        (0u32..4)
            .flat_map(|y| (0u32..8).map(move |x| (x, y)))
            .collect::<BTreeSet<_>>()
    );
    assert_eq!(
        coords(dof_read_words(&readbacks[3], 16)),
        (0u32..2)
            .flat_map(|y| (0u32..4).map(move |x| (x, y)))
            .collect::<BTreeSet<_>>()
    );
    let mut pixels = vec![0; 64 * 32 * 4];
    readbacks[4].read_bytes(0, &mut pixels).unwrap();
    // Original resolve's independently verified linear->sRGB transfer.
    // Constant RGB is invariant under the original finite DoF sampling kernels.
    for p in pixels.chunks_exact(4) {
        for (actual, expected) in p[..3].iter().zip([137i16, 188, 225]) {
            assert!(
                (i16::from(*actual) - expected).abs() <= 2,
                "DoF output actual {p:?}"
            );
        }
        assert_eq!(p[3], 255);
    }
}

/// Actual packaged AA consumers over untouched temporal allocations. Only the
/// diagnostic current color/reprojection/depth inputs are initialized here;
/// there is deliberately no history clear and no history import-as-initialized.
#[test]
fn original_taa_and_tgsr_read_first_undefined_history_then_use_real_previous_output() {
    for use_tgsr in [false, true] {
        let Some(mut fixture) = Fixture::new() else {
            return;
        };
        for frame_index in 0..2 {
            run_original_aa_fixture_frame(&mut fixture, use_tgsr, frame_index);
        }
    }
}

fn run_original_aa_fixture_frame(
    fixture: &mut Fixture,
    use_tgsr: bool,
    frame_index: u32,
) -> (
    Arc<rhapsody::resource::Texture>,
    Option<Arc<rhapsody::resource::Texture>>,
) {
    use country_core::systems::render_loop::{
        compute_passes::taa::{HistoryTexture, TaaInputs, TgsrInputs},
        render_world::formats,
    };
    fixture.world.resource_mut::<FrameConstants>().frame_counter = frame_index;
    let device = fixture.device.clone();
    let shaders = fixture.shaders.clone();
    let ((readback_buffer, history_resource, current_resource), built) = fixture
        .bridge
        .build_frame(|frame| {
            tiny_glade::frame_setup::with_frame_resources(
                frame,
                &mut fixture.world,
                |frame, world| {
                    let mut inputs = Vec::new();
                    for (format, clear) in [
                        (vk::Format::B10G11R11_UFLOAT_PACK32, [1.0, 0.0, 0.0, 1.0]),
                        (vk::Format::R16G16B16A16_SFLOAT, [0.0; 4]),
                        (vk::Format::R32_SFLOAT, [1.0, 0.0, 0.0, 0.0]),
                    ] {
                        let texture = Arc::new(rhapsody::resource::Texture::new(
                            device.clone(),
                            &rhapsody::resource::TextureDesc::new_2d(16, 16, format).usage(
                                vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
                            ),
                        )?);
                        let id = frame.import_native_texture(texture.clone(), 0)?;
                        frame.with_graph(|graph| {
                            let device = device.clone();
                            let mut pass = graph.add_pass("fixture current AA input");
                            pass.add_resource_access(id, 0x34, 0)?;
                            pass.render_with_result_context(move |ctx| {
                                unsafe {
                                    device.raw_device().cmd_clear_color_image(
                                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                                        texture.raw_handle(),
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
                        inputs.push(RgTexture {
                            resource: id.raw(),
                            desc: TextureDesc::new_2d([16, 16], format.as_raw() as u32).sampled(),
                        });
                    }
                    let (pair, output) = frame.with_compute(|compute| {
                        let desc = TextureDesc::new_2d([16, 16], formats::COLOR_HDR)
                            .sampled()
                            .storage();
                        let pair = compute
                            .get_or_create_temporal_texture_pair(
                                if use_tgsr {
                                    "tgsr_history"
                                } else {
                                    "taa_history"
                                },
                                &desc,
                            )
                            .map_err(|e| {
                                rhapsody::Error::invalid_usage(format!("AA history: {e}"))
                            })?;
                        Ok((
                            pair,
                            compute.create_raw_texture("fixture actual AA output", &desc),
                        ))
                    })?;
                    assert_eq!(
                        pair.fresh,
                        frame_index != 0,
                        "fresh is valid-history, not allocation freshness"
                    );
                    let history = HistoryTexture {
                        tex: pair.previous,
                        valid: pair.fresh,
                    };
                    let min_subgroup_size = frame
                        .hosts()
                        .buffers
                        .graphics_device()
                        .min_subgroup_size_option();
                    let mut compute = scene_compute::SceneComputeHost::new(frame, world);
                    let aa = if use_tgsr {
                        compute.calculate_tgsr(&TgsrInputs {
                            color: &inputs[0],
                            reprojection: &inputs[1],
                            depth: &inputs[2],
                            output: &output,
                            history_output: &pair.current,
                            history: &history,
                            sharpen_amount: 0.0,
                            bypass_clamp: 0,
                            history_global: false,
                            min_subgroup_size,
                        })?
                    } else {
                        compute.calculate_taa(&TaaInputs {
                            color: &inputs[0],
                            reprojection: &inputs[1],
                            output: &output,
                            history_output: &pair.current,
                            history: &history,
                            sharpen_amount: 0.0,
                            bypass_clamp: 0,
                            history_global: false,
                        })?
                    };
                    frame.with_graph(|graph| {
                        let accesses: Vec<_> = graph
                            .declaration()
                            .passes
                            .iter()
                            .flat_map(|p| &p.accesses)
                            .filter(|a| a.resource_index == pair.previous.resource)
                            .collect();
                        assert!(
                            !accesses.is_empty(),
                            "actual history remains bound and declared"
                        );
                        assert!(accesses.iter().all(|a| !a.contents_unused));
                        assert_eq!(
                            accesses.iter().any(|a| a.allow_undefined_contents),
                            frame_index == 0
                        );
                        let source =
                            &graph.declaration().resources[pair.previous.resource as usize].source;
                        if frame_index == 0 {
                            assert!(matches!(
                                source,
                                rhapsody::rg::OriginSource::ImportOwned {
                                    initial_access: 0,
                                    ..
                                } | rhapsody::rg::OriginSource::ImportUnowned {
                                    initial_access: 0,
                                    ..
                                }
                            ));
                        }
                        Ok(())
                    })?;
                    let buffer = readback(frame, world, &shaders, &device, aa)?;
                    Ok((buffer, pair.previous.resource, pair.current.resource))
                },
            )
        })
        .unwrap();
    fixture
        .bridge
        .shader_backend()
        .prepare_test_pipelines(Duration::from_secs(30))
        .unwrap();
    let receipt = fixture.bridge.execute_test_graph(built).unwrap();
    if use_tgsr {
        for required in ["TGSR blur lum", "TGSR reproject", "TGSR"] {
            assert!(
                receipt.snapshot.pass_order.iter().any(|p| p == required),
                "missing {required}"
            );
        }
    } else {
        assert!(receipt.snapshot.pass_order.iter().any(|p| p == "TAA"));
    }
    assert!(
        !receipt.receipts.is_empty(),
        "must record and submit real AA work"
    );
    fixture.bridge.wait_submits_done(10_000_000_000).unwrap();
    let mut bytes = vec![0; 16 * 16 * 4];
    readback_buffer.read_bytes(0, &mut bytes).unwrap();
    let pixel = &bytes[(8 * 16 + 8) * 4..][..4];
    // Original TGSR main writes RGB + exp2(frexp(RGB).exponent-[8,8,7])
    // to BOTH output/history. Its first-frame clamped-zero B becomes
    // 2^-7, exactly representable in B10G11R11; original resolve then
    // encodes it to UNORM22. TAA applies that bias only to history.
    // Keep the later-frame check separate: it consumes real history.
    // Constant current color has exact zero weighted variance. On
    // frame1, neighborhood clamping collapses history to that same
    // current YUV; inverse blue is still negative -> exact0 before
    // the identical TGSR output bias. Both TGSR frames require22.
    let blue_matches = if use_tgsr {
        pixel[2] == 22
    } else {
        pixel[2] <= 3
    };
    assert!(
        pixel[0] >= 250 && pixel[1] <= 3 && blue_matches && pixel[3] == 255,
        "original AA constant red center, TGSR={use_tgsr} frame={frame_index} prev={history_resource}: {pixel:?}"
    );
    let current = fixture
        .bridge
        .exported_texture(&receipt, current_resource)
        .unwrap();
    let previous = (frame_index != 0).then(|| {
        fixture
            .bridge
            .exported_texture(&receipt, history_resource)
            .unwrap()
    });
    (current, previous)
}

/// Same current HostTables/heap/dynamic owners, real worker GPU completion,
/// then original main-session temporal adoption and an actual second TAA frame.
#[test]
fn completed_background_warmup_preserves_actual_pair_owner_parity_and_pixels() {
    let Some(mut fixture) = Fixture::new() else {
        return;
    };
    let owners = fixture.bridge.warmup_owners().unwrap();
    let device = fixture.device.clone();
    let shaders = fixture.shaders.clone();
    let world = std::mem::take(&mut fixture.world);
    let worker = std::thread::spawn(move || {
        let bridge = CoreRenderBridge::from_owners(owners).unwrap();
        let mut worker_fixture = Fixture {
            device,
            bridge,
            shaders,
            world,
        };
        let (actual_current, none) = run_original_aa_fixture_frame(&mut worker_fixture, false, 0);
        assert!(none.is_none());
        let completed = worker_fixture
            .bridge
            .complete_gameplay_warmup(10_000_000_000)
            .unwrap();
        (worker_fixture.world, completed, actual_current)
    });
    let (world, completed, actual_warmup_current) = worker.join().unwrap();
    fixture.world = world;
    // An outstanding declaration makes activation temporarily inadmissible.
    // Rejecting it must preserve the worker's actual one-shot owners/parity,
    // so activation can be retried after this graph is cancelled.
    let (_, unsubmitted) = fixture.bridge.build_frame(|_| Ok(())).unwrap();
    assert!(fixture.bridge.accept_gameplay_warmup(&completed).is_err());
    fixture.bridge.cancel_built(unsubmitted).unwrap();
    let (_, unsubmitted) = fixture.bridge.build_frame(|_| Ok(())).unwrap();
    let compiled = fixture.bridge.compile_built(unsubmitted).unwrap();
    assert!(fixture.bridge.accept_gameplay_warmup(&completed).is_err());
    drop(compiled);
    fixture.bridge.accept_gameplay_warmup(&completed).unwrap();
    // Repeated activation is harmless after the single actual ownership move.
    fixture.bridge.accept_gameplay_warmup(&completed).unwrap();
    let (_, previous) = run_original_aa_fixture_frame(&mut fixture, false, 1);
    let previous = previous.expect("same actual valid previous texture survives worker handoff");
    assert!(
        Arc::ptr_eq(&actual_warmup_current, &previous),
        "must reuse the actual GPU image, not copy content into a replacement"
    );
}

/// W54 diagnostic phases use the original capture as the real persistent
/// reference producer. Explicit fixture phases are not a gameplay state machine
/// shortcut, and no CPU image is substituted for the captured reference.
#[test]
fn original_shader_refactoring_replaces_the_full_frame_and_keeps_real_reference() {
    use country_core::resources::shader_refactoring_state::ShaderRefactoringState;
    let Some(mut fixture) = Fixture::new() else {
        return;
    };
    for phase in 0..4 {
        fixture.world.increment_change_tick();
        let (disc, payload) = if phase == 0 {
            (0, 0)
        } else if phase == 1 {
            (2, 3)
        } else {
            (3, 3)
        };
        fixture
            .world
            .insert_resource(ShaderRefactoringState { disc, payload });
        let device = fixture.device.clone();
        let shaders = fixture.shaders.clone();
        let ((output, source_id, result), built) = fixture
            .bridge
            .build_frame(|frame| {
                tiny_glade::frame_setup::with_frame_resources(
                    frame,
                    &mut fixture.world,
                    |frame, world| {
                        let texture = Arc::new(rhapsody::resource::Texture::new(
                            device.clone(),
                            &rhapsody::resource::TextureDesc::new_2d(
                                16,
                                16,
                                vk::Format::B10G11R11_UFLOAT_PACK32,
                            )
                            .usage(
                                vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
                            ),
                        )?);
                        let input = frame.import_native_texture(texture.clone(), 0)?;
                        frame.with_graph(|graph| {
                            let device = device.clone();
                            let mut pass = graph.add_pass("fixture original W54 input");
                            pass.add_resource_access(input, 0x34, 0)?;
                            pass.render_with_result_context(move |ctx| {
                                unsafe {
                                    device.raw_device().cmd_clear_color_image(
                                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                                        texture.raw_handle(),
                                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                                        &vk::ClearColorValue {
                                            float32: [
                                                if phase == 3 { 0.75 } else { 0.25 },
                                                0.5,
                                                0.75,
                                                1.0,
                                            ],
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
                            resource: input.raw(),
                            desc: TextureDesc::new_2d([16, 16], 0x7a).sampled(),
                        };
                        let result = tiny_glade::world_postprocess::render_shader_refactoring(
                            frame, world, source,
                        )?;
                        if phase < 2 {
                            assert_eq!(result, source);
                        } else {
                            assert_ne!(
                                result.resource, source.resource,
                                "must replace resource ID, not just desc bits"
                            );
                            assert_eq!(
                                result.desc,
                                TextureDesc::new_2d([16, 16], 0x7a).storage().sampled()
                            );
                        }
                        if phase == 0 {
                            assert_eq!(
                                *world.resource::<ShaderRefactoringState>(),
                                ShaderRefactoringState {
                                    disc: 0,
                                    payload: 0
                                }
                            );
                            frame.with_graph(|graph| {
                                assert!(
                                    !graph
                                        .declaration()
                                        .resources
                                        .iter()
                                        .any(|r| r.name.contains("shader refactoring"))
                                );
                                Ok(())
                            })?;
                        } else if phase == 1 {
                            assert_eq!(
                                *world.resource::<ShaderRefactoringState>(),
                                ShaderRefactoringState {
                                    disc: 2,
                                    payload: 4
                                }
                            );
                        } else {
                            assert_eq!(
                                *world.resource::<ShaderRefactoringState>(),
                                ShaderRefactoringState {
                                    disc: 3,
                                    payload: 4
                                }
                            );
                        }
                        Ok((
                            readback(frame, world, &shaders, &device, result)?,
                            source.resource,
                            result,
                        ))
                    },
                )
            })
            .unwrap();
        fixture
            .bridge
            .shader_backend()
            .prepare_test_pipelines(Duration::from_secs(30))
            .unwrap();
        let receipt = fixture.bridge.execute_test_graph(built).unwrap();
        let count = |name: &str| {
            receipt
                .snapshot
                .pass_order
                .iter()
                .filter(|p| p.as_str() == name)
                .count()
        };
        assert_eq!(count("Shader refactoring capture"), usize::from(phase != 0));
        assert_eq!(count("Shader refactoring compare"), usize::from(phase >= 2));
        fixture.bridge.wait_submits_done(10_000_000_000).unwrap();
        let mut bytes = vec![0; 16 * 16 * 4];
        output.read_bytes(0, &mut bytes).unwrap();
        let pixel = &bytes[(8 * 16 + 8) * 4..][..4];
        if phase >= 2 {
            // Exact original compare: reference RGB * .08 * (0,1,1)
            // plus red1 iff any absolute difference exceeds1e-5. Original
            // packed storage rounds G.04/B.06, then resolve applies sRGB.
            let expected = [if phase == 3 { 255i16 } else { 0 }, 56, 69, 255];
            for c in 0..4 {
                assert!(
                    (i16::from(pixel[c]) - expected[c]).abs() <= 1,
                    "phase={phase} source={source_id} result={} pixel={pixel:?} expected={expected:?}",
                    result.resource
                );
            }
        }
    }
}

/// Native temporal buffers upload zero bytes even though their first history
/// byte is false. Exercise the actual original SPD cross-workgroup path twice,
/// with a fresh base image each frame and the same physical atomic allocation.
#[test]
fn original_spd_large_mip_chain_uses_uploaded_zero_atomic_and_reuses_it_next_frame() {
    use country_core::render::ssbo::rhapsody_boundary::BufferDesc as CoreBufferDesc;
    use rhapsody::rg::AccessRange;
    let Some(mut fixture) = Fixture::new() else {
        return;
    };
    let observed = Arc::new(std::sync::Mutex::new(Vec::new()));
    for frame_index in 0..2u32 {
        let source = Arc::new(
            rhapsody::resource::Texture::new(
                fixture.device.clone(),
                &rhapsody::resource::TextureDesc::new_2d(
                    256,
                    256,
                    vk::Format::B10G11R11_UFLOAT_PACK32,
                )
                .mip_levels(9)
                .usage(
                    vk::ImageUsageFlags::STORAGE
                        | vk::ImageUsageFlags::TRANSFER_SRC
                        | vk::ImageUsageFlags::TRANSFER_DST,
                ),
            )
            .unwrap(),
        );
        let output = Arc::new(
            rhapsody::resource::Buffer::new(
                fixture.device.clone(),
                &rhapsody::resource::BufferDesc::new_gpu_to_cpu(
                    44,
                    vk::BufferUsageFlags::TRANSFER_DST,
                ),
            )
            .unwrap(),
        );
        let (_, built) = fixture
            .bridge
            .build_frame(|frame| {
                tiny_glade::frame_setup::with_frame_resources(
                    frame,
                    &mut fixture.world,
                    |frame, world| {
                        let source_id = frame.import_native_texture(source.clone(), 0)?;
                        let output_id = frame.import_native_buffer(output.clone(), 0)?;
                        // Independent diagnostic named-buffer probe uses explicit
                        // TRANSFER_SRC; the actual SPD counter keeps native usage0x20.
                        let probe = frame.with_compute(|compute| {
                            compute
                                .get_or_create_temporal_buffer(
                                    "SPD creation zero-upload diagnostic",
                                    &CoreBufferDesc::new_gpu_only(24, 0x21),
                                )
                                .map_err(|e| {
                                    rhapsody::Error::invalid_usage(format!("named zero probe: {e}"))
                                })
                        })?;
                        assert_eq!(probe.flag, frame_index as u8);
                        frame.with_graph(|graph| {
                            let mut clear = graph.add_pass("fixture initialize only SPD mip0");
                            clear.add_resource_access_range(
                                source_id,
                                0x34,
                                0,
                                AccessRange::Texture {
                                    aspect_mask: 1,
                                    base_mip_level: 0,
                                    level_count: 1,
                                    base_array_layer: 0,
                                    layer_count: 1,
                                },
                            )?;
                            let device = fixture.device.clone();
                            let source = source.clone();
                            clear.render_with_result_context(move |ctx| {
                                let color = if frame_index == 0 {
                                    [1., 0., 0., 1.]
                                } else {
                                    [0., 1., 0., 1.]
                                };
                                unsafe {
                                    device.raw_device().cmd_clear_color_image(
                                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                                        source.raw_handle(),
                                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                                        &vk::ClearColorValue { float32: color },
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
                        scene_compute::SceneComputeHost::new(frame, world).generate_mips(
                            &RgTexture {
                                resource: source_id.raw(),
                                desc: TextureDesc::new_2d([256, 256], 0x7a)
                                    .storage()
                                    .mip_levels(9),
                            },
                            "large original SPD fixture",
                        )?;
                        frame.with_graph(|graph| {
                            let atomic_index = graph
                                .declaration()
                                .resources
                                .iter()
                                .position(|resource| {
                                    resource
                                        .name
                                        .contains("SPD Atomic: large original SPD fixture")
                                })
                                .expect(
                                    "the original consumer must request its real named counter",
                                );
                            let atomic = graph.resource_handle(atomic_index as u32)?;
                            let atomic_id = graph.buffer_id(atomic)?;
                            let probe = graph.resource_handle(probe.buffer.resource)?;
                            let mut read = graph
                                .add_pass("fixture read SPD last mips and real zero-upload probe");
                            read.add_resource_access_range(
                                source_id,
                                0x25,
                                0,
                                AccessRange::Texture {
                                    aspect_mask: 1,
                                    base_mip_level: 7,
                                    level_count: 2,
                                    base_array_layer: 0,
                                    layer_count: 1,
                                },
                            )?;
                            read.add_resource_access(atomic, 0x19, 0)?;
                            read.add_resource_access(probe, 0x25, 0)?;
                            read.add_resource_access(output_id, 0x34, 0)?;
                            let device = fixture.device.clone();
                            let source = source.clone();
                            let output = output.clone();
                            let observed = observed.clone();
                            read.render_with_result_context(move |ctx| {
                                assert_eq!(ctx.buffer_history_valid(atomic_id)?, frame_index != 0);
                                let managed =
                                    ctx.managed.as_ref().expect("real graph-managed resources");
                                let counter = managed.buffer_for(atomic.raw())?;
                                assert_eq!(counter.size, 24);
                                assert!(counter.usage.contains(
                                    vk::BufferUsageFlags::STORAGE_BUFFER
                                        | vk::BufferUsageFlags::TRANSFER_DST
                                ));
                                assert!(
                                    !counter.usage.contains(vk::BufferUsageFlags::TRANSFER_SRC),
                                    "do not change the native counter usage merely to inspect it"
                                );
                                observed.lock().unwrap().push(counter.shared());
                                let probe = managed.buffer_for(probe.raw())?;
                                let command = vk::CommandBuffer::from_raw(ctx.command.handle());
                                unsafe {
                                    device.raw_device().cmd_copy_image_to_buffer(
                                        command,
                                        source.raw_handle(),
                                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                                        output.raw_handle(),
                                        &[
                                            vk::BufferImageCopy::default()
                                                .image_subresource(
                                                    vk::ImageSubresourceLayers::default()
                                                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                                                        .mip_level(7)
                                                        .layer_count(1),
                                                )
                                                .image_extent(vk::Extent3D {
                                                    width: 2,
                                                    height: 2,
                                                    depth: 1,
                                                }),
                                            vk::BufferImageCopy::default()
                                                .buffer_offset(16)
                                                .image_subresource(
                                                    vk::ImageSubresourceLayers::default()
                                                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                                                        .mip_level(8)
                                                        .layer_count(1),
                                                )
                                                .image_extent(vk::Extent3D {
                                                    width: 1,
                                                    height: 1,
                                                    depth: 1,
                                                }),
                                        ],
                                    );
                                    device.raw_device().cmd_copy_buffer(
                                        command,
                                        probe.raw_handle(),
                                        output.raw_handle(),
                                        &[vk::BufferCopy::default().dst_offset(20).size(24)],
                                    );
                                }
                                Ok(())
                            });
                            graph.export(output_id, 0x26)?;
                            Ok(())
                        })
                    },
                )
            })
            .unwrap();
        fixture
            .bridge
            .shader_backend()
            .prepare_test_pipelines(Duration::from_secs(30))
            .unwrap();
        let receipt = fixture.bridge.execute_test_graph(built).unwrap();
        assert_eq!(
            receipt
                .snapshot
                .pass_order
                .iter()
                .filter(|name| name.as_str() == "Generate mips")
                .count(),
            1
        );
        assert!(
            receipt
                .snapshot
                .pass_order
                .iter()
                .any(|name| name == "fixture initialize only SPD mip0")
        );
        fixture.bridge.wait_submits_done(10_000_000_000).unwrap();
        let mut bytes = [0u8; 44];
        output.read_bytes(0, &mut bytes).unwrap();
        let expected = if frame_index == 0 {
            0x3c0u32
        } else {
            0x3c0u32 << 11
        };
        for pixel in bytes[..20].chunks_exact(4) {
            assert_eq!(
                u32::from_le_bytes(pixel.try_into().unwrap()),
                expected,
                "original SPD last two mip levels, frame {frame_index}"
            );
        }
        assert_eq!(
            &bytes[20..],
            &[0; 24],
            "actual uploaded complete zero slice remains readable"
        );
    }
    let counters = observed.lock().unwrap();
    assert_eq!(counters.len(), 2);
    assert!(
        Arc::ptr_eq(&counters[0], &counters[1]),
        "the second frame must reuse the actual atomic allocation"
    );
}

// Separate correctness-only fixture: explicit drains/readback, no clean CPU claim.
#[cfg(feature = "cpu-probe")]
mod path_upload_receipt_correctness {
    use super::*;
    use country_core::render::ssbo::rhapsody_boundary::Buffer as CoreBuffer;
    use tiny_glade::render_host::{BufferHost, BufferUploadObservation};

    fn stage_path(world: &mut World, value: u8, initialize: bool) -> rhapsody::Result<()> {
        world.resource_scope(|world, mut raster: bevy_ecs::change_detection::Mut<PathRaster>| {
            if initialize {
                raster.write_composite(None);
            } else {
                let changed: usize = (248..264).map(|y| raster.composite()[y*512+248..y*512+264]
                    .iter().filter(|&&byte| byte != value).count()).sum();
                assert_eq!(changed, 256, "the intended local edit must actually change every ROI byte");
                for y in 248..264 { raster.composite_mut()[y*512+248..y*512+264].fill(value); }
                for y in 31..33 { for x in 31..33 { raster.dirty_mut().set_cell(x,y); } }
            }
            let mut library = world.resource_mut::<AssetSsboLibrary>();
            let staging = library.get_mut::<PathRasterStagingSsbo>().expect("real typed Path staging");
            let mut spans = 0usize;
            let result = staging.with_cpu_write_regions(|mapped, ranges| {
                let start = ranges.len();
                raster.copy_to_staging_with_ranges(0, bytemuck::cast_slice_mut(mapped), ranges);
                spans = ranges.len()-start;
                if !initialize {
                    assert_eq!(&ranges[start..], &[
                        128960..129024, 129024..129088, 133056..133120, 133120..133184]);
                }
            });
            // A failed write keeps the same real CPU shadow/pending spans.
            result?;
            if !initialize { assert_eq!(spans, 4); }
            Ok(())
        })
    }

    fn assert_target_tickets(probe: &BufferUploadObservation, target: CoreBuffer,
        value: u8, buffers: &BufferHost, device: &Arc<rhapsody::device::Device>) {
        assert!(probe.valid());
        assert_eq!(probe.target(),target);
        assert_eq!(probe.rejected_admissions(),0);
        assert_eq!(probe.entries().len(),2, "four producer tiles merge into two actual queued intervals");
        let current = buffers.managed_buffer(&target).expect("live selected logical generation").shared();
        let completed = device.poll().unwrap();
        let expected = [(128960u64,128u64),(133056,128)];
        for (entry,(offset,bytes)) in probe.entries().iter().zip(expected) {
            assert_eq!(entry.logical,target);
            assert_eq!((entry.submission.offset,entry.submission.bytes),(offset,bytes));
            assert_eq!(entry.submission.options.key.0,target.id);
            assert_eq!(entry.submission.device_generation,device.generation());
            assert_eq!(entry.payload_blake3,*blake3::hash(&[value;128]).as_bytes());
            assert!(entry.submission.same_owner(&current));
            assert!(entry.submission.owner_alive() && entry.submission.device_alive());
            match entry.submission.status() {
                rhapsody::transfer::UploadStatus::Submitted(point) => {
                    assert!(rhapsody::sync::SyncSet::from_point(point).is_reached(&completed));
                }
                state => panic!("target upload has no successful completed submit: {state:?}"),
            }
        }
    }

    fn render_path(fixture: &mut Fixture, target: CoreBuffer, upload: bool, roi_value: u8) {
        fixture.world.increment_change_tick();
        let ((staging, output_resource, output), built) = fixture.bridge.build_frame(|frame| {
            tiny_glade::frame_setup::with_frame_resources(frame,&mut fixture.world,|frame,world| {
                let uploaded = scene_compute::upload_raster_staging_to_texture(frame,world,1)?;
                assert_eq!(uploaded.is_some(),upload);
                // Use the same library cache and current graph scope as W1.
                let staging = world.resource::<AssetSsboLibrary>()
                    .get_rg_buffer::<PathRasterStagingSsbo>(&mut frame.resource_graph());
                let graph = frame.with_graph(|graph| Ok(graph.graph_id().raw()))?;
                assert_eq!(staging.generation,graph);
                let source = world.resource::<AssetTextureLibrary>().get_rg_texture(TexturePreset(1));
                let output_resource = source.resource as u32;
                let output = readback(frame,world,&fixture.shaders,&fixture.device,
                    RgTexture{resource:output_resource,desc:source.desc})?;
                Ok((staging,output_resource,output))
            })
        }).unwrap();
        let receipt = fixture.bridge.execute_test_graph(built).unwrap();
        assert!(!receipt.receipts.is_empty());
        assert!(receipt.stats.command_workload.valid);
        assert_eq!(receipt.stats.command_workload.direct_dispatch_api_calls,u64::from(upload));
        assert_eq!(receipt.stats.command_workload.indirect_dispatch_api_calls,0);
        assert_eq!(receipt.snapshot.pass_order.len(),receipt.snapshot.pass_ranges.len());
        let slots:Vec<_> = receipt.snapshot.pass_order.iter().enumerate()
            .filter(|(_,name)| name.as_str()=="Upload cpu raster").map(|(i,_)|i).collect();
        assert_eq!(slots.len(),usize::from(upload));
        if let Some(&slot)=slots.first() {
            let accesses=&receipt.snapshot.accesses[receipt.snapshot.pass_ranges[slot].clone()];
            assert!(accesses.iter().any(|a| a.resource_index==staging.resource as u32 && a.access_code==0x1a));
            assert!(accesses.iter().any(|a| a.resource_index==output_resource && a.access_code==0x32));
            let mapping=receipt.snapshot.physical_mapping.iter().find(|m|m.logical==staging.resource as u32)
                .expect("retained typed staging physical mapping");
            assert_eq!(mapping.token,Some(tiny_glade::render_host::graph_token(target.id)));
        }
        fixture.bridge.wait_submits_done(10_000_000_000).unwrap();
        let completed=fixture.device.poll().unwrap();
        for &point in &receipt.receipts {
            assert!(rhapsody::sync::SyncSet::from_point(point).is_reached(&completed));
        }
        let mut bytes=vec![0;512*512*4];output.read_bytes(0,&mut bytes).unwrap();
        for y in 0..512 { for x in 0..512 {
            let expected=if (248..264).contains(&x)&&(248..264).contains(&y) {roi_value} else {0};
            let red=bytes[(y*512+x)*4];
            assert!(red.abs_diff(original_resolve_byte(expected))<=1,"Path pixel {x},{y}: {red} vs {expected}");
        }}
    }

    #[test]
    fn path_local_edit_uses_its_queued_owner_then_real_w1_and_recovers_rejection() {
        let mut fixture=Fixture::new().expect("this correctness test requires actual Vulkan initialization");
        let buffers=fixture.bridge.hosts().buffers.clone();
        fixture.world.resource::<AssetSsboLibrary>().add_new::<PathRasterStagingSsbo>(buffers.as_ref());
        fixture.world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        let target=*fixture.world.resource::<AssetSsboLibrary>().get_buffer::<PathRasterStagingSsbo>().unwrap();
        stage_path(&mut fixture.world,0,true).unwrap();
        fixture.world.resource_mut::<AssetSsboLibrary>().flush_pending_uploads(buffers.as_ref()).unwrap();
        render_path(&mut fixture,target,true,0);

        buffers.begin_buffer_upload_observation(&target).unwrap();
        stage_path(&mut fixture.world,255,false).unwrap();
        fixture.world.resource_mut::<AssetSsboLibrary>().flush_pending_uploads(buffers.as_ref()).unwrap();
        render_path(&mut fixture,target,true,255);
        let probe=buffers.take_buffer_upload_observation().unwrap();
        assert_target_tickets(&probe,target,255,&buffers,&fixture.device);
        let charged=probe.storage_bytes();let stats=fixture.device.allocator_stats();
        let before=stats.cpu_memory_usage().total();drop(probe);
        assert!(stats.cpu_memory_usage().total()<=before-charged,"diagnostic backing credit must return after export/drop");

        // Real enqueue budget rejection before any acceptance, with restoration on unwind.
        struct Restore {stats:Arc<rhapsody::device::AllocatorStats>,limit:u64}
        impl Drop for Restore {fn drop(&mut self){let _=self.stats.set_host_budget(self.limit);}}
        buffers.begin_buffer_upload_observation(&target).unwrap();
        let guard=Restore{stats:stats.clone(),limit:stats.host_budget_limit()};
        stats.set_host_budget(stats.cpu_memory_usage().total()).unwrap();
        let rejected=stage_path(&mut fixture.world,0,false);
        let retry_while_rejected=fixture.world.resource_mut::<AssetSsboLibrary>().flush_pending_uploads(buffers.as_ref());
        drop(guard); // restore before assertions and any GPU/cleanup work
        assert_eq!(rejected.unwrap_err().kind(),rhapsody::ErrorKind::OutOfMemory);
        assert_eq!(retry_while_rejected.unwrap_err().kind(),rhapsody::ErrorKind::OutOfMemory);
        assert!(fixture.world.resource::<PathRaster>().published());
        assert!((248..264).all(|y| fixture.world.resource::<PathRaster>().composite()[y*512+248..y*512+264].iter().all(|&b|b==0)));
        let refused=buffers.take_buffer_upload_observation().unwrap();
        assert!(refused.entries().is_empty(),"pre-admission failure cannot invent a submitted ticket");
        assert!(!refused.valid());
        assert!(refused.rejected_admissions() >= 2);
        assert_eq!(buffers.queued_upload_stats().unwrap().queued_jobs, 0);
        drop(refused);
        // Queue admission failed before native submit. The real incomplete-write
        // guard must reject execution, rather than clear Path's publication flag.
        assert_eq!(buffers.poll_pending_uploads().unwrap_err().kind(), rhapsody::ErrorKind::OutOfMemory);
        assert!(fixture.world.resource::<PathRaster>().published());
        buffers.begin_buffer_upload_observation(&target).unwrap();
        // Retry the retained producer spans, with NO second logical edit.
        fixture.world.resource_mut::<AssetSsboLibrary>().flush_pending_uploads(buffers.as_ref()).unwrap();
        render_path(&mut fixture,target,true,0);
        let recovered=buffers.take_buffer_upload_observation().unwrap();
        assert_target_tickets(&recovered,target,0,&buffers,&fixture.device);drop(recovered);
        render_path(&mut fixture,target,false,0);
    }
}
