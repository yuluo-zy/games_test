//! B loading graph closure 0x1401647b0, with original RNE call-site evidence.
//! Target format is fixed 0x2b (0x140165134), not the swapchain format. The
//! ordered consumers are Vermeer (0x14016532a), MEI (0x14016534a), loading draw
//! list (0x14016536e), and sampled export 0x22 (0x1401653b7).
//! Native absolute installation paths are adapted to an explicit source-build
//! asset root. No screen or primitive stream is fabricated here.
use std::{path::{Path, PathBuf}, sync::Arc};

use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::{
        placement_data_library::AssetPlacementDataLibrary, shader_library::AssetShaderLibrary,
        ssbo_library::AssetSsboLibrary, texture_library::AssetTextureLibrary,
        vao_library::AssetVAOLibrary,
    },
    name_hash::NameHash,
    render::{load_texture::{AlignedFile, parse_compiled_image, boundary::TextureDevice},
        pass_builder::{pass_resource_access::RgTexture, rg_resources::RgResources}},
    resources::{session::SavedSessionList, window::WindowSize},
    rhapsody_types::{Texture, TextureDesc},
    systems::render_loop::{
        draw_passes::{d_main_support::{DrawPassCtx, ShaderLookup}, draw_lists::DrawLists,
            ui_2d_pass, ui_pass},
        painterly::TimeView,
        render_world,
        vermeer::{self, VermeerPhase, VermeerRenderParams, VermeerState},
    },
};

use crate::{effects_host::{EffectsHost, attachment}, frame_world::{FrameLibraries, FrameWorldAccess},
    mei_host::{MeiHost, load_text_fonts},
    render_bridge::CoreGraphFrame, render_host::HostTables};

#[derive(Resource)]
pub struct LoadingGpuResources {
    pub mei: MeiHost,
    background_attempted: bool,
    background: Option<Texture>,
}

/// Register the original loading Startup subset on this App's real owners.
/// MinimalStartupPlugin's five entries are an unchained tuple. Asset loading
/// stays independent here; only canonical SSBO producer -> host attachment has
/// an explicit data dependency. No Update/flush path creates missing buffers.
pub fn install_loading_startup(
    app: &mut bevy_app::App, device: Arc<rhapsody::device::Device>, hosts: HostTables,
    installed: PathBuf, demo: bool,
) -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::startup::init_populate_minimal_startup_ssbo_library::minimal_startup_ssbo_system;
    let library = app.world().get_resource::<AssetSsboLibrary>()
        .context("loading Startup: actual AssetSsboLibrary is not installed")?;
    anyhow::ensure!(
        !library.contains::<ui_2d_pass::TextPrimSsbo>() && !library.contains::<ui_2d_pass::ShapeStreamSsbo>(),
        "loading minimal Startup must not replace already registered canonical text buffers");
    anyhow::ensure!(app.world().contains_resource::<country_core::render::device::GraphicsDeviceResource>(),
        "loading Startup: actual GraphicsDeviceResource is not installed");
    anyhow::ensure!(Arc::ptr_eq(&device, hosts.buffers.graphics_device()),
        "loading Startup belongs to another BufferHost device");
    let asset_hosts = hosts.clone();
    app.add_systems(bevy_app::Startup, move |world: &mut World| {
        initialize_common_assets(world, &asset_hosts, &installed, demo)
            .expect("original loading asset Startup");
    });
    app.add_systems(bevy_app::Startup, (
        minimal_startup_ssbo_system,
        (move |world: &mut World| {
            attach_loading_mei(world, device.clone(), &hosts)
                .expect("original loading canonical MEI Startup attachment");
        }).after(minimal_startup_ssbo_system),
    ));
    Ok(())
}

/// Direct helper for explicit loading Worlds/tests. The caller must already
/// run the same canonical minimal producer (gameplay uses its full producer).
/// This helper never fills a missing library entry with a private allocation.
pub fn initialize_loading_resources(
    world: &mut World, device: Arc<rhapsody::device::Device>, hosts: &HostTables,
    installed: &Path, demo: bool,
) -> Result<()> {
    initialize_common_assets(world, hosts, installed, demo)?;
    attach_loading_mei(world, device, hosts)
}

