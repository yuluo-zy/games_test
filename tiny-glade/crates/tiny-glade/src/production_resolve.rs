//! Original B compile-present consumer, not an out-of-graph transfer surrogate.
//!
//! 0x140166aa6 imports the main source owned at 0x22; 0x140166aca imports the
//! acquired target unowned at 0; 0x140166b07 calls original resolve_to_swapchain;
//! 0x140166b4c exports target at 0x27. The raw borrowed wrapper and WSI submit
//! injection adapt native managed ownership to this source-built Vulkan host.
use std::sync::Arc;
use bevy_ecs::world::World;
use country_core::{asset_libraries::{shader_library::AssetShaderLibrary,
    texture_library::AssetTextureLibrary, ssbo_library::AssetSsboLibrary,
    vao_library::AssetVAOLibrary, placement_data_library::AssetPlacementDataLibrary},
    render::pass_builder::{framebuffer_binding::AttachmentTexture, pass_resources::PassResources},
    systems::render_loop::draw_passes::{d_main_support::{DrawPassCtx, ShaderLookup}, final_post}};
use crate::{render_bridge::CoreGraphFrame, frame_world::{FrameWorldAccess, FrameLibraries}};

pub fn build(frame: &mut CoreGraphFrame<'_>, world: &mut World, shaders: &AssetShaderLibrary,
    source: Arc<rhapsody::raw::texture::Texture>, target: Arc<rhapsody::raw::texture::Texture>, flag: bool,
) -> rhapsody::Result<()> {
    crate::frame_setup::with_frame_resources(frame, world, |frame, world|
        build_inner(frame, world, shaders, source, target, flag))
}
fn build_inner(frame: &mut CoreGraphFrame<'_>, world: &mut World, shaders: &AssetShaderLibrary,
    source: Arc<rhapsody::raw::texture::Texture>, target: Arc<rhapsody::raw::texture::Texture>, flag: bool,
) -> rhapsody::Result<()> {
    let attachment = |id, texture: &rhapsody::raw::texture::Texture| AttachmentTexture {
        id, extent: (texture.desc.width, texture.desc.height), format: texture.desc.format.as_raw() as u32,
        sample_count: texture.desc.sample_count,
    };
    let source_id = frame.import_native_texture_owned(source.clone(), 0x22)?;
    let target_id = frame.import_native_texture(target.clone(), 0)?;
    let source = attachment(source_id.raw(), &source);
    let target = attachment(target_id.raw(), &target);
    // Preserve the original mandatory PassResources lookup, using confirmed live
    // types. No synthetic empty libraries are installed for presentation.
    let raster = frame.raster_resources();
    {
        let required = |name| rhapsody::Error::invalid_usage(format!("B compile-present: missing {name}"));
        let access = FrameWorldAccess::new(FrameLibraries { shaders,
            textures: world.get_resource::<AssetTextureLibrary>().ok_or_else(|| required("AssetTextureLibrary"))?,
            ssbos: world.get_resource::<AssetSsboLibrary>().ok_or_else(|| required("AssetSsboLibrary"))?,
            vaos: world.get_resource::<AssetVAOLibrary>().ok_or_else(|| required("AssetVAOLibrary"))?,
            placement: world.get_resource::<AssetPlacementDataLibrary>().ok_or_else(|| required("AssetPlacementDataLibrary"))?,
            constants: raster.dynamic_constants(),
        }, frame.compute_device()).with_resource_graph_factory(frame.resource_graph_factory());
        let _resources = PassResources::new_from(&access);
    }
    let elapsed = world.get_resource::<bevy_time::Time>().map(|time| time.elapsed()).unwrap_or_default();
    let lookup = LibraryShaders(shaders);
    frame.with_raster(|host| {
        let mut new_host = || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
            Box::new(host.sibling())
        };
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
        final_post::resolve_to_swapchain(&mut ctx, &source, &target, u8::from(flag));
        Ok(())
    })?;
    frame.with_graph(|graph| graph.export(target_id, 0x27).map_err(Into::into))
}
struct LibraryShaders<'a>(&'a AssetShaderLibrary);
impl ShaderLookup for LibraryShaders<'_> {
    fn raster_handle_by_name(&self, name: &str) -> Option<country_core::rhapsody_contract::Handle<country_core::render::shader::RasterShaderProgram>> {
        self.0.get_raster_handle_by_name(name).map(|handle| country_core::rhapsody_contract::Handle(
            country_core::rhapsody_contract::ManagedHandle(handle.id().raw()), core::marker::PhantomData))
    }
}
