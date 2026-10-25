//! `ui_clutter/defered_autoclutter_history.rs` — deferred auto-clutter history
//! recording.
//!
//! PDB: `history_record_auto_clutter @0x1409aa3e0` (346 B). Named entity
//! `RecordAutoClutterHistory`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::Gap;

/// `RecordAutoClutterHistory` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct RecordAutoClutterHistory {
    pub _gap: Gap,
}

/// `history_record_auto_clutter` @0x1409aa3e0 (O).
pub fn history_record_auto_clutter(
    mut _records: EventReader<RecordAutoClutterHistory>,
    mut _history: ResMut<Gap>,
) {
    todo!("recover history_record_auto_clutter (record to history)")
}
