//! B 0x140b0e380: delayed growth and endpoint-thickness propagation.
use crate::resources::{
    ivy::{IvyLeafStorage, IvyStorage},
    time::GameTime,
};
use bevy_ecs::prelude::*;

/// The native event row contains its EventId only; this is a zero-sized event.
#[derive(Event, Clone, Copy, Debug)]
pub struct IvyLeafIsGrowing;

pub fn ivy_leaf_animation(
    mut leaves: ResMut<IvyLeafStorage>,
    time: Res<GameTime>,
    storage: Res<IvyStorage>,
    mut growing: EventWriter<IvyLeafIsGrowing>,
) {
    for leaf in leaves.all_leaves_mut() {
        if leaf.growth < 1.0 {
            if leaf.growth_delay <= 0.0 {
                let value = leaf.growth + time.delta + time.delta;
                leaf.growth = if 1.0 <= value { 1.0 } else { value };
                growing.write(IvyLeafIsGrowing);
            } else {
                leaf.growth_delay -= time.delta;
            }
        }
        let Some(ivy) = storage.get(leaf.ivy) else {
            log::warn!(target: "country_core::systems::ivy::ivy_leaf_animation", "Missing ivy {} for leaf", leaf.ivy);
            continue;
        };
        let (Some(a), Some(b)) = (
            ivy.get_point_id(leaf.point_a),
            ivy.get_point_id(leaf.point_b),
        ) else {
            log::warn!(target: "country_core::systems::ivy::ivy_leaf_animation", "Missing ivy {} leaf endpoints {}, {}", leaf.ivy, leaf.point_a, leaf.point_b);
            continue;
        };
        let average = (a.thickness + b.thickness) * 0.5;
        leaf.branch_thickness = (1.0 - average) * 0.5 + average;
    }
}
