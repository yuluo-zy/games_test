//! `systems/ui_move_shapes.rs` — move shapes UI (gizmo drag → position).
//!
//! PDB: `ui_move_shapes @0x14097a060` (12822 B), `move_gizmo_and_trigger_vol
//! @0x14097d280` (2618 B) + closures @0x14097dcc0/@0x14097e290. Named entity
//! `MoveShapeCache`.
//!
//! O: `move_gizmo_and_trigger_vol` applies the drag delta to the shape and
//! triggers the shape volume recompute; `MoveShapeCache` tracks in-flight
//! movement.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::Gap;

/// `MoveShapeCache` (O named entity).
#[derive(Resource, Clone, Debug, Default)]
pub struct MoveShapeCache {
    pub raw: Gap,
}

/// `ui_move_shapes` @0x14097a060 (O).
pub fn ui_move_shapes(
    mut _cache: ResMut<MoveShapeCache>,
    _cursor: Res<Gap>,
    _input: Res<Gap>,
    _commands: Commands,
) {
    todo!("recover ui_move_shapes (gizmo drag → shape position)")
}

/// `move_gizmo_and_trigger_vol` @0x14097d280 (O).
pub fn move_gizmo_and_trigger_vol(
    _cache: &mut MoveShapeCache,
    _delta: Vec2,
    _commands: &mut Commands,
) {
    todo!("recover move_gizmo_and_trigger_vol (apply delta + volume trigger)")
}
