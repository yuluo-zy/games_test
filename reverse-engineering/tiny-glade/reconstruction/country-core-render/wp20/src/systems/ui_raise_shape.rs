//! `systems/ui_raise_shape.rs` — raise-shape UI.
//!
//! PDB: `ui_raise_shape @0x140952e70` (8742 B), `impl_raise_shape @0x140955650`
//! (4211 B) + closures @0x1409550a0/@0x1409553f0. Named entities `RaiseShape`,
//! `HandleRaiseShapeUiInteraction`.
//!
//! O: `impl_raise_shape` projects the cursor onto the raise ray
//! (`utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_
//! onto_ray`), applies the height delta to the selected shape and records the
//! change via `resources::history::History::add`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::resources::history::History;
use crate::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord;
use crate::Gap;

/// `RaiseShape` (O named entity; component/command).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RaiseShape {
    pub _gap: Gap,
}

/// `HandleRaiseShapeUiInteraction` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct HandleRaiseShapeUiInteraction {
    pub _gap: Gap,
}

/// `ui_raise_shape` @0x140952e70 (O).
pub fn ui_raise_shape(
    mut _shapes: Query<&RaiseShape>,
    _cursor: Res<Gap>,
    mut _history: ResMut<History>,
    _commands: Commands,
) {
    todo!("recover ui_raise_shape (per-shape raise interaction)")
}

/// `impl_raise_shape` @0x140955650 (O).
pub fn impl_raise_shape(
    _shape: &RaiseShape,
    _cursor: glam::Vec2,
    _ray: &ProportionalScreenSpaceCoord,
    _history: &mut History,
) {
    todo!("recover impl_raise_shape (project_onto_ray + height delta + History::add)")
}
