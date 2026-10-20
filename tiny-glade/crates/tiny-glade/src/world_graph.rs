//! B render_world@1404ec110, continuous W1..W54 on the enclosing main graph.
//! The caller supplies its actual depth/color targets and process debug flags.
//! This is the single typed consumer of the recovered stages, not a renderer,
//! frame submission or a gameplay readiness shortcut.
use bevy_ecs::world::World;
use country_core::{
    game_config::GameConfig,
    render::{render_config::{FrameRenderConfig, UpscalingMode},
        pass_builder::{pass_resource_access::RgTexture, rg_resources::RgResources}},
    resources::session::ActiveSession,
    rhapsody_types::TextureDesc,
    systems::render_loop::{
        compute_passes::{half_res::GBuffer, gpu_generate_draw_lists::NaniHzb},
        draw_passes::{draw_lists::DrawLists, main_pass::MainPassTargets},
        render_world::{formats, names},
    },
};
use crate::{
    effects_host::attachment,
    render_bridge::CoreGraphFrame,
    scene_compute::{self, SceneComputeHost}, scene_raster::SceneRasterHost,
    world_geometry as geometry, world_terrain as terrain, world_vegetation as vegetation,
    world_lighting::{self, ColorHistoryMode, WorldGeometryOutputs},
    world_postprocess::{self, PostProcessControls, WorldPostOutputs},
    world_surface::{self, SurfaceControls}, world_walls,
};

#[derive(Clone, Copy, Debug)]
pub struct WorldGraphControls {
    /// Original process debug byte143634da0, shared by W27/W28/W29.
    pub freeze_occlusion_culling: bool,
    pub postprocess: PostProcessControls,
}

fn missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!("original render_world: missing {name}"))
}

/// Native compile_main before-render_world selection. These two bools have
/// independent meanings: menu painting can reuse its captured target while
/// the 54-pass world graph remains idle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MainWorldDecision {
    pub render_world: bool,
    pub painterly_enabled: bool,
}

pub fn select_main_world(world: &World) -> rhapsody::Result<MainWorldDecision> {
    use country_core::{app_loading_state::AppLoadingState, resources::gui::EscMenuState,
        systems::render_loop::painterly::PainterlyState,
        render::frame_constants::FrameConstants, meta_commander::MetaCommander};
    let menu = world.get_resource::<EscMenuState>().ok_or_else(|| missing("EscMenuState"))?;
    let loading = world.get_resource::<bevy_state::state::State<AppLoadingState>>()
        .ok_or_else(|| missing("AppLoadingState"))?;
    let profiling = || world.get_resource::<MetaCommander>()
        .map(MetaCommander::is_profiling).ok_or_else(|| missing("MetaCommander"));
    if *loading.get() != AppLoadingState::Game || !menu.should_show_world() {
        return Ok(MainWorldDecision { render_world: profiling()?, painterly_enabled: false });
    }
    let painterly_enabled = menu.is_open();
    let render_world = if menu.is_closed() {
        true
    } else if painterly_enabled && !world.get_resource::<PainterlyState>()
        .ok_or_else(|| missing("PainterlyState"))?.has_target() {
        log::trace!(target: "tiny_glade::render_loop", "Forcing world rendering because painterly needs a target");
        true
    } else if world.get_resource::<FrameConstants>()
        .ok_or_else(|| missing("FrameConstants"))?.should_reset_temporal() {
        log::trace!(target: "tiny_glade::render_loop", "Forcing world rendering because temporal needs reset");
        true
    } else if menu.can_use_static_world_capture() {
        profiling()?
    } else {
        log::trace!(target: "tiny_glade::render_loop", "Forcing world rendering because !can_use_static_world_capture()");
        true
    };
    Ok(MainWorldDecision { render_world, painterly_enabled })
}

/// Read-only observation of the last declared world frame: whether the TAA/TGSR color history pair
/// held valid previous-frame contents (`TemporalTexturePair::history_valid`). Written by `build_world`
/// after the lighting stage; multi-frame validation reads it, no producer consumes it.
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WorldHistoryObservation {
    pub color_history_valid: bool,
}

