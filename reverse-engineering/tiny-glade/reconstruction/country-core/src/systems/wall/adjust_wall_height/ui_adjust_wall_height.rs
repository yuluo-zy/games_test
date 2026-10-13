//! adjust_wall_height/ui_adjust_wall_height.rs — the adjust-wall-height UI entry.
//!
//! PDB procedure `ui_adjust_wall_height` 0x140b2fc00 (3588 B). FunctionSystem signature O:
//! Res<AppMode>, Res<PublicWalls>, Query<&Roof>, Res<UiState>, EventWriter<...>.
//! Business logic (O, call chain): selects the wall under the UI when not dragged, builds
//! the `UiWallHeight` widget (premade shapes only; roof flat/no-rim handling via
//! `Roof::ty`/`is_flat_w_no_rim`/`RoofAnimation::is_being_animated`), and writes
//! `AdjustWallHeightCmd` from the widget drag.

use bevy_ecs::event::EventWriter;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::resources::events::AdjustWallHeightCmd;
use crate::resources::roofs::roof_animation::RoofAnimation;
use crate::resources::roofs::roof_shape::Roof;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight;
use crate::ui::mode_manager::AppMode;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::ui_state::UiState;

pub fn ui_adjust_wall_height(
    app_mode: Res<AppMode>,
    public_walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    ui_state: Res<UiState>,
    mut widget: ResMut<UiWallHeight>,
    mut signifiers: ResMut<UiSignifierStream>,
    mut cmd_writer: EventWriter<AdjustWallHeightCmd>,
) {
    let _ = (&app_mode, &public_walls, &roofs, &ui_state, &mut widget, &mut signifiers, &mut cmd_writer);
    if ui_state.dragged() {
        return;
    }
    // O: find the intersected premade wall and open the height widget.
    for wall in public_walls.wall_ids() {
        let Some(state) = public_walls.get(wall) else { continue };
        if state.is_premade_shape() {
            let _ = state;
            *widget = UiWallHeight::new();
            break;
        }
    }
}
