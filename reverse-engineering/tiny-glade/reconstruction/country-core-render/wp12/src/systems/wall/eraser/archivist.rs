//! eraser/archivist.rs — WallEraserArchivist: records erase/split history while erasing.
//!
//! PDB procedures (see address-map 0x14092e6e0..0x14092f990): `WallEraserArchivist::{new,
//! record_rewiring_event, cache_if_original_wall, shrink_wall, add_split_event,
//! add_fully_erased_event, finish (21197 B), record_erase_original_wall, finish_erase_event,
//! finish_shrink_or_split_event, affected_walls, has_anything_been_affected}`, plus
//! `ChildWalls` and `WallHistoryEdits::compile_history_edit`.
//!
//! Business logic (O, call chain): an archivist accumulates wall edits performed by the
//! eraser — original walls cached, split events (shrink_wall / add_split_event /
//! add_fully_erased_event), rewiring events — then `finish` compiles them into a
//! `WallHistoryCompoundEdit` (CompoundEditBuilder::build). Typed-gap fields.

use std::collections::BTreeMap;

use crate::resources::history::wall_compound_edit::WallHistoryCompoundEdit;
use crate::resources::walls::WallId;
use crate::typed_gap::Gap;

/// Child walls produced by an erase split (O: `ChildWalls::remove/extend/is_empty`).
#[derive(Clone, Debug, Default)]
pub struct ChildWalls {
    pub walls: Vec<WallId>,
}
impl ChildWalls {
    pub fn remove(&mut self, wall: WallId) {
        self.walls.retain(|w| *w != wall);
    }
    pub fn extend(&mut self, iter: impl IntoIterator<Item = WallId>) {
        self.walls.extend(iter);
    }
    pub fn is_empty(&self) -> bool {
        self.walls.is_empty()
    }
}

/// Compiled wall history edits for one erase gesture (O: `compile_history_edit`).
#[derive(Clone, Debug, Default)]
pub struct WallHistoryEdits {
    pub raw: Gap,
}
impl WallHistoryEdits {
    pub fn compile_history_edit(&self) -> WallHistoryCompoundEdit {
        WallHistoryCompoundEdit::new()
    }
}

/// The eraser archivist (O: `new` 0x14092e6e0).
#[derive(Clone, Debug, Default)]
pub struct WallEraserArchivist {
    /// original wall id -> cached snapshot
    pub originals: BTreeMap<WallId, Gap>,
    pub children: ChildWalls,
    pub raw: Gap,
}
impl WallEraserArchivist {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn cache_if_original_wall(&mut self, _wall: WallId, _snapshot: Gap) {}
    pub fn record_erase_original_wall(&mut self, _wall: WallId, _snapshot: Gap) {}
    pub fn record_rewiring_event(&mut self) {}
    pub fn shrink_wall(&mut self, _wall: WallId, _low: f32, _high: f32) {}
    pub fn add_split_event(&mut self, _original: WallId, _a: WallId, _b: WallId) {}
    pub fn add_fully_erased_event(&mut self, _wall: WallId) {}
    pub fn finish_erase_event(&mut self) {}
    pub fn finish_shrink_or_split_event(&mut self) {}
    pub fn finish(&self) -> WallHistoryEdits {
        WallHistoryEdits { raw: Gap }
    }
    pub fn has_anything_been_affected(&self) -> bool {
        !self.originals.is_empty()
    }
    pub fn affected_walls(&self) -> Vec<WallId> {
        self.originals.keys().copied().collect()
    }
}