/// Must run inside the original main prelude and its one FrameSetup scope.
/// Each stage borrows the same World and graph; any missing owner aborts before
/// successful compilation and before warmup can publish the SubApp.
pub fn build_world(
    frame: &mut CoreGraphFrame<'_>, world: &mut World, lists: &DrawLists,
    world_dims: [u32; 2], depth: RgTexture, color: RgTexture, controls: WorldGraphControls,
) -> rhapsody::Result<WorldPostOutputs> {
    if world_dims.contains(&0) {
        return Err(rhapsody::Error::invalid_usage("original world dimensions must be nonzero"));
    }
    // The original ActiveSession read precedes previous-depth creation and W1.
    let glade = world.get_resource::<ActiveSession>().ok_or_else(|| missing("ActiveSession"))?.meta.glade;
    let previous_depth = world_lighting::begin_depth_history(frame, depth)?;
    scene_compute::upload_world_rasters(frame, world)?;
    scene_compute::water_rasterize_from_cpu(frame, world)?;
    terrain::rasterize_terrain_stroke(frame, world)?;
    terrain::displace_rocky_terrain(frame, world)?;
    terrain::read_terrain_data(frame, world)?;
    terrain::mouse_read_terrain_stroke_index(frame, world)?;
    terrain::rasterize_foliage_exclusion_mask(frame, world)?;
    terrain::highlight_active_terrain_stroke(frame, world)?;
    terrain::object_grass_push_pass(frame, world)?;
    terrain::compute_path_masks(frame, world)?;
    terrain::rasterize_buildings(frame, world)?;
    scene_compute::garden_rasterize_from_cpu(frame, world)?;
    vegetation::water_spawn_vegetation(frame, world)?;
    vegetation::spawn_vegetation_around_buildings(frame, world)?;
    vegetation::read_bush_positions_for_cpu(frame, world)?;
    vegetation::garden_spawn_vegetation(frame, world)?;
    vegetation::path_pebbles_compute(frame, world)?;
    geometry::leaf_particles_compute(frame, world)?;
    geometry::roof_tiles_gravity_compute(frame, world)?;
    geometry::stairs(frame, world)?;
    world_walls::generate_wall_draw_lists(frame, world)?;
    scene_compute::generate_nani_draw_lists(frame, world, None)?;
    SceneRasterHost::new(frame, world, lists)?.shadow_pass()?;
    geometry::calc_terrain_lighting_texture(frame, world)?;

    let (gbuffer, velocity) = frame.with_compute(|compute| Ok((
        compute.create_raw_texture(names::GBUFFER, &TextureDesc::new_2d(world_dims, formats::GBUFFER)),
        compute.create_raw_texture(names::VELOCITY, &TextureDesc::new_2d(world_dims, formats::VELOCITY)),
    )))?;
    let targets = MainPassTargets { gbuffer: attachment(gbuffer), depth: attachment(depth),
        velocity: attachment(velocity) };
    SceneRasterHost::new(frame, world, lists)?.main_pass(&targets, &attachment(color))?;
    let hzb = SceneComputeHost::new(frame, world).generate_hzb(&depth, names::HZB)?;
    if !controls.freeze_occlusion_culling {
        world_walls::generate_wall_draw_lists_pass2(frame, world, &hzb)?;
        scene_compute::generate_nani_draw_lists(frame, world,
            Some(NaniHzb { texture: hzb.texture, inner_dims: hzb.half_extent }))?;
    }
    geometry::grass_compute(frame, world, &hzb, controls.freeze_occlusion_culling)?;
    SceneRasterHost::new(frame, world, lists)?.main_pass_oc2(&targets, &attachment(color))?;
    geometry::run_visibility_queries(frame, world, &depth)?;
    geometry::calculate_dof_autofocus(frame, world, &depth)?;
    geometry::mouse_depth_compute(frame, world, &depth)?;
    SceneRasterHost::new(frame, world, lists)?.late_forward_and_deferred_pass(&targets)?;

    let video = &world.get_resource::<GameConfig>().ok_or_else(|| missing("GameConfig"))?.video;
    let history_mode = if video.upscaling_mode == UpscalingMode::Tgsr {
        ColorHistoryMode::Tgsr { display_dims: world.get_resource::<FrameRenderConfig>()
            .ok_or_else(|| missing("FrameRenderConfig"))?.extent_8 }
    } else { ColorHistoryMode::Taa };
    let lighting = world_lighting::build_world_lighting(frame, world, lists, WorldGeometryOutputs {
        gbuffer: GBuffer { gbuffer, depth }, velocity, color, previous_depth,
    }, history_mode)?;
    world.insert_resource(WorldHistoryObservation { color_history_valid: lighting.history.fresh });
    let skip_refraction = world.resource::<GameConfig>().dev.demo;
    world_surface::build_world_surface(frame, world, lists, &lighting,
        SurfaceControls { glade, skip_refraction })?;
    world_postprocess::build_world_postprocess(frame, world, lists, &lighting, controls.postprocess)
}

#[cfg(test)]
mod main_selection_tests {
    use super::*;
    use country_core::{app_loading_state::AppLoadingState, resources::gui::EscMenuState,
        systems::render_loop::painterly::{PainterlyState, PainterlyPhase},
        render::frame_constants::FrameConstants, meta_commander::MetaCommander};
    fn game(menu: EscMenuState) -> World {
        let mut world = World::new();
        world.insert_resource(bevy_state::state::State::new(AppLoadingState::Game));
        world.insert_resource(menu);
        world
    }
    #[test]
    fn closed_world_does_not_require_idle_only_producers() {
        let world = game(EscMenuState::Closed);
        assert_eq!(select_main_world(&world).unwrap(), MainWorldDecision { render_world: true, painterly_enabled: false });
    }
    #[test]
    fn static_menu_reuses_ready_painting_until_temporal_reset() {
        let mut world = game(EscMenuState::Main);
        world.insert_resource(PainterlyState { phase: PainterlyPhase::Ready, ..Default::default() });
        world.insert_resource(MetaCommander::default());
        world.insert_resource(FrameConstants { frame_counter: 9, ..Default::default() });
        assert_eq!(select_main_world(&world).unwrap(), MainWorldDecision { render_world: false, painterly_enabled: true });
        world.resource_mut::<FrameConstants>().frame_counter = 0;
        assert!(select_main_world(&world).unwrap().render_world);
    }
    #[test]
    fn pending_target_forces_world_before_frame_constant_lookup() {
        let mut world = game(EscMenuState::Main);
        world.insert_resource(PainterlyState::default());
        assert_eq!(select_main_world(&world).unwrap(), MainWorldDecision { render_world: true, painterly_enabled: true });
    }
    #[test]
    fn hidden_world_does_not_lookup_painterly_or_frame_constants() {
        let mut world = game(EscMenuState::Load(Default::default()));
        world.insert_resource(MetaCommander::default());
        assert_eq!(select_main_world(&world).unwrap(), MainWorldDecision { render_world: false, painterly_enabled: false });
    }
}
