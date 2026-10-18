//! Original render_world W44..46 on the enclosing current graph and App owners.
//! Compute preparation/finish release their read-only WorldAccess before the
//! actual list callbacks receive &mut World. No graph, cache or owner is copied.
use std::cell::RefCell;
use bevy_ecs::world::World;
use country_core::{
    render::{
        pass_builder::{
            compute_pass_builder::{ComputePass, RecordedComputePass, RgBuffer},
            pass_resource_access::RgTexture,
            rg_resources::{RgError, RgResources, TemporalBuffer, TemporalTexture, TemporalTexturePair},
        },
        rg_helpers::rg_boundary::{RenderFn, RenderGraph, StagingAlloc, StagingCopy},
        ssbo::rhapsody_boundary::BufferDesc,
    },
    resources::session::GladeType,
    rhapsody_types::TextureDesc,
    systems::render_loop::{
        compute_passes::pass_kit::PassEnv,
        draw_passes::{draw_lists::DrawLists, transparency_pass::{
            self, SurfaceCompute, SurfaceEnv, SurfaceInputs,
            surface_ice::{prepare_ice_surface, raster_ice_surface, finish_ice_surface},
            surface_water::{prepare_water_surface, raster_water_surface, finish_water_surface},
        }},
        render_world::{formats, names},
    },
};
use crate::{
    effects_host::attachment,
    render_bridge::CoreGraphFrame,
    scene_compute::with_env,
    scene_raster::{SceneRasterHost, with_draw_context},
    world_lighting::WorldLightingOutputs,
};

/// Actual original glade and process-level branch byte14362c098. The outer
/// recovered caller supplies both; there is no fabricated/default scene state.
#[derive(Clone, Copy, Debug)]
pub struct SurfaceControls {
    pub glade: GladeType,
    pub skip_refraction: bool,
}

/// Original refraction copy -> selected water/ice -> pre-TAA transparency.
/// Caller already owns main FrameSetup and W35..43. The same color/depth are
/// mutated in place and then passed unchanged to W47..54.
pub fn build_world_surface(
    frame: &mut CoreGraphFrame<'_>, world: &mut World, lists: &DrawLists,
    lighting: &WorldLightingOutputs, controls: SurfaceControls,
) -> rhapsody::Result<()> {
    let color = lighting.geometry.color;
    let depth = lighting.geometry.gbuffer.depth;
    let half = lighting.half.depth;
    if !controls.skip_refraction {
        let refraction = frame.with_compute(|compute| Ok(compute.create_raw_texture(
            names::REFRACTION_SOURCE, &TextureDesc::new_2d(color.desc.extent_2d(), formats::COLOR_HDR),
        )))?;
        SceneRasterHost::new(frame, world, lists)?.refraction_copy_pass(
            &attachment(refraction), &attachment(color), &attachment(depth),
        )?;
        let inputs = SurfaceInputs {
            color: &color, depth: &depth, half_depth: &half,
            taa_history: &lighting.history.previous,
            history_valid: u8::from(lighting.history.fresh),
            refraction: &refraction, reprojection: &lighting.reprojection, lists,
        };
        match transparency_pass::select_water_surface(controls.glade) {
            transparency_pass::WaterSurfaceKind::Water => {
                let prepared = with_surface_env(frame, world, |env| Ok(prepare_water_surface(env, &inputs)))?;
                with_draw_context(frame, world, |ctx| raster_water_surface(ctx, &prepared, &inputs))?;
                with_surface_env(frame, world, |env| { finish_water_surface(env, prepared, &inputs); Ok(()) })?;
            }
            transparency_pass::WaterSurfaceKind::Ice => {
                let prepared = with_surface_env(frame, world, |env| prepare_ice_surface(env, &inputs))?;
                with_draw_context(frame, world, |ctx| raster_ice_surface(ctx, &prepared, &inputs))?;
                with_surface_env(frame, world, |env| finish_ice_surface(env, prepared, &inputs))?;
            }
        }
    }
    SceneRasterHost::new(frame, world, lists)?.pre_taa_transparency_pass(
        &attachment(color), &attachment(depth), &attachment(half), &attachment(lighting.history.previous),
    )
}

