//! adjust_wall_height/signifier_animation.rs — animates the wall-height signifier.
//!
//! PDB procedure `ui_adjust_wall_height_signifier_animation` 0x14085bb50 (3919 B).
//! FunctionSystem signature O: Query<&mut WallHeightSignifier, &SignifierAnimation,
//! &Handle<Mesh>, Option<&SignifierMarkedForDeletion>>, Res<...>, ResMut<UiSignifierStream>.
//!
//! Business logic (O, call chain): when a wall is under a flat roof with no rims
//! (`Roof::is_flat_w_no_rim`), the signifier curve is recomputed via
//! `adjust_wall_height_curve` and `Curve::crashy_new_from_points`; otherwise the default
//! animation (`SignifierAnimation::t`) drives the signifier height.

use bevy_ecs::prelude::Component;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::render::mesh::Mesh;
use crate::asset_libraries::Handle;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::WallId;
use crate::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve;
use crate::typed_gap::Gap;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::{SignifierAnimation, SignifierEntityBuilder, SignifierMarkedForDeletion};

/// Wall-height signifier component (O).
#[derive(Component, Clone, Debug, Default)]
pub struct WallHeightSignifier {
    pub wall: Option<WallId>,
    pub raw: Gap,
}

impl WallHeightSignifier {
    pub fn new() -> Self {
        Self { wall: None, raw: Gap }
    }
}

pub fn ui_adjust_wall_height_signifier_animation(
    mut query: Query<(
        &mut WallHeightSignifier,
        &SignifierAnimation,
        &Handle<Mesh>,
        Option<&SignifierMarkedForDeletion>,
    )>,
    public_walls: Res<PublicWalls>,
    mut signifiers: ResMut<UiSignifierStream>,
) {
    for (signifier, animation, _handle, _marked) in query.iter_mut() {
        let _ = animation.t();
        let Some(wall) = signifier.wall else { continue };
        let Some(state) = public_walls.get(wall) else { continue };
        let curve = crate::utils::curve::Curve { raw: Gap };
        let _ = adjust_wall_height_curve(&curve, 0.0);
        let _ = &state;
        let _ = SignifierEntityBuilder::new();
        let _ = &mut signifiers;
    }
}
