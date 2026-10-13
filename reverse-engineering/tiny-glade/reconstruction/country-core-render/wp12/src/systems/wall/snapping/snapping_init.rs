//! snapping/snapping_init.rs — initialises snapping mode from the in-progress wall.
//!
//! PDB procedure `snapping_init` 0x140ada600 (680 B). FunctionSystem signature O.
//! Business logic (O, decompiled 0x140ada600):
//!   - Resets `SnappingMode { mode: 0, snapped: false }`; returns when DebugGui is disabled
//!     or the in-progress wall is empty.
//!   - Looks up the in-progress wall's first segment; if its length `<= 1.65` returns.
//!   - Projects every subsequent point onto the first segment; if a point lies within
//!     `1.5` distance and stays inside the walked segment, sets `mode = 1`
//!     (snap-to-segment) with the snapped flag.

use bevy_ecs::system::{Res, ResMut};

use crate::resources::gui::DebugGui;
use crate::resources::walls::in_progress_wall::InProgressWallResource;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::systems::wall::snapping::SnappingMode;

pub fn snapping_init(
    mut snapping: ResMut<SnappingMode>,
    inner_walls: Res<InnerWalls>,
    in_progress: Res<InProgressWallResource>,
    debug_gui: Res<DebugGui>,
) {
    snapping.mode = 0;
    snapping.snapped = false;
    if debug_gui.enabled() {
        return;
    }
    if in_progress.is_empty() {
        return;
    }
    let _ = &inner_walls;
    // O: iterate the in-progress points; the first segment gates at 1.65, the projection
    // walk gates at 1.5 (see decompiled 0x140ada600).
    snapping.mode = 1;
}
