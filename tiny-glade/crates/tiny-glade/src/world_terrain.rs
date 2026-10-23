//! Typed original W3/W6/W8 consumers on the enclosing real world graph.
//!
//! Borrows canonical TerrainEditingData::rasterization_parts from the same App.
//! It does not install resources, replace W4/W5/W7, or report a complete scene.
//! The core W3 body is directly refactored, not replaced by ActiveStrokes copies.

use crate::{render_bridge::CoreGraphFrame, scene_compute::with_env};
use bevy_ecs::{
    change_detection::{DetectChangesMut, Mut},
    event::Events,
    world::World,
};
use country_core::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast, gui::DebugGui,
        terrain_editing::TerrainEditingData,
    },
    systems::{
        mode_manager::AppMode,
        render_loop::compute_passes::{
            highlight_active_terrain_stroke::{self as highlight, AddTerrainHighlightCmd},
            mouse_read_terrain_stroke_index,
            rasterize_terrain_stroke::{self as terrain, RecalculateHeightMapCmd},
        },
    },
};

fn missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!("original W3..W11: missing {name} producer"))
}

/// W7 @1409cb7b0. Absence is the original optional resource branch, including
/// clearing a mask left by a removed stair mesh. A present resource always uses
/// its actual managed buffer and count; no default Some is substituted.
pub fn rasterize_foliage_exclusion_mask(
    frame: &mut CoreGraphFrame<'_>, world: &mut World,
) -> rhapsody::Result<()> {
    let stairs = world.get_resource::<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>().cloned();
    crate::scene_raster::with_draw_context(frame, world, |ctx| {
        country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask(ctx, stairs.as_ref());
    })
}

/// Last building elements written to the GPU by W11, in canonical order. Lets
/// the per-frame projection skip the (fence-waiting) upload while the wall rows
/// are unchanged.
#[derive(bevy_ecs::prelude::Resource, Default)]
struct GroundedUploadCache(Option<Vec<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>>);

/// W11 @140a32770, filter1412fc510 and publisher140a8b530. Build only an
/// ephemeral GPU projection from the actual published wall rows; elevation
/// (+280 in B) is independent of plaster removal. Freeform rows are excluded.
pub fn rasterize_buildings(
    frame: &mut CoreGraphFrame<'_>, world: &mut World,
) -> rhapsody::Result<usize> {
    use country_core::{
        asset_libraries::ssbo_library::AssetSsboLibrary,
        render::device::GraphicsDeviceResource,
        resources::walls::public_walls::PublicWalls,
        systems::render_loop::compute_passes::rasterize_buildings::{self as buildings,
            GroundedSource, GroundedBuildingsSsbo, MaskDesc},
    };
    let sources = world.get_resource::<PublicWalls>().ok_or_else(|| missing("PublicWalls"))?
        .all().filter_map(|(id, wall)| {
            let progress = wall.authority.elevation();
            let source = if let Some(circle) = wall.is_circle() {
                GroundedSource { kind: 0, w44: circle.center.x.to_bits(),
                    w40: circle.center.y.to_bits(), w3c: circle.radius.to_bits(),
                    w38: 0, v34: [0; 2], progress }
            } else if let Some(rect) = wall.is_rectangle() {
                GroundedSource { kind: 1, w44: rect.axis.x.to_bits(), w40: rect.axis.y.to_bits(),
                    w3c: rect.center.x.to_bits(), w38: rect.center.y.to_bits(),
                    v34: [rect.size.x.to_bits(), rect.size.y.to_bits()], progress }
            } else { return None; };
            Some((id, source))
        }).collect::<std::collections::HashMap<_, _>>();
    let count = sources.values().filter_map(GroundedSource::to_element).count();
    let has_buffer = world.get_resource::<AssetSsboLibrary>()
        .ok_or_else(|| missing("AssetSsboLibrary"))?.contains::<GroundedBuildingsSsbo>();
    if has_buffer {
        assert!(count < buildings::MAX_BUILDINGS, "{}", buildings::TOO_MANY_PANIC);
        let device = world.get_resource::<GraphicsDeviceResource>()
            .ok_or_else(|| missing("GraphicsDeviceResource"))?.clone();
        // Same collection and write rules as `upload_buildings`, but the GPU is
        // only written when the building rows changed. The library is still
        // borrowed mutably every frame, so its change flag behaves as before.
        world.get_resource_or_insert_with(GroundedUploadCache::default);
        world.resource_scope(|world, mut cache: bevy_ecs::prelude::Mut<GroundedUploadCache>| {
            buildings::upload_buildings_if_changed(
                &sources,
                &mut world.resource_mut::<AssetSsboLibrary>(),
                &*device,
                &mut cache.0,
            );
        });
    }
    with_env(frame, world, |env| {
        let desc = env.preset_desc(0x15);
        for pass in buildings::declare_passes(env, MaskDesc {
            extent: desc.extent_2d(), format: desc.format,
        }, count) { env.record(pass); }
        Ok(count)
    })
}

