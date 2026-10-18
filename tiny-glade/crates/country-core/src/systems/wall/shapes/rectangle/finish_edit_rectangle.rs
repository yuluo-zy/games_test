//! B `systems/wall/shapes/rectangle/finish_edit_rectangle.rs`: rectangle edit
//! finish.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/finish_edit_rectangle.rs.md`
//! (O, 0x141464a60) + `systems/wall/shapes/rectangle/edit_decision.rs` (the
//! `finish_rectangle_edit_decision` core). The 0.01 dual criteria (orientation
//! degrees, dimension length) decide whether the change is recorded; the
//! decorator restore is host 待接线.

use bevy_ecs::prelude::*;

use super::super::FinishEditRectangleCmd;
use crate::systems::wall::shapes::rectangle::edit_decision::RectangleEditState;
use crate::systems::wall::shapes::rectangle::edit_decision::finish_rectangle_edit_decision;
use crate::resources::walls::InnerWalls;
use crate::systems::wall::history_recording::RecordWallHistory;

/// UiState80 edit snapshot resource (variants 1 = dims, 2 = orientation).
#[derive(Resource, Clone, Debug)]
pub struct RectangleEditUiState(pub [u8; 80]);

impl Default for RectangleEditUiState {
    fn default() -> Self {
        Self([0; 80])
    }
}

/// B 0x141464a60 (9-param system).
pub fn finish_edit_rectangle(
    mut finish: EventReader<FinishEditRectangleCmd>,
    mut ui_state: ResMut<RectangleEditUiState>,
    inner: Res<InnerWalls>,
    mut history: EventWriter<RecordWallHistory>,
) {
    for event in finish.read() {
        let mut state = RectangleEditState(ui_state.0);
        let source = inner.get(event.wall).map(|s| s.source());
        let decision = match finish_rectangle_edit_decision(&mut state, true, source) {
            Ok(decision) => decision,
            Err(_) => continue,
        };
        ui_state.0 = state.0;
        if let Some(edit) = decision.history {
            history.write(RecordWallHistory(std::sync::Arc::new(edit)));
        }
        if decision.restore_archived_decorators {
            // extract_before_after_all decorator restore; host 待接线.
        }
        if let Some(wall) = decision.wall_notification {
            let _ = wall;
        }
    }
}
