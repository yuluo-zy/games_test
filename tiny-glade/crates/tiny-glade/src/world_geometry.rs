//! Actual geometry-stage consumers on the enclosing original world graph.
//! Inputs are borrowed from the same App; no original-layout resource views,
//! substitute camera, readback values or additional GPU owners are installed.

use crate::{render_bridge::CoreGraphFrame, scene_compute::with_env};
use bevy_ecs::world::World;
use country_core::{
    game_config::GameConfig,
    render::pass_builder::pass_resource_access::RgTexture,
    resources::{
        compute_mouse_depth_value::MouseDepthValueSsbo,
        mouse_terrain_raycast::CursorPositionSS,
        window::WindowSize,
    },
    systems::{
        camera_rig::main_camera::MainCamera,
        render_loop::{
            compute_passes::{mouse_depth_value, terrain_lighting_texture, visibility_queries},
            resources::critters::{FireflySsbo, FireflyVisibilitySsbo},
        },
    },
};

fn missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!("original world geometry: missing {name} producer"))
}

/// W18 @1408f4d00. The original setting is GladeSettings, and the emitter
/// resource is read only after the nonzero-particle branch declares its pass.
pub fn leaf_particles_compute(
    frame: &mut CoreGraphFrame<'_>, world: &World,
) -> rhapsody::Result<()> {
    use country_core::{resources::glade_settings::GladeSettings,
        systems::render_loop::compute_passes::leaf_particles::{self, LoadedParticleEmitterCount}};
    let count = world.get_resource::<GladeSettings>().ok_or_else(|| missing("GladeSettings"))?
        .leaf_particle_count;
    with_env(frame, world, |env| leaf_particles::leaf_particles_compute(env, count, || {
        world.get_resource::<LoadedParticleEmitterCount>().map(|value| value.0)
            .ok_or_else(|| missing("LoadedParticleEmitterCount"))
    }))
}

/// W20 @140a68a50. Both original passes share the canonical startup buffers
/// and the draw-command prefix initialized by the enclosing main prelude.
pub fn stairs(frame: &mut CoreGraphFrame<'_>, world: &World) -> rhapsody::Result<()> {
    with_env(frame, world, |env| {
        country_core::systems::render_loop::compute_passes::stairs::stairs(env);
        Ok(())
    })
}

/// W29 @140a60280. The shader brush selector is the actual AppMode tag;
/// a GladeTheme has a separate identity and cannot supply this value.
pub fn grass_compute(
    frame: &mut CoreGraphFrame<'_>, world: &World,
    hzb: &country_core::systems::render_loop::compute_passes::generate_hzb::Hzb,
    freeze_occlusion_culling: bool,
) -> rhapsody::Result<()> {
    let mode = world.get_resource::<country_core::systems::mode_manager::AppMode>()
        .ok_or_else(|| missing("AppMode"))?.native_tag();
    with_env(frame, world, |env| {
        country_core::systems::render_loop::compute_passes::grass::grass_compute_with_app_mode(
            env, hzb, freeze_occlusion_culling, mode);
        Ok(())
    })
}

/// W32 @140a17880, actual six-entry camera offset tables142ac8948/978.
/// Both focus and lens are borrowed projections of the same MainCamera.
pub fn calculate_dof_autofocus(
    frame: &mut CoreGraphFrame<'_>, world: &World, depth: &RgTexture,
) -> rhapsody::Result<()> {
    use country_core::{startup::init_photomode_state::PhotomodeState,
        systems::{mode_manager::AppMode, render_loop::compute_passes::dof_autofocus::{self as dof,
            AutofocusInputs, FocusParams, CameraLens, DofAutofocusSsbo, DofFocusReadbackSsbo}}};
    let view_kind = world.get_resource::<AppMode>().ok_or_else(|| missing("AppMode"))?.native_tag();
    let window = world.get_resource::<WindowSize>().ok_or_else(|| missing("WindowSize"))?;
    let camera = world.get_resource::<MainCamera>().ok_or_else(|| missing("MainCamera"))?;
    let mode = usize::from(camera.mode_byte());
    let focus_offset = *[0x170, 0x228, 0x2a8, 0x348, 0x400, 0x488].get(mode)
        .ok_or_else(|| rhapsody::Error::invalid_usage("MainCamera has invalid native mode"))?;
    let lens_offset = [0x8c, 0x144, 0x1c4, 0x264, 0x31c, 0x3a4][mode];
    let focus = FocusParams {
        mode: u32::from_le_bytes(camera.bytes()[focus_offset..focus_offset + 4].try_into().unwrap()),
        f04: camera.read_f32(focus_offset + 4), f08: camera.read_f32(focus_offset + 8),
        f10: camera.read_f32(focus_offset + 0x10), f14: camera.read_f32(focus_offset + 0x14),
        box_height: camera.read_f32(focus_offset + 0x18),
    };
    let lock_request = if focus.mode < 2 && view_kind == 0x13 {
        Some(world.get_resource::<PhotomodeState>().ok_or_else(|| missing("PhotomodeState"))?.index)
    } else { None };
    let mouse_pos = if focus.mode == 2 {
        Some(world.get_resource::<CursorPositionSS>().ok_or_else(|| missing("CursorPositionSS"))?.0.to_array())
    } else { None };
    let inputs = AutofocusInputs { view_kind,
        window: dof::WindowSize { width: window.width, height: window.height }, focus,
        lens: CameraLens { use_tilt_shift: camera.bytes()[lens_offset + 0x128] != 0,
            aperture_size: camera.read_f32(lens_offset + 0x118) }, lock_request, mouse_pos };
    with_env(frame, world, |env| {
        let pass = dof::calculate_dof_autofocus::<DofAutofocusSsbo, DofFocusReadbackSsbo>(env, &inputs, depth);
        env.record(pass);
        Ok(())
    })
}

