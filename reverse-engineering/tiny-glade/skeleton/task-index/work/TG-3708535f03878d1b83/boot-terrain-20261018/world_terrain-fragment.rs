
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
            render_loop::{compute_passes::grass::ObjectGrassPush,
                draw_passes::object_grass_push as grass_push},
        },
        utils::components::transform::Transform,
    };
    let brush = *world.get_resource::<BrushSizes>()
        .ok_or_else(|| missing("BrushSizes"))?;
    let mode = *world.get_resource::<AppMode>()
        .ok_or_else(|| missing("AppMode"))?;
    // This is a non-resource argument unused by all non-Photomode branches.
    // It does not install or pretend to construct a MainCamera.
    let radius_camera_mode = if mode == AppMode::Photomode {
        world.get_resource::<MainCamera>()
            .ok_or_else(|| missing("MainCamera (Photomode)"))?.mode_byte()
    } else { 0 };
    let radius = grass_push::push_radius(mode, &brush, radius_camera_mode);
    let extra = if radius > 0.0 {
        let camera = world.get_resource::<MainCamera>()
            .ok_or_else(|| missing("MainCamera (positive push radius)"))?;
        if camera.mode_byte() == 3 {
            Some(camera.eye_position().extend(radius))
        } else {
            let cursor = world.get_resource::<CursorTerrainRaycast>()
                .ok_or_else(|| missing("CursorTerrainRaycast"))?;
            Some(cursor.position().extend(radius))
        }
    } else { None };
    let mut query = world.query::<(&Transform, &ObjectGrassPush)>();
    let objects = grass_push::collect_push_objects(
        query.iter(world).map(|(transform, push)| transform.translation.extend(push.0)),
        extra,
    );
    crate::scene_raster::with_draw_context(frame, world, |ctx| {
        grass_push::object_grass_push_pass(ctx, &objects)
    })
}
