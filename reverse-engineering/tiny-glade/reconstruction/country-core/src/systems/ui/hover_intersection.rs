//! `systems/ui/hover_intersection.rs` — world-canvas hover intersection
//! (`WidgetIntersection`, `HoverIntersectionCache`, `hover_intersection`).
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/hover_intersection.rs`.
//! PDB named entities: `HoverIntersectionCache`, `WidgetIntersection`.
//!
//! O evidence (Ghidra, B patched EXE; `hover_intersection` @0x1408383a0):
//! parallel (`bevy_tasks::ParallelSlice::par_chunk_map`) ray-vs-volume test
//! over `(WidgetId, &WidgetParams)`; produces `Vec<WidgetIntersection>` plus a
//! `Vec<(&WorldCanvasLayer, WidgetIntersection)>` sorted by canvas layer and
//! distance (`sort_by` closures + stable merge). `UiState` then stores the
//! top intersection (context bytes @+0x28..+0x58, O).

#![allow(dead_code, unused_imports)]

use crate::systems::ui::resources::ui_state::WidgetId;
use crate::Gap;

/// `WidgetIntersection` (O named entity): one widget hit.
#[derive(Clone, Copy, Debug, Default)]
pub struct WidgetIntersection {
    /// hit widget id.
    pub id: WidgetId,
    /// hit point (world space; I).
    pub point: glam::Vec3,
    /// distance along the cursor ray (I).
    pub distance: f32,
    pub _gap: Gap,
}

/// `HoverIntersectionCache` (O named entity; per-frame cache resource).
#[derive(Clone, Debug, Default)]
pub struct HoverIntersectionCache(pub Gap);

/// `hover_intersection` @0x1408383a0 (O).
pub fn hover_intersection(_ui_state: &mut Gap, _cursor_ray: &Gap) {
    todo!("recover hover_intersection (par_chunk_map volume test + layer/distance sort)")
}
