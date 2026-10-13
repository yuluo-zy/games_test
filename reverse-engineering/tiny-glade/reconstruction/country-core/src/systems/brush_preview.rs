//! `systems/brush_preview.rs` — brush-preview mesh that follows the cursor /
//! selected shape.
//!
//! PDB: `brush_preview @0x140865050` (5355 B),
//! `register_required_components @0x140866800`, `clone_behavior @0x140867080`.
//! Named entities `BrushPreview`, `OverwriteBrushPreview`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::utils::components::transform::Transform;
use crate::Gap;

/// `BrushPreview` (O named entity; component).
#[derive(Component, Clone, Debug, Default)]
pub struct BrushPreview {
    pub _gap: Gap,
}

/// `OverwriteBrushPreview` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct OverwriteBrushPreview {
    pub _gap: Gap,
}

/// `brush_preview` @0x140865050 (O).
pub fn brush_preview(
    mut _previews: Query<(&BrushPreview, &mut Transform)>,
    _cursor: Res<Gap>,
    _commands: Commands,
) {
    let _scope = crate::reused::puffin::Scope::new("brush_preview");
    todo!("recover brush_preview (cursor → preview Transform)")
}
