//! B 0x140a8a710: the final four terrain-mapped XZ directions drive the bend
//! scalar event. The historical reconstruction used the wrong end of the curve.

use crate::resources::walls::public_walls::{OnWallChanged, PublicWalls};
use bevy_ecs::prelude::*;

#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct WallBendChangedEvent(pub f32);

pub fn compute_bend(points: &[[f32; 3]]) -> f32 {
    if points.len() < 6 {
        return 0.0;
    }
    let directions: [[f32; 2]; 4] = std::array::from_fn(|i| {
        let end = points.len() - 1 - i;
        let dx = points[end][0] - points[end - 1][0];
        let dz = points[end][2] - points[end - 1][2];
        let inverse = 1.0 / (dz * dz + dx * dx).sqrt();
        [inverse * dx, inverse * dz]
    });
    let mut sum = 0.0;
    for pair in directions.windows(2) {
        sum += pair[0][1] * pair[1][1] + pair[0][0] * pair[1][0];
    }
    let average = sum / 3.0;
    // 140a8ace8/acec put average in the source operand; unordered values
    // survive both clamps. The final MINSS selects its numeric source 1.
    let average = if -1.0 > average { -1.0 } else { average };
    let average = if 1.0 < average { 1.0 } else { average };
    let angle = average.acos();
    if angle < 1.0 { angle } else { 1.0 }
}

pub fn calc_wall_bend(
    mut changed: EventReader<OnWallChanged>,
    public: Res<PublicWalls>,
    mut bends: EventWriter<WallBendChangedEvent>,
) {
    for &OnWallChanged(id) in changed.read() {
        if let Some(state) = public.get(id) {
            bends.write(WallBendChangedEvent(compute_bend(&state.geometry.outline)));
        }
    }
}
