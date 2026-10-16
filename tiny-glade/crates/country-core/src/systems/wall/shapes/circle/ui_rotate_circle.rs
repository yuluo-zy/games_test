//! B `systems/wall/shapes/circle/ui_rotate_circle.rs`: circle wall rotation.
//!
//! Evidence: `chapters/systems/wall/shapes/circle/ui_rotate_circle.rs.md` (O,
//! 0x1409ac910). Screen-space rotation updates the circle's roof-ridge
//! orientation (the +0x10 spare/rotation float) and rebuilds the curve and
//! enclosure. The screen↔world angle math is render/UI domain (待接线); the
//! CPU-side wall update is exposed here.

use bevy_ecs::prelude::*;

use super::super::{RotateCircleCmd, to_authority_curve};
use crate::recovered_building::wall_state::WallShapeSnapshot;
use crate::resources::walls::InnerWalls;
use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};

/// Circle sample step (0x3e4ccccd @0x1429cd068).
const CIRCLE_SAMPLE_STEP: f32 = 0.2;

/// Apply a new rotation angle to the circle wall enclosure.
pub fn rotate_circle_wall(inner: &mut InnerWalls, wall: crate::resources::walls::WallId, angle: f32) -> bool {
    let Some(state) = inner.get(wall).cloned() else {
        return false;
    };
    let Some(params) = state.get_enclosure_params() else {
        return false;
    };
    let mut circle = ShapeParameters::from_snapshot(params).as_circle();
    circle.rotation = angle;
    let curve = circle.as_curve2(CIRCLE_SAMPLE_STEP);
    inner.update_wall_state_curve(wall, to_authority_curve(&curve));
    let snapshot = WallShapeSnapshot::from_observed(
        ShapeParameters::from_shape(Shape::Circle(circle)).to_record(),
    );
    inner.update_wall_state_enclosure(wall, snapshot);
    true
}

/// B 0x1409ac910 (16-param system); input/rotation angle is the UI producer.
pub fn ui_rotate_circle(
    mut requests: EventReader<RotateCircleCmd>,
    mut inner: ResMut<InnerWalls>,
) {
    for event in requests.read() {
        rotate_circle_wall(&mut inner, event.wall, event.rotation);
    }
}
