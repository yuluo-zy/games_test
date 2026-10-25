//! Execute the original B loading closure with packaged assets and shaders.
//! The text is a bounded test input; the logo meshes/shaders and their draw
//! callbacks come from the recovered original startup producer. This does not
//! prove that gameplay's asynchronous load or complete startup UI has finished.
#[path = "support/mod.rs"]
mod test_support;
use test_support::ShaderTestDriver as _;
use test_support::BridgeTestDriver as _;
use ash::vk::{self, Handle};
use country_core::{
    asset_libraries::{
        configwatch::ConfigWatch, placement_data_library::AssetPlacementDataLibrary,
        shader_library::AssetShaderLibrary,
    },
    resources::window::WindowSize,
    systems::render_loop::{
        draw_passes::draw_lists::DrawLists,
        painterly::ShaderLoader,
        vermeer::{self, VermeerState},
    },
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tiny_glade::{
    loading_graph::{self, LoadingGpuResources},
    render_bridge::CoreRenderBridge,
};

#[test]
fn packaged_loading_background_vermeer_and_actual_text_stream_execute_and_read_back() {
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        });
    let installed = installed.canonicalize().unwrap();
    // This integration-test process has one test. Match the original process's
    // installation cwd for its original relative mesh paths and restore it on
    // return/unwind; no installed assets are modified.
    struct RestoreDirectory(PathBuf);
    impl Drop for RestoreDirectory {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.0).unwrap();
        }
    }
    let _restore_directory = RestoreDirectory(std::env::current_dir().unwrap());
    std::env::set_current_dir(&installed).unwrap();
    let device =
        match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "strict original loading GPU initialization failed: {error}"
                );
                eprintln!("SKIP original loading GPU initialization: {error}");
                return;
            }
        };
    let shaders = Arc::new(AssetShaderLibrary::new());
    let mut bridge = CoreRenderBridge::new(device.clone(), &installed, shaders.clone()).unwrap();
    country_core::startup::load_shaders::load_ui_shaders(&shaders, bridge.shader_backend());
    struct Loader<'a>(&'a CoreRenderBridge, &'a AssetShaderLibrary);
    impl ShaderLoader for Loader<'_> {
        fn load_shader_into_library(&mut self, path: &'static str, name: &'static str) {
            country_core::startup::load_shaders::load_shader_into_library(
                self.1,
                self.0.shader_backend(),
                path,
                name,
            );
        }
    }
    vermeer::startup_vermeer(&mut Loader(&bridge, &shaders));
    let extent = if let Some(value) = std::env::var_os("TG_LOADING_SIZE") {
        let value = value.to_str().expect("TG_LOADING_SIZE must be UTF-8 WIDTHxHEIGHT");
        let (width, height) = value.split_once('x').expect("TG_LOADING_SIZE must be WIDTHxHEIGHT");
        let extent = [width.parse::<u32>().expect("invalid TG_LOADING_SIZE width"),
            height.parse::<u32>().expect("invalid TG_LOADING_SIZE height")];
        assert!(!extent.contains(&0), "TG_LOADING_SIZE dimensions must be positive");
        extent
    } else {
        [320, 180]
    };
    let mut app = bevy_app::App::new();
    {
        let world = app.world_mut();
        bridge.install_asset_libraries(world).unwrap();
        // B's loading App constructs ConfigWatch(false); initialize_loading_resources
        // then invokes the recovered ColorLibrary producer on this bridge's device.
        world.insert_resource(ConfigWatch::new(false));
        world.insert_resource(country_core::render::frame_constants::FrameConstants::default());
        world.insert_resource(AssetPlacementDataLibrary::new());
        world.insert_resource(WindowSize::new(extent[0], extent[1]));
        let mut clock = bevy_time::Time::<()>::default();
        clock.advance_by(Duration::from_secs_f32(1.0 / 60.0));
        world.insert_resource(clock);
        let mut state = VermeerState::default();
        state.activate(true, true);
        world.insert_resource(state);
    }
    // Exercise the production Startup registration, not a test-only pair of
    // manual buffers. The exact minimal producer must finish before Update44.
    assert!(!app.world().contains_resource::<LoadingGpuResources>());
    loading_graph::install_loading_startup(
        &mut app,
        device.clone(),
        bridge.hosts().clone(),
        installed.clone(),
        false,
    )
    .unwrap();
    use bevy_ecs::{
        prelude::{Res, Resource},
        schedule::IntoScheduleConfigs,
    };
    use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
    use country_core::systems::render_loop::draw_passes::ui_2d_pass::{
        ShapeStreamSsbo, TextPrimSsbo,
    };
    #[derive(Resource, Default)]
    struct StartupProbe(bool);
    app.init_resource::<StartupProbe>();
    app.add_systems(
        bevy_app::Update,
        (
            |library: Res<AssetSsboLibrary>,
             _gpu: Res<LoadingGpuResources>,
             mut probe: bevy_ecs::prelude::ResMut<StartupProbe>| {
                assert!(
                    library.contains::<TextPrimSsbo>() && library.contains::<ShapeStreamSsbo>()
                );
                probe.0 = true;
            },
        )
            .in_set(tiny_glade::run_game::GameSystemSet(44)),
    );
    app.update();
    // This fixture has no SavedSessionList: the genuine optional background
    // preparation completes immediately, without creating a substitute image.
    assert!(loading_graph::poll_background_preparation(app.world_mut()).unwrap());
    assert!(
        app.world().resource::<StartupProbe>().0,
        "canonical MEI attachment must exist before Update44"
    );
    let mut world = std::mem::take(app.world_mut());
    let (text_prim_buffer, shape_stream_buffer) = {
        let library = world.resource::<AssetSsboLibrary>();
        (
            library.get::<TextPrimSsbo>().unwrap().buffer,
            library.get::<ShapeStreamSsbo>().unwrap().buffer,
        )
    };
    world.insert_resource(country_core::asset_libraries::meshwatch::MeshWatch::new(
        false,
    ));
    world.insert_resource(country_core::resources::logo_animation::LogoAnimation::default());
    country_core::resources::logo_animation::startup_animated_logo(
        &mut world,
        bridge.shader_backend(),
        &shaders,
    )
    .unwrap();
    let mut maintenance = bevy_ecs::schedule::Schedule::default();
    country_core::asset_libraries::runtime::add_asset_library_systems(&mut maintenance);
    maintenance.run(&mut world);
    let mut lists = DrawLists::new();
    country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists(
        &mut world,
        shaders.as_ref(),
        &mut lists,
    );
    assert_eq!(
        lists
            .list(country_core::systems::render_loop::draw_passes::draw_lists::list::LOADING_SCREEN)
            .len(),
        2,
        "both original startup logo meshes must reach their real draw consumer"
    );
    bridge
        .raster_resources()
        .refresh_draw_vaos(&mut world, &lists)
        .unwrap();
    {
        let table = tg_text::FontTable::from_ron(
            &std::fs::read_to_string(installed.join("assets/fonts/table.ron")).unwrap(),
        )
        .unwrap();
        let mut text = world.resource_mut::<tg_text::TextRenderer>();
        text.set_pixels_per_point(extent[1] as f32 / 1000.0)
            .unwrap();
        let mut fragment =
            tg_text::TextFragment::new(32.0, tg_text::FontFamily::Label, Arc::new(table)).unwrap();
        fragment.set_text(&mut text, "Tiny Glade").unwrap();
        fragment.prepare(&mut text).unwrap();
        fragment
            .render(&mut text, [160.0, 90.0], [300.0, 80.0], u32::MAX)
            .unwrap();
    }
    loading_graph::flush_mei_frame(&mut world);
    assert!(
        world
            .resource::<LoadingGpuResources>()
            .mei
            .output()
            .unwrap()
            .prim_count
            > 0,
        "the MEI stream must come from actual glyph producers"
    );
    let byte_count = extent[0] as usize * extent[1] as usize * 4;
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
    let (output, built) = bridge
        .build_frame(|frame| {
            let output = loading_graph::build_loading_frame(
                frame, &mut world, &shaders, extent, 43, &lists,
            )?;
            use country_core::render::pass_builder::pass_resource_access::SsboType;
            let primitives = frame.import_buffer(&text_prim_buffer, 0x37)?;
            let shapes = frame.import_buffer(&shape_stream_buffer, 0x37)?;
            let raster = frame.raster_resources();
            let assets = raster.assets.lock();
            assert_eq!(
                assets.rg_buffers.get(&TextPrimSsbo::rg_buffer_hash()),
                Some(&primitives.raw())
            );
            assert_eq!(
                assets.rg_buffers.get(&ShapeStreamSsbo::rg_buffer_hash()),
                Some(&shapes.raw())
            );
            drop(assets);
            let buffer = frame.import_native_buffer(readback.clone(), 0)?;
            frame.with_graph(|graph| {
                let texture = graph.resource_handle(output.id)?;
                let readback = readback.clone();
                let device = device.clone();
                let mut copy = graph.add_pass("fixture readback of original loading output");
                copy.add_resource_access(texture, 0x25, 0)?;
                copy.add_resource_access(buffer, 0x34, 0)?;
                copy.render_with_result_context(move |ctx| {
                    let source = ctx
                        .managed
                        .ok_or_else(|| {
                            rhapsody::Error::invalid_usage("missing managed loading output")
                        })?
                        .texture_for(texture.raw())?;
                    unsafe {
                        device.raw_device().cmd_copy_image_to_buffer(
                            vk::CommandBuffer::from_raw(ctx.command.handle()),
                            source.raw(),
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
                graph.export(buffer, 0x26)?;
                Ok(())
            })?;
            Ok(output)
        })
        .unwrap();
    bridge
        .shader_backend()
        .prepare_test_pipelines(Duration::from_secs(30))
        .unwrap();
    let compiled = bridge.compile_built(built).unwrap();
    let receipt = bridge.execute_after_uploads(compiled).unwrap();
    assert!(
        !receipt.receipts.is_empty(),
        "loading requires real Vulkan submissions"
    );
    for name in [
        vermeer::PASS_GENERATE,
        vermeer::PASS_RASTER_STROKES,
        vermeer::PASS_COMBINE,
        "Mei 2D",
        "UI",
    ] {
        assert!(
            receipt.snapshot.pass_order.iter().any(|pass| pass == name),
            "missing original pass: {name}"
        );
    }
    // Original loading caller 0x140165347: r9d=0 (cast_ui_shadows=false).
    // Its sequence at 0x14016532a/34a/36e is Vermeer, direct MEI, Logo UI.
    // Gameplay alone derives the shadow branch from EscMenuState.
    let order = &receipt.snapshot.pass_order;
    assert!(!order.iter().any(|name| name == "UI shadow combine"),
        "the original loading caller must not select gameplay's MEI shadow branch");
    let position = |name: &str| order.iter().position(|pass| pass == name).unwrap();
    assert!(position(vermeer::PASS_COMBINE) < position("Mei 2D"));
    assert!(position("Mei 2D") < position("UI"));
    bridge.wait_submits_done(10_000_000_000).unwrap();
    let owner = bridge.exported_texture(&receipt, output.id).unwrap();
    assert_eq!([owner.desc.width, owner.desc.height], extent);
    let mut pixels = vec![0; byte_count];
    readback.read_bytes(0, &mut pixels).unwrap();
    let logo = world.resource::<country_core::resources::logo_animation::LogoAnimation>();
    eprintln!(
        "LOADING FIXTURE {}x{} logo_progress={:?}; one Time tick=1/60, default FrameConstants, manual glyph input; no async handoff or welcome UI",
        extent[0], extent[1], logo.progress,
    );
    // Save the actual final loading target after Vermeer, MEI and both logo
    // draw-list entries. Unlike any-RGB!=0 below, the artifact and region facts
    // let the reviewer distinguish a background from a single visible glyph.
    if let Some(path) = std::env::var_os("TG_LOADING_PIXEL_OUT").map(PathBuf::from) {
        if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent).unwrap();
        }
        match path.extension().and_then(|extension| extension.to_str()) {
            Some("png") => {
                image::RgbaImage::from_raw(extent[0], extent[1], pixels.clone()).unwrap()
                    .save_with_format(&path, image::ImageFormat::Png).unwrap();
            }
            Some("ppm") => {
                let mut ppm = format!("P6\n{} {}\n255\n", extent[0], extent[1]).into_bytes();
                for pixel in pixels.chunks_exact(4) { ppm.extend_from_slice(&pixel[..3]); }
                std::fs::write(&path, ppm).unwrap();
            }
            _ => panic!("TG_LOADING_PIXEL_OUT must end in .png or .ppm"),
        }
        eprintln!("LOADING FINAL PIXELS: {}", path.display());
    }
    for (label, x0, y0, x1, y1) in [
        ("full", 0, 0, extent[0], extent[1]),
        ("top-left", 0, 0, extent[0] / 4, extent[1] / 4),
        ("bottom-band", 0, extent[1] * 3 / 4, extent[0], extent[1]),
    ] {
        let mut colors = std::collections::HashSet::new();
        let mut sum = 0u64;
        let mut count = 0u64;
        let mut black = 0u64;
        let mut alpha = [u8::MAX, u8::MIN];
        for y in y0..y1 {
            for x in x0..x1 {
                let pixel = &pixels[((y as usize * extent[0] as usize + x as usize) * 4)..][..4];
                let rgb = u32::from(pixel[0]) + u32::from(pixel[1]) + u32::from(pixel[2]);
                sum += u64::from(rgb);
                count += 1;
                black += u64::from(rgb == 0);
                colors.insert([pixel[0], pixel[1], pixel[2]]);
                alpha = [alpha[0].min(pixel[3]), alpha[1].max(pixel[3])];
            }
        }
        if count != 0 {
            eprintln!("LOADING REGION {label}: pixels={count} mean_rgb={:.3} black_fraction={:.6} distinct={} alpha={alpha:?}",
                sum as f64 / (3.0 * count as f64), black as f64 / count as f64, colors.len());
        }
    }
    assert!(
        pixels
            .chunks_exact(4)
            .any(|pixel| pixel[0] != 0 || pixel[1] != 0 || pixel[2] != 0),
        "actual packaged background and glyph draws must produce visible RGB data"
    );
    // The packaged background is opaque, and both native logo shaders emit
    // premultiplied color. In particular the initial transparent logo quad
    // must preserve its background, including alpha, rather than overwrite it
    // with RGBA=0. The previous missing UI blend produced a 243x243 alpha-zero
    // black rectangle at 960x540 despite the any-visible-RGB assertion passing.
    assert_eq!(logo.progress, [0.0; 5]);
    assert!(
        pixels.chunks_exact(4).all(|pixel| pixel[3] == 255),
        "transparent logo pixels must preserve the opaque packaged loading background"
    );
    // Releasing a library-backed consumer must retain the authoritative slots.
    drop(world.remove_resource::<LoadingGpuResources>());
    assert!(
        bridge
            .hosts()
            .buffers
            .managed_buffer(&text_prim_buffer)
            .is_some()
    );
    assert!(
        bridge
            .hosts()
            .buffers
            .managed_buffer(&shape_stream_buffer)
            .is_some()
    );
}
