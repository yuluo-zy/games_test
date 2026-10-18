//! Original list-driven world draws on the current App's production raster host.
//! The actual caller supplies the real DrawLists and graph targets. Missing
//! startup, geometry or shaders are errors; no list is synthesized here.
use crate::render_bridge::CoreGraphFrame;
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::shader_library::AssetShaderLibrary,
    render::pass_builder::{draw_stream::PassHost, framebuffer_binding::AttachmentTexture},
    resources::{
        color_id::WallColorIdSsbo,
        rt::{RtBvhSsbo, RtTriangleMetaSsbo, RtTrianglesSsbo},
        shader_refactoring_state::ShaderRefactoringState,
    },
    systems::render_loop::draw_passes::{
        d_main_support::{DrawPassCtx, ShaderLookup},
        deferred_raster::{self, DeferredInputs},
        draw_lists::DrawLists,
        main_pass::{self, MainPassTargets},
        shadow_pass, transparency_pass,
    },
};

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

/// Shared current-world raster adapter; direct query passes do not need DrawLists.
pub(crate) fn with_draw_context<T>(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    build: impl FnOnce(&mut DrawPassCtx<'_>) -> T,
) -> rhapsody::Result<T> {
    let shaders = frame.shader_library();
    let lookup = Shaders(&shaders);
    let elapsed = world
        .get_resource::<bevy_time::Time>()
        .ok_or_else(|| {
            rhapsody::Error::invalid_usage("original world raster pass: missing Time producer")
        })?
        .elapsed();
    frame.with_raster(|host| {
        let mut factory = || -> Box<dyn PassHost> { Box::new(host.sibling()) };
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut factory, &lookup);
        Ok(build(&mut ctx))
    })
}

pub struct SceneRasterHost<'f, 's, 'w, 'l> {
    frame: &'f mut CoreGraphFrame<'s>,
    world: &'w mut World,
    lists: &'l DrawLists,
}
impl<'f, 's, 'w, 'l> SceneRasterHost<'f, 's, 'w, 'l> {
    pub fn new(
        frame: &'f mut CoreGraphFrame<'s>,
        world: &'w mut World,
        lists: &'l DrawLists,
    ) -> rhapsody::Result<Self> {
        frame.raster_resources().refresh_draw_vaos(world, lists)?;
        Ok(Self {
            frame,
            world,
            lists,
        })
    }

    fn with_ctx<T>(
        &mut self,
        build: impl FnOnce(&mut DrawPassCtx<'_>, &DrawLists) -> T,
    ) -> rhapsody::Result<T> {
        with_draw_context(self.frame, self.world, |ctx| build(ctx, self.lists))
    }

    pub fn main_pass(
        &mut self,
        targets: &MainPassTargets,
        emissive: &AttachmentTexture,
    ) -> rhapsody::Result<()> {
        let disabled = self
            .world
            .get_resource::<ShaderRefactoringState>()
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage(
                    "original main_pass: missing ShaderRefactoringState producer",
                )
            })?
            .disabled();
        self.with_ctx(|ctx, lists| main_pass::main_pass(ctx, targets, emissive, lists, disabled))
    }
    pub fn main_pass_oc2(
        &mut self,
        targets: &MainPassTargets,
        emissive: &AttachmentTexture,
    ) -> rhapsody::Result<()> {
        self.with_ctx(|ctx, lists| main_pass::main_pass_oc2(ctx, targets, emissive, lists))
    }
    pub fn late_forward_and_deferred_pass(
        &mut self,
        targets: &MainPassTargets,
    ) -> rhapsody::Result<()> {
        self.with_ctx(|ctx, lists| main_pass::late_forward_and_deferred_pass(ctx, targets, lists))
    }
    pub fn shadow_pass(&mut self) -> rhapsody::Result<()> {
        self.with_ctx(shadow_pass::shadow_pass)
    }
    pub fn deferred_raster(&mut self, inputs: &DeferredInputs<'_>) -> rhapsody::Result<()> {
        self.with_ctx(|ctx, _| {
            deferred_raster::deferred_raster::<
                RtTrianglesSsbo,
                RtTriangleMetaSsbo,
                RtBvhSsbo,
                WallColorIdSsbo,
            >(ctx, inputs)
        })
    }
    pub fn refraction_copy_pass(
        &mut self,
        color: &AttachmentTexture,
        source: &AttachmentTexture,
        depth: &AttachmentTexture,
    ) -> rhapsody::Result<()> {
        self.with_ctx(|ctx, lists| {
            transparency_pass::refraction_copy_pass(ctx, color, source, depth, lists)
        })
    }
    pub fn pre_taa_transparency_pass(
        &mut self,
        color: &AttachmentTexture,
        depth: &AttachmentTexture,
        b: &AttachmentTexture,
        c: &AttachmentTexture,
    ) -> rhapsody::Result<()> {
        self.with_ctx(|ctx, lists| {
            transparency_pass::pre_taa_transparency_pass(ctx, color, depth, b, c, lists)
        })
    }
    pub fn post_taa_transparency_pass(
        &mut self,
        color: &AttachmentTexture,
    ) -> rhapsody::Result<()> {
        self.with_ctx(|ctx, lists| transparency_pass::post_taa_transparency_pass(ctx, color, lists))
    }
}