fn with_surface_env<T>(
    frame: &mut CoreGraphFrame<'_>, world: &World,
    build: impl FnOnce(&mut SurfaceEnv<'_>) -> rhapsody::Result<T>,
) -> rhapsody::Result<T> {
    with_env(frame, world, |pass_env| {
        let shared = RefCell::new(pass_env);
        let mut resources = SurfaceResources(&shared);
        let mut graph = SurfaceGraph(&shared);
        let mut compute = SurfacePasses(&shared);
        build(&mut SurfaceEnv::new(&mut resources, &mut graph, &mut compute))
    })
}

// These are sequential trait views of the same existing PassEnv.frame. RefCell
// checked borrows cover one operation only, never a callback or whole phase.
// They own no buffers, shader cache, session, allocator or replacement graph.
struct SurfaceResources<'a, 'b, 'c>(&'a RefCell<&'b mut PassEnv<'c>>);
struct SurfaceGraph<'a, 'b, 'c>(&'a RefCell<&'b mut PassEnv<'c>>);
struct SurfacePasses<'a, 'b, 'c>(&'a RefCell<&'b mut PassEnv<'c>>);
impl RgResources for SurfaceResources<'_, '_, '_> {
    fn create_raw_texture(&mut self, name: &str, desc: &TextureDesc) -> RgTexture {
        self.0.borrow_mut().frame.create_raw_texture(name, desc)
    }
    fn create_raw_buffer(&mut self, name: &str, desc: &BufferDesc) -> RgBuffer {
        self.0.borrow_mut().frame.create_raw_buffer(name, desc)
    }
    fn get_or_create_temporal_texture(&mut self, name: &str, desc: &TextureDesc) -> Result<TemporalTexture, RgError> {
        self.0.borrow_mut().frame.get_or_create_temporal_texture(name, desc)
    }
    fn get_or_create_temporal_texture_pair(&mut self, name: &str, desc: &TextureDesc) -> Result<TemporalTexturePair, RgError> {
        self.0.borrow_mut().frame.get_or_create_temporal_texture_pair(name, desc)
    }
    fn get_or_create_temporal_buffer(&mut self, name: &str, desc: &BufferDesc) -> Result<TemporalBuffer, RgError> {
        self.0.borrow_mut().frame.get_or_create_temporal_buffer(name, desc)
    }
}
impl RenderGraph for SurfaceGraph<'_, '_, '_> {
    fn add_pass(&mut self, name: &'static str, resource: u32, access: u8, render: RenderFn) {
        self.0.borrow_mut().frame.add_pass(name, resource, access, render)
    }
    fn allocate_dynamic(&mut self, size: usize) -> rhapsody::Result<StagingAlloc> {
        self.0.borrow_mut().frame.allocate_dynamic(size)
    }
    fn copy_from_staging(&mut self, name: &'static str, copies: Vec<StagingCopy>) -> rhapsody::Result<()> {
        self.0.borrow_mut().frame.copy_from_staging(name, copies)
    }
    fn upload_complete_bytes(&mut self, buffer: u32, bytes: &[u8], source: u64, version: u64) -> rhapsody::Result<bool> {
        self.0.borrow_mut().frame.upload_complete_bytes(buffer, bytes, source, version)
    }
}
impl SurfaceCompute for SurfacePasses<'_, '_, '_> {
    fn new_pass(&mut self, name: &'static str, uri: &'static str) -> ComputePass {
        self.0.borrow_mut().pass(name, uri)
    }
    fn record(&mut self, pass: RecordedComputePass) {
        self.0.borrow_mut().record(pass)
    }
}
