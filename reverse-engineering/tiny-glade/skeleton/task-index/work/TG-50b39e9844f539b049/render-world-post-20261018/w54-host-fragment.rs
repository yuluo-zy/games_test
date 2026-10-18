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
    if !world.contains_resource::<ShaderRefactoringState>() {
        return Err(missing("ShaderRefactoringState"));
    }
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
