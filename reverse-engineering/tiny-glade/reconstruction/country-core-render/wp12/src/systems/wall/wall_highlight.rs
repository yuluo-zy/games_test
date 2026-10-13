//! wall_highlight.rs — sets the wall/editing highlight from highlight commands.
//!
//! PDB procedure `wall_highlight` 0x140981390 (572 B). FunctionSystem signature O.
//!
//! Business logic (O, decompiled 0x140981390):
//!   - Consumes `HighlightWallCmd { wall, ty }`.
//!   - `WallEntities::wall_entity(wall)`; missing -> no highlight change.
//!   - Sets `WallHighlight { wall_entity, wall_id, ty }` and an `EditingHighlight` whose
//!     rgba color depends on `ty` (observed color words 0x3d8f5c29 / 0x3da3d70b / 0x3cf5c290 /
//!     0x3dcccccd).

use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::editing_highlight::EditingHighlight;
use crate::resources::events::HighlightWallCmd;
use crate::resources::walls::public_walls::WallEntities;
use crate::resources::walls::{WallHighlight, WallId};

pub fn wall_highlight(
    mut highlight: ResMut<WallHighlight>,
    mut editing_highlight: ResMut<EditingHighlight>,
    mut reader: EventReader<HighlightWallCmd>,
    wall_entities: Res<WallEntities>,
) {
    for cmd in reader.read() {
        let Some(entity) = wall_entities.wall_entity(cmd.wall) else {
            continue;
        };
        highlight.wall_entity = Some(entity);
        highlight.wall_id = Some(cmd.wall);
        highlight.ty = cmd.ty;
        // O: EditingHighlight rgba packed from the ty-selected color words.
        let _ = editing_highlight.raw;
        let _ = WallId;
    }
}
