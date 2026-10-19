//! Original B compile-present consumer, not an out-of-graph transfer surrogate.
//!
//! 0x140166aa6 imports the main source owned at 0x22; the acquired target is
//! the session's first-class swapchain import (discarded contents, exported as
//! Present by the library, which also injects the acquire wait and present
//! signal into whichever batches first/last touch it); 0x140166b07 calls
//! original resolve_to_swapchain.
use crate::{
    frame_world::{FrameLibraries, FrameWorldAccess},
    render_bridge::CoreGraphFrame,
};
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::{
        placement_data_library::AssetPlacementDataLibrary, shader_library::AssetShaderLibrary,
        ssbo_library::AssetSsboLibrary, texture_library::AssetTextureLibrary,
        vao_library::AssetVAOLibrary,
    },
    render::pass_builder::{framebuffer_binding::AttachmentTexture, pass_resources::PassResources},
    systems::render_loop::draw_passes::{
        d_main_support::{DrawPassCtx, ShaderLookup},
        final_post,
    },
};
use std::sync::Arc;

pub fn build(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    shaders: &AssetShaderLibrary,
    source: Arc<rhapsody::raw::texture::Texture>,
    target: &rhapsody::rg::vulkan::PresentTarget,
    flag: bool,
) -> rhapsody::Result<()> {
    crate::frame_setup::with_frame_resources(frame, world, |frame, world| {
        build_inner(frame, world, shaders, source, target, flag)
    })
}
fn build_inner(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    shaders: &AssetShaderLibrary,
    source: Arc<rhapsody::raw::texture::Texture>,
    target: &rhapsody::rg::vulkan::PresentTarget,
    flag: bool,
) -> rhapsody::Result<()> {
    let attachment = |id, texture: &rhapsody::raw::texture::Texture| AttachmentTexture {
        id,
        extent: (texture.desc.width, texture.desc.height),
        format: texture.desc.format.as_raw() as u32,
        sample_count: texture.desc.sample_count,
    };
    let source_id = frame.import_native_texture_owned(source.clone(), 0x22)?;
    let source = attachment(source_id.raw(), &source);
    let target = attachment(target.resource().raw(), target.texture());
    // Preserve the original mandatory PassResources lookup, using confirmed live
    // types. No synthetic empty libraries are installed for presentation.
    let raster = frame.raster_resources();
    {
        let required =
            |name| rhapsody::Error::invalid_usage(format!("B compile-present: missing {name}"));
        let access = FrameWorldAccess::new(
            FrameLibraries {
                shaders,
                textures: world
                    .get_resource::<AssetTextureLibrary>()
                    .ok_or_else(|| required("AssetTextureLibrary"))?,
                ssbos: world
                    .get_resource::<AssetSsboLibrary>()
                    .ok_or_else(|| required("AssetSsboLibrary"))?,
                vaos: world
                    .get_resource::<AssetVAOLibrary>()
                    .ok_or_else(|| required("AssetVAOLibrary"))?,
                placement: world
                    .get_resource::<AssetPlacementDataLibrary>()
                    .ok_or_else(|| required("AssetPlacementDataLibrary"))?,
                constants: raster.dynamic_constants(),
            },
            frame.compute_device(),
        )
        .with_resource_graph_factory(frame.resource_graph_factory());
        let _resources = PassResources::new_from(&access);
    }
    let elapsed = world
        .get_resource::<bevy_time::Time>()
        .map(|time| time.elapsed())
        .unwrap_or_default();
    let lookup = LibraryShaders(shaders);
    frame.with_raster(|host| {
        let mut new_host =
            || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
                Box::new(host.sibling())
            };
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
        final_post::resolve_to_swapchain(&mut ctx, &source, &target, u8::from(flag));
        Ok(())
    })
}
struct LibraryShaders<'a>(&'a AssetShaderLibrary);
impl ShaderLookup for LibraryShaders<'_> {
    fn raster_handle_by_name(
        &self,
        name: &str,
    ) -> Option<
        country_core::rhapsody_contract::Handle<country_core::render::shader::RasterShaderProgram>,
    > {
        self.0.get_raster_handle_by_name(name).map(|handle| {
            country_core::rhapsody_contract::Handle(
                country_core::rhapsody_contract::ManagedHandle(handle.id().raw()),
                core::marker::PhantomData,
            )
        })
    }
}
