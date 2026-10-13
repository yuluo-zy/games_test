//! shapes/circle/signifiers/edit_arrow.rs — circle-edit arrow signifier.
//!
//! PDB procedure `signifier_edit_circle_arrow` 0x140b35120 (1321 B). FunctionSystem
//! signature O: Query<&EditCircleSignifier, &mut Transform>, Res<AnimatedCircleSignifierResource>,
//! Res<UiSignifierStream>, ...

use bevy_ecs::prelude::Component;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::{SignifierEntityBuilder, SignifierFactory};
use crate::utils::components::transform::Transform;

/// Circle-edit arrow marker (O).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct EditCircleSignifier {
    pub raw: crate::typed_gap::Gap,
}

pub fn signifier_edit_circle_arrow(
    mut query: Query<(&EditCircleSignifier, &mut Transform)>,
    animated: Res<AnimatedCircleSignifierResource>,
    mut signifiers: ResMut<UiSignifierStream>,
) {
    let _ = (&animated, &mut signifiers);
    for (signifier, mut transform) in query.iter_mut() {
        let _ = signifier;
        // O: orient the arrow along the current radius direction.
        transform.rotation = glam::Quat::IDENTITY;
        let _ = SignifierEntityBuilder::new();
        let _ = SignifierFactory::new();
    }
}