/// W19 @140a29b10/1793B, iterator14130d8e0/1492B and dispatch1415f9210/317B.
/// Query the actual four roof components and the canonical sparse allocation;
/// a roof without an allocation contributes no dispatch, as in the original.
pub fn roof_tiles_gravity_compute(
    frame: &mut CoreGraphFrame<'_>, world: &mut World,
) -> rhapsody::Result<()> {
    use country_core::{
        asset_libraries::ssbo_library::AssetSsboLibrary,
        resources::{
            game_clock::GameClock, roof_tile_ssbo::RoofTileSsbo,
            roofs::{roof_animation::RoofAnimation, roof_shape::Roof,
                roof_tile_sim::RoofTileSimulation, roof_tiles::RoofTiles},
        },
        systems::render_loop::compute_passes::roof_tiles_gravity::{self, RoofTileRow},
    };
    // Original mandatory storage lookup precedes creation of the query cache.
    world.get_resource::<AssetSsboLibrary>().ok_or_else(|| missing("AssetSsboLibrary"))?
        .get::<RoofTileSsbo>().ok_or_else(|| missing("RoofTileSsbo"))?;
    let mut query = world.query::<(&RoofTiles, &RoofTileSimulation, &Roof, &RoofAnimation)>();
    let storage = world.resource::<AssetSsboLibrary>().get::<RoofTileSsbo>().unwrap();
    let rows = query.iter(world).map(|(tiles, simulation, roof, animation)| RoofTileRow {
        draws: tiles.draw_ranges(storage),
        wall_height_delta: f32::from_le_bytes(simulation.observed()[..4].try_into().unwrap()),
        roof_pivot_y: roof.floor_y(),
        animation_t: animation.values()[0],
    }).collect::<Vec<_>>();
    let delta = world.get_resource::<GameClock>().ok_or_else(|| missing("GameClock"))?.delta;
    with_env(frame, world, |env| {
        roof_tiles_gravity::roof_tiles_gravity_compute_with_delta(env, &rows, delta);
        Ok(())
    })
}

/// W24 @140a51440/2290B. The native resource identity is MainCamera;
/// `eye_position()` is used directly rather than casting its layout to the
/// historical partial CameraMatrices view. Core retains the actual preset8
/// compute output followed by the original SPD mip producer.
pub fn calc_terrain_lighting_texture(
    frame: &mut CoreGraphFrame<'_>, world: &World,
) -> rhapsody::Result<()> {
    let eye = world.get_resource::<MainCamera>().ok_or_else(|| missing("MainCamera"))?
        .eye_position();
    with_env(frame, world, |env| {
        terrain_lighting_texture::calc_terrain_lighting_texture_with_eye(env, eye);
        Ok(())
    })
}

/// W31 @140a2f2e0/904B. The actual canonical firefly buffers are among the
/// original72 startup producers; this declares the GPU query without claiming
/// that its mapped result is already available to a CPU consumer.
pub fn run_visibility_queries(
    frame: &mut CoreGraphFrame<'_>, world: &World, depth: &RgTexture,
) -> rhapsody::Result<()> {
    with_env(frame, world, |env| {
        let pass = visibility_queries::run_visibility_queries::<FireflySsbo, FireflyVisibilitySsbo>(env, depth);
        env.record(pass);
        Ok(())
    })
}

/// W33 @140a01620/1742B. GameConfig+110 is game+08, the canonical
/// camera_pivot_filter_size. CursorPositionSS supplies the original float2;
/// the distinct replay integer cursor must not be bitcast into those floats.
pub fn mouse_depth_compute(
    frame: &mut CoreGraphFrame<'_>, world: &World, depth: &RgTexture,
) -> rhapsody::Result<()> {
    let cursor = world.get_resource::<CursorPositionSS>()
        .ok_or_else(|| missing("CursorPositionSS"))?.0;
    let filter = world.get_resource::<GameConfig>().ok_or_else(|| missing("GameConfig"))?
        .game.camera_pivot_filter_size;
    let window = *world.get_resource::<WindowSize>().ok_or_else(|| missing("WindowSize"))?;
    with_env(frame, world, |env| {
        let pass = mouse_depth_value::mouse_depth_compute::<MouseDepthValueSsbo>(
            env, cursor, filter, (window.width, window.height), depth);
        env.record(pass);
        Ok(())
    })
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.component::<country_core::resources::roofs::roof_tiles::RoofTiles>();
    m.component::<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>();
    m.component::<country_core::resources::roofs::roof_shape::Roof>();
    m.component::<country_core::resources::roofs::roof_animation::RoofAnimation>();
    m.resource::<country_core::resources::glade_settings::GladeSettings>();
    m.resource::<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>();
    m.resource::<country_core::systems::mode_manager::AppMode>();
    m.resource::<country_core::systems::mode_manager::AppMode>();
    m.resource::<WindowSize>();
    m.resource::<MainCamera>();
    m.resource::<country_core::startup::init_photomode_state::PhotomodeState>();
    m.resource::<CursorPositionSS>();
    m.resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>();
    m.resource::<country_core::resources::game_clock::GameClock>();
    m.resource::<GameConfig>();
}
