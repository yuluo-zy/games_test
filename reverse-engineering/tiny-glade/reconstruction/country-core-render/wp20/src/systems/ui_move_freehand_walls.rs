//! `systems/ui_move_freehand_walls.rs` — move freehand walls UI.
//!
//! PDB: `ui_move_freehand_walls @0x1409781d0` (7816 B). Named entity
//! `MoveWallCache`.
//!
//! O: drag moves the selected freehand wall segment; `MoveWallCache` tracks the
//! in-flight movement for the wall-constructor update.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::Gap;

/// `MoveWallCache` (O named entity).
#[derive(Resource, Clone, Debug, Default)]
pub struct MoveWallCache {
    pub raw: Gap,
}

/// `ui_move_freehand_walls` @0x1409781d0 (O).
pub fn ui_move_freehand_walls(
    mut _cache: ResMut<MoveWallCache>,
    _cursor: Res<Gap>,
    _input: Res<Gap>,
    _commands: Commands,
) {
    todo!("recover ui_move_freehand_walls (drag → MoveWallCache → wall update)")
}
