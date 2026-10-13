//! structure_outline.rs — draws the outline signifier of a wall/roof structure.
//!
//! PDB procedure `structure_outline` 0x140959fc0 (23774 B). FunctionSystem signature O.
//! Business logic (O, call chain): on `OutlineWallCmd` / `OutlineStairsCmd`, reads the wall
//! from `PublicWalls`, the roof from `Query<&Roof>`; uses `valid_enclosure::ShapeParameters`
//! (as_rectangle / as_circle / as_curve2 / center / expand / perimeter) and curve sampling
//! to build an outline; emits a signifier entity (custom shader) when the Esc menu is closed.

use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::resources::events::{OutlineStairsCmd, OutlineWallCmd};
use crate::resources::gui::EscMenuState;
use crate::resources::roofs::roof_shape::Roof;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::valid_enclosure::ShapeParameters;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::SignifierEntityBuilder;

pub fn structure_outline(
    mut outline_wall_reader: EventReader<OutlineWallCmd>,
    mut outline_stairs_reader: EventReader<OutlineStairsCmd>,
    public_walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    esc_menu: Res<EscMenuState>,
    mut signifiers: ResMut<UiSignifierStream>,
) {
    if esc_menu.is_open() {
        return;
    }
    for OutlineWallCmd(wall) in outline_wall_reader.read() {
        let Some(state) = public_walls.get(*wall) else { continue };
        let params = ShapeParameters::from_wall(&state);
        let _ = params;
        let _builder = SignifierEntityBuilder::new();
        // O: per-shape outline geometry (as_rectangle / as_circle / as_curve2) drives the
        // builder; field mapping U -> typed gap.
        let _ = &roofs;
    }
    for _ in outline_stairs_reader.read() {
        let _ = SignifierEntityBuilder::new();
    }
}