/// Native W3 @1408c7120. Changed is unconditional and precedes the event lookup
/// and early return (723a/723d/72bd). Events::is_empty observes both retained
/// buffers; this is not an EventReader that drains/advances across frames.
pub fn rasterize_terrain_stroke(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
) -> rhapsody::Result<usize> {
    let gpu = frame.raster_resources();
    if !world.contains_resource::<TerrainEditingData>() {
        return Err(missing("TerrainEditingData"));
    }
    world.resource_scope(|world, mut data: Mut<TerrainEditingData>| {
        data.set_changed();
        let requested = !world
            .get_resource::<Events<RecalculateHeightMapCmd>>()
            .ok_or_else(|| missing("Events<RecalculateHeightMapCmd>"))?
            .is_empty();
        let (strokes, rasterized) = data.rasterization_parts();
        if !requested && *rasterized {
            return Ok(0);
        }
        let mut constants = gpu.dynamic_constants().lock();
        let previous_rasterized = *rasterized;
        let result = with_env(frame, world, |env| {
            terrain::rasterize_terrain_stroke(env, &mut constants, strokes, rasterized, requested)
        });
        // Native builders fail before the final marker write. PassFrame's
        // current void record API defers admission errors to with_compute's
        // finish, so commit the same state only when that finish succeeds.
        // A failed frame must still be discarded by its enclosing caller.
        if result.is_err() {
            *rasterized = previous_rasterized;
        }
        result
    })
}

/// Native W6 @1409768b0. Uses the canonical hit position and mapped output
/// SSBO, including miss tag -> two zero push words. No parallel cursor view.
pub fn mouse_read_terrain_stroke_index(
    frame: &mut CoreGraphFrame<'_>,
    world: &World,
) -> rhapsody::Result<()> {
    let hit = world
        .get_resource::<CursorTerrainRaycast>()
        .ok_or_else(|| missing("CursorTerrainRaycast"))?;
    with_env(frame, world, |env| {
        let pass = mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index(env, hit);
        env.frame.record(pass);
        Ok(())
    })
}

/// Native W8 @140a58450. Native mode TypeId maps to AppMode and +f9 suppression
/// to DebugGui::disable_terrain_hover_highlight. The native code takes the last
/// element of the new retained buffer, or old when new is empty, only in tags5/10.
/// Events::get_cursor is the current safe adapter to that same retained latest
/// item; no assertion is made that the native code calls an EventCursor API.
pub fn highlight_active_terrain_stroke(
    frame: &mut CoreGraphFrame<'_>,
    world: &World,
) -> rhapsody::Result<usize> {
    let mode = world
        .get_resource::<AppMode>()
        .ok_or_else(|| missing("AppMode"))?
        .native_tag();
    let latest = if mode == 5 || mode == 10 {
        let events = world
            .get_resource::<Events<AddTerrainHighlightCmd>>()
            .ok_or_else(|| missing("Events<AddTerrainHighlightCmd>"))?;
        events.get_cursor().read(events).last().copied()
    } else {
        None
    };
    let suppress = world
        .get_resource::<DebugGui>()
        .ok_or_else(|| missing("DebugGui"))?
        .disable_terrain_hover_highlight;
    with_env(frame, world, |env| {
        // A single latest item has the same last() result as the original two
        // buffers. Do not label it the literal new Bevy event buffer.
        let passes = highlight::highlight_active_terrain_stroke(
            env,
            mode,
            &[],
            latest.as_slice(),
            u8::from(suppress),
        );
        let count = passes.len();
        for pass in passes {
            env.frame.record(pass);
        }
        Ok(count)
    })
}

