//! B look_at_camera140873de0/1476B. Ignore profiling/library iterator glue;
//! each matching transform faces the maintained camera eye with world +Y up.
use crate::{
    systems::camera_rig::main_camera::MainCamera, utils::components::transform::Transform,
};
use bevy_ecs::prelude::*;
#[derive(Component, Debug, Default)]
pub struct LookAtCamera;
pub fn look_at_camera(mut query: Query<(ComponentWrite<Transform>, &LookAtCamera)>, camera: Res<MainCamera>) {
    for (mut transform, _) in &mut query {
        transform.look_at(camera.eye_position(), glam::Vec3::Y);
    }
}

use crate::render::extract::ComponentWrite;
