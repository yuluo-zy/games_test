//! `systems/ui_edit_focus.rs` — edit-focus UI: converts a geometry source into
//! the focus `EditSubject` and tracks the focus origin.
//!
//! PDB: `ui_edit_focus @0x1409771f0` (3411 B), `geo_src_to_edit_subject
//! @0x140977f50`, `geo_src_to_edit_subject_w_deco_clutter @0x140978000`.
//! Named entities: `TerrainFocusModeOrigin`, `ToolSelectionCancelled`.
//!
//! O: `ui_edit_focus` follows the cursor / terrain to move the focus subject;
//! `geo_src_to_edit_subject` maps the geometry source (`GeoSource` variants)
//! into `EditSubject`; the `_w_deco_clutter` variant additionally considers
//! deco-clutter selection.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::systems::mode_manager::EditSubject;
use crate::Gap;

/// `TerrainFocusModeOrigin` (O named entity; focus origin resource).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainFocusModeOrigin {
    pub _gap: Gap,
}

/// `ToolSelectionCancelled` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct ToolSelectionCancelled {
    pub _gap: Gap,
}

/// `ui_edit_focus` @0x1409771f0 (O).
pub fn ui_edit_focus(
    mut _subject: ResMut<EditSubject>,
    mut _origin: ResMut<TerrainFocusModeOrigin>,
    _cursor: Res<Gap>,
    _commands: Commands,
) {
    todo!("recover ui_edit_focus (cursor→focus subject)")
}

/// `geo_src_to_edit_subject` @0x140977f50 (O).
pub fn geo_src_to_edit_subject(_src: &Gap) -> EditSubject {
    EditSubject(Gap)
}

/// `geo_src_to_edit_subject_w_deco_clutter` @0x140978000 (O).
pub fn geo_src_to_edit_subject_w_deco_clutter(_src: &Gap) -> EditSubject {
    EditSubject(Gap)
}
