//! ui_continue_wall.rs — continues a finished wall from its end when the cursor moves.
//!
//! PDB procedures: `suppress_continue_wall` 0x140aadbb0 (375 B), `ui_continue_wall`
//! 0x140aadd30 (7834 B). FunctionSystem signatures O.
//!
//! Business logic (O, call chain): `ui_continue_wall` detects a wall whose end the cursor
//! approaches (UiState drag/hover/intersection), prepares a continuation: `InProgressWall::new`
//! from the wall end, validates `GladeBorder::is_pos_inside`, decorator cull, and
//! `WallContinueState::display_signifier` / `finish`. `suppress_continue_wall` clears the
//! continuation when modifiers change or the cursor moves away.

use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::events::FinishWall;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::in_progress_wall::{InProgressWall, InProgressWallResource};
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::window::CursorMovedScreenSpace;
use crate::ui::mod_key::Modifiers;
use crate::ui::mode_manager::AppMode;
use crate::ui::signifiers::SignifierEntityBuilder;
use crate::ui::ui_state::UiState;

/// Suppression flag resource (O: `SuppressContinueWall`).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SuppressContinueWall(pub bool);

/// Continue-wall UI state (O: `WallContinueState::display_signifier/finish`).
#[derive(Resource, Clone, Debug, Default)]
pub struct WallContinueState {
    pub active: bool,
    pub wall: Option<crate::resources::walls::WallId>,
}
impl WallContinueState {
    pub fn display_signifier(&mut self, _builder: SignifierEntityBuilder) {}
    pub fn finish(&mut self) {
        self.active = false;
        self.wall = None;
    }
}

pub fn suppress_continue_wall(
    modifiers: Res<Modifiers>,
    mut suppress: ResMut<SuppressContinueWall>,
    mut cursor: EventReader<CursorMovedScreenSpace>,
) {
    let _ = &modifiers;
    for _ in cursor.read() {
        suppress.0 = false;
    }
}

pub fn ui_continue_wall(
    app_mode: Res<AppMode>,
    ui_state: Res<UiState>,
    inner_walls: Res<InnerWalls>,
    public_walls: Res<PublicWalls>,
    mut state: ResMut<WallContinueState>,
    mut in_progress: ResMut<InProgressWallResource>,
    cursor: Res<CursorTerrainRaycast>,
    mut finish_writer: bevy_ecs::event::EventWriter<FinishWall>,
) {
    let _ = (&app_mode, &ui_state, &inner_walls, &public_walls, &cursor, &mut finish_writer);
    if state.active {
        // O: cursor near an existing wall end -> prepare continuation.
        let _pos = cursor.position();
        if in_progress.wall.is_none() {
            in_progress.wall = Some(InProgressWall::new());
        }
    } else {
        state.finish();
    }
}
