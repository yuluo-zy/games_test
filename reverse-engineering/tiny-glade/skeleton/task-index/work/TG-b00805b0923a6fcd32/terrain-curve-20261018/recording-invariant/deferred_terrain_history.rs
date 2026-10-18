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
        assert!(
            previous.is_none(),
            "terrain history pending edit was not consumed"
        );
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
        history
            .add(EditType::Terrain(Arc::new(edit)))
            .expect("terrain history recorder requires an open input recording");
        debug.write(DebugHistoryMsg);
    }
}


#[cfg(test)]
mod recording_invariant_tests {
    use super::*;
    use crate::resources::terrain_editing::StrokeId;

    #[test]
    #[should_panic(expected = "terrain history recorder requires an open input recording")]
    fn missing_recording_is_an_invariant_failure_instead_of_a_silent_lost_edit() {
        let mut world = World::new();
        world.insert_resource(History::<()>::empty_glade());
        let mut pending = PendingTerrainHistory::default();
        pending.stage(TerrainEdit::EditTerrainStroke {
            index: StrokeId(7),
            prev_height_ws: 2.5,
            new_height_ws: 3.0,
            prev_falloff: 0.4,
            new_falloff: 0.5,
        });
        world.insert_resource(pending);
        world.init_resource::<Events<DebugHistoryMsg>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(history_record_terrain::<()>);
        schedule.run(&mut world);
    }
}
