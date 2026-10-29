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
    rhapsody_types::TextureDesc,
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

#[path = "loading_background_prepare.rs"]
mod background_prepare;
pub use background_prepare::LoadingBackgroundStatus;

#[derive(Resource)]
pub struct LoadingGpuResources {
    pub mei: MeiHost,
    background_preparation: background_prepare::BackgroundPreparation,
}

/// A render-thread snapshot: the MEI view of this frame; background
/// preparation is polled on the game thread only.
impl Clone for LoadingGpuResources {
    fn clone(&self) -> Self {
        Self { mei: self.mei.render_snapshot(), background_preparation: Default::default() }
    }
}

/// The native managed owner survives the loading→gameplay App move. Graph
/// indices and TextureHost slot numbers are deliberately not persisted here.
#[derive(Clone, Resource)]
pub(crate) struct SavedLoadingBackgroundTexture(pub Arc<rhapsody::resource::Texture>);

pub(crate) fn import_saved_background(
    frame: &mut CoreGraphFrame<'_>, world: &World,
) -> rhapsody::Result<Option<RgTexture>> {
    let Some(owner) = world.get_resource::<SavedLoadingBackgroundTexture>() else { return Ok(None); };
    let texture = owner.0.clone();
    let desc = TextureDesc::new_2d([texture.desc.width, texture.desc.height], texture.desc.format.as_raw() as u32).sampled();
    // The existing import checks exact Device identity and retains this Arc
    // through graph cancellation, submission and GPU completion.
    let resource = frame.import_native_texture_owned(texture, 0x22)?;
    Ok(Some(RgTexture { resource: resource.raw(), desc }))
}

pub(crate) fn refresh_saved_background_reference(
    state: &mut VermeerState, imported: Option<RgTexture>,
) -> rhapsody::Result<()> {
    if state.background.is_some() {
        let imported = imported.ok_or_else(|| rhapsody::Error::invalid_usage(
            "Vermeer background reference has no saved managed texture owner in this App"))?;
        state.background = Some(vermeer::ManagedTexture(
            country_core::systems::render_loop::painterly::TexRef(imported.resource)));
    }
    Ok(())
}

pub(crate) fn validate_saved_background_handoff(from: &World, to: &World) -> Result<()> {
    if !from.get_resource::<VermeerState>().is_some_and(|state| state.background.is_some()) {
        return Ok(());
    }
    let owner = from.get_resource::<SavedLoadingBackgroundTexture>()
        .context("loading→gameplay: Vermeer background is Some but its managed texture owner is missing")?;
    for (world, name) in [(from, "loading"), (to, "gameplay")] {
        let device = world.get_resource::<country_core::render::device::GraphicsDeviceResource>()
            .with_context(|| format!("loading→gameplay: {name} background GraphicsDeviceResource is missing"))?;
        anyhow::ensure!(Arc::ptr_eq(owner.0.device(), device.device().inner()),
            "loading→gameplay: saved background belongs to a different {name} graphics device");
    }
    anyhow::ensure!(!to.contains_resource::<SavedLoadingBackgroundTexture>(),
        "loading→gameplay: gameplay App already owns another saved reveal background");
    Ok(())
}

/// Called immediately before the first loading render, and before later loading
/// updates while pending. Freeze the original first-session choice once. This
/// only starts/polls owned CPU work; it never uploads or waits for a worker/GPU.
/// Actual loading rendering still requires its Startup owners; CPU-only frame
/// fixtures without loading GPU resources have no optional preparation work.
pub fn poll_background_preparation(world: &mut World) -> Result<bool> {
    // CPU-only frame-driver fixtures do not install loading GPU owners. Actual
    // loading rendering retains its mandatory Startup/resource validation.
    let Some(gpu) = world.get_resource::<LoadingGpuResources>() else { return Ok(true); };
    if gpu.background_preparation.is_ready() { return Ok(true); }
    let path = if gpu.background_preparation.needs_selection() {
        world.get_resource::<SavedSessionList>()
            .and_then(|list| list.sessions.first()).map(|session| session.loading_background_path())
    } else { None };
    let mut gpu = world.get_resource_mut::<LoadingGpuResources>()
        .context("loading background: original loading Startup owner is missing")?;
    Ok(gpu.background_preparation.poll(path).map_err(anyhow::Error::msg)? == LoadingBackgroundStatus::Ready)
}

