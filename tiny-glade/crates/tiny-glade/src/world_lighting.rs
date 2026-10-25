//! Original render_world W35..W43 on the current graph/session architecture.
//!
//! Input geometry is produced by W1..W34. This continuous lighting segment
//! never creates a substitute scene, initializes histories, submits work, or
//! publishes gameplay readiness. Its typed output feeds W44..W54 unchanged.

use bevy_ecs::world::World;
use country_core::{
    render::pass_builder::{
        pass_resource_access::RgTexture,
        rg_resources::{RgResources, TemporalTexture, TemporalTexturePair},
    },
    rhapsody_contract::{Handle as DrawHandle, ManagedHandle},
    rhapsody_types::TextureDesc,
    systems::render_loop::{
        compute_passes::{
            apply_screen_shadows::ScreenShadowInputs, bin_lights::LightBinning, half_res::GBuffer,
            ray_trace_gi::RestirTextures,
        },
        draw_passes::{
            deferred_raster::{DeferredInputs, DeferredRasterResources, ENTRIES},
            draw_lists::DrawLists,
        },
        render_world::{PREV_DEPTH_USAGE, formats, names},
    },
};

use crate::{
    effects_host::attachment,
    render_bridge::CoreGraphFrame,
    scene_compute::{HalfDepthNormals, SceneComputeHost},
    scene_raster::SceneRasterHost,
};

/// The original caller selects TAA at world size or TGSR at display size.
/// Display dimensions are a required real owner value, never a fallback size.
#[derive(Clone, Copy, Debug)]
pub enum ColorHistoryMode {
    Taa,
    Tgsr { display_dims: [u32; 2] },
}

/// Actual attachments from the completed geometry stage. The previous depth
/// allocation precedes W1 in the original function; use begin_depth_history at
/// that point rather than allocating a second history inside lighting.
#[derive(Clone, Copy, Debug)]
pub struct WorldGeometryOutputs {
    pub gbuffer: GBuffer,
    pub velocity: RgTexture,
    pub color: RgTexture,
    pub previous_depth: TemporalTexture,
}

/// Full values consumed by transparency, volumetrics, DoF and temporal AA.
/// Retaining all three ReSTIR GI textures and both light buffers is necessary;
/// these values cannot be represented by a single opaque texture identifier.
#[derive(Clone, Copy, Debug)]
pub struct WorldLightingOutputs {
    pub geometry: WorldGeometryOutputs,
    pub half: HalfDepthNormals,
    pub reprojection: RgTexture,
    pub ssgi: RgTexture,
    pub history: TemporalTexturePair,
    pub ray_traced_gi: RestirTextures,
    pub gi_radiance: RgTexture,
    pub lights: LightBinning,
    pub direct_lights: RgTexture,
    pub shadows: RgTexture,
}

fn graph_error(error: anyhow::Error) -> rhapsody::Error {
    match error.downcast::<rhapsody::Error>() {
        Ok(error) => error,
        Err(error) => rhapsody::Error::InvalidUsage {
            what: "original render_world temporal resource".into(),
            source: Some(error.into()),
        },
    }
}

/// Original 1404ec1f6..260, before the six W1 uploads. The same current temporal
/// owner tracks allocation, export, cancellation and first-frame validity.
pub fn begin_depth_history(
    frame: &mut CoreGraphFrame<'_>,
    depth: RgTexture,
) -> rhapsody::Result<TemporalTexture> {
    // Current core describes usage with four booleans. Native .usage(0x0c)
    // overwrites all usage bits, so clear both attachment bits explicitly.
    let desc = TextureDesc {
        format: formats::PREV_DEPTH,
        sampled: PREV_DEPTH_USAGE & 4 != 0,
        storage: PREV_DEPTH_USAGE & 8 != 0,
        color_attachment: false,
        depth_stencil_attachment: false,
        ..depth.desc
    };
    frame.with_compute(|compute| {
        compute
            .get_or_create_temporal_texture(names::PREV_DEPTH, &desc)
            .map_err(graph_error)
    })
}