/// The unchanged asset/font producer body. These independent Startup values do
/// not depend on canonical text SSBO creation or on the MEI host attachment.
pub(crate) fn initialize_common_assets(
    world: &mut World, hosts: &HostTables, installed: &Path, demo: bool,
) -> Result<()> {
    anyhow::ensure!(!world.contains_resource::<country_core::asset_libraries::color_library::ColorLibrarySsbo>(),
        "loading ColorLibrary startup producer already ran");
    anyhow::ensure!(world.contains_resource::<country_core::asset_libraries::configwatch::ConfigWatch>(),
        "loading App ConfigWatch producer has not run");
    let palette = world.resource_scope(|world, mut watcher: Mut<country_core::asset_libraries::configwatch::ConfigWatch>| {
        let ssbos = world.get_resource::<AssetSsboLibrary>()
            .context("loading App AssetSsboLibrary producer has not run")?;
        Ok::<_, anyhow::Error>(country_core::startup::init_color_library::init_color_library(
            hosts.buffers.as_ref(), ssbos, &mut watcher))
    })?;
    world.insert_resource(palette);
    let mut textures = world.get_resource_mut::<AssetTextureLibrary>()
        .context("loading App AssetTextureLibrary producer has not run")?;
    let root = installed.join("compiled-assets/textures");
    anyhow::ensure!(root.is_dir(), "missing compiled texture directory: {}", root.display());
    for entry in walkdir::WalkDir::new(&root).into_iter() {
        let entry = entry?;
        if !entry.file_type().is_file() || entry.path().extension().and_then(|x| x.to_str()) != Some("texture") { continue; }
        let name = entry.path().strip_prefix(&root)?.with_extension("");
        if !country_core::startup::load_textures::should_load(&name, demo) { continue; }
        // The packaged NameHash keys use '/' on every host. A Windows Path's
        // native '\\' must not change the identity of nested logo/menu assets.
        let name = name.to_str().context("compiled texture asset name is not UTF-8")?
            .replace('\\', "/");
        let file = AlignedFile::read(entry.path())?;
        let image = parse_compiled_image(file.bytes(), entry.path());
        let texture = hosts.textures.create_texture(&name, &image.desc, image.data)?;
        textures.add(Some(name), texture);
    }
    let mut text = tg_text::TextRenderer::new();
    load_text_fonts(&mut text, installed)?;
    world.insert_resource(text);
    Ok(())
}

/// Gameplay LUT producer; the independent loading App only loads common assets.
/// GradingLuts::load @140b01df0 retains every successfully parsed 3D LUT. Publish
/// those exact owners under final_post/gallery's canonical asset names.
pub(crate) fn initialize_grading_assets(
    world: &mut World, hosts: &HostTables, installed: &Path,
) -> Result<()> {
    use country_core::resources::grading_lut::{GradingLuts, lut_name_to_asset_name};
    anyhow::ensure!(!world.contains_resource::<GradingLuts>(),
        "gameplay GradingLuts startup producer already ran");
    let luts = GradingLuts::load(hosts.textures.as_ref(), &installed.join("assets/luts"));
    let mut textures = world.get_resource_mut::<AssetTextureLibrary>()
        .context("gameplay AssetTextureLibrary producer has not run")?;
    for (meta, record) in luts.metas.iter().zip(&luts.textures) {
        textures.add(Some(lut_name_to_asset_name(&meta.name)), record.texture.clone());
    }
    // final_post's fixed file key is xxh3("tony_mc_mapface"), not a named
    // grading_lut alias. Decode the packaged descriptor and texels unchanged.
    for name in ["tony_mc_mapface", "menu/active_lut"] {
        let path = installed.join("compiled-assets/textures").join(name).with_extension("texture");
        let file = AlignedFile::read(&path)?;
        let image = parse_compiled_image(file.bytes(), &path);
        let texture = hosts.textures.create_texture(name, &image.desc, image.data)?;
        textures.add(Some(name.to_owned()), texture);
    }
    world.insert_resource(luts);
    Ok(())
}

pub(crate) fn attach_loading_mei(
    world: &mut World, device: Arc<rhapsody::device::Device>, hosts: &HostTables,
) -> Result<()> {
    anyhow::ensure!(!world.contains_resource::<LoadingGpuResources>(), "loading GPU resources already initialized");
    let library = world.get_resource::<AssetSsboLibrary>()
        .context("loading App AssetSsboLibrary producer has not run")?;
    let mei = MeiHost::from_library(device, hosts.buffers.clone(), hosts.textures.clone(), library)?;
    world.insert_resource(LoadingGpuResources { mei,
        background_attempted: false, background: None });
    Ok(())
}

