//! Complete B debug_viz_threshold_curve140a8b0c0/1129B producer.
use crate::{
    resources::{gui::DebugGui, walls::public_walls::PublicWalls},
    systems::ui::signifiers::UiSignifierStream,
};
use bevy_ecs::prelude::*;

#[path = "debug_threshold_policy.rs"]
mod policy;

pub fn debug_viz_threshold_curve(
    debug: Res<DebugGui>,
    walls: Res<PublicWalls>,
    mut signifiers: ResMut<UiSignifierStream>,
) {
    if !debug.show_threshold_curve {
        return;
    }
    for (_, wall) in walls.all() {
        // Original iterates keys and calls the validated PublicWalls::get for
        // every row, even when the prepared arrays are empty.
        wall.assert_valid();
        let Some(path) = wall.prepared_path.as_ref() else {
            continue;
        };
        policy::threshold_spheres(
            &path.positions,
            &path.segment_values,
            wall.arch_segment_heights().unwrap_or(&[]),
            &wall.segment_heights,
            |sphere| sphere.show(&mut signifiers),
        );
    }
}