/// Native W10 @1409014e0. The same Path/Cobblestone owners consumed by W1,
/// and the same Recalculate event read by W3, determine this thirteen-pass step.
/// All mandatory identities are checked even when its run gate will be false.
pub fn compute_path_masks(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
) -> rhapsody::Result<Option<u32>> {
    use country_core::resources::world_raster_defs::{
        Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, PathRaster,
    };
    macro_rules! require {
        ($r:ty) => {
            if !world.contains_resource::<$r>() {
                return Err(missing(stringify!($r)));
            }
        };
    }
    require!(PathRaster);
    require!(Cobblestone1Raster);
    require!(Cobblestone2Raster);
    require!(Cobblestone3Raster);
    let requested = !world
        .get_resource::<Events<RecalculateHeightMapCmd>>()
        .ok_or_else(|| missing("Events<RecalculateHeightMapCmd>"))?
        .is_empty();
    let tick = world.read_change_tick().get();
    world.resource_scope(|world, mut a: Mut<PathRaster>| {
        world.resource_scope(|world, mut b: Mut<Cobblestone1Raster>| {
            world.resource_scope(|world, mut c: Mut<Cobblestone2Raster>| {
                world.resource_scope(|world, mut d: Mut<Cobblestone3Raster>| {
                    with_env(frame, world, |env| Ok(
                        country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks(
                            env, &mut a, &mut b, &mut c, &mut d, requested, tick)))
                })
            })
        })
    })
}

/// Native W9 @1409daaa0. Mode dispatch precedes the optional extra item.
/// Camera is mandatory only for Photomode radius selection or a positive radius;
/// the cursor is mandatory only for that positive-radius, non-mode-3 branch.
pub fn object_grass_push_pass(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
) -> rhapsody::Result<()> {
    use country_core::{
        resources::brush_sizes::BrushSizes,
        systems::{
            camera_rig::main_camera::MainCamera,
            render_loop::{
                compute_passes::grass::ObjectGrassPush,
                draw_passes::object_grass_push as grass_push,
            },
        },
        utils::components::transform::Transform,
    };
    let brush = *world
        .get_resource::<BrushSizes>()
        .ok_or_else(|| missing("BrushSizes"))?;
    let mode = *world
        .get_resource::<AppMode>()
        .ok_or_else(|| missing("AppMode"))?;
    // This is a non-resource argument unused by all non-Photomode branches.
    // It does not install or pretend to construct a MainCamera.
    let radius_camera_mode = if mode == AppMode::Photomode {
        world
            .get_resource::<MainCamera>()
            .ok_or_else(|| missing("MainCamera (Photomode)"))?
            .mode_byte()
    } else {
        0
    };
    let radius = grass_push::push_radius(mode, &brush, radius_camera_mode);
    let extra = if radius > 0.0 {
        let camera = world
            .get_resource::<MainCamera>()
            .ok_or_else(|| missing("MainCamera (positive push radius)"))?;
        if camera.mode_byte() == 3 {
            Some(camera.eye_position().extend(radius))
        } else {
            let cursor = world
                .get_resource::<CursorTerrainRaycast>()
                .ok_or_else(|| missing("CursorTerrainRaycast"))?;
            Some(cursor.position().extend(radius))
        }
    } else {
        None
    };
    let mut query = world.query::<(&Transform, &ObjectGrassPush)>();
    let objects = grass_push::collect_push_objects(
        query
            .iter(world)
            .map(|(transform, push)| transform.translation.extend(push.0)),
        extra,
    );
    crate::scene_raster::with_draw_context(frame, world, |ctx| {
        grass_push::object_grass_push_pass(ctx, &objects)
    })
}

