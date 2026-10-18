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
