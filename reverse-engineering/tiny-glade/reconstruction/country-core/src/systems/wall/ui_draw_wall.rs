//! ui_draw_wall.rs — UI-layer wall drawing: emits RecordMouseInputIntoWall and finishes.
//!
//! PDB procedure `ui_draw_wall` 0x140aafe40 (3041 B). FunctionSystem signature O.
//! Business logic (O, call chain): while dragging within the glade and not clearing the
//! canvas, forwards cursor strokes as `RecordMouseInputIntoWall` events into the
//! `CurveNursery`; on release `WallDrawState::finish` writes a `FinishWall` event.

use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::events::{FinishWall, RecordMouseInputIntoWall};
use crate::resources::glade_border::GladeBorder;
use crate::ui::mode_manager::AppMode;
use crate::ui::ui_state::UiState;
use crate::utils::debug_viz::DebugLine;

/// Wall draw UI state (O: `WallDrawState::finish`).
#[derive(Resource, Clone, Debug, Default)]
pub struct WallDrawState {
    pub drawing: bool,
}
impl WallDrawState {
    pub fn finish(&mut self) {
        self.drawing = false;
    }
}

pub fn ui_draw_wall(
    app_mode: Res<AppMode>,
    mut state: ResMut<WallDrawState>,
    cursor: Res<CursorTerrainRaycast>,
    glade_border: Res<GladeBorder>,
    ui_state: Res<UiState>,
    mut record_writer: EventWriter<RecordMouseInputIntoWall>,
    mut finish_writer: EventWriter<FinishWall>,
) {
    let _ = &app_mode;
    let pos = cursor.position();
    let inside = glade_border.is_pos_inside(pos);
    let _ = &ui_state;
    if !state.drawing {
        if inside && !ui_state.clearing_canvas() {
            state.drawing = true;
        } else {
            return;
        }
    }
    if inside && !ui_state.clearing_canvas_w_layer() {
        record_writer.write(RecordMouseInputIntoWall);
    } else {
        state.finish();
        finish_writer.write(FinishWall);
    }
    let _ = DebugLine::new(glam::Vec3::ZERO, glam::Vec3::ZERO);
}
