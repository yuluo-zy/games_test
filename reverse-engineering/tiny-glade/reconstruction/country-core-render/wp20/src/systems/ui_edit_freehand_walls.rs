//! `systems/ui_edit_freehand_walls.rs` — freehand-wall edit UI.
//!
//! PDB: `ui_edit_freehand_walls @0x140ac98e0` (12129 B) + `closure$0
//! @0x140acc850`. Named entity `EditFreehandWallCache`.
//!
//! O: accumulates cursor drag into the freehand wall cache and commits wall
//! segments to the wall-constructor systems on release.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::Gap;

/// `EditFreehandWallCache` (O named entity; stroke cache).
#[derive(Resource, Clone, Debug, Default)]
pub struct EditFreehandWallCache {
    pub raw: Gap,
}

/// `ui_edit_freehand_walls` @0x140ac98e0 (O).
pub fn ui_edit_freehand_walls(
    mut _cache: ResMut<EditFreehandWallCache>,
    _cursor: Res<Gap>,
    _input: Res<Gap>,
    _commands: Commands,
) {
    todo!("recover ui_edit_freehand_walls (drag → cache → wall segments)")
}
