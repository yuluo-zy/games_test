//! Original recycle<update_roof_bird_spots::closure_env$3>, 1422f86e0/378B.
use bevy_ecs::prelude::*;
use crate::bird_spots::BirdSpotManager;
use crate::components::{Transform, Wren};
use super::SittableForBirds;

/// Query owns live-generation and archetype/filter membership checks. Merely
/// inspecting an ineligible row must not set its Changed tick.
pub fn recycle_returned_bird_spots(
    manager: &mut BirdSpotManager,
    wrens: &mut Query<(Entity, ComponentWrite<Transform>, &mut Wren), Without<SittableForBirds>>,
) {
    manager.recycle(|spot| {
        let Some(entity) = spot.occupant else { return };
        let Ok((_, _, mut wren)) = wrens.get_mut(entity) else { return };
        if wren.spot.state == 0 {
            wren.spot.state = 1;
        }
    });
}

use country_core::render::extract::ComponentWrite;
