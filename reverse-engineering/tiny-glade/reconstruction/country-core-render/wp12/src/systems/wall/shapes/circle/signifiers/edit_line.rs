//! shapes/circle/signifiers/edit_line.rs — circle-edit line signifier.
//!
//! PDB procedure `signifier_edit_circle_line` 0x140a64a40 (2760 B). FunctionSystem
//! signature O: Query<&mut SignifierEditCircleShapeLine, &Handle<Mesh>, &SignifierAnimation>,
//! Res<AnimatedCircleSignifierResource>, Res<UiSignifierStream>, ...

use bevy_ecs::prelude::Component;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::render::mesh::Mesh;
use crate::render::Handle;
use crate::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource;
use crate::typed_gap::Gap;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::{SignifierAnimation, SignifierEntityBuilder, SignifierFactory};
use crate::utils::curve::CurveU;

/// Circle-edit shape line marker (O).
#[derive(Component, Clone, Debug, Default)]
pub struct SignifierEditCircleShapeLine {
    pub raw: Gap,
}

pub fn signifier_edit_circle_line(
    mut query: Query<(
        &mut SignifierEditCircleShapeLine,
        &Handle<Mesh>,
        &SignifierAnimation,
    )>,
    animated: Res<AnimatedCircleSignifierResource>,
    mut signifiers: ResMut<UiSignifierStream>,
) {
    let _ = (&animated, &mut signifiers);
    for (signifier, _handle, animation) in query.iter_mut() {
        let _ = (signifier, animation.t());
        let _ = CurveU { raw: Gap };
        let _ = SignifierEntityBuilder::new();
        let _ = SignifierFactory::new();
    }
}
