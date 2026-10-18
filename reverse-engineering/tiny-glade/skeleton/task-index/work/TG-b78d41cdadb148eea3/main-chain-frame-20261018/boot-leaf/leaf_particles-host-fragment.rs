/// W18@1408f4d00: zero leaf count returns before emitter/GPU library access.
pub fn leaf_particles_compute(
    frame: &mut CoreGraphFrame<'_>,
    world: &World,
) -> rhapsody::Result<()> {
    use country_core::systems::render_loop::compute_passes::leaf_particles as leaf;
    let count = world
        .get_resource::<country_core::resources::glade_settings::GladeSettings>()
        .ok_or_else(|| missing("GladeSettings"))?
        .leaf_particle_count;
    if count == 0 {
        return Ok(());
    }
    with_env(frame, world, |env| leaf::leaf_particles_compute(env, count, || {
        world.get_resource::<leaf::LoadedParticleEmitterCount>()
            .map(|count| count.0)
            .ok_or_else(|| missing("LoadedParticleEmitterCount (original init_clearing)"))
    }))
}