/// Original text_flush_system consumes once per game update. This must run
/// after genuine text producers and before graph compilation, so OOM retries
/// reuse the same uploaded stream instead of consuming it twice.
pub fn flush_mei_frame(world: &mut World) {
    let result: Result<()> = world.resource_scope(|world, mut gpu: Mut<LoadingGpuResources>| {
        let window = *world.get_resource::<WindowSize>().context("WindowSize")?;
        let count = world.get_resource::<AssetTextureLibrary>().context("AssetTextureLibrary")?.len();
        let mut text = world.get_resource_mut::<tg_text::TextRenderer>().context("TextRenderer")?;
        gpu.mei.prepare(&mut text, [window.width, window.height], count)?;
        Ok(())
    });
    result.expect("text_flush_system");
}

pub fn build_loading_frame(
    frame: &mut CoreGraphFrame<'_>, world: &mut World, shaders: &AssetShaderLibrary,
    extent: [u32; 2], _swapchain_format: u32, lists: &DrawLists,
) -> rhapsody::Result<render_world::RgTexture> {
    crate::frame_setup::with_frame_resources(frame, world, |frame, world|
        build_loading_with_resources(frame, world, shaders, extent, lists))
}

fn build_loading_with_resources(frame: &mut CoreGraphFrame<'_>, world: &mut World,
    shaders: &AssetShaderLibrary, extent: [u32; 2], lists: &DrawLists,
) -> rhapsody::Result<render_world::RgTexture> {
    let mut gpu = world.remove_resource::<LoadingGpuResources>().ok_or_else(||
        rhapsody::Error::invalid_usage("B loading graph: rendering-resource/text startup producers have not run"))?;
    let result = build_loading(frame, world, shaders, extent, lists, &mut gpu);
    world.insert_resource(gpu);
    result
}

