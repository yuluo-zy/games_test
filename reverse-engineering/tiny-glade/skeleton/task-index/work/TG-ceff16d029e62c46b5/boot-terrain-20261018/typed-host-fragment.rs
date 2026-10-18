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