pub fn is_background_preparation_pending(world: &World) -> bool {
    world.get_resource::<LoadingGpuResources>()
        .is_some_and(|gpu| gpu.background_preparation.is_pending())
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
        initialize_common_assets(world, &asset_hosts, &installed, demo, true)
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
    initialize_common_assets(world, hosts, installed, demo, true)?;
    attach_loading_mei(world, device, hosts)
}

/// The unchanged asset/font producer body. These independent Startup values do
/// not depend on canonical text SSBO creation or on the MEI host attachment.
///
/// `bootup_only` selects the texture filter. The original has a
/// `TextureLoadingFilter` trait whose only symbolised impl is
/// `LoadOnlyBootupTextures::should_load` (@1409e6950); the loading App uses it.
/// The gameplay App must make every compiled texture resolvable (UI icons such as
/// `menu/settings_icon` are looked up by name every frame), so it loads all of them.
/// OPEN: the gameplay-side filter type has no symbol of its own (constant-true
/// `should_load` inlines away), so "load everything" is inferred from the trait
/// name and from the UI's by-name lookups, not read from a call site.
pub(crate) fn initialize_common_assets(
    world: &mut World, hosts: &HostTables, installed: &Path, demo: bool, bootup_only: bool,
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
        if bootup_only && !country_core::startup::load_textures::should_load(&name, demo) { continue; }
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
    // final_post's fixed file key (xxh3("tony_mc_mapface")) and menu/active_lut are
    // ordinary compiled textures; the gameplay App's full texture load
    // (`initialize_common_assets(.., bootup_only = false)`) already registered them.
    // Loading them again here would hit the library's duplicate-name check.
    world.insert_resource(luts);
    Ok(())
}

