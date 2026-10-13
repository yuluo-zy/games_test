//! shapes/shape_placement_signifiers.rs — shape-placement preview signifiers.
//!
//! PDB procedures: `shape_placement_signifiers` 0x1408dec70 (2628 B),
//! `preview_snapping_line` 0x1408dfc60 (2670 B). FunctionSystem signature O.
//! Business logic (O, call chain): draws the placement preview (snapping line to the
//! snapped wall end via `ShapeParameters::expand` / curve sampling) and the shape outline
//! signifier.

use bevy_ecs::system::{Res, ResMut};

use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::shapes::shape_placement::ShapePlacement;
use crate::systems::wall::valid_enclosure::ShapeParameters;
use crate::ui::mode_manager::AppMode;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::SignifierEntityBuilder;

pub fn preview_snapping_line(
    _a: glam::Vec3,
    _b: glam::Vec3,
    _t: f32,
    _builder: SignifierEntityBuilder,
) {
}

pub fn shape_placement_signifiers(
    app_mode: Res<AppMode>,
    placement: Res<ShapePlacement>,
    public_walls: Res<PublicWalls>,
    mut signifiers: ResMut<UiSignifierStream>,
) {
    let _ = (&app_mode, &public_walls, &mut signifiers);
    if !placement.snapped {
        return;
    }
    let _ = ShapeParameters::Curve(crate::utils::curve::Curve { raw: crate::typed_gap::Gap });
    let _builder = SignifierEntityBuilder::new();
    let _ = placement.pos;
}
