//! B PendingTerrainHistory owns an optional typed terrain edit. The original
//! empty niche is i64::MIN+2, so a fresh owner does not record an empty edit.
use bevy_ecs::prelude::*;
use std::sync::Arc;

use crate::resources::history::{EditType, History};
pub use crate::systems::history::history_process::DebugHistoryMsg;
pub use tg_terrain::schema::TerrainEdit;

pub const DEFERRED_TERRAIN_SENTINEL: u64 = 0x8000_0000_0000_0002;

#[derive(Resource, Clone, Debug, Default)]
pub struct PendingTerrainHistory {
    pub edit: Option<TerrainEdit>,
}
impl PendingTerrainHistory {
    /// Both original tool producers replace then assert the old niche is empty.
    pub fn stage(&mut self, edit: TerrainEdit) {
        let previous = self.edit.replace(edit);
        assert!(previous.is_none(), "terrain history pending edit was not consumed");
    }
    pub fn is_empty(&self) -> bool {
        self.edit.is_none()
    }
}

/// B 0x140ac9770: move the edit into History, resetting the pending owner before
/// add; publish the shared payload-free debug notification unconditionally.
pub fn history_record_terrain<S: Send + Sync + 'static>(
    mut history: ResMut<History<S>>,
    mut pending: ResMut<PendingTerrainHistory>,
    mut debug: EventWriter<DebugHistoryMsg>,
) {
    if let Some(edit) = pending.edit.take() {
        let _ = history.add(EditType::Terrain(Arc::new(edit)));
        debug.write(DebugHistoryMsg);
    }
}
