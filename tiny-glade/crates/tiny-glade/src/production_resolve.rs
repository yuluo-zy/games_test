//! Original B compile-present consumer, not an out-of-graph transfer surrogate.
//!
//! 0x140166aa6 imports the main source owned at 0x22; the acquired target is
//! the session's first-class swapchain import (discarded contents, exported as
//! Present by the library, which also injects the acquire wait and present
//! signal into whichever batches first/last touch it); 0x140166b07 calls
//! original resolve_to_swapchain.
//!
//! The presentation graph is declared on the render thread, after the main
//! graph of the same frame. It reads nothing from the game world: the resolve
//! pass binds only the main export and two uniforms, and the frame globals
//! (constants, sampled table) set up for the main graph stay installed on the
//! shared shader backend until the next main graph replaces them.
use crate::render_bridge::CoreGraphFrame;
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::shader_library::AssetShaderLibrary,
    render::pass_builder::framebuffer_binding::AttachmentTexture,
    systems::render_loop::draw_passes::{
        d_main_support::{DrawPassCtx, ShaderLookup},
        final_post,
    },
};
use std::{sync::Arc, time::Duration};

/// `scratch` is the render thread's own world; the resolve pass never reads it.
pub fn build(
    frame: &mut CoreGraphFrame<'_>,
    scratch: &mut World,
    shaders: &AssetShaderLibrary,
    elapsed: Duration,
    source: Arc<rhapsody::resource::Texture>,
    target: &rhapsody::rg::PresentTarget,
    flag: bool,
) -> rhapsody::Result<()> {
    let attachment = |id, texture: &rhapsody::resource::Texture| AttachmentTexture {
        id,
        extent: (texture.desc.width, texture.desc.height),
        format: texture.desc.format.as_raw() as u32,
        sample_count: texture.desc.sample_count,
    };
    let source_id = frame.import_native_texture_owned(source.clone(), 0x22)?;
    let source = attachment(source_id.raw(), &source);
    let target = attachment(target.resource().raw(), target.texture());
    let lookup = LibraryShaders(shaders);
    frame.with_raster(|host| {
        let mut new_host =
            || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
                Box::new(host.sibling())
            };
        let mut ctx = DrawPassCtx::new(scratch, elapsed, &mut new_host, &lookup);
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
