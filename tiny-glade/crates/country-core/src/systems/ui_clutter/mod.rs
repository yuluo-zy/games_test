//! Automatic clutter deletion authority and the recovered ID/history path.
//! Picking and conversion into placed decorator clutter are separate owners.
pub mod defered_autoclutter_history;
pub mod edit_autoclutter;
pub mod registration;

use bevy_ecs::prelude::Resource;
use std::collections::HashSet;

/// B 0x140aa4a60 reads one table of 8-byte IDs; the PDB default at source
/// ui_clutter/mod.rs:20 is std::collections::HashSet::default. Folded generic
/// HashMap aliases do not imply additional wall/decorator deletion tables.
#[derive(Resource, Clone, Debug, Default)]
pub struct DeletedAutoClutter(pub HashSet<u64>);

impl DeletedAutoClutter {
    pub fn is_deleted(&self, id: u64) -> bool {
        self.0.contains(&id)
    }
}

#[cfg(test)]
mod tests;