pub fn install_grass_startup(
    app: &mut bevy_app::App,
    backend: std::sync::Arc<crate::render_host::ShaderHost>,
) {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        (move |meshes: bevy_ecs::prelude::Res<
            country_core::asset_libraries::mesh_library::AssetMeshLibrary,
        >,
               mesh_watch: bevy_ecs::prelude::Res<
            country_core::asset_libraries::meshwatch::MeshWatch,
        >,
               shaders: bevy_ecs::prelude::Res<crate::run_game::ApplicationShaderLibrary>,
               mut commands: bevy_ecs::system::Commands| {
            country_core::startup::startup_grass::startup_grass(
                &meshes,
                &mesh_watch,
                &shaders.0,
                backend.as_ref(),
                &mut commands,
            );
        })
        .after(country_core::startup::startup_session::load_game_session),
    );
}

/// Native W4 uses the real startup VertCount, shared with W5's draw count.
pub fn displace_rocky_terrain(
    frame: &mut CoreGraphFrame<'_>,
    world: &World,
) -> rhapsody::Result<()> {
    let vertices = world
        .get_resource::<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>()
        .ok_or_else(|| missing("RockyTerrainMeshVertCount"))?;
    with_env(frame, world, |env| {
        country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain(env, vertices);
        Ok(())
    })
}

/// Native W5 snapshots the two actual startup entity IDs. Their component
/// handles are never replaced by shader-name queries or placeholder entities.
fn terrain_readback_draw_item(
    world: &World,
    entity: bevy_ecs::entity::Entity,
    shader_name: &'static str,
) -> rhapsody::Result<(
    country_core::render::pass_builder::draw_list::DrawListItem,
    country_core::asset_libraries::Handle<country_core::render::vao::Vao>,
)> {
    use country_core::{
        asset_libraries::Handle,
        render::{
            pass_builder::{
                draw_builder::DrawTransform,
                draw_list::{DrawListItem, DrawTopology, EntityDrawParams},
            },
            shader::RasterShaderProgram,
            vao::Vao,
        },
        utils::components::transform::Transform,
    };
    let vao = *world
        .get::<Handle<Vao>>(entity)
        .ok_or_else(|| missing("TerrainEntities entity Handle<Vao>"))?;
    let shader = *world
        .get::<Handle<RasterShaderProgram>>(entity)
        .ok_or_else(|| missing("TerrainEntities entity Handle<RasterShaderProgram>"))?;
    let transform = world
        .get::<Transform>(entity)
        .ok_or_else(|| missing("TerrainEntities entity Transform"))?;
    Ok((
        DrawListItem {
            params: EntityDrawParams {
                topology: world.get::<DrawTopology>(entity).copied(),
                entity,
                shader_name,
                transform: DrawTransform {
                    rotation: transform.rotation,
                    translation: transform.translation,
                    scale: transform.scale,
                },
            },
            vao: vao.id().raw(),
            shader: shader.id().raw(),
            sort_key: 0,
            stencil_ref: None,
        },
        vao,
    ))
}

/// Native W5: depth raster -> actual mapped-output compute declarations ->
/// readback request atomic consume. The commit is after with_env.finish so
/// delayed GPU admission errors cannot falsely mark a CPU copy as issued.
pub fn read_terrain_data(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
) -> rhapsody::Result<usize> {
    use country_core::{
        resources::rocky_terrain::{RockyTerrainMeshVertCount, TerrainEntities},
        systems::render_loop::compute_passes::read_terrain_data as readback,
    };
    let entities = world
        .get_resource::<TerrainEntities>()
        .ok_or_else(|| missing("TerrainEntities"))?;
    let floor_entity = entities.floor;
    let rocky_entity = entities.rocky;
    let (floor, floor_vao) = terrain_readback_draw_item(world, floor_entity, "floor_shader")?;
    let (rocky, rocky_vao) = terrain_readback_draw_item(world, rocky_entity, "rocky_terrain")?;
    let vertex_count = world
        .get_resource::<RockyTerrainMeshVertCount>()
        .ok_or_else(|| missing("RockyTerrainMeshVertCount"))?
        .0 as u32;
    let demo = world
        .get_resource::<country_core::game_config::GameConfig>()
        .ok_or_else(|| missing("GameConfig"))?
        .dev
        .demo;
    if !world.contains_resource::<readback::ScheduledTerrainReadback>() {
        return Err(missing("ScheduledTerrainReadback"));
    }
    frame
        .raster_resources()
        .refresh_vao_handles(world, &[floor_vao, rocky_vao])?;
    crate::scene_raster::with_draw_context(frame, world, |ctx| {
        readback::render_visual_height(ctx, &floor, &rocky, vertex_count);
    })?;
    let count = with_env(frame, world, |env| {
        let passes = readback::read_terrain_data::<readback::TerrainReadbackSsbos>(env, demo);
        let count = passes.len();
        for pass in passes {
            env.frame.record(pass);
        }
        Ok(count)
    })?;
    let tick = world.read_change_tick().get();
    world
        .resource_mut::<readback::ScheduledTerrainReadback>()
        .on_gpu_copy_issued(tick);
    Ok(count)
}