/// 将MEI接到同设备的既有资产与缓冲owner，供宿主和嵌入式App共用。
pub fn attach_loading_mei(
    world: &mut World, device: Arc<rhapsody::device::Device>, hosts: &HostTables,
) -> Result<()> {
    anyhow::ensure!(!world.contains_resource::<LoadingGpuResources>(), "loading GPU resources already initialized");
    let library = world.get_resource::<AssetSsboLibrary>()
        .context("loading App AssetSsboLibrary producer has not run")?;
    let mut mei = MeiHost::from_library(device, hosts.buffers.clone(), hosts.textures.clone(), library)?;
    let files = toolbar::mei::widgets::FileImageCache::new(hosts.textures.clone());
    mei.retain_file_images(files.retained_textures());
    world.insert_resource(files);
    world.insert_resource(LoadingGpuResources { mei,
        background_preparation: background_prepare::BackgroundPreparation::default() });
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
    if !poll_background_preparation(world).map_err(|error|
        rhapsody::Error::invalid_usage(format!("loading background preparation failed: {error:#}")))?
    {
        return Err(rhapsody::Error::invalid_usage(
            "loading background CPU preparation is pending; defer the first loading render before advancing reveal"));
    }
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
    // Original optional background is attempted once. The host must keep the
    // reveal/Vermeer clocks stopped while CPU preparation is pending; absence
    // and pending are different states. Upload only a genuinely decoded image
    // through this App's exact TextureHost before the first loading draw.
    if let Some(rgba) = gpu.background_preparation.take_ready().map_err(|error| fail(error))? {
        match frame.hosts().textures.create_texture("loading background",
            &TextureDesc::new_2d([rgba.width(), rgba.height()], 0x2b).sampled(), &rgba)
        {
            Ok(texture) => {
                let native = frame.hosts().textures.get(&texture).ok_or_else(||
                    fail("loading background upload returned a texture outside the original host"))?.shared();
                // This one-off upload is not a named texture-library entry.
                // Retain the actual Arc instead of pinning a loading-host slot.
                frame.hosts().textures.release(&texture);
                world.insert_resource(SavedLoadingBackgroundTexture(native));
            }
            Err(error) => log::warn!(target: "tiny_glade::startup",
                "Optional loading background GPU upload failed; retaining original absent-background fallback: {error}"),
        }
    }
    let target = frame.with_compute(|f| Ok(f.create_raw_texture("loading screen", &TextureDesc::new_2d(extent, 0x2b))))?;
    let background = import_saved_background(frame, world)?;
    let timer = {
        let mut state = world.get_resource_mut::<VermeerState>().ok_or_else(|| fail("B loading graph: VermeerPlugin state is missing"))?;
        assert_ne!(state.phase, VermeerPhase::Disabled, "loading screen requires activated Vermeer state");
        refresh_saved_background_reference(&mut state, background)?;
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
    Arc::make_mut(&mut raster.assets.lock().collect).textures_by_name.insert(0x579f_39ab_91a0_1865,
        (country_core::rhapsody_contract::ManagedHandle(blue.id), country_core::render::pass_builder::framebuffer_binding::AttachmentTexture::from(blue)));
    let lookup = LibraryShaders(shaders);
    frame.with_raster(|host| {
        let mut new_host = || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
            Box::new(host.sibling())
        };
        let mut ctx = DrawPassCtx::new(world, time, &mut new_host, &lookup);
        // B compile_loading 0x140165347 clears r9d before mei_pass:
        // cast_ui_shadows=false draws directly onto the loading target.
        // Only the gameplay caller derives this flag from EscMenuState.
        ui_2d_pass::mei_pass_with_shadows(&mut ctx, &attachment(target), None, (extent[0], extent[1]), &text);
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
        Arc::make_mut(&mut raster.assets.lock().collect).rg_buffers.insert(<ColorLibrarySsbo as SsboType>::rg_buffer_hash(),
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


/// Retry the already consumed frame without advancing text or simulation.
pub(crate) fn poll_mei_uploads(world: &mut World) -> Result<bool> {
    world.get_resource_mut::<LoadingGpuResources>()
        .map_or(Ok(true), |mut gpu| gpu.mei.poll_pending_uploads())
}

/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.resource::<SavedLoadingBackgroundTexture>();
    m.resource::<VermeerState>();
    m.resource::<country_core::render::device::GraphicsDeviceResource>();
    m.resource::<LoadingGpuResources>();
    m.resource::<SavedSessionList>();
    m.asset_library::<AssetSsboLibrary>();
    m.resource::<country_core::asset_libraries::color_library::ColorLibrarySsbo>();
    m.asset_library::<AssetTextureLibrary>();
    m.resource::<country_core::resources::grading_lut::GradingLuts>();
    m.resource::<WindowSize>();
    m.presence::<tg_text::TextRenderer>();
    m.resource::<bevy_time::Time>();
    m.asset_library::<AssetVAOLibrary>();
    m.asset_library::<AssetPlacementDataLibrary>();
}

#[cfg(test)]
mod saved_background_reference_tests {
    use super::*;

    fn imported(resource: u32) -> RgTexture {
        // Pure graph-view metadata fixture; this does not claim a GPU owner.
        RgTexture { resource, desc: TextureDesc::new_2d([2, 1], 0x2b).sampled() }
    }

    #[test]
    fn saved_background_reference_is_refreshed_for_each_graph_namespace() {
        use country_core::systems::render_loop::painterly::TexRef;
        let mut state = VermeerState::default();
        state.background = Some(vermeer::ManagedTexture(TexRef(999)));
        refresh_saved_background_reference(&mut state, Some(imported(7))).unwrap();
        assert_eq!(state.background.as_ref().unwrap().0, TexRef(7));
        // The next App/frame may reuse graph indices for completely different
        // resources. Its real import's new index must replace the old one.
        refresh_saved_background_reference(&mut state, Some(imported(42))).unwrap();
        assert_eq!(state.background.as_ref().unwrap().0, TexRef(42));
    }

    #[test]
    fn background_some_without_managed_owner_is_rejected_not_erased() {
        use country_core::systems::render_loop::painterly::TexRef;
        let mut state = VermeerState::default();
        state.background = Some(vermeer::ManagedTexture(TexRef(999)));
        assert!(refresh_saved_background_reference(&mut state, None).is_err());
        assert_eq!(state.background.as_ref().unwrap().0, TexRef(999));
    }

    #[test]
    fn original_no_background_never_gains_a_graph_reference() {
        let mut state = VermeerState::default();
        refresh_saved_background_reference(&mut state, None).unwrap();
        assert!(state.background.is_none());
    }
}