fn build_loading(
    frame: &mut CoreGraphFrame<'_>, world: &mut World, shaders: &AssetShaderLibrary,
    extent: [u32; 2], lists: &DrawLists, gpu: &mut LoadingGpuResources,
) -> rhapsody::Result<render_world::RgTexture> {
    let fail = |what| rhapsody::Error::invalid_usage(what);
    if extent.contains(&0) { return Err(fail("B loading graph requires nonzero extent")); }
    let dt = world.get_resource::<bevy_time::Time>().ok_or_else(|| fail("B loading graph: Time<()> is missing"))?.delta_secs();
    let time = world.resource::<bevy_time::Time>().elapsed();
    // O: the process-static optional image is attempted once. Missing save list,
    // image decoding or optional GPU-image creation keeps it absent.
    if !gpu.background_attempted {
        gpu.background_attempted = true;
        let path = world.get_resource::<SavedSessionList>()
            .and_then(|list| list.sessions.first()).map(|s| s.loading_background_path());
        if let Some(path) = path {
            if let Ok(image) = image::open(path) {
                let rgba = image.to_rgba8();
                gpu.background = frame.hosts().textures.create_texture("loading background",
                    &TextureDesc::new_2d([rgba.width(), rgba.height()], 0x2b).sampled(), &rgba).ok();
            }
        }
    }
    let target = frame.with_compute(|f| Ok(f.create_raw_texture("loading screen", &TextureDesc::new_2d(extent, 0x2b))))?;
    let background = if let Some(texture) = &gpu.background {
        let id = frame.import_texture(texture, 0x22)?;
        Some(RgTexture { resource: id.raw(), desc: TextureDesc::new_2d([texture.width, texture.height], texture.format).sampled() })
    } else { None };
    let timer = {
        let mut state = world.get_resource_mut::<VermeerState>().ok_or_else(|| fail("B loading graph: VermeerPlugin state is missing"))?;
        assert_ne!(state.phase, VermeerPhase::Disabled, "loading screen requires activated Vermeer state");
        state.timer += dt.min(1.0 / 30.0);
        if let Some(background) = background {
            // This source-level view is refreshed to this frame's real imported
            // index; the physical managed owner stays in the process cache.
            state.background = Some(vermeer::ManagedTexture(country_core::systems::render_loop::painterly::TexRef(background.resource)));
        }
        state.timer
    };
    let backend = frame.shader_backend();
    let raster = frame.raster_resources();
    {
        let vaos = world.get_resource::<AssetVAOLibrary>().ok_or_else(|| fail("AssetVAOLibrary"))?;
        let placement = world.get_resource::<AssetPlacementDataLibrary>().ok_or_else(|| fail("AssetPlacementDataLibrary"))?;
        let textures = world.get_resource::<AssetTextureLibrary>().ok_or_else(|| fail("AssetTextureLibrary"))?;
        let ssbos = world.get_resource::<AssetSsboLibrary>().ok_or_else(|| fail("AssetSsboLibrary"))?;
        let access = FrameWorldAccess::new(FrameLibraries { shaders, textures,
            ssbos, vaos, placement, constants: raster.dynamic_constants() }, frame.compute_device())
            .with_resource_graph_factory(frame.resource_graph_factory());
        let source = textures.get_by_name(NameHash(vermeer::BACKGROUND_NAME_DEFAULT));
        let desc = TextureDesc::new_2d([source.width, source.height], source.format).sampled();
        let source_id = frame.import_texture(source, 0x22)?;
        let source_texture = RgTexture { resource: source_id.raw(), desc };
        let mut effects = EffectsHost::new(frame, &access, &*backend, shaders, None,
            Some(TimeView { delta_secs_f64: f64::from(dt), delta_secs: dt }), None, false);
        let target_ref = effects.register_texture(target);
        let background_ref = background.map(|t| effects.register_texture(t));
        // into_sampled_rg_texture (B 0x140bd6470) retains the managed source and
        // uses sampled access 0x22; ownership is provided by this same session.
        effects.register_named_texture(vermeer::BACKGROUND_NAME_DEFAULT, source_texture);
        vermeer::vermeer_render_from_target(&mut effects, VermeerRenderParams { background: background_ref,
            source: None, target: target_ref, timer, progress: 0.0, photomode: false,
            skip_generate: false, reuse: true });
        effects.finish()?;
    }
    gpu.mei.import_into_frame(frame, &raster)?;
    let text = gpu.mei.output().cloned().ok_or_else(|| fail("B loading graph: text_flush_system has not produced an atlas/stream"))?;
    // Disk textures remain actual host handles, not graph indices.
    let textures = world.get_resource::<AssetTextureLibrary>().ok_or_else(|| fail("AssetTextureLibrary"))?;
    let blue = textures.get_by_name(NameHash(0x579f_39ab_91a0_1865));
    raster.assets.lock().textures_by_name.insert(0x579f_39ab_91a0_1865,
        (country_core::rhapsody_contract::ManagedHandle(blue.id), country_core::render::pass_builder::framebuffer_binding::AttachmentTexture::from(blue)));
    let temp = frame.with_compute(|f| Ok(f.create_raw_texture("mei temp", &TextureDesc::new_2d(extent, 0x2b))))?;
    let lookup = LibraryShaders(shaders);
    frame.with_raster(|host| {
        let mut new_host = || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
            Box::new(host.sibling())
        };
        let mut ctx = DrawPassCtx::new(world, time, &mut new_host, &lookup);
        ui_2d_pass::mei_pass(&mut ctx, &attachment(target), &attachment(temp), (extent[0], extent[1]), &text);
        Ok(())
    })?;
    if !lists.list(country_core::systems::render_loop::draw_passes::draw_lists::list::LOADING_SCREEN).is_empty() {
        // The original list consumer always calls bind_common<ColorLibrarySsbo>
        // (run_list_pass → bind_color_library @0x1416286f0). Resolve that actual
        // typed library entry only when this consumer has draws, using the same
        // frame facade as compute. Its compatibility hash is not a GPU handle.
        use country_core::{asset_libraries::color_library::ColorLibrarySsbo,
            render::pass_builder::pass_resource_access::SsboType};
        let ssbos = world.get_resource::<AssetSsboLibrary>().ok_or_else(|| fail("AssetSsboLibrary"))?;
        let buffer = ssbos.get_rg_buffer::<ColorLibrarySsbo>(&mut frame.resource_graph());
        raster.assets.lock().rg_buffers.insert(<ColorLibrarySsbo as SsboType>::rg_buffer_hash(),
            u32::try_from(buffer.resource).map_err(|_| fail("ColorLibrary graph index overflow"))?);
    }
    frame.with_raster(|host| {
        let mut new_host = || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
            Box::new(host.sibling())
        };
        let mut ctx = DrawPassCtx::new(world, time, &mut new_host, &lookup);
        ui_pass::loading_screen_pass(&mut ctx, &attachment(target), lists);
        Ok(())
    })?;
    frame.with_graph(|graph| { let id = graph.resource_handle(target.resource)?; graph.export(id, 0x22)?; Ok(()) })?;
    Ok(render_world::RgTexture { id: target.resource, desc: render_world::TextureDesc::new_2d(extent, 0x2b) })
}

struct LibraryShaders<'a>(&'a AssetShaderLibrary);
impl ShaderLookup for LibraryShaders<'_> {
    fn raster_handle_by_name(&self, name: &str) -> Option<country_core::rhapsody_contract::Handle<country_core::render::shader::RasterShaderProgram>> {
        self.0.get_raster_handle_by_name(name).map(|h| country_core::rhapsody_contract::Handle(
            country_core::rhapsody_contract::ManagedHandle(h.id().raw()), core::marker::PhantomData))
    }
}
