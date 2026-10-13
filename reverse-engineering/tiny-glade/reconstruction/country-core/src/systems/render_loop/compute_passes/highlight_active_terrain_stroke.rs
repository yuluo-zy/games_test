//! `highlight_active_terrain_stroke.rs` — W8 highlight active terrain stroke (0x140a58450).
//!
//! URI `/terrain_editing/calc_highlight_hill_mask.cs.hlsl`. `EventCursor::read` +
//! last + uniform + read_texture + write_texture + dispatch +
//! `RenderGraphRecorder::create` (O). `AddTerrainHighlightCmd` (O name; layout U).

use crate::typed_gap::Gap;

pub struct AddTerrainHighlightCmd(pub Gap);

pub fn highlight_active_terrain_stroke(world: &mut Gap, _stroke_events: &Gap) {
    let _ = world;
    unimplemented!()
}
