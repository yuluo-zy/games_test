//! shapes/rectangle/ui_edit_rectangle_orientation.rs — rectangle orientation edit UI.
//!
//! PDB procedures: `ui_edit_rectangle_rotate` 0x140ad8000 (3035 B),
//! `do_ui_edit_rectangle_rotate` 0x140ad8be0 (4905 B), `box_trigger_volume` 0x140ada0e0,
//! `skewed_box_trigger_vol` 0x140ad9f10. FunctionSystem signatures O.

use bevy_ecs::event::EventWriter;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::events::{EditRectangleCmd, FinishEditRectangleCmd};
use crate::resources::stairs_walls::StairsPillarProposals;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::shapes::rectangle::WipRectangle;
use crate::ui::mode_manager::AppMode;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::ui_state::UiState;

pub fn box_trigger_volume(_center: glam::Vec3, _dims: glam::Vec3) -> crate::typed_gap::Gap {
    crate::typed_gap::Gap
}
pub fn skewed_box_trigger_vol(_center: glam::Vec3, _dims: glam::Vec3, _axis: glam::Vec3) -> crate::typed_gap::Gap {
    crate::typed_gap::Gap
}

pub fn ui_edit_rectangle_rotate(
    app_mode: Res<AppMode>,
    public_walls: Res<PublicWalls>,
    mut signifiers: ResMut<UiSignifierStream>,
    ui_state: Res<UiState>,
    mut wip: ResMut<Option<WipRectangle>>,
) {
    let _ = (&app_mode, &public_walls, &mut signifiers, &ui_state, &mut wip);
}

pub fn do_ui_edit_rectangle_rotate(
    public_walls: Res<PublicWalls>,
    mut wip: ResMut<Option<WipRectangle>>,
    mut rect_writer: EventWriter<EditRectangleCmd>,
    mut finish_writer: EventWriter<FinishEditRectangleCmd>,
) {
    let _ = (&public_walls, &mut wip, &mut rect_writer, &mut finish_writer);
}

pub fn ui_edit_rectangle_orientation_stairs(
    mut proposals: ResMut<StairsPillarProposals>,
    mut signifiers: ResMut<UiSignifierStream>,
) {
    let _ = (&mut proposals, &mut signifiers);
}
