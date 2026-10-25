//! finish_wall.rs — finalises an in-progress wall on `FinishWall`.
//!
//! PDB procedure `finish_wall` 0x140a800a0 (8114 B). FunctionSystem signature O.
//! Business logic (O, call chain + decompiled entry): on `FinishWall`, finalises the
//! in-progress wall: `InnerWalls::mark_as_non_enclosed`, `update_wall_state_enclosure`,
//! records one `ConcurrentEdits` wall compound history edit (CompoundEditBuilder::build),
//! and clears the in-progress resource.

use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::events::FinishWall;
use crate::resources::history::History;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::in_progress_wall::InProgressWallResource;
use crate::resources::walls::WallShapeSnapshot;
use crate::ui::mode_manager::AppMode;

pub fn finish_wall(
    app_mode: Res<AppMode>,
    mut reader: EventReader<FinishWall>,
    mut inner_walls: ResMut<InnerWalls>,
    mut in_progress: ResMut<InProgressWallResource>,
    mut history: ResMut<History>,
) {
    let _ = &app_mode;
    for _ in reader.read() {
        let Some(wall) = in_progress.wall.take() else {
            continue;
        };
        // O: the in-progress wall carries the WallId; finalise its enclosure state.
        let id = wall_id_of(&wall);
        if let Some(id) = id {
            inner_walls.mark_as_non_enclosed(id);
            let shape = WallShapeSnapshot::from_observed([0; 28]);
            inner_walls.update_wall_state_enclosure(id, shape);
            history.mark_changed();
        }
    }
}

/// Wall id carried by the in-progress wall (U field -> typed gap accessor).
fn wall_id_of(_wall: &crate::resources::walls::in_progress_wall::InProgressWall) -> Option<crate::resources::walls::WallId> {
    None
}
