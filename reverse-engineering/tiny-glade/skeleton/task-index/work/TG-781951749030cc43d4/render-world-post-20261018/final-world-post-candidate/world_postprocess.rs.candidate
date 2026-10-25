//! Original continuous render_world W47..W54 on the enclosing current frame.
//! W44..W46 must already have consumed the same lighting targets. This module
//! never manufactures a gameplay-ready gate or omits the final debug consumer.
use crate::{
    effects_host::attachment, render_bridge::CoreGraphFrame, scene_compute::SceneComputeHost,
    scene_raster::SceneRasterHost, world_lighting::WorldLightingOutputs,
};
use bevy_ecs::world::World;
use country_core::{
    game_config::GameConfig,
    render::{
        pass_builder::{pass_resource_access::RgTexture, rg_resources::RgResources},
        render_config::{FrameRenderConfig, UpscalingMode},
    },
    resources::image_formation::ImageFormationConfigs,
    rhapsody_types::TextureDesc,
    systems::{
        mode_manager::AppMode,
        render_loop::{
            compute_passes::{
                dof::DofConfig,
                taa::{HistoryTexture, TaaInputs, TgsrInputs},
            },
            draw_passes::draw_lists::DrawLists,
            render_world::{formats, names},
        },
    },
};

/// Snapshots of two original process-level debug flags. No Default is provided:
/// the actual host must supply their state; AppMode is independently read below.
#[derive(Clone, Copy, Debug)]
pub struct PostProcessControls {
    /// Original byte at14362c080, read inside calc_dof@140a5a5b4.
    pub multires_dof: bool,
    /// Original history byte143634d50, inverted by TAA/TGSR before valid gating.
    pub disable_temporal_history: bool,
}

/// Native render_world returns depth/CoC/TAA, each a real texture. The internal
/// second DoF tile texture and optional tile buffers remain fully consumed by
/// the original calc_dof chain before this boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldPostOutputs {
    pub depth: RgTexture,
    pub coc: RgTexture,
    pub taa: RgTexture,
}

fn missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!("original world W47..53: missing {name} producer"))
}