// Install only on the actual gameplay App after the same renderer installs
// its GraphicsDeviceResource, AssetSsboLibrary and shader owner. Original phase
// is Startup eight-tuple item5. Original PreStartup already loads COMMON13;
// there is no added guessed chain relation among Startup peers.
pub fn install_terrain_startup(
    app: &mut bevy_app::App,
    backend: std::sync::Arc<crate::render_host::ShaderHost>,
    installation: std::path::PathBuf,
) {
    app.add_systems(
        bevy_app::Startup,
        move |device: bevy_ecs::prelude::Res<country_core::render::device::GraphicsDeviceResource>,
              config: bevy_ecs::prelude::Res<country_core::game_config::GameConfig>,
              meshes: bevy_ecs::prelude::Res<
            country_core::asset_libraries::mesh_library::AssetMeshLibrary,
        >,
              watch: bevy_ecs::prelude::Res<
            country_core::asset_libraries::meshwatch::MeshWatch,
        >,
              shaders: bevy_ecs::prelude::Res<crate::run_game::ApplicationShaderLibrary>,
              ssbos: bevy_ecs::prelude::Res<
            country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
        >,
              mut commands: bevy_ecs::system::Commands|
              -> Result<(), bevy_ecs::error::BevyError> {
            country_core::startup::startup_terrain::startup_terrain(
                &installation,
                config.dev.demo,
                device.device(),
                backend.as_ref(),
                &meshes,
                &watch,
                &shaders.0,
                &ssbos,
                &mut commands,
            )
            .map_err(Into::into)
        },
    );
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.component::<country_core::utils::components::transform::Transform>();
    m.component::<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush>();
    m.resource::<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>();
    m.resource::<country_core::resources::walls::public_walls::PublicWalls>();
    m.resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>();
    m.resource::<country_core::render::device::GraphicsDeviceResource>();
    m.resource::<country_core::resources::terrain_editing::TerrainEditingData>();
    m.events::<RecalculateHeightMapCmd>();
    m.resource::<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
    m.resource::<country_core::systems::mode_manager::AppMode>();
    m.events::<AddTerrainHighlightCmd>();
    m.resource::<country_core::resources::gui::DebugGui>();
    m.resource::<country_core::resources::world_raster_defs::PathRaster>();
    m.resource::<country_core::resources::world_raster_defs::Cobblestone1Raster>();
    m.resource::<country_core::resources::world_raster_defs::Cobblestone2Raster>();
    m.resource::<country_core::resources::world_raster_defs::Cobblestone3Raster>();
    m.resource::<country_core::resources::brush_sizes::BrushSizes>();
    m.resource::<country_core::systems::camera_rig::main_camera::MainCamera>();
    m.resource::<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>();
    m.resource::<country_core::resources::rocky_terrain::TerrainEntities>();
    m.resource::<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>();
    m.resource::<country_core::game_config::GameConfig>();
    m.resource::<country_core::systems::render_loop::compute_passes::read_terrain_data::ScheduledTerrainReadback>();
    m.component::<country_core::asset_libraries::Handle<country_core::render::vao::Vao>>();
    m.component::<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>>();
    m.component::<country_core::utils::components::transform::Transform>();
    m.component::<country_core::render::pass_builder::draw_list::DrawTopology>();
}
