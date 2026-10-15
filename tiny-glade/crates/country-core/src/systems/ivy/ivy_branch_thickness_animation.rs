//! B 0x140995a80. This is point-count based; it does not read GameTime.
use crate::resources::ivy::{Ivy, IvyStorage};
use bevy_ecs::prelude::*;

pub fn animate_branch_thickness(ivy: &mut Ivy) {
    let count = ivy.points.len();
    // Original CRT expf at 0x142923ec0, divisor -10.0 at 0x142a16d10.
    let attenuation = (count as f32 / -10.0).exp();
    for (index, point) in ivy.points.iter_mut().enumerate() {
        let taper = 1.0 - index as f32 / count.wrapping_sub(1) as f32;
        point.thickness = taper * (1.0 - attenuation) + (1.0 - taper) * 0.0;
    }
}

pub fn ivy_branch_thickness_animation(mut storage: ResMut<IvyStorage>) {
    for (_, ivy) in storage.iter_mut() {
        animate_branch_thickness(ivy);
    }
}