/// Caller owns the single main FrameSetup and the original preceding geometry,
/// lighting and surface/transparency draws. This function declares actual core
/// consumers, never submits, initializes histories or substitutes scene owners.
pub fn build_world_postprocess(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    lists: &DrawLists,
    lighting: &WorldLightingOutputs,
    controls: PostProcessControls,
) -> rhapsody::Result<WorldPostOutputs> {
    let video = world
        .get_resource::<GameConfig>()
        .ok_or_else(|| missing("GameConfig"))?
        .video
        .clone();
    let mode = world
        .get_resource::<AppMode>()
        .ok_or_else(|| missing("AppMode"))?
        .native_tag();
    let photomode = mode == 0x13;
    let config = DofConfig {
        bokeh_anisotropy: world
            .get_resource::<ImageFormationConfigs>()
            .ok_or_else(|| missing("ImageFormationConfigs"))?
            .current()
            .params_30[0],
    };
    let render_config = *world
        .get_resource::<FrameRenderConfig>()
        .ok_or_else(|| missing("FrameRenderConfig"))?;
    if !world.contains_resource::<country_core::systems::camera_rig::main_camera::MainCamera>() {
        return Err(missing("MainCamera"));
    }
    let color = lighting.geometry.color;
    let depth = lighting.geometry.gbuffer.depth;
    // Historical field name: current host constructs fresh = pair.history_valid()
    // (render_host.rs get_or_create_temporal_texture_pair), not allocation freshness.
    let history = HistoryTexture {
        tex: lighting.history.previous,
        valid: lighting.history.fresh,
    };
    let use_tgsr = video.upscaling_mode == UpscalingMode::Tgsr;
    let expected_history_dims = if use_tgsr {
        render_config.extent_8
    } else {
        color.desc.extent_2d()
    };
    if expected_history_dims.contains(&0)
        || lighting.history.current.desc.extent_2d() != expected_history_dims
    {
        return Err(rhapsody::Error::invalid_usage(
            "world post history differs from the original TAA/TGSR dimensions",
        ));
    }
    let mut compute = SceneComputeHost::new(frame, world);
    compute.compute_shadow_mask_mean()?;
    compute.apply_volumetrics(&color, &depth)?;
    let prepared = compute.dof_prepare(&depth)?;
    let dof_enabled = if photomode {
        video.photomode_dof_enable
    } else {
        video.dof_enable
    };
    let taa_input = if dof_enabled {
        compute.generate_mips(&color, names::DOF_MIPS)?;
        compute
            .calc_dof(
                &color,
                &depth,
                &prepared,
                &config,
                controls.multires_dof,
                photomode,
            )?
            .color
    } else {
        color
    };
    let extent = if use_tgsr {
        render_config.extent_8
    } else {
        taa_input.desc.extent_2d()
    };
    let output = frame.with_compute(|compute| {
        Ok(compute.create_raw_texture(
            names::TAA_OUTPUT,
            &TextureDesc::new_2d(extent, formats::COLOR_HDR),
        ))
    })?;
    let min_subgroup_size = frame
        .hosts()
        .buffers
        .graphics_device()
        .min_subgroup_size_option();
    let mut compute = SceneComputeHost::new(frame, world);
    let taa = if use_tgsr {
        compute.calculate_tgsr(&TgsrInputs {
            color: &taa_input,
            depth: &depth,
            output: &output,
            reprojection: &lighting.reprojection,
            history_output: &lighting.history.current,
            history: &history,
            sharpen_amount: video.sharpen,
            bypass_clamp: u8::from(render_config.flag_20),
            history_global: controls.disable_temporal_history,
            min_subgroup_size,
        })?
    } else {
        compute.calculate_taa(&TaaInputs {
            color: &taa_input,
            output: &output,
            reprojection: &lighting.reprojection,
            history_output: &lighting.history.current,
            history: &history,
            sharpen_amount: video.sharpen,
            bypass_clamp: u8::from(render_config.flag_20),
            history_global: controls.disable_temporal_history,
        })?
    };
    SceneRasterHost::new(frame, world, lists)?.post_taa_transparency_pass(&attachment(taa))?;
    let taa = render_shader_refactoring(frame, world, taa)?;
    Ok(WorldPostOutputs {
        depth,
        coc: prepared.coc,
        taa,
    })
}

/// W54 original mandatory resource and whole-frame replacement, over the
/// existing enclosing frame and canonical ECS owner. No new state or image is
/// installed. State mutation occurs at graph-declaration completion as native,
/// before compile/submit; later failures do not invent a rollback policy.
pub fn render_shader_refactoring(
    frame: &mut CoreGraphFrame<'_>, world: &mut World, taa: RgTexture,
) -> rhapsody::Result<RgTexture> {
    use bevy_ecs::change_detection::DetectChanges;
    use country_core::{
        resources::shader_refactoring_state::ShaderRefactoringState,
        systems::render_loop::compute_passes::shader_refactoring::{
            self, FrameHandle, StateMut,
        },
    };
    let state = world.get_resource::<ShaderRefactoringState>()
        .ok_or_else(|| missing("ShaderRefactoringState"))?;
    // Native disc0 returns after this mandatory lookup, without allocation,
    // changed-tick mutation, shaders or any extra pass-resource lookup.
    if state.disc == 0 { return Ok(taa); }
    let tick = world.read_change_tick().get();
    world.resource_scope(|world, mut canonical: bevy_ecs::change_detection::Mut<ShaderRefactoringState>| {
        let active = matches!(canonical.disc, 1..=4);
        let mut state = StateMut { state: *canonical, changed_tick: canonical.last_changed().get() };
        crate::scene_compute::with_env(frame, world, |env| {
            let run = shader_refactoring::try_render_shader_refactoring(
                env, &mut state, tick, &FrameHandle { texture: taa, format: taa.desc.format })?;
            for pass in run.passes { env.record(pass); }
            if active { *canonical = state.state; }
            Ok(run.replacement_frame.unwrap_or(taa))
        })
    })
}