/// Original 1404ec650..a67. The caller must already be inside the single
/// with_main_prelude/FrameSetup scope; this function does not nest that scope.
/// Missing real startup resources fail before lighting passes are declared.
pub fn build_world_lighting(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    lists: &DrawLists,
    geometry: WorldGeometryOutputs,
    history_mode: ColorHistoryMode,
) -> rhapsody::Result<WorldLightingOutputs> {
    let shader_library = frame.shader_library();
    let resources = world.get_resource::<DeferredRasterResources>().ok_or_else(|| {
        rhapsody::Error::invalid_usage(
            "original deferred_raster 0x1409d2113: missing DeferredRasterResources startup owner",
        )
    })?;
    // The native entries use materials 1,3,0,0,2. Slot 4 (Forward) is unused.
    // These are actual library-owned handles, not lookup-by-path substitutions.
    let shaders = resources.shaders;
    for (material, _) in ENTRIES {
        let handle = shaders[material as usize].ok_or_else(|| {
            rhapsody::Error::invalid_usage(format!(
                "original startup_deferred_raster 0x140a97a30: missing material {material}",
            ))
        })?;
        if shader_library.get_raster(handle).is_none() {
            return Err(rhapsody::Error::invalid_usage(
                "deferred shader handle does not belong to this App shader library",
            ));
        }
    }
    let history_dims = match history_mode {
        ColorHistoryMode::Taa => geometry.color.desc.extent_2d(),
        ColorHistoryMode::Tgsr { display_dims } => display_dims,
    };
    if history_dims.contains(&0) {
        return Err(rhapsody::Error::invalid_usage(
            "original world history has zero extent",
        ));
    }

    // W35..37 are declared before the color history pair, matching the caller.
    let (half, reprojection, ssgi) = {
        let mut compute = SceneComputeHost::new(frame, world);
        let half = compute.extract_half_res_depth_normals(&geometry.gbuffer)?;
        let reprojection = compute.calculate_reprojection_map(
            &geometry.gbuffer.depth,
            &geometry.velocity,
            &geometry.previous_depth,
        )?;
        let ssgi = compute.calculate_ssgi(&half, &geometry.gbuffer, &reprojection)?;
        (half, reprojection, ssgi)
    };
    let history_name = match history_mode {
        ColorHistoryMode::Taa => names::TAA_HISTORY,
        ColorHistoryMode::Tgsr { .. } => names::TGSR_HISTORY,
    };
    let history_desc = TextureDesc::new_2d(history_dims, formats::COLOR_HDR)
        .sampled()
        .storage();
    let history = frame.with_compute(|compute| {
        compute
            .get_or_create_temporal_texture_pair(history_name, &history_desc)
            .map_err(graph_error)
    })?;

    // W38..42. The history flag is valid-history, including zero on allocation;
    // do not invert it or manufacture initialized previous-frame contents.
    let (ray_traced_gi, gi_radiance, lights, direct_lights, shadows) = {
        let mut compute = SceneComputeHost::new(frame, world);
        let ray_traced_gi = compute.ray_trace_gi(
            &geometry.gbuffer,
            &half,
            &history.previous,
            history.fresh as u8,
            &reprojection,
            &ssgi,
        )?;
        let gi_radiance =
            compute.eval_gi_screen_radiance(&half, &ssgi, &ray_traced_gi, &reprojection)?;
        let lights = compute.bin_lights()?;
        let direct_lights = compute.restir_sample_direct_lights(&half, &reprojection, &lights)?;
        let shadows = compute.apply_screen_shadows(&ScreenShadowInputs {
            a: &geometry.gbuffer.gbuffer,
            b: &geometry.gbuffer.depth,
            c: &reprojection,
        })?;
        (ray_traced_gi, gi_radiance, lights, direct_lights, shadows)
    };
    let out = WorldLightingOutputs {
        geometry,
        half,
        reprojection,
        ssgi,
        history,
        ray_traced_gi,
        gi_radiance,
        lights,
        direct_lights,
        shadows,
    };
    // W43 caller stack at 1404eca08..67. The old core ABI adapter's names
    // `depth` and `gbuffer` are misleading: they carry scene color and RT GI.
    // Keep that compatibility detail only here; public stage outputs are typed.
    let inputs = DeferredInputs {
        shaders: shaders.map(|handle| {
            handle.map(|handle| {
                DrawHandle(ManagedHandle(handle.id().raw()), core::marker::PhantomData)
            })
        }),
        lighting: [
            attachment(out.geometry.gbuffer.gbuffer),
            attachment(out.geometry.gbuffer.depth),
        ],
        depth: attachment(out.geometry.color),
        arg5: attachment(out.history.previous),
        arg6: out.history.fresh as u8,
        arg7: attachment(out.ssgi),
        arg8: attachment(out.half.depth),
        gbuffer: [
            attachment(out.ray_traced_gi.radiance),
            attachment(out.ray_traced_gi.ray_dir),
            attachment(out.ray_traced_gi.reservoir),
        ],
        arg10: attachment(out.gi_radiance),
        arg11: attachment(out.shadows),
        arg12: attachment(out.direct_lights),
        rt_buffers: [out.lights.grid.resource, out.lights.indices.resource],
        _marker: core::marker::PhantomData,
    };
    SceneRasterHost::new(frame, world, lists)?.deferred_raster(&inputs)?;
    Ok(out)
}

/// Install the recovered eighth member of the original gameplay Startup tuple.
/// B 14094117f -> add_systems<Startup>@140b67b50 -> tuple@14099b8b0
/// -> config@1408c50f0 -> vtable142aa2240 -> startup_deferred_raster.
/// The current host supplies this App's already installed shader backend.
pub fn install_deferred_startup(
    app: &mut bevy_app::App,
    backend: std::sync::Arc<crate::render_host::ShaderHost>,
) {
    app.init_resource::<DeferredRasterResources>();
    app.add_systems(
        bevy_app::Startup,
        move |library: bevy_ecs::prelude::Res<crate::run_game::ApplicationShaderLibrary>,
              mut resources: bevy_ecs::prelude::ResMut<DeferredRasterResources>| {
            country_core::startup::startup_deferred_raster::startup_deferred_raster(
                &library.0,
                backend.as_ref(),
                &mut resources,
            );
        },
    );
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.resource::<DeferredRasterResources>();
}
